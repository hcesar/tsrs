use std::ffi::c_void;
use std::sync::Arc;

use tsrs_compiler::{Context, ProgramOptions, new_compiler_host, new_program};
use tsrs_plugin_api::{
    self as api, Api, ImportSpec, NodeHandle, Str, field, kind, node_flags, stage,
};
use tsrs_tsoptions::{self as tsoptions, ParseConfigHost};
use tsrs_vfs::{FS, vfstest};

use tsrs_plugin_host::{PluginStage, new_custom_plugin_transformer};
use std::rc::Rc;
use tsrs_ast::{Kind, NodeFlags};
use tsrs_core::P;
use tsrs_printer as printer;
use tsrs_transformers::{ReferenceResolverRef, Resolver, TransformOptions, tstransforms};

struct ConfigHost {
    fs: Arc<dyn FS>,
}

impl ParseConfigHost for ConfigHost {
    fn fs(&self) -> &dyn FS {
        &*self.fs
    }
    fn get_current_directory(&self) -> &'static str {
        "/"
    }
}

const TSCONFIG: &str = r#"{"compilerOptions":{"noLib":true,"module":"esnext","target":"esnext","experimentalDecorators":true},"files":["a.ts"]}"#;

/// Emits `/src/a.ts` with `entry` as a `before` plugin, the TypeScript transforms, and `entry` again as an
/// `after` plugin, and prints the result.
fn emit_with_plugin(source: &str, plugin_name: &str, entry: api::TransformFn) -> String {
    let files = [("/src/tsconfig.json", TSCONFIG), ("/src/a.ts", source)];
    let fs: Arc<dyn FS> = Arc::new(vfstest::from_map(files.iter().map(|&(k, v)| (k, v)), true));
    let host: &'static ConfigHost = Box::leak(Box::new(ConfigHost { fs: Arc::clone(&fs) }));
    let (config, diagnostics) = tsoptions::get_parsed_command_line_of_config_file(
        "/src/tsconfig.json",
        None,
        None,
        host,
        None,
    );
    assert_eq!(diagnostics, [] as [tsrs_core::P<tsrs_ast::Diagnostic>; 0]);
    let config = P::new(config.unwrap());
    let program =
        new_program(ProgramOptions::new(config, new_compiler_host("/", fs, "", None, None, None)));
    let ctx = Context::default();
    let source_file = program.get_source_file("/src/a.ts").unwrap();
    let source_files = tsrs_core::alloc_slice(&[source_file]);
    let mut checker = program.get_type_checker_for_file(&ctx, source_file);
    let slot = P::new(tsrs_checker::CheckerSlot::default());
    let context = printer::new_emit_context();
    let resolver = Resolver::new((*checker).new_emit_resolver(context), slot);
    let options = program.options();
    slot.lend(&mut checker, || {
        let opts = TransformOptions {
            context,
            compiler_options: options,
            resolver: ReferenceResolverRef::Emit(resolver),
            emit_resolver: Some(resolver),
            get_emit_module_format_of_file: Rc::new(|_| tsrs_core::ModuleKind::ES_NEXT),
            content_mapper_extension_rewrites: Vec::new(),
            use_case_sensitive_file_names: true,
        };
        let mut result = new_custom_plugin_transformer(
            &opts,
            plugin_name,
            entry,
            source_files,
            PluginStage::Before,
        )
        .transform_source_file(source_file);
        result =
            tstransforms::new_type_eraser_transformer(&opts).unwrap().transform_source_file(result);
        result = tstransforms::new_import_elision_transformer(&opts)
            .unwrap()
            .transform_source_file(result);
        result = tstransforms::new_runtime_syntax_transformer(&opts)
            .unwrap()
            .transform_source_file(result);
        if let Some(decorators) = tstransforms::new_legacy_decorators_transformer(&opts) {
            result = decorators.transform_source_file(result);
        }
        result = new_custom_plugin_transformer(
            &opts,
            plugin_name,
            entry,
            source_files,
            PluginStage::After,
        )
        .transform_source_file(result);
        let mut p = printer::new_printer(
            printer::PrinterOptions {
                target: options.get_emit_script_target(),
                ..Default::default()
            },
            printer::PrintHandlers::default(),
            Some(context),
        );
        p.emit_source_file(result)
    })
}

