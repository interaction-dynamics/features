import { ChevronRight, Lightbulb } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import {
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { MarkdownRenderer } from "./markown-renderer";

interface FeatureDecisionsProps {
	decisions: string[];
}

export function FeatureDecisions({ decisions }: FeatureDecisionsProps) {
	if (decisions.length === 0) {
		return (
			<Card>
				<CardContent>
					<p className="text-sm text-muted-foreground">
						No decisions available
					</p>
				</CardContent>
			</Card>
		);
	}

	const getPreviewText = (decision: string) => {
		// Extract the first line as the title
		const lines = decision.split("\n");
		const firstLine = lines[0].trim();

		if (!firstLine) {
			return "Untitled Decision";
		}

		// Remove markdown formatting
		return firstLine
			.replace(/^#+\s*/, "") // Remove heading markers (# ## ###)
			.replace(/\*\*(.*?)\*\*/g, "$1") // Remove bold **text**
			.replace(/\*(.*?)\*/g, "$1") // Remove italic *text*
			.replace(/`(.*?)`/g, "$1") // Remove inline code `text`
			.replace(/\[(.*?)\]\(.*?\)/g, "$1") // Remove links [text](url)
			.replace(/~~(.*?)~~/g, "$1") // Remove strikethrough ~~text~~
			.trim();
	};

	return (
		<div className="space-y-4">
			{decisions.map((decision, index) => (
				<Card key={`decision-${index}-${decision.substring(0, 20)}`}>
					<Collapsible>
						<CollapsibleTrigger className="w-full group">
							<CardContent className="transition-colors">
								<div className="flex items-start gap-3">
									<Lightbulb className="h-5 w-5 text-primary flex-shrink-0 mt-0.5" />
									<div className="flex-1 text-left">
										<p className="text-sm leading-relaxed">
											{getPreviewText(decision)}
										</p>
									</div>
									<ChevronRight className="h-4 w-4 text-muted-foreground transition-transform duration-200 group-data-[state=open]:rotate-90 flex-shrink-0 mt-0.5" />
								</div>
							</CardContent>
						</CollapsibleTrigger>
						<CollapsibleContent>
							<CardContent className="pt-3 pb-1">
								<MarkdownRenderer markdown={decision} />
							</CardContent>
						</CollapsibleContent>
					</Collapsible>
				</Card>
			))}
		</div>
	);
}
