//! Data imported from external tools with `--data <FILE>`.
//!
//! A data file is a JSON object that maps feature keys to metrics:
//!
//! ```json
//! {
//!   "checkout": { "weekly_users": 1200, "open_errors": 3 },
//!   "search": { "weekly_users": 80, "status": "healthy" }
//! }
//! ```
//!
//! Metric values are strings, numbers or booleans. The metrics end up in the
//! `external` field of every feature whose `key` matches.

use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::aggregate::AggregatedFeature;
use crate::models::Feature;

/// Metrics of one feature, by metric name
pub type Metrics = BTreeMap<String, serde_json::Value>;

/// External data, by feature key
pub type ExternalData = BTreeMap<String, Metrics>;

/// A feature tree node that can receive external data
pub trait ExternalDataTarget: Sized {
    fn key(&self) -> &str;
    fn external_mut(&mut self) -> &mut Metrics;
    fn children_mut(&mut self) -> &mut [Self];
}

impl ExternalDataTarget for Feature {
    fn key(&self) -> &str {
        &self.key
    }
    fn external_mut(&mut self) -> &mut Metrics {
        &mut self.external
    }
    fn children_mut(&mut self) -> &mut [Self] {
        &mut self.features
    }
}

impl ExternalDataTarget for AggregatedFeature {
    fn key(&self) -> &str {
        &self.key
    }
    fn external_mut(&mut self) -> &mut Metrics {
        &mut self.external
    }
    fn children_mut(&mut self) -> &mut [Self] {
        &mut self.features
    }
}

/// Parse the content of a data file. `source` names the file in error messages.
pub fn parse_external_data(content: &str, source: &str) -> Result<ExternalData> {
    let value: serde_json::Value = serde_json::from_str(content)
        .with_context(|| format!("could not parse data file `{source}` as JSON"))?;

    let serde_json::Value::Object(entries) = value else {
        bail!(
            "data file `{source}` must be a JSON object mapping feature keys to metrics, e.g. {{ \"checkout\": {{ \"weekly_users\": 1200 }} }}"
        );
    };

    let mut data = ExternalData::new();
    for (key, metrics) in entries {
        let serde_json::Value::Object(metrics) = metrics else {
            bail!(
                "in data file `{source}`, the value of feature `{key}` must be an object of metrics"
            );
        };

        for (metric, value) in &metrics {
            if !(value.is_string() || value.is_number() || value.is_boolean()) {
                bail!(
                    "in data file `{source}`, metric `{metric}` of feature `{key}` must be a string, a number or a boolean"
                );
            }
        }

        data.insert(key, metrics.into_iter().collect());
    }

    Ok(data)
}

/// Read and merge data files. Later files override earlier ones, metric by metric.
pub fn load_external_data(paths: &[PathBuf]) -> Result<ExternalData> {
    let mut data = ExternalData::new();
    for path in paths {
        let file_data = read_external_data(path)?;
        for (key, metrics) in file_data {
            data.entry(key).or_default().extend(metrics);
        }
    }
    Ok(data)
}

fn read_external_data(path: &Path) -> Result<ExternalData> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("could not read data file `{}`", path.display()))?;
    parse_external_data(&content, &path.display().to_string())
}

/// Add the metrics of `data` to every feature with a matching key, nested features
/// included, overriding metrics with the same name.
///
/// Returns the keys of `data` that match no feature.
pub fn apply_external_data<F: ExternalDataTarget>(
    features: &mut [F],
    data: &ExternalData,
) -> Vec<String> {
    fn visit<F: ExternalDataTarget>(
        features: &mut [F],
        data: &ExternalData,
        matched: &mut BTreeSet<String>,
    ) {
        for feature in features {
            if let Some(metrics) = data.get(feature.key()) {
                matched.insert(feature.key().to_string());
                feature
                    .external_mut()
                    .extend(metrics.iter().map(|(k, v)| (k.clone(), v.clone())));
            }
            visit(feature.children_mut(), data, matched);
        }
    }

    let mut matched = BTreeSet::new();
    visit(features, data, &mut matched);

    data.keys()
        .filter(|key| !matched.contains(*key))
        .cloned()
        .collect()
}

