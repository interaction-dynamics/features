//! Integration tests for file_scanner module
//!
//! This test suite verifies that the file scanner correctly identifies and parses
//! README files in direct subfolders of a "features" directory, including nested
//! features within feature folders.
//!
//! The test uses snapshot comparison with recursive validation to ensure the output
//! matches expected results at all nesting levels.

use features_cli::file_scanner::{list_files_recursive, list_files_recursive_with_changes};
use features_cli::models::Feature;
use std::path::PathBuf;

/// Recursively compares two features including their nested features.
///
/// This function validates all fields of a feature and then recursively
/// validates any nested features in the `features` field. The `parent_path`
/// parameter is used to build a hierarchical path for error messages,
/// making it easier to identify which nested feature failed the comparison.
///
/// # Arguments
///
/// * `actual` - The actual feature from the scanner
/// * `expected` - The expected feature from the snapshot
/// * `parent_path` - Hierarchical path to this feature (empty for root features)
fn compare_features_recursive(actual: &Feature, expected: &Feature, parent_path: &str) {
    let feature_path = if parent_path.is_empty() {
        actual.name.clone()
    } else {
        format!("{} -> {}", parent_path, actual.name)
    };

    assert_eq!(
        actual.name, expected.name,
        "Feature name mismatch at '{}'",
        feature_path
    );
    assert_eq!(
        actual.description, expected.description,
        "Feature '{}' description mismatch",
        feature_path
    );
    assert_eq!(
        actual.owner, expected.owner,
        "Feature '{}' owner mismatch",
        feature_path
    );
    assert!(
        actual
            .path
            .ends_with(&expected.path.split('/').next_back().unwrap()),
        "Feature '{}' path mismatch. Expected to end with '{}', got '{}'",
        feature_path,
        expected.path.split('/').next_back().unwrap(),
        actual.path
    );
    assert_eq!(
        actual.features.len(),
        expected.features.len(),
        "Feature '{}' should have {} nested features, got {}",
        feature_path,
        expected.features.len(),
        actual.features.len()
    );
    assert_eq!(
        actual.meta, expected.meta,
        "Feature '{}' meta mismatch",
        feature_path
    );
    assert_eq!(
        actual.changes.len(),
        expected.changes.len(),
        "Feature '{}' should have {} changes, got {}",
        feature_path,
        expected.changes.len(),
        actual.changes.len()
    );
    assert_eq!(
        actual.decisions.len(),
        expected.decisions.len(),
        "Feature '{}' should have {} decisions, got {}",
        feature_path,
        expected.decisions.len(),
        actual.decisions.len()
    );

    // Recursively compare nested features
    for (actual_nested, expected_nested) in actual.features.iter().zip(expected.features.iter()) {
        compare_features_recursive(actual_nested, expected_nested, &feature_path);
    }
}

#[test]
fn test_javascript_basic_snapshot() {
    // Path to the test directory (relative to the workspace root when tests run)
    let test_path = PathBuf::from("../../examples/javascript-basic/src");

    if !test_path.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_path.display()
        );
        return;
    }

    // Scan the directory
    let result = list_files_recursive_with_changes(&test_path);
    assert!(
        result.is_ok(),
        "Failed to scan directory: {:?}",
        result.err()
    );

    let actual_features = result.unwrap();

    // Load the snapshot
    let snapshot_path = PathBuf::from("tests/snapshots/javascript_basic_features.json");
    let snapshot_content = std::fs::read_to_string(&snapshot_path)
        .expect("Failed to read snapshot file. Make sure tests/snapshots/javascript_basic_features.json exists");

    let expected_features: Vec<Feature> =
        serde_json::from_str(&snapshot_content).expect("Failed to parse snapshot JSON");

    // Compare the results
    assert_eq!(
        actual_features.len(),
        expected_features.len(),
        "Number of features doesn't match snapshot. Expected: {}, Got: {}",
        expected_features.len(),
        actual_features.len()
    );

    // Compare each feature recursively
    for (actual, expected) in actual_features.iter().zip(expected_features.iter()) {
        compare_features_recursive(actual, expected, "");
    }

    // Optional: Print JSON for debugging (run with --nocapture to see)
    if std::env::var("PRINT_JSON").is_ok() {
        let json =
            serde_json::to_string_pretty(&actual_features).expect("Failed to serialize features");
        println!(
            "\n========== Actual JSON Output ==========\n{}\n========== End ==========\n",
            json
        );
    }
}

#[test]
fn test_javascript_basic_snapshot_without_changes() {
    // Path to the test directory (relative to the workspace root when tests run)
    let test_path = PathBuf::from("../../examples/javascript-basic/src");

    if !test_path.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_path.display()
        );
        return;
    }

    // Scan the directory without changes
    let result = list_files_recursive(&test_path);
    assert!(
        result.is_ok(),
        "Failed to scan directory: {:?}",
        result.err()
    );

    let actual_features = result.unwrap();

    // Load the snapshot
    let snapshot_path = PathBuf::from("tests/snapshots/javascript_basic_features_no_changes.json");
    let snapshot_content = std::fs::read_to_string(&snapshot_path)
        .expect("Failed to read snapshot file. Make sure tests/snapshots/javascript_basic_features_no_changes.json exists");

    let expected_features: Vec<Feature> =
        serde_json::from_str(&snapshot_content).expect("Failed to parse snapshot JSON");

    // Compare the results
    assert_eq!(
        actual_features.len(),
        expected_features.len(),
        "Number of features doesn't match snapshot. Expected: {}, Got: {}",
        expected_features.len(),
        actual_features.len()
    );

    // Compare each feature recursively
    for (actual, expected) in actual_features.iter().zip(expected_features.iter()) {
        compare_features_recursive(actual, expected, "");
    }

    // Optional: Print JSON for debugging (run with --nocapture to see)
    if std::env::var("PRINT_JSON").is_ok() {
        let json =
            serde_json::to_string_pretty(&actual_features).expect("Failed to serialize features");
        println!(
            "\n========== Actual JSON Output (No Changes) ==========\n{}\n========== End ==========\n",
            json
        );
    }
}
