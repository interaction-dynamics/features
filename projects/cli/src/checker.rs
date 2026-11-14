use anyhow::Result;
use std::collections::HashMap;

use crate::models::{Feature, Record};
use crate::records::record_warnings;

/// Check the features: errors fail the check, warnings are only printed.
///
/// `project_records` are the root specs and decisions not attached to a feature.
pub fn run_checks(features: &[Feature], project_records: &[Record]) -> Result<()> {
    let mut error_count = 0;

    error_count += check_duplicate_names(features);

    let warnings = record_warnings(features, project_records);
    for warning in &warnings {
        eprintln!("Warning: {}", warning);
    }

    if error_count > 0 {
        anyhow::bail!("Check failed: {} error(s) found", error_count);
    }

    if warnings.is_empty() {
        eprintln!("All checks passed successfully.");
    } else {
        eprintln!(
            "All checks passed successfully, with {} warning(s).",
            warnings.len()
        );
    }
    Ok(())
}

fn collect_features(features: &[Feature], name_to_paths: &mut HashMap<String, Vec<String>>) {
    for feature in features {
        name_to_paths
            .entry(feature.name.clone())
            .or_default()
            .push(feature.path.clone());

        collect_features(&feature.features, name_to_paths);
    }
}

fn check_duplicate_names(features: &[Feature]) -> usize {
    let mut name_to_paths: HashMap<String, Vec<String>> = HashMap::new();

    collect_features(features, &mut name_to_paths);

    let mut error_count = 0;

    for (name, paths) in name_to_paths.iter() {
        if paths.len() > 1 {
            error_count += 1;
            eprintln!(
                "Error: Duplicate feature name '{}' found in {} locations:",
                name,
                paths.len()
            );
            for path in paths {
                eprintln!("  - {}", path);
            }
        }
    }

    error_count
}
