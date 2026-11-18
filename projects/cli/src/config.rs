//! Default options read from `features.config.yaml`.
//!
//! Every option of the command line can be set in the configuration file, by
//! its long name without `--`. Options shared by several commands are at the
//! top level; options of a single command are in a section named after it:
//!
//! ```yaml
//! name: Acme Store
//! path: src
//! ignore-path: [src/legacy]
//!
//! serve:
//!   port: 4000
//!
//! diff:
//!   since: origin/main
//! ```
//!
//! An option given on the command line wins over the file, which wins over
//! the built-in default. Lists (`ignore-path`, `data`, `allow-origin`) are
//! combined: the values of the file, then those of the command line.
//! Relative paths in the file are relative to the folder of the file.
//!
//! The file is looked up in the scanned folder (the current directory when
//! none is given), then in its parents up to the root of the git repository.
//! `--config <FILE>` reads another file and `--no-config` ignores it.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::checker::{CheckConfig, CliOverrides, OutputFormat};
use crate::cli::{Action, ConfigSource, Invocation, ParsedArgs};
use crate::http_server::parse_allowed_origin;
use crate::project_name::lookup_dirs;

/// Name of the configuration file, usually at the root of the repository
pub const CONFIG_FILE_NAME: &str = "features.config.yaml";

/// Configuration file of earlier versions, which only held `name`
const LEGACY_CONFIG_FILE_NAME: &str = "features.config.toml";

/// Options of every command
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct FileConfig {
    /// Project name shown in the web dashboard (`--project-name`)
    pub name: Option<String>,
    /// Folder to scan for features when the command line gives none
    pub path: Option<PathBuf>,
    pub skip_changes: Option<bool>,
    pub ignore_path: Vec<PathBuf>,
    pub include_skipped_dirs: Option<bool>,
    pub coverage_dir: Option<PathBuf>,
    pub project_dir: Option<PathBuf>,
    pub strip_emails: Option<bool>,
    pub data: Vec<PathBuf>,
    pub codeowners_path: Option<PathBuf>,
    pub codeowners_prefix: Option<String>,

    pub scan: ScanSection,
    pub serve: ServeSection,
    pub build: BuildSection,
    pub check: CheckSection,
    pub diff: DiffSection,
    pub find_owner: JsonSection,
    pub owners: JsonSection,
    pub records: JsonSection,
    pub aggregate: AggregateSection,
}

/// Options of `features scan`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct ScanSection {
    pub json: Option<bool>,
    pub flat: Option<bool>,
    pub description: Option<bool>,
    pub coverage: Option<bool>,
}

/// Options of `features serve`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct ServeSection {
    pub port: Option<u16>,
    pub allow_origin: Vec<String>,
    pub experimental: Option<bool>,
    pub generate_codeowners: Option<bool>,
}

/// Options of `features build`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct BuildSection {
    pub build_dir: Option<PathBuf>,
    pub experimental: Option<bool>,
    pub generate_codeowners: Option<bool>,
}

/// Options of `features check`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct CheckSection {
    pub generate_codeowners: Option<bool>,
    pub json: Option<bool>,
    pub format: Option<OutputFormat>,
    /// Severity and options of the rules, by rule id (see [`crate::checker`])
    pub rules: Option<serde_yaml::Value>,
}

/// Options of `features diff`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct DiffSection {
    pub since: Option<String>,
    pub committed_only: Option<bool>,
    pub json: Option<bool>,
    pub markdown: Option<bool>,
    pub owners_only: Option<bool>,
}

/// Options of `features find-owner`, `owners` and `records`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct JsonSection {
    pub json: Option<bool>,
}

/// Options of `features aggregate`
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct AggregateSection {
    pub build_dir: Option<PathBuf>,
    pub json: Option<bool>,
    pub data: Vec<PathBuf>,
}

impl FileConfig {
    /// Parse the content of a configuration file
    pub fn parse(content: &str) -> Result<Self> {
        // A file without any option (empty, or only comments) is an empty config
        let value: serde_yaml::Value = serde_yaml::from_str(content)?;
        if value.is_null() {
            return Ok(Self::default());
        }
        // Parsed again from the text, so errors keep their line and column
        Ok(serde_yaml::from_str(content)?)
    }

