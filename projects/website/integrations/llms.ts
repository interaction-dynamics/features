import { readFile, readdir, writeFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import type { AstroIntegration } from 'astro'
import TurndownService from 'turndown'
import { nav, site } from '../src/data/site'

/**
 * Makes the site readable by AI assistants and answer engines (GEO).
 *
 * After the build, every indexable page gets a Markdown twin (`/cli` → `/cli.md`,
 * `/` → `/index.md`) generated from its `<main>` content, plus:
 * - `/llms.txt`: an index of the site following https://llmstxt.org
 * - `/llms-full.txt`: every page in one Markdown file
 *
 * The Markdown is generated from the HTML, so it never drifts from the pages.
 */
export default function llms(): AstroIntegration {
  let siteUrl = 'http://localhost:4321'

  return {
    name: 'llms',
    hooks: {
      'astro:config:done': ({ config }) => {
        if (config.site) siteUrl = config.site.replace(/\/$/, '')
      },
      'astro:build:done': async ({ dir, logger }) => {
        const outDir = fileURLToPath(dir)
        const files = (await readdir(outDir)).filter((file) =>
          file.endsWith('.html'),
        )

        const pages: Page[] = []
        for (const file of files) {
          const html = await readFile(`${outDir}/${file}`, 'utf8')
          if (/<meta name="robots" content="noindex"/.test(html)) continue

          const slug = file.replace(/\.html$/, '')
          const page = toPage(html, slug, siteUrl)
          await writeFile(`${outDir}/${slug}.md`, page.markdown)
          pages.push(page)
        }

        pages.sort((a, b) => order(a.slug) - order(b.slug))
        await writeFile(`${outDir}/llms.txt`, llmsTxt(pages, siteUrl))
        await writeFile(`${outDir}/llms-full.txt`, llmsFullTxt(pages))
        logger.info(
          `Generated llms.txt, llms-full.txt and ${pages.length} Markdown pages`,
        )
      },
    },
  }
}

interface Page {
  slug: string
  title: string
  description: string
  url: string
  markdownUrl: string
  markdown: string
}

/** Home page first, then the navigation order. */
const order = (slug: string) => {
  if (slug === 'index') return -1
  const index = nav.findIndex((item) => item.href === `/${slug}`)
  return index === -1 ? nav.length : index
}

const decode = (text: string) =>
  text
    .replace(/&quot;/g, '"')
    .replace(/&#39;|&#x27;/g, "'")
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&amp;/g, '&')

const match = (html: string, pattern: RegExp) =>
  decode(html.match(pattern)?.[1]?.trim() ?? '')

const toPage = (html: string, slug: string, siteUrl: string): Page => {
  const url = slug === 'index' ? `${siteUrl}/` : `${siteUrl}/${slug}`
  const title = match(html, /<title>([\s\S]*?)<\/title>/).replace(
    ` | ${site.name}`,
    '',
  )
  const description = match(html, /<meta name="description" content="([^"]*)"/)
  const main = html.match(/<main[^>]*>([\s\S]*)<\/main>/)?.[1] ?? ''
  // Root-relative links and images become absolute, so the Markdown works anywhere.
  const absolute = main.replace(/(href|src)="\/(?!\/)/g, `$1="${siteUrl}/`)

  const body = turndown
    .turndown(absolute)
    .replace(/\n{3,}/g, '\n\n')
    .trim()
  const markdown = `# ${title}\n\n> ${description}\n\nSource: ${url}\n\n${body}\n`

  return {
    slug,
    title,
    description,
    url,
    markdownUrl: `${siteUrl}/${slug}.md`,
    markdown,
  }
}

const turndown = new TurndownService({
  headingStyle: 'atx',
  codeBlockStyle: 'fenced',
  bulletListMarker: '-',
  emDelimiter: '_',
})
// The Markdown is read more than rendered: keep `snake_case` and `__docs__` as written.
turndown.escape = (text) => text

// Decorative markup: icons, illustrations and anything hidden from screen readers.
const ignoredTags = new Set(['SVG', 'SCRIPT', 'STYLE', 'NOSCRIPT', 'TEMPLATE'])
turndown.addRule('decorative', {
  filter: (node) =>
    ignoredTags.has(node.nodeName.toUpperCase()) ||
    node.getAttribute('aria-hidden') === 'true',
  replacement: () => '',
})

// Shell prompts (`$ `) in inline commands are decoration, not part of the command.
turndown.addRule('prompt', {
  filter: (node) =>
    node.nodeName === 'SPAN' &&
    node.classList.contains('select-none') &&
    node.closest('pre') === null,
  replacement: () => '',
})

// Inline code: drop the indentation left by multi-line markup.
turndown.addRule('inline-code', {
  filter: (node) =>
    node.nodeName === 'CODE' && node.parentNode?.nodeName !== 'PRE',
  replacement: (content) => {
    const code = content.replace(/\s+/g, ' ').trim()
    if (!code) return ''
    const fence = code.includes('`') ? '``' : '`'
    return `${fence}${code}${fence}`
  },
})

// Illustrations built from HTML are described by their accessible label.
turndown.addRule('role-img', {
  filter: (node) => node.getAttribute('role') === 'img',
  replacement: (_content, node) => {
    const label = (node as HTMLElement).getAttribute('aria-label')
    return label ? `\n\n_${label}_\n\n` : ''
  },
})

// Visual or interactive components (file trees, the install picker) provide a
// plain-text version in `data-markdown-code`, rendered as a code block.
turndown.addRule('markdown-code', {
  filter: (node) => node.hasAttribute('data-markdown-code'),
  replacement: (_content, node) => {
    const element = node as HTMLElement
    const lang = element.getAttribute('data-markdown-lang') ?? 'text'
    const code = element.getAttribute('data-markdown-code')
    return `\n\n\`\`\`${lang}\n${code}\n\`\`\`\n\n`
  },
})

// Terminal blocks are `<pre>` without `<code>`: keep them as code blocks.
turndown.addRule('pre', {
  filter: (node) =>
    node.nodeName === 'PRE' && node.firstElementChild?.nodeName !== 'CODE',
  replacement: (_content, node) =>
    `\n\n\`\`\`\n${(node.textContent ?? '').replace(/\n+$/, '')}\n\`\`\`\n\n`,
})

const llmsTxt = (pages: Page[], siteUrl: string) => {
  const docs = pages
    .map(
      (page) => `- [${page.title}](${page.markdownUrl}): ${page.description}`,
    )
    .join('\n')

  return `# ${site.name}

> ${site.description}

- Name of the CLI: \`features-cli\`, a single binary written in Rust.
- Try it without installing: \`npx features-cli@latest serve ./src\` opens the dashboard.
- Install: \`npm install -g features-cli\`, \`curl -fsSL ${siteUrl}/install.sh | sh\`, \`brew install interaction-dynamics/tap/features\` or \`cargo install features-cli\`.
- Free and open source under the ${site.license} license, maintained by ${site.organization} (${site.organizationUrl}).
- Every page below is also available as HTML at the same URL without \`.md\`.

## Docs

${docs}

## Project

- [GitHub repository](${site.repository}): source code, issues and README
- [Live demo](${site.demo}): the dashboard running on this repository
- [Changelog feed](${siteUrl}/changelog.xml): RSS feed of every release
- [npm package](${site.npm}): prebuilt binaries for macOS, Linux and Windows
- [crates.io](${site.crate}): Rust crate, to build from source

## Optional

- [Full content](${siteUrl}/llms-full.txt): every page above in one Markdown file
`
}

const llmsFullTxt = (pages: Page[]) =>
  `${pages.map((page) => page.markdown.trim()).join('\n\n---\n\n')}\n`
