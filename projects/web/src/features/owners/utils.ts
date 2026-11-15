import {
  commitType,
  flattenFeatures,
  type HistoryCommit,
} from '@/features/history/utils'
import { featureRecords, sortRecords } from '@/features/records/utils'
import { getExternalMetricKeys } from '@/lib/external-tools'
import type {
  Change,
  ExternalValue,
  Feature,
  FeatureRecord,
} from '@/models/feature'

/** URL segment of the features without owner (`owner: ""`) */
export const UNOWNED_SEGMENT = '~unowned'

/** Path of the owner page, the owner encoded as a single URL segment */
export function ownerPath(owner: string): string {
  return `/owners/${owner === '' ? UNOWNED_SEGMENT : encodeURIComponent(owner)}`
}

/** Owner of an `/owners/:owner` route param, already decoded by the router */
export function ownerFromParam(param: string | undefined): string {
  if (param === undefined || param === UNOWNED_SEGMENT) return ''
  return param
}

export type OwnedFeature = {
  feature: Feature
  /** The owner comes from a parent feature */
  inherited: boolean
  /** Closest parent defining the owner, for inherited features */
  inheritedFrom?: Feature
}

export type OwnerCoverage = {
  linesTotal: number
  linesCovered: number
  /** `linesCovered / linesTotal` in percent, 0 without lines */
  percent: number
}

export type OwnerTotals = {
  features: number
  /** Features defining this owner in their README */
  direct: number
  /** Features inheriting this owner from a parent */
  inherited: number
  files: number
  lines: number
  /** TODO comments, the technical debt measured by the CLI */
  todos: number
  /** Unique commits of the owner's features */
  commits: number
  contributors: number
  /** Only the features with coverage data count; undefined when none has */
  coverage?: OwnerCoverage
  /** CLI date (`YYYY-MM-DD HH:MM:SS`) of the latest commit */
  lastChange?: string
}

export type ExternalMetricSummary = {
  key: string
  /** Sum of the values when they are all numbers */
  total?: number
  /** Number of features per value, most frequent first, when not all numbers */
  counts?: { value: ExternalValue; count: number }[]
  values: { feature: Feature; value: ExternalValue }[]
}

export type OwnedRecord = {
  record: FeatureRecord
  feature: Feature
}

export type OwnerSummary = {
  owner: string
  features: OwnedFeature[]
  totals: OwnerTotals
  external: ExternalMetricSummary[]
  records: OwnedRecord[]
  commits: HistoryCommit[]
}

/** Closest parent whose owner is not inherited itself */
function ownerSource(feature: Feature): Feature | undefined {
  let parent = feature.parent
  while (parent?.is_owner_inherited) parent = parent.parent
  return parent
}

/**
 * Commits of the given features, deduplicated by hash, newest first.
 *
 * The CLI lists a commit in a feature when it touches a file of the feature
 * outside its nested features, so the union of the features' changes is the
 * owner's history. `features` and `introduces` only hold the given features.
 */
export function ownerCommits(features: Feature[]): HistoryCommit[] {
  const commits = new Map<string, HistoryCommit>()

  for (const feature of features) {
    let oldest: Change | undefined
    for (const change of feature.changes) {
      let commit = commits.get(change.hash)
      if (!commit) {
        commit = {
          hash: change.hash,
          title: change.title,
          description: change.description,
          author_name: change.author_name,
          author_email: change.author_email,
          date: change.date,
          type: commitType(change.title),
          features: [],
          introduces: [],
        }
        commits.set(change.hash, commit)
      }
      if (!commit.features.includes(feature)) commit.features.push(feature)
      // The CLI date format sorts as a string
      if (!oldest || change.date < oldest.date) oldest = change
    }
    if (oldest) commits.get(oldest.hash)?.introduces.push(feature)
  }

  return [...commits.values()].sort((a, b) =>
    a.date === b.date ? 0 : a.date < b.date ? 1 : -1,
  )
}

function summarizeExternal(features: Feature[]): ExternalMetricSummary[] {
  return getExternalMetricKeys(features).map((key) => {
    const values = features.flatMap((feature) => {
      const value = feature.external?.[key]
      return value === undefined ? [] : [{ feature, value }]
    })
    if (values.every(({ value }) => typeof value === 'number')) {
      const total = values.reduce(
        (sum, { value }) => sum + (value as number),
        0,
      )
      return { key, total, values }
    }
    const counts = new Map<ExternalValue, number>()
    for (const { value } of values) {
      counts.set(value, (counts.get(value) ?? 0) + 1)
    }
    return {
      key,
      counts: [...counts.entries()]
        .map(([value, count]) => ({ value, count }))
        .sort((a, b) => b.count - a.count),
      values,
    }
  })
}

/**
 * Everything about one owner, from the feature tree.
 *
 * A feature belongs to the owner when its `owner` (inherited or not) matches
 * exactly; `""` gathers the features without owner. File, line, TODO and
 * coverage counts of the CLI exclude nested features, so they are summed
 * without double counting. Returns undefined when the owner has no feature.
 */
export function summarizeOwner(
  tree: Feature[],
  owner: string,
): OwnerSummary | undefined {
  const owned = flattenFeatures(tree).filter((f) => f.owner === owner)
  if (owned.length === 0) return undefined

  const features: OwnedFeature[] = owned.map((feature) => ({
    feature,
    inherited: feature.is_owner_inherited,
    inheritedFrom: feature.is_owner_inherited
      ? ownerSource(feature)
      : undefined,
  }))

  const commits = ownerCommits(owned)

  let linesTotal = 0
  let linesCovered = 0
  let withCoverage = 0
  for (const feature of owned) {
    const coverage = feature.stats?.coverage
    if (!coverage) continue
    withCoverage += 1
    linesTotal += coverage.lines_total
    linesCovered += coverage.lines_covered
  }

  const sum = (value: (feature: Feature) => number | undefined) =>
    owned.reduce((total, feature) => total + (value(feature) ?? 0), 0)

  const inherited = features.filter((f) => f.inherited).length

  const totals: OwnerTotals = {
    features: owned.length,
    direct: owned.length - inherited,
    inherited,
    files: sum((f) => f.stats?.files_count),
    lines: sum((f) => f.stats?.lines_count),
    todos: sum((f) => f.stats?.todos_count),
    commits: commits.length,
    contributors: new Set(commits.map((c) => c.author_email || c.author_name))
      .size,
    coverage:
      withCoverage > 0
        ? {
            linesTotal,
            linesCovered,
            percent: linesTotal > 0 ? (linesCovered / linesTotal) * 100 : 0,
          }
        : undefined,
    lastChange: commits[0]?.date,
  }

  const recordFeature = new Map<FeatureRecord, Feature>()
  for (const feature of owned) {
    for (const record of featureRecords(feature)) {
      recordFeature.set(record, feature)
    }
  }
  const records = sortRecords([...recordFeature.keys()]).map((record) => ({
    record,
    feature: recordFeature.get(record) as Feature,
  }))

  return {
    owner,
    features,
    totals,
    external: summarizeExternal(owned),
    records,
    commits,
  }
}
