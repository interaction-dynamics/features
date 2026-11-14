//! `--feature-<key>` comments read with the comment syntax of each language,
//! from `examples/metadata-comments`

use features_cli::models::Feature;
use features_cli::scan::{ScanConfig, scan_features};
use serde_json::{Value, json};
use std::path::PathBuf;

fn scan() -> Vec<Feature> {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/metadata-comments/src");
    scan_features(&path, ScanConfig::new(&path).skip_changes(true)).unwrap()
}

fn meta(features: &[Feature], name: &str, key: &str) -> Value {
    let feature = features
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("feature {name} not found"));
    feature.meta.get(key).cloned().unwrap_or(Value::Null)
}

#[test]
fn test_python_hash_and_go_block_comments() {
    let features = scan();
    assert_eq!(
        meta(&features, "Search", "flag"),
        json!([
            { "name": "async-indexing" },
            { "name": "smart-ranking", "owner": "team-search" },
        ])
    );
    assert_eq!(
        meta(&features, "Search", "experiment"),
        json!([{ "name": "rerank-v2", "traffic": "10%" }])
    );
}

#[test]
fn test_ruby_yaml_and_sql_comments() {
    let features = scan();
    assert_eq!(
        meta(&features, "Accounts", "flag"),
        json!([
            // SQL `--` comment naming its feature explicitly
            { "feature": "features/accounts", "name": "soft-delete", "table": "users" },
            { "name": "passwordless-signup", "status": "beta" },
        ])
    );
    assert_eq!(
        meta(&features, "Accounts", "experiment"),
        json!([{ "name": "onboarding-checklist", "traffic": "50%" }])
    );
}

#[test]
fn test_lua_html_and_css_comments() {
    let features = scan();
    // `notes.txt` mentions --feature-flag without properties: ignored
    assert_eq!(
        meta(&features, "Storefront", "flag"),
        json!([
            { "name": "edge-cache", "ttl": "60" },
            { "name": "new-header" },
            { "name": "dark-mode" },
        ])
    );
    assert_eq!(meta(&features, "Storefront", "experiment"), Value::Null);
}

#[test]
fn test_property_order_is_kept() {
    let features = scan();
    let flags = meta(&features, "Accounts", "flag");
    let keys: Vec<&String> = flags[0].as_object().unwrap().keys().collect();
    assert_eq!(keys, vec!["feature", "name", "table"]);
}
