# Website

Marketing and documentation website for `features-cli`. It explains the concept of feature-based architecture and the tool with visual pages, built for search engines: static HTML, per-page meta tags, Open Graph image, JSON-LD structured data, `sitemap-index.xml` and `robots.txt`.

Built with [Astro](https://astro.build) and Tailwind CSS. Hosted on [Vercel](https://vercel.com).

## Pages

| Path                          | Content                                                                   | Source                                                            |
| ----------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `/`                           | What Features does, how it works, dashboard, pull request integration     | `README.md`                                                       |
| `/feature-based-architecture` | The concept, layered vs feature-based, anatomy of a feature               | `FAQ.md`, `README.md`                                             |
| `/guidelines`                 | Folder structure, README front matter, decisions, annotations             | `docs/folder-architecture-guidelines.md`, `docs/readme-format.md` |
| `/cli`                        | Install, commands, options, `diff` + GitHub Actions, CODEOWNERS, coverage | `README.md`, `FAQ.md`                                             |
| `/faq`                        | Frequently asked questions (with `FAQPage` structured data)               | `FAQ.md`                                                          |

The content mirrors the repository docs. When the README, FAQ or docs change, update the matching page.

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
node scripts/capture-screenshots.mjs # optional: a demo URL, e.g. http://localhost:3000/
```

It needs ImageMagick (`magick`) for the WebP conversion. Views that show commit authors (Changes tab, feature Insights tab) are left out on purpose.

## Open Graph image

`public/og.png` (1200×630) is rendered from [`scripts/og.svg`](./scripts/og.svg). After editing the SVG, re-render it with any headless browser at a 1200×630 viewport, for example with Playwright:

```js
await page.setViewportSize({ width: 1200, height: 630 })
await page.setContent(svg)
await page.screenshot({ path: 'public/og.png' })
```
