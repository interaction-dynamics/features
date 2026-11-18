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
| `features check [PATH]` | Run validation checks on features (e.g., duplicate names). Also warns about broken links between specs and decisions, records without a status, superseded records without a replacement and implemented specs with open tasks |
| `features records [PATH]` | List the specs and decision records of the project and of every feature. With `--json`, print the project-level records as `{ "records": [...] }` |
| `features new <spec\|decision> <FEATURE> <TITLE>` | Create the next numbered spec or decision record (`NNN-title.md`) in `<FEATURE>/__docs__/specs` or `/decisions`. `FEATURE` is a feature name or path, `.` for the project-level `docs/` folder. Use `--path` to choose the folder to scan |
| `features diff [PATH] --since <ref>` | List the features touched since `<ref>`, with their owners. See [Ping the right teams on every pull request](../README.md#ping-the-right-teams-on-every-pull-request) |
| `features mcp [PATH]` | Run an MCP server over stdio, so AI coding agents (Claude Code, Cursor...) can query the features. See [Give an AI coding agent a map of the codebase (MCP)](#give-an-ai-coding-agent-a-map-of-the-codebase-mcp) |
| `features skills <list\|install>` | List or install the Claude skills bundled with the CLI (create, document, organize and validate features). See [Install the Claude skills](#install-the-claude-skills) |
| `features aggregate <FOLDER>` | Aggregate the features JSON reports of several repositories into one dashboard. See [Aggregate several repositories](#aggregate-several-repositories) |

Run `features <command> --help` for the options of a command. The options are:

| Option | Commands | Description |
| ------ | -------- | ----------- |
| `--json` | `scan`, `find-owner`, `owners`, `diff`, `aggregate`, `records` | Output as JSON |
| `--flat` | `scan` | Output features as a flat array instead of nested structure |
| `--description` | `scan` | Include feature descriptions in the output |
| `--coverage` | `scan` | Include coverage information in the output (always on with `--json`, `serve` and `build`) |
| `--port <port>` | `serve` | Change the port (default: 3000) |
| `--allow-origin <origin>` | `serve` | Allow another origin (e.g. `http://localhost:5173`) to read the server's data via CORS. Can be repeated; `*` allows any origin. By default no cross-origin access is allowed and the server only answers requests addressed to `localhost`, `127.0.0.1` or `[::1]` |
| `--experimental` | `serve`, `build` | Enable the experimental parts of the web dashboard (History page, owner pages linked from owner names, project records in Outside features). They are hidden by default and marked as experimental when enabled |
| `--project-name <name>` | `serve`, `build` | Project name shown at the top left of the web dashboard. Overrides `name` in `features.config.toml` and the name detected from the project manifest. See [Name the dashboard](#name-the-dashboard) |
| `--build-dir <path>` | `build`, `aggregate` | Output directory for the static build (default: `build`) |
| `--since <ref>` | `diff` | Compare with the merge base of `<ref>` and `HEAD`, including uncommitted changes (required) |
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

> **Legacy flags.** Before commands existed, actions were flags: `features PATH --serve`, `--build`, `--check`, `--list-owners`, `--find-owner <TARGET>` and `--generate-codeowners`. They still work but are deprecated. Combining two actions (e.g. `--serve --build`) is now an error instead of silently running one of them; `--generate-codeowners` can still be combined with `--serve`, `--build`, `--check` and `--list-owners`. `features PATH --since <ref>` also still works, as an alias of `features diff PATH --since <ref>`.

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
features check ./src
```

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

To choose another name, add a `features.config.toml` at the root of the repository:

```toml
name = "Acme Store"
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
