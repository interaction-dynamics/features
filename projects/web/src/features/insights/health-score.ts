import type { Feature } from '@/models/feature'

/**
 * Health score of a feature, from 0 (worst) to 100 (best).
 *
 * The score is `100 − penalties`, clamped to [0, 100]. Each penalty only
 * applies when its signal exists in the report:
 *
 * | Signal                   | Penalty                                        | Max |
 * | ------------------------ | ---------------------------------------------- | --- |
 * | Line coverage            | 0.4 × (100 − coverage %)                       | 40  |
 * | TODO density             | 4 × TODOs per 1,000 lines (lines floored 1,000) | 20  |
 * | Fix-heavy history        | 30 × fix commits / commits (≥ 5 commits)       | 15  |
 * | Imported errors and bugs | Σ 8 × log10(1 + value) over matching metrics   | 25  |
 *
 * Imported metrics count when their name contains error, bug, crash,
 * exception, incident or issue. Penalties are rounded so the reasons add up
 * to the score. When no signal exists, the score is `undefined`.
 */

export type HealthSignal = 'coverage' | 'debt' | 'fixes' | 'errors'

export type HealthPenalty = {
  signal: HealthSignal
  points: number
  reason: string
}

export type HealthScore = {
  /** `undefined` when no signal could be computed */
  score: number | undefined
  /** Penalties applied, largest first */
  penalties: HealthPenalty[]
  /** Signals that could not be computed, with why */
  missing: { signal: HealthSignal; reason: string }[]
}

export const COVERAGE_MAX_PENALTY = 40
export const DEBT_MAX_PENALTY = 20
export const FIXES_MAX_PENALTY = 15
export const ERRORS_MAX_PENALTY = 25

/** Minimum number of commits before the share of fixes is meaningful */
export const MIN_COMMITS_FOR_FIXES = 5
/** Lines are floored at this value when computing the TODO density */
const DEBT_LINES_FLOOR = 1000

const ERROR_METRIC_PATTERN = /error|bug|crash|exception|incident|issue/i

/** Whether an imported metric counts as errors or bugs */
export function isErrorMetric(name: string): boolean {
  return ERROR_METRIC_PATTERN.test(name)
}

/** Imported numeric metrics counted as errors or bugs, with their value */
export function errorMetrics(feature: Feature): [string, number][] {
  return Object.entries(feature.external ?? {})
    .filter(
      (entry): entry is [string, number] =>
        typeof entry[1] === 'number' &&
        Number.isFinite(entry[1]) &&
        isErrorMetric(entry[0]),
    )
    .sort(([a], [b]) => a.localeCompare(b))
}

const clamp = (value: number, min: number, max: number) =>
  Math.min(max, Math.max(min, value))

export function computeHealthScore(feature: Feature): HealthScore {
  const penalties: HealthPenalty[] = []
  const missing: HealthScore['missing'] = []
  let signals = 0

  // Coverage
  const coverage = feature.stats?.coverage?.line_coverage_percent
  if (typeof coverage === 'number' && Number.isFinite(coverage)) {
    signals++
    const points = Math.round(
      clamp(0.4 * (100 - coverage), 0, COVERAGE_MAX_PENALTY),
    )
    if (points > 0) {
      penalties.push({
        signal: 'coverage',
        points,
        reason: `Line coverage ${Math.round(coverage)}%`,
      })
    }
  } else {
    missing.push({ signal: 'coverage', reason: 'No coverage data' })
  }

  // Technical debt (TODOs)
  const lines = feature.stats?.lines_count ?? 0
  const todos = feature.stats?.todos_count ?? 0
  if (lines > 0) {
    signals++
    const density = (todos * 1000) / Math.max(lines, DEBT_LINES_FLOOR)
    const points = Math.round(clamp(4 * density, 0, DEBT_MAX_PENALTY))
    if (points > 0) {
      penalties.push({
        signal: 'debt',
        points,
        reason: `${todos} TODO${todos === 1 ? '' : 's'} in ${lines.toLocaleString('en-US')} lines`,
      })
    }
  } else {
    missing.push({ signal: 'debt', reason: 'No lines of code' })
  }

  // Fix-heavy history
  const commits = feature.stats?.commits?.total_commits ?? 0
  const fixes = feature.stats?.commits?.count_by_type?.fix ?? 0
  if (commits >= MIN_COMMITS_FOR_FIXES) {
    signals++
    const share = Math.min(1, fixes / commits)
    const points = Math.round(clamp(30 * share, 0, FIXES_MAX_PENALTY))
    if (points > 0) {
      penalties.push({
        signal: 'fixes',
        points,
        reason: `${Math.round(share * 100)}% of ${commits} commits are fixes`,
      })
    }
  } else {
    missing.push({
      signal: 'fixes',
      reason:
        commits === 0
          ? 'No git history'
          : `Fewer than ${MIN_COMMITS_FOR_FIXES} commits`,
    })
  }

  // Imported errors and bugs
  const metrics = errorMetrics(feature)
  if (metrics.length > 0) {
    signals++
    let total = 0
    const parts: string[] = []
    for (const [name, value] of metrics) {
      total += 8 * Math.log10(1 + Math.max(0, value))
      parts.push(`${name}: ${value.toLocaleString('en-US')}`)
    }
    const points = Math.round(clamp(total, 0, ERRORS_MAX_PENALTY))
    if (points > 0) {
      penalties.push({ signal: 'errors', points, reason: parts.join(', ') })
    }
  } else {
    missing.push({ signal: 'errors', reason: 'No imported error or bug data' })
  }

  penalties.sort((a, b) => b.points - a.points)

  if (signals === 0) {
    return { score: undefined, penalties, missing }
  }

  const total = penalties.reduce((sum, penalty) => sum + penalty.points, 0)
  return { score: clamp(100 - total, 0, 100), penalties, missing }
}
