// End to end: the `tsrs` command line (in process) loads a plugin library named by tsconfig `plugins[].transform`.

use std::path::{Path, PathBuf};
use std::process::Command;

use tsrs_execute::{execute, sys};

fn dylib_filename(crate_name: &str) -> String {
    if cfg!(target_os = "macos") {
        format!("lib{crate_name}.dylib")
    } else if cfg!(target_os = "windows") {
        format!("{crate_name}.dll")
    } else {
        format!("lib{crate_name}.so")
    }
}

fn build_example_plugin(scratch_dir: &Path) -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.parent().unwrap().parent().unwrap();
    let plugin_manifest = repo_root.join("examples/transform_plugin/Cargo.toml");
    let target_dir = scratch_dir.join("plugin-target");
    let dylib = target_dir.join("debug").join(dylib_filename("tsrs_example_plugin"));
    let status = Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(&plugin_manifest)
        .arg("--target-dir")
        .arg(&target_dir)
        .status()
        .unwrap();
    assert!(status.success(), "failed to build tsrs_example_plugin");
    assert!(dylib.exists(), "expected plugin dylib at {}", dylib.display());
    dylib
}

#[test]
fn tsconfig_plugin_transforms_ast_and_types() {
    let dir = std::env::temp_dir().join(format!("tsrs-plugin-emit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    let dylib = build_example_plugin(&dir);

    let tsconfig = format!(
        r#"{{
  "compilerOptions": {{
    "target": "esnext",
    "module": "esnext",
    "rootDir": "src",
    "outDir": "out",
    "plugins": [
      {{
        "name": "example-plugin",
        "transform": "{}"
      }}
    ]
  }},
  "include": ["src"]
}}"#,
        dylib.to_string_lossy().replace('\\', "/")
    );
    std::fs::write(dir.join("tsconfig.json"), tsconfig).unwrap();

    let source = r"declare function debugLog<T>(value: T): T;
declare function typeLabel(value: unknown): string;
declare const __FILE_NAME__: string;

export function run(items: number[], count: number): string {
    debugLog(__FILE_NAME__);
    return typeLabel(items) + ':' + typeLabel(count);
}
";
    std::fs::write(dir.join("src/index.ts"), source).unwrap();

    let system: &'static sys::OsSys = Box::leak(Box::new(sys::new_system()));
    let config = dir.join("tsconfig.json").to_string_lossy().replace('\\', "/");
    let result = execute::command_line(system, vec!["-p".to_owned(), config]);
    assert_eq!(result.status as i32, 0, "tsrs -p failed");

    let js = std::fs::read_to_string(dir.join("out/index.js")).unwrap();
    assert!(
        js.contains(r#"import { __trace } from "tsrs-runtime";"#),
        "expected injected tsrs-runtime import, got:\n{js}"
    );
    assert!(
        js.contains("/*#__PURE__*/ __trace(") && js.contains("src/index.ts"),
        "expected debugLog(__FILE_NAME__) rewrite with synthetic comment, got:\n{js}"
    );
    assert!(
        js.contains(r#"return "array" + ':' + "value";"#),
        "expected typeLabel(items) -> \"array\" and typeLabel(count) -> \"value\", got:\n{js}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
