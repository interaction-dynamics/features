import type { FeatureRecord, RecordKind, RecordLinks } from '@/models/feature'

export type KindFilter = 'all' | RecordKind

/** Statuses meaning a record no longer applies */
export const INACTIVE_STATUSES = ['superseded', 'rejected', 'deprecated']

export const KIND_LABELS: Record<RecordKind, string> = {
  spec: 'Spec',
  decision: 'ADR',
}

export const isInactive = (record: FeatureRecord): boolean =>
  INACTIVE_STATUSES.includes(record.status ?? '')

/** Newest first; records without a date go last, then by id */
export const sortRecords = (records: FeatureRecord[]): FeatureRecord[] =>
  [...records].sort((a, b) => {
    if (a.date !== b.date) {
      if (!a.date) return 1
      if (!b.date) return -1
      return b.date.localeCompare(a.date)
    }
    return b.id.localeCompare(a.id)
  })

export const filterRecords = (
  records: FeatureRecord[],
  kind: KindFilter,
  hideInactive: boolean,
): FeatureRecord[] =>
  records.filter(
    (record) =>
      (kind === 'all' || record.kind === kind) &&
      (!hideInactive || !isInactive(record)),
  )

/** Stable DOM id of a record card, used to scroll to linked records */
export const recordAnchorId = (record: Pick<FeatureRecord, 'kind' | 'id'>) =>
  `record-${record.kind}-${record.id}`

export type RecordLink = {
  label: string
  targetId: string
  targetKind: RecordKind
}

/** Every link of a record, with the kind of record it points to */
export const recordLinks = (record: FeatureRecord): RecordLink[] => {
  const links: RecordLinks = record.links ?? {
    decisions: [],
    specs: [],
    supersedes: [],
    superseded_by: [],
  }
  return [
    ...links.specs.map((id) => ({
      label: 'Spec',
      targetId: id,
      targetKind: 'spec' as const,
    })),
    ...links.decisions.map((id) => ({
      label: 'ADR',
      targetId: id,
      targetKind: 'decision' as const,
    })),
    ...links.supersedes.map((id) => ({
      label: 'Supersedes',
      targetId: id,
      targetKind: record.kind,
    })),
    ...links.superseded_by.map((id) => ({
      label: 'Superseded by',
      targetId: id,
      targetKind: record.kind,
    })),
  ]
}

/** The record a link points to, preferring the expected kind */
export const findLinkTarget = (
  records: FeatureRecord[],
  link: Pick<RecordLink, 'targetId' | 'targetKind'>,
): FeatureRecord | undefined =>
  records.find((r) => r.id === link.targetId && r.kind === link.targetKind) ??
  records.find((r) => r.id === link.targetId)

const stripMarkdown = (line: string) =>
  line
    .replace(/^#+\s*/, '')
    .replace(/\*\*(.*?)\*\*/g, '$1')
    .replace(/\*(.*?)\*/g, '$1')
    .replace(/`(.*?)`/g, '$1')
    .replace(/\[(.*?)\]\(.*?\)/g, '$1')
    .replace(/~~(.*?)~~/g, '$1')
    .trim()

/**
 * Records of a feature: its `records`, or for reports produced by older
 * versions of the CLI, records built from the raw `decisions` markdown
 */
export const featureRecords = (feature: {
  records?: FeatureRecord[]
  decisions?: string[]
}): FeatureRecord[] => {
  if (feature.records) return feature.records
  return (feature.decisions ?? []).map((content, index) => {
    const firstLine = content
      .split('\n')
      .map((line) => line.trim())
      .find((line) => line.length > 0 && line !== '---')
    return {
      kind: 'decision',
      id: String(index + 1).padStart(3, '0'),
      title: firstLine ? stripMarkdown(firstLine) : 'Untitled decision',
      path: '',
      content,
      links: { decisions: [], specs: [], supersedes: [], superseded_by: [] },
    }
  })
}

/** `content` without its leading heading when it repeats `title` (case insensitive) */
export const stripTitleHeading = (content: string, title: string): string => {
  const lines = content.split('\n')
  const first = lines.findIndex((line) => line.trim().length > 0)
  if (first === -1) return content
  const heading = lines[first].trim()
  if (
    heading.startsWith('#') &&
    stripMarkdown(heading).toLowerCase() === stripMarkdown(title).toLowerCase()
  ) {
    return lines
      .slice(first + 1)
      .join('\n')
      .trimStart()
  }
  return content
}

/** Markdown of a record without its leading heading when it repeats the title */
export const contentWithoutTitle = (record: FeatureRecord): string =>
  stripTitleHeading(record.content, record.title)

export type StatusTone = 'neutral' | 'progress' | 'success' | 'muted' | 'danger'

export const statusTone = (status?: string): StatusTone => {
  switch (status) {
    case 'accepted':
    case 'implemented':
      return 'success'
    case 'draft':
    case 'proposed':
      return 'progress'
    case 'superseded':
    case 'deprecated':
      return 'muted'
    case 'rejected':
      return 'danger'
    default:
      return 'neutral'
  }
}
