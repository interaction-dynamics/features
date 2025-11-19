import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { resolveOwner } from '@/lib/resolve-owner'
import type { Feature } from '@/models/feature'

interface OwnerStats {
  owner: string
  featuresCount: number
  totalFiles: number
  totalLines: number
  totalCommits: number
  totalFixes: number
  totalRefactors: number
  features: Feature[]
}

type OwnerInsightsTableProps = {
  features: Feature[]
}

export function OwnerInsightsTable({ features }: OwnerInsightsTableProps) {
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
    const owner = resolveOwner(feature)

    const stats = {
      owner,
      featuresCount: 0,
      totalFiles: 0,
      totalLines: 0,
      totalCommits: 0,
      totalFixes: 0,
      totalRefactors: 0,
      features: [],
      ...ownerStatsMap.get(owner),
    }

    stats.featuresCount += 1
    stats.totalFiles += feature.stats?.files_count ?? 0
    stats.totalLines += feature.stats?.lines_count ?? 0
    stats.totalCommits += feature.stats?.commits.total_commits ?? 0
    stats.totalFixes += feature.stats?.commits.count_by_type?.fix ?? 0
    stats.totalRefactors += feature.stats?.commits.count_by_type?.refactor ?? 0
    stats.features.push(feature)

    ownerStatsMap.set(owner, stats)
  }

  // Convert to array and sort by total commits (most active first)
  const ownerStats = Array.from(ownerStatsMap.values()).sort(
    (a, b) => b.totalCommits - a.totalCommits,
  )

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[300px]">Owner</TableHead>
            <TableHead className="text-right">Features</TableHead>
            <TableHead className="text-right">Files</TableHead>
            <TableHead className="text-right">Lines</TableHead>
            <TableHead className="text-right">Total Changes</TableHead>
            <TableHead className="text-right">Fixes</TableHead>
            <TableHead className="text-right">Refactors</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {ownerStats.map((stat) => (
            <TableRow key={stat.owner}>
              <TableCell className="font-medium">{stat.owner}</TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.featuresCount}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalFiles}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalLines}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalCommits}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalFixes}
              </TableCell>
              <TableCell className="text-right tabular-nums">
                {stat.totalRefactors}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
