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
import type { Feature } from '@/models/feature'

interface OwnerStats {
  owner: string
  featuresCount: number
  totalFiles: number
  totalLines: number
  totalCommits: number
  totalFeat: number
  totalFixes: number
  totalRefactors: number
  features: Feature[]
  metadata: Record<string, number>
}

type OwnerInsightsTableProps = {
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

export function OwnerInsightsTable({ features }: OwnerInsightsTableProps) {
  // Get all metadata keys present in features
  const metadataKeys = getAllMetadataKeys(features)
  if (features.length === 0) {
    return (
      <div className="flex items-center justify-center p-8">
        <p className="text-muted-foreground">No features available</p>
      </div>
    )
  }

  // Group features by owner and aggregate stats
  const ownerStatsMap = new Map<string, OwnerStats>()

  for (const feature of features) {
    const owner = feature.owner ?? 'Unknown'

    const stats = {
      owner,
      featuresCount: 0,
      totalFiles: 0,
      totalLines: 0,
      totalCommits: 0,
      totalFeat: 0,
      totalFixes: 0,
      totalRefactors: 0,
      features: [],
      metadata: {},
      ...ownerStatsMap.get(owner),
    }

    stats.featuresCount += 1
    stats.totalFiles += feature.stats?.files_count ?? 0
    stats.totalLines += feature.stats?.lines_count ?? 0
    stats.totalCommits += feature.stats?.commits.total_commits ?? 0
    stats.totalFeat += feature.stats?.commits.count_by_type?.feat ?? 0
    stats.totalFixes += feature.stats?.commits.count_by_type?.fix ?? 0
    stats.totalRefactors += feature.stats?.commits.count_by_type?.refactor ?? 0
    stats.features.push(feature)

    // Aggregate metadata counts
    const featureMetadata = getMetadataArrays(feature)
    for (const [key, count] of Object.entries(featureMetadata)) {
      stats.metadata[key] = (stats.metadata[key] ?? 0) + count
    }

    ownerStatsMap.set(owner, stats)
  }

  // Convert to array and sort by total commits (most active first)
  const ownerStats = Array.from(ownerStatsMap.values())
    .sort((a, b) => b.totalCommits - a.totalCommits)
    .sort((a, b) => a.owner.localeCompare(b.owner))

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[300px]">Owner</TableHead>
            <TableHead className="text-right">Features</TableHead>
            <TableHead className="text-right">Files</TableHead>
            <TableHead className="text-right">Lines</TableHead>
            {metadataKeys.map((key) => (
              <TableHead key={key} className="text-right capitalize">
                {key}
              </TableHead>
            ))}
            {/*<TableHead className="text-right">Total Changes</TableHead>
            <TableHead className="text-right">Feat</TableHead>
            <TableHead className="text-right">Fix</TableHead>
            <TableHead className="text-right">Refactor</TableHead>*/}
          </TableRow>
        </TableHeader>
        <TableBody>
          {ownerStats.map((stat) => (
            <TableRow key={stat.owner}>
              <TableCell className="font-medium">{stat.owner}</TableCell>
              <TableCell className="text-right tabular-nums">
                <Tooltip>
                  <TooltipTrigger>{stat.featuresCount}</TooltipTrigger>
                  <TooltipContent>
                    {stat.features.map((feature) => (
                      <div key={feature.path}>{feature.name}</div>
                    ))}
                  </TooltipContent>
                </Tooltip>
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalFiles}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalLines}
              </TableCell>
              {metadataKeys.map((key) => {
                const count = stat.metadata[key] ?? 0
                // Collect all metadata items of this type from all features
                const allItems: Record<string, string>[] = []
                for (const feature of stat.features) {
                  const items =
                    (feature.meta?.[key] as Record<string, string>[]) ?? []
                  allItems.push(...items)
                }

                return (
                  <TableCell key={key} className="text-right tabular-nums">
                    {count > 0 ? (
                      <Tooltip>
                        <TooltipTrigger>{count}</TooltipTrigger>
                        <TooltipContent className="max-w-md">
                          <p className="font-semibold mb-2 capitalize">
                            {key} ({count})
                          </p>
                          <div className="space-y-1 text-xs">
                            {allItems.slice(0, 10).map((item, idx) => (
                              <div
                                key={idx}
                                className="font-mono text-muted-foreground"
                              >
                                {Object.entries(item)
                                  .filter(([k]) => k !== 'feature')
                                  .slice(0, 3)
                                  .map(
                                    ([k, v]) =>
                                      `${k}: ${v.length > 20 ? v.substring(0, 20) + '...' : v}`,
                                  )
                                  .join(', ')}
                              </div>
                            ))}
                            {allItems.length > 10 && (
                              <div className="text-muted-foreground italic">
                                +{allItems.length - 10} more
                              </div>
                            )}
                          </div>
                        </TooltipContent>
                      </Tooltip>
                    ) : (
                      <span className="text-muted-foreground/50">0</span>
                    )}
                  </TableCell>
                )
              })}
              {/*<TableCell className="text-right tabular-nums">
                {stat.totalCommits}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalFeat}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalFixes}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalRefactors}
              </TableCell>*/}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
