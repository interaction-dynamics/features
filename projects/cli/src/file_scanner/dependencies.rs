//! Dependencies between features, resolved from the imports in their files.

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::walk::{ReadmeCache, WalkFilter, is_documentation_directory, is_feature_directory};
use crate::dependency_resolver::{
    build_file_to_feature_map, collect_feature_info, resolve_feature_dependencies,
};
use crate::import_detector::{ImportStatement, build_file_map, scan_file_for_imports};
use crate::models::Feature;

/// Populate dependencies for all features by scanning imports
pub(super) fn populate_dependencies(
    features: &mut [Feature],
    base_path: &Path,
    filter: &WalkFilter,
    readmes: &ReadmeCache,
) -> Result<()> {
    // Build file map for quick path resolution
    let file_map = build_file_map(base_path);

    // Collect all feature info (flat list with paths)
    let mut feature_info_list = Vec::new();
    collect_feature_info(features, None, &mut feature_info_list);

    // Build file-to-feature mapping
    let file_to_feature_map = build_file_to_feature_map(&feature_info_list, base_path);

    // Build feature path to name mapping (path is the unique identifier)
    let mut feature_path_to_name_map = HashMap::new();
    for info in &feature_info_list {
        feature_path_to_name_map.insert(info.path.to_string_lossy().to_string(), info.name.clone());
    }

    // Scan all files in each feature for imports
    let mut feature_imports: HashMap<String, Vec<ImportStatement>> = HashMap::new();

    for feature_info in &feature_info_list {
        let feature_path = base_path.join(&feature_info.path);
        let imports = scan_feature_directory_for_imports(&feature_path, filter, readmes);
        // Use feature path as key instead of name to handle features with duplicate names
        feature_imports.insert(feature_info.path.to_string_lossy().to_string(), imports);
    }

    // Now populate dependencies in the feature tree
    populate_dependencies_recursive(
        features,
        base_path,
        &feature_imports,
        &file_to_feature_map,
        &feature_path_to_name_map,
        &file_map,
    );

    Ok(())
}

/// Scan a feature directory for all import statements
fn scan_feature_directory_for_imports(
    feature_path: &Path,
    filter: &WalkFilter,
    readmes: &ReadmeCache,
) -> Vec<ImportStatement> {
    let mut all_imports = Vec::new();

    if let Ok(entries) = fs::read_dir(feature_path) {
        // Collect and sort entries alphabetically by filename
        let mut paths: Vec<_> = entries.flatten().map(|e| e.path()).collect();
        paths.sort_by(|a, b| {
            let name_a = a.file_name().unwrap_or_default().to_string_lossy();
            let name_b = b.file_name().unwrap_or_default().to_string_lossy();
            name_a.cmp(&name_b)
        });

        for path in paths {
            // Skip documentation directories
            if is_documentation_directory(&path) {
                continue;
            }

            if path.is_file() {
                if let Ok(imports) = scan_file_for_imports(&path) {
                    all_imports.extend(imports);
                }
            } else if path.is_dir() {
                // Skip 'features' subdirectory (contains nested features)
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if dir_name == "features" || filter.skips(&path) {
                    continue;
                }

                // Skip nested feature directories (check if it's a direct child of "features" directory with README)
                if is_feature_directory(&path, readmes) {
                    continue;
                }

                // Recursively scan subdirectories
                let nested_imports = scan_feature_directory_for_imports(&path, filter, readmes);
                all_imports.extend(nested_imports);
            }
        }
    }

    all_imports
}

/// Recursively populate dependencies in the feature tree
fn populate_dependencies_recursive(
    features: &mut [Feature],
    base_path: &Path,
    feature_imports: &HashMap<String, Vec<ImportStatement>>,
    file_to_feature_map: &HashMap<PathBuf, String>,
    feature_path_to_name_map: &HashMap<String, String>,
    file_map: &HashMap<String, PathBuf>,
) {
    for feature in features {
        // Get imports for this feature using path as key (not name, since multiple features can have same name)
        if let Some(imports) = feature_imports.get(&feature.path) {
            let feature_path = PathBuf::from(&feature.path);

            // Resolve dependencies
            let dependencies = resolve_feature_dependencies(
                &feature.name,
                &feature_path,
                base_path,
                imports,
                file_to_feature_map,
                feature_path_to_name_map,
                file_map,
            );

            feature.dependencies = dependencies;
        }

        // Recursively process nested features
        if !feature.features.is_empty() {
            populate_dependencies_recursive(
                &mut feature.features,
                base_path,
                feature_imports,
                file_to_feature_map,
                feature_path_to_name_map,
                file_map,
            );
        }
    }
}
