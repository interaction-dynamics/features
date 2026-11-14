//! GitHub Spec Kit and Kiro records read from `examples/spec-tools`

use features_cli::checker::check_records;
use features_cli::models::{Feature, RecordSource, TaskProgress};
use features_cli::records::project_records;
use features_cli::scan::{ScanConfig, scan_features};
use std::path::PathBuf;

fn example_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/spec-tools")
}

fn scan() -> Vec<Feature> {
    let path = example_path();
    scan_features(&path, ScanConfig::new(&path).skip_changes(true)).unwrap()
}

fn find<'a>(features: &'a [Feature], name: &str) -> &'a Feature {
    fn walk<'a>(features: &'a [Feature], name: &str) -> Option<&'a Feature> {
        features.iter().find_map(|f| {
            (f.name == name)
                .then_some(f)
                .or_else(|| walk(&f.features, name))
        })
    }
    walk(features, name).unwrap()
}

#[test]
fn test_speckit_spec_attached_by_folder_name() {
    let features = scan();
    let auth = find(&features, "Auth");
    assert_eq!(auth.records.len(), 1);
    let spec = &auth.records[0];
    assert_eq!(spec.id, "003-auth");
    assert_eq!(spec.source, RecordSource::Speckit);
    assert_eq!(spec.title, "Passwordless sign-in");
    assert_eq!(spec.status.as_deref(), Some("draft"));
    assert_eq!(spec.date.as_deref(), Some("2026-04-02"));
    assert_eq!(spec.path, "specs/003-auth/spec.md");
    let files: Vec<&str> = spec.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(
        files,
        vec![
            "plan.md",
            "research.md",
            "data-model.md",
            "quickstart.md",
            "tasks.md",
            "contracts/login.yaml",
        ]
    );
    // From tasks.md, not from the review checklist of spec.md
    assert_eq!(spec.tasks, Some(TaskProgress { done: 2, total: 4 }));
}

#[test]
fn test_kiro_spec_attached_by_folder_name() {
    let features = scan();
    let billing = find(&features, "Billing");
    assert_eq!(billing.records.len(), 1);
    let spec = &billing.records[0];
    assert_eq!(spec.source, RecordSource::Kiro);
    assert_eq!(spec.title, "Billing");
    assert_eq!(spec.path, ".kiro/specs/billing/requirements.md");
    let files: Vec<&str> = spec.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(files, vec!["design.md", "tasks.md"]);
}

#[test]
fn test_unmatched_specs_are_project_level_without_warnings() {
    let features = scan();
    let project = project_records(&features, &example_path());
    let ids: Vec<(&str, RecordSource)> =
        project.iter().map(|r| (r.id.as_str(), r.source)).collect();
    assert!(
        ids.contains(&("004-search", RecordSource::Speckit)),
        "{ids:?}"
    );
    assert!(
        ids.contains(&("notifications-center", RecordSource::Kiro)),
        "{ids:?}"
    );
    assert_eq!(project.len(), 2);
    let kiro = project
        .iter()
        .find(|r| r.source == RecordSource::Kiro)
        .unwrap();
    assert_eq!(kiro.title, "Notifications center");

    // Kiro has no status: no "missing status" warning for tool records
    let warnings = check_records(&features, &project);
    assert!(warnings.is_empty(), "{warnings:#?}");
}

#[test]
fn test_source_is_serialized_for_tool_records_only() {
    let features = scan();
    let json = serde_json::to_value(&features).unwrap();
    let auth = json
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "Auth")
        .unwrap();
    assert_eq!(auth["records"][0]["source"], "speckit");

    let native = serde_json::to_value(features_cli::models::Record::default()).unwrap();
    assert!(native.get("source").is_none());
}
