import { describe, expect, it } from 'vitest'
import { computeHealthScore, isErrorMetric } from '../health-score'
import { makeFeature } from './fixtures'

describe('computeHealthScore', () => {
  it('returns no score when no signal exists', () => {
    const health = computeHealthScore(makeFeature('empty'))
    expect(health.score).toBeUndefined()
    expect(health.penalties).toEqual([])
    expect(health.missing.map((item) => item.signal)).toEqual([
      'coverage',
      'debt',
      'fixes',
      'errors',
    ])
  })

  it('scores 100 for a fully covered feature without debt', () => {
    const health = computeHealthScore(
      makeFeature('clean', { coverage: 100, lines: 500, todos: 0 }),
    )
    expect(health.score).toBe(100)
    expect(health.penalties).toEqual([])
  })

  it('penalizes low coverage up to 40 points', () => {
    expect(
      computeHealthScore(makeFeature('half', { coverage: 50 })).score,
    ).toBe(80)
    const none = computeHealthScore(makeFeature('none', { coverage: 0 }))
    expect(none.score).toBe(60)
    expect(none.penalties[0]).toEqual({
      signal: 'coverage',
      points: 40,
      reason: 'Line coverage 0%',
    })
  })

  it('uses the TODO density with lines floored at 1,000', () => {
    // 2 TODOs in 100 lines counts as 2 per 1,000 lines → 8 points
    expect(
      computeHealthScore(makeFeature('small', { lines: 100, todos: 2 })).score,
    ).toBe(92)
    // 10 TODOs in 2,000 lines = 5 per 1,000 → 20 points (max)
    const debt = computeHealthScore(
      makeFeature('big', { lines: 2000, todos: 10 }),
    )
    expect(debt.score).toBe(80)
    expect(debt.penalties[0].reason).toBe('10 TODOs in 2,000 lines')
    // capped at 20
    expect(
      computeHealthScore(makeFeature('worse', { lines: 1000, todos: 50 }))
        .score,
    ).toBe(80)
  })

  it('penalizes fix-heavy history only with enough commits', () => {
    const few = computeHealthScore(
      makeFeature('few', { lines: 10, commits: 4, fixes: 4 }),
    )
    expect(few.score).toBe(100)
    expect(few.missing.find((item) => item.signal === 'fixes')?.reason).toBe(
      'Fewer than 5 commits',
    )

    const fixes = computeHealthScore(
      makeFeature('fixes', { commits: 10, fixes: 3 }),
    )
    expect(fixes.score).toBe(91)
    expect(fixes.penalties[0].reason).toBe('30% of 10 commits are fixes')

    expect(
      computeHealthScore(makeFeature('all', { commits: 10, fixes: 10 })).score,
    ).toBe(85)
  })

  it('reports a missing history when there are no commits', () => {
    const health = computeHealthScore(makeFeature('nohistory', { lines: 10 }))
    expect(health.missing.find((item) => item.signal === 'fixes')?.reason).toBe(
      'No git history',
    )
  })

  it('penalizes imported error and bug metrics on a log scale', () => {
    const health = computeHealthScore(
      makeFeature('errors', {
        external: { open_errors: 9, weekly_users: 5000, team: 'core' },
      }),
    )
    // 8 × log10(10) = 8
    expect(health.score).toBe(92)
    expect(health.penalties[0]).toEqual({
      signal: 'errors',
      points: 8,
      reason: 'open_errors: 9',
    })

    const many = computeHealthScore(
      makeFeature('many', { external: { sentry_errors: 9999, open_bugs: 99 } }),
    )
    expect(many.score).toBe(75)
  })

  it('ignores non-numeric imported metrics', () => {
    const health = computeHealthScore(
      makeFeature('text', { external: { error_owner: 'team-a' } }),
    )
    expect(health.score).toBeUndefined()
  })

  it('combines penalties, largest first, and never goes below 0', () => {
    const health = computeHealthScore(
      makeFeature('bad', {
        coverage: 0,
        lines: 1000,
        todos: 50,
        commits: 20,
        fixes: 20,
        external: { open_errors: 100000 },
      }),
    )
    expect(health.score).toBe(0)
    expect(health.penalties.map((penalty) => penalty.points)).toEqual([
      40, 25, 20, 15,
    ])
  })
})

describe('isErrorMetric', () => {
  it('matches error and bug metrics', () => {
    expect(isErrorMetric('open_errors')).toBe(true)
    expect(isErrorMetric('Bugs')).toBe(true)
    expect(isErrorMetric('crash_rate')).toBe(true)
    expect(isErrorMetric('weekly_users')).toBe(false)
  })
})
