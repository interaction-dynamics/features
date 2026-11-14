import { RecordsEmptyState } from './records-empty-state'
import { RecordsTimeline } from './records-timeline'
import { useProjectRecords } from './use-project-records'

/** Project-level specs and decisions: root `docs/` records not attached to a feature */
export function ProjectRecords() {
  const { records, isLoading } = useProjectRecords()

  return (
    <div className="flex flex-col gap-4">
      <p className="text-sm text-muted-foreground">
        Records of <code className="font-mono">docs/specs</code>,{' '}
        <code className="font-mono">docs/decisions</code> and{' '}
        <code className="font-mono">docs/adr</code> that are not attached to a
        feature with a <code className="font-mono">feature:</code> frontmatter.
        They usually cover the whole project or several features.
      </p>
      {isLoading ? null : records.length === 0 ? (
        <RecordsEmptyState folder="docs" />
      ) : (
        <RecordsTimeline records={records} />
      )}
    </div>
  )
}
