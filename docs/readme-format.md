# README format

In order to have your features detected by `features-cli`, you must put your features folder inside a `features` folder or you can rely on README files.

A feature README file should be placed in the root of each feature folder. All the properties in the front matter are optional and you can of course add your own custom properties.

```markdown
---
feature: true # required if not in a folder `features`
owner: team_1
creation_date: 2022-01-01 # to override the first commit date in this feature
status: active # active, deprecated, archived
deprecated: true # true, false
key: checkout # stable identifier for external tools (default: the folder name, slugified)
links: # links to the feature in external tools, shown on the feature page
  posthog: https://eu.posthog.com/project/1/dashboard/2
  sentry: https://sentry.io/organizations/acme/issues/?query=feature:checkout
---

# Feature Name (Override the feature folder name if provided)

```

### Skipping a feature

Setting `feature: false` explicitly skips a folder from being detected as a feature, even if it's a direct subfolder of a `features` folder. Unlike `--ignore-path`, its subfolders are still scanned, so any nested features it contains (via its own `features` folder or their own `feature: true` README) are still discovered:

```markdown
---
feature: false
---
```

### Key and links

`key` and `links` are not free-form metadata:

- `key` is the identifier external tools use to refer to the feature. Tag your analytics events, errors and issues with it (e.g. a PostHog event property `feature=checkout`) and import their data with `--data`. It defaults to the folder name, lowercased, with any character other than letters, digits, `_` and `-` replaced by `-` (`Checkout Flow` → `checkout-flow`). Set it explicitly to keep it stable when the folder is renamed.
- `links` maps a name to a URL. Well-known tools (PostHog, Sentry, Datadog, Grafana, Figma, Linear, Jira, GitHub, Notion, Slack, ...) get a readable label and an icon on the feature page.

Both also work in a `FEATURES.toml` file:

```toml
key = "checkout"

[links]
posthog = "https://eu.posthog.com/project/1/dashboard/2"
```
