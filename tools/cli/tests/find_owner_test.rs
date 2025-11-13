//! Integration tests for --find-owner CLI feature
//!
//! This test suite verifies that the CLI correctly finds owners for files and folders,
//! including inheritance from parent features when the immediate feature has an unknown owner.
//!
//! The tests use snapshot comparison and CLI execution to ensure the output
//! matches expected results.

use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

/// Get the path to the features binary
fn get_binary_path() -> PathBuf {
    // The binary should be in target/debug or target/release
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("features");

    // If debug doesn't exist, try release
    if !path.exists() {
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("target");
        path.push("release");
        path.push("features");
    }

    path
}

/// Execute the CLI with given arguments and return stdout
fn run_cli(args: &[&str]) -> Result<String, String> {
    let binary = get_binary_path();

    let output = Command::new(&binary)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute binary: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[test]
fn test_find_owner_with_explicit_owner() {
    // Build the binary first
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_path = test_base.join("features/feature-1/features/feature-4");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--skip-changes",
    ]);

    assert!(result.is_ok(), "CLI execution failed: {:?}", result.err());

    let output = result.unwrap();

    // Verify output contains expected information
    assert!(
        output.contains("Owner: team2"),
        "Output should contain 'Owner: team2', got: {}",
        output
    );
    assert!(
        output.contains("Feature: feature-4"),
        "Output should contain 'Feature: feature-4', got: {}",
        output
    );
    assert!(
        !output.contains("inherited"),
        "Output should not contain 'inherited' for explicit owner, got: {}",
        output
    );
}

#[test]
fn test_find_owner_with_inherited_owner() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_path = test_base.join("features/feature-1/features/feature-2");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--skip-changes",
    ]);

    assert!(result.is_ok(), "CLI execution failed: {:?}", result.err());

    let output = result.unwrap();

    // Verify output contains expected information with inherited tag
    assert!(
        output.contains("Owner: team1 (inherited)"),
        "Output should contain 'Owner: team1 (inherited)', got: {}",
        output
    );
    assert!(
        output.contains("Feature: feature-2"),
        "Output should contain 'Feature: feature-2', got: {}",
        output
    );
}

#[test]
fn test_find_owner_for_file_inside_feature() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_file = test_base.join("features/feature-1/features/feature-2/README.md");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_file.to_str().unwrap(),
        "--skip-changes",
    ]);

    assert!(result.is_ok(), "CLI execution failed: {:?}", result.err());

    let output = result.unwrap();

    // Should find the owner of the containing feature
    assert!(
        output.contains("Owner: team1 (inherited)"),
        "Output should contain 'Owner: team1 (inherited)', got: {}",
        output
    );
    assert!(
        output.contains("Feature: feature-2"),
        "Output should contain 'Feature: feature-2', got: {}",
        output
    );
}

#[test]
fn test_find_owner_json_output_with_explicit_owner() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_path = test_base.join("features/feature-1/features/feature-4");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--json",
        "--skip-changes",
    ]);

    assert!(result.is_ok(), "CLI execution failed: {:?}", result.err());

    let output = result.unwrap();

    // Parse JSON output
    let json: Value = serde_json::from_str(&output).expect("Output should be valid JSON");

    // Load snapshot
    let snapshot_path = PathBuf::from("tests/snapshots/find_owner_explicit.json");
    let snapshot_content = std::fs::read_to_string(&snapshot_path).expect(
        "Failed to read snapshot file. Make sure tests/snapshots/find_owner_explicit.json exists",
    );

    let expected: Value =
        serde_json::from_str(&snapshot_content).expect("Failed to parse snapshot JSON");

    // Compare owner and feature_name (path may vary)
    assert_eq!(json["owner"], expected["owner"], "Owner mismatch");
    assert_eq!(
        json["feature_name"], expected["feature_name"],
        "Feature name mismatch"
    );

    // Verify that 'inherited' is not present (or is false)
    assert!(
        json.get("inherited").is_none() || json["inherited"] == false,
        "Inherited should not be present or should be false for explicit owner"
    );

    // Optional: Print JSON for debugging (run with --nocapture to see)
    if std::env::var("PRINT_JSON").is_ok() {
        println!(
            "\n========== Actual JSON Output ==========\n{}\n========== End ==========\n",
            output
        );
    }
}

