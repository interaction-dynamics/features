import { useContext } from 'react'
import { Header } from '@/components/header'
import { StatsCard } from '@/components/stats-card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { FeatureInsightsTable } from '@/features/insights/feature-insights-table'
import { OwnerInsightsTable } from '@/features/insights/owner-insights-table'

import { FeaturesContext } from '@/lib/features-context'
import { formatDate } from '@/lib/format-date'
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
    return (prev.stats?.lines_count ?? 0) > (curr.stats?.lines_count ?? 0)
      ? prev
      : curr
  }, allFeatures[0])

  const lastChangedFeature = allFeatures.reduce((prev, curr) => {
    return new Date(prev.stats?.commits.last_commit_date ?? '') >
      new Date(curr.stats?.commits.last_commit_date ?? '')
      ? prev
      : curr
  }, allFeatures[0])

  // Find features with most changes in the month before lastChangedFeature's last commit
  const mostFrequentChangedFeature = (() => {
    if (!lastChangedFeature?.stats?.commits.last_commit_date) {
      return allFeatures[0]
    }

    const lastCommitDate = new Date(
      lastChangedFeature.stats.commits.last_commit_date,
    )
    const oneMonthBefore = new Date(lastCommitDate)
    oneMonthBefore.setMonth(oneMonthBefore.getMonth() - 1)

    // Count changes for each feature within the month period
    const featuresWithChangeCounts = allFeatures.map((feature) => {
      let changesInPeriod = 0

      for (const change of feature.changes) {
        const changeDate = new Date(change.date)
        if (changeDate >= oneMonthBefore && changeDate <= lastCommitDate) {
          changesInPeriod++
        }
      }

      return {
        feature,
        changesInPeriod,
      }
    })

    // Find the feature with the most changes in this period
    const result = featuresWithChangeCounts.reduce((prev, curr) => {
      return curr.changesInPeriod > prev.changesInPeriod ? curr : prev
    }, featuresWithChangeCounts[0])

    return result.feature
  })()

  return (
    <>
      <Header>Insights</Header>
      <div className="flex flex-1 flex-col gap-4 p-4">
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
              title="Most frequent changed feature"
              value={formatFeatureName(mostFrequentChangedFeature.name)}
              subtitle={`In the last month before ${formatDate(mostFrequentChangedFeature.stats?.commits.last_commit_date)}`}
            />
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
    </>
  )
}
