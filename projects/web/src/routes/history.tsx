import {
  GitBranch,
  GitCommitVertical,
  History as HistoryIcon,
} from 'lucide-react'
import { useContext, useMemo, useState } from 'react'
import { useNavigate, useSearchParams } from 'react-router'
import { Header } from '@/components/header'
import { StatsCard } from '@/components/stats-card'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  buildHistory,
  type CommitType,
  EMPTY_FILTERS,
  featureScope,
  filterHistory,
  flattenFeatures,
  groupByMonth,
  HistoryActivityChart,
  HistoryFeatureGraph,
  type HistoryFilterState as Filters,
  HistoryFilters,
  HistoryTimeline,
  summarizeHistory,
} from '@/features/history'
import { useMetadata } from '@/hooks/use-metadata'
import { FeaturesContext } from '@/lib/features-context'
import { formatDate } from '@/lib/format-date'
import type { Feature } from '@/models/feature'

const PAGE_SIZE = 100

/** Project-wide history of the features, built from their commits */
export default function History() {
  const { features, isAggregated } = useContext(FeaturesContext)
  const { metadata } = useMetadata()
  const navigate = useNavigate()
  const [searchParams, setSearchParams] = useSearchParams()
  const view = searchParams.get('view') === 'graph' ? 'graph' : 'timeline'

  const changeView = (next: string) =>
    setSearchParams(
      (params) => {
        if (next === 'graph') params.set('view', 'graph')
        else params.delete('view')
        return params
      },
      { replace: true },
    )

  const [filters, setFilters] = useState<Filters>(EMPTY_FILTERS)
  const [limit, setLimit] = useState(PAGE_SIZE)

  const history = useMemo(() => buildHistory(features), [features])
  const filtered = useMemo(
    () => filterHistory(history, filters, features),
    [history, filters, features],
  )
  const summary = useMemo(
    () =>
      summarizeHistory(
        filtered,
        filters.featurePath
          ? featureScope(features, filters.featurePath)
          : undefined,
      ),
    [filtered, filters.featurePath, features],
  )
  const months = useMemo(
    () => groupByMonth(filtered.slice(0, limit)),
    [filtered, limit],
  )

  // Types of the commits matching the other filters, most frequent first
  const types = useMemo(() => {
    const counts = new Map<CommitType, number>()
    for (const commit of filterHistory(
      history,
      { ...filters, type: null },
      features,
    )) {
      counts.set(commit.type, (counts.get(commit.type) ?? 0) + 1)
    }
    return [...counts.entries()].sort((a, b) => b[1] - a[1])
  }, [history, filters, features])

  const selectedFeature = filters.featurePath
    ? flattenFeatures(features).find((f) => f.path === filters.featurePath)
    : undefined

  const updateFilters = (next: Filters) => {
    setFilters(next)
    setLimit(PAGE_SIZE)
  }

  const openFeature = (feature: Feature) =>
    navigate({ pathname: '/', search: `?feature=${feature.path}` })

  // Commit links need a single repository
  const repository = isAggregated ? undefined : metadata?.repository

  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>
        <div className="flex items-center gap-2 font-medium">
          <HistoryIcon className="h-4 w-4" />
          History
        </div>
      </Header>
      {history.length === 0 ? (
        <div className="flex flex-col items-center justify-center gap-2 py-16 text-center">
          <GitCommitVertical className="h-12 w-12 text-muted-foreground" />
          <p className="text-sm font-medium">No commit history</p>
          <p className="max-w-sm text-sm text-muted-foreground">
            {metadata?.skipChanges
              ? 'The report was generated with --skip-changes.'
              : 'None of the features has commits yet.'}
          </p>
        </div>
      ) : (
        <Tabs value={view} onValueChange={changeView} className="gap-4 p-4">
          <TabsList>
            <TabsTrigger value="timeline">
              <GitCommitVertical />
              Timeline
            </TabsTrigger>
            <TabsTrigger value="graph">
              <GitBranch />
              Graph
            </TabsTrigger>
          </TabsList>
          <TabsContent value="graph">
            <HistoryFeatureGraph
              features={features}
              onOpenFeature={openFeature}
            />
          </TabsContent>
          <TabsContent value="timeline" className="flex flex-col gap-4">
            <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
              <StatsCard title="Commits" value={summary.commits} />
              <StatsCard
                title="Features introduced"
                value={summary.introduced}
              />
              <StatsCard title="Contributors" value={summary.contributors} />
              <StatsCard
                title="Period"
                value={
                  summary.first
                    ? formatDate(summary.first.replace(' ', 'T'))
                    : '—'
                }
                subtitle={
                  summary.last
                    ? `to ${formatDate(summary.last.replace(' ', 'T'))}`
                    : undefined
                }
              />
            </div>
            <HistoryActivityChart commits={filtered} />
            <HistoryFilters
              filters={filters}
              onChange={updateFilters}
              types={types}
              selectedFeature={selectedFeature}
            />
            {filtered.length === 0 ? (
              <div className="flex flex-col items-center gap-3 py-12 text-center">
                <p className="text-sm text-muted-foreground">
                  No commit matches these filters.
                </p>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => updateFilters(EMPTY_FILTERS)}
                >
                  Clear filters
                </Button>
              </div>
            ) : (
              <>
                <HistoryTimeline
                  months={months}
                  repository={repository}
                  activeFeaturePath={filters.featurePath}
                  onFilterFeature={(feature) =>
                    updateFilters({ ...filters, featurePath: feature.path })
                  }
                  onOpenFeature={openFeature}
                />
                {filtered.length > limit && (
                  <div className="flex justify-center">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setLimit((current) => current + PAGE_SIZE)}
                    >
                      Show more ({filtered.length - limit} left)
                    </Button>
                  </div>
                )}
              </>
            )}
          </TabsContent>
        </Tabs>
      )}
    </div>
  )
}
