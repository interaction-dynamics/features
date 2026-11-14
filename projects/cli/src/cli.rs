//! Command line interface.
//!
//! Each action is a subcommand (`features serve ./src`). The flag forms used
//! before subcommands existed (`features ./src --serve`) are still accepted as
//! deprecated aliases: both forms are parsed into the same [`Invocation`].
//!
//! Options not given on the command line can come from the configuration file
//! (`.features.yml`), see [`Invocation::apply_config`].

use anyhow::{Context, bail};
use clap::error::ErrorKind;
use clap::parser::ValueSource;
use clap::{ArgGroup, ArgMatches, Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::checker::{CheckArgs, OutputFormat};
use crate::config::ProjectConfig;
use crate::http_server::parse_allowed_origin;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_BUILD_DIR: &str = "build";
const DEFAULT_CODEOWNERS_PREFIX: &str = "@";
const DEPRECATED_HEADING: &str = "Deprecated actions (use the commands above)";

/// A CLI tool for discovering features in a folder by reading README.md or README.mdx files,
/// and serving them via HTTP or static builds.
#[derive(Parser, Debug)]
#[command(name = "features")]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(group(ArgGroup::new("legacy_action").multiple(false)))]
#[command(after_help = "\
Before subcommands existed, actions were flags: `features <PATH> --serve`, \
`--build`, `--check`, `--list-owners`, `--find-owner <TARGET>` and \
`--generate-codeowners`. These still work but are deprecated.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Print version information
    #[arg(short = 'V', long)]
    pub version: bool,

    #[command(flatten)]
    pub legacy: LegacyArgs,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List the features of a folder (the default action)
    Scan {
        #[command(flatten)]
        output: ListArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Start an HTTP server with the web dashboard, refreshed when files change
    Serve {
        #[command(flatten)]
        server: ServerArgs,
        #[command(flatten)]
        codeowners: OptionalCodeownersArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Create a static build of the web dashboard
    Build {
        #[command(flatten)]
        build: BuildArgs,
        #[command(flatten)]
        codeowners: OptionalCodeownersArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Generate or update a CODEOWNERS file from feature ownership
    Codeowners {
        #[command(flatten)]
        codeowners: CodeownersArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Find the owner of a file or folder
    FindOwner {
        /// The file or folder to find the owner of
        target: PathBuf,

        /// The folder to scan for features (default: current directory)
        path: Option<PathBuf>,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        #[command(flatten)]
        scan: ScanArgs,
    },

    /// List the unique owners of the features
    Owners {
        /// The folder to scan for features
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,

        #[command(flatten)]
        scan: ScanArgs,
    },

    /// List the features touched since a git ref, and their owners
    Diff {
        #[command(flatten)]
        diff: DiffArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Run checks on the features (duplicate names, missing owners, stale flags...).
    /// Rules are configured in the `check` section of .features.yml or with the options below
    Check {
        /// The folder to scan for features
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output the findings as JSON (same as --format json)
        #[arg(long)]
        json: bool,

        #[command(flatten)]
        check: CheckArgs,
        #[command(flatten)]
        codeowners: OptionalCodeownersArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },
}

/// Options for how features are scanned, shared by every action
#[derive(Args, Debug, Clone, Default, PartialEq)]
pub struct ScanArgs {
    /// Skip computing changes (git commits) in the output
    #[arg(long)]
    pub skip_changes: bool,

    /// Ignore a folder and all its subfolders when scanning for features (can be repeated)
    #[arg(long)]
    pub ignore_path: Vec<PathBuf>,

    /// Also scan folders skipped by default (node_modules) for features, stats and imports
    #[arg(long)]
    pub include_skipped_dirs: bool,

    /// Path to the coverage directory (overrides default search)
    #[arg(long)]
    pub coverage_dir: Option<PathBuf>,

    /// Project directory, where CODEOWNERS is written and coverage is also searched
    #[arg(long)]
    pub project_dir: Option<PathBuf>,

    /// Remove commit author emails from the output (JSON, web dashboard and static build)
    #[arg(long)]
    pub strip_emails: bool,

    /// Path to the configuration file [default: .features.yml or .features.yaml in the
    /// scanned folder, the --project-dir folder or the current folder]
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct ListArgs {
    /// The folder to scan for features
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output features as JSON
    #[arg(long)]
    pub json: bool,

    /// Output features as a flat array instead of nested structure
    #[arg(long)]
    pub flat: bool,

    /// Include descriptions in the output
    #[arg(long)]
    pub description: bool,

    /// Include coverage information in the output (always on with --json)
    #[arg(long)]
    pub coverage: bool,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct ServerArgs {
    /// The folder to scan for features
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Port for the HTTP server
    #[arg(long, default_value_t = DEFAULT_PORT)]
    pub port: u16,

    /// Allow another origin (e.g. http://localhost:5173) to read the HTTP server's
    /// responses via CORS. Can be repeated. Use '*' to allow any origin.
    #[arg(long, value_name = "ORIGIN", value_parser = parse_allowed_origin)]
    pub allow_origin: Vec<String>,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct BuildArgs {
    /// The folder to scan for features
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output directory for the static build
    #[arg(long, default_value = DEFAULT_BUILD_DIR)]
    pub build_dir: PathBuf,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct DiffArgs {
    /// The folder to scan for features (default: current directory)
    pub path: Option<PathBuf>,

    /// List the features touched since the merge base of this git ref and HEAD,
    /// including uncommitted changes (e.g. --since origin/master). Required, here
    /// or as `diff.since` in the configuration file
    #[arg(long, value_name = "REF")]
    pub since: Option<String>,

    /// Only compare committed changes, ignoring the working tree
    #[arg(long)]
    pub committed_only: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Output a markdown table suited to a pull request comment
    #[arg(long, conflicts_with = "json")]
    pub markdown: bool,

    /// Print only the unique owners of the touched features, one per line,
    /// with --codeowners-prefix added
    #[arg(long, conflicts_with_all = ["json", "markdown"])]
    pub owners_only: bool,

    /// Prefix added to owner names
    #[arg(long, default_value = DEFAULT_CODEOWNERS_PREFIX)]
    pub codeowners_prefix: String,
}

/// Where and how the CODEOWNERS file is written
#[derive(Args, Debug, Clone, PartialEq)]
pub struct CodeownersOptions {
    /// Custom path and filename for the CODEOWNERS file (default: CODEOWNERS in the project directory)
    #[arg(long)]
    pub codeowners_path: Option<PathBuf>,

    /// Custom prefix for owner names in the CODEOWNERS file
    #[arg(long, default_value = DEFAULT_CODEOWNERS_PREFIX)]
    pub codeowners_prefix: String,
}

impl Default for CodeownersOptions {
    fn default() -> Self {
        Self {
            codeowners_path: None,
            codeowners_prefix: DEFAULT_CODEOWNERS_PREFIX.to_string(),
        }
    }
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct CodeownersArgs {
    /// The folder to scan for features
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[command(flatten)]
    pub options: CodeownersOptions,
}

/// Also generate CODEOWNERS alongside another action
#[derive(Args, Debug, Clone, PartialEq)]
pub struct OptionalCodeownersArgs {
    /// Also generate or update a CODEOWNERS file
    #[arg(long)]
    pub generate_codeowners: bool,

    #[command(flatten)]
    pub options: CodeownersOptions,
}

/// The flags of the command line from before subcommands existed
#[derive(Args, Debug, Clone)]
pub struct LegacyArgs {
    /// The path to the directory to list
    pub path: Option<PathBuf>,

    /// Output features as JSON
    #[arg(long)]
    pub json: bool,

    /// Output features as a flat array instead of nested structure
    #[arg(long)]
    pub flat: bool,

    /// Include descriptions in the output
    #[arg(long)]
    pub description: bool,

    /// Include coverage information in the output
    #[arg(long)]
    pub coverage: bool,

    /// Deprecated: use `features owners`
    #[arg(long, group = "legacy_action", help_heading = DEPRECATED_HEADING)]
    pub list_owners: bool,

    /// Deprecated: use `features check`
    #[arg(long, group = "legacy_action", help_heading = DEPRECATED_HEADING)]
    pub check: bool,

    #[command(flatten)]
    pub check_args: CheckArgs,

    /// Deprecated: use `features serve`
    #[arg(long, group = "legacy_action", help_heading = DEPRECATED_HEADING)]
    pub serve: bool,

    /// Port for the HTTP server
    #[arg(long, default_value_t = DEFAULT_PORT)]
    pub port: u16,

    /// Allow another origin (e.g. http://localhost:5173) to read the HTTP server's
    /// responses via CORS. Can be repeated. Use '*' to allow any origin.
    #[arg(long, value_name = "ORIGIN", value_parser = parse_allowed_origin)]
    pub allow_origin: Vec<String>,

    /// Deprecated: use `features build`
    #[arg(long, group = "legacy_action", help_heading = DEPRECATED_HEADING)]
    pub build: bool,

    /// Output directory for the static build
    #[arg(long, default_value = DEFAULT_BUILD_DIR)]
    pub build_dir: PathBuf,

    /// Deprecated: use `features find-owner`
    #[arg(long, group = "legacy_action", help_heading = DEPRECATED_HEADING, value_name = "TARGET")]
    pub find_owner: Option<PathBuf>,

    /// Deprecated: use `features codeowners`, or `--generate-codeowners` on `serve`,
    /// `build` and `check`
    #[arg(long, conflicts_with = "find_owner", help_heading = DEPRECATED_HEADING)]
    pub generate_codeowners: bool,

    #[command(flatten)]
    pub codeowners: CodeownersOptions,

    /// Diff mode: list the features touched since the merge base of this git ref and HEAD,
    /// including uncommitted changes (e.g. --since origin/master). Same as `features diff`
    #[arg(
        long,
        value_name = "REF",
        group = "legacy_action",
        conflicts_with = "generate_codeowners"
    )]
    pub since: Option<String>,

    /// With --since: only compare committed changes, ignoring the working tree
    #[arg(long, requires = "since")]
    pub committed_only: bool,

    /// With --since: output a markdown table suited to a pull request comment
    #[arg(long, requires = "since", conflicts_with = "json")]
    pub markdown: bool,

    /// With --since: print only the unique owners of the touched features, one per line,
    /// with --codeowners-prefix added
    #[arg(long, requires = "since", conflicts_with_all = ["json", "markdown"])]
    pub owners_only: bool,

    #[command(flatten)]
    pub scan: ScanArgs,
}

/// What to do, whichever command line form asked for it
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Print the version
    Version,
    /// List the features
    List(ListArgs),
    /// Serve the web dashboard
    Serve(ServerArgs),
    /// Create a static build of the web dashboard
    Build(BuildArgs),
    /// Only generate CODEOWNERS
    Codeowners { path: PathBuf },
    /// Find the owner of `target`, scanning `path` (or the current directory)
    FindOwner {
        target: PathBuf,
        path: Option<PathBuf>,
        json: bool,
    },
    /// List the unique owners
    Owners { path: PathBuf, json: bool },
    /// List the features touched since a git ref
    Diff(DiffArgs),
    /// Run checks on the features
    Check { path: PathBuf, check: CheckArgs },
}

/// A parsed command line
#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
    pub action: Action,
    pub scan: ScanArgs,
    /// Generate CODEOWNERS too (always on for [`Action::Codeowners`])
    pub generate_codeowners: bool,
    /// Where and how CODEOWNERS is written
    pub codeowners: CodeownersOptions,
}

/// The ids of the arguments given on the command line, as opposed to default values
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExplicitArgs(HashSet<String>);

impl ExplicitArgs {
    fn from_matches(matches: &ArgMatches) -> Self {
        let mut ids = HashSet::new();
        let mut current = Some(matches);
        while let Some(matches) = current {
            for id in matches.ids() {
                if matches.value_source(id.as_str()) == Some(ValueSource::CommandLine) {
                    ids.insert(id.as_str().to_string());
                }
            }
            current = matches.subcommand().map(|(_, sub)| sub);
        }
        Self(ids)
    }

    /// Whether the argument with this id (the field name) was given on the command line
    pub fn contains(&self, id: &str) -> bool {
        self.0.contains(id)
    }
}

impl Cli {
    /// Parses a command line into what to do, and which arguments it gave explicitly
    pub fn try_parse_invocation<I, T>(args: I) -> Result<(Invocation, ExplicitArgs), clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = Cli::command().try_get_matches_from(args)?;
        let explicit = ExplicitArgs::from_matches(&matches);
        let invocation = Cli::from_arg_matches(&matches)?.into_invocation()?;
        Ok((invocation, explicit))
    }

    /// Turn the command line, in either form, into what to do
    pub fn into_invocation(self) -> Result<Invocation, clap::Error> {
        if self.version {
            return Ok(Invocation {
                action: Action::Version,
                scan: ScanArgs::default(),
                generate_codeowners: false,
                codeowners: CodeownersOptions::default(),
            });
        }

        let invocation = match self.command {
            Some(command) => command.into_invocation(),
            None => self.legacy.into_invocation()?,
        };
        Ok(invocation)
    }
}

impl Command {
    fn into_invocation(self) -> Invocation {
        let none = OptionalCodeownersArgs {
            generate_codeowners: false,
            options: CodeownersOptions::default(),
        };
        let (action, scan, codeowners) = match self {
            Command::Scan { output, scan } => (Action::List(output), scan, none),
            Command::Serve {
                server,
                codeowners,
                scan,
            } => (Action::Serve(server), scan, codeowners),
            Command::Build {
                build,
                codeowners,
                scan,
            } => (Action::Build(build), scan, codeowners),
            Command::Codeowners { codeowners, scan } => (
                Action::Codeowners {
                    path: codeowners.path,
                },
                scan,
                OptionalCodeownersArgs {
                    generate_codeowners: true,
                    options: codeowners.options,
                },
            ),
            Command::FindOwner {
                target,
                path,
                json,
                scan,
            } => (Action::FindOwner { target, path, json }, scan, none),
            Command::Owners { path, json, scan } => (Action::Owners { path, json }, scan, none),
            Command::Diff { diff, scan } => (Action::Diff(diff), scan, none),
            Command::Check {
                path,
                json,
                check,
                codeowners,
                scan,
            } => (
                Action::Check {
                    path,
                    check: check.with_json(json),
                },
                scan,
                codeowners,
            ),
        };

        Invocation {
            action,
            scan,
            generate_codeowners: codeowners.generate_codeowners,
            codeowners: codeowners.options,
        }
    }
}

impl LegacyArgs {
    fn into_invocation(self) -> Result<Invocation, clap::Error> {
        if let Some(since) = self.since {
            return Ok(Invocation {
                action: Action::Diff(DiffArgs {
                    path: self.path,
                    since: Some(since),
                    committed_only: self.committed_only,
                    json: self.json,
                    markdown: self.markdown,
                    owners_only: self.owners_only,
                    codeowners_prefix: self.codeowners.codeowners_prefix.clone(),
                }),
                scan: self.scan,
                generate_codeowners: false,
                codeowners: self.codeowners,
            });
        }

        if self.check_args.is_used() && !self.check {
            return Err(Cli::command().error(
                ErrorKind::ArgumentConflict,
                "--rule, --enable-rule, --disable-rule, --format and --list-rules \
                 can only be used with `features check` (or the deprecated --check)",
            ));
        }

        if let Some(target) = self.find_owner {
            return Ok(Invocation {
                action: Action::FindOwner {
                    target,
                    path: self.path,
                    json: self.json,
                },
                scan: self.scan,
                generate_codeowners: false,
                codeowners: self.codeowners,
            });
        }

        let Some(path) = self.path else {
            return Err(Cli::command().error(
                ErrorKind::MissingRequiredArgument,
                "the path argument is required, e.g. `features ./src`",
            ));
        };

        let action = if self.serve {
            Action::Serve(ServerArgs {
                path,
                port: self.port,
                allow_origin: self.allow_origin,
            })
        } else if self.build {
            Action::Build(BuildArgs {
                path,
                build_dir: self.build_dir,
            })
        } else if self.check {
            Action::Check {
                path,
                check: self.check_args.with_json(self.json),
            }
        } else if self.list_owners {
            Action::Owners {
                path,
                json: self.json,
            }
        } else if self.generate_codeowners {
            Action::Codeowners { path }
        } else {
            Action::List(ListArgs {
                path,
                json: self.json,
                flat: self.flat,
                description: self.description,
                coverage: self.coverage,
            })
        };

        Ok(Invocation {
            action,
            scan: self.scan,
            generate_codeowners: self.generate_codeowners,
            codeowners: self.codeowners,
        })
    }
}

/// Replaces `target` with the configured value, unless the argument was given on the command line
fn fill<T: Clone>(target: &mut T, configured: &Option<T>, given: bool) {
    if let (false, Some(value)) = (given, configured) {
        *target = value.clone();
    }
}

/// Fills an optional argument that wasn't given with the configured value
fn fill_option<T: Clone>(target: &mut Option<T>, configured: &Option<T>) {
    if target.is_none() {
        *target = configured.clone();
    }
}

impl Invocation {
    /// The folder the action scans, if it names one
    pub fn scan_path(&self) -> Option<&Path> {
        match &self.action {
            Action::Version => None,
            Action::List(args) => Some(&args.path),
            Action::Serve(args) => Some(&args.path),
            Action::Build(args) => Some(&args.path),
            Action::Codeowners { path }
            | Action::Owners { path, .. }
            | Action::Check { path, .. } => Some(path),
            Action::FindOwner { path, .. } => path.as_deref(),
            Action::Diff(args) => args.path.as_deref(),
        }
    }

    /// Fills the options that weren't given on the command line from the configuration file.
    ///
    /// The command line wins. Lists (`ignore_path`, `serve.allow_origin`) add up.
    /// The rules of the checks are applied later, by the checker.
    pub fn apply_config(
        &mut self,
        config: &ProjectConfig,
        explicit: &ExplicitArgs,
    ) -> anyhow::Result<()> {
        let given = |id: &str| explicit.contains(id);
        let Invocation {
            action,
            scan,
            generate_codeowners,
            codeowners,
        } = self;

        fill(
            &mut scan.skip_changes,
            &config.skip_changes,
            given("skip_changes"),
        );
        scan.ignore_path.extend(config.ignore_path.iter().cloned());
        fill(
            &mut scan.include_skipped_dirs,
            &config.include_skipped_dirs,
            given("include_skipped_dirs"),
        );
        fill_option(&mut scan.coverage_dir, &config.coverage_dir);
        fill_option(&mut scan.project_dir, &config.project_dir);
        fill(
            &mut scan.strip_emails,
            &config.strip_emails,
            given("strip_emails"),
        );

        fill_option(&mut codeowners.codeowners_path, &config.codeowners_path);
        fill(
            &mut codeowners.codeowners_prefix,
            &config.codeowners_prefix,
            given("codeowners_prefix"),
        );

        let fill_path = |path: &mut PathBuf| fill(path, &config.path, given("path"));
        let mut fill_generate = |configured: &Option<bool>| {
            fill(
                generate_codeowners,
                configured,
                given("generate_codeowners"),
            )
        };

        match action {
            Action::Version => {}
            Action::List(args) => {
                let section = &config.scan;
                fill_path(&mut args.path);
                fill(&mut args.json, &section.json, given("json"));
                fill(&mut args.flat, &section.flat, given("flat"));
                fill(
                    &mut args.description,
                    &section.description,
                    given("description"),
                );
                fill(&mut args.coverage, &section.coverage, given("coverage"));
            }
            Action::Serve(args) => {
                let section = &config.serve;
                fill_path(&mut args.path);
                fill(&mut args.port, &section.port, given("port"));
                for origin in &section.allow_origin {
                    let origin = parse_allowed_origin(origin).map_err(|e| {
                        anyhow::anyhow!("invalid `serve.allow_origin` value `{origin}`: {e}")
                    })?;
                    args.allow_origin.push(origin);
                }
                fill_generate(&section.generate_codeowners);
            }
            Action::Build(args) => {
                let section = &config.build;
                fill_path(&mut args.path);
                fill(&mut args.build_dir, &section.build_dir, given("build_dir"));
                fill_generate(&section.generate_codeowners);
            }
            Action::Codeowners { path } => fill_path(path),
            Action::FindOwner { path, json, .. } => {
                fill_option(path, &config.path);
                fill(json, &config.find_owner.json, given("json"));
            }
            Action::Owners { path, json } => {
                fill_path(path);
                fill(json, &config.owners.json, given("json"));
            }
            Action::Diff(args) => {
                let section = &config.diff;
                fill_option(&mut args.path, &config.path);
                fill_option(&mut args.since, &section.since);
                fill(
                    &mut args.committed_only,
                    &section.committed_only,
                    given("committed_only"),
                );
                // The output format is all or nothing: the command line one replaces the file's
                if !["json", "markdown", "owners_only"]
                    .iter()
                    .any(|id| given(id))
                {
                    let formats = [section.json, section.markdown, section.owners_only];
                    if formats.iter().filter(|f| **f == Some(true)).count() > 1 {
                        bail!(
                            "`diff.json`, `diff.markdown` and `diff.owners_only` can't be combined"
                        );
                    }
                    fill(&mut args.json, &section.json, false);
                    fill(&mut args.markdown, &section.markdown, false);
                    fill(&mut args.owners_only, &section.owners_only, false);
                }
                fill(
                    &mut args.codeowners_prefix,
                    &config.codeowners_prefix,
                    given("codeowners_prefix"),
                );
            }
            Action::Check { path, check } => {
                let section = &config.check;
                fill_path(path);
                // --json and --format both set the format, which wins over the file's
                if check.format.is_none() {
                    check.format = section
                        .format
                        .or_else(|| (section.json == Some(true)).then_some(OutputFormat::Json));
                }
                fill_generate(&section.generate_codeowners);
            }
        }

        Ok(())
    }
}

impl DiffArgs {
    /// The git ref to compare with
    pub fn since(&self) -> anyhow::Result<&str> {
        self.since.as_deref().context(
            "`features diff` needs a git ref: pass --since <REF> or set `diff.since` in .features.yml",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Invocation, clap::Error> {
        let args = std::iter::once("features").chain(args.iter().copied());
        Cli::try_parse_from(args)?.into_invocation()
    }

    fn action(args: &[&str]) -> Action {
        parse(args).unwrap().action
    }

    fn path(p: &str) -> PathBuf {
        PathBuf::from(p)
    }

    fn list(p: &str) -> ListArgs {
        ListArgs {
            path: path(p),
            json: false,
            flat: false,
            description: false,
            coverage: false,
        }
    }

    fn serve(p: &str) -> ServerArgs {
        ServerArgs {
            path: path(p),
            port: DEFAULT_PORT,
            allow_origin: Vec::new(),
        }
    }

    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_no_arguments_shows_help() {
        let err = parse(&[]).unwrap_err();
        assert_eq!(
            err.kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }

    #[test]
    fn test_version() {
        assert_eq!(action(&["--version"]), Action::Version);
        assert_eq!(action(&["-V"]), Action::Version);
    }

    #[test]
    fn test_scan() {
        assert_eq!(action(&["./src"]), Action::List(list("./src")));
        assert_eq!(action(&["scan", "./src"]), Action::List(list("./src")));
        assert_eq!(action(&["scan"]), Action::List(list(".")));

        let expected = Action::List(ListArgs {
            json: true,
            flat: true,
            ..list("./src")
        });
        assert_eq!(action(&["./src", "--json", "--flat"]), expected);
        assert_eq!(action(&["scan", "./src", "--json", "--flat"]), expected);
    }

    #[test]
    fn test_serve() {
        assert_eq!(action(&["./src", "--serve"]), Action::Serve(serve("./src")));
        assert_eq!(action(&["serve", "./src"]), Action::Serve(serve("./src")));

        let expected = Action::Serve(ServerArgs {
            port: 4000,
            allow_origin: vec!["http://localhost:5173".to_string()],
            ..serve("./src")
        });
        let flags = ["--port", "4000", "--allow-origin", "http://localhost:5173/"];
        let legacy: Vec<&str> = ["./src", "--serve"].into_iter().chain(flags).collect();
        let subcommand: Vec<&str> = ["serve", "./src"].into_iter().chain(flags).collect();
        assert_eq!(action(&legacy), expected);
        assert_eq!(action(&subcommand), expected);

        assert!(parse(&["serve", "--allow-origin", "localhost"]).is_err());
    }

    #[test]
    fn test_build() {
        let expected = Action::Build(BuildArgs {
            path: path("./src"),
            build_dir: path("out"),
        });
        assert_eq!(
            action(&["./src", "--build", "--build-dir", "out"]),
            expected
        );
        assert_eq!(action(&["build", "./src", "--build-dir", "out"]), expected);
    }

    #[test]
    fn test_codeowners() {
        let expected = Invocation {
            action: Action::Codeowners {
                path: path("./src"),
            },
            scan: ScanArgs {
                project_dir: Some(path("..")),
                ..ScanArgs::default()
            },
            generate_codeowners: true,
            codeowners: CodeownersOptions {
                codeowners_path: Some(path(".github/CODEOWNERS")),
                codeowners_prefix: "@org/".to_string(),
            },
        };
        let flags = [
            "--project-dir",
            "..",
            "--codeowners-path",
            ".github/CODEOWNERS",
            "--codeowners-prefix",
            "@org/",
        ];
        let legacy: Vec<&str> = ["./src", "--generate-codeowners"]
            .into_iter()
            .chain(flags)
            .collect();
        let subcommand: Vec<&str> = ["codeowners", "./src"].into_iter().chain(flags).collect();
        assert_eq!(parse(&legacy).unwrap(), expected);
        assert_eq!(parse(&subcommand).unwrap(), expected);
    }

    #[test]
    fn test_codeowners_combines_with_serve_build_and_check() {
        for args in [
            vec!["./src", "--serve", "--generate-codeowners"],
            vec!["serve", "./src", "--generate-codeowners"],
            vec!["./src", "--build", "--generate-codeowners"],
            vec!["build", "./src", "--generate-codeowners"],
            vec!["./src", "--check", "--generate-codeowners"],
            vec!["check", "./src", "--generate-codeowners"],
        ] {
            let invocation = parse(&args).unwrap();
            assert!(invocation.generate_codeowners, "{args:?}");
            assert_eq!(invocation.codeowners, CodeownersOptions::default());
        }

        assert!(!parse(&["serve", "./src"]).unwrap().generate_codeowners);
    }

    #[test]
    fn test_find_owner() {
        let expected = Action::FindOwner {
            target: path("src/a.ts"),
            path: Some(path("./src")),
            json: true,
        };
        assert_eq!(
            action(&["./src", "--find-owner", "src/a.ts", "--json"]),
            expected
        );
        assert_eq!(
            action(&["find-owner", "src/a.ts", "./src", "--json"]),
            expected
        );

        // The folder to scan defaults to the current directory
        let expected = Action::FindOwner {
            target: path("src/a.ts"),
            path: None,
            json: false,
        };
        assert_eq!(action(&["--find-owner", "src/a.ts"]), expected);
        assert_eq!(action(&["find-owner", "src/a.ts"]), expected);
    }

    #[test]
    fn test_owners_and_check() {
        let owners = Action::Owners {
            path: path("./src"),
            json: true,
        };
        assert_eq!(action(&["./src", "--list-owners", "--json"]), owners);
        assert_eq!(action(&["owners", "./src", "--json"]), owners);

        let check = Action::Check {
            path: path("./src"),
            check: CheckArgs::default(),
        };
        assert_eq!(action(&["./src", "--check"]), check);
        assert_eq!(action(&["check", "./src"]), check);
    }

    #[test]
    fn test_check_options() {
        use crate::checker::{OutputFormat, Severity};

        let expected = Action::Check {
            path: path("./src"),
            check: CheckArgs {
                rules: vec![("min-coverage".to_string(), Severity::Error)],
                enable_rule: vec!["missing-readme".to_string()],
                disable_rule: vec!["stale-flag".to_string()],
                format: Some(OutputFormat::Github),
                list_rules: false,
            },
        };
        let flags = [
            "--rule",
            "min-coverage=error",
            "--enable-rule",
            "missing-readme",
            "--disable-rule",
            "stale-flag",
            "--format",
            "github",
        ];
        for prefix in [vec!["./src", "--check"], vec!["check", "./src"]] {
            let args: Vec<&str> = prefix.into_iter().chain(flags).collect();
            assert_eq!(action(&args), expected, "{args:?}");
        }
    }

    #[test]
    fn test_check_json_selects_json_format() {
        for args in [
            vec!["./src", "--check", "--json"],
            vec!["check", "./src", "--json"],
        ] {
            let Action::Check { check, .. } = action(&args) else {
                panic!("expected a check action for {args:?}");
            };
            assert_eq!(check.output_format(), crate::checker::OutputFormat::Json);
        }

        // --format wins over --json
        let Action::Check { check, .. } =
            action(&["check", "./src", "--json", "--format", "github"])
        else {
            panic!("expected a check action");
        };
        assert_eq!(check.output_format(), crate::checker::OutputFormat::Github);
    }

    #[test]
    fn test_check_options_require_check() {
        let err = parse(&["./src", "--disable-rule", "stale-flag"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
        assert!(parse(&["serve", "./src", "--rule", "stale-flag=off"]).is_err());
    }

    #[test]
    fn test_scan_options() {
        let expected = ScanArgs {
            skip_changes: true,
            ignore_path: vec![path("a"), path("b")],
            include_skipped_dirs: true,
            coverage_dir: Some(path("cov")),
            project_dir: None,
            strip_emails: true,
            config: Some(path("features.yml")),
        };
        let flags = [
            "--skip-changes",
            "--ignore-path",
            "a",
            "--ignore-path",
            "b",
            "--include-skipped-dirs",
            "--coverage-dir",
            "cov",
            "--strip-emails",
            "--config",
            "features.yml",
        ];
        for prefix in [vec!["./src", "--serve"], vec!["serve", "./src"]] {
            let args: Vec<&str> = prefix.into_iter().chain(flags).collect();
            assert_eq!(parse(&args).unwrap().scan, expected);
        }
    }

    #[test]
    fn test_conflicting_actions_are_errors() {
        for args in [
            vec!["./src", "--serve", "--build"],
            vec!["./src", "--serve", "--check"],
            vec!["./src", "--build", "--list-owners"],
            vec!["./src", "--check", "--find-owner", "a"],
            vec!["./src", "--find-owner", "a", "--generate-codeowners"],
        ] {
            let err = parse(&args).unwrap_err();
            assert_eq!(err.kind(), ErrorKind::ArgumentConflict, "{args:?}");
        }

        // Legacy flags can't be mixed with a subcommand
        assert!(parse(&["--serve", "build", "./src"]).is_err());
        assert!(parse(&["build", "./src", "--serve"]).is_err());
    }

    #[test]
    fn test_diff() {
        let expected = Action::Diff(DiffArgs {
            path: Some(path("./src")),
            since: Some("origin/master".to_string()),
            committed_only: true,
            json: false,
            markdown: false,
            owners_only: true,
            codeowners_prefix: "@org/".to_string(),
        });
        let flags = [
            "--since",
            "origin/master",
            "--committed-only",
            "--owners-only",
            "--codeowners-prefix",
            "@org/",
        ];
        let legacy: Vec<&str> = ["./src"].into_iter().chain(flags).collect();
        let subcommand: Vec<&str> = ["diff", "./src"].into_iter().chain(flags).collect();
        assert_eq!(action(&legacy), expected);
        assert_eq!(action(&subcommand), expected);

        // The folder to scan defaults to the current directory
        let expected = Action::Diff(DiffArgs {
            path: None,
            since: Some("main".to_string()),
            committed_only: false,
            json: true,
            markdown: false,
            owners_only: false,
            codeowners_prefix: DEFAULT_CODEOWNERS_PREFIX.to_string(),
        });
        assert_eq!(action(&["--since", "main", "--json"]), expected);
        assert_eq!(action(&["diff", "--since", "main", "--json"]), expected);

        // `features diff` needs a ref, from the command line or the config file
        let Action::Diff(args) = action(&["diff", "./src"]) else {
            panic!("expected a diff action");
        };
        assert!(args.since().is_err());
    }

    #[test]
    fn test_diff_conflicts() {
        for args in [
            vec!["./src", "--since", "main", "--serve"],
            vec!["./src", "--since", "main", "--build"],
            vec!["./src", "--since", "main", "--check"],
            vec!["./src", "--since", "main", "--list-owners"],
            vec!["./src", "--since", "main", "--generate-codeowners"],
            vec!["./src", "--since", "main", "--json", "--markdown"],
            vec!["diff", "--since", "main", "--json", "--markdown"],
            vec!["diff", "--since", "main", "--markdown", "--owners-only"],
        ] {
            let err = parse(&args).unwrap_err();
            assert_eq!(err.kind(), ErrorKind::ArgumentConflict, "{args:?}");
        }

        // Diff options need --since
        for flag in ["--committed-only", "--markdown", "--owners-only"] {
            assert!(parse(&["./src", flag]).is_err(), "{flag}");
        }
    }

    fn parse_with_config(args: &[&str], config: &str) -> anyhow::Result<Invocation> {
        let args = std::iter::once("features").chain(args.iter().copied());
        let (mut invocation, explicit) = Cli::try_parse_invocation(args)?;
        invocation.apply_config(&ProjectConfig::parse(config)?, &explicit)?;
        Ok(invocation)
    }

    #[test]
    fn test_config_fills_scan_options() {
        let config = "skip_changes: true\nignore_path: [vendor]\ncoverage_dir: cov\n\
                      project_dir: ..\nstrip_emails: true\ninclude_skipped_dirs: true\n";
        let invocation =
            parse_with_config(&["serve", "./src", "--ignore-path", "gen"], config).unwrap();
        assert_eq!(
            invocation.scan,
            ScanArgs {
                skip_changes: true,
                ignore_path: vec![path("gen"), path("vendor")],
                include_skipped_dirs: true,
                coverage_dir: Some(path("cov")),
                project_dir: Some(path("..")),
                strip_emails: true,
                config: None,
            }
        );

        // The command line wins
        let invocation =
            parse_with_config(&["scan", "--coverage-dir", "other"], "coverage_dir: cov\n").unwrap();
        assert_eq!(invocation.scan.coverage_dir, Some(path("other")));

        // A false in the file doesn't undo a flag
        let invocation =
            parse_with_config(&["scan", "--skip-changes"], "skip_changes: false\n").unwrap();
        assert!(invocation.scan.skip_changes);
    }

    #[test]
    fn test_config_fills_the_path_unless_given() {
        assert_eq!(
            parse_with_config(&["scan"], "path: src\n").unwrap().action,
            Action::List(list("src"))
        );
        // Even when the command line gives the default value
        assert_eq!(
            parse_with_config(&["scan", "."], "path: src\n")
                .unwrap()
                .action,
            Action::List(list("."))
        );
        assert_eq!(
            parse_with_config(&["find-owner", "a.ts"], "path: src\n")
                .unwrap()
                .action,
            Action::FindOwner {
                target: path("a.ts"),
                path: Some(path("src")),
                json: false,
            }
        );
    }

    #[test]
    fn test_config_fills_command_options() {
        let config = r#"
scan: { json: true, flat: true }
serve: { port: 4000, allow_origin: ["http://localhost:5173/"], generate_codeowners: true }
build: { build_dir: dist }
owners: { json: true }
find_owner: { json: true }
codeowners_prefix: "@org/"
"#;
        assert_eq!(
            parse_with_config(&["scan", "src"], config).unwrap().action,
            Action::List(ListArgs {
                json: true,
                flat: true,
                ..list("src")
            })
        );

        let served = parse_with_config(&["serve", "--port", "5000"], config).unwrap();
        assert_eq!(
            served.action,
            Action::Serve(ServerArgs {
                port: 5000,
                allow_origin: vec!["http://localhost:5173".to_string()],
                ..serve(".")
            })
        );
        assert!(served.generate_codeowners);
        assert_eq!(served.codeowners.codeowners_prefix, "@org/");

        assert_eq!(
            parse_with_config(&["./src", "--build"], config)
                .unwrap()
                .action,
            Action::Build(BuildArgs {
                path: path("./src"),
                build_dir: path("dist"),
            })
        );
        assert_eq!(
            parse_with_config(&["owners"], config).unwrap().action,
            Action::Owners {
                path: path("."),
                json: true
            }
        );

        let err = parse_with_config(&["serve"], "serve: { allow_origin: [localhost] }\n");
        assert!(format!("{:#}", err.unwrap_err()).contains("serve.allow_origin"));
    }

    #[test]
    fn test_config_fills_diff_options() {
        let config = "diff: { since: origin/main, markdown: true, committed_only: true }\n";
        let Action::Diff(args) = parse_with_config(&["diff"], config).unwrap().action else {
            panic!("expected a diff action");
        };
        assert_eq!(args.since().unwrap(), "origin/main");
        assert!(args.markdown && args.committed_only && !args.json);

        // An output format on the command line replaces the file's
        let Action::Diff(args) = parse_with_config(&["diff", "--since", "v1", "--json"], config)
            .unwrap()
            .action
        else {
            panic!("expected a diff action");
        };
        assert_eq!(args.since().unwrap(), "v1");
        assert!(args.json && !args.markdown);

        assert!(parse_with_config(&["diff"], "diff: { json: true, markdown: true }\n").is_err());
    }

    #[test]
    fn test_config_fills_check_options() {
        use crate::checker::OutputFormat;

        let config = "check: { format: github, generate_codeowners: true }\n";
        let invocation = parse_with_config(&["check"], config).unwrap();
        let Action::Check { check, .. } = &invocation.action else {
            panic!("expected a check action");
        };
        assert_eq!(check.output_format(), OutputFormat::Github);
        assert!(invocation.generate_codeowners);

        // --json replaces the file's format
        let Action::Check { check, .. } = parse_with_config(&["check", "--json"], config)
            .unwrap()
            .action
        else {
            panic!("expected a check action");
        };
        assert_eq!(check.output_format(), OutputFormat::Json);

        let Action::Check { check, .. } = parse_with_config(&["check"], "check: { json: true }\n")
            .unwrap()
            .action
        else {
            panic!("expected a check action");
        };
        assert_eq!(check.output_format(), OutputFormat::Json);
    }

    #[test]
    fn test_legacy_form_requires_a_path() {
        let err = parse(&["--json"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }
}
