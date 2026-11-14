# Spec template

A spec says what to build: the requirements a change must meet and the plan to get there. Save it as `__docs__/specs/NNN-kebab-case-title.md` in the feature folder (`.docs/specs/` also works), numbered from `001`, or let `features new spec <feature> "<title>"` pick the number. A large spec can be a folder, `__docs__/specs/NNN-title/`, with `spec.md` as its main file and `plan.md` or `tasks.md` next to it.

```markdown
---
status: accepted # draft, accepted, implemented or superseded
date: 2026-01-15
decisions: [002-use-stripe-checkout] # decision records taken for this spec
---

# Card payments

## Context

Why this change is needed.

## Requirements

- What must be true when it is done (behavior, constraints, edge cases).

## Out of scope

- What is explicitly not covered.

## Plan

- [x] Create the checkout session endpoint
- [ ] Handle the payment webhook
```

## Rules

- Write the requirements as observable behavior, not as code.
- Checklists (`- [ ]`, `- [x]`) show as a progress bar in the dashboard. Set `status: implemented` only when every task is checked: `features check` warns otherwise.
- Link the decisions taken for the spec in `decisions:`; the decisions get the link back automatically.
