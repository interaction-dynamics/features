import { useContext } from 'react'
import { Header } from '@/components/header'
import { StatsCard } from '@/components/stats-card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { FeatureInsightsTable } from '@/features/insights/feature-insights-table'
import { OwnerInsightsTable } from '@/features/insights/owner-insights-table'

import { FeaturesContext } from '@/lib/features-context'
import { formatFeatureName } from '@/lib/format-feature-name'
import { resolveOwner } from '@/lib/resolve-owner'
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

export function Insights() {
  const { features } = useContext(FeaturesContext)

  const allFeatures = flattenFeatures(features).sort((a, b) =>
    a.name.localeCompare(b.name),
  )

  const featureCount = allFeatures.length
  const ownerCount = [
    ...new Set(allFeatures.map((feature) => resolveOwner(feature))),
  ].length

  const largestFeature = allFeatures.reduce((prev, curr) => {
    return actualLines(prev) > actualLines(curr) ? prev : curr
  }, allFeatures[0])

  const featuresWithoutOwners = allFeatures.filter(
    (feature) => resolveOwner(feature) === 'Unknown',
  )

  const totalTodos = allFeatures.reduce(
    (acc, feature) => acc + (feature.stats?.todos_count ?? 0),
    0,
  )

  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>Insights</Header>
      <div className="flex flex-1 flex-col gap-4 p-4">
        <div className="space-y-4">
          {/* Overview Cards */}
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-5 *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card dark:*:data-[slot=card]:bg-card *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:shadow-xs">
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
            <StatsCard title="Total TODOs" value={totalTodos} />
          </div>
        </div>
        <Tabs defaultValue="feature" className="mt-1">
          <TabsList>
            <TabsTrigger value="feature">Features</TabsTrigger>
            <TabsTrigger value="owner">Ownership</TabsTrigger>
          </TabsList>
          <TabsContent value="feature" className="mt-1">
            <FeatureInsightsTable features={allFeatures} />
          </TabsContent>
          <TabsContent value="owner" className="mt-1">
            <OwnerInsightsTable features={allFeatures} />
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
