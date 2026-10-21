//! Native transformer plugins (`compilerOptions.plugins[].transform`): loader and the host side of the
//! `tsrs_plugin_api` C ABI.
//!
//! A plugin library exports `tsrs_plugin_transform` and calls back into [`HOST_VTABLE`] for every AST read, node
//! construction and checker query, so the compiler's arenas, `EmitContext` original-node links and `CheckerSlot`
//! borrows stay on this side.

use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::rc::Rc;
use std::sync::{LazyLock, Mutex};

use rustc_hash::{FxHashMap, FxHashSet};
use tsrs_ast::{self as ast, Kind, Node, NodeFlags, SourceFile, Symbol, SymbolFlags, TokenFlags};
use tsrs_checker::{Checker, ObjectFlags, Type, TypeFlags};
use tsrs_core::{P, alloc_slice, alloc_str, tspath};
use tsrs_plugin_api::{self as api, ImportSpec, NodeHandle, Str, SymbolHandle, TypeHandle, field};
use tsrs_printer::EmitContext;

use crate::{EmitHost, Resolver, TransformOptions, Transformer};

/// Stage in the emit pipeline where a plugin runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginStage {
    /// Before TypeScript type erasure, import elision and decorator transforms.
    Before,
    /// After the TypeScript and downlevel transforms, before the module transform.
    After,
    /// After the declaration transformer (`.d.ts`).
    AfterDeclarations,
}

impl PluginStage {
    fn abi_value(self) -> u32 {
        match self {
            PluginStage::Before => api::stage::BEFORE,
            PluginStage::After => api::stage::AFTER,
            PluginStage::AfterDeclarations => api::stage::AFTER_DECLARATIONS,
        }
    }
}

#[inline]
fn handle(node: Option<P<Node>>) -> NodeHandle {
    NodeHandle(P::to_bits_opt(node))
}

#[inline]
fn node_of(h: NodeHandle) -> Option<P<Node>> {
    // SAFETY: plugins only pass back handles the host produced during the current call.
    unsafe { P::from_bits_opt(h.0) }
}

#[inline]
fn symbol_handle(sym: Option<P<Symbol>>) -> SymbolHandle {
    SymbolHandle(P::to_bits_opt(sym))
}

#[inline]
fn symbol_of(h: SymbolHandle) -> Option<P<Symbol>> {
    // SAFETY: plugins only pass back handles the host produced during the current call.
    unsafe { P::from_bits_opt(h.0) }
}

#[inline]
fn type_handle(ty: Option<P<Type>>) -> TypeHandle {
    TypeHandle(P::to_bits_opt(ty))
}

#[inline]
fn type_of(h: TypeHandle) -> Option<P<Type>> {
    // SAFETY: plugins only pass back handles the host produced during the current call.
    unsafe { P::from_bits_opt(h.0) }
}

/// # Safety
/// `ptr` must point to `len` handles (or `len == 0`).
unsafe fn nodes_of(ptr: *const NodeHandle, len: usize) -> Vec<P<Node>> {
    if len == 0 || ptr.is_null() {
        return Vec::new();
    }
    // SAFETY: guaranteed by the caller.
    unsafe { std::slice::from_raw_parts(ptr, len) }.iter().filter_map(|&h| node_of(h)).collect()
}

struct PluginCallContext {
    emit_context: P<EmitContext>,
    emit_resolver: Option<Resolver>,
    source_files: &'static [P<SourceFile>],
    stage: PluginStage,
}

impl PluginCallContext {
    /// # Safety
    /// `ctx` must be the context pointer of the current plugin call.
    #[inline]
    unsafe fn from_raw<'a>(ctx: *mut c_void) -> &'a Self {
        // SAFETY: guaranteed by the caller; the context lives on the host stack for the whole call.
        unsafe { &*ctx.cast::<Self>() }
    }

    /// The parse-tree node the checker can answer for, or `None` for synthesized nodes.
    fn checker_node(&self, node: P<Node>) -> Option<P<Node>> {
        let orig = self.emit_context.parse_node(Some(node))?;
        (!orig.pos().is_synthetic()).then_some(orig)
    }
}

fn str_arg<'a>(s: Str) -> &'a str {
    // SAFETY: plugins pass UTF-8 slices that live for the duration of the callback.
    unsafe { s.as_str() }
}

fn ctx_of<'a>(ctx: *mut c_void) -> &'a PluginCallContext {
    // SAFETY: every host callback receives the context pointer of the current plugin call.
    unsafe { PluginCallContext::from_raw(ctx) }
}

fn modifier_list(f: &ast::NodeFactory, nodes: Vec<P<Node>>) -> Option<P<ast::ModifierList>> {
    (!nodes.is_empty()).then(|| f.new_modifier_list(nodes))
}

// ── Inspection ──

extern "C" fn host_transform_stage(ctx: *mut c_void) -> u32 {
    ctx_of(ctx).stage.abi_value()
}

extern "C" fn host_node_kind(_ctx: *mut c_void, node: NodeHandle) -> u16 {
    node_of(node).map_or(0, |n| n.kind() as u16)
}

extern "C" fn host_node_flags(_ctx: *mut c_void, node: NodeHandle) -> u32 {
    node_of(node).map_or(0, |n| n.flags().bits())
}

extern "C" fn host_node_pos(_ctx: *mut c_void, node: NodeHandle) -> i32 {
    node_of(node).and_then(|n| n.pos().source_offset()).map_or(-1, |p| p as i32)
}

extern "C" fn host_node_end(_ctx: *mut c_void, node: NodeHandle) -> i32 {
    node_of(node).and_then(|n| n.end().source_offset()).map_or(-1, |p| p as i32)
}

extern "C" fn host_node_parent(_ctx: *mut c_void, node: NodeHandle) -> NodeHandle {
    handle(node_of(node).and_then(|n| n.parent()))
}

