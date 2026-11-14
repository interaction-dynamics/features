# Specs and decisions fixture

Example project used by the tests of the "Specs & Decisions" records (see `docs/specs/002-specs-and-decisions-tab.md` at the repository root). Scan it from this folder:

```bash
features scan . --json --project-records --skip-changes
features check .
```

- `src/features/auth` and `src/features/billing`: native specs and decisions in `__docs__/` and `.docs/`.
- `docs/specs`, `docs/decisions`, `docs/adr`: root records. `002-postgres-billing` attaches to Billing with `feature: billing`, the others are project-level.
- `specs/` with `.specify/`: a GitHub Spec Kit layout. `003-auth` attaches to Auth by its folder name, `004-search` is project-level.
- `.kiro/specs/`: a Kiro layout. `billing` attaches to Billing, `notifications-center` is project-level.

`features check .` prints one warning on purpose: `002-sso` links to a decision that doesn't exist.
