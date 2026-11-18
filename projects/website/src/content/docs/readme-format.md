---
title: README format
description: The README front matter features-cli reads for each feature (feature, owner, status, key, links and custom properties), and the FEATURES.toml alternative.
---

A feature is described by a `README.md` (or `README.mdx`) at the root of its folder. Its title names the feature, the first paragraph describes it, and its front matter holds the metadata. Every property is optional, and you can add your own.

```markdown
---
feature: true # required if the folder is not in a `features` folder
owner: team_1
creation_date: 2022-01-01 # overrides the date of the first commit in the feature
status: active # active, deprecated, archived
deprecated: true # true, false
key: checkout # stable identifier for external tools (default: the folder name, slugified)
links: # links to the feature in external tools, shown on the feature page
  posthog: https://eu.posthog.com/project/1/dashboard/2
  sentry: https://sentry.io/organizations/acme/issues/?query=feature:checkout
---

# Checkout

Lets customers review their cart, pay and get a receipt.
```

## Title and description

- The first `#` heading is the name of the feature. Without one, the folder name is used.
- The first paragraph after the title is its description, shown in the dashboard, in `features scan --description` and to AI agents over [MCP](/docs/mcp).
- The rest of the README is shown on the feature page.

## Properties

| Property        | Value                                | Used for                                                                                                       |
| --------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------------- |
| `feature`       | `true` or `false`                    | `true` declares a feature outside a `features` folder; `false` skips the folder but still scans its subfolders |
| `owner`         | Team or person                       | Owner in the dashboard, in CODEOWNERS, in `diff` and `find-owner`. Inherited by nested features without one    |
| `creation_date` | `YYYY-MM-DD`                         | Overrides the date of the first commit in the feature                                                          |
| `status`        | `active`, `deprecated` or `archived` | Lifecycle of the feature, shown in the dashboard                                                               |
| `deprecated`    | `true` or `false`                    | Marks the feature as deprecated                                                                                |
| `key`           | Text                                 | Identifier used by external tools, see below                                                                   |
| `links`         | Map of name to URL                   | Links shown on the feature page, see below                                                                     |
| anything else   | Any value                            | Shown with the feature metadata, like `figma: <url>`                                                           |

## Key and links

`key` and `links` are not free-form metadata.

**`key`** is the identifier external tools use to refer to the feature. Tag your analytics events, errors and issues with it (for example a PostHog event property `feature=checkout`) and [import their data](/docs/external-data) with `--data`. It defaults to the folder name, lowercased, with every character other than letters, digits, `_` and `-` replaced by `-`: `Checkout Flow` becomes `checkout-flow`. Set it explicitly to keep it stable when the folder is renamed. The [`duplicate-keys`](/docs/check-rules#duplicate-keys) rule warns when two features share one.

**`links`** maps a name to a URL. Well-known tools (PostHog, Sentry, Datadog, Grafana, Figma, Linear, Jira, GitHub, Notion, Slack...) get a readable label and an icon on the feature page.

## FEATURES.toml

`key` and `links` can also live in a `FEATURES.toml` file in the feature folder, for features that don't need a README:

```toml
key = "checkout"

[links]
posthog = "https://eu.posthog.com/project/1/dashboard/2"
```

## Skip a folder

`feature: false` skips a folder, even a direct subfolder of a `features` folder:

```markdown
---
feature: false
---
```

Unlike `--ignore-path`, its subfolders are still scanned, so the features nested inside (in their own `features` folder or with `feature: true`) are still found.
