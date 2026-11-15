---
status: implemented
date: 2026-11-15
---

# Website changelog

## Context

The marketing website (`projects/website`) has no changelog. A public changelog shows the project is active, gives search engines and answer engines fresh content, and gives users one place to see what changed between versions.

What existed before:

- The dashboard's "What's new" menu (`projects/web/src/features/app-menu/changelog.ts`) listed five hand-written highlights with an id, a title and a pull-request link, but no version or date.
- GitHub releases (`v0.4.13` → `v0.14.0`) carry auto-generated notes made of raw commit subjects.
- No `CHANGELOG` file.

Two hand-maintained lists would drift, so both changelogs must read the same data.

## Requirements

- **Single source of truth**: `docs/changelog.json`, a list of releases, newest first. Each release has a `version`, a `date` (`YYYY-MM-DD`), an optional one-line `summary`, an optional `githubRelease: false` for a version published without a GitHub release, and a list of `changes`. Each change has a `type` (`feature`, `fix` or `docs`), a user-facing `title`, an optional `description` (both may mark code with backticks), an optional pull-request number `pr`, and an optional `id`.
- The dashboard's "What's new" entries are the changes that have an `id`, in file order. Existing ids (`outside-features`, `history-feature-graph`, `feature-history`, `feature-keys-links`, `specs-decisions`) are kept, so users who already saw them are not shown them again.
- **`/changelog` page** on the website, built at build time from `docs/changelog.json`:
  - Releases newest first: version, date, summary, then changes grouped under Features, Fixes and Docs (empty groups hidden).
  - Each release has an anchor `#v<version with dots replaced by dashes>` (e.g. `/changelog#v0-15-0`), a link to its GitHub release and a link to the compare view against the previous release. A release with `githubRelease: false` links its npm version page instead and has no compare link.
  - Each change links to its pull request when `pr` is set.
  - Ends with a link to the full release history on GitHub for older versions.
- **Feed**: an RSS 2.0 feed at `/changelog.xml`, one item per release, advertised with `<link rel="alternate" type="application/rss+xml">` on the changelog page.
- **Navigation**: "Changelog" in the footer (Project column). Not in the header, which already holds six links and the GitHub button.
- **SEO / GEO** (see [001](001-website-seo-pages.md) and [005](005-website-geo.md)): unique title and description, canonical URL, `BreadcrumbList` JSON-LD, one `h1`, listed in the sitemap, in `llms.txt` and with a Markdown twin (`/changelog.md`) like the other pages.
- **Design**: same monochrome Linear-style components (`PageHero`, `Section`), responsive down to 375px with no horizontal scroll.
- **Deployment**: the Vercel `ignoreCommand` also rebuilds the website when `docs/changelog.json` changes.
- Entries for `0.11.1` → `0.15.0` are written from the git history with user-facing wording, not raw commit subjects. Website-only changes are left out (the changelog is about the CLI and dashboard), except as Docs entries when they help users.
- `0.15.0` is on npm but has no GitHub release (the crates.io publish failed), so it is marked `githubRelease: false`.

## Out of scope

- Generating entries automatically from commits or GitHub releases.
- Pointing the dashboard's "Full changelog" link to the website (the website domain is not settled in the web app).
- Entries older than `0.11.1` (linked to GitHub releases instead).

## Plan

1. Add `docs/changelog.json` with releases `0.11.1` → `0.15.0`.
2. Website: `src/lib/changelog.ts` reads and validates the JSON at build time; `src/pages/changelog.astro` renders it; `src/pages/changelog.xml.ts` renders the RSS feed.
3. Add the footer link and the `llms.txt` order; update `vercel.json` `ignoreCommand`.
4. Web app: derive `CHANGELOG` in `app-menu/changelog.ts` from the JSON (small change, same exports).
5. Build, lint, format; check the page at desktop and 375px widths.
