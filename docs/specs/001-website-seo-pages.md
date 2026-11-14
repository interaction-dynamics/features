---
status: implemented
date: 2026-11-14
---

# Website with SEO-ready pages explaining the concept

## Context

The concept of feature-based architecture and the `features-cli` tool are only explained in the README, the FAQ and the `docs/` folder. These are hard to discover from search engines and are text-heavy. A small public website should explain the same content in a more visual way and rank for searches such as "feature-based architecture".

## Requirements

- New project in `projects/website`, independent from `projects/web` (the dashboard).
- Hosted on Vercel, with `projects/website` as the project root directory.
- Pages, with content taken from the README, FAQ and docs:
  - `/` home: what the tool does, how it works, capabilities, dashboard screenshot, pull request integration, supported languages.
  - `/feature-based-architecture`: the concept, layered vs feature-based, benefits, anatomy of a feature.
  - `/guidelines`: features folder and feature flag methods, MUST/SHOULD/MAY rules, README front matter, decision records, metadata annotations.
  - `/cli`: install, commands, options, `diff` with GitHub Actions, CODEOWNERS, coverage.
  - `/faq`: the FAQ.
  - a `404` page that is not indexed.
- Visual: diagrams, file trees, terminal and dashboard illustrations instead of plain text.
- Real screenshots of the dashboard demo, hosted with the site (no hotlinking):
  - home: Insights table, dependency graph, ownership table;
  - concept: a feature page with nested features;
  - guidelines: a feature page showing README metadata, the Specs & Decisions tab, the Flag tab.
  - Screenshots never show commit author emails or names (no Changes tab, no feature Insights tab).
  - Served as WebP at 1x and 2x with `srcset`, explicit size and lazy loading; each links to its full-size version.
  - Regenerated with `scripts/capture-screenshots.mjs`.
- SEO:
  - static HTML, one `h1` per page, unique title and description per page;
  - canonical URLs without trailing slash and without `.html`;
  - Open Graph and Twitter card tags with a 1200×630 image;
  - JSON-LD: `WebSite` on every page, `BreadcrumbList` on sub-pages, `SoftwareApplication` on home, `TechArticle` on the concept page, `HowTo` on guidelines, `FAQPage` on the FAQ;
  - `sitemap-index.xml` and `robots.txt` pointing to it.
- The absolute site URL comes from `SITE_URL`, else from Vercel's `VERCEL_PROJECT_PRODUCTION_URL`.
- Responsive down to 375px with no horizontal page scroll.
- Validated in CI (build, format, lint) like `projects/web`.

## Out of scope

- Buying or configuring a custom domain and creating the Vercel project.
- Automatic sync of the content with the README (pages are hand-written; the website README says which doc each page mirrors).
- Analytics, blog, i18n, light theme.

## Plan

1. Scaffold `projects/website` with Astro (static output), Tailwind CSS v4, `@astrojs/sitemap`, Lucide icons and Biome, using pnpm like `projects/web`.
2. Shared layout with SEO head (meta, canonical, Open Graph, JSON-LD), header, footer and visual components (`Terminal`, `FileTree`, `Section`, `PageHero`, `CallToAction`).
3. Write the five pages and the 404 page from the README, FAQ and docs.
4. Add `robots.txt` endpoint, sitemap, Open Graph image rendered from `scripts/og.svg`.
5. Add `vercel.json` (clean URLs, no trailing slash, asset caching, skip builds when the folder did not change).
6. Add a `validate-website` job to `.github/workflows/validate.yml`.
7. Capture dashboard screenshots from the live demo with `scripts/capture-screenshots.mjs` and place them with a `Screenshot` component.
