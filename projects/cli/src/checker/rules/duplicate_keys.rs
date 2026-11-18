//! `duplicate-keys`: several features with the same feature key.
//!
//! Only a warning by default: data imported with `--data` goes to every
//! feature with the key, which may be intended.

use crate::checker::{CheckContext, Rule, Severity, Violation, duplicate_keys, walk_features};
use crate::models::Feature;

pub struct DuplicateKeys;

impl Rule for DuplicateKeys {
    type Options = ();

    fn id(&self) -> &'static str {
        "duplicate-keys"
    }

    fn description(&self) -> &'static str {
        "Two or more features have the same feature key"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, features: &[Feature], _ctx: &CheckContext, _options: &()) -> Vec<Violation> {
        let duplicates = duplicate_keys(features);
        let mut violations = Vec::new();
        walk_features(features, &mut |feature| {
            let Some(paths) = duplicates.get(&feature.key) else {
                return;
            };
            // One finding per key, on its first feature
            if paths[0] != feature.path {
                return;
            }
            violations.push(Violation::for_feature(
                feature,
                format!(
                    "Feature key '{}' is used by {} features ({}); set a unique `key` in their README front matter to tell them apart",
                    feature.key,
                    paths.len(),
                    paths.join(", ")
                ),
            ));
        });
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::test_support::*;

    #[test]
    fn test_reports_keys_shared_across_nesting_levels() {
        let mut admin = feature("admin", "features/admin");
        admin
            .features
            .push(feature("settings", "features/admin/features/settings"));
        let mut other = feature("other-settings", "features/settings");
        other.key = "settings".to_string();
        let features = vec![admin, other];

        let violations = DuplicateKeys.check(&features, &ctx(), &());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].path, "features/admin/features/settings");
        assert!(
            violations[0]
                .message
                .starts_with("Feature key 'settings' is used by 2 features"),
            "{}",
            violations[0].message
        );
    }

    #[test]
    fn test_unique_keys() {
        let features = vec![feature("a", "a"), feature("b", "b")];
        assert!(DuplicateKeys.check(&features, &ctx(), &()).is_empty());
    }
}
