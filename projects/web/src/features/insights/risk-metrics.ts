import { humanize } from '@/lib/external-tools'
import type { Feature } from '@/models/feature'

/** Built-in axis metrics; imported ones use `external:<name>` */
export type RiskMetricId =
  | 'untested'
  | 'todo_density'
  | 'commits'
  | 'fix_commits'
  | `external:${string}`

export type RiskMetric = {
  id: RiskMetricId
  label: string
  /** Suffix shown after values, e.g. `%` */
  unit?: string
  /** Built-in metrics are meant for one axis; imported ones fit both */
  axis: 'risk' | 'activity' | 'any'
  value: (feature: Feature) => number | undefined
}

const finite = (value: unknown): number | undefined =>
  typeof value === 'number' && Number.isFinite(value) ? value : undefined

const BUILT_IN_METRICS: RiskMetric[] = [
  {
    id: 'untested',
    label: 'Untested lines',
    unit: '%',
    axis: 'risk',
    value: (feature) => {
      const coverage = finite(feature.stats?.coverage?.line_coverage_percent)
      return coverage === undefined
        ? undefined
        : Math.min(100, Math.max(0, 100 - coverage))
    },
  },
  {
    id: 'todo_density',
    label: 'TODOs per 1,000 lines',
    axis: 'risk',
    value: (feature) => {
      const lines = feature.stats?.lines_count ?? 0
      if (lines <= 0) return undefined
      return ((feature.stats?.todos_count ?? 0) * 1000) / lines
    },
  },
  {
    id: 'commits',
    label: 'Commits',
    axis: 'activity',
    value: (feature) => finite(feature.stats?.commits?.total_commits),
  },
  {
    id: 'fix_commits',
    label: 'Fix commits',
    axis: 'activity',
    value: (feature) => {
      if (finite(feature.stats?.commits?.total_commits) === undefined) {
        return undefined
      }
      return feature.stats?.commits?.count_by_type?.fix ?? 0
    },
  },
]

/** Names of the imported metrics that are numbers for at least one feature */
export function numericExternalKeys(features: Feature[]): string[] {
  const keys = new Set<string>()
  for (const feature of features) {
    for (const [key, value] of Object.entries(feature.external ?? {})) {
      if (finite(value) !== undefined) keys.add(key)
    }
  }
  return Array.from(keys).sort()
}

function externalMetric(key: string): RiskMetric {
  return {
    id: `external:${key}`,
    label: humanize(key),
    axis: 'any',
    value: (feature) => finite(feature.external?.[key]),
  }
}

/**
 * Metrics that have a value for at least one feature, built-in first.
 * With `--skip-changes`, the commit metrics are left out.
 */
export function availableMetrics(
  features: Feature[],
  { skipChanges = false }: { skipChanges?: boolean } = {},
): RiskMetric[] {
  const builtIn = BUILT_IN_METRICS.filter((metric) => {
    if (skipChanges && metric.axis === 'activity') return false
    return features.some((feature) => metric.value(feature) !== undefined)
  })
  // Commits are 0 everywhere when the history was not read
  const hasHistory = features.some(
    (feature) => (feature.stats?.commits?.total_commits ?? 0) > 0,
  )
  return [
    ...builtIn.filter((metric) => metric.axis !== 'activity' || hasHistory),
    ...numericExternalKeys(features).map(externalMetric),
  ]
}

/** Metrics offered on the X (risk) or Y (activity) axis */
export function metricsForAxis(
  metrics: RiskMetric[],
  axis: 'risk' | 'activity',
): RiskMetric[] {
  return metrics.filter(
    (metric) => metric.axis === axis || metric.axis === 'any',
  )
}

/**
 * Default axes: untested lines (or TODO density) against commits. Falls back
 * on imported metrics, and never picks the same metric twice when another
 * one exists.
 */
export function defaultAxes(metrics: RiskMetric[]): {
  x: RiskMetric | undefined
  y: RiskMetric | undefined
} {
  const xOptions = metricsForAxis(metrics, 'risk')
  const yOptions = metricsForAxis(metrics, 'activity')
  const x = xOptions[0]
  const y = yOptions.find((metric) => metric.id !== x?.id) ?? yOptions[0]
  return { x, y }
}

export function median(values: number[]): number | undefined {
  if (values.length === 0) return undefined
  const sorted = [...values].sort((a, b) => a - b)
  const middle = Math.floor(sorted.length / 2)
  return sorted.length % 2 === 0
    ? (sorted[middle - 1] + sorted[middle]) / 2
    : sorted[middle]
}

export type RiskPoint = {
  feature: Feature
  x: number
  y: number
  /** Lines of code, used for the dot size */
  size: number
}

/** One point per feature with a value on both axes, plus how many were left out */
export function buildRiskPoints(
  features: Feature[],
  x: RiskMetric,
  y: RiskMetric,
): { points: RiskPoint[]; hidden: number } {
  const points: RiskPoint[] = []
  for (const feature of features) {
    const xValue = x.value(feature)
    const yValue = y.value(feature)
    if (xValue === undefined || yValue === undefined) continue
    points.push({
      feature,
      x: xValue,
      y: yValue,
      size: Math.max(1, feature.stats?.lines_count ?? 1),
    })
  }
  return { points, hidden: features.length - points.length }
}

export type Quadrant = 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left'

/**
 * Quadrant of a point, split at the medians. A point on a median line counts
 * as low, so "high" always means strictly above the typical feature.
 */
export function quadrantOf(
  point: { x: number; y: number },
  medianX: number,
  medianY: number,
): Quadrant {
  const high = point.y > medianY
  const risky = point.x > medianX
  if (high) return risky ? 'top-right' : 'top-left'
  return risky ? 'bottom-right' : 'bottom-left'
}

/** Corner labels, the top-right one depends on the risk metric */
export function quadrantLabels(x: RiskMetric): Record<Quadrant, string> {
  const topRight =
    x.id === 'untested'
      ? 'Test this first'
      : x.id === 'todo_density'
        ? 'Pay down debt first'
        : 'Look here first'
  return {
    'top-right': topRight,
    'top-left': 'Healthy and busy',
    'bottom-right': 'Candidate for removal',
    'bottom-left': 'Stable',
  }
}

/** `12.345` → `12.3`, `1200` → `1,200`, with the metric unit */
export function formatMetricValue(metric: RiskMetric, value: number): string {
  const rounded = Number.isInteger(value)
    ? value.toLocaleString('en-US')
    : value.toLocaleString('en-US', { maximumFractionDigits: 1 })
  return metric.unit ? `${rounded}${metric.unit}` : rounded
}

/** Short axis tick: `12,500` → `12.5K`, with the metric unit */
export function formatMetricTick(metric: RiskMetric, value: number): string {
  const short = value.toLocaleString('en-US', {
    notation: 'compact',
    maximumFractionDigits: 1,
  })
  return metric.unit ? `${short}${metric.unit}` : short
}
