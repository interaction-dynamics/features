//! Discovers the feature tree: which folders are features, their README or
//! FEATURES.toml information and who owns them.
//!
//! The walker only builds the structure. Everything that reads source files or
//! git history is added afterwards by the passes in the sibling modules.

use anyhow::{Context, Result};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::feature_key::{take_key, take_links};
use crate::features_toml_parser::{find_features_toml, read_features_toml};
use crate::models::Feature;
use crate::readme_parser::{ReadmeInfo, parse_readme_info, read_readme_info};
use crate::skip_dirs::SKIPPED_DIRS;

pub(super) fn is_documentation_directory(dir_path: &Path) -> bool {
    let dir_name = dir_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // Common documentation directory names
    let doc_dirs = ["docs", "__docs__", ".docs"];

    doc_dirs.contains(&dir_name.to_lowercase().as_str())
}

fn is_inside_documentation_directory(dir_path: &Path) -> bool {
    // Check if any parent directory is a documentation directory
    for ancestor in dir_path.ancestors().skip(1) {
        if is_documentation_directory(ancestor) {
            return true;
        }
    }
    false
}

fn is_direct_subfolder_of_features(dir_path: &Path) -> bool {
    if let Some(parent) = dir_path.parent()
        && let Some(parent_name) = parent.file_name().and_then(|name| name.to_str())
    {
        return parent_name == "features";
    }
    false
}

/// Check if a directory is ignored, i.e. matches (or is nested inside) one of the
/// paths passed via `--ignore-path`. Ignored directories and all their subfolders
/// are excluded from feature detection entirely.
fn is_ignored_path(dir_path: &Path, ignore_paths: &[PathBuf]) -> bool {
    if ignore_paths.is_empty() {
        return false;
    }

    let Ok(canonical) = fs::canonicalize(dir_path) else {
        return false;
    };

    ignore_paths
        .iter()
        .any(|ignored| canonical.starts_with(ignored))
}

/// Decides which folders the feature, stats and import walkers leave out
#[derive(Clone, Copy)]
pub(super) struct WalkFilter<'a> {
    pub(super) ignore_paths: &'a [PathBuf],
    /// Scan the folders in `SKIPPED_DIRS` instead of skipping them
    pub(super) include_skipped_dirs: bool,
}

impl WalkFilter<'_> {
    pub(super) fn skips(&self, dir_path: &Path) -> bool {
        let is_skipped_name = !self.include_skipped_dirs
            && dir_path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| SKIPPED_DIRS.contains(&name));

        is_skipped_name || is_ignored_path(dir_path, self.ignore_paths)
    }
}

fn find_readme_file(dir_path: &Path) -> Option<std::path::PathBuf> {
    let readme_candidates = ["README.md", "README.mdx"];

    for candidate in &readme_candidates {
        let readme_path = dir_path.join(candidate);
        if readme_path.exists() {
            return Some(readme_path);
        }
    }

    None
}

/// A directory's README, read and parsed at most once per scan
struct Readme {
    path: PathBuf,
    /// None when the file exists but can't be read as text
    info: Option<ReadmeInfo>,
}

#[derive(Default)]
pub(super) struct ReadmeCache(RefCell<HashMap<PathBuf, Option<Rc<Readme>>>>);

impl ReadmeCache {
    fn get(&self, dir_path: &Path) -> Option<Rc<Readme>> {
        if let Some(cached) = self.0.borrow().get(dir_path) {
            return cached.clone();
        }

        let readme = find_readme_file(dir_path).map(|path| {
            let info = fs::read_to_string(&path)
                .ok()
                .map(|content| parse_readme_info(&content));
            Rc::new(Readme { path, info })
        });

        self.0
            .borrow_mut()
            .insert(dir_path.to_path_buf(), readme.clone());
        readme
    }

    /// The `feature` boolean from a directory's README front matter, if present.
    fn feature_flag(&self, dir_path: &Path) -> Option<bool> {
        self.get(dir_path)?.info.as_ref()?.feature_flag
    }
}

/// Check if a directory should be treated as a feature
pub(super) fn is_feature_directory(dir_path: &Path, readmes: &ReadmeCache) -> bool {
    // Skip documentation directories
    if is_documentation_directory(dir_path) || is_inside_documentation_directory(dir_path) {
        return false;
    }

    // An explicit `feature: false` in the README always skips this directory
    // from being a feature itself, even if it would otherwise qualify (e.g. a
    // direct subfolder of `features`). Its own subfolders are still scanned
    // normally, so any nested features it contains are kept.
    let feature_flag = readmes.feature_flag(dir_path);
    if feature_flag == Some(false) {
        return false;
    }

    // Check if it's a direct subfolder of "features" (existing behavior)
    if is_direct_subfolder_of_features(dir_path) {
        return true;
    }

    // Check if the directory has a README with feature: true
    feature_flag == Some(true)
}

/// Inputs shared by every level of the recursive feature walk
#[derive(Clone, Copy)]
pub(super) struct Walker<'a> {
    pub(super) base_path: &'a Path,
    pub(super) filter: WalkFilter<'a>,
    pub(super) readmes: &'a ReadmeCache,
}

