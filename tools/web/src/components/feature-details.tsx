import {
  BarChart3,
  CheckSquare,
  FileText,
  FolderTree,
  GitCommitVertical,
  HelpCircle,
  User,
} from 'lucide-react'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'
import { FeatureChanges } from './feature-changes'
import { FeatureDecisions } from './feature-decisions'
import { FeatureDescription } from './feature-description'
import { FeatureInsights } from './feature-insights'
import { FeatureMeta } from './feature-meta'

interface FeatureDetailsProps {
  feature: Feature
}

export function FeatureDetails({ feature }: FeatureDetailsProps) {
  // Recursively find owner by traversing parent chain
  const resolveOwner = (currentFeature: Feature): string => {
    if (currentFeature.owner !== 'Unknown') {
      return currentFeature.owner
    }
    if (currentFeature.parent) {
      return resolveOwner(currentFeature.parent)
    }
    return 'Unknown'
  }

  const resolvedOwner = resolveOwner(feature)
  const isInheritedOwner = resolvedOwner !== feature.owner

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
              {isInheritedOwner ? (
                <div className="flex items-center gap-2">
                  <span>{resolvedOwner}</span>
                  <span className="text-xs px-1.5 py-0.5 bg-amber-50 text-amber-700 dark:bg-amber-900/20 dark:text-amber-400 rounded text-[10px] font-medium">
                    inherited
                  </span>
                  {resolvedOwner === 'Unknown' && (
                    <button
                      type="button"
                      onClick={() =>
                        window.open(
                          'https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-an-owner',
                          '_blank',
                        )
                      }
                      className="cursor-pointer text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1"
                      title="Learn how to add an owner"
                    >
                      <HelpCircle className="h-3 w-3" />
                    </button>
                  )}
                </div>
              ) : (
                <div className="flex items-center gap-2">
                  <span>{resolvedOwner}</span>
                  {resolvedOwner === 'Unknown' && (
                    <button
                      type="button"
                      onClick={() =>
                        window.open(
                          'https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-an-owner',
                          '_blank',
                        )
                      }
                      className="cursor-pointer text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1"
                      title="Learn how to add an owner"
                    >
                      <HelpCircle className="h-3 w-3" />
                    </button>
                  )}
                </div>
              )}
            </div>
          </div>
        </div>
        <FeatureMeta meta={feature.meta} />
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
        </TabsList>
        <TabsContent value="description" className="mt-1">
          <FeatureDescription description={feature.description} />
        </TabsContent>
        {feature.changes.length > 0 && (
          <TabsContent value="changes" className="mt-1">
            <FeatureChanges changes={feature.changes} />
          </TabsContent>
        )}
        <TabsContent value="decisions" className="mt-1">
          <FeatureDecisions decisions={feature.decisions} />
        </TabsContent>
        {feature.stats && (
          <TabsContent value="insights" className="mt-1">
            <FeatureInsights stats={feature.stats} />
          </TabsContent>
        )}
      </Tabs>
    </div>
  )
}
