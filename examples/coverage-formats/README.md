# Coverage formats fixture

Example project used by `projects/cli/tests/coverage_formats_test.rs`. It checks that a Cobertura report and an lcov report in the same coverage folder are both read, with line and branch coverage, and mapped to the right feature. Scan it from this folder:

```bash
features scan src --json --skip-changes --coverage
```

- `.coverage/cobertura.xml` covers API (with a branch from `condition-coverage`) and `jobs/worker.py`.
- `.coverage/lcov.info` covers Web (with `BRDA` branches), its nested Forms feature, `jobs/scheduler.js` and `shared/version.js`.
- Jobs gets lines from both reports, added up. `shared/version.js` is in no feature, so it is counted nowhere. Forms is counted apart from its parent Web.
