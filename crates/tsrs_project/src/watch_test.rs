use std::sync::Arc;

use tsrs_core::collections::SyncMap;
use tsrs_core::tspath::{self, Path};

use super::create_resolution_lookup_glob_mapper;

fn watched_files(file_names: &[&str], case_sensitive: bool) -> Arc<SyncMap<Path, String>> {
    let files = Arc::new(SyncMap::default());
    for file_name in file_names {
        files.store(tspath::to_path(file_name, "/", case_sensitive), (*file_name).to_string());
    }
    files
}

// watch_test.go:31 TestResolutionLookupWatcherPreservesIncludedDirectorySpelling
#[test]
fn resolution_lookup_watcher_preserves_included_directory_spelling() {
    let files = Some(watched_files(
        &["/Workspace/src/index.ts", "/Project/src/index.ts", "/Lib/lib.d.ts"],
        false,
    ));
    let patterns =
        create_resolution_lookup_glob_mapper("/Workspace", "/Lib", "/Project", false)(&files)
            .patterns_inside_workspace;
    assert_eq!(patterns, ["/Workspace/**/*", "/Project/**/*", "/Lib/**/*"]);
}

// watch_test.go:54 TestResolutionLookupWatcherPreservesNodeModulesSpelling
#[test]
fn resolution_lookup_watcher_preserves_node_modules_spelling() {
    let files = Some(watched_files(&["/External/Node_Modules/pkg/index.ts"], false));
    let patterns =
        create_resolution_lookup_glob_mapper("/Workspace", "/Lib", "/Project", false)(&files)
            .patterns_inside_workspace;
    assert_eq!(patterns, ["/External/Node_Modules/**/*"]);
}

// watch_test.go:70 TestResolutionLookupWatcherAggregatesUsingHostCaseSensitivity
#[test]
fn resolution_lookup_watcher_aggregates_using_host_case_sensitivity() {
    let file_names = ["/External/Lib/src/a.ts", "/external/LIB/test/b.ts"];
    for case_sensitive in [true, false] {
        let files = Some(watched_files(&file_names, case_sensitive));
        let directories =
            create_resolution_lookup_glob_mapper("/Workspace", "/Lib", "/Project", case_sensitive)(
                &files,
            )
            .directories_outside_workspace;
        if case_sensitive {
            assert_eq!(directories, ["/External/Lib/src", "/external/LIB/test"]);
        } else {
            assert_eq!(directories.len(), 1);
            assert!(directories[0] == "/External/Lib" || directories[0] == "/external/LIB");
        }
    }
}
