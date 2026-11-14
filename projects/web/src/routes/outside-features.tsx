import { FileText, Shapes, TriangleAlert } from 'lucide-react'
import { useContext } from 'react'
import { Navigate } from 'react-router'
import { ExperimentalBadge } from '@/components/experimental-badge'
import { Header } from '@/components/header'
import { UnmergedFeatures } from '@/features/reconciliation/unmerged-features'
import { ProjectRecords, useProjectRecords } from '@/features/records'
import { useExperimentalFeatures } from '@/hooks/use-experimental-features'
import { FeaturesContext } from '@/lib/features-context'

/**
 * What does not sit inside the feature tree: project-level records not
 * attached to a feature, and features that could not be merged across
 * repositories (aggregated reports only).
 */
export default function OutsideFeatures() {
  const { isAggregated, reconciliationErrors } = useContext(FeaturesContext)
  const { records } = useProjectRecords()
  const { enabled: experimentalEnabled, isLoading } = useExperimentalFeatures()

  // Project-level records are experimental. records.json is not written for
  // aggregated reports: hide the empty section
  const showRecords =
    experimentalEnabled && (!isAggregated || records.length > 0)

  // Nothing to show: a single-repository report without experimental pages
  if (!isAggregated && !experimentalEnabled) {
    return isLoading ? null : <Navigate to="/" replace />
  }

  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>
        <div className="flex items-center gap-2 font-medium">
          <Shapes className="h-4 w-4" />
          Outside features
        </div>
      </Header>
      <div className="flex flex-1 flex-col gap-8 p-4">
        {showRecords && (
          <section
            className="flex flex-col gap-3"
            data-testid="outside-features-records"
          >
            <h2 className="flex items-center gap-2 text-lg font-semibold">
              <FileText className="h-4 w-4" />
              Not attached to a feature
              <span className="rounded bg-secondary px-1.5 text-xs font-normal tabular-nums text-secondary-foreground">
                {records.length}
              </span>
              <ExperimentalBadge />
            </h2>
            <ProjectRecords />
          </section>
        )}
        {isAggregated && (
          <section
            className="flex flex-col gap-3"
            data-testid="outside-features-unmerged"
          >
            <h2 className="flex items-center gap-2 text-lg font-semibold">
              <TriangleAlert
                className={
                  reconciliationErrors.length > 0
                    ? 'h-4 w-4 text-amber-600 dark:text-amber-400'
                    : 'h-4 w-4'
                }
              />
              Not merged across repositories
              <span className="rounded bg-secondary px-1.5 text-xs font-normal tabular-nums text-secondary-foreground">
                {reconciliationErrors.length}
              </span>
            </h2>
            <UnmergedFeatures />
          </section>
        )}
      </div>
    </div>
  )
}
