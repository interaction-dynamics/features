# Web Dashboard

A demo is available at [interaction-dynamics.io/features/](interaction-dynamics.io/features/).

## Getting Started

```bash
# run the cli command first
# cd ../cli
# cargo run  ../../examples/javascript-basic/ --json > ../web/public/features.json

bun install

bun run dev
```

## Build Static Version

```bash

bun run build
```

## Experimental pages

The History and Project records pages are experimental: hidden unless `metadata.json` has `"experimental": true` (written by `features serve --experimental` and `features build --experimental`). When developing the dashboard, enable them with an environment variable:

```bash
VITE_EXPERIMENTAL_FEATURES=true bun run dev
```

The CI (end-to-end tests and the online demo) runs with the experimental pages enabled.
