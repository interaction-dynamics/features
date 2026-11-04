//! Integration tests for the ownership-only scan mode used by `--generate-codeowners`.
//!
//! Verifies that `ScanConfig::ownership_only` produces the same feature tree
//! (names, owners, paths, nesting) as a full scan, while skipping the expensive
//! data that CODEOWNERS generation never uses.

use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use std::path::PathBuf;

fn ownership_tree(features: &[Feature], out: &mut Vec<(String, String, String, bool)>) {
    for feature in features {
        out.push((
            feature.path.clone(),
            feature.name.clone(),
            feature.owner.clone(),
            feature.is_owner_inherited,
        ));
        ownership_tree(&feature.features, out);
    }
}

fn assert_no_details(features: &[Feature]) {
    for feature in features {
        assert!(feature.changes.is_empty(), "{} has changes", feature.path);
        assert!(
            feature.decisions.is_empty(),
            "{} has decisions",
            feature.path
        );
        assert!(feature.stats.is_none(), "{} has stats", feature.path);
        assert!(
            feature.dependencies.is_empty(),
            "{} has dependencies",
            feature.path
        );
        assert_no_details(&feature.features);
    }
}

#[test]
fn test_ownership_only_matches_full_scan_ownership() {
    let test_path = PathBuf::from("../../examples/tests-with-changes/src");

    if !test_path.exists() {
        println!("Skipping test - test path does not exist");
        return;
    }

    let full = scan_features(&test_path, ScanConfig::new(&test_path)).expect("full scan");
    let light = scan_features(&test_path, ScanConfig::new(&test_path).ownership_only(true))
        .expect("ownership-only scan");

    let mut full_tree = Vec::new();
    ownership_tree(&full, &mut full_tree);
    let mut light_tree = Vec::new();
    ownership_tree(&light, &mut light_tree);

    assert!(!full_tree.is_empty(), "Should find at least one feature");
    assert_eq!(full_tree, light_tree);
}

#[test]
fn test_ownership_only_skips_details() {
    let test_path = PathBuf::from("../../examples/tests-with-changes/src");

    if !test_path.exists() {
        println!("Skipping test - test path does not exist");
        return;
    }

    let config = ScanConfig::new(&test_path)
        .ownership_only(true)
        .with_coverage(true);
    let features = scan_features(&test_path, config).expect("ownership-only scan");

    assert!(!features.is_empty(), "Should find at least one feature");
    assert_no_details(&features);
}

#[test]
fn test_ownership_only_respects_ignore_path() {
    let test_path = PathBuf::from("../../examples/tests-skip-changes/src");

    if !test_path.exists() {
        println!("Skipping test - test path does not exist");
        return;
    }

    let ignored_path = std::fs::canonicalize(test_path.join("features/feature-1"))
        .expect("ignored path should exist");
    let config = ScanConfig::new(&test_path)
        .ownership_only(true)
        .ignore_paths(vec![ignored_path]);
    let features = scan_features(&test_path, config).expect("ownership-only scan");

    let mut tree = Vec::new();
    ownership_tree(&features, &mut tree);
    let paths: Vec<&str> = tree.iter().map(|(path, ..)| path.as_str()).collect();

    assert!(!paths.iter().any(|p| p.starts_with("features/feature-1")));
    assert!(paths.contains(&"libs/features/feature-1"));
}
