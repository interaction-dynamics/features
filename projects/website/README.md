# Website

Marketing and documentation website for `features-cli`. It explains the concept of feature-based architecture and the tool with visual pages, built for search engines: static HTML, per-page meta tags, Open Graph image, JSON-LD structured data, `sitemap-index.xml` and `robots.txt`. It is also built for AI assistants and answer engines (GEO), see [AI assistants](#ai-assistants-geo).

Built with [Astro](https://astro.build) and Tailwind CSS. Hosted on [Vercel](https://vercel.com).

## Pages

| Path                          | Content                                                                   | Source                                                            |
| ----------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `/`                           | What Features does, how it works, dashboard, pull request integration     | `README.md`                                                       |
| `/feature-based-architecture` | The concept, layered vs feature-based, anatomy of a feature               | `FAQ.md`, `README.md`                                             |
| `/guidelines`                 | Folder structure, README front matter, specs and decisions, annotations   | `docs/folder-architecture-guidelines.md`, `docs/readme-format.md` |
| `/cli`                        | Install, commands, options, `diff` + GitHub Actions, CODEOWNERS, coverage | `README.md`, `FAQ.md`                                             |
| `/faq`                        | Frequently asked questions (with `FAQPage` structured data)               | `FAQ.md`                                                          |

The content mirrors the repository docs. When the README, FAQ or docs change, update the matching page.

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

## Analytics

The site uses [PostHog](https://posthog.com) (EU cloud) for page views and autocaptured clicks. It is off unless the `PUBLIC_POSTHOG_KEY` environment variable is set to the project API key (`phc_...`), so local builds and forks send nothing.

- Events go through `/ingest`, proxied to PostHog by the rewrites in [`vercel.json`](./vercel.json) (and by the Vite proxy in `astro.config.mjs` during `bun run dev`), so ad blockers don't drop them.
- It runs in cookieless mode: no cookies or local storage, so no consent banner. **Cookieless server hash mode** must be enabled in the PostHog project settings, otherwise PostHog ignores the events.

To test locally, put the key in `.env` (`PUBLIC_POSTHOG_KEY=phc_...`) and run `bun run dev`.

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
