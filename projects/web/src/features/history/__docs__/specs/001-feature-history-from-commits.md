---
status: implemented
date: 2026-11-14
---

# Feature history from the commit history

## Context

Each feature already lists the commits touching its folder (`changes`), but only
one feature at a time, in its "Changes" tab. There is no place to see how the
features of a project evolved over time: when each feature appeared, which
features moved in a given period, and who worked on them.

## Requirements

- A new **History** page (`#/history`), linked from the sidebar footer next to
  Insights, shown when the report has features.
- Built in the web UI only from the `changes` already in `features.json`: no
  CLI or report format change.
- Commits are deduplicated by hash across features and listed newest first,
  grouped by month.
- Each commit shows its title (PR numbers linked when the repository URL is
  known), author, date, short hash (linked to the commit when possible) and the
  features it touched.
- A commit is attributed to the **deepest** features containing it: a parent
  feature is left out when one of its sub-features also lists the commit, since
  every commit of a sub-feature is also a commit of its parent.
- The oldest commit of each feature is marked as the one that **introduced**
  the feature.
- Summary cards: number of commits, features introduced, contributors, and the
  period covered.
- A bar chart of commits per month for the current filters.
- Filters:
  - free text over title, description, author and hash;
  - a feature, chosen by clicking a feature badge; it includes its
    sub-features and can be cleared;
  - the conventional commit type (`feat`, `fix`, ...; `other` for the rest).
- Clicking the arrow next to a feature badge opens that feature's page.
- Long histories render the first 100 commits, with a "Show more" button.
- Empty states: no commits at all (e.g. `--skip-changes`), and no commit
  matching the filters.

## Out of scope

- File-level attribution of commits (the report does not carry changed files).
- Changing the per-feature "Changes" tab.
- Following renames or moves of feature folders.

## Plan

1. `src/features/history/utils.ts`: pure functions to build the history
   (dedupe, deepest-feature attribution, introductions, commit type, monthly
   buckets, filtering), unit-tested.
2. `src/features/history/` components: summary cards, activity chart,
   filters, month-grouped timeline.
3. `src/routes/history.tsx` + route in `index.tsx` + sidebar link.
4. Verify in the dev server preview.
