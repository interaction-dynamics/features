use std::collections::HashMap;
use std::fs;

use tempfile::TempDir;

use features_cli::coverage_parser::{
    CoverageStats, map_coverage_to_features, parse_coverage_reports,
};
use features_cli::models::Feature;

fn create_test_feature(name: &str, path: &str) -> Feature {
    Feature {
        name: name.to_string(),
        description: String::new(),
        owner: "Test Owner".to_string(),
        path: path.to_string(),
        features: Vec::new(),
        meta: HashMap::new(),
        changes: Vec::new(),
        decisions: Vec::new(),
        stats: None,
    }
}

fn create_cobertura_xml() -> String {
    r#"<?xml version="1.0" ?>
<coverage version="1.0" timestamp="1704067200000" lines-valid="100" lines-covered="80" line-rate="0.80" branches-valid="20" branches-covered="15" branch-rate="0.75" complexity="0">
    <sources>
        <source>/path/to/project</source>
    </sources>
    <packages>
        <package name="feature1" line-rate="0.85" branch-rate="0.80" complexity="0">
            <classes>
                <class name="lib.rs" filename="src/features/feature-1/lib.rs" line-rate="0.90" branch-rate="0.85" complexity="0">
                    <methods/>
                    <lines>
                        <line number="1" hits="10"/>
                        <line number="2" hits="10"/>
                        <line number="3" hits="10"/>
                        <line number="4" hits="10"/>
                        <line number="5" hits="0"/>
                    </lines>
                </class>
            </classes>
        </package>
        <package name="feature2" line-rate="0.75" branch-rate="0.70" complexity="0">
            <classes>
                <class name="main.rs" filename="src/features/feature-2/main.rs" line-rate="0.75" branch-rate="0.70" complexity="0">
                    <methods/>
                    <lines>
                        <line number="1" hits="5"/>
                        <line number="2" hits="5"/>
                        <line number="3" hits="0"/>
                        <line number="4" hits="5"/>
                    </lines>
                </class>
            </classes>
        </package>
    </packages>
</coverage>"#.to_string()
}

fn create_lcov_info() -> String {
    r#"TN:
SF:src/features/feature-1/lib.rs
FN:1,feature1_init
FNDA:10,feature1_init
FNF:1
FNH:1
DA:1,10
DA:2,10
DA:3,10
DA:4,10
DA:5,0
LH:4
LF:5
end_of_record
TN:
SF:src/features/feature-2/main.rs
FN:1,feature2_main
FNDA:5,feature2_main
FNF:1
FNH:1
DA:1,5
DA:2,5
DA:3,0
DA:4,5
BRDA:2,0,0,3
BRDA:2,0,1,2
LH:3
LF:4
BRH:2
BRF:2
end_of_record"#
        .to_string()
}

#[test]
fn test_parse_cobertura_xml() {
    let temp_dir = TempDir::new().unwrap();
    let coverage_dir = temp_dir.path();

    // Create Cobertura XML file
    let cobertura_path = coverage_dir.join("cobertura.xml");
    fs::write(&cobertura_path, create_cobertura_xml()).unwrap();

    // Parse coverage
    let coverage_map = parse_coverage_reports(coverage_dir).unwrap();

    // Verify feature-1 coverage
    let feature1_file = "src/features/feature-1/lib.rs";
    assert!(coverage_map.contains_key(feature1_file));
    let coverage = &coverage_map[feature1_file];
    assert_eq!(coverage.lines_total, 5);
    assert_eq!(coverage.lines_covered, 4);
    assert_eq!(coverage.lines_missed, 1);
    assert_eq!(coverage.line_coverage_percent, 80.0);

    // Verify feature-2 coverage
    let feature2_file = "src/features/feature-2/main.rs";
    assert!(coverage_map.contains_key(feature2_file));
    let coverage = &coverage_map[feature2_file];
    assert_eq!(coverage.lines_total, 4);
    assert_eq!(coverage.lines_covered, 3);
    assert_eq!(coverage.lines_missed, 1);
    assert_eq!(coverage.line_coverage_percent, 75.0);
}

