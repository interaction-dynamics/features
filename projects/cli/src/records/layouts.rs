//! Records written by spec-driven tools, at the root of the scanned folder:
//!
//! * **GitHub Spec Kit**: `specs/NNN-name/` folders with a `spec.md` main file
//!   and `plan.md`, `tasks.md`, `research.md`, `data-model.md`,
//!   `quickstart.md`, `contracts/*` sub-files. Loose markdown files in `specs/`
//!   are native specs.
//! * **Kiro**: `.kiro/specs/<name>/` folders with a `requirements.md` main file
//!   and `design.md`, `tasks.md` sub-files.
//!
//! Without a `feature:` frontmatter field, these records are attached to the
//! feature their folder name (without its `NNN-` prefix) names, see
//! [`super::split_root_records`].

use std::fs;
use std::path::{Path, PathBuf};

use super::{count_tasks, is_markdown, is_readme, parse_record, relative_path, split_frontmatter};
use crate::models::{Record, RecordFile, RecordKind, RecordSource};

/// Order of the sub-files of a folder, the others come after, sorted by name
const FILE_ORDER: &[&str] = &[
    "requirements.md",
    "design.md",
    "plan.md",
    "research.md",
    "data-model.md",
    "quickstart.md",
    "tasks.md",
];

/// Extensions of the sub-files worth showing (Spec Kit contracts are often YAML or JSON)
const TEXT_EXTENSIONS: &[&str] = &[
    "md", "mdx", "txt", "yaml", "yml", "json", "graphql", "gql", "proto", "sql",
];

/// Headings that don't name the spec: Kiro uses them as the title of every file
const GENERIC_TITLES: &[&str] = &[
    "requirements",
    "requirements document",
    "design",
    "design document",
    "implementation plan",
    "tasks",
    "spec",
    "specification",
];

