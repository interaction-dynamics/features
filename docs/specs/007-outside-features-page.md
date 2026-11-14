---
status: implemented
date: 2026-11-14
---

# Outside features page

## Context

The web dashboard had two pages for things that do not sit inside the feature tree:

- **Project records** (`#/records`): root `docs/specs`, `docs/decisions` and `docs/adr` records not attached to a feature. Shown only for single-repository reports (`records.json` is not written for aggregated builds).
- **Reconciliation issues** (`#/reconciliation`): features that could not be merged across repositories (loops, conflicting parents). Shown only for aggregated reports.

A report only ever showed one of the two, so the sidebar link changed name and meaning depending on the report. "Reconciliation issues" also sounded alarming, and "Project records" did not say these records were the ones outside every feature.

## Requirements

- One page, **Outside features**, at `#/outside-features`, with a `Shapes` icon in the header and the sidebar.
- The page has up to two sections:
  - **Not attached to a feature**: the project-level records (previous Project records content, with its empty state). Shown for single-repository reports, and for aggregated reports only when records exist.
  - **Not merged across repositories**: the previous Reconciliation issues content (stats cards, one card per error, "All features were merged without issues." when there are none). Shown only for aggregated reports.
  - Each section heading shows its item count.
- Project-wide records are not presented as problems: neutral wording, no warning color. The warning color is used only for merge errors.
- The sidebar shows a single **Outside features** link for every report. Its badge is the number of project records plus the number of merge errors, hidden when 0. Its icon turns amber when there are merge errors.
- `#/records` and `#/reconciliation` redirect to `#/outside-features` so old links keep working.
- The "Not merged" notice in a feature's repositories panel links to the new page.
- README, FAQ, `docs/cli.md`, the demo spec and the demo workflow comment use the new name.

## Out of scope

- Changes to the CLI, `records.json` or `features.json` formats.
- Writing `records.json` for aggregated builds.
- Anchors to jump straight to one section from the redirects.

## Plan

1. Move the reconciliation page body into `features/reconciliation/unmerged-features.tsx` (`UnmergedFeatures`) and the records page body into `features/records/project-records.tsx` (`ProjectRecords`).
2. Add `routes/outside-features.tsx` composing both sections; add redirects for the former routes in `index.tsx`.
3. Replace the two sidebar links with one link and a combined badge.
4. Update the link in `feature-repositories.tsx`, the docs and the e2e tests (navigation and redirects).
