//! Markdown helpers: frontmatter, title, status, date and task checkboxes.

use serde_yaml::Value;

/// YAML frontmatter of a record, empty when the file has none or it is invalid
#[derive(Debug, Default)]
pub(super) struct Frontmatter(Option<serde_yaml::Mapping>);

impl Frontmatter {
    fn get(&self, key: &str) -> Option<&Value> {
        self.0.as_ref()?.get(Value::String(key.to_string()))
    }

    /// A scalar value as a string (`2026-01-02`, `accepted`, `3`)
    pub(super) fn string(&self, key: &str) -> Option<String> {
        scalar_to_string(self.get(key)?).filter(|value| !value.is_empty())
    }

    /// A list of strings, from a YAML sequence or a comma-separated scalar
    pub(super) fn list(&self, key: &str) -> Vec<String> {
        match self.get(key) {
            Some(Value::Sequence(items)) => items.iter().filter_map(scalar_to_string).collect(),
            Some(value) => scalar_to_string(value)
                .map(|value| value.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            None => Vec::new(),
        }
        .into_iter()
        .filter(|value| !value.is_empty())
        .collect()
    }
}

fn scalar_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.trim().to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Split a Markdown file into its frontmatter and its body
pub(super) fn split_frontmatter(content: &str) -> (Frontmatter, &str) {
    let text = content.strip_prefix('\u{feff}').unwrap_or(content);
    let Some(rest) = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))
    else {
        return (Frontmatter::default(), text);
    };

    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            let yaml = &rest[..offset];
            let body = &rest[offset + line.len()..];
            let mapping = serde_yaml::from_str::<Value>(yaml)
                .ok()
                .and_then(|value| match value {
                    Value::Mapping(mapping) => Some(mapping),
                    _ => None,
                });
            return (Frontmatter(mapping), body);
        }
        offset += line.len();
    }

    (Frontmatter::default(), text)
}

/// Remove Markdown emphasis, code and link markup from a short text
pub(super) fn strip_markup(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '*' | '_' | '`' | '~' => {}
            '[' => {
                // Keep the label of `[label](url)`, drop the url
                let label: String = chars.by_ref().take_while(|&c| c != ']').collect();
                out.push_str(&label);
                if chars.peek() == Some(&'(') {
                    for c in chars.by_ref() {
                        if c == ')' {
                            break;
                        }
                    }
                }
            }
            _ => out.push(c),
        }
    }
    out.trim().to_string()
}

/// The text of the first `#` heading, preferring a level 1 heading
pub(super) fn first_heading(body: &str) -> Option<String> {
    let headings: Vec<(usize, String)> = body
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.starts_with('#') {
                return None;
            }
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            let text = strip_markup(trimmed[level..].trim());
            (!text.is_empty()).then_some((level, text))
        })
        .collect();

    headings
        .iter()
        .find(|(level, _)| *level == 1)
        .or_else(|| headings.first())
        .map(|(_, text)| {
            // Spec Kit titles its specs `Feature Specification: <name>`
            text.strip_prefix("Feature Specification:")
                .map(str::trim)
                .unwrap_or(text)
                .to_string()
        })
}

