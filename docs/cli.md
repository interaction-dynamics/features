# CLI reference

```bash
features <path> [options]
```

`<path>` is the folder to scan for features (for example `./src` or `.`).

## Options

| Option | Description |
| ------ | ----------- |
| `--serve` | Start an HTTP server with the web dashboard and live reload |
| `--port <port>` | Change the port (default: `3000`). Use with `--serve` |
| `--build` | Build a static version of the web dashboard (for GitHub Pages, S3, etc.) |
| `--build-dir <path>` | Output directory for the static build (default: `build`) |
| `--json` | Output features as JSON |
| `--flat` | Output features as a flat array instead of a nested structure |
| `--description` | Include feature descriptions in the output (automatically enabled for `--serve`, `--build`, and `--json`) |
| `--coverage` | Include coverage information in the output (automatically enabled for `--serve`, `--build`, and `--json`) |
| `--coverage-dir <path>` | Use a custom coverage directory (overrides the automatic search) |
| `--list-owners` | Display only the unique list of owners |
| `--find-owner <path>` | Find the owner of a specific file or folder |
| `--check` | Run validation checks on features (e.g., duplicate names). Exits with a non-zero code on errors |
| `--generate-codeowners` | Generate or update a `CODEOWNERS` file from feature ownership |
| `--project-dir <path>` | Project directory used for `CODEOWNERS` generation and additional coverage search locations |
| `--codeowners-path <path>` | Custom path and filename for the `CODEOWNERS` file (default: `CODEOWNERS` in the project directory) |
| `--codeowners-prefix <prefix>` | Custom prefix for owner names in the `CODEOWNERS` file and in `--since --markdown` / `--owners-only` output (default: `@`) |
| `--since <ref>` | Diff mode: list the features touched since the merge base of `<ref>` and `HEAD`, including uncommitted changes. See [Ping the right teams on every pull request](../README.md#ping-the-right-teams-on-every-pull-request) |
| `--committed-only` | With `--since`: ignore uncommitted and untracked files |
| `--markdown` | With `--since`: output a markdown table for pull request comments (use `--json` for JSON) |
| `--owners-only` | With `--since`: print only the unique owners of the touched features, one per line |
| `--skip-changes` | Skip computing git commit history (faster for large repos) |
| `--ignore-path <path>` | Ignore a folder and all its subfolders when scanning. Can be repeated |
| `--include-skipped-dirs` | Also scan folders skipped by default (`node_modules`) for features, stats and imports |
| `-V`, `--version` | Print version information |

## Recipes

### Explore a project in the browser

```bash
npx features-cli@latest ./src --serve
```

### Generate a CODEOWNERS file

```bash
features ./src --generate-codeowners --project-dir .
```

See the [FAQ](../FAQ.md#how-can-i-generate-codeowners-file) for more options.

### Validate features in CI

```bash
features ./src --check
```

### List the features and owners touched by a change

```bash
features ./src --since origin/master
```

Add `--markdown` for a pull request comment or `--owners-only` to request reviews. See the [GitHub Actions example](../README.md#comment-on-pull-requests-with-github-actions).

### Export a JSON report

```bash
features ./src --json > features.json
```

### Publish a static dashboard

```bash
features ./src --build --build-dir ./dashboard
```

The `dashboard` folder can be deployed to any static host. See the [GitHub Pages example](../README.md#share-a-dashboard-with-your-team).
