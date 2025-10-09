import { ChevronRight, Lightbulb } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import {
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
} from "@/components/ui/collapsible";

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

	const getPreviewText = (decision: string, maxLength: number = 120) => {
		// Remove extra whitespace and newlines for preview
		const cleanText = decision.replace(/\s+/g, " ").trim();
		if (cleanText.length <= maxLength) {
			return cleanText;
		}
		// Find the last space before maxLength to avoid cutting words
		const lastSpace = cleanText.lastIndexOf(" ", maxLength);
		const cutPoint = lastSpace > maxLength * 0.8 ? lastSpace : maxLength;
		return `${cleanText.substring(0, cutPoint).trim()}...`;
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
							<CardContent className="pt-0 pb-6">
								<div className="flex items-start gap-3 mt-4">
									<div className="w-5 flex-shrink-0" />
									<div className="flex-1">
										<p className="text-sm whitespace-pre-wrap leading-relaxed">
											{decision}
										</p>
									</div>
								</div>
							</CardContent>
						</CollapsibleContent>
					</Collapsible>
				</Card>
			))}
		</div>
	);
}
