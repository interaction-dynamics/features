import {
  Database,
  GitCommitVertical,
  ScrollText,
  Table as TableIcon,
  User,
  UserX,
} from 'lucide-react'
import { useContext, useMemo, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router'
import { ExperimentalBadge } from '@/components/experimental-badge'
import { Header } from '@/components/header'
import { StatsCard } from '@/components/stats-card'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { groupByMonth, HistoryTimeline } from '@/features/history'
import {
  OwnerFeaturesTable,
  OwnerRecords,
  ownerFromParam,
  summarizeOwner,
} from '@/features/owners'
import { useMetadata } from '@/hooks/use-metadata'
import { formatExternalValue, humanize } from '@/lib/external-tools'
import { FeaturesContext } from '@/lib/features-context'
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { ExternalValue, Feature } from '@/models/feature'
import { getOwnerName } from '@/models/owner'

const PAGE_SIZE = 20

const formatNumber = (value: number) => value.toLocaleString('en-US')

/** `healthy ×6 · degraded ×4`, or the number of values when there are many */
const formatValueCounts = (counts: { value: ExternalValue; count: number }[]) =>
  counts.length > 3
    ? `${counts.length} values`
    : counts
        .map(({ value, count }) =>
          count > 1
            ? `${formatExternalValue(value)} ×${count}`
            : formatExternalValue(value),
        )
        .join(' · ')

/** Everything about one owner (team or person): features, totals, changes, records */
export default function Owner() {
  const params = useParams()
  const owner = ownerFromParam(params.owner)
  // Reset the filters when navigating to another owner
  return <OwnerPage key={owner} owner={owner} />
}

function OwnerPage({ owner }: { owner: string }) {
  const name = getOwnerName(owner)

  const { features, isAggregated } = useContext(FeaturesContext)
  const { metadata } = useMetadata()
  const navigate = useNavigate()
  const [featurePath, setFeaturePath] = useState<string | null>(null)
  const [limit, setLimit] = useState(PAGE_SIZE)

  const summary = useMemo(
    () => summarizeOwner(features, owner),
    [features, owner],
  )

  const commits = useMemo(
    () =>
      (summary?.commits ?? []).filter(
        (commit) =>
          !featurePath || commit.features.some((f) => f.path === featurePath),
      ),
    [summary, featurePath],
  )
  const months = useMemo(
    () => groupByMonth(commits.slice(0, limit)),
    [commits, limit],
  )

  const openFeature = (feature: Feature) =>
    navigate({ pathname: '/', search: `?feature=${feature.path}` })

  const filterFeature = (feature: Feature) => {
    setFeaturePath((current) =>
      current === feature.path ? null : feature.path,
    )
    setLimit(PAGE_SIZE)
  }

  const skipChanges = metadata?.skipChanges ?? false
  // Commit links need a single repository
  const repository = isAggregated ? undefined : metadata?.repository

  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>
        <div className="flex min-w-0 items-center gap-2 font-medium">
          <User className="h-4 w-4 shrink-0" />
          <span className="text-muted-foreground">Owners</span>
          <span className="text-muted-foreground">/</span>
          <span className="truncate" data-testid="owner-title">
            {name}
          </span>
          <ExperimentalBadge />
        </div>
      </Header>
      {!summary ? (
        <div
          className="flex flex-col items-center justify-center gap-2 py-16 text-center"
          data-testid="owner-not-found"
        >
          <UserX className="h-12 w-12 text-muted-foreground" />
          <p className="text-sm font-medium">No feature owned by {name}</p>
          <p className="max-w-sm text-sm text-muted-foreground">
            No feature has this owner in its README front matter, or inherits it
            from a parent feature.
          </p>
          <Button variant="outline" size="sm" asChild className="mt-2">
            <Link to="/insights">See all owners in Overview</Link>
          </Button>
        </div>
      ) : (
        <div className="flex flex-col gap-4 p-4">
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6">
            <StatsCard
              title="Features"
              value={summary.totals.features}
              subtitle={`${summary.totals.direct} own · ${summary.totals.inherited} inherited`}
            />
            <StatsCard
              title="Files"
              value={formatNumber(summary.totals.files)}
            />
            <StatsCard
              title="Lines of code"
              value={formatNumber(summary.totals.lines)}
            />
            <StatsCard
              title="Coverage"
              value={
                summary.totals.coverage
                  ? `${summary.totals.coverage.percent.toFixed(1)}%`
                  : '—'
              }
              subtitle={
                summary.totals.coverage
                  ? `${formatNumber(summary.totals.coverage.linesCovered)} / ${formatNumber(summary.totals.coverage.linesTotal)} lines`
                  : 'No coverage report'
              }
            />
            <StatsCard
              title="Technical debt"
              value={formatNumber(summary.totals.todos)}
              subtitle="TODO comments"
            />
            <StatsCard
              title="Commits"
              value={skipChanges ? '—' : formatNumber(summary.totals.commits)}
              subtitle={
                skipChanges
                  ? 'Skipped (--skip-changes)'
                  : summary.totals.lastChange
                    ? `${summary.totals.contributors} contributors · last ${formatDate(summary.totals.lastChange.replace(' ', 'T'))}`
                    : undefined
              }
            />
          </div>

          {summary.external.length > 0 && (
            <div
              className="flex items-start gap-3"
              data-testid="owner-external"
            >
              <Database className="mt-0.5 h-4 w-4 text-muted-foreground" />
              <div className="flex-1">
                <p className="mb-1 text-sm font-medium">Data</p>
                <div className="flex flex-wrap gap-2">
                  {summary.external.map((metric) => (
                    <div
                      key={metric.key}
                      className="rounded-md border bg-muted px-2 py-1 text-xs"
                      title={
                        metric.total === undefined
                          ? `${metric.key}: number of features per value`
                          : `${metric.key}: sum over ${metric.values.length} features`
                      }
                    >
                      <span className="text-muted-foreground">
                        {humanize(metric.key)}
                      </span>{' '}
                      <span className="font-semibold tabular-nums">
                        {metric.total !== undefined
                          ? formatExternalValue(metric.total)
                          : formatValueCounts(metric.counts ?? [])}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          <Tabs defaultValue="features" className="mt-2">
            <TabsList>
              <TabsTrigger value="features">
                <TableIcon className="h-4 w-4" />
                Features
              </TabsTrigger>
              {!skipChanges && (
                <TabsTrigger value="changes">
                  <GitCommitVertical className="h-4 w-4" />
                  Changes
                  <span className="rounded bg-muted px-1.5 text-xs tabular-nums text-muted-foreground">
                    {summary.commits.length}
                  </span>
                </TabsTrigger>
              )}
              <TabsTrigger value="records">
                <ScrollText className="h-4 w-4" />
                Specs & Decisions
                <span className="rounded bg-muted px-1.5 text-xs tabular-nums text-muted-foreground">
                  {summary.records.length}
                </span>
              </TabsTrigger>
            </TabsList>
            <TabsContent value="features" className="mt-1 overflow-auto">
              <OwnerFeaturesTable
                features={summary.features}
                externalKeys={summary.external.map((metric) => metric.key)}
                skipChanges={skipChanges}
                onOpenFeature={openFeature}
              />
            </TabsContent>
            {!skipChanges && (
              <TabsContent value="changes" className="mt-1 flex flex-col gap-4">
                {featurePath && (
                  <div className="flex items-center gap-2 text-sm text-muted-foreground">
                    Changes of{' '}
                    <span className="font-medium text-foreground">
                      {formatFeatureName(
                        summary.features.find(
                          (f) => f.feature.path === featurePath,
                        )?.feature.name ?? featurePath,
                      )}
                    </span>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setFeaturePath(null)}
                    >
                      Show all
                    </Button>
                  </div>
                )}
                {commits.length === 0 ? (
                  <p className="py-12 text-center text-sm text-muted-foreground">
                    None of these features has commits yet.
                  </p>
                ) : (
                  <>
                    <HistoryTimeline
                      months={months}
                      repository={repository}
                      activeFeaturePath={featurePath}
                      onFilterFeature={filterFeature}
                      onOpenFeature={openFeature}
                    />
                    {commits.length > limit && (
                      <div className="flex justify-center">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setLimit((n) => n + PAGE_SIZE)}
                        >
                          Show more ({commits.length - limit} left)
                        </Button>
                      </div>
                    )}
                  </>
                )}
              </TabsContent>
            )}
            <TabsContent value="records" className="mt-1">
              <OwnerRecords
                records={summary.records}
                onOpenFeature={openFeature}
              />
            </TabsContent>
          </Tabs>
        </div>
      )}
    </div>
  )
}
