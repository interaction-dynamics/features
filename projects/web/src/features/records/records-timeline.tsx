import {
  ChevronRight,
  ClipboardList,
  FileText,
  Lightbulb,
  Link2,
} from 'lucide-react'
import { useMemo, useState } from 'react'
import { MarkdownRenderer } from '@/components/markdown-renderer'
import { Card, CardContent } from '@/components/ui/card'
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible'
import { cn } from '@/lib/utils'
import type { FeatureRecord } from '@/models/feature'
import {
  contentWithoutTitle,
  filterRecords,
  findLinkTarget,
  KIND_LABELS,
  type KindFilter,
  type RecordLink,
  recordAnchorId,
  recordLinks,
  type StatusTone,
  sortRecords,
  statusTone,
  stripTitleHeading,
} from './utils'

const TONE_CLASSES: Record<StatusTone, string> = {
  neutral: 'bg-muted text-muted-foreground',
  progress: 'bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300',
  success:
    'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300',
  muted: 'bg-muted text-muted-foreground line-through decoration-1',
  danger: 'bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300',
}

const pillClass =
  'inline-flex items-center gap-1 rounded-md px-2 py-0.5 text-xs font-medium'

function KindBadge({ record }: { record: FeatureRecord }) {
  const Icon = record.kind === 'spec' ? ClipboardList : Lightbulb
  return (
    <span
      className={cn(
        pillClass,
        record.kind === 'spec'
          ? 'bg-sky-100 text-sky-800 dark:bg-sky-950 dark:text-sky-300'
          : 'bg-violet-100 text-violet-800 dark:bg-violet-950 dark:text-violet-300',
      )}
      data-testid="record-kind"
    >
      <Icon className="h-3 w-3" />
      {KIND_LABELS[record.kind]}
    </span>
  )
}

function StatusBadge({ status }: { status?: string }) {
  if (!status) {
    return (
      <span
        className={cn(pillClass, 'border border-dashed text-muted-foreground')}
      >
        no status
      </span>
    )
  }
  return (
    <span
      className={cn(pillClass, TONE_CLASSES[statusTone(status)])}
      data-testid="record-status"
    >
      {status}
    </span>
  )
}

function TaskProgressBar({ done, total }: { done: number; total: number }) {
  return (
    <div
      className="flex items-center gap-2 text-xs text-muted-foreground"
      title={`${done} of ${total} tasks done`}
    >
      <progress
        className={cn(
          'h-1.5 w-24 appearance-none overflow-hidden rounded-full bg-muted',
          '[&::-webkit-progress-bar]:bg-muted [&::-webkit-progress-value]:rounded-full',
          done === total
            ? '[&::-moz-progress-bar]:bg-emerald-500 [&::-webkit-progress-value]:bg-emerald-500'
            : '[&::-moz-progress-bar]:bg-primary [&::-webkit-progress-value]:bg-primary',
        )}
        value={done}
        max={total}
        aria-label="Tasks done"
      />
      <span className="tabular-nums">
        {done}/{total}
      </span>
    </div>
  )
}

interface RecordCardProps {
  record: FeatureRecord
  allRecords: FeatureRecord[]
  open: boolean
  onOpenChange: (open: boolean) => void
  onLinkClick: (target: FeatureRecord) => void
}

