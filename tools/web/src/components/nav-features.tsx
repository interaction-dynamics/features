'use client'

import { ChevronRight } from 'lucide-react'

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible'
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
} from '@/components/ui/sidebar'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'
import { HelpButton } from './help-button'

interface NavFeaturesProps {
  items: Feature[]
  onFeatureClick?: (feature: Feature) => void
  activeFeature?: Feature | null
}

function FeatureMenuItem({
  feature,
  onFeatureClick,
  activeFeature,
}: {
  feature: Feature
  onFeatureClick?: (feature: Feature) => void
  activeFeature?: Feature | null
}) {
  const hasChildren = feature.features && feature.features.length > 0
  const isActive = activeFeature?.path === feature.path

  if (!hasChildren) {
    return (
      <SidebarMenuItem>
        <SidebarMenuButton
          className="cursor-pointer w-full"
          asChild
          isActive={isActive}
          onClick={() => onFeatureClick?.(feature)}
        >
          <span>{formatFeatureName(feature.name)}</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    )
  }

  return (
    <Collapsible
      key={feature.path}
      asChild
      defaultOpen={isActive || activeFeature?.path.includes(feature.path)}
      className="group/collapsible"
    >
      <SidebarMenuItem>
        <CollapsibleTrigger asChild>
          <SidebarMenuButton
            className="cursor-pointer"
            tooltip={formatFeatureName(feature.name)}
            isActive={isActive}
            onClick={() => onFeatureClick?.(feature)}
          >
            <span>{formatFeatureName(feature.name)}</span>
            <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
          </SidebarMenuButton>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <SidebarMenuSub className="me-0 pe-0">
            {feature.features?.map((subFeature) => (
              <FeatureMenuItem
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
  )
}

function NavFeaturesMenu({
  items,
  onFeatureClick,
  activeFeature,
}: NavFeaturesProps) {
  if (items.length === 0) {
    return (
      <div className="px-2 py-4">
        <div className="flex flex-col items-start gap-2">
          <p className="text-sm text-muted-foreground">No features available</p>
          <HelpButton
            title="How to add a feature"
            url="https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-feature"
          />
        </div>
      </div>
    )
  }

  return (
    <>
      {items.map((item) => (
        <FeatureMenuItem
          key={item.path}
          feature={item}
          onFeatureClick={onFeatureClick}
          activeFeature={activeFeature}
        />
      ))}
    </>
  )
}

export function NavFeatures({
  items,
  onFeatureClick,
  activeFeature,
}: NavFeaturesProps) {
  return (
    <SidebarGroup>
      <SidebarGroupLabel>All Features</SidebarGroupLabel>
      <SidebarMenu>
        <NavFeaturesMenu
          items={items}
          onFeatureClick={onFeatureClick}
          activeFeature={activeFeature}
        />
      </SidebarMenu>
    </SidebarGroup>
  )
}
