use anyhow::Result;
use std::collections::{BTreeMap, HashMap};

use crate::models::Feature;

pub fn run_checks(features: &[Feature]) -> Result<()> {
    let mut error_count = 0;

    error_count += check_duplicate_names(features);
    warn_duplicate_keys(features);

    if error_count > 0 {
        anyhow::bail!("Check failed: {} error(s) found", error_count);
    }

    eprintln!("All checks passed successfully.");
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

fn collect_keys(features: &[Feature], key_to_paths: &mut BTreeMap<String, Vec<String>>) {
    for feature in features {
        key_to_paths
            .entry(feature.key.clone())
            .or_default()
            .push(feature.path.clone());

        collect_keys(&feature.features, key_to_paths);
    }
}

/// Keys shared by several features, with the paths of those features
pub fn duplicate_keys(features: &[Feature]) -> BTreeMap<String, Vec<String>> {
    let mut key_to_paths = BTreeMap::new();
    collect_keys(features, &mut key_to_paths);
    key_to_paths.retain(|_, paths| paths.len() > 1);
    key_to_paths
}

/// Duplicate keys are only a warning: data imported with `--data` goes to every
/// feature with the key, which may be intended, and failing would break existing CI.
fn warn_duplicate_keys(features: &[Feature]) {
    for (key, paths) in duplicate_keys(features) {
        eprintln!(
            "Warning: feature key '{}' is used by {} features (set a unique `key` in their README front matter to tell them apart):",
            key,
            paths.len()
        );
        for path in paths {
            eprintln!("  - {}", path);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn feature(key: &str, path: &str, children: Vec<Feature>) -> Feature {
        Feature {
            name: path.to_string(),
            key: key.to_string(),
            description: String::new(),
            owner: String::new(),
            is_owner_inherited: false,
            path: path.to_string(),
            features: children,
            meta: BTreeMap::new(),
            links: BTreeMap::new(),
            changes: Vec::new(),
            decisions: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: BTreeMap::new(),
        }
    }

    #[test]
    fn test_duplicate_keys_are_found_across_nesting_levels() {
        let features = vec![
            feature(
                "admin",
                "features/admin",
                vec![feature(
                    "settings",
                    "features/admin/features/settings",
                    vec![],
                )],
            ),
            feature("settings", "features/settings", vec![]),
        ];

        let duplicates = duplicate_keys(&features);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(
            duplicates["settings"],
            vec![
                "features/admin/features/settings".to_string(),
                "features/settings".to_string()
            ]
        );

        // A duplicate key is a warning, not an error
        assert!(run_checks(&features).is_ok());
    }
}
