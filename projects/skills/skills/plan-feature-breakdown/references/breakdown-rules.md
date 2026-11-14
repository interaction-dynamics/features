# Breakdown rules

These rules come from the features-cli [folder architecture guidelines](https://github.com/interaction-dynamics/features/blob/master/docs/folder-architecture-guidelines.md), [README format](https://github.com/interaction-dynamics/features/blob/master/docs/readme-format.md) and [FAQ](https://github.com/interaction-dynamics/features/blob/master/FAQ.md).

## Contents

- How features are detected
- Choosing the layout
- Naming features
- Where each file goes
- Nesting features
- Ordering the migration

## How features are detected

features-cli finds a feature in two ways:

1. **A folder inside a `features` folder** (recommended): every direct subfolder of a folder named `features` is a feature. The `features` folder can be anywhere, including inside another feature.
2. **`feature: true` in a README frontmatter**: any folder whose `README.md` (or `README.mdx`) starts with this frontmatter is a feature, wherever it is.

`feature: false` in a README stops a folder from being a feature, even directly inside a `features` folder. Its subfolders are still scanned.

The feature name is the first `#` heading of its README, or the folder name when there is no heading.

## Choosing the layout

- **Default: one `features` folder** at the source root (`src/features/<name>/`), next to the shared folders:

  ```
  src/
  ├── features/
  │   ├── checkout/
  │   └── user-profile/
  ├── components/   # shared by several features
  ├── utils/        # shared by several features
  └── main.ts       # application glue
  ```

- **Monorepo**: one `features` folder per package or app (`apps/web/src/features/`, `packages/api/src/features/`). Names still have to be unique across the scanned folder.
- **Framework with a fixed folder structure** (file-system routing such as Next.js `app/` or `pages/`, Rails, Django apps, Angular modules): keep the files the framework needs where it needs them, and make them thin (a route file that imports and renders the feature). Put the feature's code in `features/<name>/`. When a framework folder already groups a feature's code (a Django app, a Rails engine), mark that folder with `feature: true` in its README instead of moving it.
- **Existing grouping by domain**: if the code is already grouped by domain under another name (`modules/`, `domains/`), ask the user whether to rename the folder to `features` or to add `feature: true` READMEs. Renaming is closer to the recommended convention; READMEs are less disruptive.

## Naming features

- Use product language: `checkout`, `user-profile`, `invoice-export`. Avoid technical names (`helpers`, `api`, `hooks`) and vague ones (`misc`, `core`, `common`).
- kebab-case, camelCase and snake_case all work. Follow the casing the repository already uses for folders.
- The folder name is the feature's id, used in feature metadata annotations (`--feature-flag feature: checkout, ...`). Keep it stable.
- Make names unique across the scanned folder, including nested features and README titles: `features check` fails on duplicate names. Prefix with the parent when needed (`admin-settings`, `user-settings`).

## Where each file goes

| The file is… | It goes… |
| --- | --- |
| Used by one feature only | Inside that feature's folder |
| Used by two or more features | A shared folder outside `features/` (`components/`, `utils/`, `lib/`…), or kept where it is |
| Application glue: entry point, router setup, dependency injection, global config, global styles | Where it is |
| A test, story, fixture, mock or style of some code | Next to that code, or in the project's existing test layout mirrored per feature if the test runner requires a separate folder |
| Generated or vendored | Where it is. Don't move generated code; move its source or config if it belongs to a feature |

Inside a feature folder, keep the subfolders the project already uses (`components/`, `hooks/`, `services/`) so the code stays familiar. A feature with few files can stay flat.

When a file mixes code for several features (a large `api.ts` with one function per feature), plan to split it, but only by moving functions, not by rewriting them.

## Nesting features

A feature can contain its own `features` folder. Nest when a feature is large and has parts with a clear identity of their own (`checkout/features/payment`, `checkout/features/shipping`). A nested feature inherits its parent's owner unless it sets its own.

As a rule of thumb, consider splitting a feature with more than about 40 files or 4,000 lines, but only along real product boundaries. Don't nest just to make folders smaller.

## Ordering the migration

1. Features that depend on no other planned feature (leaves) first.
2. Then features that only depend on already-moved features.
3. Shared code last, once you know which code is really shared.

Moving one feature per step keeps every step small enough to review and revert.
