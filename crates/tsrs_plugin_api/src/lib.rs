//! C ABI between the `tsrs` compiler and native transformer plugins (`compilerOptions.plugins[].transform`).
//!
//! A plugin is a `cdylib` that depends only on this crate and exports [`ENTRY_SYMBOL`] with the [`TransformFn`]
//! signature. Every AST read, node construction and checker query runs inside the compiler through the [`Host`]
//! table, so the plugin never touches the compiler's arenas or thread-locals and needs no compiler crates.
//!
//! Handles are only valid during the `tsrs_plugin_transform` call that produced them. [`Api`] wraps the table
//! in safe methods.

use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr;

pub mod kind;

/// Bumped on every change to [`Host`]; the host passes it in [`Host::abi_version`].
pub const ABI_VERSION: u32 = 1;

/// Name of the symbol a plugin library exports.
pub const ENTRY_SYMBOL: &str = "tsrs_plugin_transform";

/// Opaque handle to an AST node (`0` = none).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NodeHandle(pub usize);

impl NodeHandle {
    pub const NONE: Self = Self(0);

    #[inline]
    #[must_use]
    pub fn is_none(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub fn is_some(self) -> bool {
        self.0 != 0
    }

    #[inline]
    #[must_use]
    pub fn opt(self) -> Option<Self> {
        self.is_some().then_some(self)
    }
}

/// Opaque handle to a checker symbol (`0` = none).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SymbolHandle(pub usize);

impl SymbolHandle {
    pub const NONE: Self = Self(0);

    #[inline]
    #[must_use]
    pub fn is_none(self) -> bool {
        self.0 == 0
    }
}

/// Opaque handle to a checker type (`0` = none).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TypeHandle(pub usize);

impl TypeHandle {
    pub const NONE: Self = Self(0);

    #[inline]
    #[must_use]
    pub fn is_none(self) -> bool {
        self.0 == 0
    }
}

/// Borrowed UTF-8 string slice.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Str {
    pub ptr: *const u8,
    pub len: usize,
}

impl Default for Str {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Str {
    pub const EMPTY: Self = Self { ptr: ptr::null(), len: 0 };

    #[inline]
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self { ptr: s.as_ptr(), len: s.len() }
    }

    /// # Safety
    /// `self.ptr` must point to `self.len` bytes that live for `'a` (or `self.len == 0`). Invalid UTF-8 reads as `""`.
    #[inline]
    #[must_use]
    pub unsafe fn as_str<'a>(self) -> &'a str {
        if self.len == 0 || self.ptr.is_null() {
            return "";
        }
        // SAFETY: guaranteed by the caller.
        std::str::from_utf8(unsafe { std::slice::from_raw_parts(self.ptr, self.len) }).unwrap_or("")
    }
}

/// `import { property_name as name }`; an empty `property_name` imports `name` itself.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ImportSpec {
    pub property_name: Str,
    pub name: Str,
    pub is_type_only: bool,
}

/// Child callback of [`Host::visit_each_child`] (returns the replacement, `NONE` to drop the child from a list)
/// and [`Host::for_each_child`] (returns non-`NONE` to stop).
pub type VisitCallback = extern "C" fn(user_data: *mut c_void, node: NodeHandle) -> NodeHandle;

/// Signature of the exported [`ENTRY_SYMBOL`]: returns the transformed source file.
pub type TransformFn = unsafe extern "C" fn(
    host: *const Host,
    ctx: *mut c_void,
    source_file: NodeHandle,
    plugin_name: Str,
) -> NodeHandle;

