use anyhow::{Context, Result};
use git2::Repository;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::dependency_resolver::{
    build_file_to_feature_map, collect_feature_info, resolve_feature_dependencies,
};
use crate::feature_metadata_detector::{self, FeatureMetadataMap};
use crate::features_toml_parser::{find_features_toml, read_features_toml};
use crate::git_helper::{CommitHistory, get_all_commits_by_path};
use crate::import_detector::{ImportStatement, build_file_map, scan_file_for_imports};
use crate::models::{Change, Feature, Stats};
use crate::readme_parser::{ReadmeInfo, parse_readme_info, read_readme_info};
use crate::skip_dirs::SKIPPED_DIRS;

fn is_documentation_directory(dir_path: &Path) -> bool {
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
struct WalkFilter<'a> {
    ignore_paths: &'a [PathBuf],
    /// Scan the folders in `SKIPPED_DIRS` instead of skipping them
    include_skipped_dirs: bool,
}

impl WalkFilter<'_> {
    fn skips(&self, dir_path: &Path) -> bool {
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
struct ReadmeCache(RefCell<HashMap<PathBuf, Option<Rc<Readme>>>>);

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
fn is_feature_directory(dir_path: &Path, readmes: &ReadmeCache) -> bool {
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

/// Inputs shared by every level of the recursive feature scan
#[derive(Clone, Copy)]
struct ScanContext<'a> {
    base_path: &'a Path,
    history: Option<&'a CommitHistory>,
    feature_metadata_map: &'a FeatureMetadataMap,
    filter: WalkFilter<'a>,
    readmes: &'a ReadmeCache,
    /// Skip everything not needed to resolve feature ownership
    ownership_only: bool,
}

pub fn list_files_recursive(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
) -> Result<Vec<Feature>> {
    // Scan entire base_path for feature metadata once
    let feature_metadata =
        feature_metadata_detector::scan_directory_for_feature_metadata(dir).unwrap_or_default();

    // First pass: build feature structure without dependencies
    let filter = WalkFilter {
        ignore_paths,
        include_skipped_dirs,
    };
    let readmes = ReadmeCache::default();
    let ctx = ScanContext {
        base_path: dir,
        history: None,
        feature_metadata_map: &feature_metadata,
        filter,
        readmes: &readmes,
        ownership_only: false,
    };
    let mut features = list_files_recursive_impl(dir, None, &ctx)?;

    // Second pass: scan for imports and resolve dependencies
    populate_dependencies(&mut features, dir, &ctx)?;

    Ok(features)
}

/// Build the feature tree with only what is needed to know who owns what:
/// name, owner, path, meta from README/FEATURES.toml and nested features.
///
/// Skips everything that reads source files or git history (changes, stats,
/// decisions, feature metadata comments and dependencies), which makes it
/// suitable for CODEOWNERS generation on large repositories.
pub fn list_features_ownership_only(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
) -> Result<Vec<Feature>> {
    let feature_metadata = FeatureMetadataMap::new();

    let filter = WalkFilter {
        ignore_paths,
        include_skipped_dirs,
    };
    let readmes = ReadmeCache::default();
    let ctx = ScanContext {
        base_path: dir,
        history: None,
        feature_metadata_map: &feature_metadata,
        filter,
        readmes: &readmes,
        ownership_only: true,
    };
    list_files_recursive_impl(dir, None, &ctx)
}

pub fn list_files_recursive_with_changes(
    dir: &Path,
    ignore_paths: &[PathBuf],
    include_skipped_dirs: bool,
) -> Result<Vec<Feature>> {
    // Get all commits once at the beginning for efficiency
    let history = get_all_commits_by_path(dir).unwrap_or_default();
    // Scan entire base_path for feature metadata once
    let feature_metadata =
        feature_metadata_detector::scan_directory_for_feature_metadata(dir).unwrap_or_default();

    // First pass: build feature structure without dependencies
    let filter = WalkFilter {
        ignore_paths,
        include_skipped_dirs,
    };
    let readmes = ReadmeCache::default();
    let ctx = ScanContext {
        base_path: dir,
        history: Some(&history),
        feature_metadata_map: &feature_metadata,
        filter,
        readmes: &readmes,
        ownership_only: false,
    };
    let mut features = list_files_recursive_impl(dir, None, &ctx)?;

    // Second pass: scan for imports and resolve dependencies
    populate_dependencies(&mut features, dir, &ctx)?;

    Ok(features)
}

/// Populate dependencies for all features by scanning imports
fn populate_dependencies(
    features: &mut [Feature],
    base_path: &Path,
    ctx: &ScanContext,
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
        let imports = scan_feature_directory_for_imports(&feature_path, ctx);
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
    ctx: &ScanContext,
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
                if dir_name == "features" || ctx.filter.skips(&path) {
                    continue;
                }

                // Skip nested feature directories (check if it's a direct child of "features" directory with README)
                if is_feature_directory(&path, ctx.readmes) {
                    continue;
                }

                // Recursively scan subdirectories
                let nested_imports = scan_feature_directory_for_imports(&path, ctx);
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
    file_to_feature_map: &HashMap<std::path::PathBuf, String>,
    feature_path_to_name_map: &HashMap<String, String>,
    file_map: &HashMap<String, std::path::PathBuf>,
) {
    for feature in features {
        // Get imports for this feature using path as key (not name, since multiple features can have same name)
        if let Some(imports) = feature_imports.get(&feature.path) {
            let feature_path = std::path::PathBuf::from(&feature.path);

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

fn read_decision_files(feature_path: &Path) -> Result<Vec<String>> {
    let mut decisions = Vec::new();

    // Check both "decision" and "decisions" folder names
    let decision_paths = [
        feature_path.join(".docs").join("decisions"),
        feature_path.join("__docs__").join("decisions"),
    ];

    for decisions_dir in &decision_paths {
        if decisions_dir.exists() && decisions_dir.is_dir() {
            let entries = fs::read_dir(decisions_dir).with_context(|| {
                format!(
                    "could not read decisions directory `{}`",
                    decisions_dir.display()
                )
            })?;

            // Collect all decision file paths first
            let mut decision_paths_vec = Vec::new();

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                // Skip README.md files and only process .md files
                if path.is_file()
                    && let Some(file_name) = path.file_name()
                {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.ends_with(".md") && file_name_str != "README.md" {
                        decision_paths_vec.push(path);
                    }
                }
            }

            // Sort decision files alphabetically by filename
            decision_paths_vec.sort_by(|a, b| {
                let name_a = a.file_name().unwrap_or_default().to_string_lossy();
                let name_b = b.file_name().unwrap_or_default().to_string_lossy();
                name_a.cmp(&name_b)
            });

            // Read the sorted files
            for path in decision_paths_vec {
                let content = fs::read_to_string(&path).with_context(|| {
                    format!("could not read decision file `{}`", path.display())
                })?;
                decisions.push(content);
            }

            break; // If we found one of the directories, don't check the other
        }
    }

    Ok(decisions)
}

/// File, line and TODO totals for a feature directory
#[derive(Default)]
struct FileCounts {
    files: usize,
    lines: usize,
    todos: usize,
}

/// Case-insensitive search for "TODO", without allocating
fn contains_todo(line: &str) -> bool {
    line.as_bytes()
        .windows(4)
        .any(|window| window.eq_ignore_ascii_case(b"TODO"))
}

/// Count files, lines and TODO comments in a feature directory (excluding
/// documentation), reading each file once
fn count_feature_files(
    dir: &Path,
    nested_feature_paths: &[String],
    filter: &WalkFilter,
    counts: &mut FileCounts,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Skip documentation directories
        if is_documentation_directory(&path) {
            continue;
        }

        // Skip nested feature directories
        let path_str = path.to_string_lossy();
        if nested_feature_paths
            .iter()
            .any(|nfp| path_str.starts_with(nfp.as_str()))
        {
            continue;
        }

        if path.is_file() {
            counts.files += 1;

            // Files that aren't text only count towards the number of files
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    counts.lines += 1;
                    if contains_todo(line) {
                        counts.todos += 1;
                    }
                }
            }
        } else if path.is_dir() && !filter.skips(&path) {
            count_feature_files(&path, nested_feature_paths, filter, counts);
        }
    }
}

