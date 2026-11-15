import type { APIRoute } from 'astro'
import { site } from '../data/site'
import { groupChanges, releases, toPlainText } from '../lib/changelog'

const escapeXml = (text: string) =>
  text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;')

const rfc822 = (date: string) => new Date(`${date}T00:00:00Z`).toUTCString()

/** RSS 2.0 feed of the changelog, one item per release. */
export const GET: APIRoute = ({ site: siteUrl }) => {
  const url = (path: string) => new URL(path, siteUrl).href
  const page = url('/changelog')

  const items = releases.map((release) => {
    const changes = groupChanges(release.changes)
      .map(
        (group) =>
          `${group.label}:\n${group.changes.map((change) => `- ${toPlainText(change.title)}`).join('\n')}`,
      )
      .join('\n\n')
    const description = [release.summary, changes].filter(Boolean).join('\n\n')
    const link = `${page}#${release.anchor}`

    return `    <item>
      <title>${escapeXml(`${site.name} ${release.version}`)}</title>
      <link>${link}</link>
      <guid isPermaLink="true">${link}</guid>
      <pubDate>${rfc822(release.date)}</pubDate>
      <description>${escapeXml(description)}</description>
    </item>`
  })

  const body = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>${escapeXml(`${site.name} changelog`)}</title>
    <link>${page}</link>
    <description>${escapeXml(`New features, fixes and docs in every release of ${site.name}.`)}</description>
    <language>en</language>
    <lastBuildDate>${rfc822(releases[0].date)}</lastBuildDate>
    <atom:link href="${url('/changelog.xml')}" rel="self" type="application/rss+xml" />
${items.join('\n')}
  </channel>
</rss>
`

  return new Response(body, {
    headers: { 'Content-Type': 'application/rss+xml; charset=utf-8' },
  })
}
