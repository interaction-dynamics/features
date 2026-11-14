# Decision record template

Inspired by [MADR](https://adr.github.io/madr/), with the sections listed in the features-cli [FAQ](https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-spec-or-a-decision). Save it as `__docs__/decisions/NNN-kebab-case-title.md` in the feature folder (`.docs/decisions/` also works), numbered from `001`, or let `features new decision <feature> "<title>"` pick the number.

```markdown
---
status: accepted # proposed, accepted, rejected, deprecated or superseded
date: 2026-01-15
spec: 001-card-payments # the spec this decision belongs to, if any
supersedes: 001-use-paypal # the decision it replaces, if any
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
- Don't rewrite history: when a decision changes, add a new record with `supersedes: NNN-old-title` and set the old one's status to `superseded`. `features check` warns about a superseded record that nothing supersedes.
- Older records without frontmatter still work: features-cli reads a `**Status**: Accepted` line or a `## Status` section.
