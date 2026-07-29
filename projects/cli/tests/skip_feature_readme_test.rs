//! Integration tests for skipping a feature via the README `feature: false` property.
//!
//! Unlike `--ignore-path`, which excludes a folder *and all of its subfolders*
//! from scanning, marking a feature's README with `feature: false` only skips
//! that feature itself: any nested features it contains (via a `features/`
//! subfolder or their own `feature: true` README) are still discovered.

use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use std::fs;
use tempfile::TempDir;

fn write_readme(dir: &std::path::Path, content: &str) {
    fs::write(dir.join("README.md"), content).unwrap();
}

fn feature_names(features: &[Feature], out: &mut Vec<String>) {
    for feature in features {
        out.push(feature.name.clone());
        feature_names(&feature.features, out);
    }
}

#[test]
fn test_feature_false_skips_direct_subfolder_of_features_but_keeps_nested_features_folder() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let skipped_dir = base.join("features").join("skipped-feature");
    fs::create_dir_all(&skipped_dir).unwrap();
    write_readme(&skipped_dir, "---\nfeature: false\n---\n\n# Skipped\n");

    // Nested feature living inside the skipped feature's own `features/` folder.
    let nested_dir = skipped_dir.join("features").join("nested-feature");
    fs::create_dir_all(&nested_dir).unwrap();

    let config = ScanConfig::new(base).skip_changes(true);
    let features = scan_features(base, config).expect("scan should succeed");

    let mut names = Vec::new();
    feature_names(&features, &mut names);

    assert!(!names.contains(&"skipped-feature".to_string()));
    assert!(names.contains(&"nested-feature".to_string()));
}

#[test]
fn test_feature_false_skips_feature_but_keeps_readme_marked_sub_children() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let skipped_dir = base.join("features").join("skipped-feature");
    fs::create_dir_all(&skipped_dir).unwrap();
    write_readme(
        &skipped_dir,
        "---\nfeature: false\nowner: skipped-owner\n---\n\n# Skipped\n",
    );

    // A subfolder that isn't inside a `features/` folder, but declares itself
    // a feature via its own README.
    let child_dir = skipped_dir.join("child");
    fs::create_dir_all(&child_dir).unwrap();
    write_readme(
        &child_dir,
        "---\nfeature: true\nowner: child-owner\n---\n\n# Child\n",
    );

    let config = ScanConfig::new(base).skip_changes(true);
    let features = scan_features(base, config).expect("scan should succeed");

    let mut names = Vec::new();
    feature_names(&features, &mut names);

    assert!(!names.contains(&"Skipped".to_string()));
    let child = features
        .iter()
        .find(|f| f.name == "Child")
        .expect("child feature should still be discovered");
    assert_eq!(child.owner, "child-owner");
}

#[test]
fn test_feature_true_direct_subfolder_still_included_by_default() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let feature_dir = base.join("features").join("regular-feature");
    fs::create_dir_all(&feature_dir).unwrap();

    let config = ScanConfig::new(base).skip_changes(true);
    let features = scan_features(base, config).expect("scan should succeed");

    let mut names = Vec::new();
    feature_names(&features, &mut names);

    // Sanity check: a direct subfolder of `features` with no README is still
    // treated as a feature (existing behavior), confirming that the skip in
    // the tests above is due to `feature: false` and not some other cause.
    assert!(names.contains(&"regular-feature".to_string()));
}
