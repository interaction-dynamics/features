// Captures screenshots of the live dashboard demo into public/screenshots.
//
// Uses Playwright from the dashboard project (projects/web), so run `pnpm install`
// there first. Then, from projects/website:
//
//   node scripts/capture-screenshots.mjs [demo-url]
//
// Each view is saved as WebP at 1x (1280px wide) and 2x (2560px wide).
// Views showing commit authors (Changes tab, feature Insights tab) are left out on purpose.

import { execFileSync } from 'node:child_process'
import { mkdirSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const require = createRequire(join(root, '../web/package.json'))
const { chromium } = require('@playwright/test')

const demo = process.argv[2] ?? 'https://interaction-dynamics.io/features/'
const outDir = join(root, 'public/screenshots')
const tmpDir = join(root, 'node_modules/.cache/screenshots')
mkdirSync(outDir, { recursive: true })
mkdirSync(tmpDir, { recursive: true })

const width = 1280
const views = [
  { name: 'insights', hash: '#/insights', height: 800 },
  { name: 'ownership', hash: '#/insights', tab: 'Ownership', height: 560 },
  {
    name: 'dependencies',
    hash: '#/insights',
    tab: 'Dependencies',
    height: 800,
  },
  {
    name: 'feature',
    hash: '#/?feature=libs/features/feature-0',
    tab: 'Description',
    height: 560,
  },
  {
    name: 'decisions',
    hash: '#/?feature=libs/features/feature-0',
    tab: 'Decisions',
    height: 480,
  },
  {
    name: 'flags',
    hash: '#/?feature=libs/features/feature-0',
    tab: 'Flag',
    height: 480,
  },
  {
    name: 'nested-features',
    hash: '#/?feature=libs/features/feature-1',
    tab: 'Description',
    height: 560,
  },
]

const browser = await chromium.launch()
const page = await browser.newPage({
  viewport: { width, height: 800 },
  deviceScaleFactor: 2,
  colorScheme: 'dark',
})

for (const view of views) {
  await page.setViewportSize({ width, height: view.height })
  await page.goto(new URL(view.hash, demo).href, { waitUntil: 'networkidle' })
  if (view.tab) {
    await page.getByRole('tab', { name: view.tab, exact: true }).click()
  }
  await page.waitForTimeout(1000)
  const png = join(tmpDir, `${view.name}.png`)
  await page.screenshot({ path: png })
  for (const [suffix, size] of [
    ['', width],
    ['@2x', width * 2],
  ]) {
    const out = join(outDir, `${view.name}${suffix}.webp`)
    execFileSync('magick', [png, '-resize', `${size}x`, '-quality', '82', out])
    console.log(out)
  }
}

await browser.close()
rmSync(tmpDir, { recursive: true, force: true })
