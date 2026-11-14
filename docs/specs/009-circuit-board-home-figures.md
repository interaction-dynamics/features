---
status: implemented
date: 2026-11-14
---

# Circuit board home figures

## Context

The three isometric figures of the "Built for the age of AI" section first drew a city, then flat views of the dashboard. The figures should keep the content of the tools (Insights, dependency graph, Specs & Decisions) but tell it with one metaphor: the codebase as an electronic board, each feature a chip.

## Requirements

- FIG 0.1 (Insights): a board where each folder is a dashed zone printed with its folder icon and name, and each feature a chip sized like the feature. The dot in a chip's corner is its owner. In a loop, features are picked one by one: the chip lifts and its footprint lights up.
- FIG 0.2 (Dependencies): features are chips marked by owner (solid dot, ring, square), imports are traces routed between their legs, and the owner legend is printed on the board. In a loop, a pull request changes one chip, a signal runs segment by segment along the traces to the chips importing it, and the owners touched light up in the legend.
- FIG 0.3 (Specs & Decisions): the feature chip with its name, path and owner printed beside it, the Specs & Decisions tabs, then its datasheets: the decision record, the spec with its task progress, and the superseded record struck through, wired to the chip along the margin. In a loop, the next task of the spec gets done.
- Boards have a mounting hole in each corner. Chips have legs on every side, drawn so the body hides the back ones.
- Same monochrome line style, animation classes and reveal-on-scroll behavior as before; the final state stays readable without animations.

## Out of scope

- Changes to the iso drawing kit, the animation classes or the pillar texts.

## Plan

1. Add `chip`, `board` and `trace` helpers to `projects/website/src/lib/figures.ts`.
2. Redraw `insights`, `dependencies` and `specs` with them, keeping the loops.
3. Check each figure large and at its real size on the home page.
