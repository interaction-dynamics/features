import {
  BarChart3,
  CheckSquare,
  FileText,
  Flag,
  FlaskConical,
  FolderTree,
  GitCommitVertical,
  Network,
  Settings,
  Table,
  ToggleLeft,
  User,
} from 'lucide-react'
import { lazy, useContext } from 'react'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { FeaturesContext } from '@/lib/features-context'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'

import { FeatureMeta } from './feature-meta'
import { FeatureOwner } from './feature-owner'

const FeatureChanges = lazy(() => import('./feature-changes'))
const FeatureDecisions = lazy(() => import('./feature-decisions'))
const FeatureDependencies = lazy(() => import('@/features/dependencies'))
const FeatureDescription = lazy(() => import('./feature-description'))
const FeatureInsights = lazy(() => import('./feature-insights'))
const FeatureTests = lazy(() => import('./feature-tests'))

import { FeatureMetadataTable } from './feature-metadata-table'

interface FeatureDetailsProps {
  feature: Feature
}

export function FeatureDetails({ feature }: FeatureDetailsProps) {
  const { features } = useContext(FeaturesContext)

  // Extract metadata keys that have array values (e.g., 'flag', 'experiment', 'toggle')
  // Common feature metadata types
  const featureMetadataTypes = [
    'flag',
    'experiment',
    'toggle',
    'config',
    'deployment',
    'version',
    'deprecation',
  ]

  const metadataArrayKeys = feature.meta
    ? Object.keys(feature.meta).filter(
        (key) =>
          Array.isArray(feature.meta?.[key]) &&
          featureMetadataTypes.includes(key),
      )
    : []

  // Get icon for metadata key
  const getMetadataIcon = (key: string) => {
    if (key.includes('flag')) return Flag
    if (key.includes('experiment')) return FlaskConical
    if (key.includes('toggle')) return ToggleLeft
    if (key.includes('config')) return Settings
    return Table
  }

  return (
    <div className="flex flex-col gap-4">
      <div>
        <h1 className="text-3xl font-bold">
          {formatFeatureName(feature.name)}
        </h1>
      </div>
      <div className="flex flex-col gap-3">
        <div className="flex items-start gap-3">
          <FolderTree className="h-4 w-4 text-muted-foreground mt-0.5" />
          <div className="flex-1">
            <p className="text-sm font-medium text-foreground mb-1">Path</p>
            <p className="text-xs font-mono text-muted-foreground">
              {feature.path}
            </p>
          </div>
        </div>
        <div className="flex items-start gap-3">
          <User className="h-4 w-4 text-muted-foreground mt-0.5" />
          <div className="flex-1">
            <p className="text-sm font-medium text-foreground mb-1">Owner</p>
            <div className="text-xs font-mono text-muted-foreground">
              <FeatureOwner feature={feature} />
            </div>
          </div>
        </div>
        <FeatureMeta feature={feature} />
      </div>

      <Tabs defaultValue="description" className="mt-4">
        <TabsList>
          <TabsTrigger value="description">
            <FileText className="h-4 w-4" />
            Description
          </TabsTrigger>
          {feature.changes.length > 0 && (
            <TabsTrigger value="changes">
              <GitCommitVertical className="h-4 w-4" />
              Changes
            </TabsTrigger>
          )}
          {feature.stats && (
            <TabsTrigger value="tests">
              <FlaskConical className="h-4 w-4" />
              Tests
            </TabsTrigger>
          )}
          <TabsTrigger value="decisions">
            <CheckSquare className="h-4 w-4" />
            Decisions
          </TabsTrigger>
          {feature.stats && (
            <TabsTrigger value="insights">
              <BarChart3 className="h-4 w-4" />
              Insights
            </TabsTrigger>
          )}
          {feature.dependencies.length > 0 && (
            <TabsTrigger value="dependencies">
              <Network className="h-4 w-4" />
              Dependencies
            </TabsTrigger>
          )}
          {metadataArrayKeys.map((key) => {
            const Icon = getMetadataIcon(key)
            return (
              <TabsTrigger key={key} value={key}>
                <Icon className="h-4 w-4" />
                {key.charAt(0).toUpperCase() + key.slice(1)}
              </TabsTrigger>
            )
          })}
        </TabsList>
        <TabsContent value="description" className="mt-1">
          <FeatureDescription description={feature.description} />
        </TabsContent>
        {feature.changes.length > 0 && (
          <TabsContent value="changes" className="mt-1">
            <FeatureChanges changes={feature.changes} />
          </TabsContent>
        )}
        {feature.stats && (
          <TabsContent value="tests" className="mt-1">
            <FeatureTests stats={feature.stats} />
          </TabsContent>
        )}
        <TabsContent value="decisions" className="mt-1">
          <FeatureDecisions decisions={feature.decisions} />
        </TabsContent>
        {feature.stats && (
          <TabsContent value="insights" className="mt-1">
            <FeatureInsights
              stats={feature.stats}
              dependencies={feature.dependencies}
              currentFeatureName={feature.name}
              allFeatures={features}
            />
          </TabsContent>
        )}
        {feature.dependencies.length > 0 && (
          <TabsContent value="dependencies" className="mt-1">
            <FeatureDependencies
              dependencies={feature.dependencies}
              currentFeatureName={feature.name}
              allFeatures={features}
            />
          </TabsContent>
        )}
        {metadataArrayKeys.map((key) => (
          <TabsContent key={key} value={key} className="mt-1">
            <FeatureMetadataTable
              data={feature.meta?.[key] as Record<string, string>[]}
              metadataKey={key}
            />
          </TabsContent>
        ))}
      </Tabs>
    </div>
  )
}
