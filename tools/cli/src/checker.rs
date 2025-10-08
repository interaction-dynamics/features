use anyhow::Result;
use std::collections::HashMap;

use crate::models::Feature;

pub fn run_checks(features: &[Feature]) -> Result<()> {
    let mut error_count = 0;

    error_count += check_duplicate_names(features);

    if error_count > 0 {
        anyhow::bail!("Check failed: {} error(s) found", error_count);
    }

    eprintln!("All checks passed successfully.");
    Ok(())
}

fn check_duplicate_names(features: &[Feature]) -> usize {
    let mut name_to_paths: HashMap<String, Vec<String>> = HashMap::new();

    fn collect_features(features: &[Feature], name_to_paths: &mut HashMap<String, Vec<String>>) {
        for feature in features {
            name_to_paths
                .entry(feature.name.clone())
                .or_insert_with(Vec::new)
                .push(feature.path.clone());

            collect_features(&feature.features, name_to_paths);
        }
    }

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
