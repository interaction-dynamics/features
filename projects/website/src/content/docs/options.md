---
title: Options
description: Every features-cli command-line option, the commands that accept it and its key in features.config.yaml.
---

Options have a long name and take a value when they show one. Run `features <command> --help` to see the options of a command. Each option can also be set in [`features.config.yaml`](/docs/configuration): the **Config key** column gives its place in the file, where `serve.port` means the `port` key of the `serve` section. On the [CLI page](/cli#options), the same options can be filtered by command.

## Scanning

Accepted by every command that scans the project.

| Option                   | Config key             | Description                                                                                                                                                                                                          |
| ------------------------ | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--skip-changes`         | `skip-changes`         | Skip the git commit history (faster for large repositories)                                                                                                                                                          |
| `--ignore-path <path>`   | `ignore-path`          | Ignore a folder and all its subfolders. Repeatable                                                                                                                                                                   |
| `--include-skipped-dirs` | `include-skipped-dirs` | Also scan the folders skipped by default (`node_modules`) for features, stats and imports                                                                                                                            |
| `--coverage-dir <path>`  | `coverage-dir`         | Read coverage reports from this folder instead of searching for them                                                                                                                                                 |
| `--project-dir <path>`   | `project-dir`          | Project directory, for CODEOWNERS paths and additional coverage locations                                                                                                                                            |
| `--strip-emails`         | `strip-emails`         | Remove commit author emails from the output (JSON, `serve`, `build`); names are kept                                                                                                                                 |
| `--data <file>`          | `data`                 | Merge data from external tools into the features, by feature key. Repeatable; later files win. Accepted by `scan`, `serve`, `build`, `check`, `mcp` and `aggregate`. See [Import external data](/docs/external-data) |

Without `PATH` on the command line, the folder in `path` of the configuration file is scanned, else the current directory.

## Output

| Option              | Commands                                                                | Config key         | Description                                                                            |
| ------------------- | ----------------------------------------------------------------------- | ------------------ | -------------------------------------------------------------------------------------- |
| `--json`            | `scan`, `find-owner`, `owners`, `diff`, `records`, `aggregate`, `check` | `<command>.json`   | Print JSON                                                                             |
| `--flat`            | `scan`                                                                  | `scan.flat`        | A flat array of features instead of a tree                                             |
| `--description`     | `scan`                                                                  | `scan.description` | Include the description of each feature                                                |
| `--coverage`        | `scan`                                                                  | `scan.coverage`    | Include coverage (always on with `--json`, `serve` and `build`)                        |
| `--format <format>` | `check`                                                                 | `check.format`     | `text` (default), `json` or `github` annotations. See [Check rules](/docs/check-rules) |

## Dashboard

| Option                    | Commands             | Config key                                 | Description                                                                                                                                                                                                       |
| ------------------------- | -------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--port <port>`           | `serve`              | `serve.port`                               | Port of the server (default: `3000`)                                                                                                                                                                              |
| `--allow-origin <origin>` | `serve`              | `serve.allow-origin`                       | Let another origin (`http://localhost:5173`) read the data via CORS. Repeatable; `*` allows any. By default there is no cross-origin access and only requests to `localhost`, `127.0.0.1` or `[::1]` are answered |
| `--experimental`          | `serve`, `build`     | `serve.experimental`, `build.experimental` | Show the experimental parts of the dashboard (History page, owner pages, project records in Outside features)                                                                                                     |
| `--project-name <name>`   | `serve`, `build`     | `name`                                     | Name shown at the top left of the dashboard. See [Name the dashboard](/docs/dashboard#name-the-dashboard)                                                                                                         |
| `--build-dir <path>`      | `build`, `aggregate` | `build.build-dir`, `aggregate.build-dir`   | Output folder of the static build (default: `build`)                                                                                                                                                              |

## Diff

| Option             | Config key            | Description                                                                                                      |
| ------------------ | --------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `--since <ref>`    | `diff.since`          | Compare with the merge base of `<ref>` and `HEAD`, uncommitted changes included. Required unless set in the file |
| `--committed-only` | `diff.committed-only` | Ignore uncommitted and untracked files                                                                           |
| `--markdown`       | `diff.markdown`       | A markdown table for pull request comments                                                                       |
| `--owners-only`    | `diff.owners-only`    | Only the unique owners, one per line                                                                             |

`--json`, `--markdown` and `--owners-only` are one choice: give only one. A format on the command line replaces the one of the file.

## CODEOWNERS

| Option                         | Commands                                           | Config key                                        | Description                                                                                       |
| ------------------------------ | -------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `--generate-codeowners`        | `serve`, `build`, `check`                          | `serve.generate-codeowners`, `build.…`, `check.…` | Also generate or update the CODEOWNERS file                                                       |
| `--codeowners-path <path>`     | `codeowners`, with `--generate-codeowners`         | `codeowners-path`                                 | Path and name of the file (default: `CODEOWNERS` in the project directory)                        |
| `--codeowners-prefix <prefix>` | `codeowners`, `diff`, with `--generate-codeowners` | `codeowners-prefix`                               | Prefix of the owner names in CODEOWNERS and in `diff --markdown` / `--owners-only` (default: `@`) |

## Check rules

Only for `features check`. See [Check rules](/docs/check-rules).

| Option                   | Config key                | Description                                                      |
| ------------------------ | ------------------------- | ---------------------------------------------------------------- |
| `--rule <id>=<severity>` | `check.rules.<id>`        | Set the severity of a rule: `error`, `warn` or `off`. Repeatable |
| `--enable-rule <id>`     | `check.rules.<id>: error` | Turn a rule on as an error. Repeatable                           |
| `--disable-rule <id>`    | `check.rules.<id>: off`   | Turn a rule off. Repeatable                                      |
| `--list-rules`           | none                      | Print the rules with their effective severity, then exit         |

## Configuration

| Option            | Description                                                                                          |
| ----------------- | ---------------------------------------------------------------------------------------------------- |
| `--config <file>` | Read this YAML file instead of the `features.config.yaml` found in the scanned folder or its parents |
| `--no-config`     | Ignore the configuration file: only the command line counts                                          |

Both work before or after the command, and can't be combined. See [Configuration file](/docs/configuration).
