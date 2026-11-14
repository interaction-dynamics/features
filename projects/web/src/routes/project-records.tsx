import { ScrollText } from 'lucide-react'
import { Header } from '@/components/header'
import {
  RecordsEmptyState,
  RecordsTimeline,
  useProjectRecords,
} from '@/features/records'

/** Project-level specs and decisions: root `docs/` records not attached to a feature */
export default function ProjectRecords() {
  const { records, isLoading } = useProjectRecords()

  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>
        <div className="flex items-center gap-2 font-medium">
          <ScrollText className="h-4 w-4" />
          Project specs & decisions
        </div>
      </Header>
      <div className="flex flex-1 flex-col gap-4 p-4">
        <p className="text-sm text-muted-foreground">
          Records of <code className="font-mono">docs/specs</code>,{' '}
          <code className="font-mono">docs/decisions</code> and{' '}
          <code className="font-mono">docs/adr</code> that are not attached to a
          feature with a <code className="font-mono">feature:</code>{' '}
          frontmatter.
        </p>
        {isLoading ? null : records.length === 0 ? (
          <RecordsEmptyState folder="docs" />
        ) : (
          <RecordsTimeline records={records} />
        )}
      </div>
    </div>
  )
}
