//! The check rules, one per file.

mod cross_feature_import;
mod duplicate_names;
mod min_coverage;
mod missing_owner;
mod missing_readme;
mod stale_flag;

use super::DynRule;

/// All the rules, in the order they run and are reported.
pub fn all() -> Vec<Box<dyn DynRule>> {
    vec![
        Box::new(duplicate_names::DuplicateNames),
        Box::new(missing_owner::MissingOwner),
        Box::new(missing_readme::MissingReadme),
        Box::new(min_coverage::MinCoverage),
        Box::new(stale_flag::StaleFlag),
        Box::new(cross_feature_import::CrossFeatureImport),
    ]
}
