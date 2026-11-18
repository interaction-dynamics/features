//! Resolves the project name shown in the web dashboard.
//!
//! In order of priority:
//! 1. `--project-name` on `features serve` and `features build`, or `name` in
//!    `features.config.yaml` (applied by [`crate::config`] as the same option)
//! 2. The name declared by the manifest of the main technology of the project
//!    (`package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, ...), looked
//!    up in the scanned folder, then in its parents up to the root of the git
//!    repository (`features serve ./src` finds the `package.json` at the root
//!    of the repository). The nearest one wins.
//!
//! When none is found, no name is written to `metadata.json` and the dashboard
//! shows "Features".

use std::fs;
use std::path::{Path, PathBuf};

/// Resolves the project name: the override, then the manifests, looked up
/// from `dir` up to the root of its git repository.
pub fn resolve_project_name(dir: &Path, cli_override: Option<&str>) -> Option<String> {
    if let Some(name) = non_empty(cli_override.map(str::to_string)) {
        return Some(name);
    }
    lookup_dirs(dir)
        .iter()
        .find_map(|dir| detect_project_name(dir))
}

/// `dir` and its parents up to the root of its git repository (the first one
/// containing `.git`). Outside a git repository, only `dir`.
pub(crate) fn lookup_dirs(dir: &Path) -> Vec<PathBuf> {
    let dir = fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut dirs = Vec::new();
    for ancestor in dir.ancestors() {
        dirs.push(ancestor.to_path_buf());
        if ancestor.join(".git").exists() {
            return dirs;
        }
    }
    vec![dir]
}

/// Reads the project name from the content of a manifest
type Detector = fn(&str) -> Option<String>;

/// Name declared by the first manifest found in `dir`, checked in order of
/// the most common technologies.
pub fn detect_project_name(dir: &Path) -> Option<String> {
    const DETECTORS: &[(&str, Detector)] = &[
        ("package.json", from_json_name),
        ("Cargo.toml", from_cargo_toml),
        ("pyproject.toml", from_pyproject_toml),
        ("go.mod", from_go_mod),
        ("composer.json", from_json_name),
        ("pom.xml", from_pom_xml),
        ("settings.gradle.kts", from_gradle_settings),
        ("settings.gradle", from_gradle_settings),
        ("pubspec.yaml", from_pubspec_yaml),
        ("deno.json", from_json_name),
    ];

    DETECTORS.iter().find_map(|(file, detect)| {
        let content = fs::read_to_string(dir.join(file)).ok()?;
        non_empty(detect(&content))
    })
}

fn non_empty(name: Option<String>) -> Option<String> {
    name.map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
}

/// `name` of `package.json`, `composer.json` or `deno.json`
fn from_json_name(content: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(content).ok()?;
    json.get("name")?.as_str().map(str::to_string)
}

/// `[package] name` of `Cargo.toml` (a virtual workspace has none)
fn from_cargo_toml(content: &str) -> Option<String> {
    let toml: toml::Value = toml::from_str(content).ok()?;
    toml.get("package")?
        .get("name")?
        .as_str()
        .map(str::to_string)
}

/// `[project] name` (PEP 621) or `[tool.poetry] name` of `pyproject.toml`
fn from_pyproject_toml(content: &str) -> Option<String> {
    let toml: toml::Value = toml::from_str(content).ok()?;
    let project_name = toml.get("project").and_then(|p| p.get("name"));
    let poetry_name = || toml.get("tool")?.get("poetry")?.get("name");
    project_name
        .or_else(poetry_name)?
        .as_str()
        .map(str::to_string)
}

/// Last segment of the `module` path of `go.mod`
fn from_go_mod(content: &str) -> Option<String> {
    let module = content
        .lines()
        .find_map(|line| line.trim().strip_prefix("module "))?;
    let module = module.trim().trim_matches('"');
    module.rsplit('/').next().map(str::to_string)
}

/// Project `<name>`, else `<artifactId>`, of `pom.xml` (ignoring the parent's)
fn from_pom_xml(content: &str) -> Option<String> {
    // Drop the <parent> block so its artifactId is not taken for the project's
    let without_parent = match (content.find("<parent>"), content.find("</parent>")) {
        (Some(start), Some(end)) if start < end => {
            format!(
                "{}{}",
                &content[..start],
                &content[end + "</parent>".len()..]
            )
        }
        _ => content.to_string(),
    };
    xml_tag(&without_parent, "name").or_else(|| xml_tag(&without_parent, "artifactId"))
}

fn xml_tag(content: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let start = content.find(&open)? + open.len();
    let end = content[start..].find(&format!("</{tag}>"))? + start;
    Some(content[start..end].to_string())
}

