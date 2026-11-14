---
status: implemented
date: 2026-11-14
---

# App menu and project name in the sidebar

## Context

The Features logo took the top left corner of the sidebar, while the help
links (feedback, GitHub) took space in every page header and the theme toggle
and version sat loose at the bottom of the sidebar. Inspired by Linear, the
top left should name the project being explored, and the secondary actions
should live in one menu at the bottom, with a "What's new" section so users
discover recent changes.

## Requirements

- The top left of the sidebar shows the project name, with its initial as an
  avatar, and links to the home page. In an aggregated report it also shows
  the number of repositories.
- The project name is resolved by the CLI and written to `metadata.json`
  (`projectName`), in order of priority:
  1. `--project-name <name>` on `features serve` and `features build`;
  2. `name` in a `features.config.toml`;
  3. the name in the manifest of the main technology: `package.json`,
     `Cargo.toml` (`[package]`), `pyproject.toml` (`[project]` or
     `[tool.poetry]`), `go.mod` (last segment of the module), `composer.json`,
     `pom.xml` (`<name>`, else `<artifactId>`, not the parent's),
     `settings.gradle(.kts)` (`rootProject.name`), `pubspec.yaml`, `deno.json`.
  Files are looked up in the scanned folder then its parents up to the git
  root, the nearest winning; the config file wins over every manifest.
  Blank values are ignored and an invalid config file only prints a warning.
- Without a name, the dashboard shows "Features".
- The Features logo moves to the bottom of the sidebar, with the version. It
  opens the app menu, above it:
  - Docs, Give feedback, GitHub (external links, new tab);
  - Theme submenu (Light, Dark, System);
  - "What's new": the 3 latest changelog entries linked by a dashed timeline,
    then "Full changelog" (GitHub releases).
- A dot on the logo and next to "What's new" shows changelog entries the user
  has not seen. They count as seen when the menu closes (stored in
  `localStorage`; without storage they stay unread).
- The "Give Feedback" and GitHub buttons leave the page header; the theme
  toggle and version indicator leave the sidebar footer.

## Out of scope

- `--project-name` on `aggregate` (an aggregated report keeps "Features").
- Other settings in `features.config.toml`.
- Fetching the changelog from GitHub at runtime: entries are bundled in the
  web app (`features/app-menu/changelog.ts`).

## Plan

1. CLI: `project_name` module (manifest detection, `features.config.toml`,
   lookup up to the git root), `--project-name` on `serve` and `build`,
   `projectName` in `metadata.json` from `BuildConfig` and `ServerConfig`.
2. Web: `features/app-menu` (menu, changelog data, seen state), sidebar header
   with the project name, menu in the sidebar footer, header buttons removed.
3. Docs: `--project-name` and "Name the dashboard" in `docs/cli.md`.
