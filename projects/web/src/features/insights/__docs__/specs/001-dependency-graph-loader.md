---
status: implemented
date: 2026-11-14
---

# Dependency graph loader

## Context

When the Dependencies tab of the Insights page opened, the force layout was
animated frame by frame with the default view transform, so the graph appeared
in the top-left corner, moved around, then jumped/zoomed when it was fitted to
the view at the end. The loading felt broken.

## Requirements

- While the layout is computed, the graph canvas shows a centered spinner with
  "Laying out graph…" instead of the moving graph.
- The graph appears only once the layout has settled and the view is fitted,
  with a short fade-in; it never shows in the corner first.
- The first render shows the loader, not the "No features found" empty state.
- The stats bar (feature/dependency counts) and the owner legend are visible
  during loading.
- The layout still runs in chunks across animation frames so the page stays
  responsive on large graphs.

## Out of scope

- Changing the layout algorithm or its duration.
- Moving the layout to a web worker.

## Plan

- Stop rendering nodes on every simulation frame; set them once at the end,
  together with the fitted transform.
- Start `simulating` as `true` and render a loader overlay while it is true;
  hide the graph group with `opacity: 0` and fade it in when done.
- Derive counts and owners from the built graph instead of the rendered nodes.
