---
title: Check rules
description: The rules run by features check, their default severity and options, how to turn them on, off or into errors from the command line or features.config.yaml, and how to run them in GitHub Actions.
---

`features check` runs a set of rules on the features of the project. Each rule has a kebab-case id and a severity:

- **`error`**: a finding fails the check: the command exits with code `1`.
- **`warn`**: findings are printed, the check still passes.
- **`off`**: the rule doesn't run.

```bash
features check ./src
```

```text
error[duplicate-names] 1 finding
  features/payments: Duplicate feature name 'payments' found in 2 locations: features/payments, legacy/features/payments
warn[missing-owner] 2 findings
  features/search: Feature has no owner
  features/checkout/cart: Feature has no owner

1 error(s), 2 warning(s) (5 rules run, disabled: missing-readme, min-coverage, cross-feature-import).
```

## Rules

| Rule                   | Default | Finds                                                                       | Options                         |
| ---------------------- | ------- | --------------------------------------------------------------------------- | ------------------------------- |
| `duplicate-names`      | `error` | Two features with the same name                                             |                                 |
| `duplicate-keys`       | `warn`  | Several features sharing a [feature key](/docs/readme-format#key-and-links) |                                 |
| `record-issues`        | `warn`  | Problems in specs and decision records                                      |                                 |
| `missing-owner`        | `warn`  | A feature without owner                                                     | `allow-inherited`               |
| `stale-flag`           | `warn`  | Feature-flag annotations older than a maximum age                           | `max-age-days`, `metadata-keys` |
| `missing-readme`       | `off`   | A large feature without README                                              | `max-files-without-readme`      |
| `min-coverage`         | `off`   | Line coverage below a threshold                                             | `threshold`                     |
| `cross-feature-import` | `off`   | A feature importing from a sibling feature                                  | `allow`                         |

### duplicate-names

Two features have the same name. Names identify features in the dashboard, in `diff` and when [aggregating repositories](/docs/aggregate), so this is an error by default. Rename one of the folders, or set another title in its README.

### duplicate-keys

Several features share the same [feature key](/docs/readme-format#key-and-links). It matters for [`--data` imports](/docs/external-data), which match data to features by key: the data of that key goes to all of them. Set a unique `key` in their README front matter.

### record-issues

Problems in the [specs and decision records](/docs/specs-decisions):

- a broken link between a spec and a decision (`decisions`, `spec`, `supersedes`),
- a record without a `status`,
- a `superseded` record that nothing replaces,
- an `implemented` spec with open tasks.

### missing-owner

A feature has no `owner`.

| Option            | Default | Description                                                                                                           |
| ----------------- | ------- | --------------------------------------------------------------------------------------------------------------------- |
| `allow-inherited` | `true`  | Accept a feature that inherits the owner of its parent feature. With `false`, every feature needs an owner of its own |

### stale-flag

A [feature-flag annotation](/docs/folder-structure#annotations) was introduced longer ago than the maximum age, so the flag is probably due for removal:

```ts
// --feature-flag name: new-login, introduced_on: 2025-01-01
```

An `introduced_on` that isn't a `YYYY-MM-DD` date is reported too. Annotations without `introduced_on` are ignored.

| Option          | Default  | Description                                                           |
| --------------- | -------- | --------------------------------------------------------------------- |
| `max-age-days`  | `90`     | Maximum age of a flag, in days                                        |
| `metadata-keys` | `[flag]` | Annotation keys that hold feature flags (`flag` for `--feature-flag`) |

### missing-readme

A feature with many files has no `README.md`, `README.mdx` or `FEATURES.toml` to describe it.

| Option                     | Default | Description                                       |
| -------------------------- | ------- | ------------------------------------------------- |
| `max-files-without-readme` | `10`    | Number of files a feature can have without README |

### min-coverage

The line coverage of a feature is below the threshold. Only features with [coverage data](/cli#coverage) are checked, so run the tests with coverage before `features check`.

| Option      | Default | Description                       |
| ----------- | ------- | --------------------------------- |
| `threshold` | `80`    | Minimum line coverage, in percent |

### cross-feature-import

A feature imports code from a sibling feature, which couples features that should stay independent. The finding points to the file and line of the import.

| Option  | Default | Description                                                                                                                                                |
| ------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allow` | `[]`    | Feature paths any feature may import from, such as `features/shared`. Paths are relative to the scanned folder, as in the `path` of `features scan --json` |

## Choose the rules from the command line

```bash
features check --rule missing-owner=error   # set a severity: error, warn or off
features check --enable-rule min-coverage   # off → error
features check --disable-rule stale-flag    # → off
features check --list-rules                 # print the rules and their effective severity, then exit
```

`--rule`, `--enable-rule` and `--disable-rule` can be repeated. Rule options are only set in the configuration file. These options, like `--format` and `--json`, belong to the `features check` command: the deprecated `features PATH --check` flag doesn't take them.

## Configure the rules

In [`features.config.yaml`](/docs/configuration), `check.rules` maps a rule id to a severity, or to an object with a `severity` and the options of the rule:

```yaml
# features.config.yaml
check:
  format: github
  rules:
    missing-owner: error # severity only
    min-coverage: # severity and options
      severity: error
      threshold: 85
    cross-feature-import:
      severity: error
      allow: [features/shared]
    stale-flag:
      max-age-days: 180
```

An unknown rule or option is an error that names the file and the line. Rules you don't list keep their default.

**Precedence**: the built-in defaults, then `check.rules` in the file, then the command-line flags. `features check --disable-rule missing-owner` turns the rule off even when the file sets it to `error`.

## Output formats

| Format   | Option                      | Output                                                                                                                                                                    |
| -------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `text`   | default                     | Findings grouped by rule, then a summary                                                                                                                                  |
| `json`   | `--format json` or `--json` | A JSON document with the findings and a summary                                                                                                                           |
| `github` | `--format github`           | [GitHub Actions workflow commands](https://docs.github.com/actions/reference/workflow-commands-for-github-actions), shown as annotations on the files of the pull request |

With `github`, each finding becomes an `::error` or `::warning` line that points to the file (and line, when known) of the finding, or to the README of the feature:

```text
::error file=src/features/payments/README.md,title=features check%3A duplicate-names::Duplicate feature name 'payments' found in 2 locations: features/payments, legacy/features/payments
::warning file=src/features/search/README.md,title=features check%3A missing-owner::Feature has no owner
```

## Run the checks in GitHub Actions

```yaml
# .github/workflows/features-check.yml
name: Features check
on:
  pull_request:
  push:
    branches: [main]

permissions:
  contents: read

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      # Only needed for min-coverage: produce the coverage reports first.
      # - run: npm ci && npx jest --coverage

      - name: Check features
        run: npx features-cli@latest check ./src --format github
```

The job fails when a rule set to `error` finds something, and every finding shows up as an annotation on the changed files. Set `format: github` in `features.config.yaml` instead of passing `--format` to use it everywhere, or keep it on the command line so local runs print text. See [Pull requests and CI](/docs/pull-requests-ci) for the other CI recipes.
