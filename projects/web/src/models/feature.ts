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

export type Dependency = {
  sourceFilename: string
  targetFilename: string
  line: number
  content: string
  featurePath: string
  type: 'parent' | 'child' | 'sibling'
}

export type FeatureRepository = {
  /** Name of the repository where the feature was found */
  name: string
  /** Path of the feature inside that repository */
  path: string
}

export type RecordKind = 'spec' | 'decision'

export type RecordLinks = {
  /** Related decisions (`decisions:` frontmatter, mostly on specs) */
  decisions: string[]
  /** Related specs (`spec:` / `specs:` frontmatter, mostly on decisions) */
  specs: string[]
  /** Records this one replaces */
  supersedes: string[]
  /** Records replacing this one */
  superseded_by: string[]
}

export type RecordFile = {
  name: string
  path: string
  content: string
}

/**
 * A spec or a decision record read from a `specs`, `decisions` or `adr`
 * folder. Named `FeatureRecord` to avoid shadowing TypeScript's `Record`.
 */
export type FeatureRecord = {
  kind: RecordKind
  /** File stem, or folder name for a folder record (e.g. `002-use-jwt`) */
  id: string
  title: string
  /** Lowercased status: draft, proposed, accepted, implemented, superseded... */
  status?: string
  date?: string
  /** Path of the main file, relative to the scanned folder */
  path: string
  /** Markdown of the main file, without its frontmatter */
  content: string
  /** Other markdown files of a folder record (plan.md, tasks.md...) */
  files?: RecordFile[]
  links: RecordLinks
  tasks?: { done: number; total: number }
  /** Feature a project-level record is attached to (`feature:` frontmatter) */
  feature?: string
}

/** records.json: the project-level records (`features serve` / `build`) */
export type ProjectRecords = {
  records: FeatureRecord[]
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
  /** Specs and decision records; missing in reports from older versions */
  records?: FeatureRecord[]
  stats?: Stats
  dependencies: Dependency[]
  /** Only set on aggregated reports (`features --aggregate`) */
  repositories?: FeatureRepository[]
}

export type ReconciliationOccurrence = {
  feature: string
  repository: string
  /** Path of the feature inside its repository */
  path: string
  /** Name of the parent feature in that repository */
  parent?: string
  /** Path of the unmerged feature in the aggregated report */
  featurePath: string
}

export type ReconciliationError = {
  type: 'cycle' | 'conflicting_parents'
  message: string
  features: string[]
  occurrences: ReconciliationOccurrence[]
}

/** features.json produced by `features --aggregate` */
export type AggregatedReport = {
  repositories: string[]
  features: Feature[]
  errors: ReconciliationError[]
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

export const DependencySchema: z.ZodType<Dependency> = z.object({
  sourceFilename: z.string(),
  targetFilename: z.string(),
  line: z.number(),
  content: z.string(),
  featurePath: z.string(),
  type: z.enum(['parent', 'child', 'sibling']),
})

export const RecordLinksSchema: z.ZodType<RecordLinks> = z.object({
  decisions: z.array(z.string()),
  specs: z.array(z.string()),
  supersedes: z.array(z.string()),
  superseded_by: z.array(z.string()),
})

export const RecordFileSchema: z.ZodType<RecordFile> = z.object({
  name: z.string(),
  path: z.string(),
  content: z.string(),
})

export const FeatureRecordSchema: z.ZodType<FeatureRecord> = z.object({
  kind: z.enum(['spec', 'decision']),
  id: z.string(),
  title: z.string(),
  status: z.string().optional(),
  date: z.string().optional(),
  path: z.string(),
  content: z.string(),
  files: z.array(RecordFileSchema).optional(),
  links: RecordLinksSchema,
  tasks: z.object({ done: z.number(), total: z.number() }).optional(),
  feature: z.string().optional(),
})

export const ProjectRecordsSchema: z.ZodType<ProjectRecords> = z.object({
  records: z.array(FeatureRecordSchema),
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
    records: z.array(FeatureRecordSchema).optional(),
    stats: StatsSchema.optional(),
    dependencies: z.array(DependencySchema),
    repositories: z
      .array(z.object({ name: z.string(), path: z.string() }))
      .optional(),
  }),
)
