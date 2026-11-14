import { describe, expect, it } from 'vitest'
import type { Change, Feature } from '@/models/feature'
import { buildFeatureGraph, dateTime, lineage, monthTicks } from '../graph'

const change = (hash: string, date: string): Change => ({
  hash,
  date,
  title: `feat: ${hash}`,
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

const c1 = change('c1', '2025-01-10 10:00:00')
const c2 = change('c2', '2025-02-05 10:00:00')
const c3 = change('c3', '2025-02-20 10:00:00')
const c4 = change('c4', '2025-04-01 10:00:00')

// A sub-feature's commits are also listed by its parent
const signup = feature('auth/signup', [c3])
const login = feature('auth/login', [c2])
const auth = feature('auth', [c3, c2, c1], [signup, login])
const billing = feature('billing', [c4])
const draft = feature('draft', [], [feature('draft/nested', [])])

describe('buildFeatureGraph', () => {
  const graph = buildFeatureGraph([billing, auth, draft])

  it('orders lanes parents first, siblings by creation date', () => {
    expect(graph.lanes.map((lane) => lane.feature.path)).toEqual([
      'auth',
      'auth/login',
      'auth/signup',
      'billing',
    ])
  })

  it('links each lane to its parent and top-level lane', () => {
    expect(
      graph.lanes.map((lane) => [lane.depth, lane.parent, lane.root]),
    ).toEqual([
      [0, -1, 0],
      [1, 0, 0],
      [1, 0, 0],
      [0, -1, 3],
    ])
  })

  it('starts each lane at its oldest commit and ends at its newest', () => {
    const [authLane] = graph.lanes
    expect(authLane.creation.hash).toBe('c1')
    expect(authLane.lastChange).toBe(c3.date)
    expect(authLane.commits).toBe(3)
    expect(authLane.children).toBe(2)
  })

  it('spans the whole history and counts features without commits', () => {
    expect(graph.start).toBe(c1.date)
    expect(graph.end).toBe(c4.date)
    expect(graph.withoutCommits).toBe(2)
  })

  it('is empty without commits', () => {
    expect(buildFeatureGraph([draft])).toEqual({
      lanes: [],
      start: '',
      end: '',
      withoutCommits: 2,
    })
  })
})

describe('lineage', () => {
  const nested = feature('auth/login/otp', [c3])
  const tree = feature(
    'auth',
    [c3, c2, c1],
    [feature('auth/login', [c3, c2], [nested]), feature('auth/signup', [c3])],
  )
  const { lanes } = buildFeatureGraph([tree, billing])
  const index = (path: string) =>
    lanes.findIndex((lane) => lane.feature.path === path)

  it('keeps ancestors and descendants, not siblings', () => {
    const related = lineage(lanes, index('auth/login'))
    expect([...related].map((i) => lanes[i].feature.path).sort()).toEqual([
      'auth',
      'auth/login',
      'auth/login/otp',
    ])
  })
})

describe('monthTicks', () => {
  it('lists the months between two dates', () => {
    expect(monthTicks('2024-11-20 10:00:00', '2025-02-01 10:00:00')).toEqual([
      '2024-11',
      '2024-12',
      '2025-01',
      '2025-02',
    ])
  })

  it('keeps one month every step', () => {
    expect(monthTicks('2025-01-01 00:00:00', '2025-06-30 00:00:00', 2)).toEqual(
      ['2025-01', '2025-03', '2025-05'],
    )
  })
})

describe('dateTime', () => {
  it('reads CLI dates as UTC', () => {
    expect(dateTime('2025-01-10 10:00:00')).toBe(
      Date.UTC(2025, 0, 10, 10, 0, 0),
    )
  })
})
