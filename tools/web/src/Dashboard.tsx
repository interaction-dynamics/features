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

export default function Dashboard() {

  const [currentFeature, setCurrentFeature] = useState<Feature|null>(null);

  console.log('currentFeature', currentFeature)

	return (
		<FeaturesProvider>
			<SidebarProvider>
				<AppSidebar onFeatureClick={setCurrentFeature} />
				<SidebarInset>
					<header className="flex h-16 shrink-0 items-center gap-2 border-b">
						<div className="flex items-center gap-2 px-3">
							<SidebarTrigger />
							<Separator orientation="vertical" className="mr-2 h-4" />
							<FeatureBreadcrumb feature={currentFeature} onFeatureClick={setCurrentFeature} />
						</div>
					</header>
					<div className="flex flex-1 flex-col gap-4 p-4">
						<div className="grid auto-rows-min gap-4 md:grid-cols-3">
							<div className="bg-muted/50 aspect-video rounded-xl" />
							<div className="bg-muted/50 aspect-video rounded-xl" />
							<div className="bg-muted/50 aspect-video rounded-xl" />
						</div>
						<div className="bg-muted/50 min-h-[100vh] flex-1 rounded-xl md:min-h-min" />
					</div>
				</SidebarInset>
			</SidebarProvider>
		</FeaturesProvider>
	);
}
