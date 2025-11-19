import { BarChart3 } from 'lucide-react'
import type * as React from 'react'
import { useContext, useMemo, useState } from 'react'
import { Link, NavLink } from 'react-router'

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from '@/components/ui/sidebar'
import { FeaturesContext } from '@/lib/features-context'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'
import { AppSidebarSearchInput } from './app-sidebar-search-input'
import { ModeToggle } from './mode-toggle'
import { NavFeatures } from './nav-features'
import { VersionIndicator } from './version-indicator'

interface AppSidebarProps extends React.ComponentProps<typeof Sidebar> {
  activeFeature: Feature | null
  onFeatureClick: (feature: Feature) => void
}

// Recursively flatten all features into a single array
function flattenFeatures(features: Feature[]): Feature[] {
  const flattened: Feature[] = []

  function flatten(items: Feature[]) {
    for (const item of items) {
      flattened.push(item)
      if (item.features && item.features.length > 0) {
        flatten(item.features)
      }
    }
  }

  flatten(features)
  return flattened
}

export function AppSidebar({
  activeFeature,
  onFeatureClick,
  ...props
}: AppSidebarProps) {
  const { features } = useContext(FeaturesContext)
  const [searchQuery, setSearchQuery] = useState('')

  // Filter features based on search query
  const filteredFeatures = useMemo(() => {
    if (!searchQuery.trim()) {
      return features
    }

    const query = searchQuery.toLowerCase().trim()
    const flatFeatures = flattenFeatures(features)

    return flatFeatures.filter(
      (feature) =>
        feature.name.toLowerCase().includes(query) ||
        feature.owner.toLowerCase().includes(query) ||
        feature.description?.toLowerCase().includes(query),
    )
  }, [features, searchQuery])

  const isSearching = searchQuery.trim().length > 0

  return (
    <Sidebar {...props}>
      <SidebarHeader className="relative">
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <Link to={'/'}>
                <img
                  src="./feature-icon.svg"
                  alt="Feature Icon"
                  className="size-8"
                />
                <div className="flex flex-col gap-0.5 leading-none">
                  <span className="font-medium">Features</span>
                </div>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
        {features.length > 0 && (
          <div className="px-2 pb-2">
            <AppSidebarSearchInput
              value={searchQuery}
              onChange={setSearchQuery}
            />
          </div>
        )}
        <div className="h-4 absolute bottom-[-1rem] z-50 inset-x-0 bg-gradient-to-b from-sidebar  to-transparent" />
      </SidebarHeader>
      <SidebarContent>
        {isSearching ? (
          <SidebarGroup>
            <SidebarGroupLabel>
              Search Results ({filteredFeatures.length})
            </SidebarGroupLabel>
            <SidebarMenu>
              {filteredFeatures.length === 0 ? (
                <div className="px-2 py-4">
                  <p className="text-sm text-muted-foreground">
                    No features found matching "{searchQuery}"
                  </p>
                </div>
              ) : (
                filteredFeatures.map((feature) => (
                  <SidebarMenuItem key={feature.path}>
                    <SidebarMenuButton
                      asChild
                      isActive={activeFeature?.path === feature.path}
                      onClick={() => onFeatureClick(feature)}
                      title={formatFeatureName(feature.name)}
                    >
                      <div className="flex flex-col items-start gap-0.5">
                        <span className="font-medium truncate cursor-pointer text-ellipsis">
                          {formatFeatureName(feature.name)}
                        </span>
                      </div>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))
              )}
            </SidebarMenu>
          </SidebarGroup>
        ) : (
          <SidebarMenu>
            {features.length > 0 && (
              <NavFeatures
                items={features}
                onFeatureClick={onFeatureClick}
                activeFeature={activeFeature}
              />
            )}
          </SidebarMenu>
        )}
      </SidebarContent>
      <SidebarFooter className="relative flex flex-col items-stretch">
        <div className="h-4 absolute top-[-1rem] z-50 inset-x-0 bg-gradient-to-t from-sidebar  to-transparent" />

        <SidebarGroup>
          {/*<SidebarGroupLabel>Insights</SidebarGroupLabel>*/}
          {features.length > 0 && (
            <SidebarMenu>
              <SidebarMenuItem>
                <NavLink to="/insights">
                  {({ isActive }) => (
                    <SidebarMenuButton
                      className="cursor-pointer"
                      isActive={isActive}
                    >
                      <BarChart3 className="h-4 w-4" /> Insights
                    </SidebarMenuButton>
                  )}
                </NavLink>
              </SidebarMenuItem>
            </SidebarMenu>
          )}
        </SidebarGroup>
        <div className="flex flex-row justify-between items-center">
          <ModeToggle />
          <VersionIndicator />
        </div>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