/// The value of a `**Key**: value`, `**Key:** value` or `Key: value` line
fn labelled_value(body: &str, keys: &[&str]) -> Option<String> {
    for line in body.lines() {
        let trimmed = line
            .trim()
            .trim_start_matches(['-', '*', '+', '>'])
            .trim_start();
        // `**Status**: Draft` and `**Status:** Draft` both become `Status: Draft`
        let plain = trimmed.replace("**", "").replace("__", "");
        let Some((label, value)) = plain.split_once(':') else {
            continue;
        };
        if keys
            .iter()
            .any(|key| label.trim().eq_ignore_ascii_case(key))
        {
            let value = strip_markup(value);
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

/// The first line of text below a `## <title>` heading
fn section_first_line(body: &str, title: &str) -> Option<String> {
    let mut in_section = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            if in_section {
                return None;
            }
            in_section = trimmed
                .trim_start_matches('#')
                .trim()
                .eq_ignore_ascii_case(title);
            continue;
        }
        if in_section && !trimmed.is_empty() {
            let text = strip_markup(trimmed.trim_start_matches(['-', '*', '+']));
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Lowercased first word of a status: `Superseded by 0005` → `superseded`
pub(super) fn normalize_status(raw: &str) -> Option<String> {
    let word: String = strip_markup(raw)
        .chars()
        .skip_while(|c| !c.is_alphanumeric())
        .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    (!word.is_empty()).then(|| word.to_lowercase())
}

/// Status from a `**Status**:` line, else from a `## Status` section
pub(super) fn status_from_body(body: &str) -> Option<String> {
    labelled_value(body, &["status"])
        .or_else(|| section_first_line(body, "status"))
        .and_then(|raw| normalize_status(&raw))
}

/// Date from a `**Created**:`, `**Date**:` or `Date:` line
pub(super) fn date_from_body(body: &str) -> Option<String> {
    labelled_value(body, &["date", "created"]).and_then(|value| {
        value
            .split_whitespace()
            .next()
            .map(|word| word.trim_end_matches([',', '.', ';']).to_string())
    })
}

/// Count the done and total `- [x]` / `- [ ]` checkboxes
pub(super) fn count_tasks(content: &str) -> (usize, usize) {
    let mut done = 0;
    let mut total = 0;
    for line in content.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
            .or_else(|| trimmed.strip_prefix("+ "))
        else {
            continue;
        };
        let rest = rest.trim_start();
        if rest.starts_with("[ ]") {
            total += 1;
        } else if rest.starts_with("[x]") || rest.starts_with("[X]") {
            total += 1;
            done += 1;
        }
    }
    (done, total)
}

/// `001-login-flow` → `Login flow`
pub(super) fn humanize(id: &str) -> String {
    let words = strip_number_prefix(id).replace(['-', '_'], " ");
    let mut chars = words.trim().chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => id.to_string(),
    }
}

/// `001-login-flow` → `login-flow`
pub(super) fn strip_number_prefix(id: &str) -> &str {
    let digits = id.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 {
        return id;
    }
    match id[digits..].strip_prefix(['-', '_']) {
        Some(rest) if !rest.is_empty() => rest,
        _ => id,
    }
}

/// Lowercase, alphanumeric words joined by `-`: `Login Flow` → `login-flow`
pub(super) fn slugify(text: &str) -> String {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_frontmatter() {
        let (fm, body) = split_frontmatter(
            "---\nstatus: accepted\ndate: 2026-01-02\ndecisions: [002-use-jwt, 003-x]\nspec: 001-login\n---\n# Title\n",
        );
        assert_eq!(fm.string("status").as_deref(), Some("accepted"));
        assert_eq!(fm.string("date").as_deref(), Some("2026-01-02"));
        assert_eq!(fm.list("decisions"), vec!["002-use-jwt", "003-x"]);
        assert_eq!(fm.list("spec"), vec!["001-login"]);
        assert_eq!(body, "# Title\n");

        let (fm, body) = split_frontmatter("# No frontmatter\n");
        assert!(fm.string("status").is_none());
        assert_eq!(body, "# No frontmatter\n");
    }

    #[test]
    fn test_status_fallbacks() {
        assert_eq!(
            status_from_body("# Spec\n\n**Status**: Draft\n").as_deref(),
            Some("draft")
        );
        assert_eq!(
            status_from_body("# ADR\n\n**Status:** Accepted\n").as_deref(),
            Some("accepted")
        );
        assert_eq!(
            status_from_body(
                "# 1. Use JWT\n\n## Status\n\nSuperseded by [5](0005-x.md)\n\n## Context\n"
            )
            .as_deref(),
            Some("superseded")
        );
        assert_eq!(status_from_body("# Nothing\n\nSome text\n"), None);
    }

    #[test]
    fn test_date_and_title() {
        assert_eq!(
            date_from_body("**Created**: 2025-09-01\n").as_deref(),
            Some("2025-09-01")
        );
        assert_eq!(
            first_heading("## Context\n# Feature Specification: Photo albums\n").as_deref(),
            Some("Photo albums")
        );
    }

    #[test]
    fn test_count_tasks() {
        let tasks = "- [x] T001 done\n- [ ] T002 open\n  * [X] nested\n- not a task\n";
        assert_eq!(count_tasks(tasks), (2, 3));
    }

    #[test]
    fn test_names() {
        assert_eq!(humanize("003-single-sign-on"), "Single sign on");
        assert_eq!(strip_number_prefix("001-auth"), "auth");
        assert_eq!(strip_number_prefix("auth"), "auth");
        assert_eq!(strip_number_prefix("2026"), "2026");
        assert_eq!(slugify("Login Flow!"), "login-flow");
    }
}