    /// Read the configuration file at `file`, with its relative paths made
    /// relative to `current_dir` instead of the folder of the file
    pub fn read(file: &Path, current_dir: &Path) -> Result<Self> {
        let path = current_dir.join(file);
        let content = fs::read_to_string(&path)
            .with_context(|| format!("could not read {}", file.display()))?;
        let mut config =
            Self::parse(&content).with_context(|| format!("invalid {}", file.display()))?;
        let dir = path.parent().unwrap_or(current_dir);
        config.resolve_paths(&relative_to(dir, current_dir));
        Ok(config)
    }

    /// Prefix the relative paths with `base`
    fn resolve_paths(&mut self, base: &Path) {
        let resolve = |path: &mut PathBuf| {
            if path.is_relative() {
                *path = base.join(&*path);
            }
        };
        let singles = [
            &mut self.path,
            &mut self.coverage_dir,
            &mut self.project_dir,
            &mut self.codeowners_path,
            &mut self.build.build_dir,
            &mut self.aggregate.build_dir,
        ];
        singles.into_iter().flatten().for_each(resolve);
        self.ignore_path
            .iter_mut()
            .chain(&mut self.data)
            .chain(&mut self.aggregate.data)
            .for_each(resolve);
    }
}

/// `dir` relative to `current_dir` when it's inside it, else `dir` as is
fn relative_to(dir: &Path, current_dir: &Path) -> PathBuf {
    let canonical = |path: &Path| fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let dir = canonical(dir);
    match dir.strip_prefix(canonical(current_dir)) {
        Ok(relative) => relative.to_path_buf(),
        Err(_) => dir,
    }
}

/// The nearest `features.config.yaml`, from `dir` up to the root of its git
/// repository. Warns about a `features.config.toml` of earlier versions found
/// on the way, which is no longer read.
pub fn find_config_file(dir: &Path) -> Option<PathBuf> {
    for dir in lookup_dirs(dir) {
        let file = dir.join(CONFIG_FILE_NAME);
        if file.is_file() {
            return Some(file);
        }
        let legacy = dir.join(LEGACY_CONFIG_FILE_NAME);
        if legacy.is_file() {
            eprintln!(
                "Warning: {} is no longer read: move its `name` to {} (`name: ...`)",
                legacy.display(),
                CONFIG_FILE_NAME
            );
        }
    }
    None
}

/// Apply the configuration file to a parsed command line, and check what can
/// only be checked once both are known
pub fn resolve(parsed: ParsedArgs, current_dir: &Path) -> Result<Invocation> {
    let ParsedArgs {
        mut invocation,
        config,
        explicit,
    } = parsed;

    if invocation.action != Action::Version {
        let file = match config {
            ConfigSource::Disabled => None,
            ConfigSource::File(file) => Some(file),
            ConfigSource::Lookup => {
                let dir = invocation
                    .action
                    .scan_path()
                    .map_or_else(|| current_dir.to_path_buf(), |p| current_dir.join(p));
                find_config_file(&dir)
            }
        };
        if let Some(file) = file {
            let config = FileConfig::read(&file, current_dir)?;
            apply(&config, &mut invocation, &explicit)
                .with_context(|| format!("invalid {}", file.display()))?;
        }
    }

    if let Action::Diff(args) = &invocation.action
        && args.since.is_none()
    {
        bail!(
            "`features diff` needs a git ref: pass `--since <REF>` or set `since` in the \
             `diff` section of {CONFIG_FILE_NAME}"
        );
    }

    Ok(invocation)
}

