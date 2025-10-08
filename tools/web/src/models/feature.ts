import { z } from "zod";

export type Feature = {
	name: string;
	path: string;
	owner: string;
	description?: string;
	features?: Feature[];
	meta?: Record<string, any>;
	parent?: Feature;
};

export const FeatureSchema: z.ZodType<Feature> = z.lazy(() =>
	z.object({
		name: z.string(),
		path: z.string(),
		owner: z.string(),
		description: z.string().optional(),
		features: z.array(FeatureSchema).optional(),
		meta: z.record(z.string(), z.unknown()).optional(),
		parent: FeatureSchema.optional(),
	}),
);
