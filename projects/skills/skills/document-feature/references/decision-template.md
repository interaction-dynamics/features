# Decision record template

Inspired by [MADR](https://adr.github.io/madr/), with the sections listed in the features-cli [FAQ](https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-decision). Save it as `__docs__/decisions/NNN-kebab-case-title.md` in the feature folder (`.docs/decisions/` also works), numbered from `001`.

```markdown
---
status: accepted # proposed, accepted, rejected, deprecated or superseded
date: 2026-01-04
spec: 001-checkout-flow # optional: the spec this decision serves
supersedes: 001-paypal-only # optional: the decision this one replaces
---

# Use Stripe Checkout for card payments

## Context

What problem had to be solved, and the constraints at the time. Cite the source: a commit, a pull request, an issue or a code comment.

## Considered options

- Stripe Checkout (hosted page)
- Stripe Elements (embedded form)
- Build our own card form

## Decision

Chosen option: "Stripe Checkout", because it keeps card data out of our servers (no PCI scope) and was the fastest to ship.

## Consequences

- Good: no card data on our servers.
- Bad: limited control over the payment page design.
```

## Rules

- One decision per file. A short, specific title that states the choice.
- Write `proposed` when the decision is inferred from the code without written evidence, and ask the user to confirm it.
- Don't rewrite history: when a decision changes, add a new record with `supersedes: <old id>` and set the old one's status to `superseded`.
