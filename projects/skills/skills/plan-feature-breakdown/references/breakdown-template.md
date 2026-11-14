# Breakdown file template

Fill in every section. Write `TODO` where you need the user's input and list it under "Open questions".

````markdown
---
status: draft # draft, accepted, implemented
date: YYYY-MM-DD
---

# Feature breakdown of `<source root>`

## Context

What the codebase does, how it is organized today, and the features that already exist (from `features scan`).

## Layout

Where the features go (e.g. `src/features/<name>/`) and why. Mention framework folders that stay in place.

## Features

Migration order, top to bottom.

| # | Feature | Path | Owner | Description | Depends on | Done |
| - | ------- | ---- | ----- | ----------- | ---------- | ---- |
| 1 | Checkout | `src/features/checkout` | team-payments | Lets a signed-in user pay for their cart | cart | [ ] |
| 2 | … | | TODO | | | [ ] |

### <Feature name>

Files to move:

- `src/components/CheckoutForm.tsx` → `src/features/checkout/components/CheckoutForm.tsx`
- `src/services/checkout.ts` → `src/features/checkout/services/checkout.ts`
- `tests/checkout.test.ts` → `src/features/checkout/checkout.test.ts`

Files to split: (if any) which functions go where.

Decisions to record: (if any) design choices found in the code, comments or docs that deserve a decision record.

## Shared code

Files that stay outside the features, and why (used by which features).

## Open questions

- Owner of <feature>?
- Does <file> belong to <feature A> or <feature B>?
````