/// A feature's changes and where it sits in its repository
struct FeatureHistory {
    changes: Vec<Change>,
    repo_workdir: PathBuf,
    relative_path: String,
}

/// Compute commit statistics for a feature
fn compute_commit_stats(
    feature_history: &FeatureHistory,
    history: &CommitHistory,
    nested_features: &[Feature],
) -> BTreeMap<String, serde_json::Value> {
    // Convert nested feature paths to repo-relative paths once for all commits
    let nested_relative_paths: Vec<String> = nested_features
        .iter()
        .filter_map(|feature| {
            let canonical = fs::canonicalize(&feature.path).ok()?;
            let relative = canonical.strip_prefix(&feature_history.repo_workdir).ok()?;
            Some(relative.to_string_lossy().to_string())
        })
        .collect();

    // Filter changes to only include those that affect files in this feature
    // (not exclusively in nested features)
    let filtered_changes: Vec<&Change> = feature_history
        .changes
        .iter()
        .filter(|change| {
            let Some(affected_files) = history.affected_paths.get(&change.hash) else {
                return false;
            };

            // Check if any affected file is in this feature but not in a nested feature
            affected_files.iter().any(|file_path| {
                file_path.starts_with(&feature_history.relative_path)
                    && !nested_relative_paths
                        .iter()
                        .any(|nested_path| file_path.starts_with(nested_path.as_str()))
            })
        })
        .collect();

    let mut commits = BTreeMap::new();

    // Add total commit count
    commits.insert(
        "total_commits".to_string(),
        serde_json::json!(filtered_changes.len()),
    );

    // Count commits by author
    let mut authors_count: HashMap<String, usize> = HashMap::new();
    for change in &filtered_changes {
        *authors_count.entry(change.author_name.clone()).or_insert(0) += 1;
    }
    commits.insert(
        "authors_count".to_string(),
        serde_json::json!(authors_count),
    );

    // Count commits by conventional commit type
    let mut count_by_type: HashMap<String, usize> = HashMap::new();
    for change in &filtered_changes {
        let commit_type = extract_commit_type(&change.title);
        *count_by_type.entry(commit_type).or_insert(0) += 1;
    }
    commits.insert(
        "count_by_type".to_string(),
        serde_json::json!(count_by_type),
    );

    // Get first and last commit dates
    if let Some(first) = filtered_changes.first() {
        commits.insert(
            "first_commit_date".to_string(),
            serde_json::json!(first.date.clone()),
        );
    }
    if let Some(last) = filtered_changes.last() {
        commits.insert(
            "last_commit_date".to_string(),
            serde_json::json!(last.date.clone()),
        );
    }

    commits
}

