//! Aggregation of feature reports coming from several repositories.
//!
//! The input is a folder that contains one features JSON report per repository
//! (the output of `features <path> --json`, or the `features.json` of a build).
//! Each report is either a `<repository>.json` file or a `<repository>/features.json`
//! file. The repository name is taken from the file stem or the folder name.
//!
//! # Reconciliation algorithm
//!
//! * Two features with the same name are the same feature and are merged into one.
//! * A merged feature takes the parent it has in the repositories where it is nested:
//!   if it is a root feature in repository A and a child of `X` in repository B, the
//!   merged feature becomes a child of `X`.
//! * The children of a merged feature are the children it has in every repository.
//! * Features that only exist in one repository keep their place in the tree, so
//!   unmatched root features stay at the root of the aggregated tree.
//!
//! A merge is refused, and reported in the `errors` section, when:
//!
//! * the merged parents would create a loop (repository A has `B > C`, repository
//!   D has `C > B`): every feature of the loop is kept separate per repository;
//! * the feature has different parents in different repositories: the feature is
//!   kept separate per repository, each copy under its own parent.
//!
//! Features that are not merged get a path suffixed with `@<repository>` so they stay
//! unique in the aggregated output.

use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use crate::coverage_parser::CoverageStats;
use crate::models::{Change, Dependency, Record, Stats};

/// A feature as found in a features JSON report, or as produced by the aggregation.
///
/// It has the same shape as [`crate::models::Feature`], plus `repositories`, which
/// lists every repository (and the path inside it) where the feature was found.
/// All fields are optional when reading, so partial reports can still be aggregated.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AggregatedFeature {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub is_owner_inherited: bool,
    pub path: String,
    pub features: Vec<AggregatedFeature>,
    pub meta: BTreeMap<String, serde_json::Value>,
    pub changes: Vec<Change>,
    pub decisions: Vec<String>,
    pub records: Vec<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    pub dependencies: Vec<Dependency>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<FeatureRepository>,
}

/// Where a feature lives in one of the aggregated repositories.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FeatureRepository {
    /// Name of the repository (file stem or folder name of its report)
    pub name: String,
    /// Path of the feature inside that repository
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationErrorType {
    /// Merging the features would create a loop in the hierarchy
    Cycle,
    /// The feature has different parents in different repositories
    ConflictingParents,
}

/// One occurrence of a feature that could not be merged.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReconciliationOccurrence {
    pub feature: String,
    pub repository: String,
    /// Path of the feature inside its repository
    pub path: String,
    /// Name of the parent feature in that repository, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Path of the (unmerged) feature in the aggregated output
    #[serde(rename = "featurePath")]
    pub feature_path: String,
}

/// A reconciliation issue: features that share a name but could not be merged.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReconciliationError {
    #[serde(rename = "type")]
    pub error_type: ReconciliationErrorType,
    pub message: String,
    /// Names of the features that were not merged
    pub features: Vec<String>,
    pub occurrences: Vec<ReconciliationOccurrence>,
}

/// Output of the aggregation, serialized as the `features.json` of an aggregated build.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregatedReport {
    /// Names of the aggregated repositories
    pub repositories: Vec<String>,
    pub features: Vec<AggregatedFeature>,
    pub errors: Vec<ReconciliationError>,
}

/// The features report of one repository.
#[derive(Debug, Clone)]
pub struct RepositoryReport {
    pub name: String,
    pub features: Vec<AggregatedFeature>,
}

/// Accepts both a plain features array and an object with a `features` field
/// (an aggregated report, or any wrapper).
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ReportFile {
    List(Vec<AggregatedFeature>),
    Object { features: Vec<AggregatedFeature> },
}

