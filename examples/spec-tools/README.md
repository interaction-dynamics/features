# Spec Kit and Kiro fixture

Example project used by the tests of the GitHub Spec Kit and Kiro readers (see `docs/specs/004-spec-kit-and-kiro-readers.md` at the repository root). Scan it from this folder:

```bash
features scan . --json --skip-changes
features records . --json
```

- `specs/` with `.specify/`: a GitHub Spec Kit layout. `003-auth` attaches to Auth by its folder name, `004-search` is project-level.
- `.kiro/specs/`: a Kiro layout. `billing` attaches to Billing by its folder name, `notifications-center` is project-level.
