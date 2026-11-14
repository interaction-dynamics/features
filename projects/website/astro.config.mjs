import sitemap from '@astrojs/sitemap'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig, envField } from 'astro/config'

// Canonical URLs, the sitemap and Open Graph tags need an absolute site URL.
// Set SITE_URL for a custom domain; on Vercel the production domain is used by default.
const site =
  process.env.SITE_URL ??
  (process.env.VERCEL_PROJECT_PRODUCTION_URL
    ? `https://${process.env.VERCEL_PROJECT_PRODUCTION_URL}`
    : 'http://localhost:4321')

// PostHog EU ingestion. Production proxies /ingest with the rewrites in vercel.json;
// the dev server mirrors them so analytics can be tested locally.
const posthogProxy = {
  '/ingest/static': {
    target: 'https://eu-assets.i.posthog.com',
    changeOrigin: true,
    rewrite: (path) => path.replace(/^\/ingest/, ''),
  },
  '/ingest': {
    target: 'https://eu.i.posthog.com',
    changeOrigin: true,
    rewrite: (path) => path.replace(/^\/ingest/, ''),
  },
}

export default defineConfig({
  site,
  trailingSlash: 'never',
  // Keep whitespace so terminal output passed through slots into <pre> stays intact.
  compressHTML: false,
  build: {
    format: 'file',
  },
  env: {
    schema: {
      // PostHog project API key (phc_...). Public by design; analytics are off when unset.
      PUBLIC_POSTHOG_KEY: envField.string({
        context: 'client',
        access: 'public',
        optional: true,
      }),
    },
  },
  integrations: [sitemap()],
  vite: {
    plugins: [tailwindcss()],
    server: {
      proxy: posthogProxy,
    },
  },
})