fn sorted_entries(dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map(|entries| entries.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    entries.sort();
    entries
}

fn name_of(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Template files document how to write records, they aren't records
fn is_template(path: &Path) -> bool {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().to_lowercase())
        .is_some_and(|stem| stem == "template" || stem.ends_with("-template"))
}

/// Read Spec Kit's root `specs/` folder
pub(super) fn read_speckit_dir(dir: &Path, base_path: &Path) -> Vec<Record> {
    sorted_entries(dir)
        .into_iter()
        .filter_map(|path| {
            if path.is_dir() {
                read_tool_folder(&path, RecordSource::Speckit, "spec.md", base_path)
            } else if is_markdown(&path) && !is_readme(&path) && !is_template(&path) {
                let raw = fs::read_to_string(&path).ok()?;
                let id = path.file_stem()?.to_string_lossy().to_string();
                Some(parse_record(
                    RecordKind::Spec,
                    &id,
                    &relative_path(&path, base_path),
                    &raw,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Read Kiro's `.kiro/specs/` folder
pub(super) fn read_kiro_dir(dir: &Path, base_path: &Path) -> Vec<Record> {
    sorted_entries(dir)
        .into_iter()
        .filter(|path| path.is_dir())
        .filter_map(|path| {
            read_tool_folder(&path, RecordSource::Kiro, "requirements.md", base_path)
        })
        .collect()
}

/// A Spec Kit or Kiro folder: its main file and its other text files, nested
/// ones included (`contracts/api.yaml`). Folders without the main file are skipped.
fn read_tool_folder(
    dir: &Path,
    source: RecordSource,
    main_file: &str,
    base_path: &Path,
) -> Option<Record> {
    let main_path = dir.join(main_file);
    if name_of(dir).starts_with('.') || !main_path.is_file() {
        return None;
    }
    let id = name_of(dir);
    let raw = fs::read_to_string(&main_path).ok()?;
    let mut record = parse_record(
        RecordKind::Spec,
        &id,
        &relative_path(&main_path, base_path),
        &raw,
    );
    record.source = source;

    let mut files: Vec<(String, PathBuf)> = walkdir::WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || !name_of(entry.path()).starts_with('.'))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.path() != main_path)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|ext| TEXT_EXTENSIONS.iter().any(|t| ext.eq_ignore_ascii_case(t)))
        })
        .map(|entry| (relative_path(entry.path(), dir), entry.path().to_path_buf()))
        .collect();
    files.sort_by_key(|(name, _)| {
        let rank = FILE_ORDER
            .iter()
            .position(|f| f == name)
            .unwrap_or(FILE_ORDER.len() + usize::from(name.contains('/')));
        (rank, name.clone())
    });
    record.files = files
        .into_iter()
        .filter_map(|(name, path)| {
            Some(RecordFile {
                content: fs::read_to_string(&path).ok()?,
                path: relative_path(&path, base_path),
                name,
            })
        })
        .collect();

    // Progress comes from tasks.md: spec.md checklists are review checklists
    if let Some(tasks_file) = record.files.iter().find(|file| file.name == "tasks.md") {
        let tasks = count_tasks(&tasks_file.content);
        record.tasks = (tasks.total > 0).then_some(tasks);
    }

    record.title = tool_title(&record, source, &raw);
    if record.date.is_none() {
        record.date = created_date(&record.content);
    }
    Some(record)
}

/// Spec Kit titles its specs `Feature Specification: <name>`; Kiro titles
/// every file `Requirements Document`, so its folder name is used instead
fn tool_title(record: &Record, source: RecordSource, raw: &str) -> String {
    let (frontmatter, _) = split_frontmatter(raw);
    if frontmatter.is_some_and(|map| map.contains_key("title")) {
        return record.title.clone();
    }
    let title = record.title.trim();
    let title = title
        .strip_prefix("Feature Specification:")
        .map(str::trim)
        .unwrap_or(title);
    if title == record.id
        || (source == RecordSource::Kiro && GENERIC_TITLES.contains(&title.to_lowercase().as_str()))
    {
        humanize(&record.id)
    } else {
        title.to_string()
    }
}

/// Spec Kit's `**Created**: 2025-01-15` line
fn created_date(body: &str) -> Option<String> {
    body.lines()
        .find_map(|line| {
            let line = line
                .trim()
                .trim_start_matches(['*', '-', '+'])
                .replace("**", "");
            let (key, value) = line.split_once(':')?;
            key.trim().eq_ignore_ascii_case("created").then(|| {
                value
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .trim_end_matches([',', '.', ';'])
                    .to_string()
            })
        })
        .filter(|date| !date.is_empty())
}

/// `001-login-flow` → `login-flow`
pub(super) fn strip_number_prefix(id: &str) -> &str {
    let digits = id.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 {
        return id;
    }
    id[digits..].strip_prefix(['-', '_']).unwrap_or(id)
}

/// `003-notifications-center` → `Notifications center`
fn humanize(id: &str) -> String {
    let words = strip_number_prefix(id).replace(['-', '_'], " ");
    let mut chars = words.trim().chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => id.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TaskProgress;

    #[test]
    fn test_speckit_folder() {
        let temp = tempfile::TempDir::new().unwrap();
        let base = temp.path();
        let dir = base.join("specs").join("003-checkout");
        fs::create_dir_all(dir.join("contracts")).unwrap();
        fs::write(
            dir.join("spec.md"),
            "# Feature Specification: Checkout\n\n**Created**: 2025-02-01\n**Status**: Draft\n\n- [ ] Review item\n",
        )
        .unwrap();
        fs::write(dir.join("tasks.md"), "- [x] a\n- [ ] b\n- [ ] c\n").unwrap();
        fs::write(dir.join("plan.md"), "# Plan\n").unwrap();
        fs::write(dir.join("contracts").join("api.yaml"), "openapi: 3.0.0\n").unwrap();
        fs::write(dir.join("logo.png"), [0u8, 1, 2]).unwrap();
        fs::write(base.join("specs").join("README.md"), "# Specs\n").unwrap();
        fs::write(base.join("specs").join("010-loose.md"), "# Loose\n").unwrap();

        let records = read_speckit_dir(&base.join("specs"), base);
        assert_eq!(records.len(), 2);
        let record = &records[0];
        assert_eq!(record.id, "003-checkout");
        assert_eq!(record.source, RecordSource::Speckit);
        assert_eq!(record.title, "Checkout");
        assert_eq!(record.status.as_deref(), Some("draft"));
        assert_eq!(record.date.as_deref(), Some("2025-02-01"));
        let names: Vec<&str> = record.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["plan.md", "tasks.md", "contracts/api.yaml"]);
        assert_eq!(record.tasks, Some(TaskProgress { done: 1, total: 3 }));
        assert_eq!(records[1].id, "010-loose");
        assert_eq!(records[1].source, RecordSource::Native);
    }

    #[test]
    fn test_kiro_folder() {
        let temp = tempfile::TempDir::new().unwrap();
        let base = temp.path();
        let dir = base
            .join(".kiro")
            .join("specs")
            .join("notifications-center");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("requirements.md"), "# Requirements Document\n").unwrap();
        fs::write(dir.join("design.md"), "# Design Document\n").unwrap();
        fs::write(dir.join("tasks.md"), "- [x] a\n").unwrap();
        fs::create_dir_all(base.join(".kiro/specs/empty")).unwrap();

        let records = read_kiro_dir(&base.join(".kiro").join("specs"), base);
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(record.source, RecordSource::Kiro);
        assert_eq!(record.title, "Notifications center");
        assert_eq!(
            record.path,
            ".kiro/specs/notifications-center/requirements.md"
        );
        let names: Vec<&str> = record.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["design.md", "tasks.md"]);
        assert_eq!(record.tasks, Some(TaskProgress { done: 1, total: 1 }));
    }
}