/// Selectors for [`Host::node_child`] (single children) and [`Host::node_list_len`] / [`Host::node_list_at`]
/// (lists). A selector the node does not have yields `NONE` or an empty list.
pub mod field {
    pub const EXPRESSION: u16 = 1;
    pub const NAME: u16 = 2;
    pub const LEFT: u16 = 3;
    pub const RIGHT: u16 = 4;
    pub const OPERAND: u16 = 5;
    pub const INITIALIZER: u16 = 6;
    pub const TYPE: u16 = 7;
    pub const TYPE_NAME: u16 = 8;
    pub const QUESTION_DOT_TOKEN: u16 = 9;
    pub const QUESTION_TOKEN: u16 = 10;
    pub const POSTFIX_TOKEN: u16 = 11;
    pub const DOT_DOT_DOT_TOKEN: u16 = 12;
    pub const IMPORT_CLAUSE: u16 = 13;
    pub const NAMED_BINDINGS: u16 = 14;
    pub const MODULE_SPECIFIER: u16 = 15;
    pub const PROPERTY_NAME: u16 = 16;
    pub const BODY: u16 = 17;
    pub const DECLARATION_LIST: u16 = 18;
    pub const TAG: u16 = 19;
    pub const TEMPLATE: u16 = 20;
    pub const ARGUMENT_EXPRESSION: u16 = 21;
    pub const CONDITION: u16 = 22;
    pub const WHEN_TRUE: u16 = 23;
    pub const WHEN_FALSE: u16 = 24;
    pub const STATEMENT: u16 = 25;
    pub const THEN_STATEMENT: u16 = 26;
    pub const ELSE_STATEMENT: u16 = 27;
    pub const INCREMENTOR: u16 = 28;
    pub const OPERATOR_TOKEN: u16 = 29;
    pub const QUALIFIER: u16 = 30;

    pub const STATEMENTS: u16 = 100;
    pub const MEMBERS: u16 = 101;
    pub const PARAMETERS: u16 = 102;
    pub const ARGUMENTS: u16 = 103;
    pub const MODIFIERS: u16 = 104;
    pub const PROPERTIES: u16 = 105;
    pub const ELEMENTS: u16 = 106;
    pub const DECLARATIONS: u16 = 107;
    pub const TYPES: u16 = 108;
    pub const TYPE_ARGUMENTS: u16 = 109;
    pub const TEMPLATE_SPANS: u16 = 110;
    pub const CLAUSES: u16 = 111;
    pub const HERITAGE_CLAUSES: u16 = 112;
}

/// `NodeFlags` bits of the pinned compiler that plugins read or pass to [`Host::new_variable_statement`].
pub mod node_flags {
    pub const LET: u32 = 1 << 0;
    pub const CONST: u32 = 1 << 1;
    pub const SYNTHESIZED: u32 = 1 << 4;
    pub const OPTIONAL_CHAIN: u32 = 1 << 5;
}

/// Values of [`Host::transform_stage`]: where the plugin entry (`plugins[]` with `after` / `afterDeclarations`) runs.
pub mod stage {
    /// Before the TypeScript transforms (type erasure, import elision, decorators, class fields).
    pub const BEFORE: u32 = 0;
    /// After the TypeScript and downlevel transforms, before the module transform. Synthetic comments added
    /// earlier are dropped when a later transform rebuilds their node, so annotations belong here.
    pub const AFTER: u32 = 1;
    /// On the `.d.ts` output.
    pub const AFTER_DECLARATIONS: u32 = 2;
}

/// Callback table the compiler passes to the plugin. `ctx` is the opaque per-call context.
#[repr(C)]
pub struct Host {
    pub abi_version: u32,
    /// One of the [`stage`] values.
    pub transform_stage: extern "C" fn(ctx: *mut c_void) -> u32,

    // ── AST inspection ──
    pub node_kind: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> u16,
    pub node_flags: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> u32,
    /// Start of the node's range in its file (`-1` for synthesized nodes).
    pub node_pos: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> i32,
    pub node_end: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> i32,
    pub node_parent: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> NodeHandle,
    /// The parse-tree node a transformed node came from (the node itself for parse-tree nodes).
    pub node_original: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> NodeHandle,
    /// Text of identifiers, private identifiers and literals; empty for other nodes.
    pub node_text: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> Str,
    /// Operator kind of binary (operator token), prefix and postfix unary expressions; heritage clause token.
    pub node_operator: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> u16,
    pub node_child: extern "C" fn(ctx: *mut c_void, node: NodeHandle, field: u16) -> NodeHandle,
    pub node_list_len: extern "C" fn(ctx: *mut c_void, node: NodeHandle, field: u16) -> usize,
    pub node_list_at:
        extern "C" fn(ctx: *mut c_void, node: NodeHandle, field: u16, index: usize) -> NodeHandle,
    pub for_each_child: extern "C" fn(
        ctx: *mut c_void,
        node: NodeHandle,
        user_data: *mut c_void,
        cb: VisitCallback,
    ),
    pub source_file_name: extern "C" fn(ctx: *mut c_void, file: NodeHandle) -> Str,
    pub source_file_text: extern "C" fn(ctx: *mut c_void, file: NodeHandle) -> Str,
    pub program_source_file_count: extern "C" fn(ctx: *mut c_void) -> usize,
    pub program_source_file_at: extern "C" fn(ctx: *mut c_void, index: usize) -> NodeHandle,

