"use client";

import { ChevronRight } from "lucide-react";

import {
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
} from "@/components/ui/collapsible";
import {
	SidebarGroup,
	SidebarGroupLabel,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
	SidebarMenuSub,
	SidebarMenuSubButton,
	SidebarMenuSubItem,
} from "@/components/ui/sidebar";
import { formatFeatureName } from "@/lib/format-feature-name";
import type { Feature } from "@/models/feature";

interface NavFeaturesProps {
	items: Feature[];
	onFeatureClick?: (feature: Feature) => void;
	activeFeature?: Feature | null;
}

function FeatureMenuItem({
	feature,
	onFeatureClick,
	activeFeature,
}: {
	feature: Feature;
	onFeatureClick?: (feature: Feature) => void;
	activeFeature?: Feature | null;
}) {
	const hasChildren = feature.features && feature.features.length > 0;
	const isActive = activeFeature?.path === feature.path;

	if (!hasChildren) {
		return (
			<SidebarMenuItem>
				<SidebarMenuButton
					asChild
					isActive={isActive}
					onClick={() => onFeatureClick?.(feature)}
				>
					<a href={`#${feature.path}`}>
						<span>{formatFeatureName(feature.name)}</span>
					</a>
				</SidebarMenuButton>
			</SidebarMenuItem>
		);
	}

	return (
		<Collapsible
			key={feature.path}
			asChild
			defaultOpen={isActive}
			className="group/collapsible"
		>
			<SidebarMenuItem>
				<CollapsibleTrigger asChild>
					<SidebarMenuButton
						tooltip={formatFeatureName(feature.name)}
						isActive={isActive}
						onClick={() => onFeatureClick?.(feature)}
					>
						<span>{formatFeatureName(feature.name)}</span>
						<ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
					</SidebarMenuButton>
				</CollapsibleTrigger>
				<CollapsibleContent>
					<SidebarMenuSub>
						{feature.features?.map((subFeature) => (
							<FeatureSubItem
								key={subFeature.path}
								feature={subFeature}
								onFeatureClick={onFeatureClick}
								activeFeature={activeFeature}
							/>
						))}
					</SidebarMenuSub>
				</CollapsibleContent>
			</SidebarMenuItem>
		</Collapsible>
	);
}

function FeatureSubItem({
	feature,
	onFeatureClick,
	activeFeature,
}: {
	feature: Feature;
	onFeatureClick?: (feature: Feature) => void;
	activeFeature?: Feature | null;
}) {
	const hasChildren = feature.features && feature.features.length > 0;
	const isActive = activeFeature?.path === feature.path;

	if (!hasChildren) {
		return (
			<SidebarMenuSubItem>
				<SidebarMenuSubButton
					asChild
					isActive={isActive}
					onClick={() => onFeatureClick?.(feature)}
				>
					<a href={`#${feature.path}`}>
						<span>{formatFeatureName(feature.name)}</span>
					</a>
				</SidebarMenuSubButton>
			</SidebarMenuSubItem>
		);
	}

	return (
		<Collapsible
			key={feature.path}
			asChild
			defaultOpen={isActive}
			className="group/collapsible"
		>
			<SidebarMenuSubItem>
				<CollapsibleTrigger asChild>
					<SidebarMenuSubButton
						isActive={isActive}
						onClick={() => onFeatureClick?.(feature)}
					>
						<span>{formatFeatureName(feature.name)}</span>
						<ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
					</SidebarMenuSubButton>
				</CollapsibleTrigger>
				<CollapsibleContent>
					<SidebarMenuSub>
						{feature.features?.map((subFeature) => (
							<FeatureSubItem
								key={subFeature.path}
								feature={subFeature}
								onFeatureClick={onFeatureClick}
								activeFeature={activeFeature}
							/>
						))}
					</SidebarMenuSub>
				</CollapsibleContent>
			</SidebarMenuSubItem>
		</Collapsible>
	);
}

export function NavFeatures({
	items,
	onFeatureClick,
	activeFeature,
}: NavFeaturesProps) {
	return (
		<SidebarGroup>
			<SidebarGroupLabel>Features</SidebarGroupLabel>
			<SidebarMenu>
				{items.map((item) => (
					<FeatureMenuItem
						key={item.path}
						feature={item}
						onFeatureClick={onFeatureClick}
						activeFeature={activeFeature}
					/>
				))}
			</SidebarMenu>
		</SidebarGroup>
	);
}