/// Load `paths` and apply them to `features`, warning on stderr about keys that
/// match no feature. Does nothing when `paths` is empty.
pub fn import_external_data<F: ExternalDataTarget>(
    features: &mut [F],
    paths: &[PathBuf],
) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    let data = load_external_data(paths)?;
    let unmatched = apply_external_data(features, &data);
    if !unmatched.is_empty() {
        eprintln!(
            "Warning: no feature matches these keys of the data file(s): {}",
            unmatched.join(", ")
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn feature(key: &str, children: Vec<Feature>) -> Feature {
        Feature {
            name: key.to_string(),
            key: key.to_string(),
            description: String::new(),
            owner: String::new(),
            is_owner_inherited: false,
            path: key.to_string(),
            features: children,
            meta: BTreeMap::new(),
            links: BTreeMap::new(),
            changes: Vec::new(),
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: BTreeMap::new(),
        }
    }

    #[test]
    fn test_parse_external_data() {
        let data = parse_external_data(
            r#"{ "checkout": { "weekly_users": 1200, "status": "healthy", "beta": true } }"#,
            "data.json",
        )
        .unwrap();
        assert_eq!(data["checkout"]["weekly_users"], json!(1200));
        assert_eq!(data["checkout"]["status"], json!("healthy"));
        assert_eq!(data["checkout"]["beta"], json!(true));
    }

    #[test]
    fn test_parse_external_data_rejects_invalid_shapes() {
        let error = |content: &str| {
            parse_external_data(content, "data.json")
                .unwrap_err()
                .to_string()
        };

        assert!(error("not json").contains("could not parse data file `data.json`"));
        assert!(error("[1, 2]").contains("must be a JSON object"));
        assert!(error(r#"{ "checkout": 3 }"#).contains("feature `checkout`"));
        assert!(
            error(r#"{ "checkout": { "users": [1] } }"#)
                .contains("metric `users` of feature `checkout`")
        );
        assert!(error(r#"{ "checkout": { "users": null } }"#).contains("metric `users`"));
    }

    #[test]
    fn test_load_external_data_merges_files_in_order() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let first = temp_dir.path().join("first.json");
        let second = temp_dir.path().join("second.json");
        fs::write(&first, r#"{ "checkout": { "users": 1, "errors": 5 } }"#).unwrap();
        fs::write(
            &second,
            r#"{ "checkout": { "users": 2 }, "search": { "users": 3 } }"#,
        )
        .unwrap();

        let data = load_external_data(&[first, second]).unwrap();
        assert_eq!(data["checkout"]["users"], json!(2));
        assert_eq!(data["checkout"]["errors"], json!(5));
        assert_eq!(data["search"]["users"], json!(3));
    }

    #[test]
    fn test_load_external_data_reports_missing_files() {
        let error = load_external_data(&[PathBuf::from("does-not-exist.json")])
            .unwrap_err()
            .to_string();
        assert!(error.contains("could not read data file `does-not-exist.json`"));
    }

    #[test]
    fn test_apply_external_data_matches_nested_features_by_key() {
        let mut features = vec![feature(
            "checkout",
            vec![feature("payment", vec![]), feature("cart", vec![])],
        )];
        features[0].external.insert("users".to_string(), json!(1));

        let data = parse_external_data(
            r#"{ "checkout": { "users": 10, "errors": 2 }, "payment": { "errors": 1 }, "unknown": { "users": 3 } }"#,
            "data.json",
        )
        .unwrap();

        let unmatched = apply_external_data(&mut features, &data);

        assert_eq!(unmatched, vec!["unknown".to_string()]);
        assert_eq!(features[0].external["users"], json!(10));
        assert_eq!(features[0].external["errors"], json!(2));
        assert_eq!(features[0].features[0].external["errors"], json!(1));
        assert!(features[0].features[1].external.is_empty());
    }

    #[test]
    fn test_external_data_is_serialized_only_when_present() {
        let mut feature = feature("checkout", vec![]);
        let json = serde_json::to_value(&feature).unwrap();
        assert!(json.get("external").is_none());
        assert!(json.get("links").is_none());
        assert_eq!(json["key"], json!("checkout"));

        feature.external.insert("users".to_string(), json!(10));
        let json = serde_json::to_value(&feature).unwrap();
        assert_eq!(json["external"]["users"], json!(10));
    }
}
