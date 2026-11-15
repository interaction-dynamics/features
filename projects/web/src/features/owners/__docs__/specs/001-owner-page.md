---
status: implemented
date: 2026-11-15
---

# Owner page

## Context

The dashboard shows who owns each feature and has an ownership table in Overview, but nothing gathers everything about one owner (a team or a person). Engineering managers need that view: what a team owns, how big and healthy it is, what changed recently and which specs and decisions it maintains. The page is new and unproven, so it ships behind `--experimental` first, like History ([008](../../../../../../../docs/specs/008-experimental-web-features.md)).

## Requirements

- A page `/owners/:owner` shows one owner:
  - Totals: features (own and inherited), files, lines of code, test coverage (covered lines / total lines of the features that have coverage), technical debt (TODO comments), commits and contributors.
  - Data imported with `--data`: numeric metrics summed over the owner's features, other metrics shown as the number of features per value; every value listed per feature in the features table.
  - Features table: every feature whose `owner` is this owner, including features that inherit it from a parent, marked "inherited" with the feature it is inherited from. Each feature opens the feature page.
  - Recent changes: the commits of the owner's features, deduplicated by hash, newest first, paginated, reusing the History timeline. Clicking a feature badge filters the changes to that feature.
  - Specs & decisions of the owner's features, newest first, each with the feature it belongs to.
- The file, line, TODO and coverage numbers of the CLI exclude nested features, so they are summed without double counting.
- Owner names can contain `@`, `/` and spaces: the owner is URL-encoded as a single path segment (`encodeURIComponent`). Features without owner (`owner: ""`, shown as "Unknown") use the reserved segment `~unowned`.
- An owner with no feature shows a not-found state with a link back to Overview.
- Experimental: only reachable with `features serve --experimental` / `features build --experimental` (or `VITE_EXPERIMENTAL_FEATURES=true`). Otherwise `/owners/...` redirects home (`ExperimentalRoute`), and owner names stay plain text everywhere. The page header shows the `ExperimentalBadge`.
- When experimental, owner names link to the page from the Overview ownership table and from the feature owner display (feature page and Overview features table).
- Same monochrome Linear-style look, dark and light themes, existing components (`Header`, `StatsCard`, `Table`, `Tabs`).

## Out of scope

- A page listing all owners (the Overview ownership table plays that role).
- Links from the owner dots of the sidebar, the History badges and the dependency graph (dots are tooltips inside other clickable elements).
- Owners defined outside README front matter (CODEOWNERS import), team membership, owner aliases.

## Plan

1. `projects/web/src/features/owners/`: `utils.ts` (URL helpers, `summarizeOwner` aggregation, `ownerCommits`) with unit tests, `OwnerLink` (link when experimental, text otherwise), page sections (features table, specs & decisions list).
2. `src/routes/owner.tsx`, lazy route `/owners/:owner` wrapped in `ExperimentalRoute`.
3. Use `OwnerLink` in `FeatureOwner` and in the Overview ownership table.
4. Docs: `docs/cli.md` (`--experimental`), `projects/web/README.md`, in-app changelog entry.
5. End-to-end test `e2e/owner-page.spec.ts` (link from Overview, owner with spaces, not-found).
