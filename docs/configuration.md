# Configuration file

`features` works without any configuration. When you want to change how it behaves for a project, for example which checks fail your CI, put the settings in a YAML configuration file instead of repeating command-line options.

- [Where the file is](#where-the-file-is)
- [Format](#format)
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

The file is a YAML mapping with one section per part of the tool:

```yaml
check:
  # settings of `features check`
```

Today only the `check` section is read. Other top-level keys are ignored, so the file can grow new sections without breaking older versions of `features`. An empty file, or a file with only comments, is valid.

YAML 1.2 is used: `off`, `yes` and `no` are plain strings, not booleans, so `stale-flag: off` works without quotes.

## `check` section

The `check` section configures [`features check`](../README.md#checks). It has a single key, `rules`, mapping rule ids to their settings:

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
check:
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
- `check` contains a key other than `rules`
- a rule id is unknown (the error lists the known ids)
- a severity is not `error`, `warn` or `off`
- a rule has an unknown option, or an option of the wrong type
- an option is out of range, e.g. `threshold: 120` or a negative `max_age_days`

```
Error: invalid check configuration (config file `/path/to/project/.features.yml`)

Caused by:
    0: invalid `check` section in the config file
    1: unknown rule `missing-owners` in check.rules. Known rules: duplicate-names, missing-owner, missing-readme, min-coverage, stale-flag, cross-feature-import
```

## `.features.yml` vs `FEATURES.toml`

The two files are unrelated:

| File | Scope | Purpose |
| ---- | ----- | ------- |
| `.features.yml` | The whole project, usually at its root | Configures how `features` behaves |
| `FEATURES.toml` | A single feature, in its folder | Describes the feature (name, owner, description, metadata), like a [README front matter](./readme-format.md) |
