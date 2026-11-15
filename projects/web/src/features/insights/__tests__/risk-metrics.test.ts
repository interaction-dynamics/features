import { describe, expect, it } from 'vitest'
import {
  availableMetrics,
  buildRiskPoints,
  defaultAxes,
  formatMetricTick,
  formatMetricValue,
  median,
  metricsForAxis,
  quadrantLabels,
  quadrantOf,
} from '../risk-metrics'
import { makeFeature } from './fixtures'

const ids = (metrics: { id: string }[]) => metrics.map((metric) => metric.id)

describe('availableMetrics', () => {
  const features = [
    makeFeature('a', {
      coverage: 80,
      lines: 200,
      todos: 1,
      commits: 12,
      fixes: 2,
      external: { weekly_users: 100, tier: 'gold' },
    }),
    makeFeature('b', { lines: 100, commits: 3 }),
  ]

  it('lists built-in metrics with data, then numeric imported metrics', () => {
    expect(ids(availableMetrics(features))).toEqual([
      'untested',
      'todo_density',
      'commits',
      'fix_commits',
      'external:weekly_users',
    ])
  })

  it('drops the commit metrics with --skip-changes', () => {
    expect(ids(availableMetrics(features, { skipChanges: true }))).toEqual([
      'untested',
      'todo_density',
      'external:weekly_users',
    ])
  })

  it('drops the commit metrics when no commit was read', () => {
    expect(
      ids(availableMetrics([makeFeature('a', { lines: 10, commits: 0 })])),
    ).toEqual(['todo_density'])
  })

  it('drops coverage when no feature has coverage', () => {
    expect(ids(availableMetrics([makeFeature('a', { commits: 2 })]))).toEqual([
      'commits',
      'fix_commits',
    ])
  })

  it('returns nothing for features without data', () => {
    expect(availableMetrics([makeFeature('a')])).toEqual([])
    expect(availableMetrics([])).toEqual([])
  })
})

describe('defaultAxes', () => {
  it('picks untested lines against commits', () => {
    const metrics = availableMetrics([
      makeFeature('a', { coverage: 50, lines: 10, commits: 4 }),
    ])
    const { x, y } = defaultAxes(metrics)
    expect(x?.id).toBe('untested')
    expect(y?.id).toBe('commits')
  })

  it('falls back on TODO density and imported metrics', () => {
    const metrics = availableMetrics([
      makeFeature('a', { lines: 10, external: { weekly_users: 3 } }),
    ])
    const { x, y } = defaultAxes(metrics)
    expect(x?.id).toBe('todo_density')
    expect(y?.id).toBe('external:weekly_users')
  })

  it('avoids using the same imported metric on both axes', () => {
    const metrics = availableMetrics([
      makeFeature('a', { external: { open_errors: 1, weekly_users: 3 } }),
    ])
    const { x, y } = defaultAxes(metrics)
    expect(x?.id).toBe('external:open_errors')
    expect(y?.id).toBe('external:weekly_users')
  })

  it('returns undefined axes without metrics', () => {
    expect(defaultAxes([])).toEqual({ x: undefined, y: undefined })
  })

  it('offers imported metrics on both axes', () => {
    const metrics = availableMetrics([
      makeFeature('a', { lines: 10, commits: 2, external: { users: 1 } }),
    ])
    expect(ids(metricsForAxis(metrics, 'risk'))).toEqual([
      'todo_density',
      'external:users',
    ])
    expect(ids(metricsForAxis(metrics, 'activity'))).toEqual([
      'commits',
      'fix_commits',
      'external:users',
    ])
  })
})

describe('buildRiskPoints', () => {
  it('keeps features with a value on both axes', () => {
    const features = [
      makeFeature('a', { coverage: 30, lines: 400, commits: 10 }),
      makeFeature('b', { lines: 50, commits: 2 }),
      makeFeature('c', { coverage: 90, commits: 1 }),
    ]
    const metrics = availableMetrics(features)
    const { x, y } = defaultAxes(metrics)
    if (!x || !y) throw new Error('missing axes')
    const { points, hidden } = buildRiskPoints(features, x, y)
    expect(hidden).toBe(1)
    expect(
      points.map((point) => [point.feature.name, point.x, point.y]),
    ).toEqual([
      ['a', 70, 10],
      ['c', 10, 1],
    ])
    expect(points[0].size).toBe(400)
    expect(points[1].size).toBe(1)
  })
})

describe('median', () => {
  it('handles odd, even and empty lists', () => {
    expect(median([3, 1, 2])).toBe(2)
    expect(median([4, 1, 3, 2])).toBe(2.5)
    expect(median([])).toBeUndefined()
  })
})

describe('quadrantOf', () => {
  it('splits at the medians, a value on the line counts as low', () => {
    expect(quadrantOf({ x: 5, y: 5 }, 1, 1)).toBe('top-right')
    expect(quadrantOf({ x: 0, y: 5 }, 1, 1)).toBe('top-left')
    expect(quadrantOf({ x: 5, y: 0 }, 1, 1)).toBe('bottom-right')
    expect(quadrantOf({ x: 1, y: 1 }, 1, 1)).toBe('bottom-left')
  })
})

describe('quadrantLabels', () => {
  it('adapts the top-right label to the risk metric', () => {
    const [untested, todo, users] = availableMetrics([
      makeFeature('a', { coverage: 1, lines: 1, external: { users: 1 } }),
    ])
    expect(quadrantLabels(untested)['top-right']).toBe('Test this first')
    expect(quadrantLabels(todo)['top-right']).toBe('Pay down debt first')
    expect(quadrantLabels(users)['top-right']).toBe('Look here first')
    expect(quadrantLabels(users)['bottom-right']).toBe('Candidate for removal')
  })
})

describe('formatMetricValue', () => {
  it('rounds and adds the unit', () => {
    const [untested, todo] = availableMetrics([
      makeFeature('a', { coverage: 1, lines: 1 }),
    ])
    expect(formatMetricValue(untested, 12.345)).toBe('12.3%')
    expect(formatMetricValue(todo, 1200)).toBe('1,200')
  })
})

describe('formatMetricTick', () => {
  it('uses compact numbers', () => {
    const [untested, todo] = availableMetrics([
      makeFeature('a', { coverage: 1, lines: 1 }),
    ])
    expect(formatMetricTick(untested, 25)).toBe('25%')
    expect(formatMetricTick(todo, 12500)).toBe('12.5K')
  })
})