    // ── Checker ── (queries on synthesized nodes return `NONE`)
    pub get_symbol_at_location: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> SymbolHandle,
    pub symbol_name: extern "C" fn(ctx: *mut c_void, symbol: SymbolHandle) -> Str,
    pub symbol_flags: extern "C" fn(ctx: *mut c_void, symbol: SymbolHandle) -> u32,
    pub symbol_declaration_count: extern "C" fn(ctx: *mut c_void, symbol: SymbolHandle) -> usize,
    pub symbol_declaration_at:
        extern "C" fn(ctx: *mut c_void, symbol: SymbolHandle, index: usize) -> NodeHandle,
    pub get_type_at_location: extern "C" fn(ctx: *mut c_void, node: NodeHandle) -> TypeHandle,
    pub type_flags: extern "C" fn(ctx: *mut c_void, ty: TypeHandle) -> u32,
    pub type_symbol: extern "C" fn(ctx: *mut c_void, ty: TypeHandle) -> SymbolHandle,
    /// Whether `ty` (or a union/intersection member) is, or nominally derives from, the type named `name`.
    /// `Array` uses the checker's array test; `String`/`Number`/`Boolean` also accept the primitives.
    pub is_type_subtype_of_named:
        extern "C" fn(ctx: *mut c_void, ty: TypeHandle, name: Str) -> bool,

    // ── Node factory ──
    pub new_identifier: extern "C" fn(ctx: *mut c_void, text: Str) -> NodeHandle,
    pub new_string_literal: extern "C" fn(ctx: *mut c_void, text: Str) -> NodeHandle,
    pub new_numeric_literal: extern "C" fn(ctx: *mut c_void, text: Str) -> NodeHandle,
    /// A keyword expression (`this`, `null`, `true`, ...), a modifier (`static`, `export`, ...) or a token.
    pub new_token: extern "C" fn(ctx: *mut c_void, kind: u16) -> NodeHandle,
    pub new_property_access: extern "C" fn(
        ctx: *mut c_void,
        expression: NodeHandle,
        name: NodeHandle,
        question_dot: bool,
    ) -> NodeHandle,
    pub new_call_expression: extern "C" fn(
        ctx: *mut c_void,
        callee: NodeHandle,
        args: *const NodeHandle,
        args_len: usize,
    ) -> NodeHandle,
    pub new_parenthesized_expression:
        extern "C" fn(ctx: *mut c_void, expression: NodeHandle) -> NodeHandle,
    pub new_array_literal: extern "C" fn(
        ctx: *mut c_void,
        elements: *const NodeHandle,
        len: usize,
        multi_line: bool,
    ) -> NodeHandle,
    pub new_object_literal: extern "C" fn(
        ctx: *mut c_void,
        properties: *const NodeHandle,
        len: usize,
        multi_line: bool,
    ) -> NodeHandle,
    pub new_property_assignment:
        extern "C" fn(ctx: *mut c_void, name: NodeHandle, initializer: NodeHandle) -> NodeHandle,
    pub new_conditional_expression: extern "C" fn(
        ctx: *mut c_void,
        condition: NodeHandle,
        when_true: NodeHandle,
        when_false: NodeHandle,
    ) -> NodeHandle,
    pub new_binary_expression: extern "C" fn(
        ctx: *mut c_void,
        left: NodeHandle,
        operator: u16,
        right: NodeHandle,
    ) -> NodeHandle,
    pub new_arrow_function: extern "C" fn(
        ctx: *mut c_void,
        parameters: *const NodeHandle,
        len: usize,
        body: NodeHandle,
    ) -> NodeHandle,
    pub new_block: extern "C" fn(
        ctx: *mut c_void,
        statements: *const NodeHandle,
        len: usize,
        multi_line: bool,
    ) -> NodeHandle,
    pub new_return_statement: extern "C" fn(ctx: *mut c_void, expression: NodeHandle) -> NodeHandle,
    pub new_expression_statement:
        extern "C" fn(ctx: *mut c_void, expression: NodeHandle) -> NodeHandle,
    /// `<modifiers> <let|const|var> <name> = <initializer>;` (`flags`: [`node_flags::LET`] / [`node_flags::CONST`]).
    pub new_variable_statement: extern "C" fn(
        ctx: *mut c_void,
        modifiers: *const NodeHandle,
        modifiers_len: usize,
        flags: u32,
        name: NodeHandle,
        initializer: NodeHandle,
    ) -> NodeHandle,
    pub new_property_declaration: extern "C" fn(
        ctx: *mut c_void,
        modifiers: *const NodeHandle,
        modifiers_len: usize,
        name: NodeHandle,
        initializer: NodeHandle,
    ) -> NodeHandle,
    pub new_get_accessor_declaration: extern "C" fn(
        ctx: *mut c_void,
        modifiers: *const NodeHandle,
        modifiers_len: usize,
        name: NodeHandle,
        body: NodeHandle,
    ) -> NodeHandle,
    /// `import { ... } from "<module>"`, or `import "<module>"` when `specs_len == 0`.
    pub new_import_declaration: extern "C" fn(
        ctx: *mut c_void,
        module_specifier: Str,
        specs: *const ImportSpec,
        specs_len: usize,
    ) -> NodeHandle,
    /// Several nodes returned in place of one from a [`Host::visit_each_child`] callback on a list element.
    pub new_syntax_list:
        extern "C" fn(ctx: *mut c_void, nodes: *const NodeHandle, len: usize) -> NodeHandle,

