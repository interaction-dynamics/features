---
status: implemented
date: 2026-11-14
---

# Website ready for generative engines (GEO)

## Context

The website was built for classic search engines ([001](001-website-seo-pages.md)): static HTML, meta tags, JSON-LD, sitemap and robots.txt. People now also find tools through AI assistants and answer engines (ChatGPT, Claude, Perplexity, Gemini, Google AI Overviews). They crawl, read and cite pages differently: they prefer clean text over styled HTML, look for `llms.txt`, and need to be explicitly allowed in `robots.txt`. Generative Engine Optimization (GEO) makes the site easy for them to read, understand and cite.

## Requirements

- `robots.txt` explicitly allows the main AI crawlers (OpenAI, Anthropic, Perplexity, Google-Extended, Applebot-Extended, Meta, Amazon, DuckDuckGo, Mistral, Common Crawl) and points to `llms.txt`.
- `/llms.txt` follows [llmstxt.org](https://llmstxt.org): site name, one-line summary, key facts (CLI name, install commands, license), links to every page with its description, project links, and `llms-full.txt` as optional.
- Every indexable page has a Markdown twin: `/cli` → `/cli.md`, `/` → `/index.md`. `/llms-full.txt` concatenates all of them.
- The Markdown is generated from the built HTML (`<main>`), so it never drifts from the pages. No hand-written copy to maintain.
- The Markdown is clean: no icons, SVG illustrations or `aria-hidden` decoration; HTML illustrations (`role="img"`) are replaced by their `aria-label`; root-relative links and images become absolute; shell prompts (`$ `) are dropped from inline commands; terminal blocks become code blocks.
- Visual or interactive components provide their own plain-text version through `data-markdown-code` (and optional `data-markdown-lang`): the file trees and the install picker on `/cli`, which lists every install method instead of only the selected one.
- Every page links its Markdown twin and `llms.txt` with `<link rel="alternate">`. The footer links `llms.txt`.
- Markdown files are served as `text/markdown; charset=utf-8`, `llms*.txt` as `text/plain; charset=utf-8`.
- Structured data is richer for entity understanding: `Organization` with `@id` and `sameAs` (GitHub), `WebSite` with `inLanguage`, `SoftwareApplication` with `sameAs` (GitHub, npm, crates.io), `isAccessibleForFree`, `keywords`, `featureList`, `screenshot` and `installUrl`.
- Pages with `noindex` (404) get no Markdown twin and are not listed in `llms.txt`.

## Out of scope

- Rewriting page copy for answer engines (it already answers questions directly, with an FAQ page).
- Blocking or rate-limiting any crawler.
- `dateModified` / freshness signals (would need the git date of each page).

## Plan

1. Astro integration `integrations/llms.ts`: on `astro:build:done`, convert the `<main>` of each built page to Markdown with `turndown` and custom rules, write `<page>.md`, `llms.txt` and `llms-full.txt`.
2. `data-markdown-code` on `FileTree` and the `/cli` install picker.
3. AI crawler groups and `llms.txt` pointer in `robots.txt`.
4. `<link rel="alternate">` in `Base.astro`, footer link, richer JSON-LD.
5. Content-Type headers in `vercel.json`.
6. Document it in the website README.
