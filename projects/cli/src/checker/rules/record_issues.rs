//! `record-issues`: problems in the specs and decision records.
//!
//! Broken links between specs and decisions, records without a status,
//! superseded records without a replacement and implemented specs with open
//! tasks, for every feature and for the project-level records.

use crate::checker::{CheckContext, Rule, Severity, Violation, walk_features};
use crate::models::Feature;
use crate::records::record_warnings;

pub struct RecordIssues;

impl Rule for RecordIssues {
    type Options = ();

    fn id(&self) -> &'static str {
        "record-issues"
    }

    fn description(&self) -> &'static str {
        "A spec or decision record has a broken link, no status, or is inconsistent"
    }

    fn default_severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(&self, features: &[Feature], ctx: &CheckContext, _options: &()) -> Vec<Violation> {
        let project = &ctx.project_records;
        let mut violations: Vec<Violation> = record_warnings(project, &[], "project")
            .into_iter()
            .map(|message| Violation {
                feature: "project".to_string(),
                path: ".".to_string(),
                file: None,
                line: None,
                message,
            })
            .collect();

        walk_features(features, &mut |feature| {
            let owner = format!("feature '{}'", feature.name);
            for message in record_warnings(&feature.records, project, &owner) {
                violations.push(Violation::for_feature(feature, message));
            }
        });
        violations
    }
}
