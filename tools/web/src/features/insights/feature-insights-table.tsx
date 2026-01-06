import { ArrowUpRight } from 'lucide-react'
import { useState } from 'react'
import { Link } from 'react-router'
import { FeatureOwner } from '@/components/feature-owner'
import { MetadataTooltipContent } from '@/components/metadata-tooltip-content'
import { SmartSearchInput } from '@/components/smart-search-input'
import { SortableTableHeader } from '@/components/sortable-table-header'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { useTableFilter } from '@/hooks/use-table-filter'
import { useTableSort } from '@/hooks/use-table-sort'
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'

type FeatureInsightsTableProps = {
  features: Feature[]
}

// Helper to get metadata array keys and their counts
function getMetadataArrays(feature: Feature): Record<string, number> {
  const metadataArrays: Record<string, number> = {}
  if (!feature.meta) return metadataArrays

  const knownMetadataTypes = [
    'flag',
    'experiment',
    'toggle',
    'config',
    'deployment',
    'version',
    'deprecation',
  ]

  for (const key of Object.keys(feature.meta)) {
    if (knownMetadataTypes.includes(key) && Array.isArray(feature.meta[key])) {
      metadataArrays[key] = (feature.meta[key] as unknown[]).length
    }
  }

  return metadataArrays
}

// Get all unique metadata keys across all features
function getAllMetadataKeys(features: Feature[]): string[] {
  const keysSet = new Set<string>()
  for (const feature of features) {
    const keys = Object.keys(getMetadataArrays(feature))
    for (const key of keys) {
      keysSet.add(key)
    }
  }
  return Array.from(keysSet).sort()
}

