---
status: implemented
date: 2026-11-14
---

# Feature key, links and external data import

## Context

Phase 0 of the [integrations roadmap](../roadmap/integrations.md). External tools (PostHog, Sentry, Linear, ...) can only be connected to features if both sides share a stable identifier, and the dashboard has no way to show data that does not come from the code. This change adds the identifier, a convention for links to external tools, and a generic import of external data, without any network call.

## Requirements

### Feature key

- Every feature in the JSON report has a `key`, a stable identifier that apps use to tag events, errors and issues (e.g. PostHog property `feature=checkout`).
- By default, the key is the folder basename, slugified: lowercase, letters, digits, `_` and `-` kept, any other run of characters replaced by a single `-`, no leading or trailing `-`. `Checkout Flow/` → `checkout-flow`. An empty result falls back to `feature`.
- The key can be overridden with `key` in the README front matter or in `FEATURES.toml`. The override is used as written (trimmed). A non-string or empty override is ignored.
- `key` is removed from `meta` (it is a first-class field).
- The key is set in every scan, ownership-only scans included.
- `features check` prints a warning (not an error, so existing CI keeps passing) for every key used by more than one feature, listing their paths and suggesting a `key` override.
- `features aggregate`: a merged feature takes the first non-empty key of its occurrences; reports produced before this change (no key) get the slugified last segment of the feature path.

### Links

- `links` in the README front matter or `FEATURES.toml` is a map of link name to URL:

  ```yaml
  links:
    posthog: https://eu.posthog.com/project/1/dashboard/2
    sentry: https://sentry.io/organizations/acme/issues/?query=feature:checkout
  ```

- It is serialized as a `links` field of the feature (omitted when empty) and removed from `meta`. Entries whose value is not a string are dropped. If `links` is not a map, it stays in `meta` untouched.
- The dashboard shows a *Links* block on the feature page: one button per link, opening in a new tab, with a readable label and an icon for well-known tools (analytics, errors, monitoring, design, issues, docs, chat), and a generic link icon otherwise. Only `http` and `https` URLs are rendered as links.
- `features aggregate` merges the links of all occurrences (first value wins per name).

### External data import

- `--data <FILE>` (repeatable) on `scan`, `serve`, `build`, `check` and `aggregate` merges external data into the report.
- The file is a JSON object mapping feature keys to an object of metrics; metric values are strings, numbers or booleans:

  ```json
  {
    "checkout": { "weekly_users": 1200, "open_errors": 3 },
    "search": { "weekly_users": 80, "status": "healthy" }
  }
  ```

- Data is stored in an `external` field of the feature (omitted when empty). With several files, later files override earlier ones metric by metric.
- Every feature with a matching key receives the data (nested features included).
- Keys that match no feature are reported on stderr as a warning; the command still succeeds.
- A missing file, invalid JSON, a top-level value that is not an object, a feature entry that is not an object or a metric value that is not a string, number or boolean is an error naming the file and the offending key.
- `serve` re-reads the files on each rescan, so editing a data file and touching the code refreshes the dashboard.
- Ownership-only commands (`codeowners`, `owners`, `find-owner`, `diff`) ignore `--data`.
- `features aggregate --data`: the data is applied to the aggregated features, after merging; external data already present in the reports is merged (first value wins per metric), then overridden by `--data`.
- The dashboard shows the external data on the feature page (*Data* block, one badge per metric, numbers formatted) and one sortable column per metric in the features insights table. Features without a value for the sorted column stay at the bottom in both directions (this applies to every sortable column of the table).

## Out of scope

- Fetching data from any tool (PostHog, Sentry, ...): later phases of the roadmap.
- Path-map export (`--path-map`), Sentry ownership rules, any config file.
- Filtering the insights table by external metrics.
- Metric names containing a `.`: they are imported and shown, but sorting their column in the insights table does not work (the sort follows dotted paths).
- Merging aggregated features by key instead of name.

## Plan

1. CLI model: add `key`, `links` and `external` to `Feature` and `AggregatedFeature`.
2. Scanner: compute the key and extract `key` and `links` from the metadata when a feature directory is processed.
3. New `external_data` module: load and validate data files, apply them to a feature tree (both feature types), return unmatched keys.
4. Wire `--data` into `ScanArgs` and `ScanConfig` (applied at the end of a full scan) and into `AggregateArgs`.
5. `features check`: duplicate key warning. `features aggregate`: merge keys, links and external data.
6. Web: types and schema, *Key*, *Links* and *Data* blocks on the feature page, external metric columns in the insights table.
7. Docs: README, `docs/readme-format.md`, `docs/cli.md`, roadmap status. Tests for each step.