fn api_of<'a>(host: *const api::Host, ctx: *mut c_void) -> Api<'a> {
    // SAFETY: called with the arguments of the current `tsrs_plugin_transform` call.
    unsafe { Api::new(host, ctx) }.expect("ABI version matches")
}

#[test]
fn kind_constants_match_the_compiler() {
    // Every compiler kind has an API code; append a missing one to `tsrs_plugin_api::kind`.
    let names: rustc_hash::FxHashSet<&str> = kind::NAMES.iter().copied().collect();
    for value in 0..Kind::Count as i16 {
        let name = format!("{:?}", Kind::from_i16(value));
        assert!(names.contains(name.as_str()), "tsrs_plugin_api::kind has no {name}");
    }
    assert_eq!(kind::NAMES[usize::from(kind::CALL_EXPRESSION)], "CallExpression");
    assert_eq!(kind::NAMES[usize::from(kind::SOURCE_KEYWORD)], "SourceKeyword");
    assert_eq!(node_flags::CONST, NodeFlags::Const.bits());
    assert_eq!(node_flags::LET, NodeFlags::Let.bits());
    assert_eq!(node_flags::SYNTHESIZED, NodeFlags::Synthesized.bits());
    assert_eq!(node_flags::OPTIONAL_CHAIN, NodeFlags::OptionalChain.bits());
}

// ── Inspection, factory, synthetic comments and imports ──

fn rewrite_macros(
    a: Api<'_>,
    file_name: &str,
    used_trace: &mut bool,
    node: NodeHandle,
) -> NodeHandle {
    match a.kind(node) {
        kind::CALL_EXPRESSION => {
            let callee = a.child(node, field::EXPRESSION);
            if a.kind(callee) == kind::IDENTIFIER
                && a.text(callee) == "debugLog"
                && a.list_len(node, field::ARGUMENTS) == 1
            {
                let arg =
                    rewrite_macros(a, file_name, used_trace, a.list_at(node, field::ARGUMENTS, 0));
                let call = a.call(a.identifier("__trace"), &[arg]);
                a.add_leading_comment(call, true, "#__PURE__", false);
                *used_trace = true;
                return a.set_original(call, node);
            }
        }
        kind::IDENTIFIER if a.text(node) == "__FILE__" => {
            return a.set_original(a.string_literal(file_name), node);
        }
        kind::IDENTIFIER if a.text(node) == "__LINE_COUNT__" => {
            return a.set_original(a.numeric_literal("42"), node);
        }
        _ => {}
    }
    a.visit_each_child(node, |child| rewrite_macros(a, file_name, used_trace, child))
}

extern "C" fn macro_plugin(
    host: *const api::Host,
    ctx: *mut c_void,
    file: NodeHandle,
    name: Str,
) -> NodeHandle {
    let a = api_of(host, ctx);
    if a.stage() != stage::BEFORE {
        return file;
    }
    // SAFETY: the host passes the plugin name as UTF-8.
    assert_eq!(unsafe { name.as_str() }, "macro-plugin");
    let file_name = a.source_file_name(file);
    assert_eq!(file_name, "/src/a.ts");
    assert!(a.source_file_text(file).contains("__FILE__"));
    assert_eq!(a.program_source_files(), [file]);

    let mut used_trace = false;
    let file =
        a.visit_each_child(file, |child| rewrite_macros(a, file_name, &mut used_trace, child));
    if !used_trace {
        return file;
    }
    let spec = ImportSpec {
        property_name: Str::new("trace"),
        name: Str::new("__trace"),
        is_type_only: false,
    };
    let mut statements = vec![a.import_declaration("runtime-logger", &[spec])];
    statements.extend(a.list(file, field::STATEMENTS));
    a.update_source_file(file, &statements)
}

