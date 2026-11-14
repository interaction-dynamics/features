//! File, line and TODO counts for each feature.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::for_each_feature_mut;
use super::walk::{WalkFilter, is_documentation_directory};
use crate::models::{Feature, Stats};

/// File, line and TODO totals for a feature directory
#[derive(Default)]
struct FileCounts {
    files: usize,
    lines: usize,
    todos: usize,
}

/// Give every feature its `stats`, with file counts filled in. Commit statistics
/// start empty and are added by the history pass.
pub(super) fn populate_file_stats(features: &mut [Feature], base_path: &Path, filter: &WalkFilter) {
    for_each_feature_mut(features, &mut |feature| {
        // Paths of nested features to exclude from file/line counts
        let nested_feature_paths: Vec<String> =
            feature.features.iter().map(|f| f.path.clone()).collect();

        let mut counts = FileCounts::default();
        count_feature_files(
            &base_path.join(&feature.path),
            &nested_feature_paths,
            filter,
            &mut counts,
        );

        feature.stats = Some(Stats {
            files_count: Some(counts.files),
            lines_count: Some(counts.lines),
            todos_count: Some(counts.todos),
            commits: BTreeMap::new(),
            coverage: None,
        });
    });
}

/// Case-insensitive search for "TODO", without allocating
fn contains_todo(line: &str) -> bool {
    line.as_bytes()
        .windows(4)
        .any(|window| window.eq_ignore_ascii_case(b"TODO"))
}

/// Count files, lines and TODO comments in a feature directory (excluding
/// documentation), reading each file once
fn count_feature_files(
    dir: &Path,
    nested_feature_paths: &[String],
    filter: &WalkFilter,
    counts: &mut FileCounts,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip documentation directories
        if is_documentation_directory(&path) {
            continue;
        }

        // Skip nested feature directories
        let path_str = path.to_string_lossy();
        if nested_feature_paths
            .iter()
            .any(|nfp| path_str.starts_with(nfp.as_str()))
        {
            continue;
        }

        if path.is_file() {
            counts.files += 1;

            // Files that aren't text only count towards the number of files
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    counts.lines += 1;
                    if contains_todo(line) {
                        counts.todos += 1;
                    }
                }
            }
        } else if path.is_dir() && !filter.skips(&path) {
            count_feature_files(&path, nested_feature_paths, filter, counts);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_todo() {
        assert!(contains_todo("// TODO: fix"));
        assert!(contains_todo("// todo: fix"));
        assert!(contains_todo("// ToDo"));
        assert!(contains_todo("todo"));
        assert!(contains_todo("écrit // tOdO é"));
        assert!(!contains_todo(""));
        assert!(!contains_todo("tod"));
        assert!(!contains_todo("to do"));
    }

    #[test]
    fn test_count_feature_files() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let base = temp_dir.path();
        fs::create_dir_all(base.join("sub")).unwrap();
        fs::create_dir_all(base.join("docs")).unwrap();
        fs::create_dir_all(base.join("node_modules")).unwrap();
        fs::write(base.join("a.ts"), "// todo one\nline\n// TODO two\n").unwrap();
        fs::write(base.join("sub/b.ts"), "todo\n").unwrap();
        fs::write(base.join("binary"), [0xff, 0xfe]).unwrap();
        fs::write(base.join("docs/doc.md"), "todo\n").unwrap();
        fs::write(base.join("node_modules/c.js"), "todo\n").unwrap();

        let filter = WalkFilter {
            ignore_paths: &[],
            include_skipped_dirs: false,
        };
        let mut counts = FileCounts::default();
        count_feature_files(base, &[], &filter, &mut counts);
        assert_eq!((counts.files, counts.lines, counts.todos), (3, 4, 3));

        let filter = WalkFilter {
            ignore_paths: &[],
            include_skipped_dirs: true,
        };
        let mut counts = FileCounts::default();
        count_feature_files(base, &[], &filter, &mut counts);
        assert_eq!((counts.files, counts.lines, counts.todos), (4, 5, 4));
    }
}