/// Read every features report found in `folder`.
///
/// `<folder>/<name>.json` and `<folder>/<name>/features.json` are both read; the
/// repository is called `<name>`. Reports are returned sorted by repository name.
pub fn load_reports(folder: &Path) -> Result<Vec<RepositoryReport>> {
    let entries = std::fs::read_dir(folder)
        .with_context(|| format!("could not read directory `{}`", folder.display()))?;

    let mut candidates: Vec<(String, std::path::PathBuf)> = Vec::new();
    for entry in entries {
        let path = entry?.path();
        let Some(stem) = path.file_stem().map(|s| s.to_string_lossy().to_string()) else {
            continue;
        };

        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            candidates.push((stem, path));
        } else if path.is_dir() {
            let nested = path.join("features.json");
            if nested.is_file() {
                let name = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or(stem);
                candidates.push((name, nested));
            }
        }
    }

    candidates.sort();

    let mut reports = Vec::new();
    let mut seen = BTreeSet::new();
    for (name, path) in candidates {
        if !seen.insert(name.clone()) {
            anyhow::bail!(
                "two reports are named `{}` in `{}`: repository names must be unique",
                name,
                folder.display()
            );
        }

        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("could not read `{}`", path.display()))?;
        let report: ReportFile = serde_json::from_str(&content)
            .with_context(|| format!("`{}` is not a valid features JSON report", path.display()))?;
        let features = match report {
            ReportFile::List(features) | ReportFile::Object { features } => features,
        };
        reports.push(RepositoryReport { name, features });
    }

    Ok(reports)
}

/// Load the reports of `folder` and aggregate them.
pub fn aggregate_folder(folder: &Path) -> Result<AggregatedReport> {
    let reports = load_reports(folder)?;
    if reports.is_empty() {
        anyhow::bail!(
            "no features JSON report found in `{}` (expected `<repository>.json` or `<repository>/features.json` files)",
            folder.display()
        );
    }
    Ok(aggregate(&reports))
}

/// One feature of one repository, with its children removed.
struct Occurrence {
    repo: usize,
    parent: Option<usize>,
    feature: AggregatedFeature,
}

/// Identity of a feature in the aggregated tree.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NodeKey {
    /// All the occurrences of a name, merged
    Merged(String),
    /// A single occurrence, kept apart because it could not be merged
    Single(usize),
}

fn flatten_occurrences(
    repo: usize,
    features: &[AggregatedFeature],
    parent: Option<usize>,
    occurrences: &mut Vec<Occurrence>,
) {
    for feature in features {
        let index = occurrences.len();
        let mut own = feature.clone();
        own.features = Vec::new();
        own.repositories = Vec::new();
        occurrences.push(Occurrence {
            repo,
            parent,
            feature: own,
        });
        flatten_occurrences(repo, &feature.features, Some(index), occurrences);
    }
}