#[test]
fn test_parse_lcov() {
    let temp_dir = TempDir::new().unwrap();
    let coverage_dir = temp_dir.path();

    // Create Lcov file
    let lcov_path = coverage_dir.join("lcov.info");
    fs::write(&lcov_path, create_lcov_info()).unwrap();

    // Parse coverage
    let coverage_map = parse_coverage_reports(coverage_dir).unwrap();

    // Verify feature-1 coverage
    let feature1_file = "src/features/feature-1/lib.rs";
    assert!(coverage_map.contains_key(feature1_file));
    let coverage = &coverage_map[feature1_file];
    assert_eq!(coverage.lines_total, 5);
    assert_eq!(coverage.lines_covered, 4);

    // Verify feature-2 coverage with branch data
    let feature2_file = "src/features/feature-2/main.rs";
    assert!(coverage_map.contains_key(feature2_file));
    let coverage = &coverage_map[feature2_file];
    assert_eq!(coverage.lines_total, 4);
    assert_eq!(coverage.lines_covered, 3);
    assert_eq!(coverage.branches_total, Some(2));
    assert_eq!(coverage.branches_covered, Some(2));
    assert_eq!(coverage.branch_coverage_percent, Some(100.0));
}

#[test]
fn test_merge_multiple_coverage_files() {
    let temp_dir = TempDir::new().unwrap();
    let coverage_dir = temp_dir.path();

    // Create both Cobertura and Lcov files
    let cobertura_path = coverage_dir.join("cobertura.xml");
    fs::write(&cobertura_path, create_cobertura_xml()).unwrap();

    let lcov_path = coverage_dir.join("lcov.info");
    fs::write(&lcov_path, create_lcov_info()).unwrap();

    // Parse coverage
    let coverage_map = parse_coverage_reports(coverage_dir).unwrap();

    // Coverage should be merged for the same file
    let feature1_file = "src/features/feature-1/lib.rs";
    assert!(coverage_map.contains_key(feature1_file));
    let coverage = &coverage_map[feature1_file];

    // Both files have 5 lines, so merged should be 10 total
    assert_eq!(coverage.lines_total, 10);
    assert_eq!(coverage.lines_covered, 8);
}

#[test]
fn test_map_coverage_to_features() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create feature directories
    let feature1_dir = base_path.join("src/features/feature-1");
    let feature2_dir = base_path.join("src/features/feature-2");
    fs::create_dir_all(&feature1_dir).unwrap();
    fs::create_dir_all(&feature2_dir).unwrap();

    // Create dummy files so paths can be canonicalized
    fs::write(feature1_dir.join("lib.rs"), "").unwrap();
    fs::write(feature2_dir.join("main.rs"), "").unwrap();

    // Create features
    let features = vec![
        create_test_feature("Feature 1", "./src/features/feature-1"),
        create_test_feature("Feature 2", "./src/features/feature-2"),
    ];

    // Create coverage map
    let mut coverage_map = HashMap::new();

    let mut cov1 = CoverageStats::new();
    cov1.lines_total = 100;
    cov1.lines_covered = 85;
    cov1.calculate_percentages();
    coverage_map.insert("src/features/feature-1/lib.rs".to_string(), cov1);

    let mut cov2 = CoverageStats::new();
    cov2.lines_total = 50;
    cov2.lines_covered = 40;
    cov2.calculate_percentages();
    coverage_map.insert("src/features/feature-2/main.rs".to_string(), cov2);

    // Map coverage to features
    let feature_coverage = map_coverage_to_features(&features, coverage_map, base_path);

    // Verify Feature 1 coverage
    assert!(feature_coverage.contains_key("Feature 1"));
    let f1_cov = &feature_coverage["Feature 1"];
    assert_eq!(f1_cov.lines_total, 100);
    assert_eq!(f1_cov.lines_covered, 85);
    assert_eq!(f1_cov.line_coverage_percent, 85.0);

    // Verify Feature 2 coverage
    assert!(feature_coverage.contains_key("Feature 2"));
    let f2_cov = &feature_coverage["Feature 2"];
    assert_eq!(f2_cov.lines_total, 50);
    assert_eq!(f2_cov.lines_covered, 40);
    assert_eq!(f2_cov.line_coverage_percent, 80.0);
}