#[test]
fn test_find_owner_json_output_with_inherited_owner() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_path = test_base.join("features/feature-1/features/feature-2");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--json",
        "--skip-changes",
    ]);

    assert!(result.is_ok(), "CLI execution failed: {:?}", result.err());

    let output = result.unwrap();

    // Parse JSON output
    let json: Value = serde_json::from_str(&output).expect("Output should be valid JSON");

    // Load snapshot
    let snapshot_path = PathBuf::from("tests/snapshots/find_owner_inherited.json");
    let snapshot_content = std::fs::read_to_string(&snapshot_path).expect(
        "Failed to read snapshot file. Make sure tests/snapshots/find_owner_inherited.json exists",
    );

    let expected: Value =
        serde_json::from_str(&snapshot_content).expect("Failed to parse snapshot JSON");

    // Compare fields
    assert_eq!(json["owner"], expected["owner"], "Owner mismatch");
    assert_eq!(
        json["feature_name"], expected["feature_name"],
        "Feature name mismatch"
    );
    assert_eq!(
        json["inherited"], expected["inherited"],
        "Inherited flag mismatch"
    );

    // Verify that inherited is true
    assert_eq!(json["inherited"], true, "Inherited should be true");

    // Optional: Print JSON for debugging (run with --nocapture to see)
    if std::env::var("PRINT_JSON").is_ok() {
        println!(
            "\n========== Actual JSON Output ==========\n{}\n========== End ==========\n",
            output
        );
    }
}

#[test]
fn test_find_owner_with_unknown_owner() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_path = test_base.join("features/feature-0");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--skip-changes",
    ]);

    assert!(result.is_ok(), "CLI execution failed: {:?}", result.err());

    let output = result.unwrap();

    // Verify output contains Unknown owner (with no inheritance since no parent)
    assert!(
        output.contains("Owner: Unknown"),
        "Output should contain 'Owner: Unknown', got: {}",
        output
    );
    assert!(
        output.contains("Feature: feature-0"),
        "Output should contain 'Feature: feature-0', got: {}",
        output
    );
    assert!(
        !output.contains("inherited"),
        "Output should not contain 'inherited' when there's no parent, got: {}",
        output
    );
}

#[test]
fn test_find_owner_nonexistent_path() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    let target_path = test_base.join("nonexistent/path/to/file.txt");

    if !test_base.exists() {
        println!(
            "Skipping test - test path does not exist: {}",
            test_base.display()
        );
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--skip-changes",
    ]);

    // Should fail with error
    assert!(result.is_err(), "Should fail for nonexistent path");

    let error = result.err().unwrap();
    assert!(
        error.contains("does not exist"),
        "Error should mention path doesn't exist, got: {}",
        error
    );
}

#[test]
fn test_find_owner_path_not_in_any_feature() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_base = manifest_dir.join("../../examples/javascript-basic/src");
    // Use a path that exists but is not in any feature
    let target_path = manifest_dir.join("tests");

    if !test_base.exists() || !target_path.exists() {
        println!("Skipping test - paths do not exist");
        return;
    }

    let result = run_cli(&[
        test_base.to_str().unwrap(),
        "--find-owner",
        target_path.to_str().unwrap(),
        "--skip-changes",
    ]);

    // Should fail with error indicating no feature found
    assert!(
        result.is_err(),
        "Should fail when path is not in any feature"
    );

    let error = result.err().unwrap();
    assert!(
        error.contains("No feature found"),
        "Error should mention no feature found, got: {}",
        error
    );
}
