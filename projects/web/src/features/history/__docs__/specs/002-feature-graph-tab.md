---
status: implemented
date: 2026-11-14
---

# Feature graph tab on the History page

## Context

The History page lists commits, which shows what changed but not how the
feature tree grew. Users want a view like a git graph: when each feature was
created and when its sub-features branched off from it.

## Requirements

- The History page has two tabs: **Timeline** (the existing commit list, with
  its summary cards, chart and filters) and **Graph**.
- The selected tab is kept in the URL (`#/history?view=graph`); Timeline is the
  default and has no parameter.
- The graph has one row per feature, parents before their sub-features,
  siblings ordered by creation date, indented by depth in the label column.
- Time runs left to right with month ticks; labels that would be cut at the
  edges are skipped, and fewer months are labelled on narrow widths.
- A **Project** trunk spans the whole history; top-level features fork from it.
- Each feature lane starts at its oldest commit (its creation) with a dot and
  forks from its parent's lane with a rounded connector, like a git branch.
  When a feature is created in the same commit as its parent the connector is
  a straight vertical line.
- The lane is solid up to the feature's last commit, then faded up to the end
  of the history (the feature still exists).
- Each top-level feature and its sub-features share a color.
- Hovering (or focusing) a lane highlights it, its ancestors and its
  sub-features, dims the others, and shows details below the graph: creation
  date, author, creating commit title, number of commits, last change and
  number of sub-features.
- Clicking a label or a lane opens the feature page.
- Features without commits are left out, with a note giving their count.
- Built only from the `changes` already in `features.json`.

## Out of scope

- Every commit as a dot on the lanes (only creations are shown).
- Applying the Timeline filters to the graph.
- Feature deletions, renames or moves (the report only has current features).

## Plan

1. `src/features/history/graph.ts`: build the lanes (order, parent, color
   root, creation, last change), lineage for highlighting, month ticks;
   unit-tested.
2. `src/features/history/history-feature-graph.tsx`: label column + SVG lanes,
   connectors, axis, hover details.
3. `src/routes/history.tsx`: Timeline / Graph tabs synced with `?view=`.
4. Verify in the dev server preview with the demo history.
