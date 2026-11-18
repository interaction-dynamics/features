//! Claude skills bundled in the binary (`features skills list|install`).
//!
//! The skills of `projects/skills/skills` (see `build.rs`) are embedded at
//! compile time, so the installed skills always match the commands and JSON
//! fields of this version of the CLI. Installing copies each skill folder into
//! a skills folder (`.claude/skills` by default) and never touches the network.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use include_dir::{Dir, DirEntry, include_dir};

static SKILLS_DIR: Dir<'_> = include_dir!("$FEATURES_SKILLS_DIR");

/// File each skill folder must contain
const SKILL_FILE: &str = "SKILL.md";

/// File written at the root of the skills folder with the CLI version that
/// installed the skills
pub const VERSION_FILE: &str = ".features-version";

/// Default skills folder of a project, relative to the current directory
pub const PROJECT_SKILLS_DIR: &str = ".claude/skills";

/// Files that may sit in the source folder but must not be installed
const IGNORED_NAMES: &[&str] = &["__pycache__", ".DS_Store"];

/// A skill bundled in the binary
#[derive(Debug, Clone)]
pub struct Skill {
    /// Folder name, also the skill name
    pub name: String,
    /// `description` of the `SKILL.md` frontmatter
    pub description: String,
    dir: &'static Dir<'static>,
}

/// What happens to a skill when installing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStatus {
    /// The skill folder doesn't exist yet
    Installed,
    /// The skill folder exists and is left untouched (no `--force`)
    Skipped,
    /// The skill folder exists and its files are replaced (`--force`)
    Overwritten,
}

impl InstallStatus {
    pub fn label(self) -> &'static str {
        match self {
            InstallStatus::Installed => "installed",
            InstallStatus::Skipped => "skipped (already exists, use --force to overwrite)",
            InstallStatus::Overwritten => "overwritten",
        }
    }
}

/// The result of installing the skills
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallReport {
    /// Absolute or relative skills folder the skills were installed into
    pub target: PathBuf,
    /// Each skill with what happened to it, sorted by name
    pub skills: Vec<(String, InstallStatus)>,
    /// CLI version that installed the skills before, if any
    pub previous_version: Option<String>,
}

/// The skills bundled in the binary, sorted by name
pub fn bundled_skills() -> Vec<Skill> {
    let mut skills: Vec<Skill> = SKILLS_DIR
        .dirs()
        .filter_map(|dir| {
            let name = dir.path().file_name()?.to_str()?.to_string();
            let skill_file = dir.get_file(dir.path().join(SKILL_FILE))?;
            let description = skill_file
                .contents_utf8()
                .and_then(frontmatter_description)
                .unwrap_or_default();
            Some(Skill {
                name,
                description,
                dir,
            })
        })
        .collect();
    skills.sort_by(|a, b| a.name.cmp(&b.name));
    skills
}

/// The `description` field of a `SKILL.md` frontmatter
fn frontmatter_description(content: &str) -> Option<String> {
    let rest = content.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let frontmatter = &rest[..end];
    if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(frontmatter)
        && let Some(description) = yaml.get("description").and_then(|value| value.as_str())
    {
        return Some(description.to_string());
    }
    // Agents read a one-line description even when it isn't valid YAML (e.g.
    // it contains `: `), so fall back to the raw line
    frontmatter.lines().find_map(|line| {
        let value = line.strip_prefix("description:")?.trim();
        (!value.is_empty()).then(|| value.trim_matches(['"', '\'']).to_string())
    })
}

/// The skills folder to install into: `dir` when given, `~/.claude/skills`
/// with `global`, `./.claude/skills` otherwise
pub fn target_dir(dir: Option<&Path>, global: bool, current_dir: &Path) -> Result<PathBuf> {
    if let Some(dir) = dir {
        return Ok(if dir.is_absolute() {
            dir.to_path_buf()
        } else {
            current_dir.join(dir)
        });
    }
    if global {
        let Some(home) = home_dir() else {
            bail!("could not find the home directory: use --dir to choose the skills folder");
        };
        return Ok(home.join(PROJECT_SKILLS_DIR));
    }
    Ok(current_dir.join(PROJECT_SKILLS_DIR))
}

fn home_dir() -> Option<PathBuf> {
    ["HOME", "USERPROFILE"]
        .iter()
        .filter_map(std::env::var_os)
        .map(PathBuf::from)
        .find(|path| !path.as_os_str().is_empty())
}

/// What installing into `target` would do to each bundled skill
pub fn plan_install(skills: &[Skill], target: &Path, force: bool) -> Vec<(String, InstallStatus)> {
    skills
        .iter()
        .map(|skill| {
            let status = match (target.join(&skill.name).exists(), force) {
                (false, _) => InstallStatus::Installed,
                (true, false) => InstallStatus::Skipped,
                (true, true) => InstallStatus::Overwritten,
            };
            (skill.name.clone(), status)
        })
        .collect()
}

