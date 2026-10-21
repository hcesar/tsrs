//! Example native transformer plugin for `tsrs`, loaded from `tsconfig.json`:
//!
//! ```json
//! { "compilerOptions": { "plugins": [{ "transform": "./libtsrs_example_plugin.dylib" }] } }
//! ```
//!
//! 1. Replaces the identifier `__FILE_NAME__` with a string literal of the current file's path.
//! 2. Rewrites `debugLog(x)` into `/*#__PURE__*/ __trace(x)` and prepends `import { __trace } from "tsrs-runtime";`.
//! 3. Replaces `typeLabel(x)` with `"array"` when the static type of `x` is an array, `"value"` otherwise.

use std::ffi::c_void;

use tsrs_plugin_api::{Api, Host, ImportSpec, NodeHandle, Str, field, kind, stage};

struct State<'a> {
    api: Api<'a>,
    file_name: &'a str,
    used_trace: bool,
}

impl State<'_> {
    fn visit(&mut self, node: NodeHandle) -> NodeHandle {
        let a = self.api;
        match a.kind(node) {
            kind::CALL_EXPRESSION => {
                let callee = a.child(node, field::EXPRESSION);
                let args = a.list(node, field::ARGUMENTS);
                if a.kind(callee) == kind::IDENTIFIER && args.len() == 1 {
                    match a.text(callee) {
                        "debugLog" => {
                            let arg = self.visit(args[0]);
                            let call = a.call(a.identifier("__trace"), &[arg]);
                            a.add_leading_comment(call, true, "#__PURE__", false);
                            self.used_trace = true;
                            return a.set_original(call, node);
                        }
                        "typeLabel" => {
                            let is_array = a
                                .type_at(args[0])
                                .is_some_and(|t| a.is_subtype_of_named(t, "Array"));
                            let label = a.string_literal(if is_array { "array" } else { "value" });
                            return a.set_original(label, node);
                        }
                        _ => {}
                    }
                }
            }
            kind::IDENTIFIER if a.text(node) == "__FILE_NAME__" => {
                return a.set_original(a.string_literal(self.file_name), node);
            }
            _ => {}
        }
        a.visit_each_child(node, |child| self.visit(child))
    }
}

/// # Safety
/// Called by `tsrs` with its host table and call context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tsrs_plugin_transform(
    host: *const Host,
    ctx: *mut c_void,
    source_file: NodeHandle,
    _plugin_name: Str,
) -> NodeHandle {
    // SAFETY: `host` and `ctx` are the arguments of this call.
    let Some(api) = (unsafe { Api::new(host, ctx) }) else {
        return source_file;
    };
    if api.stage() != stage::BEFORE {
        return source_file;
    }
    let mut state = State { api, file_name: api.source_file_name(source_file), used_trace: false };
    let file = api.visit_each_child(source_file, |child| state.visit(child));
    if !state.used_trace {
        return file;
    }
    let spec =
        ImportSpec { property_name: Str::EMPTY, name: Str::new("__trace"), is_type_only: false };
    let mut statements = vec![api.import_declaration("tsrs-runtime", &[spec])];
    statements.extend(api.list(file, field::STATEMENTS));
    api.update_source_file(file, &statements)
}
