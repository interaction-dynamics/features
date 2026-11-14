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
| `features check [PATH]` | Run validation checks on features (e.g., duplicate names) |
| `features diff [PATH] --since <ref>` | List the features touched since `<ref>`, with their owners. See [Ping the right teams on every pull request](../README.md#ping-the-right-teams-on-every-pull-request) |
| `features aggregate <FOLDER>` | Aggregate the features JSON reports of several repositories into one dashboard. See [Aggregate several repositories](#aggregate-several-repositories) |

Run `features <command> --help` for the options of a command. The options are:

| Option | Commands | Description |
| ------ | -------- | ----------- |
| `--json` | `scan`, `find-owner`, `owners`, `diff`, `aggregate` | Output as JSON |
| `--flat` | `scan` | Output features as a flat array instead of nested structure |
| `--description` | `scan` | Include feature descriptions in the output |
| `--coverage` | `scan` | Include coverage information in the output (always on with `--json`, `serve` and `build`) |
| `--port <port>` | `serve` | Change the port (default: 3000) |
| `--allow-origin <origin>` | `serve` | Allow another origin (e.g. `http://localhost:5173`) to read the server's data via CORS. Can be repeated; `*` allows any origin. By default no cross-origin access is allowed and the server only answers requests addressed to `localhost`, `127.0.0.1` or `[::1]` |
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
| `--data <file>` | `scan`, `serve`, `build`, `check`, `aggregate` | Merge data from external tools (analytics, errors, issues, ...) into the features, matched by feature key. Can be repeated; later files override earlier ones. See [Import data from external tools](#import-data-from-external-tools) |

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

### List the features and owners touched by a change

```bash
features diff ./src --since origin/master
```

Add `--markdown` for a pull request comment or `--owners-only` to request reviews. See the [GitHub Actions example](../README.md#comment-on-pull-requests).

### Export a JSON report

```bash
features scan ./src --json > features.json
```

### Publish a static dashboard

```bash
features build ./src --build-dir ./dashboard --strip-emails
```

The `dashboard` folder can be deployed to any static host. See the [GitHub Pages example](../README.md#publish-the-dashboard-to-github-pages).

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
- Features are **not** merged when merging them would create a loop (e.g. `B > C` in one repository and `C > B` in another) or when they have different parents in different repositories. They are kept once per repository, and the issue is listed in the `errors` section of `features.json` and on the *Reconciliation issues* page of the dashboard.

The aggregated `features.json` is an object: `{ "repositories": [...], "features": [...], "errors": [...] }`, where `features` has the same structure as the output of `features scan --json`.
