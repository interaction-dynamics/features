# Configuration file

`features` works without any configuration. When you want to change how it behaves for a project, for example which checks fail your CI, put the settings in a YAML configuration file instead of repeating command-line options. Every command-line option can be set there.

- [Where the file is](#where-the-file-is)
- [Format](#format)
- [Options](#options)
  - [Shared options](#shared-options)
  - [Command options](#command-options)
  - [Precedence](#precedence)
- [`check` section](#check-section)
  - [Severities](#severities)
  - [Rules and their options](#rules-and-their-options)
  - [Command-line overrides](#command-line-overrides)
- [Full example](#full-example)
- [Errors](#errors)
- [`.features.yml` vs `FEATURES.toml`](#featuresyml-vs-featurestoml)

## Where the file is

The file is named `.features.yml` (`.features.yaml` also works). `features` uses the first one it finds in:

1. the scanned folder, e.g. `./src` in `features check ./src`
2. the `--project-dir` folder, if given
3. the current folder

In each folder, `.features.yml` wins over `.features.yaml`. If no file is found, the defaults apply.

To use another file, pass `--config <path>`. Every command accepts it:

```bash
features check ./src --config ./config/features.yml
```

An explicit `--config` file must exist; otherwise the command fails. When `--config` is given, no other file is searched for.

Most projects keep the file at the root of the repository and run `features` from there:

```
my-project/
├── .features.yml
└── src/
    └── features/
        └── ...
```

## Format

The file is a YAML mapping. Each key is a command-line option, without the leading `--` and with `_` instead of `-`: `--skip-changes` becomes `skip_changes`. Options shared by several commands are at the top level; the options of a single command are in a section named after it:

```yaml
path: src              # the folder to scan, for every command
skip_changes: true     # --skip-changes, for every command

serve:
  port: 4000           # --port, for `features serve`

check:
  format: github       # --format, for `features check`
  rules:
    missing-owner: error
```

An empty file, or a file with only comments, is valid. Unknown keys are errors, so a typo doesn't go unnoticed.

YAML 1.2 is used: `off`, `yes` and `no` are plain strings, not booleans, so `stale-flag: off` works without quotes. Flags take `true` or `false`.

Relative paths are relative to the folder of the configuration file, not to the folder `features` runs from.

## Options

### Shared options

These keys are at the top level and apply to every command that has the option.

| Key | Option | Type |
| --- | ------ | ---- |
| `path` | the `PATH` argument: the folder to scan | path |
| `skip_changes` | `--skip-changes` | `true` / `false` |
| `ignore_path` | `--ignore-path` | list of paths |
| `include_skipped_dirs` | `--include-skipped-dirs` | `true` / `false` |
| `coverage_dir` | `--coverage-dir` | path |
| `project_dir` | `--project-dir` | path |
| `strip_emails` | `--strip-emails` | `true` / `false` |
| `codeowners_path` | `--codeowners-path` | path |
| `codeowners_prefix` | `--codeowners-prefix` (also used by `features diff`) | string |

### Command options

| Section | Key | Option | Type |
| ------- | --- | ------ | ---- |
| `scan` | `json`, `flat`, `description`, `coverage` | `--json`, `--flat`, `--description`, `--coverage` | `true` / `false` |
| `serve` | `port` | `--port` | number |
| `serve` | `allow_origin` | `--allow-origin` | list of origins |
| `serve` | `generate_codeowners` | `--generate-codeowners` | `true` / `false` |
| `build` | `build_dir` | `--build-dir` | path |
| `build` | `generate_codeowners` | `--generate-codeowners` | `true` / `false` |
| `owners` | `json` | `--json` | `true` / `false` |
| `find_owner` | `json` | `--json` | `true` / `false` |
| `diff` | `since` | `--since` | git ref |
| `diff` | `committed_only` | `--committed-only` | `true` / `false` |
| `diff` | `json`, `markdown`, `owners_only` | `--json`, `--markdown`, `--owners-only` (only one) | `true` / `false` |
| `check` | `format` | `--format` | `text`, `json` or `github` |
| `check` | `json` | `--json` | `true` / `false` |
| `check` | `generate_codeowners` | `--generate-codeowners` | `true` / `false` |
| `check` | `rules` | `--rule`, `--enable-rule`, `--disable-rule` | see [below](#check-section) |

The deprecated flag forms (`features ./src --serve`...) read the same sections as their commands. A few options stay on the command line only: the target of `features find-owner`, `--list-rules`, `--config` itself, `--version` and `--help`.

### Precedence

An option given on the command line wins over the file, which wins over the default value:

- Values (`port`, `since`, `format`, paths...) from the command line replace the file's.
- Lists (`ignore_path`, `allow_origin`) add up: the command line values are used together with the file's.
- Flags can only be turned on from the command line: `skip_changes: true` in the file can't be turned off for one run. Leave such a flag out of the file if you need both.
- The output format is replaced as a whole: `--json` on the command line wins over `diff.markdown: true` or `check.format: github` in the file.

```yaml
# .features.yml
path: src
diff:
  since: origin/main
  markdown: true
```

```bash
features diff                    # features touched since origin/main, as markdown
features diff --since v1.0 --json  # since v1.0, as JSON
```

## `check` section

The `check` section configures [`features check`](../README.md#checks). Besides the output options above, its `rules` key maps rule ids to their settings:

```yaml
check:
  rules:
    missing-owner: error          # a severity alone
    min-coverage:                 # a severity and options
      severity: warn
      threshold: 70
    stale-flag:                   # options alone keep the default severity
      max_age_days: 30
```

Rules missing from the file keep their default severity and options.

### Severities

| Severity | Effect |
| -------- | ------ |
| `error` | The rule runs; its findings are reported and make `features check` exit with code 1 |
| `warn` (or `warning`) | The rule runs; its findings are reported but don't fail the check |
| `off` | The rule doesn't run |

### Rules and their options

Run `features check --list-rules` to see every rule with the severity that applies to your project.

#### `duplicate-names`

Two or more features have the same name. Default severity: `error`. No options.

#### `missing-owner`

A feature has no owner (see [how to add an owner](../FAQ.md#how-can-i-add-an-owner)). Default severity: `warn`.

| Option | Default | Description |
| ------ | ------- | ----------- |
| `allow_inherited` | `true` | Whether an owner inherited from a parent feature is enough. Set it to `false` to require every feature to declare its own owner |

#### `missing-readme`

A large feature has no `README.md`, `README.mdx` or `FEATURES.toml`. Default severity: `off`.

| Option | Default | Description |
| ------ | ------- | ----------- |
| `max_files_without_readme` | `10` | Features with more files than this need a README. Files of nested features don't count |

#### `min-coverage`

A feature's line coverage is below a threshold. Only features with coverage data are checked (see [how to add a test coverage report](../FAQ.md#how-can-i-add-a-test-coverage-report)). Coverage reports are only read when this rule is on. Default severity: `off`.

| Option | Default | Description |
| ------ | ------- | ----------- |
| `threshold` | `80` | Minimum line coverage, in percent (0 to 100) |

#### `stale-flag`

A feature flag has been around for too long, based on the `introduced_on` property (`YYYY-MM-DD`) of its [metadata annotation](../README.md#feature-metadata-annotations):

```typescript
// --feature-flag name: new-checkout, introduced_on: 2025-01-15
```

Flags with an invalid `introduced_on` date are reported too. Flags without one are ignored. Default severity: `warn`.

| Option | Default | Description |
| ------ | ------- | ----------- |
| `max_age_days` | `90` | Maximum age of a flag, in days |
| `metadata_keys` | `[flag]` | Metadata keys to look at: `flag` for `--feature-flag` annotations, `experiment` for `--feature-experiment`, and so on |

#### `cross-feature-import`

A feature imports code from a sibling feature, i.e. a feature that is neither its parent nor its child. Each import is reported with its file and line. Default severity: `off`.

| Option | Default | Description |
| ------ | ------- | ----------- |
| `allow` | `[]` | Paths of features any feature may import from, relative to the scanned folder, e.g. shared features. Their nested features are allowed too |

### Command-line overrides

Command-line options win over the file, which wins over the defaults. The options are applied in this order:

1. `--rule <id>=<severity>` sets the severity of a rule
2. `--enable-rule <id>` turns a rule that is `off` into `error`; a rule that is already on keeps its severity
3. `--disable-rule <id>` turns a rule `off`

Each option can be repeated:

```bash
features check ./src --rule min-coverage=error --enable-rule cross-feature-import --disable-rule stale-flag
```

Options of rules (thresholds, allowed paths...) can only be set in the file.

## Full example

```yaml
# .features.yml
path: src
skip_changes: true        # checks don't need the git history
ignore_path:
  - src/generated
coverage_dir: coverage

check:
  format: github          # annotate pull requests in GitHub Actions
  rules:
    # Fail the build on duplicates and on features nobody owns
    duplicate-names: error
    missing-owner:
      severity: error
      allow_inherited: true

    # Ask for documentation once a feature grows
    missing-readme:
      severity: warn
      max_files_without_readme: 15

    # Report poorly tested features without failing the build
    min-coverage:
      severity: warn
      threshold: 70

    # Clean up flags after two months
    stale-flag:
      severity: warn
      max_age_days: 60
      metadata_keys:
        - flag
        - experiment

    # Keep features independent, except for the shared ones
    cross-feature-import:
      severity: error
      allow:
        - features/shared
        - features/design-system
```

The [`examples/check-rules`](../examples/check-rules) project has a configuration where every rule finds something.

## Errors

`features` checks the configuration before scanning and stops with an error naming the file when:

- the file is not valid YAML, or its top level is not a mapping
- a key is unknown, at the top level or in a section (the error lists the expected keys)
- a value has the wrong type, e.g. `port: high` or `format: xml`
- `diff` turns on more than one of `json`, `markdown` and `owners_only`
- a rule id is unknown (the error lists the known ids)
- a severity is not `error`, `warn` or `off`
- a rule has an unknown option, or an option of the wrong type
- an option is out of range, e.g. `threshold: 120` or a negative `max_age_days`

```
Error: invalid config `./.features.yml`

Caused by:
    unknown field `prot`, expected one of `port`, `allow_origin`, `generate_codeowners`
```

```
Error: invalid check configuration (config file `./.features.yml`)

Caused by:
    0: invalid `check.rules` section in the config file
    1: unknown rule `missing-owners` in check.rules. Known rules: duplicate-names, missing-owner, missing-readme, min-coverage, stale-flag, cross-feature-import
```

## `.features.yml` vs `FEATURES.toml`

The two files are unrelated:

| File | Scope | Purpose |
| ---- | ----- | ------- |
| `.features.yml` | The whole project, usually at its root | Configures how `features` behaves |
| `FEATURES.toml` | A single feature, in its folder | Describes the feature (name, owner, description, metadata), like a [README front matter](./readme-format.md) |