/// Extract the commit type from a conventional commit title
fn extract_commit_type(title: &str) -> String {
    // Common conventional commit types
    let known_types = [
        "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
        "revert",
    ];

    // Check if the title follows conventional commit format (type: description or type(scope): description)
    if let Some(colon_pos) = title.find(':') {
        let prefix = &title[..colon_pos];

        // Remove scope if present (e.g., "feat(auth)" -> "feat")
        let type_part = if let Some(paren_pos) = prefix.find('(') {
            &prefix[..paren_pos]
        } else {
            prefix
        };

        let type_part = type_part.trim().to_lowercase();

        // Check if it's a known conventional commit type
        if known_types.contains(&type_part.as_str()) {
            return type_part;
        }
    }

    // If not a conventional commit, return "other"
    "other".to_string()
}

fn process_feature_directory(
    path: &Path,
    name: &str,
    parent_owner: Option<&str>,
    ctx: &ScanContext,
) -> Result<Feature> {
    let ScanContext {
        base_path,
        history,
        feature_metadata_map,
        filter,
        readmes,
        ownership_only,
    } = *ctx;

    // First try to find and read FEATURES.toml file
    let (title, owner, description, mut meta) = if let Some(toml_path) = find_features_toml(path) {
        if let Ok(toml_data) = read_features_toml(&toml_path) {
            (
                toml_data.name,
                toml_data.owner.unwrap_or_default(),
                toml_data.description.unwrap_or_default(),
                toml_data.meta,
            )
        } else {
            (
                None,
                String::new(),
                String::new(),
                std::collections::BTreeMap::new(),
            )
        }
    } else {
        // Fall back to README file if FEATURES.toml not found
        let readme_info = match readmes.get(path) {
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

    // Get the relative path to this feature directory for metadata lookup
    let relative_path = path
        .strip_prefix(base_path)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    // Check if this feature has any metadata from the global scan (matched by feature path)
    if let Some(metadata_map) = feature_metadata_map.get(&relative_path) {
        // Iterate through each metadata key (e.g., "feature-flag", "feature-experiment")
        for (metadata_key, flags) in metadata_map {
            // Convert Vec<HashMap<String, String>> to JSON array
            let flags_json: Vec<serde_json::Value> = flags
                .iter()
                .map(|flag_map| {
                    let json_map: serde_json::Map<String, serde_json::Value> = flag_map
                        .iter()
                        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                        .collect();
                    serde_json::Value::Object(json_map)
                })
                .collect();

            // Check if this metadata key already exists, append if it does
            meta.entry(metadata_key.clone())
                .and_modify(|existing| {
                    if let serde_json::Value::Array(arr) = existing {
                        arr.extend(flags_json.clone());
                    }
                })
                .or_insert_with(|| serde_json::Value::Array(flags_json));
        }
    }

    // Convert the absolute path to a repo-relative path and look up changes
    let feature_history = history.and_then(|h| get_feature_history(path, h).ok());

    // Always include decisions regardless of include_changes flag
    let decisions = if ownership_only {
        Vec::new()
    } else {
        read_decision_files(path).unwrap_or_default()
    };

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
        && !filter.skips(&nested_features_path)
    {
        list_files_recursive_impl(&nested_features_path, Some(&actual_owner), ctx)
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
            && !filter.skips(&entry_path)
        {
            if readmes.feature_flag(&entry_path) == Some(true) {
                // This directory is a feature itself
                let nested_feature =
                    process_feature_directory(&entry_path, &entry_name, Some(&actual_owner), ctx)?;
                nested_features.push(nested_feature);
            } else {
                // This directory is not a feature, but might contain features
                // Recursively search for features inside it
                let deeper_features =
                    list_files_recursive_impl(&entry_path, Some(&actual_owner), ctx)?;
                nested_features.extend(deeper_features);
            }
        }
    }

    let stats = if ownership_only {
        None
    } else {
        // Collect paths of nested features to exclude from file/line counts
        let nested_feature_paths: Vec<String> =
            nested_features.iter().map(|f| f.path.clone()).collect();

        let mut counts = FileCounts::default();
        count_feature_files(path, &nested_feature_paths, &filter, &mut counts);

        // Commit statistics are only available if the feature has changes
        let commits = match (&feature_history, history) {
            (Some(feature_history), Some(history)) if !feature_history.changes.is_empty() => {
                compute_commit_stats(feature_history, history, &nested_features)
            }
            _ => BTreeMap::new(),
        };

        Some(Stats {
            files_count: Some(counts.files),
            lines_count: Some(counts.lines),
            todos_count: Some(counts.todos),
            commits,
            coverage: None,
        })
    };

    // Make path relative to base_path
    let relative_path = path
        .strip_prefix(base_path)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    Ok(Feature {
        name: title.unwrap_or_else(|| name.to_string()),
        description,
        owner: actual_owner,
        is_owner_inherited,
        path: relative_path,
        features: nested_features,
        meta,
        changes: feature_history.map(|fh| fh.changes).unwrap_or_default(),
        decisions,
        stats,
        dependencies: Vec::new(), // Will be populated in second pass
    })
}

fn list_files_recursive_impl(
    dir: &Path,
    parent_owner: Option<&str>,
    ctx: &ScanContext,
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
            if ctx.filter.skips(&path) {
                continue;
            }

            if is_feature_directory(&path, ctx.readmes) {
                let feature = process_feature_directory(&path, &name, parent_owner, ctx)?;
                features.push(feature);
            } else if !is_documentation_directory(&path)
                && !is_inside_documentation_directory(&path)
            {
                // Recursively search for features in non-documentation subdirectories
                let new_features = list_files_recursive_impl(&path, parent_owner, ctx)?;
                features.extend(new_features);
            }
        }
    }

    Ok(features)
}

