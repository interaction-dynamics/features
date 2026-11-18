---
title: Configuration file
description: Set every features-cli option once in features.config.yaml, so a team shares the same settings and commands stay short. Keys, sections, precedence, lookup and a full example.
---

Every command-line option can be set in a `features.config.yaml` file, usually at the root of the repository. The team shares the same settings, and the commands in scripts, CI and on laptops stay short:

```yaml
# features.config.yaml
path: src
ignore-path: [src/generated]

serve:
  port: 4000

diff:
  since: origin/main
```

With this file, `features serve` scans `./src` on port 4000 and ignores `src/generated`, and `features diff` compares with `origin/main` without `--since`.

## Keys

Keys are the long names of the options without `--`, in kebab-case: `--skip-changes` is `skip-changes`, `--build-dir` is `build-dir`. Flags take `true` or `false`, repeatable options take a list.

- **Options shared by the commands go at the top level**: the scanning options, the CODEOWNERS file, the project name.
- **Options of one command go in a section named after it**: `serve.port`, `diff.since`, `check.rules`.

### Top level

| Key                    | Option                   | Value                                                   |
| ---------------------- | ------------------------ | ------------------------------------------------------- |
| `name`                 | `--project-name`         | Name of the dashboard                                   |
| `path`                 | `[PATH]`                 | Folder to scan when the command line gives none         |
| `skip-changes`         | `--skip-changes`         | `true` or `false`                                       |
| `ignore-path`          | `--ignore-path`          | List of folders                                         |
| `include-skipped-dirs` | `--include-skipped-dirs` | `true` or `false`                                       |
| `coverage-dir`         | `--coverage-dir`         | Folder                                                  |
| `project-dir`          | `--project-dir`          | Folder                                                  |
| `strip-emails`         | `--strip-emails`         | `true` or `false`                                       |
| `data`                 | `--data`                 | List of JSON files                                      |
| `codeowners-path`      | `--codeowners-path`      | File                                                    |
| `codeowners-prefix`    | `--codeowners-prefix`    | Text. Quote it: `"@acme/"`, YAML reserves a leading `@` |

### Sections

| Section      | Keys                                                                          |
| ------------ | ----------------------------------------------------------------------------- |
| `scan`       | `json`, `flat`, `description`, `coverage`                                     |
| `serve`      | `port`, `allow-origin` (list), `experimental`, `generate-codeowners`          |
| `build`      | `build-dir`, `experimental`, `generate-codeowners`                            |
| `check`      | `generate-codeowners`, `format` (`text`, `json` or `github`), `json`, `rules` |
| `diff`       | `since`, `committed-only`, and one of `json`, `markdown` or `owners-only`     |
| `find-owner` | `json`                                                                        |
| `owners`     | `json`                                                                        |
| `records`    | `json`                                                                        |
| `aggregate`  | `build-dir`, `json`, `data` (list)                                            |

`check.rules` sets the severity and the options of each check rule; see [Check rules](/docs/check-rules#configure-the-rules).

## Full example

Every key, with a value other than its default where that makes sense:

```yaml
# features.config.yaml
name: Acme Store # --project-name
path: src # folder to scan when the command line gives none
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
codeowners-prefix: '@acme/' # quoted: YAML reserves a leading @

scan:
  json: false
  flat: false
  description: true
  coverage: true

serve:
  port: 4000
  allow-origin: ['http://localhost:5173']
  experimental: true
  generate-codeowners: false

build:
  build-dir: dashboard
  experimental: true
  generate-codeowners: true

check:
  generate-codeowners: true
  format: text # text, json or github
  rules:
    missing-owner: error
    min-coverage:
      severity: error
      threshold: 85

diff:
  since: origin/main
  committed-only: false
  markdown: true # or json, or owners-only: only one

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

## How options are combined

- **The command line wins.** An option given on the command line overrides the file, and the file overrides the built-in default. This holds even when the command line repeats the default: `features serve --port 3000` uses port 3000 whatever the file says.
- **Lists are combined.** For `ignore-path`, `data` and `allow-origin`, the values of the file come first, then those of the command line.
- **One diff format.** `json`, `markdown` and `owners-only` are one choice: the file can set only one, and a format given on the command line replaces it.
- **`diff` without `--since`.** `features diff` works without `--since` when `diff.since` is set. Without either, it fails with a message naming both.
- **Paths are relative to the file.** `path: src` is the `src` folder next to `features.config.yaml`, wherever the command runs from.

The arguments that name what to act on stay on the command line: the `TARGET` of `find-owner`, the kind, feature and title of `new`, and the `FOLDER` of `aggregate`. So does `--list-rules`.

## Where the file is found

1. The CLI looks for `features.config.yaml` in the scanned folder, or in the current directory when none is given.
2. Then in each parent folder, up to the root of the git repository.
3. The nearest file wins. An empty file, or one with only comments, is valid and sets nothing.

Two global options change this. They work before or after the command, and can't be combined:

```bash
features check --config ci/features.config.yaml   # read another file, for example in CI
features serve --no-config                        # ignore the file
```

`--no-config` is also the way to turn off a flag the file turns on: there is no `--no-<flag>` option.

## Errors

The file is strict, so a typo doesn't silently do nothing. An unknown key or a value of the wrong type stops the command with an error that names the file and the line. With this file, every command fails and points to `prot` on line 2:

```yaml
serve:
  prot: 4000 # typo: unknown field `prot`
```

The same goes for an unknown check rule or rule option in `check.rules`, and for an invalid `serve.allow-origin`.

## Migrating from features.config.toml

Earlier versions read the dashboard name from a `features.config.toml` file that held only `name`. That file is no longer read: when the CLI finds one, it prints a warning. Move the name to `features.config.yaml`, in YAML syntax, and delete the TOML file.

```toml
# features.config.toml (before)
name = "Acme Store"
```

```yaml
# features.config.yaml (after)
name: Acme Store
```
