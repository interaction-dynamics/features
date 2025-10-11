import { MessageSquare } from 'lucide-react'
import { useContext, useEffect, useState } from 'react'
import { AppSidebar } from '@/components/app-sidebar'
import { FeatureBreadcrumb } from '@/components/feature-breadcrumb'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { FeatureDetails } from './components/feature-details'
import { FeaturesContext } from './lib/features-context'
import type { Feature } from './models/feature'

// Helper function to find a feature by path in the features tree
const findFeatureByPath = (
  features: Feature[],
  path: string,
): Feature | null => {
  for (const feature of features) {
    if (feature.path === path) {
      return feature
    }
    if (feature.features) {
      const found = findFeatureByPath(feature.features, path)
      if (found) {
        return found
      }
    }
  }
  return null
}

export function Dashboard() {
  const { features } = useContext(FeaturesContext)
  const [currentFeature, setCurrentFeature] = useState<Feature | null>(null)

  // biome-ignore lint/correctness/useExhaustiveDependencies: we don't want to re-render on every feature change
  useEffect(() => {
    if (currentFeature?.path && features.length > 0) {
      // Find the updated version of the current feature by path
      const updatedFeature = findFeatureByPath(features, currentFeature.path)
      if (updatedFeature) {
        setCurrentFeature(updatedFeature)
      } else {
        // Feature was deleted, clear the selection
        setCurrentFeature(null)
      }
    }
  }, [features])

  return (
    <SidebarProvider>
      <AppSidebar
        activeFeature={currentFeature}
        onFeatureClick={setCurrentFeature}
      />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b">
          <div className="flex items-center gap-2 px-3">
            <SidebarTrigger />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <FeatureBreadcrumb
              feature={currentFeature}
              onFeatureClick={setCurrentFeature}
            />
          </div>
          <div className="ml-auto px-3 flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                window.open(
                  'https://github.com/interaction-dynamics/features/issues/new/choose',
                  '_blank',
                )
              }
              className="gap-2"
            >
              <MessageSquare className="h-4 w-4" />
              Give Feedback
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                window.open(
                  'https://github.com/interaction-dynamics/features',
                  '_blank',
                )
              }
            >
              <svg viewBox="0 0 98 96" className="h-4 w-4">
                <title>Github</title>
                <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"
                  fill="currentColor"
                />
              </svg>
            </Button>
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
  )
}
