import type { Dependency, Feature } from "@/models/feature";

export interface GroupedDependency {
	feature: string;
	type: "parent" | "child" | "sibling";
	count: number;
	items: Dependency[];
}

/**
 * Build a map of feature names to their dependencies (recursively)
 */
export function buildDependencyMap(
	features: Feature[],
): Map<string, Set<string>> {
	const map = new Map<string, Set<string>>();

	const collectDeps = (feature: Feature) => {
		const deps = new Set<string>();
		feature.dependencies.forEach((dep) => {
			deps.add(dep.feature);
		});
		map.set(feature.name, deps);

		// Recursively collect from nested features
		feature.features?.forEach((nested) => {
			collectDeps(nested);
		});
	};

	features.forEach((feature) => {
		collectDeps(feature);
	});
	return map;
}

/**
 * Group dependencies by feature and type
 */
export function groupDependencies(
	dependencies: Dependency[],
): GroupedDependency[] {
	const grouped = dependencies.reduce<Record<string, GroupedDependency>>(
		(acc, dep) => {
			const key = `${dep.feature}-${dep.type}`;
			if (!acc[key]) {
				acc[key] = {
					feature: dep.feature,
					type: dep.type,
					count: 0,
					items: [],
				};
			}
			acc[key].count++;
			acc[key].items.push(dep);
			return acc;
		},
		{},
	);

	return Object.values(grouped);
}

/**
 * Detect alerts for a dependency group
 */
export function detectAlerts(
	group: GroupedDependency,
	currentFeatureName: string,
	dependencyMap: Map<string, Set<string>>,
): string[] {
	const alerts: string[] = [];

	// Check for circular dependency: if the feature we depend on also depends on us
	const targetDeps = dependencyMap.get(group.feature);
	if (targetDeps && targetDeps.has(currentFeatureName)) {
		alerts.push("Circular Dependency");
	}

	// Check for tight dependency (based on target files)
	const uniqueFiles = new Set(group.items.map((item) => item.targetFilename));
	const fileCount = uniqueFiles.size;

	if (fileCount === 1 && group.count > 5) {
		alerts.push("Tight Dependency");
	} else if (fileCount >= 3 && group.count > 3) {
		alerts.push("Tight Dependency");
	}

	return alerts;
}
