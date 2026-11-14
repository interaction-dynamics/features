---
status: implemented
date: 2026-11-14
---

# PostHog analytics on the website

## Context

The website (`projects/website`) has no analytics, so there is no way to know which pages people read or what they click. PostHog (EU cloud) is the chosen tool.

## Requirements

- PostHog captures page views and autocaptured clicks on every page (added once in the `Base` layout).
- The project API key comes from the `PUBLIC_POSTHOG_KEY` environment variable (typed with `astro:env`). When it is unset, nothing is loaded or sent, so local builds and forks stay silent.
- Events go through a same-origin `/ingest` path proxied to `eu.i.posthog.com` / `eu-assets.i.posthog.com`, so ad blockers don't drop them: Vercel rewrites in production, a Vite proxy in `bun run dev`.
- Cookieless mode (`cookieless_mode: 'always'`): no cookies, local or session storage, and no consent banner. Cookieless server hash mode must be enabled in the PostHog project settings.
- `.env` files are git-ignored in the website project.

## Out of scope

- Custom events, identification, feature flags, surveys.
- A cookie consent banner.
- Analytics in the web dashboard (`projects/web`).

## Plan

1. Add `posthog-js` and an `Analytics.astro` component that initializes it, included in `Base.astro`.
2. Declare `PUBLIC_POSTHOG_KEY` in the `astro.config.mjs` env schema; add the dev proxy there.
3. Add the `/ingest` rewrites to `vercel.json`.
4. Document the setup in the website README.
