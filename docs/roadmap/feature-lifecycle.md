---
status: draft
date: 2026-11-19
---

# Feature lifecycle roadmap

## Why

Today a feature in `features-cli` is a folder with an owner, code, tests, debt, specs and decisions. That covers the build half of its life. The other half happens after the merge: the feature is turned on for some users, tested against a variant, shipped in a release, used (or not), and watched in production. Each of those facts lives in a different tool, keyed by a flag, an event, an error tag or a ticket, never by the feature.

The [competitor audit](../../reports/Feature%20lifecycle%20dashboard%20SaaS%20audit.md) found that no product shows one feature end to end, and that the feature key is the asset nobody else has. This roadmap lists what to **attach** to each feature so the key carries the whole lifecycle:

| Dimension | Question it answers per feature |
| --------- | ------------------------------- |
| [SDLC lifecycle](#1-sdlc-lifecycle) | Where is it: idea, spec, in development, beta, generally available, deprecated, removed? |
| [Activation](#2-activation) | Who has it turned on, through which flags, plans and environments? |
| [Experimentation](#3-experimentation) | Which experiments ran on it, what won, and was the losing code removed? |
| [Versions](#4-versions) | Which release introduced it, changed it, deprecated it? What changed between two releases? |
| [Analytics](#5-analytics) | Is it used, by how many people, and is that going up or down? |
| [Monitoring](#6-monitoring) | Is it healthy in production: errors, latency, SLOs, alerts, incidents? |

The [integrations roadmap](integrations.md) is about **how** external data gets in (generic import, native adapters, matching). This one is about **what** a feature record holds and what the dashboard and the checks do with it.

## Principles

1. **Attach, don't rebuild.** `features-cli` never becomes a flag service, an experimentation engine, an analytics tool or an APM. It records which flags, experiments, events and alerts belong to a feature, links out to the tool that owns them, and shows a few numbers imported from it.
2. **Declared in the repository first.** Every dimension starts as front matter or a code annotation, so it works offline, is reviewed in pull requests and is read by the MCP server and the Claude skills. Production numbers come later, through `--data` or adapters.
3. **Derive before asking.** When git, the scanner or existing annotations can infer a value (stage, introduced version), infer it and let front matter override it.
4. **Every dimension gets a check.** Attached data is only worth keeping if it drives a decision. Each dimension comes with at least one `features check` rule that catches the gap (stale flag, untracked feature, unmonitored GA feature).
5. **Same key everywhere.** Flags, experiments, events and alerts point to the feature through its `key`, as in the integrations roadmap.

## Where each value comes from

| Source | Examples | Network | Already exists |
| ------ | -------- | ------- | -------------- |
| **Declared**: README front matter | `stage`, `plans`, `introduced_in`, `slo`, `links` | No | `owner`, `key`, `links`, `feature` |
| **Declared**: code annotations | `--feature-flag`, `--feature-experiment`, `--feature-event`, `--feature-alert` | No | Free-form annotations, `--feature-flag` with the `stale-flag` rule |
| **Derived**: git and the scanner | First commit, release tags that touched the folder, stage inferred from specs, flags and age | No | Commit history, `features diff` |
| **Imported**: `--data` | Rollout %, weekly users, open errors, p95 latency | No (the user's script fetches it) | Generic data import |
| **Fetched**: native adapters | Same numbers, pulled from PostHog, Sentry, LaunchDarkly... | Yes, opt-in | Planned in the integrations roadmap, Phase 2 |

## The feature record

Everything below adds up to one optional front matter shape. Every field is optional; a feature with only `owner` keeps working as today.

```yaml
---
key: checkout
owner: payments-team
stage: ga                      # 1. lifecycle (inferred when absent)
introduced_in: v1.4.0          # 4. versions (inferred from tags when absent)
deprecated_in:                 # 4. versions
sunset_on:                     # 1. lifecycle, date the feature should be removed
plans: [pro, enterprise]       # 2. activation, entitlements
environments: [staging, production]
slo:                           # 6. monitoring
  availability: 99.9
  p95_ms: 400
links:
  posthog: https://eu.posthog.com/project/1/dashboard/2
  sentry: https://sentry.io/organizations/acme/issues/?query=feature:checkout
  launchdarkly: https://app.launchdarkly.com/default/production/features/new-checkout
  grafana: https://grafana.acme.io/d/checkout
---
```

```typescript
// --feature-flag name: new-checkout, provider: launchdarkly, introduced_on: 2026-09-01
// --feature-experiment name: one-page-checkout, flag: new-checkout, metric: checkout.completed, status: running, started_on: 2026-10-01
// --feature-event name: checkout.completed
// --feature-alert name: checkout-error-rate, provider: sentry
```

In the JSON report, each feature gets a `lifecycle` object next to the existing fields, so the MCP server and scripts read one shape:

```json
{
  "key": "checkout",
  "lifecycle": {
    "stage": { "value": "ga", "source": "declared" },
    "versions": { "introduced_in": "v1.4.0", "changed_in": ["v1.5.0", "v1.6.2"] },
    "activation": { "flags": ["new-checkout"], "plans": ["pro", "enterprise"] },
    "experiments": [{ "name": "one-page-checkout", "status": "running" }],
    "analytics": { "events": ["checkout.completed"], "weekly_users": 12840 },
    "monitoring": { "alerts": ["checkout-error-rate"], "open_errors": 3, "slo": { "p95_ms": 400 } }
  }
}
```

## 1. SDLC lifecycle

**Attach:** a `stage` per feature, from a configurable list. Default stages: `idea` → `spec` → `development` → `beta` → `ga` → `deprecated` → `removed`.

**Inference when `stage` is absent:**

| Signal | Inferred stage |
| ------ | -------------- |
| Only specs or decisions, no code | `spec` |
| Code, a spec with open tasks, or no commit on the default branch yet | `development` |
| At least one flag or a running experiment, or a `beta` link/label | `beta` |
| Code on the default branch, no open flag | `ga` |
| `deprecated_in` or `sunset_on` set | `deprecated` |

The dashboard shows whether a stage was declared or inferred, and why.

**Dashboard:** a *Stage* column and filter in the features table, a lifecycle board (one column per stage), and a stage badge on the feature page.

**Checks:**

| Rule | Default | Catches |
| ---- | ------- | ------- |
| `deprecated-still-changing` | `warn` | Commits on a deprecated feature in the last N days, outside removal work |
| `past-sunset` | `warn` | `sunset_on` is in the past and the code is still there |
| `ga-requirements` | off | A `ga` feature missing an owner, coverage above a threshold, or a monitoring link (configurable list) |
| `stuck-stage` | off | A feature in `beta` or `development` for longer than N days |

**Config:** stage names and order, and `ga-requirements`, in `features.config.yaml` under `lifecycle:`.

## 2. Activation

**Attach:** how a feature reaches users. That is the flags that gate it (several per feature), the plans or entitlements that include it, the environments where it is on, and its kill switch.

- Flags already come from `--feature-flag` annotations. Add a `provider` property and allow one flag to gate several features (`feature: checkout, search`).
- `plans` and `environments` live in front matter.
- Rollout state (percentage per environment, last change) is imported or fetched, never declared, because it changes outside the repository.

**Dashboard:** an *Activation* panel on the feature page with flags, provider links, plans, and rollout per environment when imported. An *Activation* column (`off`, `partial`, `on`) in the table.

**Checks:**

| Rule | Default | Catches |
| ---- | ------- | ------- |
| `stale-flag` (exists) | `warn` | Flag annotation older than the allowed age. Extend it to use imported rollout: 100% everywhere for N days = stale |
| `ga-behind-flag` | `warn` | A feature declared `ga` still gated by a flag |
| `orphan-flag` | `warn` | A flag in the imported provider data with no annotation in code (needs import) |
| `missing-kill-switch` | off | A `ga` feature with a declared SLO and no flag or `kill_switch` link |

## 3. Experimentation

**Attach:** experiments run on the feature, declared with `--feature-experiment` annotations (`name`, `flag`, `metric`, `hypothesis`, `status`, `started_on`, `ended_on`). Results (variants, lift, significance, winner) are imported. The final decision is a decision record in the feature's `__docs__/decisions`, which the dashboard already reads.

**Dashboard:** an *Experiments* tab on the feature page: running and past experiments, results when imported, and a link to the decision record that closed each one.

**Checks:**

| Rule | Default | Catches |
| ---- | ------- | ------- |
| `experiment-without-decision` | `warn` | An experiment with `status: concluded` and no decision record referencing it |
| `experiment-overdue` | off | A running experiment older than N days |
| `losing-variant-in-code` | `warn` | A concluded experiment whose flag annotation is still in the code |

**Skill:** extend the `create-feature` skill, or add a `conclude-experiment` skill, that writes the decision record and removes the losing branch and its flag.

## 4. Versions

**Attach:** the releases a feature was introduced, changed, deprecated and removed in. For libraries and APIs, also the feature's own `version` (for example an API version).

- **Derived from git:** for each release tag (configurable pattern, default `v*`), the features whose folder changed since the previous tag. That gives `introduced_in` (first tag that contains the folder) and `changed_in`.
- **Declared:** `introduced_in`, `deprecated_in` and `removed_in` override the derived values, for features moved or renamed between folders.
- **Imported:** the version deployed per environment, so the dashboard can say "changed in v1.6.2, not deployed to production yet".

**CLI:**

```bash
features changelog ./src --since v1.5.0             # features changed since a release, grouped by feature
features changelog ./src --feature checkout         # release history of one feature
features changelog ./src --since v1.5.0 --markdown  # release notes draft, one section per feature
```

`features diff --since v1.5.0` already lists the touched features. `changelog` adds the per-release grouping and uses commit subjects (Conventional Commits scope when present) as entries.

**Dashboard:** a *Versions* timeline on the feature page, and a release picker on the features table ("features changed in this release").

**Checks:**

| Rule | Default | Catches |
| ---- | ------- | ------- |
| `removed-still-present` | `error` | `removed_in` set to a released tag and the folder still exists |
| `breaking-change-without-version` | off | For features with a `version`, a change to its public entry point without a version bump |

## 5. Analytics

**Attach:** the tracking plan of the feature, meaning which events prove it is used, declared with `--feature-event` annotations next to the code that sends them. The usage numbers (weekly users, adoption as a share of active users, trend, funnel conversion) are imported or fetched.

- Event naming convention suggested in the docs: `<feature key>.<action>` (`checkout.completed`), or a `feature` event property set to the key. `features scan --path-map` (integrations roadmap, Phase 0) lets an app set that property automatically.
- Usage numbers come in through `--data` today. Group them under `analytics` so the dashboard knows where to show them (see [Data import shape](#data-import-shape)).

**Dashboard:** an *Analytics* panel with declared events, imported numbers and the trend, and a link to the analytics tool. The existing risk matrix already plots usage against coverage or debt.

**Checks:**

| Rule | Default | Catches |
| ---- | ------- | ------- |
| `untracked-feature` | off | A `beta` or `ga` feature with no `--feature-event` annotation and no imported usage |
| `unused-feature` | off | Imported usage below a threshold for N days: a removal candidate |

## 6. Monitoring

**Attach:** how the feature is watched in production. That is its SLO targets (front matter `slo`), its alerts (`--feature-alert` annotations or links), its dashboards (`links`), and imported health numbers: open errors, error rate, p95 latency, Web Vitals for UI features, open incidents.

- Errors already join through the Sentry tag or stack trace path (integrations roadmap). Sentry ownership rules (integrations roadmap, Phase 1) route errors to the feature's owner.
- The imported numbers are compared with `slo` to give a status: `healthy`, `at-risk`, `breaching`.

**Dashboard:** a *Monitoring* panel with the SLO status, alerts and links. Breaching features feed the existing health score, and a *Production* filter shows only features with a monitoring problem.

**Checks:**

| Rule | Default | Catches |
| ---- | ------- | ------- |
| `unmonitored-feature` | off | A `ga` feature with no alert, no monitoring link and no SLO |
| `slo-breach` | `warn` | Imported numbers outside the declared `slo` |

## Data import shape

The generic import stays backward compatible (flat metrics per feature key). It also accepts one level of grouping, so each number lands in the right panel:

```json
{
  "checkout": {
    "activation": { "rollout_production": 100, "rollout_staging": 100 },
    "experiments": { "one-page-checkout": { "winner": "B", "lift": 0.042, "significance": 0.97 } },
    "versions": { "deployed_production": "v1.6.1" },
    "analytics": { "weekly_users": 12840, "adoption": 0.31 },
    "monitoring": { "open_errors": 3, "p95_ms": 380, "availability": 99.95 }
  }
}
```

Flat metrics keep showing in the generic *Data* block, as today.

## The lifecycle timeline

Once the six dimensions are attached, the feature page gets a single **timeline** that merges them in date order: spec written, first commit, flag introduced, experiment started, decision recorded, released in v1.4.0, flag removed, error spike, deprecated. That is the end-to-end view no other tool has, and it costs little once each dimension has dates.

## Phases

### Phase A: declare (offline, small)

| Item | Dimension | Value | Effort |
| ---- | --------- | ----- | ------ |
| `stage`, `sunset_on`, `plans`, `environments`, `introduced_in`, `deprecated_in`, `removed_in`, `slo` front matter, read into a `lifecycle` object in the JSON report | All | High | S |
| Conventions for `--feature-experiment`, `--feature-event`, `--feature-alert`, and `provider` on `--feature-flag`, documented in the README and the `create-feature` skill | 2, 3, 5, 6 | High | S |
| Stage column, filter and badge in the dashboard | 1 | High | S |
| Expose `lifecycle` in the MCP server (`get_feature`, and a `list_features` filter by stage) | All | High | S |

### Phase B: derive (offline, git)

| Item | Dimension | Value | Effort |
| ---- | --------- | ----- | ------ |
| Stage inference with reasons | 1 | High | M |
| Release tags per feature (`introduced_in`, `changed_in`) | 4 | High | M |
| `features changelog` | 4 | High | M |
| Lifecycle timeline on the feature page | All | **Very high** | M |

### Phase C: check

| Item | Dimension | Value | Effort |
| ---- | --------- | ----- | ------ |
| `deprecated-still-changing`, `past-sunset`, `ga-behind-flag`, `removed-still-present`, `experiment-without-decision`, `losing-variant-in-code` | 1, 2, 3, 4 | High | S each |
| `ga-requirements`, `untracked-feature`, `unmonitored-feature` | 1, 5, 6 | Medium | S each |

### Phase D: import production state

| Item | Dimension | Value | Effort |
| ---- | --------- | ----- | ------ |
| Grouped `--data` shape and the Activation, Experiments, Analytics and Monitoring panels | 2, 3, 5, 6 | High | M |
| Rules that need imported data: `orphan-flag`, rollout-aware `stale-flag`, `unused-feature`, `slo-breach` | 2, 5, 6 | High | S each |
| Lifecycle board (one column per stage) | 1 | Medium | M |
| Native adapters | 2, 3, 5, 6 | | See the [integrations roadmap](integrations.md), Phase 2 |

## Suggested order

1. Phase A in one change: the front matter fields and annotation conventions cost little and make the record usable by agents right away.
2. Release tags per feature and `features changelog`: no network, useful on day one for any repository with tags.
3. Stage inference and the lifecycle timeline: the view that sells the "feature system of record" positioning.
4. The Phase C checks, driven by what the first users ask for.
5. Grouped import and the production panels, then the adapters from the integrations roadmap.

## Out of scope

- Serving or evaluating flags, assigning experiment variants, computing statistical significance, collecting events or storing metrics. Those belong to the provider; `features-cli` links to it and imports a summary.
- Planning and roadmapping features before code exists, beyond the `idea` and `spec` stages read from specs. That stays in Linear or Jira, linked through `links:`.
- Design status. Figma stays a link.

## Open questions

- **Stage list:** is the default list right, or should it start smaller (`development`, `beta`, `ga`, `deprecated`)? Should teams be able to rename stages?
- **Inference vs. declaration:** should a declared stage that contradicts the signals (declared `ga`, still behind a flag) be a warning, or should the declaration always win silently?
- **Tags in monorepos:** release tags are often per package (`web@1.4.0`). Map tag patterns to folders in `features.config.yaml`?
- **Annotation sprawl:** four new annotation keys, or one `--feature-lifecycle` annotation with a `kind` property?
- **Public dashboards:** imported rollout, usage and SLO numbers may be sensitive. Add a `--strip-production-data` option, like `--strip-emails`?