extern "C" fn host_node_original(ctx: *mut c_void, node: NodeHandle) -> NodeHandle {
    let n = node_of(node);
    handle(ctx_of(ctx).emit_context.parse_node(n).or(n))
}

extern "C" fn host_node_text(_ctx: *mut c_void, node: NodeHandle) -> Str {
    let Some(n) = node_of(node) else {
        return Str::EMPTY;
    };
    match n.kind() {
        Kind::Identifier
        | Kind::PrivateIdentifier
        | Kind::StringLiteral
        | Kind::NumericLiteral
        | Kind::BigIntLiteral
        | Kind::NoSubstitutionTemplateLiteral
        | Kind::RegularExpressionLiteral
        | Kind::TemplateHead
        | Kind::TemplateMiddle
        | Kind::TemplateTail => Str::new(n.text()),
        _ => Str::EMPTY,
    }
}

extern "C" fn host_node_operator(_ctx: *mut c_void, node: NodeHandle) -> u16 {
    let Some(n) = node_of(node) else {
        return 0;
    };
    match n.kind() {
        Kind::BinaryExpression => n.as_binary_expression().operator_token.kind() as u16,
        Kind::PrefixUnaryExpression => n.as_prefix_unary_expression().operator as u16,
        Kind::PostfixUnaryExpression => n.as_postfix_unary_expression().operator as u16,
        Kind::HeritageClause => n.as_heritage_clause().token as u16,
        _ => 0,
    }
}

fn expression_child(n: P<Node>) -> Option<P<Node>> {
    match n.kind() {
        Kind::PropertyAccessExpression
        | Kind::ElementAccessExpression
        | Kind::ParenthesizedExpression
        | Kind::CallExpression
        | Kind::NewExpression
        | Kind::ExpressionWithTypeArguments
        | Kind::ComputedPropertyName
        | Kind::NonNullExpression
        | Kind::TypeAssertionExpression
        | Kind::AsExpression
        | Kind::SatisfiesExpression
        | Kind::TypeOfExpression
        | Kind::SpreadAssignment
        | Kind::SpreadElement
        | Kind::TemplateSpan
        | Kind::DeleteExpression
        | Kind::VoidExpression
        | Kind::AwaitExpression
        | Kind::YieldExpression
        | Kind::PartiallyEmittedExpression
        | Kind::IfStatement
        | Kind::DoStatement
        | Kind::WhileStatement
        | Kind::WithStatement
        | Kind::ForInStatement
        | Kind::ForOfStatement
        | Kind::SwitchStatement
        | Kind::CaseClause
        | Kind::ExpressionStatement
        | Kind::ReturnStatement
        | Kind::ThrowStatement
        | Kind::ExternalModuleReference
        | Kind::ExportAssignment
        | Kind::Decorator => n.expression(),
        _ => None,
    }
}

fn child_of(n: P<Node>, selector: u16) -> Option<P<Node>> {
    let kind = n.kind();
    match selector {
        field::EXPRESSION => expression_child(n),
        field::NAME => n.name(),
        field::LEFT => match kind {
            Kind::BinaryExpression => Some(n.as_binary_expression().left),
            Kind::QualifiedName => Some(n.as_qualified_name().left),
            _ => None,
        },
        field::RIGHT => match kind {
            Kind::BinaryExpression => Some(n.as_binary_expression().right()),
            Kind::QualifiedName => Some(n.as_qualified_name().right),
            _ => None,
        },
        field::OPERAND => match kind {
            Kind::PrefixUnaryExpression => Some(n.as_prefix_unary_expression().operand),
            Kind::PostfixUnaryExpression => Some(n.as_postfix_unary_expression().operand),
            _ => None,
        },
        field::INITIALIZER => match kind {
            Kind::VariableDeclaration
            | Kind::Parameter
            | Kind::BindingElement
            | Kind::PropertyDeclaration
            | Kind::PropertySignature
            | Kind::PropertyAssignment
            | Kind::EnumMember
            | Kind::ForStatement
            | Kind::ForInStatement
            | Kind::ForOfStatement => n.initializer(),
            _ => None,
        },
        field::TYPE => n.type_node(),
        field::TYPE_NAME => {
            (kind == Kind::TypeReference).then(|| n.as_type_reference_node().type_name)
        }
        field::QUESTION_DOT_TOKEN => match kind {
            Kind::PropertyAccessExpression
            | Kind::ElementAccessExpression
            | Kind::CallExpression
            | Kind::TaggedTemplateExpression => n.question_dot_token(),
            _ => None,
        },
        field::QUESTION_TOKEN => n.question_token(),
        field::POSTFIX_TOKEN => n.postfix_token(),
        field::DOT_DOT_DOT_TOKEN => match kind {
            Kind::Parameter => n.as_parameter_declaration().dot_dot_dot_token(),
            Kind::BindingElement => n.as_binding_element().dot_dot_dot_token(),
            _ => None,
        },
        field::IMPORT_CLAUSE => {
            (kind == Kind::ImportDeclaration).then(|| n.import_clause()).flatten()
        }
        field::NAMED_BINDINGS => {
            (kind == Kind::ImportClause).then(|| n.as_import_clause().named_bindings).flatten()
        }
        field::MODULE_SPECIFIER => match kind {
            Kind::ImportDeclaration | Kind::ExportDeclaration => n.module_specifier(),
            _ => None,
        },
        field::PROPERTY_NAME => match kind {
            Kind::ImportSpecifier | Kind::ExportSpecifier | Kind::BindingElement => {
                n.property_name()
            }
            _ => None,
        },
        field::BODY => match kind {
            Kind::ModuleDeclaration => n.as_module_declaration().body(),
            Kind::ClassStaticBlockDeclaration => Some(n.as_class_static_block_declaration().body),
            _ => n.body(),
        },
        field::DECLARATION_LIST => {
            (kind == Kind::VariableStatement).then(|| n.as_variable_statement().declaration_list)
        }
        field::TAG => {
            (kind == Kind::TaggedTemplateExpression).then(|| n.as_tagged_template_expression().tag)
        }
        field::TEMPLATE => (kind == Kind::TaggedTemplateExpression)
            .then(|| n.as_tagged_template_expression().template),
        field::ARGUMENT_EXPRESSION => (kind == Kind::ElementAccessExpression)
            .then(|| n.as_element_access_expression().argument_expression),
        field::CONDITION => match kind {
            Kind::ConditionalExpression => Some(n.as_conditional_expression().condition),
            Kind::ForStatement => n.as_for_statement().condition,
            _ => None,
        },
        field::WHEN_TRUE => {
            (kind == Kind::ConditionalExpression).then(|| n.as_conditional_expression().when_true)
        }
        field::WHEN_FALSE => {
            (kind == Kind::ConditionalExpression).then(|| n.as_conditional_expression().when_false)
        }
        field::STATEMENT => match kind {
            Kind::DoStatement
            | Kind::WhileStatement
            | Kind::ForStatement
            | Kind::ForInStatement
            | Kind::ForOfStatement
            | Kind::WithStatement
            | Kind::LabeledStatement => Some(n.statement()),
            _ => None,
        },
        field::THEN_STATEMENT => {
            (kind == Kind::IfStatement).then(|| n.as_if_statement().then_statement)
        }
        field::ELSE_STATEMENT => {
            (kind == Kind::IfStatement).then(|| n.as_if_statement().else_statement).flatten()
        }
        field::INCREMENTOR => {
            (kind == Kind::ForStatement).then(|| n.as_for_statement().incrementor).flatten()
        }
        field::OPERATOR_TOKEN => {
            (kind == Kind::BinaryExpression).then(|| n.as_binary_expression().operator_token)
        }
        _ => None,
    }
}