/// Copy the bundled skills into `target`. An existing skill folder is left
/// untouched unless `force`, in which case its files are overwritten (files
/// that are not part of the bundled skill are kept). With `dry_run`, nothing
/// is written.
pub fn install(target: &Path, force: bool, dry_run: bool) -> Result<InstallReport> {
    let skills = bundled_skills();
    if skills.is_empty() {
        bail!("this build of features-cli contains no skills");
    }
    if target.exists() && !target.is_dir() {
        bail!("`{}` exists and is not a folder", target.display());
    }

    let plan = plan_install(&skills, target, force);
    let previous_version = fs::read_to_string(target.join(VERSION_FILE))
        .ok()
        .map(|version| version.trim().to_string())
        .filter(|version| !version.is_empty());

    if !dry_run {
        for (skill, (_, status)) in skills.iter().zip(&plan) {
            if *status != InstallStatus::Skipped {
                write_dir(skill.dir, &target.join(&skill.name), skill.dir.path())?;
            }
        }
        if plan
            .iter()
            .any(|(_, status)| *status != InstallStatus::Skipped)
        {
            fs::write(
                target.join(VERSION_FILE),
                format!("{}\n", env!("CARGO_PKG_VERSION")),
            )
            .with_context(|| format!("could not write `{}`", target.display()))?;
        }
    }

    Ok(InstallReport {
        target: target.to_path_buf(),
        skills: plan,
        previous_version,
    })
}

/// Write the files of `dir` into `destination`. Embedded paths are relative
/// to the skills root, so `root` (the skill folder) is stripped from them.
fn write_dir(dir: &Dir<'_>, destination: &Path, root: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("could not create `{}`", destination.display()))?;

    for entry in dir.entries() {
        let name = entry.path().file_name().and_then(|name| name.to_str());
        if name.is_some_and(|name| IGNORED_NAMES.contains(&name)) {
            continue;
        }
        let path = destination.join(entry.path().strip_prefix(root).unwrap_or(entry.path()));
        match entry {
            DirEntry::Dir(sub_dir) => write_dir(sub_dir, &path, sub_dir.path())?,
            DirEntry::File(file) => fs::write(&path, file.contents())
                .with_context(|| format!("could not write `{}`", path.display()))?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundles_the_repository_skills_with_their_description() {
        let skills = bundled_skills();
        let names: Vec<&str> = skills.iter().map(|skill| skill.name.as_str()).collect();
        for expected in [
            "create-feature",
            "document-feature",
            "explore-features",
            "extract-feature",
            "organize-by-feature",
            "plan-feature-breakdown",
            "validate-features",
        ] {
            assert!(
                names.contains(&expected),
                "{expected} missing from {names:?}"
            );
        }
        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(names, sorted);
        assert!(skills.iter().all(|skill| !skill.description.is_empty()));
    }

    #[test]
    fn reads_the_description_of_the_frontmatter() {
        assert_eq!(
            frontmatter_description("---\nname: x\ndescription: Does things - well.\n---\n# X"),
            Some("Does things - well.".to_string())
        );
        assert_eq!(
            frontmatter_description(
                "---\nname: x\ndescription: Plans (paths: shared code) first.\n---\n"
            ),
            Some("Plans (paths: shared code) first.".to_string())
        );
        assert_eq!(frontmatter_description("# No frontmatter"), None);
    }

    #[test]
    fn plans_installed_skipped_and_overwritten() {
        let target = tempfile::tempdir().unwrap();
        let skills = bundled_skills();
        fs::create_dir_all(target.path().join("create-feature")).unwrap();

        let plan = plan_install(&skills, target.path(), false);
        let status = |plan: &[(String, InstallStatus)], name: &str| {
            plan.iter().find(|(n, _)| n == name).unwrap().1
        };
        assert_eq!(status(&plan, "create-feature"), InstallStatus::Skipped);
        assert_eq!(status(&plan, "document-feature"), InstallStatus::Installed);

        let plan = plan_install(&skills, target.path(), true);
        assert_eq!(status(&plan, "create-feature"), InstallStatus::Overwritten);
        assert_eq!(status(&plan, "document-feature"), InstallStatus::Installed);
    }

    #[test]
    fn target_dir_defaults_to_the_project_skills_folder() {
        let current = Path::new("/work/project");
        assert_eq!(
            target_dir(None, false, current).unwrap(),
            current.join(".claude/skills")
        );
        assert_eq!(
            target_dir(Some(Path::new("agents/skills")), false, current).unwrap(),
            current.join("agents/skills")
        );
        assert_eq!(
            target_dir(Some(Path::new("/abs/skills")), true, current).unwrap(),
            PathBuf::from("/abs/skills")
        );
    }
}
