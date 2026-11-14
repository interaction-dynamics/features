//! Feature discovery.
//!
//! A scan first walks the project to build the feature tree ([`walk`]), then
//! enriches it in separate passes, the same way `scan.rs` adds coverage:
//!
//! 1. [`metadata`]: `--feature-<key>` annotations merged into `meta`
//! 2. [`decisions`]: decision records (raw markdown) and
//!    [`crate::records`]: specs and decision records, root ones included
//! 3. [`stats`]: file, line and TODO counts
//! 4. [`history`]: git changes and commit statistics (only with history)
//! 5. [`dependencies`]: dependencies resolved from imports
//!
//! An ownership-only scan stops after the walk.

mod decisions;
mod dependencies;
mod history;
mod metadata;
mod stats;
mod walk;

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::feature_metadata_detector;
use crate::git_helper::{CommitHistory, get_all_commits_by_path};
use crate::models::{Feature, Record};
use crate::records::populate_records;
use walk::{ReadmeCache, WalkFilter, Walker};

/// Call `f` on every feature of the tree, parents before their nested features
fn for_each_feature_mut(features: &mut [Feature], f: &mut impl FnMut(&mut Feature)) {
    for feature in features {
        f(feature);
        for_each_feature_mut(&mut feature.features, f);
    }
}

/// Scan `dir` for features with everything but git history
pub fn list_files_recursive(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
) -> Result<Vec<Feature>> {
    scan_with_enrichment(dir, ignore_paths, include_skipped_dirs, None).map(|(f, _)| f)
}

/// Scan `dir` for features and project-level records (root specs and
/// decisions not attached to a feature), with or without git history
pub fn scan_features_and_records(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
    with_history: bool,
) -> Result<(Vec<Feature>, Vec<Record>)> {
    if with_history {
        let history = get_all_commits_by_path(dir).unwrap_or_default();
        scan_with_enrichment(dir, ignore_paths, include_skipped_dirs, Some(&history))
    } else {
        scan_with_enrichment(dir, ignore_paths, include_skipped_dirs, None)
    }
}

/// Build the feature tree with only what is needed to know who owns what:
/// name, owner, path, meta from README/FEATURES.toml and nested features.
///
/// Skips everything that reads source files or git history (changes, stats,
/// decisions, feature metadata comments and dependencies), which makes it
/// suitable for CODEOWNERS generation on large repositories.
pub fn list_features_ownership_only(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
) -> Result<Vec<Feature>> {
    let readmes = ReadmeCache::default();
    let walker = Walker {
        base_path: dir,
        filter: WalkFilter {
            ignore_paths,
            include_skipped_dirs,
        },
        readmes: &readmes,
    };
    walker.list_features(dir, None)
}

/// Scan `dir` for features, git history included
pub fn list_files_recursive_with_changes(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
) -> Result<Vec<Feature>> {
    scan_features_and_records(dir, ignore_paths, include_skipped_dirs, true).map(|(f, _)| f)
}

/// Walk `dir` for the feature tree, then run every enrichment pass on it.
/// Returns the features and the project-level records.
fn scan_with_enrichment(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
    history: Option<&CommitHistory>,
) -> Result<(Vec<Feature>, Vec<Record>)> {
    // Scan entire base_path for feature metadata once
    let feature_metadata =
        feature_metadata_detector::scan_directory_for_feature_metadata(dir).unwrap_or_default();

    let filter = WalkFilter {
        ignore_paths,
        include_skipped_dirs,
    };
    let readmes = ReadmeCache::default();
    let walker = Walker {
        base_path: dir,
        filter,
        readmes: &readmes,
    };
    let mut features = walker.list_features(dir, None)?;

    metadata::apply_feature_metadata(&mut features, &feature_metadata);
    decisions::populate_decisions(&mut features, dir);
    let project_records = populate_records(&mut features, dir);
    stats::populate_file_stats(&mut features, dir, &filter);
    if let Some(history) = history {
        history::populate_history(&mut features, dir, history);
    }
    dependencies::populate_dependencies(&mut features, dir, &filter, &readmes)?;

    Ok((features, project_records))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_readme(dir: &Path, content: &str) {
        fs::write(dir.join("README.md"), content).unwrap();
    }

    /// Skipping a feature via `feature: false` must keep its sub-children discoverable:
    /// both a nested `features/` folder and a subfolder with its own `feature: true` README.
    #[test]
    fn test_list_files_recursive_skips_feature_but_keeps_sub_children() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let base = temp_dir.path();

        let features_dir = base.join("features");
        let skipped_dir = features_dir.join("skipped-feature");
        fs::create_dir_all(&skipped_dir).unwrap();
        write_readme(
            &skipped_dir,
            "---\nfeature: false\nowner: skipped-owner\n---\n\n# Skipped Feature\n",
        );

        // A nested `features` folder inside the skipped directory.
        let nested_via_features_folder = skipped_dir.join("features").join("child-a");
        fs::create_dir_all(&nested_via_features_folder).unwrap();

        // A subfolder marked as a feature via its own README.
        let nested_via_readme = skipped_dir.join("child-b");
        fs::create_dir_all(&nested_via_readme).unwrap();
        write_readme(
            &nested_via_readme,
            "---\nfeature: true\nowner: child-b-owner\n---\n\n# Child B\n",
        );

        let features = list_files_recursive(base, &[], false).unwrap();

        // The skipped feature itself must not appear anywhere in the tree.
        fn collect_names(features: &[Feature], out: &mut Vec<String>) {
            for feature in features {
                out.push(feature.name.clone());
                collect_names(&feature.features, out);
            }
        }
        let mut names = Vec::new();
        collect_names(&features, &mut names);
        assert!(!names.contains(&"skipped-feature".to_string()));
        assert!(!names.contains(&"Skipped Feature".to_string()));

        // Both sub-children are still discovered, as top-level features (since their
        // skipped parent no longer exists as a node in the tree).
        assert!(names.contains(&"child-a".to_string()));
        let child_b = features
            .iter()
            .find(|f| f.name == "Child B")
            .expect("child-b feature should be present");
        assert_eq!(child_b.owner, "child-b-owner");
    }

    #[test]
    fn test_enrichment_passes_fill_every_feature() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let base = temp_dir.path();
        let parent = base.join("features").join("parent");
        let child = parent.join("features").join("child");
        fs::create_dir_all(child.join("__docs__").join("decisions")).unwrap();
        fs::write(parent.join("a.ts"), "// TODO one\n").unwrap();
        fs::write(child.join("b.ts"), "line\nline\n").unwrap();
        fs::write(
            child.join("__docs__").join("decisions").join("001.md"),
            "# Decision\n",
        )
        .unwrap();

        let features = list_files_recursive(base, &[], false).unwrap();
        let parent = &features[0];
        let child = &parent.features[0];

        let parent_stats = parent.stats.as_ref().unwrap();
        assert_eq!(parent_stats.todos_count, Some(1));
        assert!(parent_stats.commits.is_empty());
        assert_eq!(child.decisions, vec!["# Decision\n".to_string()]);
        assert_eq!(child.records.len(), 1);
        assert_eq!(child.records[0].title, "Decision");
        assert_eq!(child.stats.as_ref().unwrap().lines_count, Some(2));

        // An ownership-only scan leaves everything but the tree out
        let owners = list_features_ownership_only(base, &[], false).unwrap();
        assert!(owners[0].stats.is_none());
        assert!(owners[0].features[0].decisions.is_empty());
        assert!(owners[0].features[0].records.is_empty());
    }
}
