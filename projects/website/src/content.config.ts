import { glob } from 'astro/loaders'
import { z } from 'astro/zod'
import { defineCollection } from 'astro:content'

/** Pages of the technical documentation, served under /docs. */
const docs = defineCollection({
  loader: glob({ pattern: '*.md', base: './src/content/docs' }),
  schema: z.object({
    /** Page title, also used in the <title> tag. */
    title: z.string(),
    /** Meta description and lead paragraph under the title. */
    description: z.string(),
  }),
})

export const collections = { docs }
