# Website

Marketing and documentation website for `features-cli`. It explains the concept of feature-based architecture and the tool with visual pages, built for search engines: static HTML, per-page meta tags, Open Graph image, JSON-LD structured data, `sitemap-index.xml` and `robots.txt`. It is also built for AI assistants and answer engines (GEO), see [AI assistants](#ai-assistants-geo).

Built with [Astro](https://astro.build) and Tailwind CSS. Hosted on [Vercel](https://vercel.com).

## Pages

| Path                          | Content                                                                   | Source                                                            |
| ----------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `/`                           | What Features does, how it works, capabilities, dashboard, specs teaser   | `README.md`                                                       |
| `/feature-based-architecture` | The concept, layered vs feature-based, anatomy of a feature               | `FAQ.md`, `README.md`                                             |
| `/guidelines`                 | Folder structure, README front matter, specs and decisions, annotations   | `docs/folder-architecture-guidelines.md`, `docs/readme-format.md` |
| `/specs-decisions`            | Specs and decision records, Spec Kit and Kiro, `new` and `check` commands | `docs/folder-architecture-guidelines.md`, `FAQ.md`                |
| `/cli`                        | Install, commands, options, `diff` + GitHub Actions, CODEOWNERS, coverage | `README.md`, `FAQ.md`                                             |
| `/faq`                        | Frequently asked questions (with `FAQPage` structured data)               | `FAQ.md`                                                          |
| `/changelog`                  | Releases newest first, plus an RSS feed at `/changelog.xml`               | `docs/changelog.json` (also read by the dashboard's "What's new") |

The content mirrors the repository docs. When the README, FAQ or docs change, update the matching page.

To publish a release in the changelog, add it at the top of [`docs/changelog.json`](../../docs/changelog.json) with user-facing wording. Give a change an `id` to also show it in the dashboard's "What's new" menu. See [spec 011](../../docs/specs/011-website-changelog.md).

## AI assistants (GEO)

To be read and cited by ChatGPT, Claude, Perplexity, Gemini and other answer engines:

- `robots.txt` allows the AI crawlers by name and points to `llms.txt`.
- [`integrations/llms.ts`](./integrations/llms.ts) runs after the build and converts the `<main>` of every page to Markdown: `/cli.md`, `/index.md`... plus [`/llms.txt`](https://llmstxt.org) (index of the site) and `/llms-full.txt` (every page in one file). Pages link their Markdown twin with `<link rel="alternate" type="text/markdown">`.
- The Markdown follows the pages automatically. Icons, SVGs and `aria-hidden` elements are dropped and `role="img"` illustrations become their `aria-label`. When a component is visual or interactive, give it a plain-text version with `data-markdown-code` (and `data-markdown-lang`), like `FileTree` and the install picker on `/cli`.

Check the output in `dist/*.md` and `dist/llms.txt` after `bun run build`.

## Getting started

```bash
bun install
bun run dev
```

```bash
bun run build   # static site in dist/
bun run preview
bun run format  # oxfmt (format:check to only check)
bun run lint    # oxlint
```

## Deploying on Vercel

Create a Vercel project from this repository with **Root Directory** set to `projects/website`. The framework (Astro) and the settings in [`vercel.json`](./vercel.json) are picked up automatically:

- clean URLs without trailing slash (`/cli`, not `/cli.html` or `/cli/`)
- long-term caching for hashed assets in `/_astro/`
- builds are skipped when nothing changed in `projects/website`

Canonical URLs, the sitemap and Open Graph tags need the absolute site URL. On Vercel the production domain (`VERCEL_PROJECT_PRODUCTION_URL`) is used by default. Set the `SITE_URL` environment variable (e.g. `https://features.example.com`) to force another one.

## Dashboard screenshots

`public/screenshots/*.webp` are real screenshots of the [live demo](https://interaction-dynamics.io/features/), at 1x (1280px) and 2x (2560px), shown with the `Screenshot` component. To refresh them after a dashboard change, install the dashboard dependencies (the script reuses its Playwright) and run:

```bash
bun install --cwd ../web
node scripts/capture-screenshots.mjs # optional: a demo URL, e.g. http://localhost:5173/
```

The live demo is only redeployed from `master`. To capture a dashboard change before it ships (a new logo, a renamed tab), serve the demo locally from this branch:

```bash
cd ../cli && cargo build --release
./target/release/features ../../examples/demo/src --project-dir ../../examples/demo --json > ../web/public/features.json
echo '{"version": "0.14.0"}' > ../web/public/metadata.json
bun run --cwd ../web dev -- --port 5173
```

It needs ImageMagick (`magick`) for the WebP conversion. Views that show commit authors (Changes tab, feature Insights tab) are left out on purpose.

## Favicons

`public/favicon.svg` is the logo. `public/favicon.ico` (16, 32, 48 px, for browsers without SVG favicons such as Safari) and `public/apple-touch-icon.png` (180 px) are rendered from it. After changing the logo, re-render them (needs the dashboard Playwright and ImageMagick) and bump the `?v=` query of the SVG link in `src/layouts/Base.astro` so browsers drop the cached icon:

```bash
node scripts/render-icons.mjs
```

## Open Graph image

`public/og.png` (1200×630) is rendered from [`scripts/og.svg`](./scripts/og.svg). After editing the SVG, re-render it with any headless browser at a 1200×630 viewport, for example with Playwright:

```js
await page.setViewportSize({ width: 1200, height: 630 })
await page.setContent(svg)
await page.screenshot({ path: 'public/og.png' })
```
