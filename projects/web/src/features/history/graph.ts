import type { Change, Feature } from '@/models/feature'

/** A feature drawn as a lane of the feature graph */
export type FeatureLane = {
  feature: Feature
  /** 0 for top-level features */
  depth: number
  /** Index of the parent lane in `lanes`, -1 when it forks from the project */
  parent: number
  /** Index of the top-level lane of this branch, to color it */
  root: number
  /** Oldest commit of the feature: the one that created it */
  creation: Change
  /** Date of the newest commit of the feature */
  lastChange: string
  commits: number
  /** Sub-features shown in the graph */
  children: number
}

export type FeatureGraph = {
  /** Parents before their sub-features, siblings by creation date */
  lanes: FeatureLane[]
  /** Oldest commit date of the graph */
  start: string
  /** Newest commit date of the graph */
  end: string
  /** Features left out because they have no commit */
  withoutCommits: number
}

/** Oldest and newest change of a list; the CLI date format sorts as a string */
function bounds(changes: Change[]): [Change, Change] | undefined {
  if (changes.length === 0) return undefined
  let oldest = changes[0]
  let newest = changes[0]
  for (const change of changes) {
    if (change.date < oldest.date) oldest = change
    if (change.date > newest.date) newest = change
  }
  return [oldest, newest]
}

function countFeatures(features: Feature[]): number {
  return features.reduce((n, f) => n + 1 + countFeatures(f.features ?? []), 0)
}

/**
 * Builds the feature graph: one lane per feature, from the commit that
 * created it (its oldest commit) to its last commit, forking from its parent
 * like a git branch. Features without commits are left out with their
 * sub-features (a sub-feature's commits are also its parent's).
 */
export function buildFeatureGraph(features: Feature[]): FeatureGraph {
  const lanes: FeatureLane[] = []

  const visit = (items: Feature[], depth: number, parent: number) => {
    const dated = items
      .map((feature) => ({ feature, range: bounds(feature.changes) }))
      .filter(
        (item): item is { feature: Feature; range: [Change, Change] } =>
          item.range !== undefined,
      )
      .sort((a, b) =>
        a.range[0].date === b.range[0].date
          ? a.feature.path.localeCompare(b.feature.path)
          : a.range[0].date < b.range[0].date
            ? -1
            : 1,
      )

    for (const { feature, range } of dated) {
      const index = lanes.length
      lanes.push({
        feature,
        depth,
        parent,
        root: parent === -1 ? index : lanes[parent].root,
        creation: range[0],
        lastChange: range[1].date,
        commits: feature.changes.length,
        children: 0,
      })
      if (parent !== -1) lanes[parent].children += 1
      visit(feature.features ?? [], depth + 1, index)
    }
  }
  visit(features, 0, -1)

  let start = ''
  let end = ''
  for (const lane of lanes) {
    if (!start || lane.creation.date < start) start = lane.creation.date
    if (!end || lane.lastChange > end) end = lane.lastChange
  }

  return {
    lanes,
    start,
    end,
    withoutCommits: countFeatures(features) - lanes.length,
  }
}

/** Indexes of a lane, its ancestors and its sub-features (at any depth) */
export function lineage(lanes: FeatureLane[], index: number): Set<number> {
  const related = new Set<number>()
  for (let i = index; i !== -1; i = lanes[i].parent) related.add(i)

  // Parents come before their sub-features
  const descendants = new Set([index])
  for (let i = index + 1; i < lanes.length; i++) {
    if (!descendants.has(lanes[i].parent)) continue
    descendants.add(i)
    related.add(i)
  }
  return related
}

/** Milliseconds of a CLI date (`YYYY-MM-DD HH:MM:SS`, read as UTC) */
export function dateTime(date: string): number {
  return Date.parse(`${date.replace(' ', 'T')}Z`)
}

/**
 * First day of the months between two CLI dates, as `YYYY-MM`, keeping one
 * month every `step` so that labels don't overlap
 */
export function monthTicks(start: string, end: string, step = 1): string[] {
  if (!start || !end) return []
  const ticks: string[] = []
  let [year, month] = start.slice(0, 7).split('-').map(Number)
  const last = end.slice(0, 7)
  for (let i = 0; ; i++) {
    const key = `${year}-${String(month).padStart(2, '0')}`
    if (key > last) break
    if (i % step === 0) ticks.push(key)
    month += 1
    if (month === 13) {
      month = 1
      year += 1
    }
  }
  return ticks
}
