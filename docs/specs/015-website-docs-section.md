---
status: implemented
date: 2026-11-18
---

# Website documentation section

## Context

The website had marketing pages (`/cli`, `/ai`, `/guidelines`...) but no technical reference: the CLI options, the new `features.config.yaml` and the check rules only lived in `docs/cli.md` on GitHub.

## Requirements

- A `/docs` section with a left sidebar listing its pages, grouped, with the current page highlighted. Below `lg`, the sidebar becomes a drawer opened from a sticky "Docs / <page>" bar.
- Multiple pages, written for the web from `docs/cli.md`, `docs/readme-format.md` and `docs/folder-architecture-guidelines.md`: Introduction, Commands, Options, **Configuration file**, **Check rules**, Folder structure, README format, Specs and decisions, Publish the dashboard, Pull requests and CI, MCP server, Import external data, Aggregate repositories.
- The Configuration file page documents `features.config.yaml` (see [014](014-config-file-arguments.md)): keys, sections, a full example, precedence, lookup, `--config` / `--no-config`, errors and the migration from `features.config.toml`.
- The Check rules page documents the 8 rules, severities, options, `check.rules`, the CLI flags, the report formats and a GitHub Actions example.
- Previous/next links, an "On this page" list on wide screens, an "Edit this page on GitHub" link.
- A "Docs" link in the header; docs pages appear in `llms.txt` as Markdown twins.
- No horizontal scroll at phone width.

## Out of scope

- Search.
- Versioned docs.

## Plan

1. Astro content collection `docs` (`src/content/docs/*.md`, `src/content.config.ts`), sidebar order in `src/data/docs.ts` (the build fails when a page and the sidebar disagree).
2. `src/layouts/Docs.astro` and `src/components/DocsNav.astro`, route `src/pages/docs/[...slug].astro`.
3. Header link, `.prose-docs` styles, `llms` integration reading nested pages.
4. Links from `/cli` to the docs.
