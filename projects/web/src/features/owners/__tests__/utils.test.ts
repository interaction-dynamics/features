import { describe, expect, it } from 'vitest'
import { parseFeaturesData } from '@/components/features-provider'
import type { Change, Feature, FeatureRecord } from '@/models/feature'
import {
  ownerCommits,
  ownerFromParam,
  ownerPath,
  summarizeOwner,
  UNOWNED_SEGMENT,
} from '../utils'

const change = (hash: string, date: string, author = hash): Change => ({
  hash,
  date,
  title: `feat: ${hash}`,
  description: '',
  author_name: author,
  author_email: `${author}@example.com`,
})

const record = (id: string, date?: string): FeatureRecord => ({
  kind: 'spec',
  id,
  title: id,
  date,
  path: `${id}.md`,
  content: '',
  links: { decisions: [], specs: [], supersedes: [], superseded_by: [] },
})

const feature = (
  path: string,
  owner: string,
  overrides: Partial<Feature> = {},
): Feature => ({
  name: path.split('/').at(-1) ?? path,
  path,
  owner,
  is_owner_inherited: false,
  changes: [],
  decisions: [],
  dependencies: [],
  ...overrides,
})

const c1 = change('c1', '2025-01-10 10:00:00', 'ada')
const c2 = change('c2', '2025-02-05 10:00:00', 'bob')
const c3 = change('c3', '2025-03-20 10:00:00', 'ada')
const c4 = change('c4', '2025-04-01 10:00:00', 'eve')

// `auth` is owned by @team/web; `auth/login` inherits it and
// `auth/login/sso` inherits it through `auth/login`; `auth/admin` and
// `billing` have other owners.
const tree = (): Feature[] =>
  parseFeaturesData([
    feature('auth', '@team/web', {
      changes: [c3, c1],
      stats: {
        files_count: 3,
        lines_count: 100,
        todos_count: 2,
        commits: {},
        coverage: {
          lines_total: 80,
          lines_covered: 60,
          lines_missed: 20,
          line_coverage_percent: 75,
        },
      },
      external: { errors: 4, plan: 'pro' },
      records: [record('001-login', '2025-01-01')],
      features: [
        feature('auth/login', '@team/web', {
          is_owner_inherited: true,
          changes: [c3, c2],
          stats: {
            files_count: 2,
            lines_count: 50,
            todos_count: 1,
            commits: {},
            coverage: {
              lines_total: 20,
              lines_covered: 20,
              lines_missed: 0,
              line_coverage_percent: 100,
            },
          },
          external: { errors: 6, plan: 'free' },
          records: [record('002-sso', '2025-03-01'), record('003-undated')],
          features: [
            feature('auth/login/sso', '@team/web', {
              is_owner_inherited: true,
              stats: { files_count: 1, lines_count: 10, commits: {} },
            }),
          ],
        }),
        feature('auth/admin', 'Jane Doe', { changes: [c4] }),
      ],
    }),
    feature('billing', '', { changes: [c4] }),
  ]).features

describe('ownerPath / ownerFromParam', () => {
  it('encodes owners with @, / and spaces as a single segment', () => {
    expect(ownerPath('@team/web')).toBe('/owners/%40team%2Fweb')
    expect(ownerPath('Jane Doe')).toBe('/owners/Jane%20Doe')
  })

  it('round-trips through the decoded route param', () => {
    for (const owner of ['@team/web', 'Jane Doe', 'a%b', 'é']) {
      const segment = ownerPath(owner).slice('/owners/'.length)
      expect(ownerFromParam(decodeURIComponent(segment))).toBe(owner)
    }
  })

  it('uses a reserved segment for features without owner', () => {
    expect(ownerPath('')).toBe(`/owners/${UNOWNED_SEGMENT}`)
    expect(ownerFromParam(UNOWNED_SEGMENT)).toBe('')
  })
})

describe('summarizeOwner', () => {
  it('returns undefined for an owner without feature', () => {
    expect(summarizeOwner(tree(), 'nobody')).toBeUndefined()
  })

  it('lists own and inherited features, with the feature they inherit from', () => {
    const summary = summarizeOwner(tree(), '@team/web')

    expect(
      summary?.features.map((f) => [
        f.feature.path,
        f.inherited,
        f.inheritedFrom?.path,
      ]),
    ).toEqual([
      ['auth', false, undefined],
      ['auth/login', true, 'auth'],
      ['auth/login/sso', true, 'auth'],
    ])
    expect(summary?.totals).toMatchObject({
      features: 3,
      direct: 1,
      inherited: 2,
    })
  })

  it('sums files, lines, TODOs and coverage of the features', () => {
    const totals = summarizeOwner(tree(), '@team/web')?.totals

    expect(totals).toMatchObject({ files: 6, lines: 160, todos: 3 })
    expect(totals?.coverage).toEqual({
      linesTotal: 100,
      linesCovered: 80,
      percent: 80,
    })
  })

  it('has no coverage when no feature has coverage data', () => {
    expect(summarizeOwner(tree(), 'Jane Doe')?.totals.coverage).toBeUndefined()
  })

  it('counts unique commits and contributors', () => {
    const totals = summarizeOwner(tree(), '@team/web')?.totals

    expect(totals).toMatchObject({
      commits: 3,
      contributors: 2,
      lastChange: '2025-03-20 10:00:00',
    })
  })

  it('sums numeric external metrics, counts the others and lists every value', () => {
    const external = summarizeOwner(tree(), '@team/web')?.external

    expect(
      external?.map((m) => ({
        key: m.key,
        total: m.total,
        counts: m.counts,
        values: m.values.map((v) => [v.feature.path, v.value]),
      })),
    ).toEqual([
      {
        key: 'errors',
        total: 10,
        counts: undefined,
        values: [
          ['auth', 4],
          ['auth/login', 6],
        ],
      },
      {
        key: 'plan',
        total: undefined,
        counts: [
          { value: 'pro', count: 1 },
          { value: 'free', count: 1 },
        ],
        values: [
          ['auth', 'pro'],
          ['auth/login', 'free'],
        ],
      },
    ])
  })

  it('lists the records of the features newest first, with their feature', () => {
    const records = summarizeOwner(tree(), '@team/web')?.records

    expect(records?.map((r) => [r.record.id, r.feature.path])).toEqual([
      ['002-sso', 'auth/login'],
      ['001-login', 'auth'],
      ['003-undated', 'auth/login'],
    ])
  })

  it('gathers the features without owner under the empty owner', () => {
    const summary = summarizeOwner(tree(), '')

    expect(summary?.features.map((f) => f.feature.path)).toEqual(['billing'])
  })
})

describe('ownerCommits', () => {
  it('deduplicates commits by hash, newest first, with the features listing them', () => {
    const [auth] = tree()
    const login = auth.features?.[0] as Feature
    const commits = ownerCommits([auth, login])

    expect(commits.map((c) => [c.hash, c.features.map((f) => f.path)])).toEqual(
      [
        ['c3', ['auth', 'auth/login']],
        ['c2', ['auth/login']],
        ['c1', ['auth']],
      ],
    )
  })

  it('marks the oldest commit of each feature as introducing it', () => {
    const [auth] = tree()
    const login = auth.features?.[0] as Feature
    const commits = ownerCommits([auth, login])

    expect(
      commits.map((c) => [c.hash, c.introduces.map((f) => f.path)]),
    ).toEqual([
      ['c3', []],
      ['c2', ['auth/login']],
      ['c1', ['auth']],
    ])
  })
})