/// Fill the options not given on the command line (`explicit`) from `config`
pub fn apply(
    config: &FileConfig,
    invocation: &mut Invocation,
    explicit: &HashSet<String>,
) -> Result<()> {
    let fill = Fill(explicit);

    let scan = &mut invocation.scan;
    fill.set(&mut scan.skip_changes, "skip_changes", config.skip_changes);
    fill.set(
        &mut scan.include_skipped_dirs,
        "include_skipped_dirs",
        config.include_skipped_dirs,
    );
    fill.set(&mut scan.strip_emails, "strip_emails", config.strip_emails);
    fill.option(&mut scan.coverage_dir, "coverage_dir", &config.coverage_dir);
    fill.option(&mut scan.project_dir, "project_dir", &config.project_dir);
    prepend(&mut scan.ignore_path, &config.ignore_path);
    prepend(&mut scan.data, &config.data);

    let codeowners = &mut invocation.codeowners;
    fill.option(
        &mut codeowners.codeowners_path,
        "codeowners_path",
        &config.codeowners_path,
    );
    let prefix = config.codeowners_prefix.clone();
    fill.set(
        &mut codeowners.codeowners_prefix,
        "codeowners_prefix",
        prefix.clone(),
    );

    let generate = &mut invocation.generate_codeowners;
    let path = config.path.clone();
    let name = config.name.clone().map(Some);
    match &mut invocation.action {
        Action::Version => {}
        Action::List(args) => {
            fill.set(&mut args.path, "path", path);
            fill.set(&mut args.json, "json", config.scan.json);
            fill.set(&mut args.flat, "flat", config.scan.flat);
            fill.set(
                &mut args.description,
                "description",
                config.scan.description,
            );
            fill.set(&mut args.coverage, "coverage", config.scan.coverage);
        }
        Action::Serve(args) => {
            let section = &config.serve;
            fill.set(&mut args.path, "path", path);
            fill.set(&mut args.port, "port", section.port);
            fill.set(&mut args.project_name, "project_name", name);
            fill.set(&mut args.experimental, "experimental", section.experimental);
            fill.set(generate, "generate_codeowners", section.generate_codeowners);
            let origins = section
                .allow_origin
                .iter()
                .map(|origin| {
                    parse_allowed_origin(origin)
                        .map_err(|err| anyhow::anyhow!("serve.allow-origin: {err}"))
                })
                .collect::<Result<Vec<_>>>()?;
            prepend(&mut args.allow_origin, &origins);
        }
        Action::Build(args) => {
            let section = &config.build;
            fill.set(&mut args.path, "path", path);
            fill.set(&mut args.build_dir, "build_dir", section.build_dir.clone());
            fill.set(&mut args.project_name, "project_name", name);
            fill.set(&mut args.experimental, "experimental", section.experimental);
            fill.set(generate, "generate_codeowners", section.generate_codeowners);
        }
        Action::Check {
            path: arg,
            json,
            check,
        } => {
            let section = &config.check;
            fill.set(arg, "path", path);
            fill.set(generate, "generate_codeowners", section.generate_codeowners);

            // The output format is one choice: the command line's replaces the file's
            if !explicit.contains("json") && !explicit.contains("format") {
                *json = section.json.unwrap_or(*json);
                check.format = section.format.or(check.format);
            }

            // Checked now, so an unknown rule or option names the file
            CheckConfig::resolve(section.rules.as_ref(), &CliOverrides::default())?;
            check.file_rules = section.rules.clone();
        }
        Action::Codeowners { path: arg } | Action::Mcp { path: arg } => fill.set(arg, "path", path),
        Action::New(args) => fill.set(&mut args.path, "path", path),
        Action::FindOwner {
            path: arg, json, ..
        } => {
            fill.set(arg, "path", path.map(Some));
            fill.set(json, "json", config.find_owner.json);
        }
        Action::Owners { path: arg, json } => {
            fill.set(arg, "path", path);
            fill.set(json, "json", config.owners.json);
        }
        Action::Records { path: arg, json } => {
            fill.set(arg, "path", path);
            fill.set(json, "json", config.records.json);
        }
        Action::Diff(args) => {
            let section = &config.diff;
            fill.set(&mut args.path, "path", path.map(Some));
            fill.set(&mut args.since, "since", section.since.clone().map(Some));
            fill.set(&mut args.codeowners_prefix, "codeowners_prefix", prefix);
            fill.set(
                &mut args.committed_only,
                "committed_only",
                section.committed_only,
            );

            // The output format is one choice: the command line's replaces the file's
            let formats = [section.json, section.markdown, section.owners_only];
            if formats.iter().filter(|f| **f == Some(true)).count() > 1 {
                bail!("diff.json, diff.markdown and diff.owners-only can't be combined");
            }
            if !["json", "markdown", "owners_only"]
                .iter()
                .any(|id| explicit.contains(*id))
            {
                args.json = section.json.unwrap_or(args.json);
                args.markdown = section.markdown.unwrap_or(args.markdown);
                args.owners_only = section.owners_only.unwrap_or(args.owners_only);
            }
        }
        Action::Aggregate(args) => {
            let section = &config.aggregate;
            fill.set(&mut args.build_dir, "build_dir", section.build_dir.clone());
            fill.set(&mut args.json, "json", section.json);
            prepend(&mut args.data, &section.data);
        }
    }

    Ok(())
}

