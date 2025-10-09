import { MarkdownRenderer } from "./markown-renderer";
import { Card, CardContent } from "./ui/card";

interface FeatureDescriptionProps {
	description?: string;
}

export function FeatureDescription({ description }: FeatureDescriptionProps) {
	if (!description) {
		return (
			<Card>
				<CardContent>
					<p className="text-sm text-muted-foreground">
						No description available
					</p>
				</CardContent>
			</Card>
		);
	}

	return (
		<Card>
			<CardContent>
				<MarkdownRenderer markdown={description} />
			</CardContent>
		</Card>
	);
}
