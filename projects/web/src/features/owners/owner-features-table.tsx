import { SortableTableHeader } from '@/components/sortable-table-header'
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
import { useTableSort } from '@/hooks/use-table-sort'
import { formatExternalValue, humanize } from '@/lib/external-tools'
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import type { OwnedFeature } from './utils'

type OwnerFeaturesTableProps = {
  features: OwnedFeature[]
  /** Names of the `--data` metrics to show as columns */
  externalKeys: string[]
  skipChanges: boolean
  onOpenFeature: (feature: Feature) => void
}

const muted = (value: number | undefined) =>
  (value ?? 0) === 0 ? 'text-muted-foreground/50' : ''

/** Features of an owner, own and inherited, with their numbers */
export function OwnerFeaturesTable({
  features,
  externalKeys,
  skipChanges,
  onOpenFeature,
}: OwnerFeaturesTableProps) {
  const { sortedData, sortConfig, requestSort } = useTableSort(features)

  const header = (field: string, label: string, align?: 'right') => (
    <SortableTableHeader
      field={field}
      label={label}
      sortConfig={sortConfig}
      onSort={requestSort}
      align={align}
    />
  )

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <SortableTableHeader
              field="feature.name"
              label={`Feature (${features.length})`}
              sortConfig={sortConfig}
              onSort={requestSort}
              className="w-75"
            />
            <TableHead>Path</TableHead>
            {header('feature.stats.files_count', 'Files', 'right')}
            {header('feature.stats.lines_count', 'Lines', 'right')}
            {header(
              'feature.stats.coverage.line_coverage_percent',
              'Coverage',
              'right',
            )}
            {header('feature.stats.todos_count', 'TODOs', 'right')}
            {!skipChanges &&
              header('feature.stats.commits.last_commit_date', 'Last changed')}
            {externalKeys.map((key) => (
              <SortableTableHeader
                key={key}
                field={`feature.external.${key}`}
                label={humanize(key)}
                sortConfig={sortConfig}
                onSort={requestSort}
                align="right"
              />
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {sortedData.map(({ feature, inherited, inheritedFrom }) => {
            const coverage = feature.stats?.coverage
            return (
              <TableRow key={feature.path} data-testid="owner-feature-row">
                <TableCell className="font-medium">
                  <div className="flex items-center gap-2">
                    <button
                      type="button"
                      className="max-w-64 truncate text-left hover:underline underline-offset-4"
                      onClick={() => onOpenFeature(feature)}
                      title={`Open ${formatFeatureName(feature.name)}`}
                    >
                      {formatFeatureName(feature.name)}
                    </button>
                    {inherited && (
                      <Tooltip>
                        <TooltipTrigger>
                          <span className="rounded border px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
                            inherited
                          </span>
                        </TooltipTrigger>
                        <TooltipContent>
                          {inheritedFrom
                            ? `Owner inherited from ${formatFeatureName(inheritedFrom.name)}`
                            : 'Owner inherited from a parent feature'}
                        </TooltipContent>
                      </Tooltip>
                    )}
                  </div>
                </TableCell>
                <TableCell className="max-w-64 truncate font-mono text-xs text-muted-foreground">
                  {feature.path}
                </TableCell>
                <TableCell
                  className={cn(
                    'text-right tabular-nums',
                    muted(feature.stats?.files_count),
                  )}
                >
                  {feature.stats?.files_count ?? 0}
                </TableCell>
                <TableCell
                  className={cn(
                    'text-right tabular-nums',
                    muted(feature.stats?.lines_count),
                  )}
                >
                  {(feature.stats?.lines_count ?? 0).toLocaleString('en-US')}
                </TableCell>
                <TableCell className="text-right tabular-nums">
                  {coverage ? (
                    `${coverage.line_coverage_percent.toFixed(1)}%`
                  ) : (
                    <span className="text-muted-foreground/50">—</span>
                  )}
                </TableCell>
                <TableCell
                  className={cn(
                    'text-right tabular-nums',
                    muted(feature.stats?.todos_count),
                  )}
                >
                  {feature.stats?.todos_count ?? 0}
                </TableCell>
                {!skipChanges && (
                  <TableCell className="text-muted-foreground">
                    {formatDate(feature.stats?.commits.last_commit_date)}
                  </TableCell>
                )}
                {externalKeys.map((key) => (
                  <TableCell key={key} className="text-right tabular-nums">
                    {feature.external?.[key] === undefined ? (
                      <span className="text-muted-foreground/50">—</span>
                    ) : (
                      formatExternalValue(feature.external[key])
                    )}
                  </TableCell>
                ))}
              </TableRow>
            )
          })}
        </TableBody>
      </Table>
    </div>
  )
}