/// Aggregate the features of several repositories (see the module documentation).
pub fn aggregate(reports: &[RepositoryReport]) -> AggregatedReport {
    let repo_names: Vec<String> = reports.iter().map(|r| r.name.clone()).collect();

    let mut occurrences = Vec::new();
    for (repo, report) in reports.iter().enumerate() {
        flatten_occurrences(repo, &report.features, None, &mut occurrences);
    }

    let mut split: BTreeSet<String> = BTreeSet::new();
    let mut pending_errors: Vec<(ReconciliationErrorType, Vec<String>)> = Vec::new();

    let key_of = |index: usize, split: &BTreeSet<String>| -> NodeKey {
        let name = &occurrences[index].feature.name;
        if split.contains(name) {
            NodeKey::Single(index)
        } else {
            NodeKey::Merged(name.clone())
        }
    };

    // Each round refuses at least one merge, so this terminates after at most
    // one round per feature name.
    let (groups, parents) = loop {
        let mut groups: BTreeMap<NodeKey, Vec<usize>> = BTreeMap::new();
        for index in 0..occurrences.len() {
            groups.entry(key_of(index, &split)).or_default().push(index);
        }

        // 1. A feature must have a single parent across repositories
        let mut parents: BTreeMap<NodeKey, Option<NodeKey>> = BTreeMap::new();
        let mut conflicts = Vec::new();
        for (key, members) in &groups {
            let candidates: BTreeSet<NodeKey> = members
                .iter()
                .filter_map(|&m| occurrences[m].parent)
                .map(|p| key_of(p, &split))
                .collect();

            if candidates.len() > 1 {
                if let NodeKey::Merged(name) = key {
                    conflicts.push(name.clone());
                }
            } else {
                parents.insert(key.clone(), candidates.into_iter().next());
            }
        }

        if !conflicts.is_empty() {
            for name in conflicts {
                split.insert(name.clone());
                pending_errors.push((ReconciliationErrorType::ConflictingParents, vec![name]));
            }
            continue;
        }

        // 2. The merged hierarchy must not contain loops
        let cycles = find_cycles(&parents);
        if cycles.is_empty() {
            break (groups, parents);
        }

        for cycle in cycles {
            let names: Vec<String> = cycle
                .iter()
                .filter_map(|key| match key {
                    NodeKey::Merged(name) => Some(name.clone()),
                    NodeKey::Single(_) => None,
                })
                .collect();
            for name in &names {
                split.insert(name.clone());
            }
            pending_errors.push((ReconciliationErrorType::Cycle, names));
        }
    };

    // Build the tree
    let mut children: BTreeMap<NodeKey, Vec<NodeKey>> = BTreeMap::new();
    let mut roots: Vec<NodeKey> = Vec::new();
    for (key, parent) in &parents {
        match parent {
            Some(parent) => children
                .entry(parent.clone())
                .or_default()
                .push(key.clone()),
            None => roots.push(key.clone()),
        }
    }

    let display_name = |key: &NodeKey| -> String {
        match key {
            NodeKey::Merged(name) => name.clone(),
            NodeKey::Single(index) => occurrences[*index].feature.name.clone(),
        }
    };
    let segment_of = |key: &NodeKey| -> String {
        match key {
            NodeKey::Merged(name) => slugify(name),
            NodeKey::Single(index) => {
                let occurrence = &occurrences[*index];
                format!(
                    "{}@{}",
                    slugify(&occurrence.feature.name),
                    slugify(&repo_names[occurrence.repo])
                )
            }
        }
    };
    let sort_nodes = |nodes: &mut Vec<NodeKey>| {
        nodes.sort_by_cached_key(|key| (display_name(key).to_lowercase(), segment_of(key)));
    };

    sort_nodes(&mut roots);
    for list in children.values_mut() {
        sort_nodes(list);
    }

    // Assign unique aggregated paths, parents first
    let mut node_paths: BTreeMap<NodeKey, String> = BTreeMap::new();
    let mut stack: Vec<(Option<String>, Vec<NodeKey>)> = vec![(None, roots.clone())];
    while let Some((parent_path, siblings)) = stack.pop() {
        let mut used: HashMap<String, usize> = HashMap::new();
        for key in siblings {
            let segment = segment_of(&key);
            let count = used.entry(segment.clone()).or_insert(0);
            *count += 1;
            let segment = if *count > 1 {
                format!("{}~{}", segment, count)
            } else {
                segment
            };
            let path = match &parent_path {
                Some(parent) => format!("{}/{}", parent, segment),
                None => segment,
            };
            if let Some(list) = children.get(&key) {
                stack.push((Some(path.clone()), list.clone()));
            }
            node_paths.insert(key, path);
        }
    }

    // (repository, path in repository) -> aggregated path, used to rewrite dependencies
    let mut path_lookup: HashMap<(usize, String), String> = HashMap::new();
    for (key, members) in &groups {
        let aggregated_path = &node_paths[key];
        for &m in members {
            let occurrence = &occurrences[m];
            path_lookup.insert(
                (occurrence.repo, occurrence.feature.path.clone()),
                aggregated_path.clone(),
            );
        }
    }

    fn build_node(
        key: &NodeKey,
        ctx: &BuildContext<'_>,
        children: &BTreeMap<NodeKey, Vec<NodeKey>>,
    ) -> AggregatedFeature {
        let mut feature = merge_occurrences(key, ctx);
        if let Some(list) = children.get(key) {
            feature.features = list.iter().map(|c| build_node(c, ctx, children)).collect();
        }
        feature
    }

    let ctx = BuildContext {
        occurrences: &occurrences,
        groups: &groups,
        node_paths: &node_paths,
        path_lookup: &path_lookup,
        repo_names: &repo_names,
    };
    let features = roots
        .iter()
        .map(|key| build_node(key, &ctx, &children))
        .collect();

    // Describe the refused merges
    let errors = pending_errors
        .into_iter()
        .map(|(error_type, names)| describe_error(error_type, names, &ctx, &split))
        .collect();

    AggregatedReport {
        repositories: repo_names,
        features,
        errors,
    }
}

struct BuildContext<'a> {
    occurrences: &'a [Occurrence],
    groups: &'a BTreeMap<NodeKey, Vec<usize>>,
    node_paths: &'a BTreeMap<NodeKey, String>,
    path_lookup: &'a HashMap<(usize, String), String>,
    repo_names: &'a [String],
}

