---
title: Folder structure
description: The conventions features-cli uses to detect features, from the features folder and the feature flag in a README to specs, decisions and metadata annotations.
---

Features needs two conventions to detect a feature, and a few more to describe it. The [Guidelines page](/guidelines) shows them with illustrations; this page is the checklist.

## Two ways to declare a feature

**A `features` folder.** Every direct subfolder of a folder named `features` is a feature. The `features` folder can be anywhere in the source tree, and there can be several.

```text
src/
├── features/
│   ├── save-user/          # a feature
│   │   ├── components/
│   │   ├── utils/
│   │   └── README.md
│   └── checkout/           # a feature
├── components/             # shared code, not a feature
└── utils/
```

**A flag in a README.** Any folder whose README front matter has `feature: true` is a feature, wherever it is:

```markdown
---
feature: true
---

# Payments
```

Both methods can be mixed. Folder names can be in snake_case, camelCase or kebab-case. Features can be nested: a feature can have its own `features` folder.

To skip a folder, set `feature: false` in its README. Unlike [`--ignore-path`](/docs/options#scanning), its subfolders are still scanned, so the features nested inside are still found.

## Checklist

| Level      | Rule                                                                                                      | Example                                    |
| ---------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| **MUST**   | Create features in a `features` folder, or add `feature: true` to the README front matter                 | `features/feature-1/`                      |
| **MUST**   | Put the code of the feature inside its folder                                                             | `feature-1/file.js`                        |
| **SHOULD** | Add a `README.md` or `README.mdx` to the feature folder                                                   | `features/feature-1/README.md`             |
| **SHOULD** | Describe the feature in a line of text after the README title                                             | `Handles user login and logout`            |
| **SHOULD** | Add an `owner` to the README front matter                                                                 | `owner: backend-team`                      |
| **MAY**    | Add any other property to the front matter                                                                | `figma: <url>`                             |
| **MAY**    | Add [MADR](https://adr.github.io/madr/) decision records to the feature's decisions folder                | `__docs__/decisions/001-use-jwt-tokens.md` |
| **MAY**    | Add specs (a file, or a folder with a `spec.md`) to the feature's specs folder, linked to their decisions | `__docs__/specs/001-login-flow.md`         |
| **MAY**    | Add project-level specs and decisions at the root, in `docs/specs`, `docs/decisions` or `docs/adr`        | `docs/decisions/001-use-a-monorepo.md`     |
| **MAY**    | Keep the specs of GitHub Spec Kit (`specs/NNN-name/`) or Kiro (`.kiro/specs/<name>/`) at the root         | `specs/003-checkout/spec.md`               |

The [README format](/docs/readme-format) page lists the front matter properties, and [Specs and decisions](/docs/specs-decisions) the record format. `features check` validates the result: see [Check rules](/docs/check-rules).

## What is scanned

- `node_modules` is skipped. `--include-skipped-dirs` scans it too, and `--ignore-path` skips more folders (with their subfolders).
- Each file belongs to its closest feature. Files outside any feature are listed apart, in the _Outside features_ page of the dashboard and in `features diff`.
- The owner of a feature without `owner` is inherited from its parent feature.

## Annotations

Comments attach metadata to a feature from any file, in any language. The metadata key becomes a tab on the feature page of the dashboard, with one column per property:

```ts
// --feature-flag feature: feature-1, name: new-login, type: experiment, owner: team-a, introduced_on: 2025-01-01
```

The syntax is `--feature-<metadata-key>` followed by `key: value` pairs separated by commas. `feature:` names the feature by its folder name; it can be left out when the comment is inside the feature folder. `--feature-flag` annotations with an `introduced_on` date are checked by the [`stale-flag`](/docs/check-rules#stale-flag) rule. The [Guidelines page](/guidelines#annotations) lists the supported languages.
