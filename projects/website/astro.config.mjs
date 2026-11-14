import sitemap from '@astrojs/sitemap'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'astro/config'

// Canonical URLs, the sitemap and Open Graph tags need an absolute site URL.
// Set SITE_URL for a custom domain; on Vercel the production domain is used by default.
const site =
  process.env.SITE_URL ??
  (process.env.VERCEL_PROJECT_PRODUCTION_URL
    ? `https://${process.env.VERCEL_PROJECT_PRODUCTION_URL}`
    : 'http://localhost:4321')

export default defineConfig({
  site,
  trailingSlash: 'never',
  // Keep whitespace so terminal output passed through slots into <pre> stays intact.
  compressHTML: false,
  build: {
    format: 'file',
  },
  integrations: [sitemap()],
  vite: {
    plugins: [tailwindcss()],
  },
})
