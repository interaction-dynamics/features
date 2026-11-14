// Renders the raster favicons from public/favicon.svg (the logo):
// public/favicon.ico (16, 32, 48) for browsers without SVG favicon support (Safari)
// and public/apple-touch-icon.png (180) for iOS home screens.
//
// Uses Playwright from the dashboard project (projects/web) and ImageMagick (`magick`).
// From projects/website:
//
//   node scripts/render-icons.mjs

import { execFileSync } from 'node:child_process'
import { mkdirSync, readFileSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const require = createRequire(join(root, '../web/package.json'))
const { chromium } = require('@playwright/test')

const svg = readFileSync(join(root, 'public/favicon.svg'), 'utf8')
const tmpDir = join(root, 'node_modules/.cache/icons')
mkdirSync(tmpDir, { recursive: true })

const browser = await chromium.launch()
const page = await browser.newPage()

async function render(size, path, background = 'transparent') {
  await page.setViewportSize({ width: size, height: size })
  await page.setContent(
    `<style>html,body{margin:0;background:${background}}svg{display:block;width:${size}px;height:${size}px}</style>${svg}`,
  )
  await page.screenshot({ path, omitBackground: background === 'transparent' })
}

const icoSizes = [16, 32, 48]
const pngs = []
for (const size of icoSizes) {
  const png = join(tmpDir, `favicon-${size}.png`)
  await render(size, png)
  pngs.push(png)
}
execFileSync('magick', [...pngs, join(root, 'public/favicon.ico')])

// iOS adds its own rounded corners: fill the corners with the tile color.
await render(180, join(root, 'public/apple-touch-icon.png'), '#1A1A1D')

await browser.close()
rmSync(tmpDir, { recursive: true, force: true })
console.log('public/favicon.ico, public/apple-touch-icon.png')
