---
status: implemented
date: 2026-11-14
---

# Website screenshots, accent colors and favicons

## Context

The dashboard screenshots on the website still showed the old blue "F" logo and the old "Decisions" tab, because the live demo is only redeployed from `master`. The fully monochrome palette from [004](004-linear-style-illustrations.md) also felt flat: Linear's site keeps a gray base but uses a handful of colors sparingly (a highlighted bar, a status dot, a colored glow). The favicon showed the old logo in some browsers: Safari ignores SVG favicons and browsers cache icons aggressively.

## Requirements

- All `public/screenshots/*.webp` re-captured from a dashboard built from this branch: new `{F}` logo, "Specs & Decisions" tab.
- `scripts/capture-screenshots.mjs` clicks the "Specs & Decisions" tab (its name carries a count) and the decisions view is 560px high.
- A few color tokens in `global.css`: `indigo`, `violet`, `green`, `orange`, `yellow`. Used for small accents only, never for large surfaces:
  - hero: the `auth` coverage bar (indigo → violet with a glow), its coverage ring and row highlight, one trend line of the back chart, a green "Open source" status dot;
  - "How it works": the four output icons;
  - featured screenshots: an indigo/violet glow (`spotlight-color`) instead of the gray spotlight.
- `Screenshot` gets a `fade` prop: the frame dissolves into the page from 45% of its height (`fade-shot` mask) and the caption moves up. Used on the tall screenshots only (home Insights, concept page nested features), not on short ones where it would hide content.
- Favicons: `favicon.ico` (16/32/48) and `apple-touch-icon.png` (180) rendered from `favicon.svg` by `scripts/render-icons.mjs`; the SVG link carries a `?v=2` cache buster.
- `.claude/launch.json` gets a `dashboard` configuration (port 5173) to serve the dashboard locally for captures.

## Out of scope

- The dashboard's own theme and the Open Graph image.
- Coloring the other illustrations (file trees, slices diagram).

## Plan

1. Build the CLI, generate the demo `features.json`, serve the dashboard locally and re-run the capture script.
2. Add the color tokens and `spotlight-color` / `fade-shot` utilities; use them in the hero, "How it works" and `Screenshot`.
3. Render the raster favicons and link them in `Base.astro`.
4. Build the website and check desktop and mobile.
