import type * as React from 'react'
import { useContext } from 'react'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from '@/components/ui/sidebar'
import { FeaturesContext } from '@/lib/features-context'
import type { Feature } from '@/models/feature'
import { ModeToggle } from './mode-toggle'
import { NavFeatures } from './nav-features'

interface AppSidebarProps extends React.ComponentProps<typeof Sidebar> {
  activeFeature: Feature | null
  onFeatureClick: (feature: Feature) => void
}

export function AppSidebar({
  activeFeature,
  onFeatureClick,
  ...props
}: AppSidebarProps) {
  const { features } = useContext(FeaturesContext)

  return (
    <Sidebar {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <a href="./">
                <img
                  src="/feature-icon.svg"
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
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarMenu>
            <NavFeatures
              items={features}
              onFeatureClick={onFeatureClick}
              activeFeature={activeFeature}
            />
          </SidebarMenu>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <ModeToggle />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
