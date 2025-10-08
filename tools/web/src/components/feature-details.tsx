import type { Feature } from "@/models/feature";
import { formatFeatureName } from "@/utils/format-feature-name";

interface FeatureDetailsProps {
	feature: Feature;
}

export function FeatureDetails({ feature }: FeatureDetailsProps) {
	return (
		<div className="flex flex-col gap-4">
			<div>
				<h1 className="text-3xl font-bold">{formatFeatureName(feature.name)}</h1>
			</div>

			<div className="flex flex-col gap-2">
				<div>
					<span className="text-sm font-semibold text-muted-foreground">
						Path:
					</span>
					<p className="text-sm">{feature.path}</p>
				</div>

				<div>
					<span className="text-sm font-semibold text-muted-foreground">
						Owner:
					</span>
					<p className="text-sm">{feature.owner}</p>
				</div>

				{feature.description && (
					<div>
						<span className="text-sm font-semibold text-muted-foreground">
							Description:
						</span>
						<p className="text-sm">{feature.description}</p>
					</div>
				)}

				{feature.meta && Object.keys(feature.meta).length > 0 && (
					<div>
						<span className="text-sm font-semibold text-muted-foreground">
							Meta:
						</span>
						<div className="mt-1 flex flex-wrap gap-2">
							{Object.entries(feature.meta).map(([key, value]) => (
								<div
									key={key}
									className="bg-muted rounded-md px-2 py-1 text-xs"
								>
									<span className="font-semibold">{key}:</span>{" "}
									{typeof value === "object"
										? JSON.stringify(value)
										: String(value)}
								</div>
							))}
						</div>
					</div>
				)}
			</div>
		</div>
	);
}
