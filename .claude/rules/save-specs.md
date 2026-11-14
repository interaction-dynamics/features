# Save specs automatically

Whenever a spec is agreed on, save it to the repository as a Markdown file without waiting to be asked.

## When to save

Save a spec when any of these happens:

- A plan is approved after plan mode (`ExitPlanMode` accepted).
- The user describes the requirements or expected behavior of a feature or change and confirms them.
- A spec already saved changes during the conversation (update the existing file, don't create a new one).

Do not save specs for trivial changes (typo fixes, dependency bumps, one-line tweaks).

## Where to save

Follow the feature folder convention from [docs/folder-architecture-guidelines.md](../../docs/folder-architecture-guidelines.md):

- Spec for a single feature → `<feature-folder>/__docs__/specs/<NNN>-<kebab-case-title>.md`
  (e.g. `projects/web/src/features/sidebar/__docs__/specs/001-collapsible-groups.md`), next to the feature's `__docs__/decisions/`.
- Spec spanning several features or the whole project → `docs/specs/<NNN>-<kebab-case-title>.md`.

`<NNN>` is the next free three-digit number in that `specs` folder, starting at `001`. Create the folder if it doesn't exist.

## Format

```markdown
---
status: draft # draft, accepted, implemented, superseded
date: YYYY-MM-DD
---

# <Title>

## Context

Why this change is needed.

## Requirements

- What must be true when it is done (behavior, constraints, edge cases).

## Out of scope

- What is explicitly not covered.

## Plan

High-level implementation steps (copy the approved plan when there is one).
```

## Keeping it current

- Set `status: accepted` once the user approves it, and `status: implemented` when the work is done.
- If the implementation diverges from the spec, update the spec in the same change.
- Commit the spec together with the code it describes.
- Tell the user the path of the spec file you saved or updated.
