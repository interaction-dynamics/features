import { ArrowUpRight } from 'lucide-react'
import { Link } from 'react-router'
import { FeatureOwner } from '@/components/feature-owner'
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
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Feature } from '@/models/feature'

type FeatureInsightsTableProps = {
  features: Feature[]
}

export function FeatureInsightsTable({ features }: FeatureInsightsTableProps) {
  if (features.length === 0) {
    return (
      <div className="flex items-center justify-center p-8">
        <p className="text-muted-foreground">No features available</p>
      </div>
    )
  }

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[300px]">Feature</TableHead>
            <TableHead>Owner</TableHead>
            <TableHead>Initial Date</TableHead>
            <TableHead>Last changed</TableHead>
            <TableHead className="text-right">Files</TableHead>
            <TableHead className="text-right">Lines</TableHead>
            <TableHead className="text-right">TODOs</TableHead>
            <TableHead className="text-right">Total Changes</TableHead>
            <TableHead className="text-right">Feat</TableHead>
            <TableHead className="text-right">Fix</TableHead>
            <TableHead className="text-right">Refactor</TableHead>
            <TableHead className="w-[50px]"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {features.length === 0 ? (
            <TableRow>
              <TableCell
                colSpan={11}
                className="text-center text-muted-foreground"
              >
                No feature data available
              </TableCell>
            </TableRow>
          ) : (
            features.map((feature) => (
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
                <TableCell className="text-right tabular-nums">
                  {feature.stats?.todos_count ?? 0}
                </TableCell>
                <TableCell className="text-right tabular-nums">
                  {feature.stats?.commits.total_commits}
                </TableCell>
                <TableCell className="text-right tabular-nums">
                  {feature.stats?.commits.count_by_type?.feat ?? 0}
                </TableCell>
                <TableCell className="text-right tabular-nums">
                  {feature.stats?.commits.count_by_type?.fix ?? 0}
                </TableCell>
                <TableCell className="text-right tabular-nums">
                  {feature.stats?.commits.count_by_type?.refactor ?? 0}
                </TableCell>
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
  )
}
