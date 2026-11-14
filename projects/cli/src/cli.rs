//! Command line interface.
//!
//! Each action is a subcommand (`features serve ./src`). The flag forms used
//! before subcommands existed (`features ./src --serve`) are still accepted as
//! deprecated aliases: both forms are parsed into the same [`Invocation`].

use clap::error::ErrorKind;
use clap::{ArgGroup, Args, CommandFactory, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::http_server::parse_allowed_origin;
use crate::models::RecordKind;

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

    /// Aggregate the features JSON reports of several repositories into one
    /// web dashboard (static build), merging features with the same name
    Aggregate {
        #[command(flatten)]
        aggregate: AggregateArgs,
    },

    /// List the specs and decision records of the project and of every feature
    Records {
        /// The folder to scan for features
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output the project-level records as JSON (`{ "records": [...] }`),
        /// the same as the `records.json` of `serve` and `build`
        #[arg(long)]
        json: bool,

        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Create the next numbered spec or decision record of a feature
    New {
        #[command(flatten)]
        new: NewArgs,
        #[command(flatten)]
        scan: ScanArgs,
    },

    /// Run checks on the features (e.g. duplicate names) and warn about
    /// broken specs and decision records
    Check {
        /// The folder to scan for features
        #[arg(default_value = ".")]
        path: PathBuf,

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

    /// JSON file with data from external tools, by feature key, e.g.
    /// { "checkout": { "weekly_users": 1200 } } (can be repeated)
    #[arg(long, value_name = "FILE")]
    pub data: Vec<PathBuf>,
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

    /// Enable the experimental parts of the web dashboard (History, project records)
    #[arg(long)]
    pub experimental: bool,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct BuildArgs {
    /// The folder to scan for features
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output directory for the static build
    #[arg(long, default_value = DEFAULT_BUILD_DIR)]
    pub build_dir: PathBuf,

    /// Enable the experimental parts of the web dashboard (History, project records)
    #[arg(long)]
    pub experimental: bool,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct AggregateArgs {
    /// Folder with one features JSON report per repository:
    /// `<repository>.json` (from `features scan --json`) or
    /// `<repository>/features.json` (from `features build`)
    pub folder: PathBuf,

    /// Output directory for the static build
    #[arg(long, default_value = DEFAULT_BUILD_DIR)]
    pub build_dir: PathBuf,

    /// Print the aggregated JSON instead of creating a static build
    #[arg(long)]
    pub json: bool,

    /// JSON file with data from external tools, by feature key, applied to the
    /// aggregated features (can be repeated)
    #[arg(long, value_name = "FILE")]
    pub data: Vec<PathBuf>,
}

/// Kind of record created by `features new`
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewRecordKind {
    Spec,
    Decision,
}

impl From<NewRecordKind> for RecordKind {
    fn from(kind: NewRecordKind) -> Self {
        match kind {
            NewRecordKind::Spec => RecordKind::Spec,
            NewRecordKind::Decision => RecordKind::Decision,
        }
    }
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct NewArgs {
    /// Kind of record to create
    pub kind: NewRecordKind,

    /// Name or path of the feature, or `.` for the project-level `docs/` folder
    pub feature: String,

    /// Title of the record, also used for its file name
    pub title: String,

    /// The folder to scan for features
    #[arg(long, default_value = ".")]
    pub path: PathBuf,
}

#[derive(Args, Debug, Clone, PartialEq)]
pub struct DiffArgs {
    /// The folder to scan for features (default: current directory)
    pub path: Option<PathBuf>,

    /// List the features touched since the merge base of this git ref and HEAD,
    /// including uncommitted changes (e.g. --since origin/master)
    #[arg(long, value_name = "REF")]
    pub since: String,

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

impl OptionalCodeownersArgs {
    fn requested(self) -> Option<CodeownersOptions> {
        self.generate_codeowners.then_some(self.options)
    }
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
    Check { path: PathBuf },
    /// List the specs and decision records
    Records { path: PathBuf, json: bool },
    /// Create a spec or decision record
    New(NewArgs),
    /// Aggregate the features reports of several repositories
    Aggregate(AggregateArgs),
}

/// A parsed command line
#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
    pub action: Action,
    pub scan: ScanArgs,
    /// Generate CODEOWNERS too, with these options
    pub codeowners: Option<CodeownersOptions>,
}

impl Cli {
    /// Turn the command line, in either form, into what to do
    pub fn into_invocation(self) -> Result<Invocation, clap::Error> {
        if self.version {
            return Ok(Invocation {
                action: Action::Version,
                scan: ScanArgs::default(),
                codeowners: None,
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
        let (action, scan, codeowners) = match self {
            Command::Scan { output, scan } => (Action::List(output), scan, None),
            Command::Serve {
                server,
                codeowners,
                scan,
            } => (Action::Serve(server), scan, codeowners.requested()),
            Command::Build {
                build,
                codeowners,
                scan,
            } => (Action::Build(build), scan, codeowners.requested()),
            Command::Codeowners { codeowners, scan } => (
                Action::Codeowners {
                    path: codeowners.path,
                },
                scan,
                Some(codeowners.options),
            ),
            Command::FindOwner {
                target,
                path,
                json,
                scan,
            } => (Action::FindOwner { target, path, json }, scan, None),
            Command::Owners { path, json, scan } => (Action::Owners { path, json }, scan, None),
            Command::Diff { diff, scan } => (Action::Diff(diff), scan, None),
            Command::Aggregate { aggregate } => {
                (Action::Aggregate(aggregate), ScanArgs::default(), None)
            }
            Command::Check {
                path,
                codeowners,
                scan,
            } => (Action::Check { path }, scan, codeowners.requested()),
            Command::Records { path, json, scan } => (Action::Records { path, json }, scan, None),
            Command::New { new, scan } => (Action::New(new), scan, None),
        };

        Invocation {
            action,
            scan,
            codeowners,
        }
    }
}

impl LegacyArgs {
    fn into_invocation(self) -> Result<Invocation, clap::Error> {
        if let Some(since) = self.since {
            return Ok(Invocation {
                action: Action::Diff(DiffArgs {
                    path: self.path,
                    since,
                    committed_only: self.committed_only,
                    json: self.json,
                    markdown: self.markdown,
                    owners_only: self.owners_only,
                    codeowners_prefix: self.codeowners.codeowners_prefix,
                }),
                scan: self.scan,
                codeowners: None,
            });
        }

        let codeowners = self.generate_codeowners.then_some(self.codeowners);

        if let Some(target) = self.find_owner {
            return Ok(Invocation {
                action: Action::FindOwner {
                    target,
                    path: self.path,
                    json: self.json,
                },
                scan: self.scan,
                codeowners: None,
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
                experimental: false,
            })
        } else if self.build {
            Action::Build(BuildArgs {
                path,
                build_dir: self.build_dir,
                experimental: false,
            })
        } else if self.check {
            Action::Check { path }
        } else if self.list_owners {
            Action::Owners {
                path,
                json: self.json,
            }
        } else if codeowners.is_some() {
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
            codeowners,
        })
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
            experimental: false,
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
    fn test_serve_experimental() {
        let expected = Action::Serve(ServerArgs {
            experimental: true,
            ..serve("./src")
        });
        assert_eq!(action(&["serve", "./src", "--experimental"]), expected);
    }

    #[test]
    fn test_build() {
        let expected = Action::Build(BuildArgs {
            path: path("./src"),
            build_dir: path("out"),
            experimental: false,
        });
        assert_eq!(
            action(&["./src", "--build", "--build-dir", "out"]),
            expected
        );
        assert_eq!(action(&["build", "./src", "--build-dir", "out"]), expected);

        let experimental = Action::Build(BuildArgs {
            path: path("./src"),
            build_dir: path(DEFAULT_BUILD_DIR),
            experimental: true,
        });
        assert_eq!(action(&["build", "./src", "--experimental"]), experimental);
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
            codeowners: Some(CodeownersOptions {
                codeowners_path: Some(path(".github/CODEOWNERS")),
                codeowners_prefix: "@org/".to_string(),
            }),
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
            assert_eq!(
                invocation.codeowners,
                Some(CodeownersOptions::default()),
                "{args:?}"
            );
        }

        assert_eq!(parse(&["serve", "./src"]).unwrap().codeowners, None);
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
        };
        assert_eq!(action(&["./src", "--check"]), check);
        assert_eq!(action(&["check", "./src"]), check);
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
            data: vec![path("usage.json"), path("errors.json")],
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
            "--data",
            "usage.json",
            "--data",
            "errors.json",
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
            since: "origin/master".to_string(),
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
            since: "main".to_string(),
            committed_only: false,
            json: true,
            markdown: false,
            owners_only: false,
            codeowners_prefix: DEFAULT_CODEOWNERS_PREFIX.to_string(),
        });
        assert_eq!(action(&["--since", "main", "--json"]), expected);
        assert_eq!(action(&["diff", "--since", "main", "--json"]), expected);

        // `features diff` needs a ref
        assert!(parse(&["diff", "./src"]).is_err());
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

    #[test]
    fn test_aggregate() {
        assert_eq!(
            action(&["aggregate", "./reports"]),
            Action::Aggregate(AggregateArgs {
                folder: path("./reports"),
                build_dir: path(DEFAULT_BUILD_DIR),
                json: false,
                data: vec![],
            })
        );
        assert_eq!(
            action(&[
                "aggregate",
                "./reports",
                "--build-dir",
                "out",
                "--json",
                "--data",
                "usage.json",
            ]),
            Action::Aggregate(AggregateArgs {
                folder: path("./reports"),
                build_dir: path("out"),
                json: true,
                data: vec![path("usage.json")],
            })
        );

        // The folder of reports is required
        assert!(parse(&["aggregate"]).is_err());
    }

    #[test]
    fn test_records_and_new() {
        assert_eq!(
            action(&["records", "./src", "--json"]),
            Action::Records {
                path: path("./src"),
                json: true,
            }
        );
        assert_eq!(
            action(&["new", "spec", "auth", "Login flow"]),
            Action::New(NewArgs {
                kind: NewRecordKind::Spec,
                feature: "auth".to_string(),
                title: "Login flow".to_string(),
                path: path("."),
            })
        );
        assert_eq!(
            action(&["new", "decision", ".", "Use JWT", "--path", "./src"]),
            Action::New(NewArgs {
                kind: NewRecordKind::Decision,
                feature: ".".to_string(),
                title: "Use JWT".to_string(),
                path: path("./src"),
            })
        );
        assert!(parse(&["new", "note", "auth", "x"]).is_err());
        assert!(parse(&["new", "spec", "auth"]).is_err());
    }

    #[test]
    fn test_legacy_form_requires_a_path() {
        let err = parse(&["--json"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }
}
