//! Integration tests for the folders skipped by default.
//!
//! Verifies that `node_modules` is left out of feature detection, stats and import
//! scanning unless `ScanConfig::include_skipped_dirs` (the CLI's
//! `--include-skipped-dirs` argument) is set, that `--ignore-path` also applies to
//! stats, and that other folders with common build output names are still scanned.

use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn write(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn create_project() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    write(&base.join("src/features/app/index.ts"), "// todo: one\n");
    write(
        &base.join("src/features/app/node_modules/dep/index.js"),
        "import { x } from '../../../other/index';\n// todo: two\n",
    );
    write(
        &base.join("src/features/app/node_modules/dep/features/vendored/index.js"),
        "line\n",
    );
    write(&base.join("src/features/other/index.ts"), "line\n");
    write(
        &base.join("node_modules/pkg/README.md"),
        "---\nfeature: true\n---\n\n# Vendored package\n",
    );

    temp_dir
}

fn feature_paths(features: &[Feature], out: &mut Vec<String>) {
    for feature in features {
        out.push(feature.path.clone());
        feature_paths(&feature.features, out);
    }
}

fn scan(base: &Path, include_skipped_dirs: bool) -> Vec<Feature> {
    let config = ScanConfig::new(base)
        .skip_changes(true)
        .include_skipped_dirs(include_skipped_dirs);
    scan_features(base, config).expect("scan should succeed")
}

fn find<'a>(features: &'a [Feature], path: &str) -> &'a Feature {
    features
        .iter()
        .find(|f| f.path == path)
        .unwrap_or_else(|| panic!("feature {path} should be present"))
}

#[test]
fn test_node_modules_is_skipped_by_default() {
    let temp_dir = create_project();
    let features = scan(temp_dir.path(), false);

    let mut paths = Vec::new();
    feature_paths(&features, &mut paths);
    assert_eq!(paths, vec!["src/features/app", "src/features/other"]);

    let stats = find(&features, "src/features/app").stats.as_ref().unwrap();
    assert_eq!(stats.files_count, Some(1));
    assert_eq!(stats.lines_count, Some(1));
    assert_eq!(stats.todos_count, Some(1));

    assert!(
        find(&features, "src/features/app").dependencies.is_empty(),
        "imports inside node_modules should not create dependencies"
    );
}

#[test]
fn test_include_skipped_dirs_preserves_node_modules() {
    let temp_dir = create_project();
    let features = scan(temp_dir.path(), true);

    let mut paths = Vec::new();
    feature_paths(&features, &mut paths);
    assert!(paths.contains(&"node_modules/pkg".to_string()));
    assert!(
        paths.contains(&"src/features/app/node_modules/dep/features/vendored".to_string()),
        "got {paths:?}"
    );

    let stats = find(&features, "src/features/app").stats.as_ref().unwrap();
    assert_eq!(stats.files_count, Some(3));
    assert_eq!(stats.lines_count, Some(4));
    assert_eq!(stats.todos_count, Some(2));
}

#[test]
fn test_ownership_only_scan_skips_node_modules() {
    let temp_dir = create_project();
    let base = temp_dir.path();

    let scan_ownership = |include_skipped_dirs| {
        let config = ScanConfig::new(base)
            .ownership_only(true)
            .include_skipped_dirs(include_skipped_dirs);
        let features = scan_features(base, config).expect("scan should succeed");
        let mut paths = Vec::new();
        feature_paths(&features, &mut paths);
        paths
    };

    assert!(!scan_ownership(false).contains(&"node_modules/pkg".to_string()));
    assert!(scan_ownership(true).contains(&"node_modules/pkg".to_string()));
}

/// Only `SKIPPED_DIRS` are skipped: a feature can be named like common build output.
#[test]
fn test_features_named_like_build_output_are_kept() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    for name in ["build", "dist", "target", "vendor", "coverage"] {
        write(&base.join("features").join(name).join("index.ts"), "line\n");
    }

    let features = scan(base, false);

    let mut paths = Vec::new();
    feature_paths(&features, &mut paths);
    assert_eq!(
        paths,
        vec![
            "features/build",
            "features/coverage",
            "features/dist",
            "features/target",
            "features/vendor",
        ]
    );
    let stats = find(&features, "features/build").stats.as_ref().unwrap();
    assert_eq!(stats.files_count, Some(1));
}

#[test]
fn test_ignore_path_is_left_out_of_stats_and_imports() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    write(&base.join("features/app/index.ts"), "line\n");
    write(
        &base.join("features/app/generated/out.ts"),
        "import { x } from '../../other/index';\nline\n",
    );
    write(&base.join("features/other/index.ts"), "line\n");

    let config = ScanConfig::new(base).skip_changes(true);
    let features = scan_features(base, config).expect("scan should succeed");
    let app = find(&features, "features/app");
    assert_eq!(app.stats.as_ref().unwrap().files_count, Some(2));
    assert_eq!(app.dependencies.len(), 1);

    let ignored = fs::canonicalize(base.join("features/app/generated")).unwrap();
    let config = ScanConfig::new(base)
        .skip_changes(true)
        .ignore_paths(vec![ignored]);
    let features = scan_features(base, config).expect("scan should succeed");
    let app = find(&features, "features/app");
    assert_eq!(app.stats.as_ref().unwrap().files_count, Some(1));
    assert_eq!(app.stats.as_ref().unwrap().lines_count, Some(1));
    assert!(app.dependencies.is_empty());
}
