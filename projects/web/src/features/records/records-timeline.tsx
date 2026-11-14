import { Eye, EyeOff } from 'lucide-react'
import { useContext, useEffect, useMemo, useState } from 'react'
import { useSearchParams } from 'react-router'
import { HelpButton } from '@/components/help-button'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { FeaturesContext } from '@/lib/features-context'
import type { DocRecord } from '@/models/feature'
import { RecordCard, recordElementId } from './record-card'
import {
  countByKind,
  filterRecords,
  indexRecords,
  isInactive,
  type KindFilter,
  sortRecords,
} from './record-utils'

const FAQ_URL =
  'https://github.com/interaction-dynamics/features/blob/master/FAQ.md'

const KIND_FILTERS: { value: KindFilter; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'spec', label: 'Specs' },
  { value: 'decision', label: 'Decisions' },
]

function EmptyState({ scope }: { scope: 'feature' | 'project' }) {
  return (
    <Card>
      <CardContent>
        <div className="flex flex-col items-start gap-2">
          <p className="text-sm font-medium">No specs or decisions yet</p>
          {scope === 'feature' ? (
            <p className="text-sm text-muted-foreground">
              Add specs (what to build) in the feature's{' '}
              <code className="font-mono text-xs">__docs__/specs/</code> folder
              and decision records (why it's built this way) in{' '}
              <code className="font-mono text-xs">__docs__/decisions/</code>.
              Root specs, GitHub Spec Kit and Kiro specs show up here when they
              match the feature.
            </p>
          ) : (
            <p className="text-sm text-muted-foreground">
              Specs and decisions at the root of the project (
              <code className="font-mono text-xs">docs/specs/</code>,{' '}
              <code className="font-mono text-xs">docs/decisions/</code>,{' '}
              <code className="font-mono text-xs">docs/adr/</code>, Spec Kit's{' '}
              <code className="font-mono text-xs">specs/</code> and{' '}
              <code className="font-mono text-xs">.kiro/specs/</code>) that are
              not attached to a feature show up here.
            </p>
          )}
          <div className="flex flex-wrap gap-4">
            <HelpButton
              title="How to add a spec"
              url={`${FAQ_URL}#how-can-i-add-a-spec`}
            />
            <HelpButton
              title="How to add a decision"
              url={`${FAQ_URL}#how-can-i-add-a-decision`}
            />
          </div>
        </div>
      </CardContent>
    </Card>
  )
}

interface RecordsTimelineProps {
  records: DocRecord[]
  /** Feature the records belong to, `null` for project-level records */
  featurePath: string | null
}

export function RecordsTimeline({
  records,
  featurePath,
}: RecordsTimelineProps) {
  const { features, projectRecords } = useContext(FeaturesContext)
  const [searchParams] = useSearchParams()
  const selectedPath = searchParams.get('record')

  const [kind, setKind] = useState<KindFilter>('all')
  const [hideInactive, setHideInactive] = useState(false)
  const [openPaths, setOpenPaths] = useState<Set<string>>(
    () => new Set(selectedPath ? [selectedPath] : []),
  )

  const index = useMemo(
    () => indexRecords(features, projectRecords),
    [features, projectRecords],
  )
  const sorted = useMemo(() => sortRecords(records), [records])
  const visible = filterRecords(sorted, kind, hideInactive)
  const counts = countByKind(records)
  const inactiveCount = records.filter(isInactive).length

  // Open and scroll to the record of the URL (from a link chip)
  useEffect(() => {
    if (!selectedPath) return
    const selected = records.find((r) => r.path === selectedPath)
    if (!selected) return
    setOpenPaths((paths) => new Set(paths).add(selectedPath))
    setKind((current) =>
      current === 'all' || current === selected.kind ? current : 'all',
    )
    if (isInactive(selected)) setHideInactive(false)
    requestAnimationFrame(() => {
      document
        .getElementById(recordElementId(selected))
        ?.scrollIntoView({ behavior: 'smooth', block: 'start' })
    })
  }, [selectedPath, records])

  if (records.length === 0) {
    return <EmptyState scope={featurePath === null ? 'project' : 'feature'} />
  }

  const setOpen = (path: string, open: boolean) =>
    setOpenPaths((paths) => {
      const next = new Set(paths)
      if (open) next.add(path)
      else next.delete(path)
      return next
    })

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-wrap items-center gap-2">
        {KIND_FILTERS.map((filter) => (
          <Button
            key={filter.value}
            size="sm"
            variant={kind === filter.value ? 'secondary' : 'outline'}
            aria-pressed={kind === filter.value}
            onClick={() => setKind(filter.value)}
          >
            {filter.label}
            <span className="tabular-nums text-muted-foreground">
              {counts[filter.value]}
            </span>
          </Button>
        ))}
        {inactiveCount > 0 && (
          <Button
            size="sm"
            variant={hideInactive ? 'secondary' : 'ghost'}
            aria-pressed={hideInactive}
            onClick={() => setHideInactive((hide) => !hide)}
            className="ml-auto"
          >
            {hideInactive ? (
              <EyeOff className="h-4 w-4" />
            ) : (
              <Eye className="h-4 w-4" />
            )}
            {hideInactive
              ? `Show superseded and rejected (${inactiveCount})`
              : 'Hide superseded and rejected'}
          </Button>
        )}
      </div>
      {visible.length === 0 ? (
        <p className="text-sm text-muted-foreground">
          No spec or decision matches these filters.
        </p>
      ) : (
        <div className="flex flex-col gap-3">
          {visible.map((record) => (
            <RecordCard
              key={record.path}
              record={record}
              featurePath={featurePath}
              index={index}
              open={openPaths.has(record.path)}
              onOpenChange={(open) => setOpen(record.path, open)}
              highlighted={record.path === selectedPath}
            />
          ))}
        </div>
      )}
    </div>
  )
}
