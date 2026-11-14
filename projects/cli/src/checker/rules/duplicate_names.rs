//! `duplicate-names`: two features with the same name.

use std::collections::BTreeMap;

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::Feature;

pub struct DuplicateNames;

impl Rule for DuplicateNames {
    type Options = ();

    fn id(&self) -> &'static str {
        "duplicate-names"
    }

    fn description(&self) -> &'static str {
        "Two or more features have the same name"
    }

    fn default_severity(&self) -> Severity {
        Severity::Error
    }

    fn check(&self, features: &[Feature], _ctx: &CheckContext, _options: &()) -> Vec<Violation> {
        let mut by_name: BTreeMap<&str, Vec<&Feature>> = BTreeMap::new();
        walk_features(features, &mut |feature| {
            by_name.entry(&feature.name).or_default().push(feature);
        });

        by_name
            .into_iter()
            .filter(|(_, features)| features.len() > 1)
            .map(|(name, features)| {
                let paths: Vec<&str> = features.iter().map(|f| f.path.as_str()).collect();
                Violation::for_feature(
                    features[0],
                    format!(
                        "Duplicate feature name '{}' found in {} locations: {}",
                        name,
                        paths.len(),
                        paths.join(", ")
                    ),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checker::test_support::*;

    #[test]
    fn test_reports_duplicates_including_nested() {
        let mut parent = feature("billing", "features/billing");
        parent
            .features
            .push(feature("invoices", "features/billing/features/invoices"));
        let features = vec![
            parent,
            feature("invoices", "features/invoices"),
            feature("checkout", "features/checkout"),
        ];

        let violations = DuplicateNames.check(&features, &ctx(), &());
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].feature, "invoices");
        assert_eq!(
            violations[0].message,
            "Duplicate feature name 'invoices' found in 2 locations: features/billing/features/invoices, features/invoices"
        );
    }

    #[test]
    fn test_no_duplicates() {
        let features = vec![feature("a", "a"), feature("b", "b")];
        assert!(DuplicateNames.check(&features, &ctx(), &()).is_empty());
    }
}
