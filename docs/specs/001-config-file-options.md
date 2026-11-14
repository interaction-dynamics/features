---
status: implemented
date: 2026-11-14
---

# Every command-line option in the configuration file

## Context

`.features.yml` (or `--config <path>`) first only held the settings of the check rules. Projects repeat the same options on every run (`--skip-changes`, `--ignore-path`, `--since origin/main`, `--format github`...), so every option should be settable in the file.

## Requirements

- Each key is the long option name in snake_case. Options shared by several commands are at the top level (`path`, `skip_changes`, `ignore_path`, `include_skipped_dirs`, `coverage_dir`, `project_dir`, `strip_emails`, `codeowners_path`, `codeowners_prefix`). The options of one command are in a section named after it: `scan`, `serve`, `build`, `owners`, `find_owner`, `diff`, `check`.
- Precedence: command line > file > defaults. A value typed on the command line wins even when it equals the default (detected with clap's value source).
- Lists (`ignore_path`, `serve.allow_origin`) add up with the command line values.
- Flags can only be turned on from the command line. A `false` in the file doesn't undo a flag.
- An output format on the command line replaces the file's output format as a whole (`diff.json`/`markdown`/`owners_only`, `check.format`/`json`).
- `diff.since` can replace `--since`. `features diff` fails with a clear message when neither gives a ref.
- Relative paths in the file are relative to the file's folder.
- Unknown keys and wrong types are errors that name the file.
- These stay on the command line only: the find-owner target, `--list-rules`, `--config`, `--version` and `--help`.
- The deprecated flag forms read the same sections as their commands.

## Out of scope

- Turning a flag off from the command line (`--no-skip-changes`).
- Environment variables.

## Plan

1. `config.rs`: typed `ProjectConfig` with top-level and per-command sections, `deny_unknown_fields`, paths resolved against the file's folder.
2. `cli.rs`: `ExplicitArgs` (ids given on the command line) and `Invocation::apply_config`. `DiffArgs.since` becomes optional. `Invocation` keeps the CODEOWNERS options even when generation isn't requested, so the file can turn it on.
3. `bin.rs`: load the configuration once for every command and merge it before running.
4. Checker: reads `check.rules`.
5. Tests: unit tests for the merge rules, integration tests on `examples/check-rules`.
6. Docs: `docs/configuration.md` and the README.
