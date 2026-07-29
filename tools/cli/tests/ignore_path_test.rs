//! Integration tests for the `--ignore-path` scanning option.
//!
//! Verifies that a path passed via `ScanConfig::ignore_paths` (the CLI's
//! `--ignore-path` argument) is excluded from the feature tree along with all
//! of its subfolders, while unrelated features (including ones with the same
//! name at a different path) are left untouched.

use features_cli::scan::{ScanConfig, scan_features};
use std::path::PathBuf;

fn feature_paths(features: &[features_cli::models::Feature], out: &mut Vec<String>) {
    for feature in features {
        out.push(feature.path.clone());
        feature_paths(&feature.features, out);
    }
}

#[test]
fn test_ignore_path_excludes_folder_and_subfolders() {
    let test_path = PathBuf::from("../../examples/tests-skip-changes/src");

    if !test_path.exists() {
        println!("Skipping test - test path does not exist");
        return;
    }

    // Sanity check: without ignoring anything, feature-1 (with its nested
    // feature-2/3/4) and the same-named libs/features/feature-1 both show up.
    let config = ScanConfig::new(&test_path).skip_changes(true);
    let features = scan_features(&test_path, config).expect("scan should succeed");

    let mut all_paths = Vec::new();
    feature_paths(&features, &mut all_paths);

    assert!(all_paths.contains(&"features/feature-1".to_string()));
    assert!(all_paths.contains(&"features/feature-1/features/feature-2".to_string()));
    assert!(all_paths.contains(&"features/feature-1/features/feature-3".to_string()));
    assert!(all_paths.contains(&"features/feature-1/features/feature-4".to_string()));
    assert!(all_paths.contains(&"libs/features/feature-1".to_string()));

    // Now ignore `features/feature-1`.
    let ignored_path = test_path.join("features/feature-1");
    let canonical_ignored_path =
        std::fs::canonicalize(&ignored_path).expect("ignored path should exist");

    let config = ScanConfig::new(&test_path)
        .skip_changes(true)
        .ignore_paths(vec![canonical_ignored_path]);
    let features = scan_features(&test_path, config).expect("scan should succeed");

    let mut ignored_paths = Vec::new();
    feature_paths(&features, &mut ignored_paths);

    // The ignored feature and all of its nested subfolders are gone.
    assert!(!ignored_paths.contains(&"features/feature-1".to_string()));
    assert!(!ignored_paths.contains(&"features/feature-1/features/feature-2".to_string()));
    assert!(!ignored_paths.contains(&"features/feature-1/features/feature-3".to_string()));
    assert!(!ignored_paths.contains(&"features/feature-1/features/feature-4".to_string()));

    // A same-named feature at a different path is unaffected.
    assert!(ignored_paths.contains(&"libs/features/feature-1".to_string()));
    // Unrelated features are unaffected.
    assert!(ignored_paths.contains(&"features/feature-0".to_string()));
    assert!(ignored_paths.contains(&"features/feature-2".to_string()));
}