    // ── Updates (return a new node; unchanged inputs return the node itself) ──
    pub visit_each_child: extern "C" fn(
        ctx: *mut c_void,
        node: NodeHandle,
        user_data: *mut c_void,
        cb: VisitCallback,
    ) -> NodeHandle,
    pub update_source_file: extern "C" fn(
        ctx: *mut c_void,
        file: NodeHandle,
        statements: *const NodeHandle,
        len: usize,
    ) -> NodeHandle,
    /// Replaces the name (when `name` is not `NONE`) and the members of a class declaration.
    pub update_class_declaration: extern "C" fn(
        ctx: *mut c_void,
        node: NodeHandle,
        name: NodeHandle,
        members: *const NodeHandle,
        len: usize,
    ) -> NodeHandle,

    // ── Emit ──
    pub add_synthetic_leading_comment: extern "C" fn(
        ctx: *mut c_void,
        node: NodeHandle,
        multi_line: bool,
        text: Str,
        has_trailing_new_line: bool,
    ) -> NodeHandle,
    pub set_original_node:
        extern "C" fn(ctx: *mut c_void, node: NodeHandle, original: NodeHandle) -> NodeHandle,
}

/// Safe view of the host table for one `tsrs_plugin_transform` call.
#[derive(Clone, Copy)]
pub struct Api<'a> {
    host: &'a Host,
    ctx: *mut c_void,
    _call: PhantomData<&'a ()>,
}

macro_rules! slice_arg {
    ($nodes:expr) => {
        ($nodes.as_ptr(), $nodes.len())
    };
}

impl<'a> Api<'a> {
    /// # Safety
    /// `host` and `ctx` must be the arguments of the current `tsrs_plugin_transform` call.
    #[must_use]
    pub unsafe fn new(host: *const Host, ctx: *mut c_void) -> Option<Self> {
        // SAFETY: guaranteed by the caller.
        let host = unsafe { host.as_ref()? };
        (host.abi_version == ABI_VERSION).then_some(Self { host, ctx, _call: PhantomData })
    }

