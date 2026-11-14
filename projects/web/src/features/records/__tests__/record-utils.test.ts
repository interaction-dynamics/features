import { describe, expect, it } from 'vitest'
import type { DocRecord, Feature } from '@/models/feature'
import {
  countByKind,
  fileMarkdown,
  filterRecords,
  indexRecords,
  recordLocation,
  recordsOf,
  resolveLink,
  sortRecords,
  statusTone,
  titleFromMarkdown,
  withoutTitle,
} from '../record-utils'

const record = (overrides: Partial<DocRecord>): DocRecord => ({
  kind: 'decision',
  id: '001-x',
  title: 'X',
  path: 'x.md',
  content: '',
  source: 'native',
  links: { decisions: [], specs: [], supersedes: [], superseded_by: [] },
  ...overrides,
})

const feature = (overrides: Partial<Feature>): Feature => ({
  name: 'auth',
  path: 'features/auth',
  owner: '',
  is_owner_inherited: false,
  changes: [],
  decisions: [],
  dependencies: [],
  ...overrides,
})

describe('recordsOf', () => {
  it('returns the records of the feature', () => {
    const records = [record({})]
    expect(recordsOf(feature({ records }))).toBe(records)
  })

  it('falls back to the raw decisions of older reports', () => {
    const records = recordsOf(
      feature({ decisions: ['# Use **JWT**\n\nBody', ''] }),
    )
    expect(records).toHaveLength(2)
    expect(records[0]).toMatchObject({ kind: 'decision', title: 'Use JWT' })
    expect(records[1].title).toBe('Untitled Decision')
  })
})

describe('titleFromMarkdown', () => {
  it('strips headings and markup', () => {
    expect(titleFromMarkdown('## [Use](x) `jwt`')).toBe('Use jwt')
  })
})

describe('sortRecords', () => {
  it('sorts newest first, undated last', () => {
    const sorted = sortRecords([
      record({ id: 'a' }),
      record({ id: 'b', date: '2026-01-01' }),
      record({ id: 'c', date: '2026-03-01' }),
    ])
    expect(sorted.map((r) => r.id)).toEqual(['c', 'b', 'a'])
  })
})

describe('filterRecords', () => {
  const records = [
    record({ id: 'spec', kind: 'spec', status: 'accepted' }),
    record({ id: 'old', status: 'superseded' }),
    record({ id: 'no', status: 'rejected' }),
    record({ id: 'adr', status: 'accepted' }),
  ]

  it('filters by kind', () => {
    expect(filterRecords(records, 'spec', false).map((r) => r.id)).toEqual([
      'spec',
    ])
    expect(filterRecords(records, 'decision', false)).toHaveLength(3)
  })

  it('hides superseded and rejected records', () => {
    expect(filterRecords(records, 'all', true).map((r) => r.id)).toEqual([
      'spec',
      'adr',
    ])
  })

  it('counts by kind', () => {
    expect(countByKind(records)).toEqual({ all: 4, spec: 1, decision: 3 })
  })
})

describe('resolveLink', () => {
  const features = [
    feature({
      path: 'a',
      records: [record({ id: '001-x', path: 'a/x.md' })],
      features: [
        feature({ path: 'a/b', records: [record({ id: '001-x', path: 'b' })] }),
      ],
    }),
  ]
  const index = indexRecords(features, [record({ id: '002-y', path: 'y' })])

  it('prefers the same feature', () => {
    expect(resolveLink('001-x', 'a/b', index)?.record.path).toBe('b')
  })

  it('falls back to project-level records, then any record', () => {
    expect(resolveLink('002-y', 'a', index)?.featurePath).toBeNull()
    expect(resolveLink('001-x', null, index)?.record.path).toBe('a/x.md')
    expect(resolveLink('404', 'a', index)).toBeUndefined()
  })

  it('builds the location of a record', () => {
    expect(
      recordLocation({
        record: record({ path: 'a/x y.md' }),
        featurePath: 'a',
      }),
    ).toEqual({ pathname: '/', search: '?feature=a&record=a%2Fx%20y.md' })
    expect(
      recordLocation({ record: record({ path: 'y' }), featurePath: null }),
    ).toEqual({ pathname: '/records', search: '?record=y' })
  })
})

describe('statusTone', () => {
  it('groups statuses', () => {
    expect(statusTone('implemented')).toBe('positive')
    expect(statusTone('draft')).toBe('progress')
    expect(statusTone('superseded')).toBe('inactive')
    expect(statusTone(undefined)).toBe('neutral')
  })
})

describe('fileMarkdown', () => {
  it('fences code files', () => {
    expect(fileMarkdown('plan.md', '# Plan')).toBe('# Plan')
    expect(fileMarkdown('contracts/api.yaml', 'a: 1')).toBe(
      '```yaml\na: 1\n```',
    )
  })
})

describe('withoutTitle', () => {
  it('drops the leading level 1 heading only', () => {
    expect(withoutTitle('# Title\n\n## Context')).toBe('\n## Context')
    expect(withoutTitle('## Context\n# Title')).toBe('## Context\n# Title')
  })
})