fn list_of(n: P<Node>, selector: u16) -> &'static [P<Node>] {
    let kind = n.kind();
    match selector {
        field::STATEMENTS => match kind {
            Kind::SourceFile
            | Kind::Block
            | Kind::ModuleBlock
            | Kind::CaseClause
            | Kind::DefaultClause => n.statements(),
            _ => &[],
        },
        field::MEMBERS => match kind {
            Kind::ClassDeclaration
            | Kind::ClassExpression
            | Kind::InterfaceDeclaration
            | Kind::EnumDeclaration
            | Kind::TypeLiteral => n.members(),
            _ => &[],
        },
        field::PARAMETERS => n.parameters(),
        field::ARGUMENTS => match kind {
            Kind::CallExpression | Kind::NewExpression => n.arguments(),
            _ => &[],
        },
        field::MODIFIERS => n.modifier_nodes(),
        field::PROPERTIES => match kind {
            Kind::ObjectLiteralExpression | Kind::JsxAttributes => n.properties(),
            _ => &[],
        },
        field::ELEMENTS => match kind {
            Kind::NamedImports
            | Kind::NamedExports
            | Kind::ObjectBindingPattern
            | Kind::ArrayBindingPattern
            | Kind::ArrayLiteralExpression
            | Kind::TupleType => n.elements(),
            _ => &[],
        },
        field::DECLARATIONS => match kind {
            Kind::VariableDeclarationList => n.as_variable_declaration_list().declarations.nodes(),
            _ => &[],
        },
        field::TYPES => match kind {
            Kind::UnionType => n.as_union_type_node().types().nodes(),
            Kind::IntersectionType => n.as_intersection_type_node().types().nodes(),
            Kind::HeritageClause => n.as_heritage_clause().types().nodes(),
            _ => &[],
        },
        field::TYPE_ARGUMENTS => match kind {
            Kind::CallExpression
            | Kind::NewExpression
            | Kind::TaggedTemplateExpression
            | Kind::TypeReference
            | Kind::ExpressionWithTypeArguments => n.type_arguments(),
            _ => &[],
        },
        field::TEMPLATE_SPANS => match kind {
            Kind::TemplateExpression => n.as_template_expression().template_spans.nodes(),
            _ => &[],
        },
        field::CLAUSES => match kind {
            Kind::CaseBlock => n.as_case_block().clauses.nodes(),
            _ => &[],
        },
        field::HERITAGE_CLAUSES => match kind {
            Kind::ClassDeclaration => {
                n.as_class_declaration().heritage_clauses().map_or(&[], |l| l.nodes())
            }
            Kind::ClassExpression => {
                n.as_class_expression().heritage_clauses().map_or(&[], |l| l.nodes())
            }
            Kind::InterfaceDeclaration => {
                n.as_interface_declaration().heritage_clauses.map_or(&[], |l| l.nodes())
            }
            _ => &[],
        },
        _ => &[],
    }
}

extern "C" fn host_node_child(_ctx: *mut c_void, node: NodeHandle, selector: u16) -> NodeHandle {
    handle(node_of(node).and_then(|n| child_of(n, selector)))
}

extern "C" fn host_node_list_len(_ctx: *mut c_void, node: NodeHandle, selector: u16) -> usize {
    node_of(node).map_or(0, |n| list_of(n, selector).len())
}

extern "C" fn host_node_list_at(
    _ctx: *mut c_void,
    node: NodeHandle,
    selector: u16,
    index: usize,
) -> NodeHandle {
    handle(node_of(node).and_then(|n| list_of(n, selector).get(index).copied()))
}

