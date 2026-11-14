import { CircleCheck, Repeat, Split } from 'lucide-react'
import { useContext } from 'react'
import { useNavigate } from 'react-router'
import { StatsCard } from '@/components/stats-card'
import { Badge } from '@/components/ui/badge'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { FeaturesContext } from '@/lib/features-context'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { ReconciliationError } from '@/models/feature'

const ERROR_LABELS: Record<ReconciliationError['type'], string> = {
  cycle: 'Loop',
  conflicting_parents: 'Conflicting parents',
}

function ReconciliationErrorCard({ error }: { error: ReconciliationError }) {
  const navigate = useNavigate()
  const Icon = error.type === 'cycle' ? Repeat : Split

  return (
    <section
      className="rounded-lg border bg-card p-4"
      data-testid="reconciliation-error"
    >
      <div className="flex flex-wrap items-center gap-2">
        <Icon className="h-4 w-4 text-amber-600 dark:text-amber-400" />
        <h3 className="font-semibold">
          {error.features.map(formatFeatureName).join(', ')}
        </h3>
        <Badge variant="outline">
          {ERROR_LABELS[error.type] ?? error.type}
        </Badge>
      </div>
      <p className="mt-2 text-sm text-muted-foreground">{error.message}</p>
      <Table className="mt-3">
        <TableHeader>
          <TableRow>
            <TableHead>Feature</TableHead>
            <TableHead>Repository</TableHead>
            <TableHead>Path in repository</TableHead>
            <TableHead>Parent in repository</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {error.occurrences.map((occurrence) => (
            <TableRow
              key={occurrence.featurePath}
              className="cursor-pointer"
              onClick={() =>
                navigate({
                  pathname: '/',
                  search: `?feature=${occurrence.featurePath}`,
                })
              }
            >
              <TableCell className="font-medium text-primary">
                {formatFeatureName(occurrence.feature)}
              </TableCell>
              <TableCell>{occurrence.repository}</TableCell>
              <TableCell className="font-mono text-xs">
                {occurrence.path}
              </TableCell>
              <TableCell>
                {occurrence.parent ? (
                  formatFeatureName(occurrence.parent)
                ) : (
                  <span className="text-muted-foreground">none (root)</span>
                )}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </section>
  )
}

/** Features of an aggregated report that could not be merged across repositories */
export function UnmergedFeatures() {
  const { reconciliationErrors, repositories } = useContext(FeaturesContext)

  const cycles = reconciliationErrors.filter((e) => e.type === 'cycle').length

  return (
    <div className="flex flex-col gap-4">
      <p className="text-sm text-muted-foreground">
        Features with the same name are merged across repositories. These
        features could not be merged and are kept separately, once per
        repository.
      </p>
      <div className="grid gap-4 md:grid-cols-3">
        <StatsCard title="Repositories" value={repositories.length} />
        <StatsCard title="Loops" value={cycles} />
        <StatsCard
          title="Conflicting parents"
          value={reconciliationErrors.length - cycles}
        />
      </div>
      {reconciliationErrors.length === 0 ? (
        <div className="flex items-center gap-2 py-8 text-muted-foreground">
          <CircleCheck className="h-5 w-5 text-green-600" />
          All features were merged without issues.
        </div>
      ) : (
        reconciliationErrors.map((error) => (
          <ReconciliationErrorCard
            key={`${error.type}:${error.features.join(',')}`}
            error={error}
          />
        ))
      )}
    </div>
  )
}
