# Feature README template

Based on the features-cli [README format](https://github.com/interaction-dynamics/features/blob/master/docs/readme-format.md). Every frontmatter property is optional; remove the ones you can't source.

```markdown
---
owner: team-payments # team or user handle, without @
creation_date: 2022-01-01 # date the oldest file of the feature was added
status: active # active, deprecated or archived
# feature: true      # only when the folder isn't directly inside a `features` folder
# deprecated: true   # when the feature is being removed
# figma: https://... # any other key you need is shown as metadata
---

# Checkout

Lets a signed-in user pay for the items in their cart and get an order confirmation.

## How it works

- Entry point: the `/checkout` route renders `CheckoutPage`.
- `services/payment.ts` talks to the payment provider; `services/order.ts` creates the order once the payment succeeds.
- Uses the shared `components/Button` and the `cart` feature's public API (`features/cart/index.ts`).

## Related

- Decisions: see `__docs__/decisions/`.
- Depends on: [cart](../cart/README.md).
```

## Good descriptions

- **Specific**: "Lets a signed-in user pay for the items in their cart" rather than "Checkout feature".
- **User-facing first**: what the feature lets someone do, then how it's built.
- **Short**: a reader should get the picture in under a minute. Link to longer docs instead of copying them.

## A folder that isn't a feature

To stop a folder directly inside `features` from being detected as a feature (for example a `shared` folder), give it a README with:

```markdown
---
feature: false
---
```

Its subfolders are still scanned, so nested features are still found.
