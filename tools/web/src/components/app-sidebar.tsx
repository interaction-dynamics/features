import { GalleryVerticalEnd } from "lucide-react";
import type * as React from "react";
import { useContext } from "react";
import { FeaturesContext } from "@/components/features-provider";
import {
	Sidebar,
	SidebarContent,
	SidebarGroup,
	SidebarHeader,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
	SidebarMenuSub,
	SidebarMenuSubButton,
	SidebarMenuSubItem,
	SidebarRail,
} from "@/components/ui/sidebar";
import type { Feature } from "@/models/feature";


interface FeatureItemProps {
  feature: Feature;
  onFeatureClick: (feature: Feature) => void;
}

function FeatureItem({ feature, onFeatureClick }: FeatureItemProps) {
	return (
		<SidebarMenuItem key={feature.path}>
			<SidebarMenuButton asChild>
				<a href={`#${feature.path}`} className="font-medium" onClick={() => onFeatureClick(feature)}>
					{feature.name}
				</a>
			</SidebarMenuButton>
			{feature.features && feature.features.length > 0 ? (
				<SidebarMenuSub>
					{feature.features.map((subFeature) => (
						<FeatureSubItem key={subFeature.path} feature={subFeature} onFeatureClick={onFeatureClick} />
					))}
				</SidebarMenuSub>
			) : null}
		</SidebarMenuItem>
	);
}

interface FeatureItemProps {
  feature: Feature;
  onFeatureClick: (feature: Feature) => void;
}

function FeatureSubItem({ feature, onFeatureClick }: FeatureItemProps) {
	return (
		<SidebarMenuSubItem key={feature.path}>
			<SidebarMenuSubButton asChild>
				<a href={`#${feature.path}`} onClick={() => onFeatureClick(feature)}>{feature.name}</a>
			</SidebarMenuSubButton>
			{feature.features && feature.features.length > 0 ? (
				<SidebarMenuSub>
					{feature.features.map((subFeature) => (
						<FeatureSubItem key={subFeature.path} feature={subFeature} onFeatureClick={onFeatureClick} />
					))}
				</SidebarMenuSub>
			) : null}
		</SidebarMenuSubItem>
	);
}

interface AppSidebarProps extends  React.ComponentProps<typeof Sidebar>  {
  onFeatureClick: (feature: Feature) => void;

}

export function AppSidebar({ onFeatureClick, ...props }:AppSidebarProps) {
	const features = useContext(FeaturesContext);

	return (
		<Sidebar {...props}>
			<SidebarHeader>
				<SidebarMenu>
					<SidebarMenuItem>
						<SidebarMenuButton size="lg" asChild>
							<a href="./">
								<div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
									<GalleryVerticalEnd className="size-4" />
								</div>
								<div className="flex flex-col gap-0.5 leading-none">
									<span className="font-medium">Features</span>
									{/*<span className="">v1.0.0</span>*/}
								</div>
							</a>
						</SidebarMenuButton>
					</SidebarMenuItem>
				</SidebarMenu>
			</SidebarHeader>
			<SidebarContent>
				<SidebarGroup>
					<SidebarMenu>
						{features.map((feature) => (
							<FeatureItem key={feature.path} feature={feature} onFeatureClick={onFeatureClick} />
						))}
					</SidebarMenu>
				</SidebarGroup>
			</SidebarContent>
			<SidebarRail />
		</Sidebar>
	);
}
