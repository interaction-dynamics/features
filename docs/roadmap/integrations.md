---
status: draft
date: 2026-11-14
---

# Integrations roadmap

## Why

`features-cli` knows **what** each feature is: code, owner, tests, debt, history. Tools like PostHog, Sentry and Linear know **how** each feature behaves in production: usage, errors, bugs, flags. Today the two never meet, so nobody can answer questions like these:

- Which features are heavily used but poorly tested?
- Which features nobody uses but still cost maintenance?
- Which feature flags are fully rolled out but still in the code?
- Which team should get this production error?

Integrations should turn the dashboard from a map of the code into a decision tool.

## Principles

1. **Offline by default.** Scanning never calls the network. Fetching external data is opt-in, per command or config.
2. **Generic before specific.** A generic import format comes first, so any tool can be connected with a script before a native adapter exists.
3. **Secrets never land in reports.** API keys come from environment variables and are never written to `features.json` or a static build.
4. **The feature is the join key.** Every integration matches external data to a feature through a stable key, never through guesswork.
5. **Light default binary.** Network adapters sit behind a Cargo feature (for example `integrations`), so the default install stays small.

## How data gets matched to features

Every inbound integration needs one of these matching strategies, from most to least reliable:

| Strategy | Example | Good for |
| -------- | ------- | -------- |
| Feature tag on the external item | PostHog event property `feature=checkout`, Sentry tag `feature:checkout`, Linear label `feature:checkout` | Usage, errors, issues |
| Code annotation naming the external item | `// --feature-flag name: new-checkout, provider: posthog` | Feature flags |
| Code path in the external item | Sentry stack trace frame in `src/features/checkout/**` | Errors, without changing the app |
| URL pattern declared on the feature | `posthog_url: /checkout/*` in the README front matter | Page views |

## Phase 0: foundations (no network)

These items unblock everything else and keep the CLI offline. ✅ = shipped, see [spec 005](../specs/005-feature-key-links-and-data-import.md).

| Item | What it does | Value | Effort |
| ---- | ------------ | ----- | ------ |
| **Stable feature key ✅** | Add a `key` (slug) to every feature in the JSON report, overridable in front matter. It is the value apps use to tag events, errors and issues. | High | S |
| **Links section ✅** | Document a `links:` map in the front matter (`posthog`, `sentry`, `figma`, `linear`, `notion`, `grafana`, ...) and show it as a *Links* block with tool icons on the feature page. URLs already render as links today; this makes it a first-class convention. | Medium | S |
| **Generic data import ✅** | `features build --data external.json` (also `scan`, `serve`, `check` and `aggregate`) merges `{ "<feature key>": { "<metric>": value } }` into the report and shows it on the feature page and in the insights tables. Anyone can connect any tool with a short script. | **Very high** | M |
| **Path map export** | `features scan --path-map` prints a glob-to-feature-key map an app can bundle to tag its own events and errors with the feature, from the file that raised them. | High | S |

## Phase 1: push the feature map into tools (outbound)

Generated files and CI recipes, like the existing CODEOWNERS generator. No API keys needed in the CLI.

| Item | What it does | Value | Effort |
| ---- | ------------ | ----- | ------ |
| **Sentry ownership rules** | `features sentry-ownership` generates `path:src/features/checkout/** #team-checkout` rules, so errors go to the owning team. Reuses the CODEOWNERS logic. | High | S |
| **Pull request labels** | GitHub Action recipe: `features diff --json` then label the PR `feature:checkout`. Labels make PRs searchable by feature and feed issue trackers. | Medium | S |
| **Backstage catalog export** | Generate `catalog-info.yaml` entities per feature (owner, docs, links), for teams that already run a developer portal. | Medium | M |
| **Owner to channel mapping** | Optional `channel` property per owner, so `diff --markdown` and CI recipes can ping the right Slack or Teams channel. | Low | S |

## Phase 2: pull production data in (inbound, opt-in)

Native adapters behind the `integrations` Cargo feature. Configured in a `features.config.toml` (or flags), keys in environment variables, results cached so `serve` stays fast.

| Item | What it shows per feature | Matching | Value | Effort |
| ---- | ------------------------- | -------- | ----- | ------ |
| **Feature flag hygiene (PostHog first)** | Flags annotated in code compared with the flag provider: stale flags (100% rolled out, still in code), orphan flags (in the provider, not in code), flags missing an annotation. | Annotation `provider: posthog` | **Very high** | M |
| **Usage (PostHog)** | Events, unique users and trend over 30 days. | Event property `feature`, or URL pattern | High | M |
| **Errors (Sentry)** | Unresolved issues, events over 30 days, top issue. | Tag `feature`, or stack trace path | High | M |
| **Open bugs (Linear, GitHub Issues, Jira)** | Open bugs and age of the oldest one. | Label `feature:<key>` | Medium | M |
| **Other flag providers** | Same flag hygiene for LaunchDarkly, Unleash, GrowthBook, Statsig. | Annotation `provider: ...` | Medium | S each, once the PostHog adapter exists |

Flag hygiene comes first because annotations already exist: it needs no change in the user's app, only an API key.

## Phase 3: insights that combine sources

This is where integrations pay off.

| Item | What it does | Value | Effort |
| ---- | ------------ | ----- | ------ |
| **Risk matrix** | Plot usage against coverage or debt. High usage with low coverage = test it first. No usage with high churn = candidate for removal. | Very high | M |
| **Feature health score** | One score per feature combining errors, coverage, debt, bugs and usage, with the reasons listed. Sortable in the insights tables. | High | M |
| **Trends over time** | Keep past reports (for example on the GitHub Pages branch) and show how each metric evolves per feature. | Medium | L |
| **MCP server ✅** | `features mcp` exposes the feature map and the imported data to Claude Code, Cursor and other agents, so they can check usage and errors of a feature before changing it. Shipped, see [spec 011](../specs/011-mcp-server.md). | High | M |

## Suggested order

1. Stable feature key, links section, generic data import (Phase 0). Every later item builds on them, and users can connect any tool right away.
2. Sentry ownership rules (quick win, reuses CODEOWNERS).
3. PostHog flag hygiene (unique value, no app changes).
4. PostHog usage and Sentry errors.
5. Risk matrix (first composite insight).
6. Everything else, driven by user requests.

## Open questions

- **Network calls in the CLI:** acceptable behind an opt-in flag, or should inbound data only come through the generic import and external scripts?
- **Config file:** introduce `features.config.toml` for integrations, or keep everything in flags and environment variables?
- **Static builds:** should a published dashboard embed fetched production data, given that it may be public? Probably opt-in, like `--strip-emails`.
- **Which tools first:** PostHog and Sentry are assumptions. User requests in issues should confirm them.
