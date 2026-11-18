//! `missing-owner`: a feature nobody owns.

use serde::Deserialize;

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::Feature;

pub struct MissingOwner;

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Options {
    /// Whether an owner inherited from a parent feature is enough
    pub allow_inherited: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            allow_inherited: true,
        }
    }
}

impl Rule for MissingOwner {
    type Options = Options;

    fn id(&self) -> &'static str {
        "missing-owner"
    }

    fn description(&self) -> &'static str {
        "A feature has no owner"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(
        &self,
        features: &[Feature],
        _ctx: &CheckContext,
        options: &Options,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();
        walk_features(features, &mut |feature| {
            if feature.owner.trim().is_empty() {
                violations.push(Violation::for_feature(feature, "Feature has no owner"));
            } else if feature.is_owner_inherited && !options.allow_inherited {
                violations.push(Violation::for_feature(
                    feature,
                    format!(
                        "Feature has no owner of its own (inherits '{}')",
                        feature.owner
                    ),
                ));
            }
        });
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::test_support::*;

    fn features() -> Vec<Feature> {
        let mut parent = feature("billing", "billing");
        let mut child = feature("invoices", "billing/features/invoices");
        child.is_owner_inherited = true;
        parent.features.push(child);

        let mut orphan = feature("orphan", "orphan");
        orphan.owner = String::new();

        vec![parent, orphan]
    }

    #[test]
    fn test_reports_features_without_owner() {
        let violations = MissingOwner.check(&features(), &ctx(), &Options::default());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].path, "orphan");
        assert_eq!(violations[0].message, "Feature has no owner");
    }

    #[test]
    fn test_reports_inherited_owners_when_not_allowed() {
        let options = Options {
            allow_inherited: false,
        };
        let violations = MissingOwner.check(&features(), &ctx(), &options);
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].path, "billing/features/invoices");
        assert_eq!(
            violations[0].message,
            "Feature has no owner of its own (inherits 'team')"
        );
    }
}
