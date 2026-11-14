import { describe, expect, it } from 'vitest'
import type { Change, Feature } from '@/models/feature'
import {
  buildHistory,
  commitType,
  EMPTY_FILTERS,
  type HistoryFilterState,
  filterHistory,
  formatMonth,
  groupByMonth,
  monthlyActivity,
  summarizeHistory,
} from '../utils'

const change = (hash: string, date: string, title = hash): Change => ({
  hash,
  date,
  title,
  description: '',
  author_name: `author-${hash}`,
  author_email: `${hash}@example.com`,
})

const feature = (
  path: string,
  changes: Change[],
  features: Feature[] = [],
): Feature => ({
  name: path.split('/').at(-1) ?? path,
  path,
  owner: '',
  is_owner_inherited: false,
  changes,
  decisions: [],
  dependencies: [],
  features,
})

// `auth/login` is a sub-feature of `auth`: its commits are listed by both
const c1 = change('c1', '2025-01-10 10:00:00', 'feat(auth): add auth')
const c2 = change('c2', '2025-02-05 10:00:00', 'feat: add login')
const c3 = change('c3', '2025-02-20 10:00:00', 'fix(auth): token expiry')
const c4 = change('c4', '2025-04-01 10:00:00', 'chore: bump billing')
const login = feature('auth/login', [c3, c2])
const auth = feature('auth', [c3, c2, c1], [login])
const billing = feature('billing', [c4, c3])
const features = [auth, billing]

describe('commitType', () => {
  it('reads conventional commit types, with scope and breaking marker', () => {
    expect(commitType('feat: x')).toBe('feat')
    expect(commitType('Fix(api): x')).toBe('fix')
    expect(commitType('refactor!: x')).toBe('refactor')
    expect(commitType('feat(web)!: x')).toBe('feat')
  })

  it('falls back to other', () => {
    expect(commitType('Merge branch main')).toBe('other')
    expect(commitType('wip: stuff')).toBe('other')
  })
})

describe('buildHistory', () => {
  const history = buildHistory(features)

  it('dedupes commits and sorts them newest first', () => {
    expect(history.map((c) => c.hash)).toEqual(['c4', 'c3', 'c2', 'c1'])
  })

  it('attributes a commit to the deepest features listing it', () => {
    const paths = (hash: string) =>
      history.find((c) => c.hash === hash)?.features.map((f) => f.path)

    expect(paths('c1')).toEqual(['auth'])
    expect(paths('c2')).toEqual(['auth/login'])
    expect(paths('c3')).toEqual(['auth/login', 'billing'])
    expect(paths('c4')).toEqual(['billing'])
  })

  it('marks the oldest commit of each feature as introducing it', () => {
    const introduces = (hash: string) =>
      history.find((c) => c.hash === hash)?.introduces.map((f) => f.path)

    expect(introduces('c1')).toEqual(['auth'])
    expect(introduces('c2')).toEqual(['auth/login'])
    expect(introduces('c3')).toEqual(['billing'])
    expect(introduces('c4')).toEqual([])
  })

  it('reads the commit type', () => {
    expect(history.map((c) => c.type)).toEqual(['chore', 'fix', 'feat', 'feat'])
  })

  it('is empty without changes', () => {
    expect(buildHistory([feature('a', [])])).toEqual([])
  })
})

describe('filterHistory', () => {
  const history = buildHistory(features)
  const hashes = (filters: Partial<HistoryFilterState>) =>
    filterHistory(history, { ...EMPTY_FILTERS, ...filters }, features).map(
      (c) => c.hash,
    )

  it('keeps everything without filters', () => {
    expect(hashes({})).toEqual(['c4', 'c3', 'c2', 'c1'])
  })

  it('filters by feature, including its sub-features', () => {
    expect(hashes({ featurePath: 'auth' })).toEqual(['c3', 'c2', 'c1'])
    expect(hashes({ featurePath: 'auth/login' })).toEqual(['c3', 'c2'])
    expect(hashes({ featurePath: 'billing' })).toEqual(['c4', 'c3'])
  })

  it('filters by type', () => {
    expect(hashes({ type: 'feat' })).toEqual(['c2', 'c1'])
  })

  it('searches title, author and hash, case-insensitively', () => {
    expect(hashes({ query: 'TOKEN' })).toEqual(['c3'])
    expect(hashes({ query: 'author-c4' })).toEqual(['c4'])
    expect(hashes({ query: 'c1' })).toEqual(['c1'])
  })

  it('combines filters', () => {
    expect(hashes({ featurePath: 'auth', type: 'fix' })).toEqual(['c3'])
  })
})

describe('groupByMonth', () => {
  it('groups consecutive commits of the same month', () => {
    const months = groupByMonth(buildHistory(features))
    expect(months.map((m) => [m.key, m.commits.map((c) => c.hash)])).toEqual([
      ['2025-04', ['c4']],
      ['2025-02', ['c3', 'c2']],
      ['2025-01', ['c1']],
    ])
  })
})

describe('monthlyActivity', () => {
  it('counts commits per month, oldest first, filling empty months', () => {
    expect(monthlyActivity(buildHistory(features))).toEqual([
      { month: '2025-01', commits: 1 },
      { month: '2025-02', commits: 2 },
      { month: '2025-03', commits: 0 },
      { month: '2025-04', commits: 1 },
    ])
  })

  it('crosses years', () => {
    const history = buildHistory([
      feature('a', [
        change('b', '2025-01-01 00:00:00'),
        change('a', '2024-12-01 00:00:00'),
      ]),
    ])
    expect(monthlyActivity(history).map((m) => m.month)).toEqual([
      '2024-12',
      '2025-01',
    ])
  })

  it('is empty without commits', () => {
    expect(monthlyActivity([])).toEqual([])
  })
})

describe('summarizeHistory', () => {
  it('counts commits, introductions, contributors and the period', () => {
    expect(summarizeHistory(buildHistory(features))).toEqual({
      commits: 4,
      introduced: 3,
      contributors: 4,
      first: '2025-01-10 10:00:00',
      last: '2025-04-01 10:00:00',
    })
  })
})

describe('summarizeHistory with a feature scope', () => {
  it('only counts the introductions of features in scope', () => {
    const history = buildHistory(features)
    expect(summarizeHistory(history, new Set(['auth/login'])).introduced).toBe(
      1,
    )
  })
})

describe('formatMonth', () => {
  it('formats a month key', () => {
    expect(formatMonth('2025-03')).toBe('March 2025')
    expect(formatMonth('2025-03', 'short')).toBe('Mar 25')
  })
})
