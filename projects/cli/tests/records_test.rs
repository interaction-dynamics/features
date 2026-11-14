//! Specs and decision records read from `examples/specs-decisions`

use features_cli::checker::check_records;
use features_cli::models::{Feature, RecordKind, TaskProgress};
use features_cli::records::project_records;
use features_cli::scan::{ScanConfig, scan_features};
use std::path::PathBuf;

fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/specs-decisions")
}

fn scan() -> Vec<Feature> {
    let path = example_path();
    scan_features(&path, ScanConfig::new(&path).skip_changes(true)).unwrap()
}

fn find<'a>(features: &'a [Feature], name: &str) -> &'a Feature {
    features.iter().find(|f| f.name == name).unwrap()
}

#[test]
fn test_feature_records_from_every_folder_and_root() {
    let features = scan();
    let auth = find(&features, "Auth");
    let ids: Vec<&str> = auth.records.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(
        ids,
        vec![
            "001-use-sessions",
            "001-auth-overhaul",
            "001-login-flow",
            "002-use-jwt",
            "003-sso",
            "003-token-storage",
        ]
    );

    // The legacy field keeps the raw decisions of the first folder found (`.docs`)
    assert_eq!(auth.decisions.len(), 1);

    let overhaul = &auth.records[1];
    assert_eq!(overhaul.path, "docs/specs/001-auth-overhaul.md");
    assert_eq!(overhaul.feature.as_deref(), Some("auth"));

    let jwt = &auth.records[3];
    assert_eq!(jwt.kind, RecordKind::Decision);
    assert_eq!(jwt.links.specs, vec!["001-login-flow", "001-auth-overhaul"]);
    assert_eq!(jwt.links.supersedes, vec!["001-use-sessions"]);
    assert_eq!(auth.records[0].links.superseded_by, vec!["002-use-jwt"]);
    assert_eq!(auth.records[2].links.decisions, vec!["002-use-jwt"]);

    let sso = &auth.records[4];
    assert_eq!(sso.path, "features/auth/__docs__/specs/003-sso/spec.md");
    let files: Vec<&str> = sso.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(files, vec!["plan.md", "tasks.md"]);
    assert_eq!(sso.tasks, Some(TaskProgress { done: 1, total: 3 }));

    let storage = &auth.records[5];
    assert_eq!(storage.status.as_deref(), Some("proposed"));
    assert_eq!(storage.date.as_deref(), Some("2025-04-02"));
    assert_eq!(storage.links.specs, vec!["003-sso"]);
}

#[test]
fn test_project_records_and_warnings() {
    let features = scan();
    let project = project_records(&features, &example_path());
    let ids: Vec<&str> = project.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, vec!["0001-use-a-monorepo", "002-design-system"]);
    assert_eq!(project[0].status.as_deref(), Some("accepted"));
    assert_eq!(project[0].date.as_deref(), Some("2025-01-10"));

    let warnings = check_records(&features, &project);
    assert_eq!(warnings.len(), 3, "{warnings:#?}");
    assert!(
        warnings
            .iter()
            .any(|w| w.contains("'002-invoices'") && w.contains("no status"))
    );
    assert!(
        warnings
            .iter()
            .any(|w| w.contains("unknown record '099-missing'"))
    );
    assert!(
        warnings
            .iter()
            .any(|w| w.contains("'001-use-stripe'") && w.contains("no record supersedes it"))
    );
}

#[test]
fn test_records_json_shape() {
    let features = scan();
    let json = serde_json::to_value(&features).unwrap();
    let auth = json
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "Auth")
        .unwrap();
    let login = &auth["records"][2];
    assert_eq!(login["kind"], "spec");
    assert_eq!(login["status"], "implemented");
    assert_eq!(login["tasks"], serde_json::json!({ "done": 2, "total": 2 }));
    assert!(login.get("files").is_none());
    assert!(auth["decisions"].is_array());
}
