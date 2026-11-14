# Metadata comments fixture

Example project used by `projects/cli/tests/metadata_comments_test.rs`. It checks that `--feature-<key>` comments are read with the comment syntax of each language and merged into the `meta` of the feature that holds the file. Scan it from this folder:

```bash
features scan src --json --skip-changes
```

- Search: `#` comments in Python and a `/* */` block comment in Go, with a `flag` and an `experiment` key.
- Accounts: `#` comments in Ruby and YAML, and a `--` comment in SQL that names its feature explicitly with `feature:features/accounts`.
- Storefront: `<!-- -->` in HTML, `/* */` in CSS and `--` in Lua. `notes.txt` mentions `--feature-flag` without properties, so it is ignored.
