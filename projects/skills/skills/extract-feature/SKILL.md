---
name: extract-feature
description: Moves the code of one feature into its own feature folder (features/<name>/) in small, safe steps, with git mv, updated imports and passing tests, without changing behavior. Use when the user asks to move, extract or group the code of a feature into a folder, or as the "move" step of the organize-by-feature workflow. Works with any language.
---

# Extract one feature into its folder

Move the files of **one** feature into its folder. Moves, renames and import updates only: behavior must not change.

Before you start, you need:

- the feature name and target path (e.g. `src/features/checkout`), and the list of files to move. Take them from the approved breakdown file (see the **plan-feature-breakdown** skill) or ask the user;
- the project's build, type check, lint and test commands, and their result before the move (the baseline);
- a clean `git status`, so the diff only shows this move.

## Steps

Copy this checklist and tick it as you go:

```
- [ ] 1. Check who uses each file
- [ ] 2. Move the files with git mv
- [ ] 3. Update imports and path references
- [ ] 4. Run the baseline commands
- [ ] 5. Review the diff
```

### 1. Check who uses each file

For each file in the list, search the repository for the places that import or reference it (its path, its module name, its exported names when imports use aliases). Then:

- used only by this feature → move it;
- also used by other features → leave it in, or move it to, a shared folder outside `features/`, and tell the user the plan changed;
- not used at all → move it with the feature anyway and mention it in your summary. Don't delete code in this skill.

### 2. Move the files

Create the folder, then move each file with `git mv` so git records a rename and `git log --follow` still finds the file's history:

```bash
mkdir -p src/features/checkout/components
git mv src/components/CheckoutForm.tsx src/features/checkout/components/CheckoutForm.tsx
```

Keep the subfolders the project already uses inside the feature (`components/`, `services/`…). Move tests, stories, styles, fixtures and snapshots together with the code they cover, unless the test runner needs them elsewhere.

Don't edit the content of a file in the same step as moving it, beyond its imports.

### 3. Update imports and path references

Update every reference to a moved file:

- imports **in** the moved files (relative paths change),
- imports **of** the moved files everywhere else,
- references that aren't imports: test mocks (`jest.mock`, `vi.mock`, `patch(...)`), lazy imports and route tables, config globs (`tsconfig`, coverage, lint, Storybook, bundler entries), CI paths, `CODEOWNERS` lines, docs links.

Follow the import style the project already uses (path aliases such as `@/`, relative paths, absolute module paths). Read [references/moving-code.md](references/moving-code.md) for language-specific details (Python packages, Go packages, Rust modules, Java packages…).

Keep feature metadata annotations (comments starting with `--feature-`) unchanged: they move with the code.

Don't add an `index` or barrel file unless the project already uses them for its modules.

### 4. Run the baseline commands

Run the build, type check, lint and tests. Compare with the baseline:

- a failure caused by a path or import → fix it and run again;
- a failure that was already in the baseline → leave it;
- anything else → stop, show the error to the user and ask. Don't change logic to make a test pass.

### 5. Review the diff

```bash
git status
git diff -M --stat HEAD
```

Every moved file should appear as a rename (`R`), with only import lines changed. If a file shows up as deleted and added, check its content didn't change by accident.

Then run `features scan PATH --skip-changes` (or `npx features-cli@latest scan PATH --skip-changes`) and check the new feature is listed. The feature has no README yet: continue with the **document-feature** skill.

## Report

Tell the user: the files moved, the files you left in shared folders and why, the references you updated outside the feature, and the result of the baseline commands.
