import type { Change, Feature } from '@/models/feature'

/** Conventional commit types, same list as the CLI (`file_scanner/history.rs`) */
export const COMMIT_TYPES = [
  'feat',
  'fix',
  'docs',
  'style',
  'refactor',
  'perf',
  'test',
  'build',
  'ci',
  'chore',
  'revert',
] as const

export type CommitType = (typeof COMMIT_TYPES)[number] | 'other'

/** A commit of the project, with the features it touched */
export type HistoryCommit = {
  hash: string
  title: string
  description: string
  author_name: string
  author_email: string
  /** `YYYY-MM-DD HH:MM:SS`, as written by the CLI */
  date: string
  type: CommitType
  /** Deepest features listing this commit (see `buildHistory`) */
  features: Feature[]
  /** Features whose oldest commit is this one */
  introduces: Feature[]
}

export type HistoryMonth = {
  /** `YYYY-MM` */
  key: string
  commits: HistoryCommit[]
}

export type HistoryFilterState = {
  query: string
  /** Path of a feature: keeps commits of this feature and its sub-features */
  featurePath: string | null
  type: CommitType | null
}

export const EMPTY_FILTERS: HistoryFilterState = {
  query: '',
  featurePath: null,
  type: null,
}

/** Type of a conventional commit title (`feat(scope): ...`), `other` otherwise */
export function commitType(title: string): CommitType {
  const colon = title.indexOf(':')
  if (colon === -1) return 'other'

  const prefix = title
    .slice(0, colon)
    .replace(/\(.*$/, '')
    .replace(/!$/, '')
    .trim()
    .toLowerCase()

  return (COMMIT_TYPES as readonly string[]).includes(prefix)
    ? (prefix as CommitType)
    : 'other'
}

/** Every feature of the tree, parents before their sub-features */
export function flattenFeatures(features: Feature[]): Feature[] {
  const flattened: Feature[] = []
  const visit = (items: Feature[]) => {
    for (const item of items) {
      flattened.push(item)
      if (item.features) visit(item.features)
    }
  }
  visit(features)
  return flattened
}

function descendants(feature: Feature): Feature[] {
  return flattenFeatures(feature.features ?? [])
}

/** Oldest change of a list; the CLI date format sorts as a string */
function oldestChange(changes: Change[]): Change | undefined {
  let oldest: Change | undefined
  for (const change of changes) {
    if (!oldest || change.date < oldest.date) oldest = change
  }
  return oldest
}

/**
 * Builds the project history from the `changes` of every feature.
 *
 * Commits are deduplicated by hash and sorted newest first. A sub-feature's
 * commits are also listed by its parents, so a commit is attributed to the
 * deepest features listing it: a feature is left out when one of its
 * sub-features lists the commit too.
 */
export function buildHistory(features: Feature[]): HistoryCommit[] {
  const commits = new Map<string, HistoryCommit>()
  const listedBy = new Map<string, Set<string>>()

  for (const feature of flattenFeatures(features)) {
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
        listedBy.set(change.hash, new Set())
      }
      listedBy.get(change.hash)?.add(feature.path)
      commit.features.push(feature)
    }

    const oldest = oldestChange(feature.changes)
    if (oldest) commits.get(oldest.hash)?.introduces.push(feature)
  }

  const descendantPaths = new Map<string, string[]>()
  for (const feature of flattenFeatures(features)) {
    descendantPaths.set(
      feature.path,
      descendants(feature).map((d) => d.path),
    )
  }

  for (const commit of commits.values()) {
    const paths = listedBy.get(commit.hash) ?? new Set<string>()
    commit.features = commit.features.filter(
      (feature) =>
        !(descendantPaths.get(feature.path) ?? []).some((d) => paths.has(d)),
    )
  }

  return [...commits.values()].sort((a, b) =>
    a.date === b.date ? 0 : a.date < b.date ? 1 : -1,
  )
}

/** Paths of a feature and all its sub-features */
export function featureScope(features: Feature[], path: string): Set<string> {
  const feature = flattenFeatures(features).find((f) => f.path === path)
  if (!feature) return new Set([path])
  return new Set([feature, ...descendants(feature)].map((f) => f.path))
}

export function filterHistory(
  commits: HistoryCommit[],
  filters: HistoryFilterState,
  features: Feature[],
): HistoryCommit[] {
  const query = filters.query.trim().toLowerCase()
  const scope = filters.featurePath
    ? featureScope(features, filters.featurePath)
    : null

  return commits.filter((commit) => {
    if (filters.type && commit.type !== filters.type) return false
    if (scope && !commit.features.some((f) => scope.has(f.path))) return false
    if (!query) return true
    return [
      commit.title,
      commit.description,
      commit.author_name,
      commit.hash,
    ].some((value) => value.toLowerCase().includes(query))
  })
}

/** `YYYY-MM` of a CLI date */
export function monthKey(date: string): string {
  return date.slice(0, 7)
}

/** Groups commits (already sorted newest first) by month */
export function groupByMonth(commits: HistoryCommit[]): HistoryMonth[] {
  const months: HistoryMonth[] = []
  for (const commit of commits) {
    const key = monthKey(commit.date)
    const last = months.at(-1)
    if (last?.key === key) {
      last.commits.push(commit)
    } else {
      months.push({ key, commits: [commit] })
    }
  }
  return months
}

function nextMonth(key: string): string {
  const [year, month] = key.split('-').map(Number)
  return month === 12
    ? `${year + 1}-01`
    : `${year}-${String(month + 1).padStart(2, '0')}`
}

/** Commits per month, oldest first, with empty months filled in */
export function monthlyActivity(
  commits: HistoryCommit[],
): { month: string; commits: number }[] {
  if (commits.length === 0) return []

  const counts = new Map<string, number>()
  for (const commit of commits) {
    const key = monthKey(commit.date)
    counts.set(key, (counts.get(key) ?? 0) + 1)
  }

  const keys = [...counts.keys()].sort()
  const last = keys[keys.length - 1]
  const activity: { month: string; commits: number }[] = []
  for (let key = keys[0]; key <= last; key = nextMonth(key)) {
    activity.push({ month: key, commits: counts.get(key) ?? 0 })
  }
  return activity
}

/** `2025-03` → `March 2025` */
export function formatMonth(key: string, style: 'long' | 'short' = 'long') {
  const [year, month] = key.split('-').map(Number)
  return new Date(Date.UTC(year, month - 1, 1)).toLocaleDateString('en-US', {
    month: style,
    year: style === 'long' ? 'numeric' : '2-digit',
    timeZone: 'UTC',
  })
}

export type HistorySummary = {
  commits: number
  introduced: number
  contributors: number
  first?: string
  last?: string
}

/**
 * @param scope paths of the features to count introductions for (all when
 * missing), so that a feature filter does not count unrelated features
 */
export function summarizeHistory(
  commits: HistoryCommit[],
  scope?: Set<string>,
): HistorySummary {
  const contributors = new Set(
    commits.map((c) => c.author_email || c.author_name),
  )
  const introduced = commits.reduce(
    (n, c) =>
      n + c.introduces.filter((f) => !scope || scope.has(f.path)).length,
    0,
  )
  return {
    commits: commits.length,
    introduced,
    contributors: contributors.size,
    first: commits.at(-1)?.date,
    last: commits[0]?.date,
  }
}
