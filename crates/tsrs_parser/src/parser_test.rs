use tsrs_ast::{Kind, NodeFlags, SourceFile, SourceFileParseOptions, SubtreeFacts};
use tsrs_core::{P, ScriptKind};

use crate::parse_source_file;

fn parse(source: &str) -> P<SourceFile> {
    parse_source_file(
        SourceFileParseOptions { file_name: "/a.ts".to_string(), ..Default::default() },
        source,
        ScriptKind::TS,
    )
}

#[test]
fn test_parse_static_source_phase_import() {
    for (source, phase, binding) in [
        ("import source a from './a.wasm';", Kind::SourceKeyword, "a"),
        ("import source from from './a.js';", Kind::SourceKeyword, "from"),
        ("import source from './a.js';", Kind::Unknown, "source"),
        ("import s\\u006furce from './a.js';", Kind::Unknown, "source"),
    ] {
        let file = parse(source);
        assert!(file.diagnostics().is_empty(), "{source}");
        let statements = file.statements().nodes();
        assert_eq!(statements.len(), 1, "{source}");
        let import = statements[0].as_import_declaration();
        assert_eq!(tsrs_ast::is_source_phase_import(statements[0]), phase == Kind::SourceKeyword);
        let clause = import.import_clause().unwrap().as_import_clause();
        assert_eq!(clause.phase_modifier(), phase, "{source}");
        assert_eq!(clause.name().unwrap().text(), binding, "{source}");
    }
}

#[test]
fn test_parse_invalid_static_source_phase_import_shape() {
    for (source, has_name, has_named_bindings) in [
        ("import source './a.js';", false, false),
        ("import source * as a from './a.js';", false, true),
        ("import source { a } from './a.js';", false, true),
        ("import source a, { b } from './a.js';", true, true),
    ] {
        let file = parse(source);
        assert!(file.diagnostics().is_empty(), "{source}");
        let import = file.statements().nodes()[0].as_import_declaration();
        let clause = import.import_clause().unwrap().as_import_clause();
        assert_eq!(clause.phase_modifier(), Kind::SourceKeyword, "{source}");
        assert_eq!(clause.name().is_some(), has_name, "{source}");
        assert_eq!(clause.named_bindings().is_some(), has_named_bindings, "{source}");
    }
}

#[test]
fn test_parse_source_as_import_equals_binding() {
    let file = parse("import source = require('./a.js');");
    assert_eq!(file.diagnostics().len(), 0);
    let statement = file.statements().nodes()[0];
    assert_eq!(statement.kind(), Kind::ImportEqualsDeclaration);
    assert_eq!(statement.name().unwrap().text(), "source");
}

#[test]
fn test_parse_dynamic_source_phase_import() {
    let file = parse("import.source('./a.wasm', { with: { type: 'webassembly' } });");
    assert_eq!(file.diagnostics().len(), 0);
    let statement = file.statements().nodes()[0];
    let call = statement.as_expression_statement().expression();
    assert!(tsrs_ast::is_import_call(call));
    assert!(tsrs_ast::is_source_phase_import_call(call));
    assert!(tsrs_ast::is_source_phase_import(call));
    assert!(call.subtree_facts().contains(SubtreeFacts::ContainsDynamicImport));
    assert!(file.as_node().flags().contains(NodeFlags::PossiblyContainsDynamicImport));
    assert!(!file.as_node().flags().contains(NodeFlags::PossiblyContainsImportMeta));
    assert!(file.external_module_indicator.get().is_none());
}

#[test]
fn test_parse_escaped_dynamic_import_phase() {
    for source in ["import.d\\u0065fer('./a.js');", "import.s\\u006furce('./a.wasm');"] {
        let file = parse(source);
        assert_eq!(file.diagnostics().len(), 1, "{source}");
        let call = file.statements().nodes()[0].as_expression_statement().expression();
        assert!(tsrs_ast::is_import_call(call), "{source}");
        assert!(file.as_node().flags().contains(NodeFlags::PossiblyContainsDynamicImport));
        assert!(!file.as_node().flags().contains(NodeFlags::PossiblyContainsImportMeta));
    }
}
