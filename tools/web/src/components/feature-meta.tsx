import { Info } from "lucide-react";
import { MetaValue } from "./meta-value";

interface FeatureMetaProps {
	meta?: Record<string, unknown>;
}

export function FeatureMeta({ meta }: FeatureMetaProps) {
	if (!meta || Object.keys(meta).length === 0) {
		return null;
	}

	return (
		<div className="flex items-start gap-3">
			<Info className="h-4 w-4 text-muted-foreground mt-0.5" />
			<div className="flex-1">
				<p className="text-sm font-medium text-foreground mb-1">Meta</p>
				<div className="text-xs font-mono text-muted-foreground flex flex-wrap gap-2">
					{Object.entries(meta).map(([key, value]) => (
						<MetaValue key={key} metaKey={key} value={value} />
					))}
				</div>
			</div>
		</div>
	);
}
