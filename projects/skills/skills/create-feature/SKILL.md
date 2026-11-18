---
name: create-feature
description: Creates a new feature in a codebase organized by feature, following the features-cli conventions - checks that no existing feature already covers it, picks the folder (top-level or sub-feature), writes its README (owner, title, description) and first spec, builds the code inside the folder without reaching into other features, records decisions and validates with features check. Use when the user asks to add, build, start or scaffold a new feature, module or capability ("add a referral program", "create a feature for invoices"). Not for changes inside an existing feature.
---

# Create a feature

Start a new feature so that it is detected, owned and documented from its first commit: its code in one folder, a README that says what it does and who owns it, a spec for what is being built, and decision records for the choices made along the way.

If `features` isn't installed, use `npx features-cli@latest` in place of `features` below. `PATH` is the scanned source root (e.g. `./src`); in a monorepo, the package the feature belongs to.

## 1. Look for an existing feature first

Map the features with the **explore-features** skill (or `list_features` / `get_feature` when the features MCP server is connected):

```bash
features scan PATH --json --skip-changes > "${TMPDIR:-/tmp}/features.json"
python3 ../explore-features/scripts/feature_map.py "${TMPDIR:-/tmp}/features.json"
```

Search names, descriptions, specs and folders for the same capability, with the user's words and their synonyms (`referral` / `invite` / `sponsor`). Also look for code that already does part of it outside any feature.

- A feature already covers it → stop, show it to the user and propose to extend it (or to add a sub-feature to it) instead of creating a second one.
- Part of it exists as shared code → reuse it; don't copy it into the new feature.
- No feature is detected at all (`[]`) and the codebase isn't organized by feature → tell the user, and propose the **organize-by-feature** skill or a first `features/` folder for this feature only.

## 2. Choose the folder

Pick the location with these rules, then **show it to the user before writing anything** (folder, name, owner):

- **Sub-feature**, `<parent folder>/features/<name>/`, when the new feature only makes sense inside an existing one (a "refund" flow inside `checkout`). It inherits the parent's owner.
- **Top-level feature** otherwise, in the `features/` folder of `PATH`. If there are several `features` folders that could fit (e.g. `web/src/features` and `api/src/features`), ask which one, or whether the feature spans several packages.
- **Folder name**: follow the style of the sibling folders (`kebab-case`, `camelCase`, `PascalCase`). Name it after what the user can do, not after a technical layer (`invoices`, not `invoice-service`).
- **Unique name and key**: the README title must not be the name of another feature (`features check` fails on duplicates), and the folder name must not give the same key as another feature (folder name lowercased, non `[a-z0-9_-]` characters replaced by `-`).

A folder that isn't directly inside a `features` folder needs `feature: true` in its README frontmatter. Prefer a `features` folder when the project has one.

## 3. Write the README

Create `README.md` at the root of the folder with the **document-feature** rules and its [README template](../document-feature/references/readme-template.md):

- **Owner**: ask the user, or take it from the parent feature (then leave `owner` out), the team that asked for the feature, or `CODEOWNERS` rules that already cover the folder. Never invent one. If nobody knows, add a `# TODO: owner` comment line in the frontmatter, never a placeholder value.
- **Title** (the first `#` heading): the feature's display name.
- **Description**: one sentence right after the title, written for a product manager: what the user can do.
- Leave `creation_date` out: features-cli dates a new feature from its first commit.

Keep the rest short for now ("How it works" gets written once the code exists, in step 5).

## 4. Write the first spec

When the user described what to build (requirements, a ticket, a conversation), write it down before coding:

```bash
features new spec <feature> "<title>" --path PATH
```

`<feature>` is the feature's name or its path relative to `PATH` (e.g. `features/invoices`). It creates `__docs__/specs/001-<title>.md` in the feature, with the next free number. Fill it with the [spec template](../document-feature/references/spec-template.md): `status: draft`, Context, Requirements, Out of scope, Plan with `- [ ]` tasks. Ask the user to review it, then set `status: accepted`. Skip the spec for a trivial feature, or when the project keeps its specs elsewhere (an issue tracker, `docs/specs/` at the root with `feature: <name>`).

## 5. Build the code inside the folder

- Put everything that belongs to this feature in its folder: UI, API handlers, services, state, types, tests (when the project's test setup finds tests there; otherwise follow the project's test layout).
- Use the project's existing structure inside feature folders: open a sibling feature and mirror its subfolders (`components/`, `hooks/`, `api/`...) and its entry point (`index.ts`, `__init__.py`, `mod.rs`).
- **Shared code stays shared**: reuse the shared folders (`components/`, `utils/`, `lib/`) instead of copying code. Move code to a shared folder only when a second feature needs it, and tell the user.
- **Other features through their front door**: import other features only through their public entry point, never their internal files. If you need something internal, ask whether it should become part of that feature's public API.
- Wire the feature into the application (routes, navigation, dependency injection, feature flags) in the application's glue code, outside the feature folder, the way the existing features are wired.
- Update the README's "How it works" (entry points, main concepts, features it depends on) and tick the spec's tasks as they get done.

## 6. Record decisions

When a real choice comes up (a library picked over another, a data model, a trade-off, a deliberate limitation), write a decision record with the **document-feature** rules:

```bash
features new decision <feature> "<title>" --path PATH
```

Link it to the spec with `spec: 001-<title>` in its frontmatter. A decision the user made is `accepted`; one you made on your own is `proposed` until the user confirms it.

## 7. Check

```bash
features check PATH
features scan PATH --json --skip-changes > "${TMPDIR:-/tmp}/features.json"
python3 ../validate-features/scripts/feature_report.py "${TMPDIR:-/tmp}/features.json" --root PATH
```

- `features check` passes, without warnings about the new feature or its records. If it already failed before you started (duplicates elsewhere in the project), make sure the new feature adds no error or warning, and mention the existing ones to the user without fixing them.
- The report lists the new feature at the planned path, with its owner and description, and no *imports from sibling feature* warning for it (see the **validate-features** skill to fix one).
- The project's own build, type check, lint and tests pass.

Before opening a pull request, list the features and owners the branch touches (use the repository's default branch in place of `main`) and include it in the description:

```bash
features diff PATH --since origin/main
```

Then tell the user what was created (folder, README, spec, decisions), what is still `TODO` (owner, `proposed` decisions, open spec tasks), and anything you changed outside the feature folder (wiring, shared code).

## Rules

- **One feature, one folder.** Code that is only used by this feature lives in it.
- **Ask before writing** when the location, the owner or the scope is unclear, and before changing another feature.
- **Don't invent facts.** Owners and decisions come from the user, the parent feature, `CODEOWNERS` or existing docs. Write a visible `TODO` instead of a guess.
- **Follow the conventions** in the [folder architecture guidelines](https://github.com/interaction-dynamics/features/blob/master/docs/folder-architecture-guidelines.md) and the [README format](https://github.com/interaction-dynamics/features/blob/master/docs/readme-format.md).
