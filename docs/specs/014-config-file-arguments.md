---
status: implemented
date: 2026-11-18
---

# Every CLI option and the check rules in features.config.yaml

## Context

`features.config.toml` only held `name`. Teams repeat the same flags on every command (`--ignore-path`, `--strip-emails`, `--port`, `--since origin/main`…), in scripts, CI and local runs. Any command line option should be settable once in a configuration file, written in YAML.

PR #138 (configurable check rules and a `.features.yml` config file) was merged into `claude/security-hardening` after that branch had already been merged into master, so it never shipped. Its check rules are ported here, reading the same single configuration file.

## Requirements

### Configuration file

- One file: `features.config.yaml`. No `.yml` variant, no `.features.yml`. `features.config.toml` (shipped in 0.15.0 with only `name`) is no longer read; when found during lookup, a warning says to move `name` to `features.config.yaml`.
- Every option of the command line can be set in it, by its long name without `--` (kebab-case).
  - Top level: `name` (`--project-name`), `path` (folder to scan when none is given), `skip-changes`, `ignore-path`, `include-skipped-dirs`, `coverage-dir`, `project-dir`, `strip-emails`, `data`, `codeowners-path`, `codeowners-prefix`.
  - One section per command for its own options: `scan` (`json`, `flat`, `description`, `coverage`), `serve` (`port`, `allow-origin`, `experimental`, `generate-codeowners`), `build` (`build-dir`, `experimental`, `generate-codeowners`), `check` (`generate-codeowners`, `json`, `format`, `rules`), `diff` (`since`, `committed-only`, `json`, `markdown`, `owners-only`), `find-owner`, `owners`, `records` (`json`), `aggregate` (`build-dir`, `json`, `data`).
- Precedence: command line, then file, then built-in default. A value passed on the command line wins even when it equals the default (`--port 3000`).
- Lists (`ignore-path`, `data`, `allow-origin`) combine the file's values, then the command line's.
- Output formats are one choice: a format flag on the command line replaces the file's (`diff`: json/markdown/owners-only, the file can't set two; `check`: json/format).
- `features diff` without `--since` works when `diff.since` is set, otherwise fails with a message naming both.
- Relative paths in the file are relative to the file's folder (kept relative when that folder is the current directory).
- Lookup: the scanned folder (current directory when none is given), then its parents up to the git root; the nearest file wins. An empty file (or only comments) is an empty config. `--config <FILE>` reads another file, `--no-config` ignores it. Both are global (before or after the command) and conflict with each other.
- Unknown keys, wrong types, unknown rules and invalid rule options are errors naming the file (and the line for YAML errors).
- The legacy flag form (`features ./src --serve`) reads the file too.
- `allow-origin` values are validated and normalized like on the command line.

### Check rules

- `features check` runs a registry of rules, each with a kebab-case id, a severity (`error` fails the check, `warn` is printed, `off` doesn't run) and typed options (kebab-case keys):
  - `duplicate-names` (error), `duplicate-keys` (warn), `record-issues` (warn: broken record links, no status, superseded without replacement, implemented spec with open tasks), `missing-owner` (warn, `allow-inherited: true`), `stale-flag` (warn, `max-age-days: 90`, `metadata-keys: [flag]`), `missing-readme` (off, `max-files-without-readme: 10`), `min-coverage` (off, `threshold: 80`), `cross-feature-import` (off, `allow: []`).
  - `duplicate-keys` and `record-issues` keep the warnings `features check` printed before.
- Severity and options come from the defaults, then `check.rules` (`<id>: <severity>` or `<id>: { severity, ...options }`), then `--rule <id>=<severity>`, `--enable-rule <id>` (off → error), `--disable-rule <id>` (wins over enable).
- Report formats: `text` (grouped by rule, summary), `json` (`--json` or `--format json`), `github` (GitHub Actions annotations, files relative to the current directory). `--list-rules` prints the effective severities and exits.
- Coverage is only computed when `min-coverage` runs.
- The check options are only on `features check` (not on the deprecated `--check` flag, which still reads `check.rules` from the file).

## Out of scope

- Positional arguments naming what to act on: `find-owner` target, `new` kind/feature/title, `aggregate` folder; and `--list-rules`.
- `--no-<flag>` negations: `--no-config` is the way to turn off a flag the file turns on.
- Per-command overrides of top-level options (e.g. `serve.skip-changes`).
- TOML for the configuration.

## Plan

1. `cli.rs`: global `--config` / `--no-config`; `diff --since` optional; `check` gets `--json` and the check options; `Invocation` keeps `generate_codeowners` + `codeowners` options separately; `try_parse_args_from` returns the invocation, the config source and the ids of arguments given on the command line (clap `ValueSource::CommandLine`).
2. `config.rs`: `FileConfig` (serde + `serde_yaml`, `deny_unknown_fields`, kebab-case), lookup, legacy TOML warning, relative path resolution, `apply` (fill non-explicit options, validate `check.rules`), `resolve` (load + apply + check `since`).
3. `checker` (ported from #138): rule registry, `checker/config.rs` (severities and options), `checker/args.rs` (CLI options, `--list-rules`), `checker/report.rs` (text/json/github), one file per rule in `checker/rules/`, plus the new `duplicate-keys` and `record-issues`.
4. `project_name.rs`: no config file reading; the name comes from `--project-name` or `name`, then manifests.
5. `bin.rs`: parse, resolve, run.
6. Tests: unit tests for parsing, precedence, lists, sections, lookup, `--config`/`--no-config`, errors and every rule; integration tests on `examples/check-rules` (with its coverage report in `.coverage/`, since `coverage/` is gitignored).
7. Docs: `docs/cli.md` "Configuration file" and "Check rules" sections, options table, changelog, and the website docs section.
