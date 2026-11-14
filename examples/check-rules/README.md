# check-rules example

Fixture for `features --check`. Every rule enabled in `.features.yml` finds something:

| Rule | Finding |
| --- | --- |
| `duplicate-names` | two features named `invoices` |
| `missing-owner` | `checkout` has no owner, both `invoices` only inherit one |
| `missing-readme` | `checkout` has 2 files and no README |
| `min-coverage` | `billing` has 25% line coverage (`coverage/lcov.info`) |
| `stale-flag` | `billing` has a flag introduced on 2020-01-01 |
| `cross-feature-import` | `billing` imports from `checkout` (imports from `shared` are allowed) |

```bash
cd examples/check-rules
features src --check --skip-changes
```
