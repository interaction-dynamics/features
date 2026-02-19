//! Integration tests for file_scanner with changes (git history)
//!
//! This test suite verifies that the file scanner correctly identifies and parses
//! features with their git history, changes, commits, and statistics.
//!
//! Tests compare 100% of the JSON output against snapshots to ensure complete accuracy.

use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use serde_json::Value;
use std::path::PathBuf;

/// Compare two JSON values deeply and return detailed error message with path to first difference
fn compare_json_values(actual: &Value, expected: &Value, path: &str) -> Result<(), String> {
    match (actual, expected) {
        (Value::Object(actual_obj), Value::Object(expected_obj)) => {
            // Check all expected keys exist in actual
            for (key, expected_value) in expected_obj {
                let current_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                match actual_obj.get(key) {
                    Some(actual_value) => {
                        compare_json_values(actual_value, expected_value, &current_path)?;
                    }
                    None => {
                        return Err(format!(
                            "Missing key at path: {}\nExpected: {:?}\nActual: (missing)",
                            current_path, expected_value
                        ));
                    }
                }
            }

            // Check for extra keys in actual that aren't in expected
            for key in actual_obj.keys() {
                if !expected_obj.contains_key(key) {
                    let current_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    return Err(format!(
                        "Unexpected extra key at path: {}\nActual: {:?}\nExpected: (not present)",
                        current_path,
                        actual_obj.get(key)
                    ));
                }
            }

            Ok(())
        }
        (Value::Array(actual_arr), Value::Array(expected_arr)) => {
            if actual_arr.len() != expected_arr.len() {
                return Err(format!(
                    "Array length mismatch at path: {}\nExpected length: {}\nActual length: {}",
                    path,
                    expected_arr.len(),
                    actual_arr.len()
                ));
            }

            for (i, (actual_item, expected_item)) in
                actual_arr.iter().zip(expected_arr.iter()).enumerate()
            {
                let current_path = format!("{}[{}]", path, i);
                compare_json_values(actual_item, expected_item, &current_path)?;
            }

            Ok(())
        }
        (actual_val, expected_val) => {
            if actual_val != expected_val {
                Err(format!(
                    "Value mismatch at path: {}\nExpected: {:?}\nActual: {:?}",
                    path, expected_val, actual_val
                ))
            } else {
                Ok(())
            }
        }
    }
}

#[test]
fn test_tests_with_changes_snapshot() {
    // Path to the test directory (relative to the workspace root when tests run)
    let test_path = PathBuf::from("../../examples/tests-with-changes/src");

    if !test_path.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_path.display()
        );
        return;
    }

    // Scan the directory
    let config = ScanConfig::new(&test_path)
        .skip_changes(false)
        .with_coverage(false);

    let result = scan_features(&test_path, config);
    assert!(
        result.is_ok(),
        "Failed to scan directory: {:?}",
        result.err()
    );

    let actual_features = result.unwrap();

    // Load the snapshot
    let snapshot_path = PathBuf::from("tests/snapshots/tests_with_changes.json");
    let snapshot_content = std::fs::read_to_string(&snapshot_path).expect(
        "Failed to read snapshot file. Make sure tests/snapshots/tests_with_changes.json exists",
    );

    let expected_features: Vec<Feature> =
        serde_json::from_str(&snapshot_content).expect("Failed to parse snapshot JSON");

    // Convert to JSON Values for deep comparison
    let actual_json: Value = serde_json::to_value(&actual_features)
        .expect("Failed to serialize actual features to JSON");
    let expected_json: Value = serde_json::to_value(&expected_features)
        .expect("Failed to serialize expected features to JSON");

    // Compare with detailed error messages
    if let Err(diff_msg) = compare_json_values(&actual_json, &expected_json, "") {
        panic!(
            "Features JSON does not match snapshot exactly.\n\n{}\n\nTo update snapshot, run:\n./tools/cli/target/debug/features ./examples/tests-with-changes/src --json > tools/cli/tests/snapshots/tests_with_changes.json",
            diff_msg
        );
    }

    // Optional: Print JSON for manual inspection (run with --nocapture to see)
    if std::env::var("PRINT_JSON").is_ok() {
        let actual_json_str = serde_json::to_string_pretty(&actual_features)
            .expect("Failed to serialize actual features");
        println!(
            "\n========== Actual JSON Output ==========\n{}\n========== End ==========\n",
            actual_json_str
        );
    }
}