#[test]
fn test_coverage_stats_merge() {
    let mut stats1 = CoverageStats::new();
    stats1.lines_total = 100;
    stats1.lines_covered = 80;
    stats1.branches_total = Some(20);
    stats1.branches_covered = Some(15);
    stats1.calculate_percentages();

    let mut stats2 = CoverageStats::new();
    stats2.lines_total = 50;
    stats2.lines_covered = 45;
    stats2.branches_total = Some(10);
    stats2.branches_covered = Some(9);
    stats2.calculate_percentages();

    // Merge stats2 into stats1
    stats1.merge(&stats2);

    // Verify merged values
    assert_eq!(stats1.lines_total, 150);
    assert_eq!(stats1.lines_covered, 125);
    assert_eq!(stats1.lines_missed, 25);

    // Check percentage is recalculated correctly
    assert_eq!(stats1.line_coverage_percent, (125.0 / 150.0) * 100.0);

    // Verify branches
    assert_eq!(stats1.branches_total, Some(30));
    assert_eq!(stats1.branches_covered, Some(24));
    assert_eq!(stats1.branch_coverage_percent, Some((24.0 / 30.0) * 100.0));
}

#[test]
fn test_empty_coverage_directory() {
    let temp_dir = TempDir::new().unwrap();
    let coverage_dir = temp_dir.path().join(".coverage");
    fs::create_dir(&coverage_dir).unwrap();

    // Parse empty directory
    let coverage_map = parse_coverage_reports(&coverage_dir).unwrap();

    // Should be empty
    assert!(coverage_map.is_empty());
}

#[test]
fn test_nonexistent_coverage_directory() {
    let temp_dir = TempDir::new().unwrap();
    let coverage_dir = temp_dir.path().join(".coverage");

    // Parse non-existent directory
    let coverage_map = parse_coverage_reports(&coverage_dir).unwrap();

    // Should return empty map, not error
    assert!(coverage_map.is_empty());
}

#[test]
fn test_nested_features_coverage() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create nested feature structure
    let parent_dir = base_path.join("src/features/parent");
    let child_dir = parent_dir.join("features/child");
    fs::create_dir_all(&child_dir).unwrap();

    // Create dummy files
    fs::write(parent_dir.join("lib.rs"), "").unwrap();
    fs::write(child_dir.join("lib.rs"), "").unwrap();

    // Create features with nesting
    let child_feature =
        create_test_feature("Child Feature", "./src/features/parent/features/child");
    let mut parent_feature = create_test_feature("Parent Feature", "./src/features/parent");
    parent_feature.features = vec![child_feature];
    let features = vec![parent_feature];

    // Create coverage map
    let mut coverage_map = HashMap::new();

    let mut parent_cov = CoverageStats::new();
    parent_cov.lines_total = 100;
    parent_cov.lines_covered = 90;
    parent_cov.calculate_percentages();
    coverage_map.insert("src/features/parent/lib.rs".to_string(), parent_cov);

    let mut child_cov = CoverageStats::new();
    child_cov.lines_total = 50;
    child_cov.lines_covered = 45;
    child_cov.calculate_percentages();
    coverage_map.insert(
        "src/features/parent/features/child/lib.rs".to_string(),
        child_cov,
    );

    // Map coverage to features
    let feature_coverage = map_coverage_to_features(&features, coverage_map, base_path);

    // Verify both parent and child have separate coverage
    assert!(feature_coverage.contains_key("Parent Feature"));
    assert!(feature_coverage.contains_key("Child Feature"));

    let parent_stats = &feature_coverage["Parent Feature"];
    assert_eq!(parent_stats.lines_total, 100);

    let child_stats = &feature_coverage["Child Feature"];
    assert_eq!(child_stats.lines_total, 50);
}
