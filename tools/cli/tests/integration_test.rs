//! Integration tests for file_scanner module
//!
//! This test suite verifies that the file scanner correctly identifies and parses
//! README files in direct subfolders of a "features" directory.
//!
//! The test uses snapshot comparison to ensure the output matches expected results.

use features_cli::file_scanner::list_files_recursive;
use features_cli::models::Feature;
use std::path::PathBuf;

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
    let result = list_files_recursive(&test_path);
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

    // Compare each feature
    for (actual, expected) in actual_features.iter().zip(expected_features.iter()) {
        assert_eq!(actual.name, expected.name, "Feature name mismatch");
        assert_eq!(
            actual.description, expected.description,
            "Feature '{}' description mismatch",
            actual.name
        );
        assert_eq!(
            actual.owner, expected.owner,
            "Feature '{}' owner mismatch",
            actual.name
        );
        assert!(
            actual
                .path
                .ends_with(&expected.path.split('/').last().unwrap()),
            "Feature '{}' path mismatch. Expected to end with '{}', got '{}'",
            actual.name,
            expected.path.split('/').last().unwrap(),
            actual.path
        );
        assert_eq!(
            actual.features.len(),
            expected.features.len(),
            "Feature '{}' should have {} nested features, got {}",
            actual.name,
            expected.features.len(),
            actual.features.len()
        );
        assert_eq!(
            actual.meta, expected.meta,
            "Feature '{}' meta mismatch",
            actual.name
        );
        assert_eq!(
            actual.changes.len(),
            expected.changes.len(),
            "Feature '{}' should have {} changes, got {}",
            actual.name,
            expected.changes.len(),
            actual.changes.len()
        );
        assert_eq!(
            actual.decisions.len(),
            expected.decisions.len(),
            "Feature '{}' should have {} decisions, got {}",
            actual.name,
            expected.decisions.len(),
            actual.decisions.len()
        );
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