function RecordCard({
  record,
  allRecords,
  open,
  onOpenChange,
  onLinkClick,
}: RecordCardProps) {
  const links = recordLinks(record)

  const renderLink = (link: RecordLink) => {
    const target = findLinkTarget(allRecords, link)
    const key = `${link.label}-${link.targetId}`
    if (!target) {
      return (
        <span
          key={key}
          className={cn(
            pillClass,
            'border border-dashed text-muted-foreground',
          )}
          title="This record is not part of this list"
        >
          <Link2 className="h-3 w-3" />
          {link.label}: {link.targetId}
        </span>
      )
    }
    return (
      <button
        key={key}
        type="button"
        className={cn(
          pillClass,
          'border bg-background hover:bg-accent hover:text-accent-foreground cursor-pointer',
        )}
        onClick={() => onLinkClick(target)}
        title={target.title}
      >
        <Link2 className="h-3 w-3" />
        {link.label}: {target.title}
      </button>
    )
  }

  return (
    <Card
      id={recordAnchorId(record)}
      className="py-0 gap-0 scroll-mt-20"
      data-testid="record-card"
    >
      <Collapsible open={open} onOpenChange={onOpenChange}>
        <CollapsibleTrigger className="w-full group text-left cursor-pointer">
          <CardContent className="py-4">
            <div className="flex items-start gap-3">
              <div className="flex-1 min-w-0 flex flex-col gap-2">
                <div className="flex flex-wrap items-center gap-2">
                  <KindBadge record={record} />
                  <StatusBadge status={record.status} />
                  {record.date && (
                    <span className="text-xs text-muted-foreground tabular-nums">
                      {record.date}
                    </span>
                  )}
                </div>
                <p className="text-sm font-medium leading-relaxed">
                  {record.title}
                </p>
                {record.tasks && record.tasks.total > 0 && (
                  <TaskProgressBar
                    done={record.tasks.done}
                    total={record.tasks.total}
                  />
                )}
              </div>
              <ChevronRight className="h-4 w-4 text-muted-foreground transition-transform duration-200 group-data-[state=open]:rotate-90 flex-shrink-0 mt-0.5" />
            </div>
          </CardContent>
        </CollapsibleTrigger>
        {links.length > 0 && (
          <div className="flex flex-wrap gap-1.5 px-6 pb-4 -mt-1">
            {links.map(renderLink)}
          </div>
        )}
        <CollapsibleContent>
          <CardContent className="border-t pt-3 pb-4">
            {record.path && (
              <p className="mb-2 text-xs font-mono text-muted-foreground">
                {record.path}
              </p>
            )}
            <MarkdownRenderer markdown={contentWithoutTitle(record)} />
            {record.files?.map((file) => (
              <div key={file.path} className="mt-4 border-t pt-3">
                <p className="mb-2 flex items-center gap-1 text-xs font-mono text-muted-foreground">
                  <FileText className="h-3 w-3" />
                  {file.name}
                </p>
                <MarkdownRenderer
                  markdown={stripTitleHeading(
                    file.content,
                    file.name.replace(/\.mdx?$/i, ''),
                  )}
                />
              </div>
            ))}
          </CardContent>
        </CollapsibleContent>
      </Collapsible>
    </Card>
  )
}

function FilterChip({
  active,
  onClick,
  children,
}: {
  active: boolean
  onClick: () => void
  children: React.ReactNode
}) {
  return (
    <button
      type="button"
      aria-pressed={active}
      onClick={onClick}
      className={cn(
        'rounded-full border px-3 py-1 text-xs font-medium transition-colors cursor-pointer',
        active
          ? 'bg-primary text-primary-foreground border-primary'
          : 'bg-background text-muted-foreground hover:text-foreground',
      )}
    >
      {children}
    </button>
  )
}

interface RecordsTimelineProps {
  records: FeatureRecord[]
}

/**
 * Specs and decisions merged in one timeline, newest first, with filters and
 * link chips that open the linked record
 */
export function RecordsTimeline({ records }: RecordsTimelineProps) {
  const [kind, setKind] = useState<KindFilter>('all')
  const [hideInactive, setHideInactive] = useState(false)
  const [openAnchors, setOpenAnchors] = useState<Set<string>>(new Set())

  const sorted = useMemo(() => sortRecords(records), [records])
  const visible = filterRecords(sorted, kind, hideInactive)
  const specsCount = records.filter((r) => r.kind === 'spec').length
  const decisionsCount = records.length - specsCount

  const setOpen = (anchor: string, open: boolean) =>
    setOpenAnchors((current) => {
      const next = new Set(current)
      if (open) next.add(anchor)
      else next.delete(anchor)
      return next
    })

  const openLinked = (target: FeatureRecord) => {
    if (!visible.includes(target)) {
      setKind('all')
      setHideInactive(false)
    }
    const anchor = recordAnchorId(target)
    setOpen(anchor, true)
    requestAnimationFrame(() =>
      document
        .getElementById(anchor)
        ?.scrollIntoView({ behavior: 'smooth', block: 'start' }),
    )
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap items-center gap-2">
        <FilterChip active={kind === 'all'} onClick={() => setKind('all')}>
          All ({records.length})
        </FilterChip>
        <FilterChip active={kind === 'spec'} onClick={() => setKind('spec')}>
          Specs ({specsCount})
        </FilterChip>
        <FilterChip
          active={kind === 'decision'}
          onClick={() => setKind('decision')}
        >
          Decisions ({decisionsCount})
        </FilterChip>
        <span className="mx-1 h-4 w-px bg-border" />
        <FilterChip
          active={hideInactive}
          onClick={() => setHideInactive((value) => !value)}
        >
          Hide superseded/rejected
        </FilterChip>
      </div>
      {visible.length === 0 ? (
        <p className="text-sm text-muted-foreground">
          No record matches these filters.
        </p>
      ) : (
        visible.map((record) => {
          const anchor = recordAnchorId(record)
          return (
            <RecordCard
              key={`${anchor}-${record.path}`}
              record={record}
              allRecords={sorted}
              open={openAnchors.has(anchor)}
              onOpenChange={(open) => setOpen(anchor, open)}
              onLinkClick={openLinked}
            />
          )
        })
      )}
    </div>
  )
}
