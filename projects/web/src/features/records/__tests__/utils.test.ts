import { describe, expect, it } from 'vitest'
import type { FeatureRecord } from '@/models/feature'
import {
  contentWithoutTitle,
  featureRecords,
  fileMarkdown,
  filterRecords,
  isToolRecord,
  findLinkTarget,
  recordLinks,
  sortRecords,
  statusTone,
} from '../utils'

const record = (
  overrides: Partial<FeatureRecord> & Pick<FeatureRecord, 'id'>,
): FeatureRecord => ({
  kind: 'decision',
  title: overrides.id,
  path: `${overrides.id}.md`,
  content: '',
  links: { decisions: [], specs: [], supersedes: [], superseded_by: [] },
  ...overrides,
})

describe('sortRecords', () => {
  it('puts the newest first and records without a date last', () => {
    const sorted = sortRecords([
      record({ id: '001', date: '2025-01-01' }),
      record({ id: '003' }),
      record({ id: '002', date: '2025-06-01' }),
    ])
    expect(sorted.map((r) => r.id)).toEqual(['002', '001', '003'])
  })
})

describe('filterRecords', () => {
  const records = [
    record({ id: 'a', kind: 'spec', status: 'accepted' }),
    record({ id: 'b', status: 'superseded' }),
    record({ id: 'c', status: 'rejected' }),
    record({ id: 'd' }),
  ]

  it('filters by kind', () => {
    expect(filterRecords(records, 'spec', false).map((r) => r.id)).toEqual([
      'a',
    ])
    expect(filterRecords(records, 'decision', false)).toHaveLength(3)
  })

  it('hides superseded and rejected records', () => {
    expect(filterRecords(records, 'all', true).map((r) => r.id)).toEqual([
      'a',
      'd',
    ])
  })
})

describe('recordLinks and findLinkTarget', () => {
  it('lists links with the kind of their target', () => {
    const spec = record({
      id: '001-login',
      kind: 'spec',
      links: {
        decisions: ['002-jwt'],
        specs: [],
        supersedes: ['000-old'],
        superseded_by: [],
      },
    })
    expect(recordLinks(spec)).toEqual([
      { label: 'ADR', targetId: '002-jwt', targetKind: 'decision' },
      { label: 'Supersedes', targetId: '000-old', targetKind: 'spec' },
    ])
  })

  it('prefers a target of the expected kind', () => {
    const records = [
      record({ id: '001', kind: 'spec' }),
      record({ id: '001', kind: 'decision', title: 'decision' }),
    ]
    expect(
      findLinkTarget(records, { targetId: '001', targetKind: 'decision' })
        ?.title,
    ).toBe('decision')
    expect(
      findLinkTarget(records, { targetId: '404', targetKind: 'spec' }),
    ).toBeUndefined()
  })
})

describe('featureRecords', () => {
  it('uses records when present', () => {
    const records = [record({ id: '001' })]
    expect(featureRecords({ records, decisions: ['# Other'] })).toBe(records)
  })

  it('falls back to the legacy decisions of older reports', () => {
    const records = featureRecords({ decisions: ['# Use **JWT**\n\nBody'] })
    expect(records).toHaveLength(1)
    expect(records[0].title).toBe('Use JWT')
    expect(records[0].kind).toBe('decision')
  })
})

describe('statusTone', () => {
  it('maps statuses to tones', () => {
    expect(statusTone('implemented')).toBe('success')
    expect(statusTone('draft')).toBe('progress')
    expect(statusTone('superseded')).toBe('muted')
    expect(statusTone('rejected')).toBe('danger')
    expect(statusTone(undefined)).toBe('neutral')
  })
})

describe('contentWithoutTitle', () => {
  it('drops the heading repeating the title', () => {
    expect(
      contentWithoutTitle(
        record({ id: 'a', title: 'Use JWT', content: '\n# Use `JWT`\n\nBody' }),
      ),
    ).toBe('Body')
  })

  it('keeps other headings', () => {
    const content = '# Something else\n\nBody'
    expect(contentWithoutTitle(record({ id: 'a', title: 'A', content }))).toBe(
      content,
    )
  })
})

describe('Spec Kit and Kiro records', () => {
  it('only flags records read from a tool layout', () => {
    expect(isToolRecord(record({ id: '001-a' }))).toBe(false)
    expect(isToolRecord(record({ id: '001-a', source: 'native' }))).toBe(false)
    expect(isToolRecord(record({ id: '003-auth', source: 'speckit' }))).toBe(
      true,
    )
  })

  it('shows contracts as code and drops the title of markdown files', () => {
    expect(fileMarkdown('contracts/login.yaml', 'openapi: 3.0.0\n')).toBe(
      '```yaml\nopenapi: 3.0.0\n```',
    )
    expect(fileMarkdown('plan.md', '# Plan\n\nSteps')).not.toContain('# Plan')
  })
})
