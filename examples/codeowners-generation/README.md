# CODEOWNERS generation fixture

Example project used by `projects/cli/tests/codeowners_generation_test.rs`. It checks how `features codeowners` writes the generated section of a CODEOWNERS file. Run it from this folder (it rewrites `CODEOWNERS`, so pass `--codeowners-path` to write elsewhere):

```bash
features codeowners src --project-dir . --skip-changes --codeowners-path /tmp/CODEOWNERS
```

- The hand-written rules before and after the generated section of `CODEOWNERS` are kept, and the stale generated line (`/src/features/removed`) is replaced.
- Auth has the same owner as its parent Platform, so it gets no line of its own. Audit has another owner, so it does.
- Growth's owner already starts with `@`, so no second prefix is added. Legacy has no owner and is written as `@Unknown`.
