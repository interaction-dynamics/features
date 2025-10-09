import { FolderTree, Info, User } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { formatFeatureName } from "@/lib/format-feature-name";
import type { Feature } from "@/models/feature";
import { FeatureChanges } from "./feature-changes";
import { FeatureDescription } from "./feature-description";

interface FeatureDetailsProps {
	feature: Feature;
}

export function FeatureDetails({ feature }: FeatureDetailsProps) {
	return (
		<div className="flex flex-col gap-4">
			<div>
				<h1 className="text-3xl font-bold">
					{formatFeatureName(feature.name)}
				</h1>
			</div>
			<div className="flex flex-col gap-3">
				<div className="flex items-start gap-3">
					<FolderTree className="h-4 w-4 text-muted-foreground mt-0.5" />
					<div className="flex-1">
						<p className="text-sm font-medium text-foreground mb-1">Path</p>
						<p className="text-xs font-mono text-muted-foreground">
							{feature.path}
						</p>
					</div>
				</div>
				<div className="flex items-start gap-3">
					<User className="h-4 w-4 text-muted-foreground mt-0.5" />
					<div className="flex-1">
						<p className="text-sm font-medium text-foreground mb-1">Owner</p>
						<p className="text-xs font-mono text-muted-foreground">
							{feature.owner}
						</p>
					</div>
				</div>
				{feature.meta && Object.keys(feature.meta).length > 0 && (
					<div className="flex items-start gap-3">
						<Info className="h-4 w-4 text-muted-foreground mt-0.5" />
						<div className="flex-1">
							<p className="text-sm font-medium text-foreground mb-1">Meta</p>
							<p className="text-xs font-mono text-muted-foreground flex flex-wrap gap-2">
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
							</p>
						</div>
					</div>
				)}
			</div>

			<Tabs defaultValue="description" className="mt-4">
				<TabsList>
					<TabsTrigger value="description">Description</TabsTrigger>
					<TabsTrigger value="changes">Changes</TabsTrigger>
					<TabsTrigger value="files">Files</TabsTrigger>
					<TabsTrigger value="dependencies">Dependencies</TabsTrigger>
				</TabsList>
				<TabsContent value="description" className="mt-1">
					<FeatureDescription description={feature.description} />
				</TabsContent>
				<TabsContent value="changes" className="mt-1">
					<FeatureChanges changes={feature.changes} />
				</TabsContent>
				<TabsContent value="files" className="mt-1">
					<p className="text-sm text-muted-foreground">
						Files list coming soon...
					</p>
				</TabsContent>
				<TabsContent value="dependencies" className="mt-1">
					<p className="text-sm text-muted-foreground">
						Dependencies coming soon...
					</p>
				</TabsContent>
			</Tabs>
		</div>
	);
}