/// `rootProject.name` of `settings.gradle` or `settings.gradle.kts`
fn from_gradle_settings(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let value = line.trim().strip_prefix("rootProject.name")?;
        let value = value.trim_start().strip_prefix('=')?.trim();
        Some(value.trim_matches(|c| c == '"' || c == '\'').to_string())
    })
}

/// Top-level `name` of `pubspec.yaml`
fn from_pubspec_yaml(content: &str) -> Option<String> {
    let yaml: serde_yaml::Value = serde_yaml::from_str(content).ok()?;
    yaml.get("name")?.as_str().map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir_with(files: &[(&str, &str)]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for (name, content) in files {
            fs::write(dir.path().join(name), content).unwrap();
        }
        dir
    }

    #[test]
    fn detects_each_manifest() {
        let cases: &[(&str, &str, &str)] = &[
            ("package.json", r#"{ "name": "web-app" }"#, "web-app"),
            ("Cargo.toml", "[package]\nname = \"cli-tool\"\n", "cli-tool"),
            ("pyproject.toml", "[project]\nname = \"py-lib\"\n", "py-lib"),
            (
                "pyproject.toml",
                "[tool.poetry]\nname = \"poetry-app\"\n",
                "poetry-app",
            ),
            (
                "go.mod",
                "module github.com/acme/billing\n\ngo 1.22\n",
                "billing",
            ),
            ("composer.json", r#"{ "name": "acme/shop" }"#, "acme/shop"),
            (
                "pom.xml",
                "<project><parent><artifactId>parent</artifactId></parent><artifactId>orders</artifactId></project>",
                "orders",
            ),
            (
                "pom.xml",
                "<project><artifactId>orders</artifactId><name>Orders API</name></project>",
                "Orders API",
            ),
            (
                "settings.gradle.kts",
                "rootProject.name = \"android-app\"\n",
                "android-app",
            ),
            (
                "settings.gradle",
                "rootProject.name = 'groovy-app'\n",
                "groovy-app",
            ),
            ("pubspec.yaml", "name: flutter_app\n", "flutter_app"),
        ];

        for (file, content, expected) in cases {
            let dir = dir_with(&[(file, content)]);
            assert_eq!(
                detect_project_name(dir.path()).as_deref(),
                Some(*expected),
                "{file}"
            );
        }
    }

    #[test]
    fn skips_manifests_without_a_name() {
        let dir = dir_with(&[
            ("package.json", r#"{ "private": true }"#),
            ("Cargo.toml", "[workspace]\nmembers = [\"cli\"]\n"),
            ("go.mod", "module example.com/monorepo\n"),
        ]);
        assert_eq!(detect_project_name(dir.path()).as_deref(), Some("monorepo"));
    }

    #[test]
    fn looks_up_parents_up_to_the_git_root() {
        let repo = dir_with(&[("package.json", r#"{ "name": "web-app" }"#)]);
        fs::create_dir(repo.path().join(".git")).unwrap();
        let src = repo.path().join("src");
        fs::create_dir(&src).unwrap();
        fs::write(src.join("package.json"), r#"{ "name": "nested" }"#).unwrap();

        // The nearest manifest wins
        assert_eq!(resolve_project_name(&src, None).as_deref(), Some("nested"));
        fs::remove_file(src.join("package.json")).unwrap();
        assert_eq!(resolve_project_name(&src, None).as_deref(), Some("web-app"));
    }

    #[test]
    fn stays_in_the_folder_outside_a_git_repository() {
        let parent = dir_with(&[("package.json", r#"{ "name": "outside" }"#)]);
        let child = parent.path().join("child");
        fs::create_dir(&child).unwrap();
        assert_eq!(resolve_project_name(&child, None), None);
    }

    #[test]
    fn nothing_found_without_manifest() {
        let dir = dir_with(&[]);
        assert_eq!(resolve_project_name(dir.path(), None), None);
    }

    #[test]
    fn override_wins() {
        let dir = dir_with(&[("package.json", r#"{ "name": "web-app" }"#)]);
        assert_eq!(
            resolve_project_name(dir.path(), Some("Acme")).as_deref(),
            Some("Acme")
        );
    }

    #[test]
    fn blank_override_is_ignored() {
        let dir = dir_with(&[("package.json", r#"{ "name": "web-app" }"#)]);
        assert_eq!(
            resolve_project_name(dir.path(), Some("  ")).as_deref(),
            Some("web-app")
        );
    }

    #[test]
    fn toml_config_file_is_not_read() {
        let dir = dir_with(&[
            ("package.json", r#"{ "name": "web-app" }"#),
            ("features.config.toml", "name = \"Acme Store\"\n"),
        ]);
        assert_eq!(
            resolve_project_name(dir.path(), None).as_deref(),
            Some("web-app")
        );
    }
}
