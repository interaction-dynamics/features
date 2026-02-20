//! Module for resolving dependencies between features
//!
//! This module takes import statements and determines which features they belong to,
//! and what type of relationship exists between features (parent, child, sibling).

use crate::import_detector::{ImportStatement, resolve_import_path};
use crate::models::{Dependency, DependencyType};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Represents a feature with its path for dependency resolution
#[derive(Debug, Clone)]
pub struct FeatureInfo {
    pub name: String,
    pub path: PathBuf,
}

/// Build a map of file paths to their containing features (using feature path as identifier)
pub fn build_file_to_feature_map(
    features: &[FeatureInfo],
    base_path: &Path,
) -> HashMap<PathBuf, String> {
    let mut map = HashMap::new();

    // Sort features by path length (longest first) to ensure more specific features
    // take precedence over parent features when mapping files
    let mut sorted_features = features.to_vec();
    sorted_features.sort_by(|a, b| {
        b.path
            .to_string_lossy()
            .len()
            .cmp(&a.path.to_string_lossy().len())
    });

    for feature in sorted_features {
        let feature_path = base_path.join(&feature.path);

        // Map all files within this feature directory using the feature's path as identifier
        if std::fs::read_dir(&feature_path).is_ok() {
            map_directory_files(&feature_path, &feature.path.to_string_lossy(), &mut map);
        }
    }

    map
}

/// Recursively map all files in a directory to a feature path
fn map_directory_files(dir: &Path, feature_path: &str, map: &mut HashMap<PathBuf, String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                // Canonicalize the path to resolve .. and .
                if let Ok(canonical_path) = path.canonicalize() {
                    map.insert(canonical_path, feature_path.to_string());
                } else {
                    map.insert(path.clone(), feature_path.to_string());
                }
            } else if path.is_dir()
                && let Some(dir_name) = path.file_name().and_then(|n| n.to_str())
                && !should_skip_directory(dir_name)
                && !is_nested_feature_directory(&path)
            {
                map_directory_files(&path, feature_path, map);
            }
        }
    }
}

/// Check if a directory is a nested feature directory
/// A directory is considered a feature if:
/// 1. It's a direct child of a "features" directory, OR
/// 2. It has a features.toml file
fn is_nested_feature_directory(dir: &Path) -> bool {
    // Check for features.toml
    if dir.join("features.toml").exists() {
        return true;
    }

    // Check if it's a direct child of a "features" directory
    if let Some(parent) = dir.parent()
        && let Some(parent_name) = parent.file_name()
        && parent_name == "features"
    {
        return true;
    }

    false
}

/// Check if a directory should be skipped
fn should_skip_directory(dir_name: &str) -> bool {
    matches!(
        dir_name,
        "node_modules" | "target" | "dist" | "build" | ".git" | "__pycache__" | "coverage"
    )
}

/// Determine the relationship type between two features based on their paths
pub fn determine_dependency_type(
    source_feature_path: &Path,
    target_feature_path: &Path,
) -> DependencyType {
    // Check if target is a child (descendant) of source
    if target_feature_path.starts_with(source_feature_path) {
        return DependencyType::Child;
    }

    // Check if source is a child (descendant) of target
    if source_feature_path.starts_with(target_feature_path) {
        return DependencyType::Parent;
    }

    // Otherwise, they're siblings
    DependencyType::Sibling
}

