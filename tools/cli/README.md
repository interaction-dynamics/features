# Features CLI

A zero-config CLI tool for discovering and managing features in a feature-based architecture project by detecting README.md or README.mdx files in directories.

## Getting Started

```bash

cargo run ../../examples/demo/src/features

# For the web dashboard
cd ../web
pnpm build_for_cli
cd ../cli

cargo run ../../examples/demo/src/features --serve # for the Web dashboard
```

## Deploy

```bash
cd ../web
pnpm build_for_cli
cd ../cli

cargo publish --allow-dirty
```