    fn str(s: Str) -> &'a str {
        // SAFETY: the host returns strings that live in the compiler's arenas for the whole compilation.
        unsafe { s.as_str() }
    }

    // ── Inspection ──

    #[must_use]
    pub fn stage(self) -> u32 {
        (self.host.transform_stage)(self.ctx)
    }
    #[must_use]
    pub fn kind(self, node: NodeHandle) -> u16 {
        (self.host.node_kind)(self.ctx, node)
    }
    #[must_use]
    pub fn flags(self, node: NodeHandle) -> u32 {
        (self.host.node_flags)(self.ctx, node)
    }
    #[must_use]
    pub fn pos(self, node: NodeHandle) -> i32 {
        (self.host.node_pos)(self.ctx, node)
    }
    #[must_use]
    pub fn end(self, node: NodeHandle) -> i32 {
        (self.host.node_end)(self.ctx, node)
    }
    #[must_use]
    pub fn parent(self, node: NodeHandle) -> NodeHandle {
        (self.host.node_parent)(self.ctx, node)
    }
    #[must_use]
    pub fn original(self, node: NodeHandle) -> NodeHandle {
        (self.host.node_original)(self.ctx, node)
    }
    #[must_use]
    pub fn text(self, node: NodeHandle) -> &'a str {
        Self::str((self.host.node_text)(self.ctx, node))
    }
    #[must_use]
    pub fn operator(self, node: NodeHandle) -> u16 {
        (self.host.node_operator)(self.ctx, node)
    }
    #[must_use]
    pub fn child(self, node: NodeHandle, field: u16) -> NodeHandle {
        (self.host.node_child)(self.ctx, node, field)
    }
    #[must_use]
    pub fn list_len(self, node: NodeHandle, field: u16) -> usize {
        (self.host.node_list_len)(self.ctx, node, field)
    }
    #[must_use]
    pub fn list_at(self, node: NodeHandle, field: u16, index: usize) -> NodeHandle {
        (self.host.node_list_at)(self.ctx, node, field, index)
    }
    #[must_use]
    pub fn list(self, node: NodeHandle, field: u16) -> Vec<NodeHandle> {
        (0..self.list_len(node, field)).map(|i| self.list_at(node, field, i)).collect()
    }
    /// Calls `f` on each child in order; `f` returns `true` to stop.
    pub fn for_each_child(self, node: NodeHandle, mut f: impl FnMut(NodeHandle) -> bool) {
        extern "C" fn trampoline(user_data: *mut c_void, node: NodeHandle) -> NodeHandle {
            // SAFETY: `user_data` is the `&mut dyn FnMut` below, alive for the host call.
            let f = unsafe { &mut *user_data.cast::<&mut dyn FnMut(NodeHandle) -> bool>() };
            if f(node) { NodeHandle(1) } else { NodeHandle::NONE }
        }
        let mut f: &mut dyn FnMut(NodeHandle) -> bool = &mut f;
        (self.host.for_each_child)(self.ctx, node, (&raw mut f).cast::<c_void>(), trampoline);
    }
    #[must_use]
    pub fn source_file_name(self, file: NodeHandle) -> &'a str {
        Self::str((self.host.source_file_name)(self.ctx, file))
    }
    #[must_use]
    pub fn source_file_text(self, file: NodeHandle) -> &'a str {
        Self::str((self.host.source_file_text)(self.ctx, file))
    }
    #[must_use]
    pub fn program_source_file_count(self) -> usize {
        (self.host.program_source_file_count)(self.ctx)
    }
    #[must_use]
    pub fn program_source_file_at(self, index: usize) -> NodeHandle {
        (self.host.program_source_file_at)(self.ctx, index)
    }
    #[must_use]
    pub fn program_source_files(self) -> Vec<NodeHandle> {
        (0..self.program_source_file_count()).map(|i| self.program_source_file_at(i)).collect()
    }

    // ── Checker ──

    #[must_use]
    pub fn symbol_at(self, node: NodeHandle) -> Option<SymbolHandle> {
        let sym = (self.host.get_symbol_at_location)(self.ctx, node);
        (!sym.is_none()).then_some(sym)
    }
    #[must_use]
    pub fn symbol_name(self, symbol: SymbolHandle) -> &'a str {
        Self::str((self.host.symbol_name)(self.ctx, symbol))
    }
    #[must_use]
    pub fn symbol_flags(self, symbol: SymbolHandle) -> u32 {
        (self.host.symbol_flags)(self.ctx, symbol)
    }
    #[must_use]
    pub fn symbol_declarations(self, symbol: SymbolHandle) -> Vec<NodeHandle> {
        let count = (self.host.symbol_declaration_count)(self.ctx, symbol);
        (0..count).map(|i| (self.host.symbol_declaration_at)(self.ctx, symbol, i)).collect()
    }
    #[must_use]
    pub fn type_at(self, node: NodeHandle) -> Option<TypeHandle> {
        let ty = (self.host.get_type_at_location)(self.ctx, node);
        (!ty.is_none()).then_some(ty)
    }
    #[must_use]
    pub fn type_flags(self, ty: TypeHandle) -> u32 {
        (self.host.type_flags)(self.ctx, ty)
    }
    #[must_use]
    pub fn type_symbol(self, ty: TypeHandle) -> Option<SymbolHandle> {
        let sym = (self.host.type_symbol)(self.ctx, ty);
        (!sym.is_none()).then_some(sym)
    }
    #[must_use]
    pub fn is_subtype_of_named(self, ty: TypeHandle, name: &str) -> bool {
        (self.host.is_type_subtype_of_named)(self.ctx, ty, Str::new(name))
    }

    // ── Factory ──

    #[must_use]
    pub fn identifier(self, text: &str) -> NodeHandle {
        (self.host.new_identifier)(self.ctx, Str::new(text))
    }
    #[must_use]
    pub fn string_literal(self, text: &str) -> NodeHandle {
        (self.host.new_string_literal)(self.ctx, Str::new(text))
    }
    #[must_use]
    pub fn numeric_literal(self, text: &str) -> NodeHandle {
        (self.host.new_numeric_literal)(self.ctx, Str::new(text))
    }
    #[must_use]
    pub fn token(self, kind: u16) -> NodeHandle {
        (self.host.new_token)(self.ctx, kind)
    }
    #[must_use]
    pub fn property_access(
        self,
        expression: NodeHandle,
        name: NodeHandle,
        question_dot: bool,
    ) -> NodeHandle {
        (self.host.new_property_access)(self.ctx, expression, name, question_dot)
    }
    #[must_use]
    pub fn call(self, callee: NodeHandle, args: &[NodeHandle]) -> NodeHandle {
        let (ptr, len) = slice_arg!(args);
        (self.host.new_call_expression)(self.ctx, callee, ptr, len)
    }
    #[must_use]
    pub fn paren(self, expression: NodeHandle) -> NodeHandle {
        (self.host.new_parenthesized_expression)(self.ctx, expression)
    }
    #[must_use]
    pub fn array_literal(self, elements: &[NodeHandle], multi_line: bool) -> NodeHandle {
        let (ptr, len) = slice_arg!(elements);
        (self.host.new_array_literal)(self.ctx, ptr, len, multi_line)
    }
    #[must_use]
    pub fn object_literal(self, properties: &[NodeHandle], multi_line: bool) -> NodeHandle {
        let (ptr, len) = slice_arg!(properties);
        (self.host.new_object_literal)(self.ctx, ptr, len, multi_line)
    }
    #[must_use]
    pub fn property_assignment(self, name: NodeHandle, initializer: NodeHandle) -> NodeHandle {
        (self.host.new_property_assignment)(self.ctx, name, initializer)
    }
    #[must_use]
    pub fn conditional(
        self,
        condition: NodeHandle,
        when_true: NodeHandle,
        when_false: NodeHandle,
    ) -> NodeHandle {
        (self.host.new_conditional_expression)(self.ctx, condition, when_true, when_false)
    }
    #[must_use]
    pub fn binary(self, left: NodeHandle, operator: u16, right: NodeHandle) -> NodeHandle {
        (self.host.new_binary_expression)(self.ctx, left, operator, right)
    }
    #[must_use]
    pub fn arrow_function(self, parameters: &[NodeHandle], body: NodeHandle) -> NodeHandle {
        let (ptr, len) = slice_arg!(parameters);
        (self.host.new_arrow_function)(self.ctx, ptr, len, body)
    }
    #[must_use]
    pub fn block(self, statements: &[NodeHandle], multi_line: bool) -> NodeHandle {
        let (ptr, len) = slice_arg!(statements);
        (self.host.new_block)(self.ctx, ptr, len, multi_line)
    }
    #[must_use]
    pub fn return_statement(self, expression: NodeHandle) -> NodeHandle {
        (self.host.new_return_statement)(self.ctx, expression)
    }
    #[must_use]
    pub fn expression_statement(self, expression: NodeHandle) -> NodeHandle {
        (self.host.new_expression_statement)(self.ctx, expression)
    }
    #[must_use]
    pub fn variable_statement(
        self,
        modifiers: &[NodeHandle],
        flags: u32,
        name: NodeHandle,
        initializer: NodeHandle,
    ) -> NodeHandle {
        let (ptr, len) = slice_arg!(modifiers);
        (self.host.new_variable_statement)(self.ctx, ptr, len, flags, name, initializer)
    }
    #[must_use]
    pub fn property_declaration(
        self,
        modifiers: &[NodeHandle],
        name: NodeHandle,
        initializer: NodeHandle,
    ) -> NodeHandle {
        let (ptr, len) = slice_arg!(modifiers);
        (self.host.new_property_declaration)(self.ctx, ptr, len, name, initializer)
    }
    #[must_use]
    pub fn get_accessor(
        self,
        modifiers: &[NodeHandle],
        name: NodeHandle,
        body: NodeHandle,
    ) -> NodeHandle {
        let (ptr, len) = slice_arg!(modifiers);
        (self.host.new_get_accessor_declaration)(self.ctx, ptr, len, name, body)
    }
    #[must_use]
    pub fn import_declaration(self, module_specifier: &str, specs: &[ImportSpec]) -> NodeHandle {
        let (ptr, len) = slice_arg!(specs);
        (self.host.new_import_declaration)(self.ctx, Str::new(module_specifier), ptr, len)
    }
    #[must_use]
    pub fn syntax_list(self, nodes: &[NodeHandle]) -> NodeHandle {
        let (ptr, len) = slice_arg!(nodes);
        (self.host.new_syntax_list)(self.ctx, ptr, len)
    }

    // ── Updates ──

    /// Rebuilds `node` with each child replaced by `f(child)` (`NONE` drops a list element; a syntax list
    /// splices several). Returns `node` itself when nothing changed.
    pub fn visit_each_child(
        self,
        node: NodeHandle,
        mut f: impl FnMut(NodeHandle) -> NodeHandle,
    ) -> NodeHandle {
        extern "C" fn trampoline(user_data: *mut c_void, node: NodeHandle) -> NodeHandle {
            // SAFETY: `user_data` is the `&mut dyn FnMut` below, alive for the host call.
            let f = unsafe { &mut *user_data.cast::<&mut dyn FnMut(NodeHandle) -> NodeHandle>() };
            f(node)
        }
        let mut f: &mut dyn FnMut(NodeHandle) -> NodeHandle = &mut f;
        (self.host.visit_each_child)(self.ctx, node, (&raw mut f).cast::<c_void>(), trampoline)
    }
    #[must_use]
    pub fn update_source_file(self, file: NodeHandle, statements: &[NodeHandle]) -> NodeHandle {
        let (ptr, len) = slice_arg!(statements);
        (self.host.update_source_file)(self.ctx, file, ptr, len)
    }
    #[must_use]
    pub fn update_class(
        self,
        node: NodeHandle,
        name: NodeHandle,
        members: &[NodeHandle],
    ) -> NodeHandle {
        let (ptr, len) = slice_arg!(members);
        (self.host.update_class_declaration)(self.ctx, node, name, ptr, len)
    }

    // ── Emit ──

    /// Attaches `/*text*/` (or `//text`) before the node when it is printed.
    pub fn add_leading_comment(
        self,
        node: NodeHandle,
        multi_line: bool,
        text: &str,
        trailing_new_line: bool,
    ) -> NodeHandle {
        (self.host.add_synthetic_leading_comment)(
            self.ctx,
            node,
            multi_line,
            Str::new(text),
            trailing_new_line,
        )
    }
    /// Links a new node to the node it replaces (source maps, comments and checker queries of later transforms).
    pub fn set_original(self, node: NodeHandle, original: NodeHandle) -> NodeHandle {
        (self.host.set_original_node)(self.ctx, node, original)
    }
}
