import { AppSidebar } from "@/components/app-sidebar";
import { FeatureBreadcrumb } from "@/components/feature-breadcrumb";
import { Separator } from "@/components/ui/separator";
import {
	SidebarInset,
	SidebarProvider,
	SidebarTrigger,
} from "@/components/ui/sidebar";
import { FeaturesProvider } from "./components/features-provider";
import { useState } from "react";
import type { Feature } from "./models/feature";
import { FeatureDetails } from "./components/feature-details";
import { ThemeProvider } from "@/components/theme-provider"

export default function Dashboard() {

  const [currentFeature, setCurrentFeature] = useState<Feature|null>(null);

	return (
	  <ThemeProvider>
		<FeaturesProvider>
			<SidebarProvider>
				<AppSidebar activeFeature={currentFeature} onFeatureClick={setCurrentFeature} />
				<SidebarInset>
					<header className="flex h-16 shrink-0 items-center gap-2 border-b">
						<div className="flex items-center gap-2 px-3">
							<SidebarTrigger />
							<Separator orientation="vertical" className="mr-2 h-4" />
							<FeatureBreadcrumb feature={currentFeature} onFeatureClick={setCurrentFeature} />
						</div>
					</header>
					<div className="flex flex-1 flex-col gap-4 p-4">
						{currentFeature === null ? (
							<div className="flex flex-1 items-center justify-center">
								<p className="text-muted-foreground text-lg">Select a feature</p>
							</div>
						) : (
							<FeatureDetails feature={currentFeature} />
						)}
					</div>
				</SidebarInset>
			</SidebarProvider>
		</FeaturesProvider>
	</ThemeProvider>
	);
}
