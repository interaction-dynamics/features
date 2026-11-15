import { ClipboardList, Lightbulb, ScrollText } from 'lucide-react'
import { isInactive, KIND_LABELS } from '@/features/records/utils'
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import type { OwnedRecord } from './utils'

type OwnerRecordsProps = {
  records: OwnedRecord[]
  onOpenFeature: (feature: Feature) => void
}

/** Specs and decisions of an owner's features, newest first */
export function OwnerRecords({ records, onOpenFeature }: OwnerRecordsProps) {
  if (records.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center gap-2 py-12 text-center">
        <ScrollText className="h-10 w-10 text-muted-foreground" />
        <p className="text-sm font-medium">No specs or decisions</p>
        <p className="max-w-sm text-sm text-muted-foreground">
          Add them to the <code>__docs__/specs</code> and{' '}
          <code>__docs__/decisions</code> folders of the features.
        </p>
      </div>
    )
  }

  return (
    <ul className="divide-y rounded-md border" data-testid="owner-records">
      {records.map(({ record, feature }) => {
        const Icon = record.kind === 'spec' ? ClipboardList : Lightbulb
        return (
          <li
            key={`${feature.path}:${record.kind}:${record.id}`}
            className="flex flex-wrap items-center gap-x-3 gap-y-1 px-3 py-2.5 text-sm"
          >
            <span className="inline-flex w-14 shrink-0 items-center gap-1 text-xs text-muted-foreground">
              <Icon className="h-3.5 w-3.5" />
              {KIND_LABELS[record.kind]}
            </span>
            <span
              className={cn(
                'min-w-0 flex-1 truncate font-medium',
                isInactive(record) &&
                  'text-muted-foreground line-through decoration-1',
              )}
              title={record.path}
            >
              {record.title}
            </span>
            {record.status && (
              <span className="rounded-md border px-1.5 py-0.5 text-xs capitalize text-muted-foreground">
                {record.status}
              </span>
            )}
            <button
              type="button"
              className="text-xs text-muted-foreground underline-offset-4 hover:text-foreground hover:underline"
              onClick={() => onOpenFeature(feature)}
              title={`Open ${formatFeatureName(feature.name)}`}
            >
              {formatFeatureName(feature.name)}
            </button>
            <span className="w-24 shrink-0 text-right text-xs tabular-nums text-muted-foreground">
              {record.date ? formatDate(record.date) : '—'}
            </span>
          </li>
        )
      })}
    </ul>
  )
}
