---
status: implemented
date: 2026-11-14
---

# Animated, product-accurate illustrations

## Context

The Linear-style illustrations from [004](004-linear-style-illustrations.md) gave the website depth, but they were static and the home hero was a generic feature list that didn't look like the dashboard people get. The illustrations should move like Linear's and show more of the real product.

## Requirements

- Home hero (`HeroIllustration.astro`) is a miniature of the real dashboard:
  - Front panel: the Insights page with the sidebar (logo, search with ⌘K, feature tree with owner dots, Insights / Project records links), stat cards, Features / Ownership / Dependencies tabs, the filter input and the features table (owner with dot, `inherited` badge, lines, TODOs, commit mix and change count).
  - Back panel: the `auth` feature page on its Changes tab (owner, meta chip, path, tab bar, commits per week chart).
  - A floating toast shows a decision linked to `auth` with task progress.
  - Owner dot colors use the same hash as `projects/web/src/lib/owner-color.ts`.
  - Sidebar and back panel are hidden on small screens.
- Hero animations: panels rise in, stats count up, rows slide in, commit bars fill, chart lines draw and dots pop. Then a 9s loop types `owner:team-auth` in the filter, dims the rows that don't match, highlights `auth` in the sidebar and erases the query.
- Concept page slices diagram: each band and column shows real file names and the columns show their owner. Bands slide in, the `auth` files light up one folder after the other in the layered view, columns rise and their files wipe in in the feature-based view.
- File trees: rows slide in one after the other, feature rows light up once the tree is in.
- Below-the-fold illustrations (`.reveal`) start animating when they scroll into view (IntersectionObserver in `Base.astro`). Without JavaScript nothing is paused.
- Every animation is off with `prefers-reduced-motion`, leaving the finished illustration.
- Palette stays the one of [003](003-monochrome-brand-palette.md) and [006](006-website-screenshots-and-accent-colors.md).

## Out of scope

- Dashboard screenshots and the web dashboard itself.
- Animating the other website sections (capability cards, terminal, pull request comment).

## Plan

1. Add `anim-*` utilities and the `.reveal` pause rule to `global.css`; add the `js` class and the reveal observer to `Base.astro`.
2. Build `HeroIllustration.astro` from the Insights and feature pages and use it on the home page.
3. Rework the slices diagram with files and owners, and animate it.
4. Stagger the `FileTree` rows.
5. Check desktop, mobile and reduced motion; build and lint the website.