/// Get a feature's changes from the pre-computed commit history
fn get_feature_history(path: &Path, history: &CommitHistory) -> Result<FeatureHistory> {
    // Canonicalize the path
    let canonical_path = std::fs::canonicalize(path)?;

    // Find the repository and get the working directory
    let repo = Repository::discover(path)?;
    let repo_workdir = repo
        .workdir()
        .context("repository has no working directory")?;

    // Convert to relative path from repo root
    let relative_path = canonical_path
        .strip_prefix(repo_workdir)
        .context("path is not within repository")?
        .to_string_lossy()
        .to_string();

    // Look up the changes in the map
    let changes = history
        .changes_by_path
        .get(&relative_path)
        .cloned()
        .unwrap_or_default();

    Ok(FeatureHistory {
        changes,
        repo_workdir: repo_workdir.to_path_buf(),
        relative_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_commit_type() {
        // Test standard conventional commit types
        assert_eq!(extract_commit_type("feat: add new feature"), "feat");
        assert_eq!(extract_commit_type("fix: resolve bug"), "fix");
        assert_eq!(extract_commit_type("docs: update README"), "docs");
        assert_eq!(extract_commit_type("style: format code"), "style");
        assert_eq!(
            extract_commit_type("refactor: improve structure"),
            "refactor"
        );
        assert_eq!(extract_commit_type("perf: optimize performance"), "perf");
        assert_eq!(extract_commit_type("test: add unit tests"), "test");
        assert_eq!(extract_commit_type("build: update dependencies"), "build");
        assert_eq!(extract_commit_type("ci: fix CI pipeline"), "ci");
        assert_eq!(extract_commit_type("chore: update gitignore"), "chore");
        assert_eq!(
            extract_commit_type("revert: undo previous commit"),
            "revert"
        );

        // Test with scope
        assert_eq!(extract_commit_type("feat(auth): add login"), "feat");
        assert_eq!(
            extract_commit_type("fix(api): resolve endpoint issue"),
            "fix"
        );
        assert_eq!(
            extract_commit_type("docs(readme): update instructions"),
            "docs"
        );

        // Test case insensitivity
        assert_eq!(extract_commit_type("FEAT: uppercase type"), "feat");
        assert_eq!(extract_commit_type("Fix: mixed case"), "fix");
        assert_eq!(extract_commit_type("DOCS: all caps"), "docs");

        // Test non-conventional commits
        assert_eq!(extract_commit_type("random commit message"), "other");
        assert_eq!(extract_commit_type("update: not conventional"), "other");
        assert_eq!(
            extract_commit_type("feature: close but not standard"),
            "other"
        );
        assert_eq!(extract_commit_type("no colon here"), "other");
        assert_eq!(extract_commit_type(""), "other");

        // Test edge cases
        assert_eq!(extract_commit_type("feat:no space after colon"), "feat");
        assert_eq!(extract_commit_type("feat  : extra spaces"), "feat");
        assert_eq!(
            extract_commit_type("feat(scope)(weird): nested parens"),
            "feat"
        );
    }

    fn write_readme(dir: &Path, content: &str) {
        fs::write(dir.join("README.md"), content).unwrap();
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
    fn test_contains_todo() {
        assert!(contains_todo("// TODO: fix"));
        assert!(contains_todo("// todo: fix"));
        assert!(contains_todo("// ToDo"));
        assert!(contains_todo("todo"));
        assert!(contains_todo("écrit // tOdO é"));
        assert!(!contains_todo(""));
        assert!(!contains_todo("tod"));
        assert!(!contains_todo("to do"));
    }

    #[test]
    fn test_count_feature_files() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let base = temp_dir.path();
        fs::create_dir_all(base.join("sub")).unwrap();
        fs::create_dir_all(base.join("docs")).unwrap();
        fs::create_dir_all(base.join("node_modules")).unwrap();
        fs::write(base.join("a.ts"), "// todo one\nline\n// TODO two\n").unwrap();
        fs::write(base.join("sub/b.ts"), "todo\n").unwrap();
        fs::write(base.join("binary"), [0xff, 0xfe]).unwrap();
        fs::write(base.join("docs/doc.md"), "todo\n").unwrap();
        fs::write(base.join("node_modules/c.js"), "todo\n").unwrap();

        let filter = WalkFilter {
            ignore_paths: &[],
            include_skipped_dirs: false,
        };
        let mut counts = FileCounts::default();
        count_feature_files(base, &[], &filter, &mut counts);
        assert_eq!((counts.files, counts.lines, counts.todos), (3, 4, 3));

        let filter = WalkFilter {
            ignore_paths: &[],
            include_skipped_dirs: true,
        };
        let mut counts = FileCounts::default();
        count_feature_files(base, &[], &filter, &mut counts);
        assert_eq!((counts.files, counts.lines, counts.todos), (4, 5, 4));
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

    /// Skipping a feature via `feature: false` must keep its sub-children discoverable:
    /// both a nested `features/` folder and a subfolder with its own `feature: true` README.
    #[test]
    fn test_list_files_recursive_skips_feature_but_keeps_sub_children() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let base = temp_dir.path();

        let features_dir = base.join("features");
        let skipped_dir = features_dir.join("skipped-feature");
        fs::create_dir_all(&skipped_dir).unwrap();
        write_readme(
            &skipped_dir,
            "---\nfeature: false\nowner: skipped-owner\n---\n\n# Skipped Feature\n",
        );

        // A nested `features` folder inside the skipped directory.
        let nested_via_features_folder = skipped_dir.join("features").join("child-a");
        fs::create_dir_all(&nested_via_features_folder).unwrap();

        // A subfolder marked as a feature via its own README.
        let nested_via_readme = skipped_dir.join("child-b");
        fs::create_dir_all(&nested_via_readme).unwrap();
        write_readme(
            &nested_via_readme,
            "---\nfeature: true\nowner: child-b-owner\n---\n\n# Child B\n",
        );

        let features = list_files_recursive(base, &[], false).unwrap();

        // The skipped feature itself must not appear anywhere in the tree.
        fn collect_names(features: &[Feature], out: &mut Vec<String>) {
            for feature in features {
                out.push(feature.name.clone());
                collect_names(&feature.features, out);
            }
        }
        let mut names = Vec::new();
        collect_names(&features, &mut names);
        assert!(!names.contains(&"skipped-feature".to_string()));
        assert!(!names.contains(&"Skipped Feature".to_string()));

        // Both sub-children are still discovered, as top-level features (since their
        // skipped parent no longer exists as a node in the tree).
        assert!(names.contains(&"child-a".to_string()));
        let child_b = features
            .iter()
            .find(|f| f.name == "Child B")
            .expect("child-b feature should be present");
        assert_eq!(child_b.owner, "child-b-owner");
    }
}
