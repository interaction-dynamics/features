import { z } from 'zod'

export type Change = {
  title: string
  author_name: string
  author_email: string
  description: string
  date: string
  hash: string
}

export type FileCoverageStats = {
  lines_total: number
  lines_covered: number
  lines_missed: number
  line_coverage_percent: number
  branches_total?: number
  branches_covered?: number
  branch_coverage_percent?: number
}

export type CoverageStats = {
  lines_total: number
  lines_covered: number
  lines_missed: number
  line_coverage_percent: number
  branches_total?: number
  branches_covered?: number
  branch_coverage_percent?: number
  files?: Record<string, FileCoverageStats>
}

export type Stats = {
  files_count?: number
  lines_count?: number
  todos_count?: number
  commits: {
    total_commits?: number
    authors_count?: Record<string, number>
    count_by_type?: Record<string, number>
    first_commit_date?: string
    last_commit_date?: string
  }
  coverage?: CoverageStats
}

export type Feature = {
  name: string
  path: string
  owner: string
  is_owner_inherited: boolean
  description?: string
  features?: Feature[]
  meta?: Record<string, unknown>
  parent?: Feature
  changes: Change[]
  decisions: string[]
  stats?: Stats
}

export const ChangeSchema: z.ZodType<Change> = z.object({
  title: z.string(),
  author_name: z.string(),
  author_email: z.string(),
  description: z.string(),
  date: z.string(),
  hash: z.string(),
})

export const FileCoverageStatsSchema: z.ZodType<FileCoverageStats> = z.object({
  lines_total: z.number(),
  lines_covered: z.number(),
  lines_missed: z.number(),
  line_coverage_percent: z.number(),
  branches_total: z.number().optional(),
  branches_covered: z.number().optional(),
  branch_coverage_percent: z.number().optional(),
})

export const CoverageStatsSchema: z.ZodType<CoverageStats> = z.object({
  lines_total: z.number(),
  lines_covered: z.number(),
  lines_missed: z.number(),
  line_coverage_percent: z.number(),
  branches_total: z.number().optional(),
  branches_covered: z.number().optional(),
  branch_coverage_percent: z.number().optional(),
  files: z.record(z.string(), FileCoverageStatsSchema).optional(),
})

export const StatsSchema: z.ZodType<Stats> = z.object({
  files_count: z.number().optional(),
  lines_count: z.number().optional(),
  todos_count: z.number().optional(),
  commits: z.object({
    total_commits: z.number().optional(),
    authors_count: z.record(z.string(), z.number()).optional(),
    count_by_type: z.record(z.string(), z.number()).optional(),
    first_commit_date: z.string().optional(),
    last_commit_date: z.string().optional(),
  }),
  coverage: CoverageStatsSchema.optional(),
})

export const FeatureSchema: z.ZodType<Feature> = z.lazy(() =>
  z.object({
    name: z.string(),
    path: z.string(),
    owner: z.string(),
    is_owner_inherited: z.boolean(),
    description: z.string().optional(),
    features: z.array(FeatureSchema).optional(),
    meta: z.record(z.string(), z.unknown()).optional(),
    parent: FeatureSchema.optional(),
    changes: z.array(ChangeSchema),
    decisions: z.array(z.string()),
    stats: StatsSchema.optional(),
  }),
)