/// Resolve imports to dependencies for a specific feature
pub fn resolve_feature_dependencies(
    _feature_name: &str,
    feature_path: &Path,
    base_path: &Path,
    imports: &[ImportStatement],
    file_to_feature_map: &HashMap<PathBuf, String>,
    feature_path_to_name_map: &HashMap<String, String>,
    file_map: &HashMap<String, PathBuf>,
) -> Vec<Dependency> {
    let mut dependencies = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for import in imports {
        let source_file = Path::new(&import.file_path);

        // Resolve the import to an actual file path
        if let Some(resolved_path) =
            resolve_import_path(&import.imported_path, source_file, base_path, file_map)
        {
            // Find which feature this file belongs to (returns feature path)
            if let Some(target_feature_path_str) = file_to_feature_map.get(&resolved_path) {
                // Skip if it's the same feature
                if target_feature_path_str == feature_path.to_string_lossy().as_ref() {
                    continue;
                }

                // Create a unique key to avoid duplicates
                let dep_key = format!(
                    "{}:{}:{}",
                    resolved_path.display(),
                    import.line_number,
                    target_feature_path_str
                );

                if seen.contains(&dep_key) {
                    continue;
                }
                seen.insert(dep_key);

                // Get the target feature name from the path (for validation)
                if feature_path_to_name_map
                    .get(target_feature_path_str)
                    .is_some()
                {
                    let target_path = PathBuf::from(target_feature_path_str);
                    let full_target_path = base_path.join(&target_path);
                    let full_source_path = base_path.join(feature_path);

                    // Determine the dependency type
                    let dependency_type =
                        determine_dependency_type(&full_source_path, &full_target_path);

                    // Convert source file path to be relative to base_path
                    let relative_source_filename = if let Ok(canonical_base) =
                        base_path.canonicalize()
                    {
                        let source_path = Path::new(&import.file_path);
                        if let Ok(canonical_source) = source_path.canonicalize() {
                            if let Ok(rel_path) = canonical_source.strip_prefix(&canonical_base) {
                                rel_path.to_string_lossy().to_string()
                            } else {
                                import.file_path.clone()
                            }
                        } else {
                            import.file_path.clone()
                        }
                    } else {
                        import.file_path.clone()
                    };

                    // Convert target file path to be relative to base_path
                    let relative_target_filename =
                        if let Ok(canonical_base) = base_path.canonicalize() {
                            if let Ok(rel_path) = resolved_path.strip_prefix(&canonical_base) {
                                rel_path.to_string_lossy().to_string()
                            } else {
                                resolved_path.to_string_lossy().to_string()
                            }
                        } else {
                            resolved_path.to_string_lossy().to_string()
                        };

                    // Create dependency
                    dependencies.push(Dependency {
                        source_filename: relative_source_filename,
                        target_filename: relative_target_filename,
                        line: import.line_number,
                        content: import.line_content.clone(),
                        feature_path: target_feature_path_str.to_string(),
                        dependency_type,
                    });
                }
            }
        }
    }

    dependencies
}

/// Collect all feature information recursively
pub fn collect_feature_info(
    features: &[crate::models::Feature],
    _parent_path: Option<&Path>,
    result: &mut Vec<FeatureInfo>,
) {
    for feature in features {
        // Feature paths are already relative to base, not to parent
        let feature_path = PathBuf::from(&feature.path);

        result.push(FeatureInfo {
            name: feature.name.clone(),
            path: feature_path.clone(),
        });

        // Recursively collect nested features (don't pass parent_path since paths are base-relative)
        if !feature.features.is_empty() {
            collect_feature_info(&feature.features, None, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_dependency_type_child() {
        let source = Path::new("/project/features/parent");
        let target = Path::new("/project/features/parent/child");

        assert!(matches!(
            determine_dependency_type(source, target),
            DependencyType::Child
        ));
    }

    #[test]
    fn test_determine_dependency_type_parent() {
        let source = Path::new("/project/features/parent/child");
        let target = Path::new("/project/features/parent");

        assert!(matches!(
            determine_dependency_type(source, target),
            DependencyType::Parent
        ));
    }

    #[test]
    fn test_determine_dependency_type_sibling() {
        let source = Path::new("/project/features/feature-a");
        let target = Path::new("/project/features/feature-b");

        assert!(matches!(
            determine_dependency_type(source, target),
            DependencyType::Sibling
        ));
    }
}
