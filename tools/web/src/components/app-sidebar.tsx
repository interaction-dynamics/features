import { Search, X } from 'lucide-react'
import type * as React from 'react'
import { useContext, useEffect, useMemo, useRef, useState } from 'react'
import { Input } from '@/components/ui/input'
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
  const searchInputRef = useRef<HTMLInputElement>(null)

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

  // Handle Meta+K shortcut to focus search
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.metaKey && event.key === 'k') {
        event.preventDefault()
        searchInputRef.current?.focus()
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [])

  return (
    <Sidebar {...props}>
      <SidebarHeader className="relative">
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <a href="./">
                <img
                  src="./feature-icon.svg"
                  alt="Feature Icon"
                  className="size-8"
                />
                <div className="flex flex-col gap-0.5 leading-none">
                  <span className="font-medium">Features</span>
                  {/*<span className="">v1.0.0</span>*/}
                </div>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
        {features.length > 0 && (
          <div className="px-2 pb-2">
            <div className="relative">
              <Search className="absolute left-2 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                ref={searchInputRef}
                placeholder="Search features..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-8 pr-16"
              />
              <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1">
                {searchQuery.trim() ? (
                  <button
                    type="button"
                    onClick={() => setSearchQuery('')}
                    className="cursor-pointer inline-flex h-5 w-5 items-center justify-center rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors"
                    aria-label="Clear search"
                  >
                    <X className="h-3 w-3" />
                  </button>
                ) : (
                  <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
                    <span className="text-xs">⌘</span>K
                  </kbd>
                )}
              </div>
            </div>
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
                    >
                      <a
                        href={`#${feature.path}`}
                        title={formatFeatureName(feature.name)}
                      >
                        <div className="flex flex-col items-start gap-0.5">
                          <span className="font-medium truncate cursor-pointer text-ellipsis">
                            {formatFeatureName(feature.name)}
                          </span>
                        </div>
                      </a>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))
              )}
            </SidebarMenu>
          </SidebarGroup>
        ) : (
          <SidebarMenu>
            {activeFeature && (
              <NavFeatures
                items={features}
                onFeatureClick={onFeatureClick}
                activeFeature={activeFeature}
              />
            )}
          </SidebarMenu>
        )}
      </SidebarContent>
      <SidebarFooter className="relative flex flex-row justify-between items-center">
        <div className="h-4 absolute top-[-1rem] z-50 inset-x-0 bg-gradient-to-t from-sidebar  to-transparent" />
        <ModeToggle />
        <VersionIndicator />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
