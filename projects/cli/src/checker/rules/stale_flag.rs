//! `stale-flag`: a feature flag that has been around for too long.
//!
//! Looks at the feature metadata annotations (e.g. `// --feature-flag
//! name: new-login, introduced_on: 2025-01-01`) and reports the ones whose
//! `introduced_on` date is older than the allowed age.

use anyhow::Result;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::Feature;

pub struct StaleFlag;

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Options {
    /// Maximum age of a flag, in days
    pub max_age_days: i64,
    /// Metadata keys holding flags (`flag` for `--feature-flag` annotations)
    pub metadata_keys: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_age_days: 90,
            metadata_keys: vec!["flag".to_string()],
        }
    }
}

impl Rule for StaleFlag {
    type Options = Options;

    fn id(&self) -> &'static str {
        "stale-flag"
    }

    fn description(&self) -> &'static str {
        "A feature flag's `introduced_on` date is older than the allowed age"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn validate(&self, options: &Options) -> Result<()> {
        if options.max_age_days < 0 {
            anyhow::bail!(
                "`max-age-days` must not be negative, got {}",
                options.max_age_days
            );
        }
        Ok(())
    }

    fn check(&self, features: &[Feature], ctx: &CheckContext, options: &Options) -> Vec<Violation> {
        let mut violations = Vec::new();
        walk_features(features, &mut |feature| {
            for key in &options.metadata_keys {
                let Some(serde_json::Value::Array(entries)) = feature.meta.get(key) else {
                    continue;
                };
                for entry in entries {
                    if let Some(message) = check_entry(entry, key, ctx.today, options.max_age_days)
                    {
                        violations.push(Violation::for_feature(feature, message));
                    }
                }
            }
        });
        violations
    }
}

fn check_entry(
    entry: &serde_json::Value,
    key: &str,
    today: NaiveDate,
    max_age_days: i64,
) -> Option<String> {
    let introduced_on = entry.get("introduced_on")?.as_str()?.trim();
    let name = entry
        .get("name")
        .and_then(|name| name.as_str())
        .map(|name| format!("'{}'", name))
        .unwrap_or_else(|| "(unnamed)".to_string());

    let Ok(date) = NaiveDate::parse_from_str(introduced_on, "%Y-%m-%d") else {
        return Some(format!(
            "Feature {} {} has an invalid introduced_on date '{}' (expected YYYY-MM-DD)",
            key, name, introduced_on
        ));
    };

    let age = (today - date).num_days();
    (age > max_age_days).then(|| {
        format!(
            "Feature {} {} was introduced on {} ({} days ago, more than {})",
            key, name, introduced_on, age, max_age_days
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::test_support::*;
    use serde_json::json;

    fn with_meta(mut feature: Feature, key: &str, value: serde_json::Value) -> Feature {
        feature.meta.insert(key.to_string(), value);
        feature
    }

    #[test]
    fn test_reports_old_and_invalid_flags() {
        // ctx().today is 2026-01-01
        let features = vec![with_meta(
            feature("billing", "billing"),
            "flag",
            json!([
                {"name": "old", "introduced_on": "2025-01-01"},
                {"name": "recent", "introduced_on": "2025-12-01"},
                {"name": "limit", "introduced_on": "2025-10-03"},
                {"name": "typo", "introduced_on": "01/02/2025"},
                {"introduced_on": "2024-01-01"},
                {"name": "no-date"}
            ]),
        )];

        let messages: Vec<String> = StaleFlag
            .check(&features, &ctx(), &Options::default())
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(
            messages,
            vec![
                "Feature flag 'old' was introduced on 2025-01-01 (365 days ago, more than 90)",
                "Feature flag 'typo' has an invalid introduced_on date '01/02/2025' (expected YYYY-MM-DD)",
                "Feature flag (unnamed) was introduced on 2024-01-01 (731 days ago, more than 90)",
            ]
        );
    }

    #[test]
    fn test_metadata_keys_and_age_are_configurable() {
        let features = vec![with_meta(
            feature("billing", "billing"),
            "experiment",
            json!([{"name": "exp", "introduced_on": "2025-12-01"}]),
        )];

        assert!(
            StaleFlag
                .check(&features, &ctx(), &Options::default())
                .is_empty()
        );

        let options = Options {
            max_age_days: 7,
            metadata_keys: vec!["experiment".to_string()],
        };
        assert_eq!(StaleFlag.check(&features, &ctx(), &options).len(), 1);
    }

    #[test]
    fn test_ignores_non_array_metadata() {
        let features = vec![with_meta(
            feature("billing", "billing"),
            "flag",
            json!("introduced_on: 2000-01-01"),
        )];
        assert!(
            StaleFlag
                .check(&features, &ctx(), &Options::default())
                .is_empty()
        );
    }

    #[test]
    fn test_max_age_must_not_be_negative() {
        let options = Options {
            max_age_days: -1,
            ..Default::default()
        };
        assert!(StaleFlag.validate(&options).is_err());
    }
}