#[test]
fn plugin_rewrites_nodes_and_adds_comments_and_imports() {
    let out = emit_with_plugin(
        "export const info = debugLog(__FILE__);\nexport const n = __LINE_COUNT__;\n",
        "macro-plugin",
        macro_plugin,
    );
    assert!(out.contains(r#"import { trace as __trace } from "runtime-logger";"#), "got:\n{out}");
    assert!(
        out.contains(r#"export const info = /*#__PURE__*/ __trace("/src/a.ts");"#),
        "got:\n{out}"
    );
    assert!(out.contains("export const n = 42;"), "got:\n{out}");
}

// ── Symbol and type queries ──

fn is_private_undecorated(a: Api<'_>, node: NodeHandle) -> bool {
    let Some(sym) = a.symbol_at(node) else {
        return false;
    };
    let modifiers: Vec<u16> = a
        .symbol_declarations(sym)
        .into_iter()
        .flat_map(|decl| a.list(decl, field::MODIFIERS))
        .map(|m| a.kind(m))
        .collect();
    modifiers.contains(&kind::PRIVATE_KEYWORD) && !modifiers.contains(&kind::DECORATOR)
}

fn rewrite_typed(a: Api<'_>, node: NodeHandle) -> NodeHandle {
    if a.kind(node) == kind::PROPERTY_ACCESS_EXPRESSION
        && a.child(node, field::QUESTION_DOT_TOKEN).is_none()
    {
        let name = a.child(node, field::NAME);
        let receiver = a.child(node, field::EXPRESSION);
        if a.text(name) == "value"
            && a.type_at(receiver).is_some_and(|t| a.is_subtype_of_named(t, "BaseBox"))
        {
            let access =
                a.property_access(rewrite_typed(a, receiver), a.identifier("renamedValue"), false);
            return a.set_original(access, node);
        }
    }
    if a.kind(node) == kind::IDENTIFIER && is_private_undecorated(a, node) {
        let renamed = a.identifier(&format!("_priv_{}", a.text(node)));
        return a.set_original(renamed, node);
    }
    a.visit_each_child(node, |child| rewrite_typed(a, child))
}

extern "C" fn type_query_plugin(
    host: *const api::Host,
    ctx: *mut c_void,
    file: NodeHandle,
    _: Str,
) -> NodeHandle {
    let a = api_of(host, ctx);
    if a.stage() != stage::BEFORE {
        return file;
    }
    a.visit_each_child(file, |child| rewrite_typed(a, child))
}

#[test]
fn plugin_queries_symbols_and_type_hierarchy() {
    let out = emit_with_plugin(
        r"interface BaseBox { value: number; }
interface DerivedBox extends BaseBox { extra: string; }
interface Unrelated { value: number; }

export class Container {
    private hidden: number = 1;
    public visible: number = 2;
    read(b: DerivedBox, u: Unrelated): number {
        return this.hidden + this.visible + b.value + u.value;
    }
}
",
        "type-query-plugin",
        type_query_plugin,
    );
    assert!(out.contains("_priv_hidden = 1;"), "got:\n{out}");
    assert!(out.contains("visible = 2;"), "got:\n{out}");
    assert!(
        out.contains("return this._priv_hidden + this.visible + b.renamedValue + u.value;"),
        "got:\n{out}"
    );
}

// ── Structural rewrites: class members, modifiers, decorators and statement lists ──

/// Adds `static $count = <constructor parameter count>` to `@Tracked` classes, drops that decorator, and wraps
/// every top-level class in `const X = /* @__PURE__ */ ((() => { class X {} return X; })());`. The `before` stage
/// marks the IIFE with a `__PURE__(...)` call; the `after` stage turns it into the annotated parentheses, since
/// the TypeScript transforms rebuild the nodes in between and would drop an early comment.
extern "C" fn structure_plugin(
    host: *const api::Host,
    ctx: *mut c_void,
    file: NodeHandle,
    _: Str,
) -> NodeHandle {
    let a = api_of(host, ctx);
    if a.stage() == stage::AFTER {
        return annotate_pure(a, file);
    }
    let is_tracked = |m: NodeHandle| {
        a.kind(m) == kind::DECORATOR && a.text(a.child(m, field::EXPRESSION)) == "Tracked"
    };
    let mut statements = Vec::new();
    for stmt in a.list(file, field::STATEMENTS) {
        if a.kind(stmt) != kind::CLASS_DECLARATION {
            statements.push(stmt);
            continue;
        }
        let mut class = stmt;
        if a.list(class, field::MODIFIERS).into_iter().any(is_tracked) {
            let params = a
                .list(class, field::MEMBERS)
                .into_iter()
                .find(|&m| a.kind(m) == kind::CONSTRUCTOR)
                .map_or(0, |ctor| a.list_len(ctor, field::PARAMETERS));
            let count = a.property_declaration(
                &[a.token(kind::STATIC_KEYWORD)],
                a.identifier("$count"),
                a.numeric_literal(&params.to_string()),
            );
            let mut members = vec![count];
            members.extend(a.list(class, field::MEMBERS));
            class = a.update_class(class, NodeHandle::NONE, &members);
            class =
                a.visit_each_child(
                    class,
                    |child| if is_tracked(child) { NodeHandle::NONE } else { child },
                );
        }
        let exported =
            a.list(class, field::MODIFIERS).into_iter().any(|m| a.kind(m) == kind::EXPORT_KEYWORD);
        let class = a.visit_each_child(class, |child| {
            if a.kind(child) == kind::EXPORT_KEYWORD { NodeHandle::NONE } else { child }
        });
        let name = a.text(a.child(class, field::NAME));
        let body = a.block(&[class, a.return_statement(a.identifier(name))], true);
        let iife = a.call(a.paren(a.arrow_function(&[], body)), &[]);
        let pure = a.call(a.identifier("__PURE__"), &[iife]);
        let modifiers = if exported { vec![a.token(kind::EXPORT_KEYWORD)] } else { Vec::new() };
        statements.push(a.variable_statement(
            &modifiers,
            node_flags::CONST,
            a.identifier(name),
            pure,
        ));
    }
    a.update_source_file(file, &statements)
}

fn annotate_pure(a: Api<'_>, node: NodeHandle) -> NodeHandle {
    let node = a.visit_each_child(node, |child| annotate_pure(a, child));
    let callee = a.child(node, field::EXPRESSION);
    if a.kind(node) == kind::CALL_EXPRESSION
        && a.kind(callee) == kind::IDENTIFIER
        && a.text(callee) == "__PURE__"
    {
        let paren = a.paren(a.list_at(node, field::ARGUMENTS, 0));
        return a.add_leading_comment(paren, true, " @__PURE__ ", false);
    }
    node
}

#[test]
fn plugin_rebuilds_classes_and_statement_lists() {
    let out = emit_with_plugin(
        r"declare function Tracked(target: unknown): void;
declare function Logged(): ClassDecorator;

@Tracked
export class Service {
    constructor(a: number, b: string) {}
}

@Logged()
class Plain {}
",
        "structure-plugin",
        structure_plugin,
    );
    let expected = r"export const Service = /* @__PURE__ */ ((() => {
    class Service {
        static $count = 2;
        constructor(a, b) { }
    }
    return Service;
})());";
    assert!(out.contains(expected), "got:\n{out}");
    assert!(out.contains("const Plain = /* @__PURE__ */ ((() => {"), "got:\n{out}");
    assert!(out.contains("Plain = __decorate([\n        Logged()\n    ], Plain);"), "got:\n{out}");
    assert!(!out.contains("Tracked"), "got:\n{out}");
}
