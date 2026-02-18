import { AlertTriangle } from "lucide-react";
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from "@/components/ui/tooltip";
import {
	buildDependencyMap,
	buildNameToPathMap,
	detectAlerts,
	groupDependencies,
} from "@/features/dependencies/utils";
import type { Feature } from "@/models/feature";

interface DependenciesCellProps {
	feature: Feature;
	allFeatures: Feature[];
}

// Get unique feature dependencies count
function getUniqueDependenciesCount(feature: Feature): number {
	return new Set(feature.dependencies.map((dep) => dep.feature)).size;
}

// Get dependency alerts for a feature
function getDependencyAlerts(
	feature: Feature,
	allFeatures: Feature[],
): string[] {
	if (feature.dependencies.length === 0) return [];

	const dependencyMap = buildDependencyMap(allFeatures);
	const nameToPath = buildNameToPathMap(allFeatures);
	const groupedDeps = groupDependencies(feature.dependencies);
	const allAlerts = new Set<string>();

	groupedDeps.forEach((group) => {
		const alerts = detectAlerts(group, feature.path, dependencyMap, nameToPath);
		alerts.forEach((alert) => {
			allAlerts.add(alert);
		});
	});

	return Array.from(allAlerts);
}

export function DependenciesCell({
	feature,
	allFeatures,
}: DependenciesCellProps) {
	const count = getUniqueDependenciesCount(feature);
	const alerts = getDependencyAlerts(feature, allFeatures);
	const hasAlerts = alerts.length > 0;

	if (count === 0) {
		return <span className="text-muted-foreground/50">0</span>;
	}

	// Build tooltip content with feature dependencies and their alerts
	const dependencyMap = buildDependencyMap(allFeatures);
	const nameToPath = buildNameToPathMap(allFeatures);
	const groupedDeps = groupDependencies(feature.dependencies);
	const featureDepsWithAlerts = groupedDeps.map((group) => {
		const groupAlerts = detectAlerts(
			group,
			feature.path,
			dependencyMap,
			nameToPath,
		);
		return {
			feature: group.feature,
			alerts: groupAlerts,
		};
	});

	return (
		<Tooltip>
			<TooltipTrigger className="inline-flex items-center justify-end gap-1">
				{hasAlerts && <AlertTriangle className="h-3 w-3 text-orange-500" />}
				{count}
			</TooltipTrigger>
			<TooltipContent className="max-w-md">
				<div className="space-y-1">
					<p className="font-semibold mb-2">Feature Dependencies</p>
					{featureDepsWithAlerts.map((dep) => (
						<p key={dep.feature} className="text-xs">
							{dep.feature}
							{dep.alerts.length > 0 && (
								<span className="text-orange-500 font-semibold">
									{" "}
									({dep.alerts.join(", ")})
								</span>
							)}
						</p>
					))}
				</div>
			</TooltipContent>
		</Tooltip>
	);
}
