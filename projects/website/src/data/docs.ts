/**
 * Sidebar of the documentation, in reading order.
 *
 * Each `slug` is a Markdown file in `src/content/docs/` (`index` is `/docs`).
 * To add a page, create the file and list its slug here: the build fails when
 * a page is missing from one or the other.
 */
export const docsGroups = [
  {
    title: 'Getting started',
    pages: [{ slug: 'index', label: 'Introduction' }],
  },
  {
    title: 'CLI reference',
    pages: [
      { slug: 'commands', label: 'Commands' },
      { slug: 'options', label: 'Options' },
      { slug: 'configuration', label: 'Configuration file' },
      { slug: 'check-rules', label: 'Check rules' },
    ],
  },
  {
    title: 'Project structure',
    pages: [
      { slug: 'folder-structure', label: 'Folder structure' },
      { slug: 'readme-format', label: 'README format' },
      { slug: 'specs-decisions', label: 'Specs and decisions' },
    ],
  },
  {
    title: 'Guides',
    pages: [
      { slug: 'dashboard', label: 'Publish the dashboard' },
      { slug: 'pull-requests-ci', label: 'Pull requests and CI' },
      { slug: 'mcp', label: 'MCP server' },
      { slug: 'external-data', label: 'Import external data' },
      { slug: 'aggregate', label: 'Aggregate repositories' },
    ],
  },
] as const

export interface DocsPage {
  slug: string
  label: string
  group: string
  href: string
}

export const docsHref = (slug: string) =>
  slug === 'index' ? '/docs' : `/docs/${slug}`

/** Every page of the sidebar, flat and in order, for prev/next links. */
export const docsPages: DocsPage[] = docsGroups.flatMap((group) =>
  group.pages.map((page) => ({
    ...page,
    group: group.title,
    href: docsHref(page.slug),
  })),
)
