# Spec template

Save it as `__docs__/specs/NNN-kebab-case-title.md` in the feature folder (`.docs/specs/` also works), numbered from `001`. For a large spec, use a folder `__docs__/specs/NNN-kebab-case-title/` with this file as `spec.md` and the plan and tasks in `plan.md` and `tasks.md`.

```markdown
---
status: draft # draft, accepted, implemented, superseded
date: 2026-01-02
decisions: [002-use-jwt] # optional: decisions taken for this spec
---

# Login flow

## Context

Why this change is needed: the problem, who has it, the constraints.

## Requirements

- [ ] Users sign in with an email and a password
- [ ] The account is locked after 5 failed attempts

## Out of scope

- Single sign-on

## Plan

1. Build the form
2. Issue a token
```

## Rules

- One spec per change worth reviewing. A short title that names the behavior, not the implementation.
- Requirements say what must be true when it's done, as checkboxes. Tick them as they ship.
- Set `status: accepted` once the user approves the spec and `status: implemented` when every task is done.
- When a spec is replaced, add a new one with `supersedes: <old id>` and set the old one's status to `superseded`.
