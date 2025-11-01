import { z } from 'zod'

export type Change = {
  title: string
  author_name: string
  author_email: string
  description: string
  date: string
  hash: string
}

export type Stats = {
  commits: {
    total_commits?: number
    authors_count?: Record<string, number>
    count_by_type?: Record<string, number>
    first_commit_date?: string
    last_commit_date?: string
  }
}

export type Feature = {
  name: string
  path: string
  owner: string
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

export const StatsSchema: z.ZodType<Stats> = z.object({
  commits: z.object({
    total_commits: z.number().optional(),
    authors_count: z.record(z.string(), z.number()).optional(),
    count_by_type: z.record(z.string(), z.number()).optional(),
    first_commit_date: z.string().optional(),
    last_commit_date: z.string().optional(),
  }),
})

export const FeatureSchema: z.ZodType<Feature> = z.lazy(() =>
  z.object({
    name: z.string(),
    path: z.string(),
    owner: z.string(),
    description: z.string().optional(),
    features: z.array(FeatureSchema).optional(),
    meta: z.record(z.string(), z.unknown()).optional(),
    parent: FeatureSchema.optional(),
    changes: z.array(ChangeSchema),
    decisions: z.array(z.string()),
    stats: StatsSchema.optional(),
  }),
)