extern "C" fn host_for_each_child(
    _ctx: *mut c_void,
    node: NodeHandle,
    user_data: *mut c_void,
    cb: api::VisitCallback,
) {
    let Some(n) = node_of(node) else {
        return;
    };
    n.for_each_child(&mut |child| cb(user_data, handle(Some(child))).is_some());
}

extern "C" fn host_source_file_name(_ctx: *mut c_void, file: NodeHandle) -> Str {
    match node_of(file) {
        Some(n) if ast::is_source_file(n) => Str::new(n.as_source_file().file_name()),
        _ => Str::EMPTY,
    }
}

extern "C" fn host_source_file_text(_ctx: *mut c_void, file: NodeHandle) -> Str {
    match node_of(file) {
        Some(n) if ast::is_source_file(n) => Str::new(n.as_source_file().text()),
        _ => Str::EMPTY,
    }
}

extern "C" fn host_program_source_file_count(ctx: *mut c_void) -> usize {
    ctx_of(ctx).source_files.len()
}

extern "C" fn host_program_source_file_at(ctx: *mut c_void, index: usize) -> NodeHandle {
    handle(ctx_of(ctx).source_files.get(index).map(|f| f.as_node()))
}

// ── Checker ──

extern "C" fn host_get_symbol_at_location(ctx: *mut c_void, node: NodeHandle) -> SymbolHandle {
    let pctx = ctx_of(ctx);
    let (Some(orig), Some(resolver)) =
        (node_of(node).and_then(|n| pctx.checker_node(n)), pctx.emit_resolver)
    else {
        return SymbolHandle::NONE;
    };
    symbol_handle(resolver.lock(|c| c.get_symbol_at_location_exported(orig)))
}

extern "C" fn host_symbol_name(_ctx: *mut c_void, symbol: SymbolHandle) -> Str {
    symbol_of(symbol).map_or(Str::EMPTY, |s| Str::new(s.name()))
}

extern "C" fn host_symbol_flags(_ctx: *mut c_void, symbol: SymbolHandle) -> u32 {
    symbol_of(symbol).map_or(0, |s| s.flags().bits())
}

extern "C" fn host_symbol_declaration_count(_ctx: *mut c_void, symbol: SymbolHandle) -> usize {
    symbol_of(symbol).map_or(0, |s| s.declarations().len())
}

extern "C" fn host_symbol_declaration_at(
    _ctx: *mut c_void,
    symbol: SymbolHandle,
    index: usize,
) -> NodeHandle {
    handle(symbol_of(symbol).and_then(|s| s.declarations().get(index).copied()))
}

extern "C" fn host_get_type_at_location(ctx: *mut c_void, node: NodeHandle) -> TypeHandle {
    let pctx = ctx_of(ctx);
    let (Some(orig), Some(resolver)) =
        (node_of(node).and_then(|n| pctx.checker_node(n)), pctx.emit_resolver)
    else {
        return TypeHandle::NONE;
    };
    type_handle(Some(resolver.lock(|c| c.get_type_at_location(orig))))
}

extern "C" fn host_type_flags(_ctx: *mut c_void, ty: TypeHandle) -> u32 {
    type_of(ty).map_or(0, |t| t.flags().bits())
}

extern "C" fn host_type_symbol(_ctx: *mut c_void, ty: TypeHandle) -> SymbolHandle {
    symbol_handle(type_of(ty).and_then(|t| t.symbol()))
}

fn type_derives_from(
    c: &mut Checker,
    t: P<Type>,
    target: &str,
    seen: &mut FxHashSet<P<Type>>,
) -> bool {
    if !seen.insert(t) {
        return false;
    }
    if t.flags().intersects(TypeFlags::UnionOrIntersection) {
        return t
            .as_union_or_intersection_type()
            .types()
            .iter()
            .any(|&member| type_derives_from(c, member, target, seen));
    }
    if let Some(sym) = t.symbol() {
        if sym.name() == target {
            return true;
        }
        if sym
            .flags()
            .intersects(SymbolFlags::Class | SymbolFlags::Interface | SymbolFlags::TypeAlias)
        {
            let declared = c.get_declared_type_of_symbol_exported(sym);
            if declared != t && type_derives_from(c, declared, target, seen) {
                return true;
            }
        }
    }
    if t.object_flags().intersects(ObjectFlags::Reference)
        && let Some(target_ty) = t.target()
        && target_ty != t
        && type_derives_from(c, target_ty, target, seen)
    {
        return true;
    }
    if t.data().as_interface_type().is_some() {
        for &base in c.get_base_types_exported(t) {
            if type_derives_from(c, base, target, seen) {
                return true;
            }
        }
    }
    false
}

fn is_subtype_of_named(c: &mut Checker, t: P<Type>, target: &str) -> bool {
    if t.flags().intersects(TypeFlags::UnionOrIntersection) {
        return t
            .as_union_or_intersection_type()
            .types()
            .iter()
            .any(|&member| is_subtype_of_named(c, member, target));
    }
    match target {
        "Array" => return c.is_array_type_exported(t),
        "String" if t.flags().intersects(TypeFlags::StringLike) => return true,
        "Number" if t.flags().intersects(TypeFlags::NumberLike) => return true,
        "Boolean" if t.flags().intersects(TypeFlags::BooleanLike) => return true,
        _ => {}
    }
    type_derives_from(c, t, target, &mut FxHashSet::default())
}

extern "C" fn host_is_type_subtype_of_named(ctx: *mut c_void, ty: TypeHandle, name: Str) -> bool {
    let target = str_arg(name);
    let (Some(t), Some(resolver)) = (type_of(ty), ctx_of(ctx).emit_resolver) else {
        return false;
    };
    !target.is_empty() && resolver.lock(|c| is_subtype_of_named(c, t, target))
}

// ── Factory ──

