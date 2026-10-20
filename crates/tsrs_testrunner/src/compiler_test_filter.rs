// internal/testrunner/compiler_test_filter.go: skip unrelated compiler fixtures before reading them.
// The Rust runner's --filter is a substring OR a regex over `suite/stem(config)`, unlike Go's
// test.run pattern. Only a plain filter with an ASCII uppercase letter can be ruled out by the
// filename: configuration descriptions are lowercased, and ambiguous patterns read the file.

use rustc_hash::FxHashSet;

use crate::compiler_runner::baseline_stem;

pub struct CompilerTestFileFilter<'a> {
    suite: &'a str,
    literal: Option<&'a str>,
    list: Option<&'a FxHashSet<String>>,
    sorted_list: Vec<&'a str>,
}

impl<'a> CompilerTestFileFilter<'a> {
    pub fn new(
        suite: &'a str,
        filter: Option<&'a str>,
        list: Option<&'a FxHashSet<String>>,
    ) -> Self {
        let literal = filter.filter(|pattern| {
            pattern.bytes().any(|byte| byte.is_ascii_uppercase())
                && pattern
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'/'))
        });
        let mut sorted_list: Vec<&str> =
            list.into_iter().flat_map(|entries| entries.iter().map(String::as_str)).collect();
        sorted_list.sort_unstable();
        Self { suite, literal, list, sorted_list }
    }

    pub fn includes(&self, basename: &str) -> bool {
        if self.literal.is_none() && self.list.is_none() {
            return true;
        }
        let stem = baseline_stem(basename);
        let id = format!("{}/{stem}", self.suite);

        if let Some(literal) = self.literal {
            // The regex and substring branches have the same meaning for this literal. A match
            // cannot live wholly in the lowercase configuration or cross its '(' delimiter.
            if !id.contains(literal) {
                return false;
            }
        }

        if let Some(list) = self.list {
            let config_name_prefix = format!("{stem}(");
            let config_id_prefix = format!("{id}(");
            if !list.contains(basename)
                && !list.contains(stem.as_str())
                && !list.contains(&id)
                && !self.has_list_prefix(&config_name_prefix)
                && !self.has_list_prefix(&config_id_prefix)
            {
                return false;
            }
        }

        true
    }

    fn has_list_prefix(&self, prefix: &str) -> bool {
        let index = self.sorted_list.partition_point(|&entry| entry < prefix);
        self.sorted_list.get(index).is_some_and(|entry| entry.starts_with(prefix))
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashSet;

    use super::CompilerTestFileFilter;

    #[test]
    fn filters_plain_uppercase_names_before_expansion() {
        let filter =
            CompilerTestFileFilter::new("compiler", Some("compiler/FlakyDiagnostic"), None);
        assert!(filter.includes("FlakyDiagnostic1038.ts"));
        assert!(!filter.includes("other.ts"));
        assert!(filter.includes("FlakyDiagnostic1038.tsx"));
        assert!(
            !CompilerTestFileFilter::new("conformance", Some("compiler/FlakyDiagnostic"), None)
                .includes("FlakyDiagnostic1038.ts")
        );
    }

    #[test]
    fn ambiguous_patterns_fall_back_to_reading_the_file() {
        for pattern in [
            "target",
            "^compiler/lowercase",
            "compiler/Example\\.ts",
            "compiler/(Example|Other)",
            "compiler/Example(",
            "compiler/Example Name",
        ] {
            assert!(
                CompilerTestFileFilter::new("compiler", Some(pattern), None).includes("other.ts"),
                "{pattern}"
            );
        }
    }

    #[test]
    fn list_keeps_file_and_configuration_names() {
        for entry in [
            "Example.ts",
            "Example",
            "compiler/Example",
            "Example(target=es2020)",
            "compiler/Example(target=es2020)",
        ] {
            let list = FxHashSet::from_iter([entry.to_string()]);
            let filter = CompilerTestFileFilter::new("compiler", None, Some(&list));
            assert!(filter.includes("Example.ts"), "{entry}");
            assert!(!filter.includes("other.ts"), "{entry}");
        }
    }
}
