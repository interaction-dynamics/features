# Moving code: language notes

What to update besides the file path when moving code, per language. In every language, prefer the project's own tools (IDE refactorings exposed through a CLI, `cargo`, `go`, `dotnet`…) over hand edits when they exist.

## Contents

- JavaScript and TypeScript
- Python
- Go
- Rust
- Java, Kotlin and Scala
- C#
- Ruby
- PHP
- C and C++
- CSS, SCSS and Less

## JavaScript and TypeScript

- Update relative imports in both directions, and path aliases (`@/features/...`) if the project uses them. Check `tsconfig.json` `paths`, bundler aliases and `package.json` `imports`.
- Search for the old path in strings too: `jest.mock('...')`, `vi.mock('...')`, `import('...')`, `require('...')`, `React.lazy`, route configs, Storybook `stories` globs, coverage `include`/`exclude`, ESLint overrides.
- CSS modules, images and other assets imported by the moved files must move with them or keep a valid path.
- Snapshot files (`__snapshots__/`) move with their test.
- Run the type checker (`tsc --noEmit` or the project's script): it catches most broken imports.

## Python

- Every folder in the import path needs an `__init__.py` if the project uses regular packages. Create empty ones in the new folders when the old folders had them.
- Update absolute imports (`from app.services.checkout import ...` → `from app.features.checkout.services import ...`) and relative ones.
- Search for dotted paths in strings: `mock.patch("app.services.checkout.charge")`, Django `INSTALLED_APPS`, Celery task names, entry points in `pyproject.toml`.
- Renaming a module changes the name of pickled objects and Celery task names: tell the user if the code relies on them.

## Go

- A folder is a package. Moving files to a new folder creates a new package: update the `package` clause and the import paths (module path + folder).
- Unexported identifiers used across the old package won't be visible from the new one. Exporting them changes the API: ask the user before doing it, or move the whole package instead of single files.
- `go build ./... && go vet ./... && go test ./...` catch the rest.

## Rust

- Update `mod` declarations: remove the old `mod x;`, add `pub mod features;` and `mod checkout;` (with `mod.rs` or `checkout.rs` files) along the new path.
- Update `use crate::...` paths. A `pub use` re-export at the old location can keep the change small, but remove it once callers are updated.
- Run `cargo check --all-targets` and `cargo test`.

## Java, Kotlin and Scala

- The `package` declaration must match the folder: update it in each moved file, and update `import` statements everywhere.
- Package-private members become invisible across packages. Making them public changes the API: ask first.
- Check Spring component scanning, JPA entity scanning and other configuration that lists packages.

## C#

- Update the `namespace` to match the new folder if the project follows that convention, and the `using` directives.
- `.csproj` files with explicit `Compile Include` entries need updating; SDK-style projects pick files up automatically.

## Ruby

- Rails autoloading (Zeitwerk) maps folders to constants, and every direct subfolder of `app/` is an autoload root. So `app/features/checkout/order.rb` must define `Checkout::Order`, not `Order`. Either namespace the moved classes (this renames constants: ask the user first) or add the feature's subfolders to the autoload paths in `config/application.rb`. When a Rails engine or a folder already groups a feature, marking it with `feature: true` in a README is often simpler than moving it.
- Update `require` and `require_relative` paths.

## PHP

- PSR-4 autoloading maps namespaces to folders: update the `namespace` and `use` statements, then run `composer dump-autoload`.

## C and C++

- Update `#include` paths and the build files (CMake `add_library`/`target_sources`, Makefiles, Bazel `BUILD`).

## CSS, SCSS and Less

- Update `@import` and `@use` paths and `url(...)` references to assets.