extern "C" fn host_new_identifier(ctx: *mut c_void, text: Str) -> NodeHandle {
    handle(Some(ctx_of(ctx).emit_context.factory.new_identifier(alloc_str(str_arg(text)))))
}

extern "C" fn host_new_string_literal(ctx: *mut c_void, text: Str) -> NodeHandle {
    let f = &ctx_of(ctx).emit_context.factory;
    handle(Some(f.new_string_literal(alloc_str(str_arg(text)), TokenFlags::None)))
}

extern "C" fn host_new_numeric_literal(ctx: *mut c_void, text: Str) -> NodeHandle {
    let f = &ctx_of(ctx).emit_context.factory;
    handle(Some(f.new_numeric_literal(alloc_str(str_arg(text)), TokenFlags::None)))
}

extern "C" fn host_new_token(ctx: *mut c_void, kind: u16) -> NodeHandle {
    if kind >= Kind::Count as u16 {
        return NodeHandle::NONE;
    }
    let kind = Kind::from_i16(kind as i16);
    let f = &ctx_of(ctx).emit_context.factory;
    let node = match kind {
        Kind::ThisKeyword
        | Kind::SuperKeyword
        | Kind::NullKeyword
        | Kind::TrueKeyword
        | Kind::FalseKeyword
        | Kind::ImportKeyword => f.new_keyword_expression(kind),
        _ if ast::is_modifier_kind(kind) => f.new_modifier(kind),
        _ => f.new_token(kind),
    };
    handle(Some(node))
}

extern "C" fn host_new_property_access(
    ctx: *mut c_void,
    expression: NodeHandle,
    name: NodeHandle,
    question_dot: bool,
) -> NodeHandle {
    let (Some(expr), Some(name)) = (node_of(expression), node_of(name)) else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    let (token, flags) = if question_dot {
        (Some(f.new_token(Kind::QuestionDotToken)), NodeFlags::OptionalChain)
    } else {
        (None, NodeFlags::None)
    };
    handle(Some(f.new_property_access_expression(expr, token, name, flags)))
}

extern "C" fn host_new_call_expression(
    ctx: *mut c_void,
    callee: NodeHandle,
    args: *const NodeHandle,
    args_len: usize,
) -> NodeHandle {
    let Some(callee) = node_of(callee) else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `args_len` handles at `args`.
    let args = f.new_node_list(unsafe { nodes_of(args, args_len) });
    handle(Some(f.new_call_expression(callee, None, None, args, NodeFlags::None)))
}

extern "C" fn host_new_parenthesized_expression(
    ctx: *mut c_void,
    expression: NodeHandle,
) -> NodeHandle {
    let Some(expr) = node_of(expression) else {
        return NodeHandle::NONE;
    };
    handle(Some(ctx_of(ctx).emit_context.factory.new_parenthesized_expression(expr)))
}

extern "C" fn host_new_array_literal(
    ctx: *mut c_void,
    elements: *const NodeHandle,
    len: usize,
    multi_line: bool,
) -> NodeHandle {
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `len` handles at `elements`.
    let list = f.new_node_list(unsafe { nodes_of(elements, len) });
    handle(Some(f.new_array_literal_expression(list, multi_line)))
}

extern "C" fn host_new_object_literal(
    ctx: *mut c_void,
    properties: *const NodeHandle,
    len: usize,
    multi_line: bool,
) -> NodeHandle {
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `len` handles at `properties`.
    let list = f.new_node_list(unsafe { nodes_of(properties, len) });
    handle(Some(f.new_object_literal_expression(list, multi_line)))
}

extern "C" fn host_new_property_assignment(
    ctx: *mut c_void,
    name: NodeHandle,
    initializer: NodeHandle,
) -> NodeHandle {
    let (Some(name), Some(init)) = (node_of(name), node_of(initializer)) else {
        return NodeHandle::NONE;
    };
    handle(Some(
        ctx_of(ctx).emit_context.factory.new_property_assignment(None, name, None, None, init),
    ))
}

extern "C" fn host_new_conditional_expression(
    ctx: *mut c_void,
    condition: NodeHandle,
    when_true: NodeHandle,
    when_false: NodeHandle,
) -> NodeHandle {
    let (Some(c), Some(t), Some(e)) = (node_of(condition), node_of(when_true), node_of(when_false))
    else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    handle(Some(f.new_conditional_expression(
        c,
        f.new_token(Kind::QuestionToken),
        t,
        f.new_token(Kind::ColonToken),
        e,
    )))
}

extern "C" fn host_new_binary_expression(
    ctx: *mut c_void,
    left: NodeHandle,
    operator: u16,
    right: NodeHandle,
) -> NodeHandle {
    let (Some(l), Some(r)) = (node_of(left), node_of(right)) else {
        return NodeHandle::NONE;
    };
    if operator >= Kind::Count as u16 {
        return NodeHandle::NONE;
    }
    let f = &ctx_of(ctx).emit_context.factory;
    let op = f.new_token(Kind::from_i16(operator as i16));
    handle(Some(f.new_binary_expression(None, l, None, op, r)))
}

extern "C" fn host_new_arrow_function(
    ctx: *mut c_void,
    parameters: *const NodeHandle,
    len: usize,
    body: NodeHandle,
) -> NodeHandle {
    let Some(body) = node_of(body) else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `len` handles at `parameters`.
    let params = f.new_node_list(unsafe { nodes_of(parameters, len) });
    handle(Some(f.new_arrow_function(
        None,
        None,
        Some(params),
        None,
        None,
        Some(f.new_token(Kind::EqualsGreaterThanToken)),
        Some(body),
    )))
}

extern "C" fn host_new_block(
    ctx: *mut c_void,
    statements: *const NodeHandle,
    len: usize,
    multi_line: bool,
) -> NodeHandle {
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `len` handles at `statements`.
    let list = f.new_node_list(unsafe { nodes_of(statements, len) });
    handle(Some(f.new_block(list, multi_line)))
}

