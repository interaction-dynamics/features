import type { Feature, Stats } from '@/models/feature'

type FeatureOptions = {
  coverage?: number
  lines?: number
  todos?: number
  commits?: number
  fixes?: number
  external?: Feature['external']
}

/** Minimal feature with only the stats the insights need */
export function makeFeature(
  name: string,
  { coverage, lines, todos, commits, fixes, external }: FeatureOptions = {},
): Feature {
  const stats: Stats = {
    lines_count: lines,
    todos_count: todos,
    commits: {
      total_commits: commits,
      count_by_type: fixes === undefined ? undefined : { fix: fixes },
    },
  }
  if (coverage !== undefined) {
    stats.coverage = {
      lines_total: 100,
      lines_covered: coverage,
      lines_missed: 100 - coverage,
      line_coverage_percent: coverage,
    }
  }
  return {
    name,
    path: `src/features/${name}`,
    owner: '',
    is_owner_inherited: false,
    changes: [],
    decisions: [],
    dependencies: [],
    stats,
    external,
  }
}
