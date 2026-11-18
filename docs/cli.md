# CLI reference

```bash
features <command> [PATH] [options]
```

Each action is a command. `PATH` is the folder to scan for features and defaults to the current directory:

| Command | Description |
| ------- | ----------- |
| `features scan [PATH]` | List the features (the default action, so `features PATH` works too) |
| `features serve [PATH]` | Start an HTTP server with the web dashboard, refreshed when files change |
| `features build [PATH]` | Build a static version of the web dashboard |
| `features codeowners [PATH]` | Generate or update a CODEOWNERS file with feature ownership information |
| `features find-owner <TARGET> [PATH]` | Find the owner of a specific file or folder |
| `features owners [PATH]` | Display only the unique list of owners |
| `features check [PATH]` | Run the [check rules](#check-rules) on the features: duplicate names and keys, missing owners, broken specs and decision records, and opt-in rules (coverage, imports between features, stale feature flags...). Fails when a rule set to `error` finds something |
| `features records [PATH]` | List the specs and decision records of the project and of every feature. With `--json`, print the project-level records as `{ "records": [...] }` |
| `features new <spec\|decision> <FEATURE> <TITLE>` | Create the next numbered spec or decision record (`NNN-title.md`) in `<FEATURE>/__docs__/specs` or `/decisions`. `FEATURE` is a feature name or path, `.` for the project-level `docs/` folder. Use `--path` to choose the folder to scan |
| `features diff [PATH] --since <ref>` | List the features touched since `<ref>`, with their owners. See [Ping the right teams on every pull request](../README.md#ping-the-right-teams-on-every-pull-request) |
| `features mcp [PATH]` | Run an MCP server over stdio, so AI coding agents (Claude Code, Cursor...) can query the features. See [Give an AI coding agent a map of the codebase (MCP)](#give-an-ai-coding-agent-a-map-of-the-codebase-mcp) |
| `features skills <list\|install>` | List or install the Claude skills bundled with the CLI (create, document, organize and validate features). See [Install the Claude skills](#install-the-claude-skills) |
| `features aggregate <FOLDER>` | Aggregate the features JSON reports of several repositories into one dashboard. See [Aggregate several repositories](#aggregate-several-repositories) |

Run `features <command> --help` for the options of a command. The options are:

| Option | Commands | Description |
| ------ | -------- | ----------- |
| `--json` | `scan`, `find-owner`, `owners`, `diff`, `aggregate`, `records`, `check` | Output as JSON |
| `--flat` | `scan` | Output features as a flat array instead of nested structure |
| `--description` | `scan` | Include feature descriptions in the output |
| `--coverage` | `scan` | Include coverage information in the output (always on with `--json`, `serve` and `build`) |
| `--port <port>` | `serve` | Change the port (default: 3000) |
| `--allow-origin <origin>` | `serve` | Allow another origin (e.g. `http://localhost:5173`) to read the server's data via CORS. Can be repeated; `*` allows any origin. By default no cross-origin access is allowed and the server only answers requests addressed to `localhost`, `127.0.0.1` or `[::1]` |
| `--experimental` | `serve`, `build` | Enable the experimental parts of the web dashboard (History page, owner pages linked from owner names, project records in Outside features). They are hidden by default and marked as experimental when enabled |
| `--project-name <name>` | `serve`, `build` | Project name shown at the top left of the web dashboard. Overrides `name` in `features.config.yaml` and the name detected from the project manifest. See [Name the dashboard](#name-the-dashboard) |
| `--build-dir <path>` | `build`, `aggregate` | Output directory for the static build (default: `build`) |
| `--since <ref>` | `diff` | Compare with the merge base of `<ref>` and `HEAD`, including uncommitted changes (required, unless `since` is set in the [configuration file](#configuration-file)) |
| `--committed-only` | `diff` | Ignore uncommitted and untracked files |
| `--markdown` | `diff` | Output a markdown table for pull request comments |
| `--owners-only` | `diff` | Print only the unique owners of the touched features, one per line |
| `--generate-codeowners` | `serve`, `build`, `check` | Also generate or update the CODEOWNERS file |
| `--codeowners-path <path>` | `codeowners`, and with `--generate-codeowners` | Custom path and filename for CODEOWNERS file (default: `CODEOWNERS` in project directory) |
| `--codeowners-prefix <prefix>` | `codeowners`, `diff`, and with `--generate-codeowners` | Custom prefix for owner names in CODEOWNERS file and in `diff --markdown` / `--owners-only` output (default: `@`) |
| `--project-dir <path>` | all | Project directory for CODEOWNERS generation and additional coverage search locations |
| `--skip-changes` | all | Skip computing git commit history (faster for large repos) |
| `--ignore-path <path>` | all | Ignore a folder and all its subfolders when scanning for features. Can be repeated to ignore multiple folders |
| `--include-skipped-dirs` | all | Also scan folders that are skipped by default (`node_modules`) for features, stats and imports |
| `--coverage-dir <path>` | all | Specify a custom coverage directory (overrides automatic search) |
| `--strip-emails` | all | Remove commit author emails from the output (JSON, `serve` and `build`). Useful before publishing a static build. Author names are kept |
| `--dir <path>` | `skills install` | Skills folder to install into (default: `.claude/skills` in the current directory) |
| `--global` | `skills install` | Install into `~/.claude/skills`, for every project |
| `--force` | `skills install` | Overwrite the files of skills that are already installed |
| `--dry-run` | `skills install` | Print what would be installed without writing anything |
| `--data <file>` | `scan`, `serve`, `build`, `check`, `mcp`, `aggregate` | Merge data from external tools (analytics, errors, issues, ...) into the features, matched by feature key. Can be repeated; later files override earlier ones. See [Import data from external tools](#import-data-from-external-tools) |
| `--rule <id>=<severity>` | `check` | Set the severity of a [check rule](#check-rules): `error`, `warn` or `off`. Can be repeated |
| `--enable-rule <id>` | `check` | Turn on a rule that is off (as `error`). Can be repeated |
| `--disable-rule <id>` | `check` | Turn off a rule. Can be repeated |
| `--format <format>` | `check` | Output of the check report: `text` (default), `json` or `github` (GitHub Actions annotations) |
| `--list-rules` | `check` | List the check rules with their effective severity, then exit |
| `--config <file>` | all | Read the default options from this file (YAML) instead of the `features.config.yaml` found in the scanned folder or its parents. See [Configuration file](#configuration-file) |
| `--no-config` | all | Ignore `features.config.yaml`: only use the options of the command line |

> **Legacy flags.** Before commands existed, actions were flags: `features PATH --serve`, `--build`, `--check`, `--list-owners`, `--find-owner <TARGET>` and `--generate-codeowners`. They still work but are deprecated. Combining two actions (e.g. `--serve --build`) is now an error instead of silently running one of them; `--generate-codeowners` can still be combined with `--serve`, `--build`, `--check` and `--list-owners`. `features PATH --since <ref>` also still works, as an alias of `features diff PATH --since <ref>`.

## Configuration file

Every option can also be set in `features.config.yaml`, usually at the root of the repository, so a team shares the same settings and commands stay short. Keys are the option names without `--`. Options used by every command go at the top level; options of a single command go in a section named after it:

```yaml
name: Acme Store                 # --project-name
path: src                        # folder to scan when the command line gives none
skip-changes: false
strip-emails: true
ignore-path:
  - src/legacy
  - src/generated
include-skipped-dirs: false
coverage-dir: coverage
project-dir: .
data: [reports/usage.json]
codeowners-path: .github/CODEOWNERS
codeowners-prefix: "@acme/"      # quoted: YAML reserves a leading @

scan:
  json: false
  flat: false
  description: true
  coverage: true

serve:
  port: 4000
  allow-origin: ["http://localhost:5173"]
  experimental: true
  generate-codeowners: false

build:
  build-dir: dashboard
  experimental: true
  generate-codeowners: true

check:
  generate-codeowners: true
  format: github                 # text, json or github
  rules:                         # see Check rules
    missing-owner: error
    min-coverage:
      severity: error
      threshold: 85

diff:
  since: origin/main
  committed-only: false
  markdown: true                 # or json, or owners-only

find-owner:
  json: true

owners:
  json: true

records:
  json: true

aggregate:
  build-dir: build
  json: false
  data: [reports/usage.json]
```

With this file, `features serve` scans `./src` on port 4000, and `features diff` compares with `origin/main`.

- **The command line wins.** An option given on the command line overrides the file, which overrides the built-in default. `features serve --port 3000` uses port 3000 whatever the file says.
- **Lists are combined.** `ignore-path`, `data` and `allow-origin` take the values of the file, then those of the command line.
- **Paths are relative to the file.** `path: src` means the `src` folder next to `features.config.yaml`, wherever the command runs from.
- **Lookup.** The file is looked up in the scanned folder (the current directory when none is given), then in its parents up to the root of the git repository. The nearest one wins. `--config <file>` reads another file (for example a CI-specific one) and `--no-config` ignores it, which is also the way to turn off a flag the file turns on.
- **Strict.** An unknown key or a value of the wrong type is an error with its line, so a typo doesn't silently do nothing.

The arguments that name what to act on (the `TARGET` of `find-owner`, the kind, feature and title of `new`, the `FOLDER` of `aggregate`) and `--list-rules` stay on the command line. `--rule`, `--enable-rule` and `--disable-rule` are written as `check.rules`.

> Earlier versions read the dashboard name from `name = "..."` in a `features.config.toml`. That file is no longer read (a warning says so): move the name to `features.config.yaml` as `name: ...`.

## Check rules

`features check` runs a set of rules. Each has an id and a severity: `error` findings make the check fail (exit code 1), `warn` findings are only printed, `off` rules don't run.

| Rule | Default | Finds | Options (default) |
| ---- | ------- | ----- | ----------------- |
| `duplicate-names` | `error` | Two features with the same name | |
| `duplicate-keys` | `warn` | Several features with the same [feature key](readme-format.md#key-and-links), so `--data` goes to all of them | |
| `record-issues` | `warn` | Specs and decision records with a broken link, no status, superseded without a replacement, or implemented with open tasks | |
| `missing-owner` | `warn` | A feature without owner | `allow-inherited` (`true`): an owner inherited from a parent feature is enough |
| `stale-flag` | `warn` | A feature flag annotation (`// --feature-flag name: new-login, introduced_on: 2025-01-01`) older than the allowed age | `max-age-days` (`90`), `metadata-keys` (`[flag]`) |
| `missing-readme` | `off` | A large feature without `README.md`, `README.mdx` or `FEATURES.toml` | `max-files-without-readme` (`10`) |
| `min-coverage` | `off` | A feature whose line coverage is below the threshold (features with coverage data only) | `threshold` (`80`, percent) |
| `cross-feature-import` | `off` | A feature importing code from a sibling feature | `allow` (`[]`): feature paths any feature may import from, e.g. `features/shared` |

Set the severity and options in `features.config.yaml`, by rule id, then override them on the command line:

```yaml
check:
  rules:
    missing-owner: error              # severity only
    cross-feature-import:             # severity and options
      severity: error
      allow: [features/shared]
```

```bash
features check --rule stale-flag=error --enable-rule missing-readme --disable-rule duplicate-keys
features check --list-rules           # effective severity of every rule
```

`features check --format github` prints [GitHub Actions annotations](https://docs.github.com/actions/reference/workflow-commands-for-github-actions), so findings show up on the lines of the pull request:

```yaml
- run: npx features-cli@latest check --format github
```

## Recipes

### Explore a project in the browser

```bash
npx features-cli@latest serve ./src
```

### Generate a CODEOWNERS file

```bash
features codeowners ./src --project-dir .
```

See the [FAQ](../FAQ.md#how-can-i-generate-codeowners-file) for more options.

### Validate features in CI

```bash
features check ./src --format github
```

See [Check rules](#check-rules) to choose what fails the build.

### Read specs and decisions

```bash
features records ./               # every spec and decision, per feature
features new spec auth "Login flow"
```

Each feature of the JSON report has a `records` array (the `decisions` array of raw markdown is kept for compatibility):

```json
{
  "kind": "spec",
  "id": "001-login-flow",
  "title": "Login flow",
  "status": "implemented",
  "date": "2026-01-15",
  "path": "features/auth/__docs__/specs/001-login-flow.md",
  "content": "# Login flow ...",
  "links": { "decisions": ["002-use-jwt"], "specs": [], "supersedes": [], "superseded_by": [] },
  "tasks": { "done": 2, "total": 2 }
}
```

`status`, `date`, `tasks`, `files` (the other files of a folder spec) and `feature` are only present when known. `source` is `speckit` or `kiro` for specs read from a [GitHub Spec Kit or Kiro folder](../FAQ.md#are-github-spec-kit-and-kiro-specs-supported), and absent otherwise. The top level of `features scan --json` stays an array of features: project-level records (root `docs/specs`, `docs/decisions` and `docs/adr` files without a `feature:` frontmatter) are printed by `features records --json`, served at `/records.json` by `features serve` and written to `records.json` by `features build`, as `{ "records": [...] }`. See the [FAQ](../FAQ.md#how-can-i-add-a-spec-or-a-decision) for the file format.

### List the features and owners touched by a change

```bash
features diff ./src --since origin/master
```

Add `--markdown` for a pull request comment or `--owners-only` to request reviews. See the [GitHub Actions example](../README.md#comment-on-pull-requests).

### Give an AI coding agent a map of the codebase (MCP)

```bash
features mcp ./src
```

runs a [Model Context Protocol](https://modelcontextprotocol.io) server over stdio (newline-delimited JSON-RPC 2.0). Agents start it themselves; register it once:

```bash
claude mcp add features -- npx features-cli@latest mcp ./src   # Claude Code
```

For Cursor, in `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "features": { "command": "npx", "args": ["features-cli@latest", "mcp", "./src"] }
  }
}
```

| Tool | Arguments | Returns |
| ---- | --------- | ------- |
| `list_features` | `owner`, `query` (both optional) | `root` (absolute path of the scanned folder) and every feature, flat: `name`, `key`, `path`, `owner`, `parent` (path of the parent feature) and `summary` (first paragraph of the README). `owner` ignores case and a leading `@`; `query` searches the name, key, path and README |
| `get_feature` | `feature`: key, path, name or folder name | Name, key, path, owner, parent, sub-features, README, `meta`, `links`, `external` (`--data`), stats (files, lines, TODOs, commits, coverage totals), records (summaries with their `path`), dependencies grouped by feature, and the 10 most recent commits |
| `find_owner` | `path`: absolute, relative to the current directory or to the scanned folder | `owner`, `inherited`, `feature_name`, `feature_key`, `feature_path`. The path does not need to exist |
| `features_touched` | `since` (git ref), `committed_only` (optional) | The same report as `features diff --json` |
| `list_records` | `kind` (`spec` or `decision`), `feature` (both optional) | The project-level specs and decision records, or the ones of a feature, without their content |

All tools are read-only and work offline. A tool that fails (unknown feature, unknown git ref) returns an error result the agent can read. The project is scanned on the first tool call, with coverage and git history (unless `--skip-changes`), and rescanned after files that can change the features change or a `--data` file is updated. The scan options (`--skip-changes`, `--ignore-path`, `--data`, `--strip-emails`, `--coverage-dir`, `--project-dir`...) work as for the other commands. stdout only carries protocol messages; logs go to stderr.

### Install the Claude skills

```bash
features skills list              # the bundled skills and what they do
features skills install           # into ./.claude/skills, shared with your team through git
features skills install --global  # into ~/.claude/skills, for every project
```

The [Claude skills](../projects/skills/README.md) (`create-feature`, `organize-by-feature`, `document-feature`, `validate-features`...) are embedded in the binary, so the installed skills always match the commands and JSON fields of your version of the CLI. All the skills are installed together, since they refer to each other by name. Nothing is downloaded.

A skill folder that already exists is left untouched; `--force` overwrites its files (other files in it are kept), e.g. after upgrading the CLI. The CLI version is written to `.features-version` in the skills folder. `--dir <path>` installs into another folder, for agents that read the Agent Skills format. `--dry-run` lists what would be installed, skipped or overwritten.

### Export a JSON report

```bash
features scan ./src --json > features.json
```

### Publish a static dashboard

```bash
features build ./src --build-dir ./dashboard --strip-emails
```

The `dashboard` folder can be deployed to any static host. See the [GitHub Pages example](../README.md#publish-the-dashboard-to-github-pages).

### Name the dashboard

The dashboard shows the project name at the top left. By default it comes from the manifest of the project (`package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, `composer.json`, `pom.xml`, `settings.gradle(.kts)`, `pubspec.yaml` or `deno.json`), looked up in the scanned folder then in its parents up to the root of the git repository. Without one, it shows "Features".

To choose another name, add a [`features.config.yaml`](#configuration-file) at the root of the repository:

```yaml
name: Acme Store
```

or pass it to `serve` or `build`, which wins over the file:

```bash
features build ./src --project-name "Acme Store"
```

### Import data from external tools

Give every feature data from PostHog, Sentry, Linear or any other tool. Export it with a script to a JSON file that maps [feature keys](readme-format.md#key-and-links) to metrics:

```json
{
  "checkout": { "weekly_users": 12840, "open_errors": 3 },
  "search": { "weekly_users": 310, "status": "healthy" }
}
```

```bash
features build ./src --data usage.json --data errors.json
```

The metrics are added to the `external` field of every feature with a matching key, shown on the feature page and as sortable columns of the insights table. Metric values must be strings, numbers or booleans. Keys that match no feature are reported as a warning. `serve` reads the files again on every rescan. Run `features scan ./src --json` to see the key of each feature, and `features check` to find keys shared by several features.

### Aggregate several repositories

Export one JSON report per repository, put them in the same folder, then combine them into a single dashboard:

```bash
features scan ./repo-a/src --json > reports/repo-a.json
features scan ./repo-b/src --json > reports/repo-b.json

features aggregate ./reports                  # static build in ./build (index.html + features.json)
features aggregate ./reports --json           # print the aggregated JSON
```

A report is either `<repository>.json` or `<repository>/features.json` (e.g. the output of `features build`). The repository is named after the file or the folder.

Reconciliation rules:

- Features with the same name are the same feature and are merged. Each aggregated feature lists, in `repositories`, every repository where it was found and its path there.
- A merged feature takes the parent it has in the repositories where it is nested, and gets the children it has in every repository.
- Features found in a single repository keep their place: unmatched root features stay at the root.
- Features are **not** merged when merging them would create a loop (e.g. `B > C` in one repository and `C > B` in another) or when they have different parents in different repositories. They are kept once per repository, and the issue is listed in the `errors` section of `features.json` and in the *Not merged across repositories* section of the *Outside features* page of the dashboard.

The aggregated `features.json` is an object: `{ "repositories": [...], "features": [...], "errors": [...] }`, where `features` has the same structure as the output of `features scan --json`.
