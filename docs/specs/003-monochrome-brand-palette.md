---
status: implemented
date: 2026-11-14
---

# Monochrome brand palette

## Context

The `{F}` logo and the website used a neon magenta → cyan → lime gradient that read as too much rainbow. The brand should feel modern and restrained, in the spirit of Linear and Vercel: near-black, off-white and grays, with light falling from the top instead of multiple hues.

## Requirements

- Logo (`projects/web/public/feature-icon.svg`, `projects/website/public/favicon.svg`, logo inside `projects/website/scripts/og.svg`):
  - Flat dark tile `#1A1A1D` with a faint white edge; no gradient on the tile.
  - Gradient only on the light parts (braces and F): `#F9F9F9` → `#B3B3B3`, top to bottom, matching the tone of Linear's app icon.
  - Braces and F share one light falloff (the F has its own gradient in its scaled coordinates).
- Website palette (`projects/website/src/styles/global.css`):

  | Token | Value | Use |
  | --- | --- | --- |
  | `ink` | `#08090A` | page background |
  | `surface` | `#0F1011` | cards |
  | `surface-2` | `#161718` | raised elements, inline code |
  | `line` | `#23252A` | borders |
  | `text` | `#F7F8F8` | main text |
  | `muted` | `#8A8F98` | secondary text |
  | `accent` | `#EDEEF0` | emphasis, links, focus ring (was `cyan`) |
  | `accent-2` | `#B4B8C0` | second emphasis / data series (was `magenta`) |
  | `accent-3` | `#80858E` | third emphasis / data series (was `lime`) |
  | `gradient-from` → `gradient-to` | `#F9F9F9` → `#B3B3B3` | `text-gradient`, `bg-gradient-brand` |

- The two colored page glows are replaced by one faint off-white glow at the top.
- The Open Graph image uses the same palette and `public/og.png` is re-rendered from `scripts/og.svg`.
- macOS terminal traffic-light dots and the red "deleted" marker keep their colors.

## Out of scope

- The web dashboard theme (`projects/web`) beyond its favicon/logo.
- Dashboard screenshots embedded in the website.

## Plan

1. Recolor the logo SVGs and the OG logo block.
2. Replace the neon tokens with role-named gray tokens and rename the `cyan`/`magenta`/`lime` utilities to `accent`/`accent-2`/`accent-3`.
3. Restyle `og.svg` and re-render `og.png`.
4. Build the website and check every page visually.
