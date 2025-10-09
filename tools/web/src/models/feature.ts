import { z } from "zod";

export type Change = {
	title: string;
	author_name: string;
	author_email: string;
	description: string;
	date: string;
	hash: string;
};

export type Feature = {
	name: string;
	path: string;
	owner: string;
	description?: string;
	features?: Feature[];
	meta?: Record<string, unknown>;
	parent?: Feature;
	changes: Change[];
};

export const ChangeSchema: z.ZodType<Change> = z.object({
	title: z.string(),
	author_name: z.string(),
	author_email: z.string(),
	description: z.string(),
	date: z.string(),
	hash: z.string(),
});

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
	}),
);
