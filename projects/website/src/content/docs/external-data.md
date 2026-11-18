---
title: Import external data
description: Merge data from PostHog, Sentry, Linear or any other tool into the features with --data, matched by feature key, and show it in the dashboard, the JSON report and to AI agents.
---

Give every feature data from your analytics, error tracking, issue tracker or any other tool: weekly users, open errors, open issues, a health status. The data shows on the feature page and as sortable columns of the _Insights_ table of the dashboard.

## 1. Tag the features with a key

External tools refer to a feature by its [key](/docs/readme-format#key-and-links). It defaults to the folder name, slugified (`Checkout Flow` → `checkout-flow`). Set it in the README front matter to keep it stable when the folder is renamed:

```markdown
---
key: checkout
---
```

Then tag your events, errors and issues with it, for example a PostHog event property `feature=checkout`. Run `features scan ./src --json` to see the key of each feature.

## 2. Export the data to JSON

Write a script that queries the tool and maps each feature key to its metrics:

```json
{
  "checkout": { "weekly_users": 12840, "open_errors": 3 },
  "search": { "weekly_users": 310, "status": "healthy" }
}
```

Metric values must be strings, numbers or booleans.

## 3. Pass the files to the CLI

```bash
features build ./src --data usage.json --data errors.json
```

`--data` can be repeated; when two files set the same metric of a feature, the later one wins. It works with `scan`, `serve`, `build`, `check`, `mcp` and `aggregate`. To pass the files every time, list them in [`features.config.yaml`](/docs/configuration):

```yaml
data:
  - reports/usage.json
  - reports/errors.json
```

The files of the configuration come first, then those of the command line.

## What you get

- **In the report**: the metrics are added to the `external` field of every feature with a matching key.
- **In the dashboard**: on the feature page, and as sortable columns of the _Insights_ table. `serve` reads the files again on every rescan.
- **For AI agents**: `get_feature` of the [MCP server](/docs/mcp) returns them in `external`.

Keys that match no feature are reported as a warning. Several features sharing one key are reported by the [`duplicate-keys`](/docs/check-rules#duplicate-keys) rule of `features check`.

## Links to the tools

To link a feature to its dashboard in each tool, add `links` to its front matter. Well-known tools get a label and an icon on the feature page:

```markdown
---
key: checkout
links:
  posthog: https://eu.posthog.com/project/1/dashboard/2
  sentry: https://sentry.io/organizations/acme/issues/?query=feature:checkout
---
```
