import { useContext } from 'react'
import { Header } from '@/components/header'
import { StatsCard } from '@/components/stats-card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { DependencyGraph } from '@/features/insights/dependency-graph'
import { FeatureInsightsTable } from '@/features/insights/feature-insights-table'
import { OwnerInsightsTable } from '@/features/insights/owner-insights-table'

import { FeaturesContext } from '@/lib/features-context'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'

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

const actualLines = (feature: Feature) => {
  const lines = feature.stats?.lines_count ?? 0

  const childrenLinesCount =
    feature.features?.reduce(
      (acc, current) => acc + (current.stats?.lines_count ?? 0),
      0,
    ) ?? 0

  return lines - childrenLinesCount
}

export default function Insights() {
  const { features } = useContext(FeaturesContext)

  const allFeatures = flattenFeatures(features).sort((a, b) =>
    a.name.localeCompare(b.name),
  )

  const featureCount = allFeatures.length
  const ownerCount = [...new Set(allFeatures.map((feature) => feature.owner))]
    .length

  const largestFeature = allFeatures.reduce((prev, curr) => {
    return actualLines(prev) > actualLines(curr) ? prev : curr
  }, allFeatures[0])

  const featuresWithoutOwners = allFeatures.filter(
    (feature) => feature.owner === '',
  )

  return (
    <div className="absolute inset-0 flex flex-col">
      <Header>Insights</Header>
      <div className="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden p-4">
        <div className="space-y-4">
          {/* Overview Cards */}
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4 *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card dark:*:data-[slot=card]:bg-card *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:shadow-xs">
            <StatsCard title="Total Features" value={featureCount} />
            <StatsCard title="Total Owners" value={ownerCount} />
            <StatsCard
              title="Largest feature"
              value={formatFeatureName(largestFeature.name)}
              subtitle={`${largestFeature.stats?.lines_count ?? 'N/A'} lines`}
            />
            <StatsCard
              title="Number of features without owners"
              value={featuresWithoutOwners.length}
            />
          </div>
        </div>
        <Tabs defaultValue="feature" className="mt-1 flex min-h-0 flex-1 flex-col">
          <TabsList>
            <TabsTrigger value="feature">Features</TabsTrigger>
            <TabsTrigger value="owner">Ownership</TabsTrigger>
            <TabsTrigger value="dependencies">Dependencies</TabsTrigger>
          </TabsList>
          <TabsContent value="feature" className="mt-1 overflow-auto">
            <FeatureInsightsTable features={allFeatures} />
          </TabsContent>
          <TabsContent value="owner" className="mt-1 overflow-auto">
            <OwnerInsightsTable features={allFeatures} />
          </TabsContent>
          <TabsContent value="dependencies" className="mt-1 flex min-h-0 flex-1 flex-col">
            <DependencyGraph features={features} />
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