/// Fills the options not given on the command line
struct Fill<'a>(&'a HashSet<String>);

impl Fill<'_> {
    /// Set `field` to `value`, unless the argument `id` is on the command line
    fn set<T>(&self, field: &mut T, id: &str, value: Option<T>) {
        if let Some(value) = value
            && !self.0.contains(id)
        {
            *field = value;
        }
    }

    fn option<T: Clone>(&self, field: &mut Option<T>, id: &str, value: &Option<T>) {
        self.set(field, id, value.clone().map(Some));
    }
}

/// The values of the file, then those of the command line
fn prepend<T: Clone>(field: &mut Vec<T>, values: &[T]) {
    let mut combined = values.to_vec();
    combined.append(field);
    *field = combined;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{CodeownersOptions, DiffArgs, ServerArgs, try_parse_args_from};

    fn path(p: &str) -> PathBuf {
        PathBuf::from(p)
    }

    /// Parse `args`, then apply the configuration `yaml`
    fn run(yaml: &str, args: &[&str]) -> Result<Invocation> {
        let args = std::iter::once("features").chain(args.iter().copied());
        let parsed = try_parse_args_from(args).unwrap();
        let mut invocation = parsed.invocation;
        apply(&FileConfig::parse(yaml)?, &mut invocation, &parsed.explicit)?;
        Ok(invocation)
    }

    #[test]
    fn test_empty_config_changes_nothing() {
        for args in [
            vec!["scan", "./src", "--json"],
            vec!["serve"],
            vec!["./src", "--build"],
            vec!["diff", "--since", "main"],
            vec!["aggregate", "./reports"],
        ] {
            let full: Vec<&str> = std::iter::once("features").chain(args.clone()).collect();
            let expected = try_parse_args_from(full).unwrap().invocation;
            assert_eq!(run("", &args).unwrap(), expected, "{args:?}");
            assert_eq!(run("# nothing yet\n", &args).unwrap(), expected, "{args:?}");
        }
    }

    #[test]
    fn test_scan_options_from_config() {
        let yaml = "
skip-changes: true
include-skipped-dirs: true
strip-emails: true
coverage-dir: cov
project-dir: ..
ignore-path: [a]
data:
  - usage.json
";
        let scan = run(yaml, &["scan"]).unwrap().scan;
        assert!(scan.skip_changes);
        assert!(scan.include_skipped_dirs);
        assert!(scan.strip_emails);
        assert_eq!(scan.coverage_dir, Some(path("cov")));
        assert_eq!(scan.project_dir, Some(path("..")));
        assert_eq!(scan.ignore_path, vec![path("a")]);
        assert_eq!(scan.data, vec![path("usage.json")]);
    }

    #[test]
    fn test_command_line_wins() {
        let yaml = "
path: src
coverage-dir: cov
serve:
  port: 4000
  experimental: false
";
        let invocation = run(
            yaml,
            &[
                "serve",
                "./web",
                "--port",
                "5000",
                "--coverage-dir",
                "other",
                "--experimental",
            ],
        )
        .unwrap();
        assert_eq!(
            invocation.action,
            Action::Serve(ServerArgs {
                path: path("./web"),
                port: 5000,
                allow_origin: vec![],
                experimental: true,
                project_name: None,
            })
        );
        assert_eq!(invocation.scan.coverage_dir, Some(path("other")));

        // Passing the default value explicitly still wins
        let invocation = run(yaml, &["serve", ".", "--port", "3000"]).unwrap();
        let Action::Serve(args) = invocation.action else {
            panic!()
        };
        assert_eq!(args.path, path("."));
        assert_eq!(args.port, 3000);
    }

    #[test]
    fn test_lists_are_combined() {
        let yaml = "
ignore-path: [a]
data: [usage.json]
serve:
  allow-origin: ['http://localhost:5173/']
";
        let invocation = run(
            yaml,
            &[
                "serve",
                "--ignore-path",
                "b",
                "--data",
                "errors.json",
                "--allow-origin",
                "*",
            ],
        )
        .unwrap();
        assert_eq!(invocation.scan.ignore_path, vec![path("a"), path("b")]);
        assert_eq!(
            invocation.scan.data,
            vec![path("usage.json"), path("errors.json")]
        );
        let Action::Serve(args) = invocation.action else {
            panic!()
        };
        // Normalized like on the command line
        assert_eq!(args.allow_origin, vec!["http://localhost:5173", "*"]);

        let err = run("serve:\n  allow-origin: [localhost]", &["serve"]).unwrap_err();
        assert!(err.to_string().contains("allow-origin"), "{err}");
    }

    #[test]
    fn test_path_applies_to_every_command() {
        let yaml = "path: src";
        for args in [
            vec!["scan"],
            vec!["serve"],
            vec!["build"],
            vec!["check"],
            vec!["codeowners"],
            vec!["owners"],
            vec!["records"],
            vec!["mcp"],
            vec!["new", "spec", "auth", "Login"],
            vec!["find-owner", "a.ts"],
            vec!["diff", "--since", "main"],
        ] {
            let invocation = run(yaml, &args).unwrap();
            assert_eq!(
                invocation.action.scan_path(),
                Some(Path::new("src")),
                "{args:?}"
            );
        }
    }

    #[test]
    fn test_command_sections() {
        let yaml = "
name: Acme
scan:
  json: true
  flat: true
  description: true
  coverage: true
build:
  build-dir: dashboard
  experimental: true
  generate-codeowners: true
find-owner:
  json: true
owners:
  json: true
records:
  json: true
aggregate:
  build-dir: out
  json: true
  data: [usage.json]
";
        let Action::List(args) = run(yaml, &["scan"]).unwrap().action else {
            panic!()
        };
        assert!(args.json && args.flat && args.description && args.coverage);

        let invocation = run(yaml, &["build"]).unwrap();
        assert!(invocation.generate_codeowners);
        let Action::Build(args) = invocation.action else {
            panic!()
        };
        assert_eq!(args.build_dir, path("dashboard"));
        assert!(args.experimental);
        assert_eq!(args.project_name.as_deref(), Some("Acme"));

        // The name on the command line wins
        let Action::Build(args) = run(yaml, &["build", "--project-name", "X"]).unwrap().action
        else {
            panic!()
        };
        assert_eq!(args.project_name.as_deref(), Some("X"));

        for args in [vec!["find-owner", "a.ts"], vec!["owners"], vec!["records"]] {
            let action = run(yaml, &args).unwrap().action;
            let json = match action {
                Action::FindOwner { json, .. }
                | Action::Owners { json, .. }
                | Action::Records { json, .. } => json,
                _ => panic!(),
            };
            assert!(json, "{args:?}");
        }

        let Action::Aggregate(args) = run(yaml, &["aggregate", "r"]).unwrap().action else {
            panic!()
        };
        assert_eq!(args.build_dir, path("out"));
        assert!(args.json);
        assert_eq!(args.data, vec![path("usage.json")]);

        // A section only applies to its command
        let Action::Owners { json, .. } = run("scan:\n  json: true", &["owners"]).unwrap().action
        else {
            panic!()
        };
        assert!(!json);
    }

    #[test]
    fn test_generate_codeowners() {
        let yaml = "
codeowners-path: .github/CODEOWNERS
codeowners-prefix: '@org/'
serve:
  generate-codeowners: true
check:
  generate-codeowners: true
";
        let expected = CodeownersOptions {
            codeowners_path: Some(path(".github/CODEOWNERS")),
            codeowners_prefix: "@org/".to_string(),
        };
        for args in [vec!["serve"], vec!["check"], vec!["codeowners"]] {
            let invocation = run(yaml, &args).unwrap();
            assert!(invocation.generate_codeowners, "{args:?}");
            assert_eq!(invocation.codeowners, expected, "{args:?}");
        }
        assert!(!run(yaml, &["build"]).unwrap().generate_codeowners);

        // The legacy form too
        assert!(
            run(yaml, &["./src", "--serve"])
                .unwrap()
                .generate_codeowners
        );

        let invocation = run(yaml, &["codeowners", "--codeowners-prefix", "@x/"]).unwrap();
        assert_eq!(invocation.codeowners.codeowners_prefix, "@x/");
    }

    #[test]
    fn test_check() {
        use crate::checker::{CheckArgs, OutputFormat, Severity};
        let yaml = "
check:
  generate-codeowners: true
  format: github
  rules:
    missing-owner: error
    min-coverage:
      severity: error
      threshold: 85
";
        let check_of = |invocation: Invocation| match invocation.action {
            Action::Check { json, check, .. } => (json, check),
            _ => panic!(),
        };

        let invocation = run(yaml, &["check"]).unwrap();
        assert!(invocation.generate_codeowners);
        let (json, check) = check_of(invocation);
        assert!(!json);
        assert_eq!(check.format, Some(OutputFormat::Github));
        let rules = check.resolve_config().unwrap();
        assert_eq!(rules.rule("missing-owner").severity, Severity::Error);
        assert_eq!(rules.rule("min-coverage").severity, Severity::Error);

        // The command line wins: format, and rule severities
        let (json, check) =
            check_of(run(yaml, &["check", "--json", "--rule", "missing-owner=off"]).unwrap());
        assert!(json);
        assert_eq!(check.format, None);
        let check = CheckArgs { ..check }.with_json(json);
        assert_eq!(check.output_format(), OutputFormat::Json);
        let rules = check.resolve_config().unwrap();
        assert_eq!(rules.rule("missing-owner").severity, Severity::Off);
        assert_eq!(rules.rule("min-coverage").severity, Severity::Error);

        // Unknown rules and invalid options are errors
        let err = run("check:\n  rules:\n    missing-tests: error", &["check"]).unwrap_err();
        assert!(
            format!("{err:#}").contains("unknown rule `missing-tests`"),
            "{err:#}"
        );
        let err = run(
            "check:\n  rules:\n    min-coverage:\n      threshold: 120",
            &["check"],
        )
        .unwrap_err();
        assert!(format!("{err:#}").contains("threshold"), "{err:#}");
        assert!(run("check:\n  format: xml", &["check"]).is_err());
    }

    #[test]
    fn test_diff() {
        let yaml = "
codeowners-prefix: '@org/'
diff:
  since: origin/main
  committed-only: true
  markdown: true
";
        let expected = DiffArgs {
            path: None,
            since: Some("origin/main".to_string()),
            committed_only: true,
            json: false,
            markdown: true,
            owners_only: false,
            codeowners_prefix: "@org/".to_string(),
        };
        assert_eq!(
            run(yaml, &["diff"]).unwrap().action,
            Action::Diff(expected.clone())
        );

        // A format on the command line replaces the file's
        assert_eq!(
            run(yaml, &["diff", "--since", "main", "--json"])
                .unwrap()
                .action,
            Action::Diff(DiffArgs {
                since: Some("main".to_string()),
                json: true,
                markdown: false,
                ..expected
            })
        );

        let err = run("diff:\n  json: true\n  markdown: true", &["diff"]).unwrap_err();
        assert!(err.to_string().contains("can't be combined"), "{err}");
    }

    #[test]
    fn test_invalid_config() {
        let err = FileConfig::parse("prot: 1").unwrap_err();
        assert!(err.to_string().contains("unknown field `prot`"), "{err}");
        let err = FileConfig::parse("serve:\n  port: x").unwrap_err();
        assert!(err.to_string().contains("invalid type"), "{err}");
        assert!(err.to_string().contains("line 2"), "{err}");
        assert!(FileConfig::parse("unknown:\n  x: 1").is_err());
        assert!(FileConfig::parse("- not a mapping").is_err());
        assert!(FileConfig::parse("serve: [").is_err());
    }

    fn parse_args(args: &[&str]) -> ParsedArgs {
        let args = std::iter::once("features").chain(args.iter().copied());
        try_parse_args_from(args).unwrap()
    }

    #[test]
    fn test_resolve_reads_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("src/web")).unwrap();
        fs::create_dir(root.join(".git")).unwrap();
        fs::write(
            root.join(CONFIG_FILE_NAME),
            "path: src\nignore-path: [src/legacy]\nserve:\n  port: 4000\n",
        )
        .unwrap();

        // From the folder of the file: paths stay relative
        let invocation = resolve(parse_args(&["serve"]), root).unwrap();
        let Action::Serve(args) = &invocation.action else {
            panic!()
        };
        assert_eq!(args.path, path("src"));
        assert_eq!(args.port, 4000);
        assert_eq!(invocation.scan.ignore_path, vec![path("src/legacy")]);

        // Scanning a subfolder: the file of a parent is found, up to the git root...
        let invocation = resolve(parse_args(&["serve", "src/web"]), root).unwrap();
        let Action::Serve(args) = &invocation.action else {
            panic!()
        };
        assert_eq!(args.path, path("src/web"));
        assert_eq!(args.port, 4000);

        // ...and its relative paths are relative to its folder
        let sub = root.join("src");
        let other = resolve(
            parse_args(&["serve", "--config", "../features.config.yaml"]),
            &sub,
        )
        .unwrap();
        let canonical_root = fs::canonicalize(root).unwrap();
        assert_eq!(
            other.scan.ignore_path,
            vec![canonical_root.join("src/legacy")]
        );

        // --no-config ignores it
        let invocation = resolve(parse_args(&["serve", "--no-config"]), root).unwrap();
        let Action::Serve(args) = &invocation.action else {
            panic!()
        };
        assert_eq!(args.port, crate::cli::DEFAULT_PORT);

        // A missing --config file is an error
        assert!(resolve(parse_args(&["serve", "--config", "missing.yaml"]), root).is_err());
    }

    #[test]
    fn test_only_one_file_name() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let port = |root: &Path| {
            let Action::Serve(args) = resolve(parse_args(&["serve"]), root).unwrap().action else {
                panic!()
            };
            args.port
        };

        // Neither the .yml extension nor the TOML file of earlier versions is read
        fs::write(root.join("features.config.yml"), "serve:\n  port: 4001").unwrap();
        fs::write(root.join("features.config.toml"), "[serve]\nport = 4003").unwrap();
        assert_eq!(port(root), crate::cli::DEFAULT_PORT);

        fs::write(root.join(CONFIG_FILE_NAME), "serve:\n  port: 4002").unwrap();
        assert_eq!(port(root), 4002);
    }

    #[test]
    fn test_resolve_checks_since() {
        let dir = tempfile::tempdir().unwrap();
        let err = resolve(parse_args(&["diff"]), dir.path()).unwrap_err();
        assert!(err.to_string().contains("--since"), "{err}");

        fs::write(dir.path().join(CONFIG_FILE_NAME), "diff:\n  since: main").unwrap();
        let invocation = resolve(parse_args(&["diff"]), dir.path()).unwrap();
        let Action::Diff(args) = invocation.action else {
            panic!()
        };
        assert_eq!(args.since.as_deref(), Some("main"));
    }

    #[test]
    fn test_invalid_file_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(CONFIG_FILE_NAME), "serve:\n  prot: 1").unwrap();
        let err = resolve(parse_args(&["serve"]), dir.path()).unwrap_err();
        let message = format!("{err:#}");
        assert!(message.contains("features.config.yaml"), "{message}");
        assert!(message.contains("unknown field `prot`"), "{message}");
    }
}