/// Find the loops of a parent map (every node has at most one parent).
fn find_cycles(parents: &BTreeMap<NodeKey, Option<NodeKey>>) -> Vec<Vec<NodeKey>> {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        InProgress,
        Done,
    }

    let mut state: HashMap<&NodeKey, State> = HashMap::new();
    let mut cycles = Vec::new();

    for start in parents.keys() {
        if state.contains_key(start) {
            continue;
        }

        let mut walk: Vec<&NodeKey> = Vec::new();
        let mut current = Some(start);
        while let Some(key) = current {
            match state.get(key) {
                Some(State::Done) => break,
                Some(State::InProgress) => {
                    let begin = walk.iter().position(|k| *k == key).unwrap_or(0);
                    cycles.push(walk[begin..].iter().map(|k| (*k).clone()).collect());
                    break;
                }
                None => {
                    state.insert(key, State::InProgress);
                    walk.push(key);
                    current = parents.get(key).and_then(|p| p.as_ref());
                }
            }
        }

        for key in walk {
            state.insert(key, State::Done);
        }
    }

    cycles
}

fn describe_error(
    error_type: ReconciliationErrorType,
    names: Vec<String>,
    ctx: &BuildContext<'_>,
    split: &BTreeSet<String>,
) -> ReconciliationError {
    let mut occurrences = Vec::new();
    for (key, members) in ctx.groups {
        let NodeKey::Single(index) = key else {
            continue;
        };
        let occurrence = &ctx.occurrences[*index];
        if !names.contains(&occurrence.feature.name) {
            continue;
        }
        debug_assert_eq!(members.len(), 1);
        occurrences.push(ReconciliationOccurrence {
            feature: occurrence.feature.name.clone(),
            repository: ctx.repo_names[occurrence.repo].clone(),
            path: occurrence.feature.path.clone(),
            parent: occurrence
                .parent
                .map(|p| ctx.occurrences[p].feature.name.clone()),
            feature_path: ctx.node_paths[key].clone(),
        });
    }
    occurrences.sort_by(|a, b| (&a.feature, &a.repository).cmp(&(&b.feature, &b.repository)));

    let quoted = |names: &[String]| {
        names
            .iter()
            .map(|n| format!("'{}'", n))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let message = match error_type {
        ReconciliationErrorType::Cycle => {
            let chain: Vec<String> = occurrences
                .iter()
                .filter_map(|o| {
                    o.parent.as_ref().filter(|p| names.contains(p)).map(|p| {
                        format!(
                            "'{}' is the parent of '{}' in {}",
                            p, o.feature, o.repository
                        )
                    })
                })
                .collect();
            format!(
                "Features {} were not merged because merging them would create a loop: {}.",
                quoted(&names),
                chain.join("; ")
            )
        }
        ReconciliationErrorType::ConflictingParents => {
            let mut parents: Vec<String> = Vec::new();
            for description in occurrences.iter().map(|o| match &o.parent {
                Some(parent) if split.contains(parent) => {
                    format!("'{}' (unmerged) in {}", parent, o.repository)
                }
                Some(parent) => format!("'{}' in {}", parent, o.repository),
                None => format!("no parent in {}", o.repository),
            }) {
                if !parents.contains(&description) {
                    parents.push(description);
                }
            }
            format!(
                "Feature {} was not merged because it has different parents across repositories: {}.",
                quoted(&names),
                parents.join(", ")
            )
        }
    };

    ReconciliationError {
        error_type,
        message,
        features: names,
        occurrences,
    }
}

/// Build the aggregated feature of a node from all its occurrences (without children).
fn merge_occurrences(key: &NodeKey, ctx: &BuildContext<'_>) -> AggregatedFeature {
    let members: Vec<&Occurrence> = ctx.groups[key]
        .iter()
        .map(|&m| &ctx.occurrences[m])
        .collect();
    let first = &members[0].feature;

    let owner_source = members
        .iter()
        .find(|o| !o.feature.owner.is_empty() && !o.feature.is_owner_inherited)
        .or_else(|| members.iter().find(|o| !o.feature.owner.is_empty()))
        .map(|o| &o.feature);

    let description = members
        .iter()
        .map(|o| o.feature.description.as_str())
        .find(|d| !d.trim().is_empty())
        .unwrap_or_default()
        .to_string();

    let mut meta = BTreeMap::new();
    for occurrence in &members {
        for (k, v) in &occurrence.feature.meta {
            meta.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }

    let mut seen_hashes = BTreeSet::new();
    let mut changes: Vec<Change> = members
        .iter()
        .flat_map(|o| o.feature.changes.iter())
        .filter(|c| c.hash.is_empty() || seen_hashes.insert(c.hash.clone()))
        .cloned()
        .collect();
    changes.sort_by(|a, b| b.date.cmp(&a.date));

    let mut decisions: Vec<String> = Vec::new();
    for occurrence in &members {
        for decision in &occurrence.feature.decisions {
            if !decisions.contains(decision) {
                decisions.push(decision.clone());
            }
        }
    }

    let mut records: Vec<Record> = Vec::new();
    for occurrence in &members {
        for record in &occurrence.feature.records {
            if !records.contains(record) {
                records.push(record.clone());
            }
        }
    }

    let dependencies = members
        .iter()
        .flat_map(|o| {
            o.feature.dependencies.iter().map(|d| {
                let mut dependency = d.clone();
                if let Some(path) = ctx.path_lookup.get(&(o.repo, d.feature_path.clone())) {
                    dependency.feature_path = path.clone();
                }
                dependency
            })
        })
        .collect();

    let stats = merge_stats(&members, ctx.repo_names);

    let repositories = members
        .iter()
        .map(|o| FeatureRepository {
            name: ctx.repo_names[o.repo].clone(),
            path: o.feature.path.clone(),
        })
        .collect();

    AggregatedFeature {
        name: first.name.clone(),
        description,
        owner: owner_source.map(|f| f.owner.clone()).unwrap_or_default(),
        is_owner_inherited: owner_source.is_some_and(|f| f.is_owner_inherited),
        path: ctx.node_paths[key].clone(),
        features: Vec::new(),
        meta,
        changes,
        decisions,
        records,
        stats,
        dependencies,
        repositories,
    }
}

fn merge_stats(members: &[&Occurrence], repo_names: &[String]) -> Option<Stats> {
    let with_stats: Vec<(&Occurrence, &Stats)> = members
        .iter()
        .filter_map(|o| o.feature.stats.as_ref().map(|s| (*o, s)))
        .collect();
    if with_stats.is_empty() {
        return None;
    }

    let sum = |get: fn(&Stats) -> Option<usize>| -> Option<usize> {
        with_stats
            .iter()
            .filter_map(|(_, s)| get(s))
            .fold(None, |acc, v| Some(acc.unwrap_or(0) + v))
    };

    let mut coverage: Option<CoverageStats> = None;
    for (occurrence, stats) in &with_stats {
        let Some(other) = &stats.coverage else {
            continue;
        };
        // Prefix file paths with the repository so they stay unique
        let mut prefixed = other.clone();
        prefixed.files = other
            .files
            .iter()
            .map(|(file, s)| {
                (
                    format!("{}/{}", repo_names[occurrence.repo], file),
                    s.clone(),
                )
            })
            .collect();
        match coverage.as_mut() {
            Some(total) => total.merge(&prefixed),
            None => coverage = Some(prefixed),
        }
    }
    if let Some(total) = coverage.as_mut() {
        total.calculate_percentages();
    }

    Some(Stats {
        files_count: sum(|s| s.files_count),
        lines_count: sum(|s| s.lines_count),
        todos_count: sum(|s| s.todos_count),
        commits: merge_commits(with_stats.iter().map(|(_, s)| &s.commits).collect()),
        coverage,
    })
}

/// Merge commit statistics: counts are summed, dates keep the widest range.
fn merge_commits(
    all: Vec<&BTreeMap<String, serde_json::Value>>,
) -> BTreeMap<String, serde_json::Value> {
    use serde_json::Value;

    let mut merged: BTreeMap<String, Value> = BTreeMap::new();
    let mut firsts: Vec<String> = Vec::new();
    let mut lasts: Vec<String> = Vec::new();

    for commits in all {
        for (key, value) in commits {
            match key.as_str() {
                "first_commit_date" => firsts.extend(value.as_str().map(String::from)),
                "last_commit_date" => lasts.extend(value.as_str().map(String::from)),
                _ => match (merged.get_mut(key), value) {
                    (None, _) => {
                        merged.insert(key.clone(), value.clone());
                    }
                    (Some(Value::Number(a)), Value::Number(b)) => {
                        let total = a.as_u64().unwrap_or(0) + b.as_u64().unwrap_or(0);
                        *a = total.into();
                    }
                    (Some(Value::Object(a)), Value::Object(b)) => {
                        for (k, v) in b {
                            let current = a.get(k).and_then(Value::as_u64).unwrap_or(0);
                            a.insert(k.clone(), (current + v.as_u64().unwrap_or(0)).into());
                        }
                    }
                    // Unknown shape: keep the first value
                    (Some(_), _) => {}
                },
            }
        }
    }

    // Keep the convention of the inputs: the scanner lists the newest commit first,
    // so `first_commit_date` may be later than `last_commit_date`.
    let newest_first = firsts
        .first()
        .zip(lasts.first())
        .is_some_and(|(f, l)| f > l);
    let (first, last) = if newest_first {
        (firsts.iter().max(), lasts.iter().min())
    } else {
        (firsts.iter().min(), lasts.iter().max())
    };
    if let Some(first) = first {
        merged.insert("first_commit_date".to_string(), first.clone().into());
    }
    if let Some(last) = last {
        merged.insert("last_commit_date".to_string(), last.clone().into());
    }

    merged
}

/// Turn a name into a path segment that is safe in a URL query parameter.
fn slugify(name: &str) -> String {
    let mut slug = String::new();
    for c in name.chars() {
        if c.is_alphanumeric() || matches!(c, '_' | '.') {
            slug.push(c);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-').to_string();
    if slug.is_empty() {
        "feature".to_string()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feature(name: &str, path: &str, children: Vec<AggregatedFeature>) -> AggregatedFeature {
        AggregatedFeature {
            name: name.to_string(),
            path: path.to_string(),
            owner: String::new(),
            features: children,
            ..Default::default()
        }
    }

    fn repo(name: &str, features: Vec<AggregatedFeature>) -> RepositoryReport {
        RepositoryReport {
            name: name.to_string(),
            features,
        }
    }

    /// Render the tree as `name[repos](children)` to keep assertions short
    fn render(features: &[AggregatedFeature]) -> String {
        features
            .iter()
            .map(|f| {
                let repos: Vec<&str> = f.repositories.iter().map(|r| r.name.as_str()).collect();
                let mut out = format!("{}[{}]", f.name, repos.join(","));
                if !f.features.is_empty() {
                    out.push_str(&format!("({})", render(&f.features)));
                }
                out
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn merges_features_with_the_same_name_and_unions_children() {
        let report = aggregate(&[
            repo(
                "a",
                vec![feature(
                    "auth",
                    "src/auth",
                    vec![feature("login", "src/auth/login", vec![])],
                )],
            ),
            repo(
                "b",
                vec![feature(
                    "auth",
                    "lib/auth",
                    vec![feature("logout", "lib/auth/logout", vec![])],
                )],
            ),
        ]);

        assert_eq!(render(&report.features), "auth[a,b](login[a] logout[b])");
        assert!(report.errors.is_empty());
        assert_eq!(report.features[0].path, "auth");
        assert_eq!(report.features[0].features[0].path, "auth/login");
        assert_eq!(
            report.features[0].repositories,
            vec![
                FeatureRepository {
                    name: "a".into(),
                    path: "src/auth".into()
                },
                FeatureRepository {
                    name: "b".into(),
                    path: "lib/auth".into()
                },
            ]
        );
    }

    #[test]
    fn root_feature_takes_the_parent_it_has_in_another_repository() {
        let report = aggregate(&[
            repo(
                "a",
                vec![feature(
                    "checkout",
                    "checkout",
                    vec![feature("payment", "checkout/payment", vec![])],
                )],
            ),
            repo(
                "b",
                vec![feature(
                    "payment",
                    "payment",
                    vec![feature("stripe", "payment/stripe", vec![])],
                )],
            ),
        ]);

        assert_eq!(
            render(&report.features),
            "checkout[a](payment[a,b](stripe[b]))"
        );
        assert_eq!(
            report.features[0].features[0].features[0].path,
            "checkout/payment/stripe"
        );
        assert!(report.errors.is_empty());
    }

    #[test]
    fn unmatched_root_features_stay_at_the_root() {
        let report = aggregate(&[
            repo("a", vec![feature("billing", "billing", vec![])]),
            repo("b", vec![feature("search", "search", vec![])]),
        ]);

        assert_eq!(render(&report.features), "billing[a] search[b]");
    }

    #[test]
    fn refuses_merges_that_create_a_loop() {
        let report = aggregate(&[
            repo(
                "a",
                vec![feature("B", "b", vec![feature("C", "b/c", vec![])])],
            ),
            repo(
                "d",
                vec![feature("C", "c", vec![feature("B", "c/b", vec![])])],
            ),
        ]);

        assert_eq!(render(&report.features), "B[a](C[a]) C[d](B[d])");
        assert_eq!(report.features[0].path, "B@a");
        assert_eq!(report.features[0].features[0].path, "B@a/C@a");

        assert_eq!(report.errors.len(), 1);
        let error = &report.errors[0];
        assert_eq!(error.error_type, ReconciliationErrorType::Cycle);
        assert_eq!(error.features, vec!["B".to_string(), "C".to_string()]);
        assert_eq!(error.occurrences.len(), 4);
        assert!(error.message.contains("loop"));
    }

    #[test]
    fn refuses_merges_with_conflicting_parents() {
        let report = aggregate(&[
            repo(
                "a",
                vec![feature(
                    "auth",
                    "auth",
                    vec![feature("login", "auth/login", vec![])],
                )],
            ),
            repo(
                "b",
                vec![feature(
                    "users",
                    "users",
                    vec![feature("login", "users/login", vec![])],
                )],
            ),
        ]);

        assert_eq!(
            render(&report.features),
            "auth[a](login[a]) users[b](login[b])"
        );
        assert_eq!(report.errors.len(), 1);
        assert_eq!(
            report.errors[0].error_type,
            ReconciliationErrorType::ConflictingParents
        );
        assert_eq!(report.errors[0].features, vec!["login".to_string()]);
        let paths: Vec<&str> = report.errors[0]
            .occurrences
            .iter()
            .map(|o| o.feature_path.as_str())
            .collect();
        assert_eq!(paths, vec!["auth/login@a", "users/login@b"]);
    }

    #[test]
    fn children_of_unmerged_features_are_kept_apart_too() {
        let report = aggregate(&[
            repo(
                "a",
                vec![feature(
                    "B",
                    "b",
                    vec![feature("C", "b/c", vec![]), feature("X", "b/x", vec![])],
                )],
            ),
            repo(
                "d",
                vec![feature(
                    "C",
                    "c",
                    vec![feature("B", "c/b", vec![feature("X", "c/b/x", vec![])])],
                )],
            ),
        ]);

        assert_eq!(render(&report.features), "B[a](C[a] X[a]) C[d](B[d](X[d]))");
        assert_eq!(report.errors.len(), 2);
    }

    #[test]
    fn rewrites_dependency_paths_and_merges_stats() {
        let mut a = feature("auth", "src/auth", vec![]);
        a.dependencies.push(Dependency {
            source_filename: "src/auth/index.ts".into(),
            target_filename: "src/billing/index.ts".into(),
            line: 1,
            content: "import".into(),
            feature_path: "src/billing".into(),
            dependency_type: crate::models::DependencyType::Sibling,
        });
        a.stats = Some(Stats {
            files_count: Some(2),
            lines_count: Some(10),
            todos_count: None,
            commits: BTreeMap::from([("total_commits".to_string(), 3.into())]),
            coverage: None,
        });
        let mut b = feature("auth", "auth", vec![]);
        b.stats = Some(Stats {
            files_count: Some(1),
            lines_count: Some(5),
            todos_count: Some(1),
            commits: BTreeMap::from([("total_commits".to_string(), 4.into())]),
            coverage: None,
        });

        let report = aggregate(&[
            repo("a", vec![a, feature("billing", "src/billing", vec![])]),
            repo("b", vec![b]),
        ]);

        let auth = &report.features[0];
        assert_eq!(auth.dependencies[0].feature_path, "billing");
        let stats = auth.stats.as_ref().unwrap();
        assert_eq!(stats.files_count, Some(3));
        assert_eq!(stats.lines_count, Some(15));
        assert_eq!(stats.todos_count, Some(1));
        assert_eq!(stats.commits["total_commits"], 7);
    }

    #[test]
    fn keeps_first_owner_and_description() {
        let mut a = feature("auth", "auth", vec![]);
        a.owner = "parent-team".into();
        a.is_owner_inherited = true;
        let mut b = feature("auth", "auth", vec![]);
        b.owner = "team-b".into();
        b.description = "Authentication".into();

        let report = aggregate(&[repo("a", vec![a]), repo("b", vec![b])]);

        assert_eq!(report.features[0].owner, "team-b");
        assert!(!report.features[0].is_owner_inherited);
        assert_eq!(report.features[0].description, "Authentication");
    }
}