impl Walker<'_> {
    /// Find the features under `dir`, nested features included
    pub(super) fn list_features(
        &self,
        dir: &Path,
        parent_owner: Option<&str>,
    ) -> Result<Vec<Feature>> {
        let entries = fs::read_dir(dir)
            .with_context(|| format!("could not read directory `{}`", dir.display()))?;

        let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;
        entries.sort_by_key(|entry| entry.path());

        let mut features: Vec<Feature> = Vec::new();

        for entry in entries {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();

            if path.is_dir() {
                if self.filter.skips(&path) {
                    continue;
                }

                if is_feature_directory(&path, self.readmes) {
                    let feature = self.process_feature_directory(&path, &name, parent_owner)?;
                    features.push(feature);
                } else if !is_documentation_directory(&path)
                    && !is_inside_documentation_directory(&path)
                {
                    // Recursively search for features in non-documentation subdirectories
                    let new_features = self.list_features(&path, parent_owner)?;
                    features.extend(new_features);
                }
            }
        }

        Ok(features)
    }

    fn process_feature_directory(
        &self,
        path: &Path,
        name: &str,
        parent_owner: Option<&str>,
    ) -> Result<Feature> {
        // First try to find and read FEATURES.toml file
        let (title, owner, description, mut meta) =
            if let Some(toml_path) = find_features_toml(path) {
                if let Ok(toml_data) = read_features_toml(&toml_path) {
                    (
                        toml_data.name,
                        toml_data.owner.unwrap_or_default(),
                        toml_data.description.unwrap_or_default(),
                        toml_data.meta,
                    )
                } else {
                    (None, String::new(), String::new(), BTreeMap::new())
                }
            } else {
                // Fall back to README file if FEATURES.toml not found
                let readme_info = match self.readmes.get(path) {
                    Some(readme) => match &readme.info {
                        Some(info) => info.clone(),
                        // Read it again to report why it can't be read
                        None => read_readme_info(&readme.path)?,
                    },
                    None => ReadmeInfo::default(),
                };
                (
                    readme_info.title,
                    readme_info.owner,
                    readme_info.description,
                    readme_info.meta,
                )
            };

        // Remove the 'feature' key from meta if it exists (it's redundant since we know it's a feature)
        meta.remove("feature");

        // `key` and `links` are first-class fields, not free-form metadata
        let key = take_key(&mut meta, name);
        let links = take_links(&mut meta);

        // Determine the actual owner and whether it's inherited
        let (actual_owner, is_owner_inherited) = if owner.is_empty() {
            if let Some(parent) = parent_owner {
                (parent.to_string(), true)
            } else {
                ("".to_string(), false)
            }
        } else {
            (owner.clone(), false)
        };

        // Check if this feature has nested features in a 'features' subdirectory
        let nested_features_path = path.join("features");
        let mut nested_features = if nested_features_path.exists()
            && nested_features_path.is_dir()
            && !self.filter.skips(&nested_features_path)
        {
            self.list_features(&nested_features_path, Some(&actual_owner))
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        // Also check for nested features marked with feature: true in subdirectories
        let entries = fs::read_dir(path)
            .with_context(|| format!("could not read directory `{}`", path.display()))?;

        let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;
        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let entry_path = entry.path();
            let entry_name = entry_path.file_name().unwrap().to_string_lossy();

            if entry_path.is_dir()
                && entry_name != "features" // Don't process 'features' folder twice
                && !is_documentation_directory(&entry_path)
                && !self.filter.skips(&entry_path)
            {
                if self.readmes.feature_flag(&entry_path) == Some(true) {
                    // This directory is a feature itself
                    let nested_feature = self.process_feature_directory(
                        &entry_path,
                        &entry_name,
                        Some(&actual_owner),
                    )?;
                    nested_features.push(nested_feature);
                } else {
                    // This directory is not a feature, but might contain features
                    // Recursively search for features inside it
                    let deeper_features = self.list_features(&entry_path, Some(&actual_owner))?;
                    nested_features.extend(deeper_features);
                }
            }
        }

        // Make path relative to base_path
        let relative_path = path
            .strip_prefix(self.base_path)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        Ok(Feature {
            name: title.unwrap_or_else(|| name.to_string()),
            key,
            description,
            owner: actual_owner,
            is_owner_inherited,
            path: relative_path,
            features: nested_features,
            meta,
            links,
            // Filled in by the enrichment passes
            changes: Vec::new(),
            decisions: Vec::new(),
            stats: None,
            dependencies: Vec::new(),
            external: BTreeMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_readme(dir: &Path, content: &str) {
        fs::write(dir.join("README.md"), content).unwrap();
    }

    fn walk(base: &Path) -> Vec<Feature> {
        let readmes = ReadmeCache::default();
        let walker = Walker {
            base_path: base,
            filter: WalkFilter {
                ignore_paths: &[],
                include_skipped_dirs: false,
            },
            readmes: &readmes,
        };
        walker.list_features(base, None).unwrap()
    }

    #[test]
    fn test_feature_key_and_links() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let features_dir = temp_dir.path().join("features");

        let default_key = features_dir.join("Checkout Flow");
        fs::create_dir_all(&default_key).unwrap();

        let readme_key = features_dir.join("search");
        fs::create_dir_all(&readme_key).unwrap();
        write_readme(
            &readme_key,
            "---\nkey: search-v2\nstatus: active\nlinks:\n  posthog: https://eu.posthog.com/project/1\n---\n\n# Search\n",
        );

        let toml_key = features_dir.join("billing");
        fs::create_dir_all(&toml_key).unwrap();
        fs::write(
            toml_key.join("FEATURES.toml"),
            "key = \"payments\"\n\n[links]\nsentry = \"https://sentry.io/acme\"\n",
        )
        .unwrap();

        let features = walk(temp_dir.path());
        let by_name = |name: &str| features.iter().find(|f| f.name == name).unwrap();

        let checkout = by_name("Checkout Flow");
        assert_eq!(checkout.key, "checkout-flow");
        assert!(checkout.links.is_empty());

        let search = by_name("Search");
        assert_eq!(search.key, "search-v2");
        assert_eq!(search.links["posthog"], "https://eu.posthog.com/project/1");
        assert!(!search.meta.contains_key("key"));
        assert!(!search.meta.contains_key("links"));
        assert!(search.meta.contains_key("status"));

        let billing = by_name("billing");
        assert_eq!(billing.key, "payments");
        assert_eq!(billing.links["sentry"], "https://sentry.io/acme");
        assert!(billing.meta.is_empty());
    }

    #[test]
    fn test_feature_flag_in_readme_true() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        write_readme(temp_dir.path(), "---\nfeature: true\n---\n\n# Feature\n");

        let readmes = ReadmeCache::default();
        assert_eq!(readmes.feature_flag(temp_dir.path()), Some(true));
    }

    #[test]
    fn test_feature_flag_in_readme_false() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        write_readme(temp_dir.path(), "---\nfeature: false\n---\n\n# Feature\n");

        let readmes = ReadmeCache::default();
        assert_eq!(readmes.feature_flag(temp_dir.path()), Some(false));
    }

    #[test]
    fn test_feature_flag_in_readme_missing() {
        let temp_dir = tempfile::TempDir::new().unwrap();

        // No README at all
        assert_eq!(ReadmeCache::default().feature_flag(temp_dir.path()), None);

        // README without a `feature` property
        write_readme(temp_dir.path(), "---\nowner: team\n---\n\n# Feature\n");
        assert_eq!(ReadmeCache::default().feature_flag(temp_dir.path()), None);
    }

    #[test]
    fn test_readme_cache_reads_each_readme_once() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        write_readme(temp_dir.path(), "---\nfeature: true\n---\n\n# Feature\n");

        let readmes = ReadmeCache::default();
        assert_eq!(readmes.feature_flag(temp_dir.path()), Some(true));

        // A later change on disk isn't seen, because the file isn't read again
        write_readme(temp_dir.path(), "---\nfeature: false\n---\n\n# Feature\n");
        assert_eq!(readmes.feature_flag(temp_dir.path()), Some(true));
    }

    #[test]
    fn test_is_feature_directory_direct_subfolder_of_features() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let features_dir = temp_dir.path().join("features");
        let feature_dir = features_dir.join("my-feature");
        fs::create_dir_all(&feature_dir).unwrap();

        // No README needed: a direct subfolder of `features` is a feature by default
        assert!(is_feature_directory(&feature_dir, &ReadmeCache::default()));
    }

    #[test]
    fn test_is_feature_directory_skips_when_feature_false_even_under_features_folder() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let features_dir = temp_dir.path().join("features");
        let feature_dir = features_dir.join("my-feature");
        fs::create_dir_all(&feature_dir).unwrap();
        write_readme(&feature_dir, "---\nfeature: false\n---\n\n# My Feature\n");

        // Explicit `feature: false` overrides the "direct subfolder of `features`" rule
        assert!(!is_feature_directory(&feature_dir, &ReadmeCache::default()));
    }

    #[test]
    fn test_is_feature_directory_readme_feature_true_outside_features_folder() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let feature_dir = temp_dir.path().join("some-dir");
        fs::create_dir_all(&feature_dir).unwrap();
        write_readme(&feature_dir, "---\nfeature: true\n---\n\n# Some Dir\n");

        assert!(is_feature_directory(&feature_dir, &ReadmeCache::default()));
    }

    #[test]
    fn test_is_feature_directory_readme_feature_false_outside_features_folder() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let feature_dir = temp_dir.path().join("some-dir");
        fs::create_dir_all(&feature_dir).unwrap();
        write_readme(&feature_dir, "---\nfeature: false\n---\n\n# Some Dir\n");

        assert!(!is_feature_directory(&feature_dir, &ReadmeCache::default()));
    }
}
