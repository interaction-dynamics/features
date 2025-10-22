# Integration Tests

This directory contains integration tests for the `features-cli` tool, specifically testing the file scanning functionality that detects README files in feature directories.

## Test Structure

```
tests/
├── README.md                           # This file
├── integration_test.rs                 # Main integration test file
└── snapshots/                          # Snapshot files directory
    └── javascript_basic_features.json  # Expected output for javascript-basic example
```

## Snapshot Testing

The integration tests use **snapshot testing** to verify that the file scanner produces the expected output. Instead of writing manual assertions for each field, we compare the actual output against a saved JSON snapshot.

### How It Works

1. The test scans the `examples/javascript-basic/src` directory
2. It loads the expected output from `snapshots/javascript_basic_features.json`
3. It compares the actual output with the snapshot
4. If they match, the test passes; otherwise, it fails with a detailed diff

### Benefits

- **Easier maintenance**: Update one JSON file instead of many assertions
- **Clear expectations**: The snapshot shows exactly what output is expected
- **Comprehensive validation**: All fields are automatically compared
- **Better debugging**: Failures show exactly what changed

## Running Tests

### Run all tests

```bash
cargo test
```

### Run only integration tests

```bash
cargo test --test integration_test
```

### Run with JSON output (for debugging)

```bash
PRINT_JSON=1 cargo test --test integration_test -- --nocapture
```

This will print the actual JSON output during the test run, useful for debugging or seeing what the scanner produces.

## Updating Snapshots

When you intentionally change the scanner's behavior and the output changes, you'll need to update the snapshot:

1. Run the test with `PRINT_JSON=1` to see the new output:
   ```bash
   PRINT_JSON=1 cargo test --test integration_test -- --nocapture
   ```

2. Copy the JSON output from the test results

3. Update `snapshots/javascript_basic_features.json` with the new output

4. Run the test again to verify it passes:
   ```bash
   cargo test --test integration_test
   ```

## Test Coverage

The integration test verifies:

- ✅ Only README files in **direct subfolders** of a "features" directory are detected
- ✅ Both `README.md` and `README.mdx` files are supported
- ✅ Nested features are **not** included in the output
- ✅ Documentation directories (`__docs__`, `.docs`, `docs`) are excluded
- ✅ Feature metadata (owner, description, meta) is correctly parsed
- ✅ Empty arrays for `features`, `changes`, and `decisions` when using `list_files_recursive()`
- ✅ JSON serialization/deserialization works correctly

## Expected Output Format

The snapshot follows this structure:

```json
[
  {
    "name": "feature-1",
    "description": "Feature description from README",
    "owner": "team1",
    "path": "path/to/feature",
    "features": [],
    "meta": {},
    "changes": [],
    "decisions": []
  }
]
```

### Field Descriptions

- **name**: The directory name of the feature
- **description**: Content from the README file (excluding the title)
- **owner**: Extracted from YAML frontmatter in the README
- **path**: Full path to the feature directory
- **features**: Nested features (empty due to direct-subfolder-only restriction)
- **meta**: Additional metadata from YAML frontmatter
- **changes**: Git commit history (empty when using `list_files_recursive`)
- **decisions**: Decision documents (empty when using `list_files_recursive`)

## Adding New Tests

To add a new snapshot test:

1. Create a new test function in `integration_test.rs`
2. Run the test with `PRINT_JSON=1` to generate the output
3. Save the output as a new snapshot file in `snapshots/`
4. Update the test to load and compare against the new snapshot

Example:

```rust
#[test]
fn test_new_example_snapshot() {
    let test_path = PathBuf::from("../../examples/new-example/src");
    let result = list_files_recursive(&test_path).unwrap();
    
    let snapshot_path = PathBuf::from("tests/snapshots/new_example_features.json");
    let expected: Vec<Feature> = serde_json::from_str(
        &std::fs::read_to_string(&snapshot_path).unwrap()
    ).unwrap();
    
    assert_eq!(result, expected);
}
```
