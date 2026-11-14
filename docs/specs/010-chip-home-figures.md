---
status: implemented
date: 2026-11-14
---

# Chip home figures, Linear style

## Context

The three isometric figures of the "Built for the age of AI" section were drawn as flat views of the dashboard, then as real-world comparisons (a city map, fenced plots, a building site, #192). They should tell the content of the tools (Insights, dependency graph, Specs & Decisions) with one metaphor, the codebase as electronics with each feature a chip, and look like Linear's figures: a few solid objects, more 3D, simple.

## Requirements

- A few large objects per figure with rounded corners, thin lines and almost no text; chips show their legs as a dotted band along the bottom.
- FIG 0.1 (Every feature, mapped): the layers of a board stacked with gaps and dotted corner guides, the chip on top printed with a 3×3 grid of features. In a loop, features are picked one by one and stand out of the grid.
- FIG 0.2 (Owned and connected): four chips of different heights side by side, each marked with its owner (dot, ring or square). A pull request hovers over the chip it changes, tied to it by dotted lines. In a loop, the change pings, then the chips importing it.
- FIG 0.3 (Intent kept in the repository): specs mapped to features. Three features, each a chip inside its dashed folder footprint, with its own datasheets standing right behind it and wired to it. The middle feature carries a superseded record (dashed, at the back), a decision record, and its spec in front with its title and task progress. In a loop, the next task of that spec gets done.
- The three figures have a similar visual weight in their 224×176 frame.
- Same monochrome strokes, animation classes and reveal-on-scroll behavior as before; the final state stays readable without animations.

## Out of scope

- Changes to the pillar texts or the figure names (`map`, `owners`, `specs`).

## Plan

1. Add a `slab` primitive to the iso kit: a box with rounded corners, optional dotted pins.
2. Restore the `anim-iso-select` loop (dropped with the city figures, replacing the unused `anim-iso-bob`).
3. Redraw `map`, `owners` and `specs` with it, keeping their loops.
4. Check each figure large and at its real size on the home page.
