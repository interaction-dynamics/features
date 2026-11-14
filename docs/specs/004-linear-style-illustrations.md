---
status: implemented
date: 2026-11-14
---

# Linear-style illustrations and gradients

## Context

After the switch to a monochrome palette ([003](003-monochrome-brand-palette.md)), the website images and diagrams were still flat gray boxes. Linear's site gets depth out of the same restrained palette: panels lit from the top, layered mockups, edges that fade into the page, soft spotlights and dot grids. The website illustrations should follow that style and use more gradients.

## Requirements

- New utilities in `projects/website/src/styles/global.css`:
  - `card`: surface brighter at the top and a gradient border (highlight at the top fading into `line`), built with `padding-box`/`border-box` backgrounds so it survives `overflow-hidden`.
  - `icon-tile`: lit icon square for feature/benefit grids.
  - `spotlight`, `dot-grid`, `hairline`, `fade-b`, `fade-x`: glows, dotted backdrops, fading rules and masks.
  - `text-gradient` is angled (bright top left → transparent bottom right); `text-fade` fades big numbers downwards.
  - `beam`: a light travelling along a line; `tilt`: screenshots straighten as they scroll into view (scroll-driven animation, skipped where unsupported).
  - Both animations are disabled with `prefers-reduced-motion`.
- Home hero: two layered panels, a dimmed "commits per week" scatter chart behind and the feature list in front, with coverage rings, owner avatars and gradient bars. The back panel is hidden on small screens.
- Screenshots: no fake window dots; inset frame, `featured` prop adds a spotlight and the tilt; caption sits under the frame.
- File trees: an icon per entry kind, gradient highlight on feature rows, owner shown as an avatar chip.
- Concept page slices diagram: `auth` is highlighted in both views (spread across four layers vs one glowing column with a beam).
- Sections get a fading hairline on top; the call to action gets a spotlight and dot grid.
- Palette stays monochrome: only the tokens from spec 003 are used.

## Out of scope

- Dashboard screenshots themselves and the web dashboard theme.
- The Open Graph image.

## Plan

1. Add the utilities and rework `card`.
2. Restyle `Screenshot`, `FileTree`, `Terminal`, `Section`, `CallToAction`.
3. Rebuild the home hero illustration and the slices diagram; use `icon-tile` and `text-fade` on the home and AI pages.
4. Build the website and check desktop and mobile.