extern "C" fn host_new_return_statement(ctx: *mut c_void, expression: NodeHandle) -> NodeHandle {
    handle(Some(ctx_of(ctx).emit_context.factory.new_return_statement(node_of(expression))))
}

extern "C" fn host_new_expression_statement(
    ctx: *mut c_void,
    expression: NodeHandle,
) -> NodeHandle {
    let Some(expr) = node_of(expression) else {
        return NodeHandle::NONE;
    };
    handle(Some(ctx_of(ctx).emit_context.factory.new_expression_statement(expr)))
}

extern "C" fn host_new_variable_statement(
    ctx: *mut c_void,
    modifiers: *const NodeHandle,
    modifiers_len: usize,
    flags: u32,
    name: NodeHandle,
    initializer: NodeHandle,
) -> NodeHandle {
    let Some(name) = node_of(name) else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    let decl = f.new_variable_declaration(name, None, None, node_of(initializer));
    let flags = NodeFlags::from_bits_truncate(flags) & NodeFlags::BlockScoped;
    let list = f.new_variable_declaration_list(f.new_node_list(vec![decl]), flags);
    // SAFETY: the plugin passes `modifiers_len` handles at `modifiers`.
    let mods = modifier_list(f, unsafe { nodes_of(modifiers, modifiers_len) });
    handle(Some(f.new_variable_statement(mods, list)))
}

extern "C" fn host_new_property_declaration(
    ctx: *mut c_void,
    modifiers: *const NodeHandle,
    modifiers_len: usize,
    name: NodeHandle,
    initializer: NodeHandle,
) -> NodeHandle {
    let Some(name) = node_of(name) else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `modifiers_len` handles at `modifiers`.
    let mods = modifier_list(f, unsafe { nodes_of(modifiers, modifiers_len) });
    handle(Some(f.new_property_declaration(mods, name, None, None, node_of(initializer))))
}

