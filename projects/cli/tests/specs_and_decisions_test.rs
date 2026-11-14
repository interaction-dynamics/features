//! Specs and decision records read from `examples/specs-and-decisions`:
//! native per-feature and root records, a GitHub Spec Kit layout and a Kiro layout.

use std::path::PathBuf;

use features_cli::models::{Feature, Record, RecordKind, RecordSource, TaskProgress};
use features_cli::records::record_warnings;
use features_cli::scan::{ScanConfig, ScanOutput, scan_project_with};

fn scan() -> ScanOutput {
    let base = PathBuf::from("../../examples/specs-and-decisions");
    let config = ScanConfig::new(&base).skip_changes(true);
    scan_project_with(&base, &config).unwrap()
}

fn feature<'a>(output: &'a ScanOutput, name: &str) -> &'a Feature {
    output
        .features
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("feature {} not found", name))
}

fn record<'a>(records: &'a [Record], id: &str) -> &'a Record {
    records
        .iter()
        .find(|r| r.id == id)
        .unwrap_or_else(|| panic!("record {} not found", id))
}

fn ids(records: &[Record]) -> Vec<&str> {
    records.iter().map(|r| r.id.as_str()).collect()
}

#[test]
fn test_native_records_of_a_feature() {
    let output = scan();
    let auth = feature(&output, "Auth");

    assert_eq!(
        ids(&auth.records),
        vec![
            "001-login-flow",
            "002-sso",
            "001-session-cookies",
            "002-use-jwt",
            "003-rate-limiting",
            "003-auth",
        ]
    );
    // `decisions` keeps its previous behavior: the first decisions folder only
    assert_eq!(auth.decisions.len(), 1);

    let login = record(&auth.records, "001-login-flow");
    assert_eq!(login.kind, RecordKind::Spec);
    assert_eq!(login.status.as_deref(), Some("implemented"));
    assert_eq!(login.date.as_deref(), Some("2026-01-05"));
    assert_eq!(login.tasks, Some(TaskProgress { done: 2, total: 2 }));
    assert_eq!(login.links.decisions, vec!["002-use-jwt"]);

    let sso = record(&auth.records, "002-sso");
    assert_eq!(sso.path, "src/features/auth/__docs__/specs/002-sso/spec.md");
    let files: Vec<&str> = sso.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(files, vec!["plan.md", "tasks.md"]);
    assert_eq!(sso.tasks, Some(TaskProgress { done: 1, total: 3 }));

    // Links are resolved in both directions
    let jwt = record(&auth.records, "002-use-jwt");
    assert_eq!(jwt.links.specs, vec!["001-login-flow"]);
    assert_eq!(jwt.links.supersedes, vec!["001-session-cookies"]);
    let cookies = record(&auth.records, "001-session-cookies");
    assert_eq!(cookies.links.superseded_by, vec!["002-use-jwt"]);

    // Classic MADR status section, in `.docs/`
    let rate = record(&auth.records, "003-rate-limiting");
    assert_eq!(rate.status.as_deref(), Some("proposed"));
    assert!(!rate.content.starts_with("---"));
}

#[test]
fn test_speckit_spec_is_attached_by_folder_name() {
    let output = scan();
    let spec = record(&feature(&output, "Auth").records, "003-auth");

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
            "contracts/login.yaml"
        ]
    );
    // Tasks come from tasks.md, not from the checklist of spec.md
    assert_eq!(spec.tasks, Some(TaskProgress { done: 2, total: 4 }));
}

#[test]
fn test_root_and_kiro_records_attached_to_billing() {
    let output = scan();
    let billing = feature(&output, "Billing");

    assert_eq!(
        ids(&billing.records),
        vec!["001-invoices", "002-postgres-billing", "billing"]
    );

    let postgres = record(&billing.records, "002-postgres-billing");
    assert_eq!(postgres.path, "docs/decisions/002-postgres-billing.md");
    assert_eq!(postgres.links.specs, vec!["001-invoices"]);

    let kiro = record(&billing.records, "billing");
    assert_eq!(kiro.source, RecordSource::Kiro);
    assert_eq!(kiro.title, "Billing");
    assert_eq!(kiro.path, ".kiro/specs/billing/requirements.md");
    assert_eq!(kiro.tasks, Some(TaskProgress { done: 2, total: 3 }));
    assert!(kiro.status.is_none());
}

#[test]
fn test_project_level_records() {
    let output = scan();
    assert_eq!(
        ids(&output.records),
        vec![
            "001-platform-observability",
            "001-monorepo",
            "0001-record-architecture-decisions",
            "004-search",
            "notifications-center",
        ]
    );
    let adr = record(&output.records, "0001-record-architecture-decisions");
    assert_eq!(adr.status.as_deref(), Some("accepted"));
    assert_eq!(adr.date.as_deref(), Some("2025-10-01"));
}

#[test]
fn test_check_warnings() {
    let output = scan();
    let warnings = record_warnings(&output.features, &output.records);
    assert_eq!(
        warnings,
        vec![
            "src/features/auth/__docs__/specs/002-sso/spec.md: `decisions` link `004-saml-provider` does not match any spec or decision"
        ]
    );
}

#[test]
fn test_records_json_shape() {
    let output = scan();
    let json = serde_json::to_value(&output).unwrap();
    let record = &json["features"][0]["records"][0];
    assert_eq!(record["kind"], "spec");
    assert_eq!(record["source"], "native");
    assert!(record["links"]["superseded_by"].is_array());
    // Unknown values are left out
    assert!(json["records"][4].get("status").is_none());
    assert!(json["records"][4].get("files").is_none());
}