export function FeatureInsightsTable({ features }: FeatureInsightsTableProps) {
  const [showSearch] = useState(true)

  // Get all metadata keys present in features
  const metadataKeys = getAllMetadataKeys(features)

  // Define searchable fields for the smart filter
  const searchableFields = [
    'name',
    'owner',
    'path',
    'stats.files_count',
    'stats.lines_count',
    'stats.todos_count',
    'stats.commits.total_commits',
    'stats.commits.first_commit_date',
    'stats.commits.last_commit_date',
    'stats.commits.count_by_type.feat',
    'stats.commits.count_by_type.fix',
    'stats.commits.count_by_type.refactor',
  ]

  // Apply filtering
  const { filteredData, query, setQuery, clearQuery } = useTableFilter(
    features,
    searchableFields,
  )

  // Apply sorting
  const { sortedData, sortConfig, requestSort } = useTableSort(filteredData)

  // Available fields for the help popover
  const availableFields = [
    'name',
    'owner',
    'path',
    'stats.files_count',
    'stats.lines_count',
    'stats.todos_count',
    'stats.commits.total_commits',
    'stats.commits.count_by_type.feat',
    'stats.commits.count_by_type.fix',
    'stats.commits.count_by_type.refactor',
  ]

  if (features.length === 0) {
    return (
      <div className="flex items-center justify-center p-8">
        <p className="text-muted-foreground">No features available</p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      {/* Search and Filter Controls */}
      <div className="flex items-center justify-between gap-4">
        <div className="flex-1">
          {showSearch && (
            <SmartSearchInput
              value={query}
              onChange={setQuery}
              onClear={clearQuery}
              placeholder="Filter features... (e.g., owner:john AND lines:>1000)"
              fields={availableFields}
            />
          )}
        </div>
        {/*<Button
          variant={showSearch ? 'default' : 'outline'}
          size="sm"
          onClick={() => setShowSearch(!showSearch)}
          className="gap-2 h-9"
        >
          <Filter className="h-4 w-4" />
          {showSearch ? 'Hide Filter' : 'Show Filter'}
        </Button>*/}
      </div>

      {/* Results Count */}
      {query && (
        <div className="text-sm text-muted-foreground">
          Showing {sortedData.length} of {features.length} features
        </div>
      )}

      {/* Table */}
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <SortableTableHeader
                field="name"
                label={`Feature (${sortedData.length})`}
                sortConfig={sortConfig}
                onSort={requestSort}
                className="w-75"
              />
              <SortableTableHeader
                field="owner"
                label="Owner"
                sortConfig={sortConfig}
                onSort={requestSort}
              />
              <SortableTableHeader
                field="stats.commits.first_commit_date"
                label="Initial Date"
                sortConfig={sortConfig}
                onSort={requestSort}
              />
              <SortableTableHeader
                field="stats.commits.last_commit_date"
                label="Last changed"
                sortConfig={sortConfig}
                onSort={requestSort}
              />
              <SortableTableHeader
                field="stats.files_count"
                label="Files"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              <SortableTableHeader
                field="stats.lines_count"
                label="Lines"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              <SortableTableHeader
                field="stats.todos_count"
                label="TODOs"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              <SortableTableHeader
                field="stats.commits.total_commits"
                label="Total Changes"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              <SortableTableHeader
                field="stats.commits.count_by_type.feat"
                label="Feat"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              <SortableTableHeader
                field="stats.commits.count_by_type.fix"
                label="Fix"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              <SortableTableHeader
                field="stats.commits.count_by_type.refactor"
                label="Refactor"
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
              {metadataKeys.map((key) => (
                <TableHead key={key} className="text-right capitalize">
                  {key}
                </TableHead>
              ))}
              <TableHead className="w-12.5"></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {sortedData.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={12}
                  className="text-center text-muted-foreground"
                >
                  {query
                    ? 'No features match your search criteria'
                    : 'No feature data available'}
                </TableCell>
              </TableRow>
            ) : (
              sortedData.map((feature) => (
                <TableRow key={feature.path}>
                  <TableCell className="font-medium ">
                    <Tooltip>
                      <TooltipTrigger className="max-w-64 truncate">
                        {formatFeatureName(feature.name)}
                      </TooltipTrigger>
                      <TooltipContent>
                        <p className="font-semibold">
                          {formatFeatureName(feature.name)}
                        </p>
                        <p className="font-mono">{feature.path}</p>
                      </TooltipContent>
                    </Tooltip>
                  </TableCell>
                  <TableCell className="text-muted-foreground">
                    <FeatureOwner feature={feature} />
                  </TableCell>
                  <TableCell className="text-muted-foreground">
                    {formatDate(feature.stats?.commits.first_commit_date)}
                  </TableCell>
                  <TableCell className="text-muted-foreground">
                    {formatDate(feature.stats?.commits.last_commit_date)}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {feature.stats?.files_count ?? 0}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {feature.stats?.lines_count ?? 0}
                  </TableCell>
                  <TableCell
                    className={cn(
                      'text-right tabular-nums',
                      (feature.stats?.todos_count ?? 0) === 0
                        ? 'text-muted-foreground/50'
                        : '',
                    )}
                  >
                    {feature.stats?.todos_count ?? 0}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {feature.stats?.commits.total_commits}
                  </TableCell>
                  <TableCell
                    className={cn(
                      'text-right tabular-nums',
                      (feature.stats?.commits.count_by_type?.feat ?? 0) === 0
                        ? 'text-muted-foreground/50'
                        : '',
                    )}
                  >
                    {feature.stats?.commits.count_by_type?.feat ?? 0}
                  </TableCell>
                  <TableCell
                    className={cn(
                      'text-right tabular-nums',
                      (feature.stats?.commits.count_by_type?.fix ?? 0) === 0
                        ? 'text-muted-foreground/50'
                        : '',
                    )}
                  >
                    {feature.stats?.commits.count_by_type?.fix ?? 0}
                  </TableCell>
                  <TableCell
                    className={cn(
                      'text-right tabular-nums',
                      (feature.stats?.commits.count_by_type?.refactor ?? 0) ===
                        0
                        ? 'text-muted-foreground/50'
                        : '',
                    )}
                  >
                    {feature.stats?.commits.count_by_type?.refactor ?? 0}
                  </TableCell>
                  {metadataKeys.map((key) => {
                    const metadataArrays = getMetadataArrays(feature)
                    const count = metadataArrays[key] ?? 0
                    const items =
                      (feature.meta?.[key] as Record<string, string>[]) ?? []

                    return (
                      <TableCell
                        key={key}
                        className={cn(
                          'text-right tabular-nums',
                          count === 0 ? 'text-muted-foreground/50' : '',
                        )}
                      >
                        {count > 0 ? (
                          <Tooltip>
                            <TooltipTrigger>{count}</TooltipTrigger>
                            <TooltipContent className="max-w-md">
                              <MetadataTooltipContent
                                items={items}
                                metadataKey={key}
                              />
                            </TooltipContent>
                          </Tooltip>
                        ) : (
                          0
                        )}
                      </TableCell>
                    )
                  })}
                  <TableCell>
                    <Button
                      variant="ghost"
                      size="sm"
                      asChild
                      className="h-8 w-8 p-0"
                    >
                      <Link
                        to={`/?feature=${feature.path}`}
                        title={`Go to ${formatFeatureName(feature.name)}`}
                      >
                        <ArrowUpRight className="h-4 w-4" />
                        <span className="sr-only">Go to feature</span>
                      </Link>
                    </Button>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  )
}
