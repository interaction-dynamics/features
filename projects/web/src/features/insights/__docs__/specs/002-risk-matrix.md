---
status: implemented
date: 2026-11-15
---

# Risk matrix and feature health score

## Context

The Insights page lists features, owners and dependencies, but it does not say
what to work on next. Coverage, technical debt (TODOs), git history and data
imported with `--data` are all in the report; combining them answers questions
like "which busy feature is barely tested?" or "which feature nobody touches
still carries debt?". This is Phase 3 of `docs/roadmap/integrations.md`
(risk matrix and feature health score).

## Requirements

### Risk matrix

- A new **Risk matrix** tab on the Insights page, next to _Features_.
- A scatter chart with one dot per feature (every level of the tree).
- X axis: a risk signal, picked from a select. Built-in choices:
  - **Untested lines (%)**: `100 − line coverage %` (only features with coverage).
  - **TODOs per 1,000 lines** (only features with lines).
- Y axis: an activity signal, picked from a select. Built-in choices:
  - **Commits**: total commits over the git history.
  - **Fix commits**: commits of type `fix`.
- Every numeric metric imported with `--data` (e.g. `weekly_users`,
  `open_errors`) can be picked on either axis.
- Defaults: X = untested lines when any feature has coverage, else TODO density;
  Y = commits when history is available, else the first imported metric.
- The plot is split in four quadrants at the median of each axis (dashed
  lines). Quadrants are labeled in the corners:
  - top right (high activity, high risk): **Test this first** when X is
    coverage, **Pay down debt first** when X is TODO density, **Look here
    first** otherwise;
  - top left: **Healthy and busy**;
  - bottom right: **Candidate for removal**;
  - bottom left: **Stable**.
- Dots in the top-right quadrant are drawn darker than the others. Dot size
  follows the number of lines.
- Hovering a dot shows the feature name, owner and both values.
- Clicking a dot opens the feature page (`/?feature=<path>`).
- Features missing a value on either axis are left out, with a note
  ("3 features without data are hidden").
- When no risk signal or no activity signal exists, the tab shows an empty
  state explaining how to get one (coverage report, git history, `--data`),
  and never crashes.

### Health score

- A sortable **Health** column in the features table, from 0 (worst) to 100
  (best), with a small bar and a tooltip listing the reasons.
- The score is `100 − penalties`, clamped to `[0, 100]`. Each penalty only
  applies when its signal exists:

  | Signal                   | Penalty                                                                                                                                    | Max |
  | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ | --- |
  | Line coverage            | `0.4 × (100 − coverage %)`                                                                                                                 | 40  |
  | TODO density             | `4 × TODOs per 1,000 lines`, lines floored at 1,000 so small features are not over-penalized                                               | 20  |
  | Fix-heavy history        | `30 × fix commits / total commits`, only with at least 5 commits                                                                           | 15  |
  | Imported errors and bugs | `8 × log10(1 + value)` per numeric `--data` metric whose name contains `error`, `bug`, `crash`, `exception`, `incident` or `issue`, summed | 25  |

- Penalties are rounded to integers so the reasons add up to the score.
- When no signal can be computed (no coverage, no lines, no history, no
  imported metrics), the cell shows a greyed `—` with a tooltip saying why,
  and sorts last.
- The formula lives in a pure function (`health-score.ts`) with unit tests.

### Missing data

- No coverage: untested-lines axis and coverage penalty are skipped.
- `--skip-changes` (no history): commit axes and fix penalty are skipped.
- No imported data: no extra axis options, no error penalty.

### Design

- Monochrome palette (foreground/muted tokens), works in light and dark themes.
- Uses the existing `Card`, `ChartContainer`, `Table`, `Tooltip` and
  `SortableTableHeader` components.
- Ships as a normal (non-experimental) feature.

## Out of scope

- Trends over time or comparing reports.
- Configuring the formula or the quadrant thresholds.
- Showing the health score on the feature page or in the owner table.

## Plan

1. `health-score.ts`: `computeHealthScore(feature)` returning
   `{ score, penalties, missing }`, plus unit tests.
2. `risk-metrics.ts`: available axis metrics for a list of features, value
   extraction, medians and quadrant labels, plus unit tests.
3. `risk-matrix.tsx`: recharts `ScatterChart` in a `Card`, with two selects,
   median reference lines, corner labels, custom tooltip and click navigation.
4. Add the Health column (with tooltip) to `feature-insights-table.tsx`.
5. Add the _Risk matrix_ tab to `routes/insights.tsx`.
6. Update the roadmap rows.
