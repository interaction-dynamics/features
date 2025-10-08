import ReactMarkdown from 'react-markdown';

import type { Feature } from "@/models/feature";
import { formatFeatureName } from "@/lib/format-feature-name";

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
						<p className="text-sm"><div className="**:list-disc **:[ul]:ms-4 **:[p]:py-2 **:[h1]:scroll-m-20 **:[h1]:text-4xl **:[h1]:font-extrabold **:[h1]:tracking-tight **:[h1]:text-balance **:[h2]:scroll-m-20 **:[h2]:pb-2 **:[h2]:text-3xl **:[h2]:font-semibold **:[h2]:tracking-tight first:mt-0 ">
						  <ReactMarkdown >{feature.description}</ReactMarkdown>
						</div>
						</p>
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