extern "C" fn host_new_get_accessor_declaration(
    ctx: *mut c_void,
    modifiers: *const NodeHandle,
    modifiers_len: usize,
    name: NodeHandle,
    body: NodeHandle,
) -> NodeHandle {
    let Some(name) = node_of(name) else {
        return NodeHandle::NONE;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `modifiers_len` handles at `modifiers`.
    let mods = modifier_list(f, unsafe { nodes_of(modifiers, modifiers_len) });
    let params = f.new_node_list(Vec::new());
    handle(Some(f.new_get_accessor_declaration(
        mods,
        name,
        None,
        Some(params),
        None,
        None,
        node_of(body),
    )))
}

extern "C" fn host_new_import_declaration(
    ctx: *mut c_void,
    module_specifier: Str,
    specs: *const ImportSpec,
    specs_len: usize,
) -> NodeHandle {
    let f = &ctx_of(ctx).emit_context.factory;
    let specs = if specs_len == 0 || specs.is_null() {
        &[]
    } else {
        // SAFETY: the plugin passes `specs_len` specs at `specs`.
        unsafe { std::slice::from_raw_parts(specs, specs_len) }
    };
    let import_clause = (!specs.is_empty()).then(|| {
        let elements = specs
            .iter()
            .map(|spec| {
                let (property, local) = (str_arg(spec.property_name), str_arg(spec.name));
                let property = (!property.is_empty() && property != local)
                    .then(|| f.new_identifier(alloc_str(property)));
                f.new_import_specifier(
                    spec.is_type_only,
                    property,
                    f.new_identifier(alloc_str(local)),
                )
            })
            .collect();
        let named = f.new_named_imports(f.new_node_list(elements));
        f.new_import_clause(Kind::Unknown, None, Some(named))
    });
    let module = f.new_string_literal(alloc_str(str_arg(module_specifier)), TokenFlags::None);
    handle(Some(f.new_import_declaration(None, import_clause, module, None)))
}

extern "C" fn host_new_syntax_list(
    ctx: *mut c_void,
    nodes: *const NodeHandle,
    len: usize,
) -> NodeHandle {
    // SAFETY: the plugin passes `len` handles at `nodes`.
    let children = alloc_slice(&unsafe { nodes_of(nodes, len) });
    handle(Some(ctx_of(ctx).emit_context.factory.new_syntax_list(children)))
}

// ── Updates ──

extern "C" fn host_visit_each_child(
    ctx: *mut c_void,
    node: NodeHandle,
    user_data: *mut c_void,
    cb: api::VisitCallback,
) -> NodeHandle {
    let Some(n) = node_of(node) else {
        return NodeHandle::NONE;
    };
    let mut visitor = ctx_of(ctx).emit_context.new_node_visitor(Rc::new(
        move |_: &mut ast::NodeVisitor, child: P<Node>| node_of(cb(user_data, handle(Some(child)))),
    ));
    handle(visitor.visit_each_child(Some(n)))
}

extern "C" fn host_update_source_file(
    ctx: *mut c_void,
    file: NodeHandle,
    statements: *const NodeHandle,
    len: usize,
) -> NodeHandle {
    let Some(n) = node_of(file).filter(|n| ast::is_source_file(*n)) else {
        return file;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    // SAFETY: the plugin passes `len` handles at `statements`.
    let list = f.new_node_list(unsafe { nodes_of(statements, len) });
    handle(Some(f.update_source_file(n, list, n.as_source_file().end_of_file_token)))
}

extern "C" fn host_update_class_declaration(
    ctx: *mut c_void,
    node: NodeHandle,
    name: NodeHandle,
    members: *const NodeHandle,
    len: usize,
) -> NodeHandle {
    let Some(n) = node_of(node).filter(|n| n.kind() == Kind::ClassDeclaration) else {
        return node;
    };
    let f = &ctx_of(ctx).emit_context.factory;
    let class = n.as_class_declaration();
    // SAFETY: the plugin passes `len` handles at `members`.
    let members = f.new_node_list(unsafe { nodes_of(members, len) });
    handle(Some(f.update_class_declaration(
        n,
        class.modifiers(),
        node_of(name).or(class.name()),
        class.type_parameters(),
        class.heritage_clauses(),
        members,
    )))
}

// ── Emit ──

extern "C" fn host_add_synthetic_leading_comment(
    ctx: *mut c_void,
    node: NodeHandle,
    multi_line: bool,
    text: Str,
    has_trailing_new_line: bool,
) -> NodeHandle {
    if let Some(n) = node_of(node) {
        let kind =
            if multi_line { Kind::MultiLineCommentTrivia } else { Kind::SingleLineCommentTrivia };
        ctx_of(ctx).emit_context.add_synthetic_leading_comment(
            n,
            kind,
            str_arg(text),
            has_trailing_new_line,
        );
    }
    node
}

extern "C" fn host_set_original_node(
    ctx: *mut c_void,
    node: NodeHandle,
    original: NodeHandle,
) -> NodeHandle {
    if let (Some(n), Some(orig)) = (node_of(node), node_of(original)) {
        ctx_of(ctx).emit_context.set_original(n, orig);
    }
    node
}

pub static HOST_VTABLE: api::Host = api::Host {
    abi_version: api::ABI_VERSION,
    transform_stage: host_transform_stage,
    node_kind: host_node_kind,
    node_flags: host_node_flags,
    node_pos: host_node_pos,
    node_end: host_node_end,
    node_parent: host_node_parent,
    node_original: host_node_original,
    node_text: host_node_text,
    node_operator: host_node_operator,
    node_child: host_node_child,
    node_list_len: host_node_list_len,
    node_list_at: host_node_list_at,
    for_each_child: host_for_each_child,
    source_file_name: host_source_file_name,
    source_file_text: host_source_file_text,
    program_source_file_count: host_program_source_file_count,
    program_source_file_at: host_program_source_file_at,
    get_symbol_at_location: host_get_symbol_at_location,
    symbol_name: host_symbol_name,
    symbol_flags: host_symbol_flags,
    symbol_declaration_count: host_symbol_declaration_count,
    symbol_declaration_at: host_symbol_declaration_at,
    get_type_at_location: host_get_type_at_location,
    type_flags: host_type_flags,
    type_symbol: host_type_symbol,
    is_type_subtype_of_named: host_is_type_subtype_of_named,
    new_identifier: host_new_identifier,
    new_string_literal: host_new_string_literal,
    new_numeric_literal: host_new_numeric_literal,
    new_token: host_new_token,
    new_property_access: host_new_property_access,
    new_call_expression: host_new_call_expression,
    new_parenthesized_expression: host_new_parenthesized_expression,
    new_array_literal: host_new_array_literal,
    new_object_literal: host_new_object_literal,
    new_property_assignment: host_new_property_assignment,
    new_conditional_expression: host_new_conditional_expression,
    new_binary_expression: host_new_binary_expression,
    new_arrow_function: host_new_arrow_function,
    new_block: host_new_block,
    new_return_statement: host_new_return_statement,
    new_expression_statement: host_new_expression_statement,
    new_variable_statement: host_new_variable_statement,
    new_property_declaration: host_new_property_declaration,
    new_get_accessor_declaration: host_new_get_accessor_declaration,
    new_import_declaration: host_new_import_declaration,
    new_syntax_list: host_new_syntax_list,
    visit_each_child: host_visit_each_child,
    update_source_file: host_update_source_file,
    update_class_declaration: host_update_class_declaration,
    add_synthetic_leading_comment: host_add_synthetic_leading_comment,
    set_original_node: host_set_original_node,
};

// ── Loading ──

#[derive(Clone, Copy)]
struct LoadedPlugin {
    entry: api::TransformFn,
}

static LOADED_PLUGINS: LazyLock<Mutex<FxHashMap<String, Result<LoadedPlugin, String>>>> =
    LazyLock::new(Default::default);

#[cfg(unix)]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

fn load_plugin_library(path: &str) -> Result<LoadedPlugin, String> {
    LOADED_PLUGINS
        .lock()
        .unwrap()
        .entry(path.to_string())
        .or_insert_with(|| load_plugin_library_uncached(path))
        .clone()
}

#[cfg(unix)]
fn load_plugin_library_uncached(path: &str) -> Result<LoadedPlugin, String> {
    let c_path = CString::new(path).map_err(|e| format!("invalid plugin path {path:?}: {e}"))?;
    let symbol = CString::new(api::ENTRY_SYMBOL).unwrap();
    // RTLD_NOW is 2 on macOS and Linux. The library stays loaded for the life of the process.
    // SAFETY: `c_path` is NUL-terminated.
    let lib = unsafe { dlopen(c_path.as_ptr(), 2) };
    if lib.is_null() {
        // SAFETY: `dlerror` returns a NUL-terminated message (or null) for the calling thread.
        let err = unsafe {
            let p = dlerror();
            if p.is_null() {
                "unknown dlopen error".to_string()
            } else {
                CStr::from_ptr(p).to_string_lossy().into_owned()
            }
        };
        return Err(format!("failed to load plugin {path:?}: {err}"));
    }
    // SAFETY: `lib` came from `dlopen` and `symbol` is NUL-terminated.
    let sym = unsafe { dlsym(lib, symbol.as_ptr()) };
    if sym.is_null() {
        return Err(format!("plugin {path:?} does not export `{}`", api::ENTRY_SYMBOL));
    }
    // SAFETY: the exported entry point has the `TransformFn` signature (the plugin API contract).
    let entry = unsafe { std::mem::transmute::<*mut c_void, api::TransformFn>(sym) };
    Ok(LoadedPlugin { entry })
}

#[cfg(not(unix))]
fn load_plugin_library_uncached(path: &str) -> Result<LoadedPlugin, String> {
    Err(format!("native plugins are only supported on unix targets ({path:?})"))
}

fn resolve_plugin_path(raw_path: &str, options: &tsrs_core::CompilerOptions, cwd: &str) -> String {
    let base_dir = if options.config_file_path.is_empty() {
        cwd.to_string()
    } else {
        tspath::get_directory_path(&options.config_file_path)
    };
    let expanded = match raw_path.strip_prefix("${configDir}") {
        Some(rest) => format!("{base_dir}{rest}"),
        None => raw_path.to_string(),
    };
    tspath::get_normalized_absolute_path(&tspath::normalize_slashes(&expanded), &base_dir)
}

fn call_plugin(
    plugin: LoadedPlugin,
    mut ctx: PluginCallContext,
    file: P<Node>,
    plugin_name: &str,
) -> Option<P<Node>> {
    // SAFETY: `HOST_VTABLE` and `ctx` outlive the call, and `file` is a live node.
    let out = unsafe {
        (plugin.entry)(
            &raw const HOST_VTABLE,
            (&raw mut ctx).cast::<c_void>(),
            handle(Some(file)),
            Str::new(plugin_name),
        )
    };
    node_of(out).filter(|n| ast::is_source_file(*n))
}

struct NativePluginTransformer {
    base: Transformer,
    plugin: LoadedPlugin,
    plugin_name: String,
    emit_resolver: Option<Resolver>,
    source_files: &'static [P<SourceFile>],
    stage: PluginStage,
}

impl NativePluginTransformer {
    fn visit(&self, node: P<Node>) -> Option<P<Node>> {
        if !ast::is_source_file(node) {
            return Some(node);
        }
        let ctx = PluginCallContext {
            emit_context: self.base.emit_context(),
            emit_resolver: self.emit_resolver,
            source_files: self.source_files,
            stage: self.stage,
        };
        call_plugin(self.plugin, ctx, node, &self.plugin_name).or(Some(node))
    }
}

fn new_native_plugin_transformer(
    opt: &TransformOptions,
    plugin: LoadedPlugin,
    plugin_name: &str,
    source_files: &'static [P<SourceFile>],
    stage: PluginStage,
) -> P<Transformer> {
    let tx = P::new(NativePluginTransformer {
        base: Transformer::default(),
        plugin,
        plugin_name: plugin_name.to_string(),
        emit_resolver: opt.emit_resolver,
        source_files,
        stage,
    });
    tx.get().base.new_transformer(
        Rc::new(move |_: &mut ast::NodeVisitor, n: P<Node>| tx.visit(n)),
        Some(opt.context),
    )
}

/// The native plugin transformers configured in `compilerOptions.plugins` for `stage`.
pub fn get_plugin_transformers(
    opt: &TransformOptions,
    host: &dyn EmitHost,
    stage: PluginStage,
) -> Vec<P<Transformer>> {
    let Some(plugins) = opt.compiler_options.plugins.as_ref() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for plugin_cfg in plugins {
        let Some(raw_path) = plugin_cfg.transform_library_path() else {
            continue;
        };
        let plugin_stage = if plugin_cfg.after_declarations {
            PluginStage::AfterDeclarations
        } else if plugin_cfg.after {
            PluginStage::After
        } else {
            PluginStage::Before
        };
        if plugin_stage != stage {
            continue;
        }
        let resolved =
            resolve_plugin_path(raw_path, &opt.compiler_options, host.get_current_directory());
        match load_plugin_library(&resolved) {
            Ok(loaded) => {
                out.push(new_native_plugin_transformer(
                    opt,
                    loaded,
                    &plugin_cfg.name,
                    host.source_files(),
                    stage,
                ));
            }
            Err(err) => eprintln!("tsrs: {err}"),
        }
    }
    out
}

/// Runs the `afterDeclarations` native plugins on a transformed `.d.ts` source file.
pub fn run_declaration_plugins(
    emit_context: P<EmitContext>,
    compiler_options: P<tsrs_core::CompilerOptions>,
    emit_resolver: Resolver,
    current_directory: &str,
    source_files: &'static [P<SourceFile>],
    mut source_file: P<SourceFile>,
) -> P<SourceFile> {
    let Some(plugins) = compiler_options.plugins.as_ref() else {
        return source_file;
    };
    for plugin_cfg in plugins {
        if !plugin_cfg.after_declarations {
            continue;
        }
        let Some(raw_path) = plugin_cfg.transform_library_path() else {
            continue;
        };
        let resolved = resolve_plugin_path(raw_path, &compiler_options, current_directory);
        match load_plugin_library(&resolved) {
            Ok(loaded) => {
                let ctx = PluginCallContext {
                    emit_context,
                    emit_resolver: Some(emit_resolver),
                    source_files,
                    stage: PluginStage::AfterDeclarations,
                };
                if let Some(node) =
                    call_plugin(loaded, ctx, source_file.as_node(), &plugin_cfg.name)
                {
                    source_file = node.as_source_file_p();
                }
            }
            Err(err) => eprintln!("tsrs: {err}"),
        }
    }
    source_file
}

/// A transformer running `entry` in process, as if loaded from a plugin library (tests and embedders).
pub fn new_custom_plugin_transformer(
    opt: &TransformOptions,
    plugin_name: &str,
    entry: api::TransformFn,
    source_files: &'static [P<SourceFile>],
    stage: PluginStage,
) -> P<Transformer> {
    new_native_plugin_transformer(opt, LoadedPlugin { entry }, plugin_name, source_files, stage)
}
