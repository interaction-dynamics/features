//! Cobertura and lcov reports read together, from `examples/coverage-formats`

use features_cli::coverage_parser::CoverageStats;
use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use std::path::PathBuf;

fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/coverage-formats")
}

fn scan(config: impl FnOnce(ScanConfig) -> ScanConfig) -> Vec<Feature> {
    let path = example_path().join("src");
    let base = ScanConfig::new(&path)
        .skip_changes(true)
        .with_coverage(true);
    scan_features(&path, config(base)).unwrap()
}

fn coverage<'a>(features: &'a [Feature], name: &str) -> &'a CoverageStats {
    fn walk<'a>(features: &'a [Feature], name: &str) -> Option<&'a Feature> {
        features.iter().find_map(|f| {
            (f.name == name)
                .then_some(f)
                .or_else(|| walk(&f.features, name))
        })
    }
    walk(features, name)
        .unwrap_or_else(|| panic!("feature {name} not found"))
        .stats
        .as_ref()
        .and_then(|s| s.coverage.as_ref())
        .unwrap_or_else(|| panic!("no coverage for {name}"))
}

fn files(stats: &CoverageStats) -> Vec<&str> {
    let mut files: Vec<&str> = stats.files.keys().map(String::as_str).collect();
    files.sort();
    files
}

/// (lines total, lines covered, branches total, branches covered)
fn counts(stats: &CoverageStats) -> (usize, usize, Option<usize>, Option<usize>) {
    (
        stats.lines_total,
        stats.lines_covered,
        stats.branches_total,
        stats.branches_covered,
    )
}

fn scan_from_project_dir() -> Vec<Feature> {
    scan(|c| c.project_dir(example_path()))
}

#[test]
fn test_cobertura_lines_and_condition_coverage() {
    let features = scan_from_project_dir();
    let api = coverage(&features, "API");
    assert_eq!(counts(api), (6, 4, Some(2), Some(1)));
    assert_eq!(api.branch_coverage_percent, Some(50.0));
    assert_eq!(
        files(api),
        vec!["features/api/handler.py", "features/api/routes.py"]
    );
    let routes = &api.files["features/api/routes.py"];
    assert_eq!(routes.lines_total, 2);
    assert_eq!(routes.branches_total, None);
}

#[test]
fn test_lcov_lines_and_branches() {
    let features = scan_from_project_dir();
    let web = coverage(&features, "Web");
    assert_eq!(counts(web), (3, 2, Some(2), Some(1)));
    // The nested feature is counted apart from its parent
    assert_eq!(files(web), vec!["features/web/render.js"]);

    let forms = coverage(&features, "Forms");
    assert_eq!(counts(forms), (3, 3, Some(2), Some(2)));
    assert_eq!(forms.line_coverage_percent, 100.0);
}

#[test]
fn test_both_reports_are_added_up() {
    let features = scan_from_project_dir();
    let jobs = coverage(&features, "Jobs");
    // worker.py from Cobertura (2/3) + scheduler.js from lcov (0/3)
    assert_eq!(counts(jobs), (6, 2, None, None));
    assert_eq!(
        files(jobs),
        vec!["features/jobs/scheduler.js", "features/jobs/worker.py"]
    );
}

#[test]
fn test_file_outside_features_is_not_counted() {
    let features = scan_from_project_dir();
    for name in ["API", "Web", "Forms", "Jobs"] {
        assert!(
            !coverage(&features, name)
                .files
                .keys()
                .any(|f| f.contains("shared")),
            "{name}"
        );
    }
}

#[test]
fn test_coverage_dir_override() {
    let features = scan(|c| c.coverage_dir(example_path().join(".coverage")));
    assert_eq!(counts(coverage(&features, "API")), (6, 4, Some(2), Some(1)));
}

#[test]
fn test_missing_coverage_dir_adds_no_coverage() {
    let features = scan(|c| c.coverage_dir(example_path().join("no-such-dir")));
    assert!(
        features
            .iter()
            .all(|f| f.stats.as_ref().and_then(|s| s.coverage.as_ref()).is_none())
    );
}
