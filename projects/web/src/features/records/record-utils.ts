import type {
  DocRecord,
  Feature,
  RecordLinks,
  RecordSource,
} from '@/models/feature'

export type KindFilter = 'all' | 'spec' | 'decision'

/** A record with the feature it belongs to (`null` for project-level) */
export type LocatedRecord = {
  record: DocRecord
  featurePath: string | null
}

const INACTIVE_STATUSES = ['superseded', 'rejected', 'deprecated']

const EMPTY_LINKS: RecordLinks = {
  decisions: [],
  specs: [],
  supersedes: [],
  superseded_by: [],
}

/** Title of a raw decision: its first line without Markdown markup */
export const titleFromMarkdown = (markdown: string): string => {
  const firstLine = markdown.split('\n')[0]?.trim() ?? ''
  const title = firstLine
    .replace(/^#+\s*/, '')
    .replace(/\*\*(.*?)\*\*/g, '$1')
    .replace(/\*(.*?)\*/g, '$1')
    .replace(/`(.*?)`/g, '$1')
    .replace(/\[(.*?)\]\(.*?\)/g, '$1')
    .replace(/~~(.*?)~~/g, '$1')
    .trim()
  return title || 'Untitled Decision'
}

/**
 * The records of a feature. Reports of older versions only have the raw
 * `decisions`: show them as decisions without metadata.
 */
export const recordsOf = (feature: Feature): DocRecord[] => {
  if (feature.records) return feature.records
  return feature.decisions.map((content, index) => ({
    kind: 'decision',
    id: `decision-${index + 1}`,
    title: titleFromMarkdown(content),
    path: `${feature.path}#decision-${index + 1}`,
    content,
    source: 'native',
    links: EMPTY_LINKS,
  }))
}

/** Superseded, rejected and deprecated records are hidden on demand */
export const isInactive = (record: DocRecord): boolean =>
  INACTIVE_STATUSES.includes(record.status ?? '')

/** Newest first, undated records last, then by id */
export const sortRecords = (records: DocRecord[]): DocRecord[] =>
  [...records].sort((a, b) => {
    if (a.date && b.date && a.date !== b.date) {
      return a.date < b.date ? 1 : -1
    }
    if (a.date && !b.date) return -1
    if (!a.date && b.date) return 1
    return a.id.localeCompare(b.id)
  })

export const filterRecords = (
  records: DocRecord[],
  kind: KindFilter,
  hideInactive: boolean,
): DocRecord[] =>
  records.filter(
    (record) =>
      (kind === 'all' || record.kind === kind) &&
      !(hideInactive && isInactive(record)),
  )

export const countByKind = (records: DocRecord[]) => ({
  all: records.length,
  spec: records.filter((r) => r.kind === 'spec').length,
  decision: records.filter((r) => r.kind === 'decision').length,
})

/** Every record of the tree and the project, parents before nested features */
export const indexRecords = (
  features: Feature[],
  projectRecords: DocRecord[],
): LocatedRecord[] => {
  const out: LocatedRecord[] = []
  const walk = (list: Feature[]) => {
    for (const feature of list) {
      for (const record of feature.records ?? []) {
        out.push({ record, featurePath: feature.path })
      }
      walk(feature.features ?? [])
    }
  }
  walk(features)
  for (const record of projectRecords) {
    out.push({ record, featurePath: null })
  }
  return out
}

/**
 * The record a link points to, with the CLI's rule: same feature first, then
 * project-level, then any record with that id.
 */
export const resolveLink = (
  id: string,
  from: string | null,
  index: LocatedRecord[],
): LocatedRecord | undefined => {
  const candidates = index.filter((entry) => entry.record.id === id)
  return (
    candidates.find((c) => from !== null && c.featurePath === from) ??
    candidates.find((c) => c.featurePath === null) ??
    candidates[0]
  )
}

/** URL search of the page showing a record */
export const recordLocation = (target: LocatedRecord) => {
  const record = `record=${encodeURIComponent(target.record.path)}`
  return target.featurePath === null
    ? { pathname: '/records', search: `?${record}` }
    : { pathname: '/', search: `?feature=${target.featurePath}&${record}` }
}

export const SOURCE_LABELS: Record<RecordSource, string | null> = {
  native: null,
  speckit: 'Spec Kit',
  kiro: 'Kiro',
}

export type StatusTone = 'positive' | 'progress' | 'inactive' | 'neutral'

export const statusTone = (status?: string): StatusTone => {
  switch (status) {
    case 'accepted':
    case 'approved':
    case 'implemented':
    case 'done':
      return 'positive'
    case 'draft':
    case 'proposed':
    case 'in-progress':
    case 'review':
      return 'progress'
    case 'superseded':
    case 'rejected':
    case 'deprecated':
      return 'inactive'
    default:
      return 'neutral'
  }
}

/** Markdown for a sub-file: code files are shown as a fenced block */
export const fileMarkdown = (name: string, content: string): string => {
  const extension = name.split('.').pop()?.toLowerCase() ?? ''
  if (extension === 'md' || extension === 'mdx') return content
  return `\`\`\`${extension}\n${content}\n\`\`\``
}

/** The card already shows the title: drop the leading `# Title` line */
export const withoutTitle = (markdown: string): string =>
  markdown.replace(/^\s*# [^\n]*\n?/, '')
