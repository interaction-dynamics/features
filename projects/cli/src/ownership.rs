//! Map files and folders to the feature (and owner) that contains them.
//!
//! Matching works on paths relative to the scanned base path, component by
//! component, so it also works for paths that no longer exist on disk (for
//! example files deleted in a diff).

use std::path::{Component, Path, PathBuf};

use crate::models::Feature;

/// The feature that owns a path, and who owns that feature.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct OwnerInfo {
    pub owner: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub inherited: bool,
    pub feature_name: String,
    pub feature_path: String,
}

impl OwnerInfo {
    fn from_feature(feature: &Feature) -> Self {
        Self {
            owner: feature.owner.clone(),
            inherited: feature.is_owner_inherited,
            feature_name: feature.name.clone(),
            feature_path: feature.path.clone(),
        }
    }
}

/// Find the owner of an existing file or folder.
///
/// `target_path` and `base_path` may be relative to the current directory;
/// both are canonicalized, so the target must exist.
pub fn find_owner_for_path(
    target_path: &Path,
    features: &[Feature],
    base_path: &Path,
) -> Option<OwnerInfo> {
    let canonical_target = std::fs::canonicalize(target_path).ok()?;
    let canonical_base = std::fs::canonicalize(base_path).ok()?;
    let relative = canonical_target.strip_prefix(&canonical_base).ok()?;
    find_owner_for_relative_path(relative, features)
}

/// Find the owner of a path given relative to the scanned base path.
///
/// The path does not need to exist. Returns the deepest feature whose folder
/// contains the path, or `None` when the path is outside every feature.
pub fn find_owner_for_relative_path(
    relative_path: &Path,
    features: &[Feature],
) -> Option<OwnerInfo> {
    let target = normalize(relative_path);
    find_closest_feature(&target, features).map(OwnerInfo::from_feature)
}

fn find_closest_feature<'a>(target: &Path, features: &'a [Feature]) -> Option<&'a Feature> {
    features
        .iter()
        .find(|feature| target.starts_with(normalize(Path::new(&feature.path))))
        .map(|feature| find_closest_feature(target, &feature.features).unwrap_or(feature))
}

/// Drop `.` components and resolve `..` lexically, so `./a/../b` matches `b`.
fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other),
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn feature(name: &str, path: &str, owner: &str, children: Vec<Feature>) -> Feature {
        Feature {
            name: name.to_string(),
            description: String::new(),
            owner: owner.to_string(),
            is_owner_inherited: false,
            path: path.to_string(),
            features: children,
            meta: BTreeMap::new(),
            changes: Vec::new(),
            decisions: Vec::new(),
            records: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
        }
    }

    fn sample() -> Vec<Feature> {
        vec![
            feature(
                "auth",
                "features/auth",
                "team-a",
                vec![feature(
                    "login",
                    "features/auth/features/login",
                    "team-b",
                    vec![],
                )],
            ),
            feature("billing", "features/billing", "team-c", vec![]),
            feature("bill", "features/bill", "team-d", vec![]),
        ]
    }

    #[test]
    fn matches_file_directly_in_feature() {
        let info = find_owner_for_relative_path(Path::new("features/auth/index.ts"), &sample());
        let info = info.unwrap();
        assert_eq!(info.feature_path, "features/auth");
        assert_eq!(info.owner, "team-a");
    }

    #[test]
    fn prefers_deepest_nested_feature() {
        let info = find_owner_for_relative_path(
            Path::new("features/auth/features/login/a/b.ts"),
            &sample(),
        )
        .unwrap();
        assert_eq!(info.feature_name, "login");
        assert_eq!(info.owner, "team-b");
    }

    #[test]
    fn matches_the_feature_folder_itself() {
        let info = find_owner_for_relative_path(Path::new("features/billing"), &sample()).unwrap();
        assert_eq!(info.feature_name, "billing");
    }

    #[test]
    fn matches_whole_components_only() {
        // `features/billing` must not match the `features/bill` feature
        let info =
            find_owner_for_relative_path(Path::new("features/billing/x.ts"), &sample()).unwrap();
        assert_eq!(info.feature_name, "billing");
        let info =
            find_owner_for_relative_path(Path::new("features/bill/x.ts"), &sample()).unwrap();
        assert_eq!(info.feature_name, "bill");
    }

    #[test]
    fn returns_none_outside_features() {
        assert!(find_owner_for_relative_path(Path::new("src/utils/x.ts"), &sample()).is_none());
        assert!(find_owner_for_relative_path(Path::new("features"), &sample()).is_none());
    }

    #[test]
    fn normalizes_dot_components() {
        let info = find_owner_for_relative_path(Path::new("./features/x/../auth/a.ts"), &sample())
            .unwrap();
        assert_eq!(info.feature_name, "auth");
    }

    #[test]
    fn reports_inherited_owner() {
        let mut child = feature("child", "features/parent/features/child", "team-a", vec![]);
        child.is_owner_inherited = true;
        let features = vec![feature("parent", "features/parent", "team-a", vec![child])];
        let info = find_owner_for_relative_path(
            Path::new("features/parent/features/child/a.ts"),
            &features,
        )
        .unwrap();
        assert_eq!(info.feature_name, "child");
        assert_eq!(info.owner, "team-a");
        assert!(info.inherited);
    }
}
