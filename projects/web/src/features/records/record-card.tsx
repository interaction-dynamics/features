import { ChevronRight, FileText, Lightbulb, Link2Off } from 'lucide-react'
import { useNavigate } from 'react-router'
import { MarkdownRenderer } from '@/components/markdown-renderer'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent } from '@/components/ui/card'
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { cn } from '@/lib/utils'
import type { DocRecord, RecordLinks } from '@/models/feature'
import {
  fileMarkdown,
  type LocatedRecord,
  recordLocation,
  resolveLink,
  SOURCE_LABELS,
  type StatusTone,
  statusTone,
  withoutTitle,
} from './record-utils'

const TONE_CLASSES: Record<StatusTone, string> = {
  positive:
    'border-transparent bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-300',
  progress:
    'border-transparent bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300',
  inactive: 'border-transparent bg-muted text-muted-foreground',
  neutral: '',
}

const LINK_LABELS: Record<keyof RecordLinks, string> = {
  specs: 'Specs',
  decisions: 'Decisions',
  supersedes: 'Supersedes',
  superseded_by: 'Superseded by',
}

const LINK_ORDER: (keyof RecordLinks)[] = [
  'specs',
  'decisions',
  'supersedes',
  'superseded_by',
]

export const recordElementId = (record: DocRecord) =>
  `record-${encodeURIComponent(record.path)}`

function TaskProgress({ done, total }: { done: number; total: number }) {
  const percent = total === 0 ? 0 : Math.round((done / total) * 100)
  return (
    <div
      className="flex items-center gap-2"
      title={`${done} of ${total} tasks done`}
    >
      <div className="h-1.5 w-24 overflow-hidden rounded-full bg-muted">
        <div
          className={cn(
            'h-full rounded-full',
            done === total ? 'bg-green-600' : 'bg-primary',
          )}
          style={{ width: `${percent}%` }}
        />
      </div>
      <span className="text-xs tabular-nums text-muted-foreground">
        {done}/{total} tasks
      </span>
    </div>
  )
}

function LinkChips({
  record,
  featurePath,
  index,
}: {
  record: DocRecord
  featurePath: string | null
  index: LocatedRecord[]
}) {
  const navigate = useNavigate()
  const groups = LINK_ORDER.filter((key) => record.links[key]?.length > 0)
  if (groups.length === 0) return null

  return (
    <div className="flex flex-wrap items-center gap-x-3 gap-y-1">
      {groups.map((key) => (
        <div key={key} className="flex flex-wrap items-center gap-1">
          <span className="text-xs text-muted-foreground">
            {LINK_LABELS[key]}:
          </span>
          {record.links[key].map((id) => {
            const target = resolveLink(id, featurePath, index)
            if (!target) {
              return (
                <span
                  key={id}
                  className="inline-flex items-center gap-1 rounded-md border border-dashed px-1.5 py-0.5 text-xs text-muted-foreground"
                  title="No spec or decision has this id"
                >
                  <Link2Off className="h-3 w-3" />
                  {id}
                </span>
              )
            }
            return (
              <button
                key={id}
                type="button"
                onClick={() => navigate(recordLocation(target))}
                className="inline-flex items-center gap-1 rounded-md border px-1.5 py-0.5 text-xs hover:bg-accent cursor-pointer"
                title={`${target.record.path}${
                  target.featurePath !== featurePath
                    ? target.featurePath
                      ? ` (feature ${target.featurePath})`
                      : ' (project)'
                    : ''
                }`}
              >
                {target.record.kind === 'spec' ? (
                  <FileText className="h-3 w-3" />
                ) : (
                  <Lightbulb className="h-3 w-3" />
                )}
                {target.record.title}
              </button>
            )
          })}
        </div>
      ))}
    </div>
  )
}

function RecordBody({ record }: { record: DocRecord }) {
  const files = record.files ?? []
  if (files.length === 0) {
    return <MarkdownRenderer markdown={withoutTitle(record.content)} />
  }

  const mainName = record.path.split('/').pop() ?? 'main'
  return (
    <Tabs defaultValue={mainName}>
      <TabsList className="flex-wrap h-auto">
        <TabsTrigger value={mainName}>{mainName}</TabsTrigger>
        {files.map((file) => (
          <TabsTrigger key={file.name} value={file.name}>
            {file.name}
          </TabsTrigger>
        ))}
      </TabsList>
      <TabsContent value={mainName}>
        <MarkdownRenderer markdown={withoutTitle(record.content)} />
      </TabsContent>
      {files.map((file) => (
        <TabsContent key={file.name} value={file.name}>
          <MarkdownRenderer markdown={fileMarkdown(file.name, file.content)} />
        </TabsContent>
      ))}
    </Tabs>
  )
}

interface RecordCardProps {
  record: DocRecord
  featurePath: string | null
  index: LocatedRecord[]
  open: boolean
  onOpenChange: (open: boolean) => void
  highlighted?: boolean
}

export function RecordCard({
  record,
  featurePath,
  index,
  open,
  onOpenChange,
  highlighted = false,
}: RecordCardProps) {
  const Icon = record.kind === 'spec' ? FileText : Lightbulb
  const sourceLabel = SOURCE_LABELS[record.source]
  const tone = statusTone(record.status)

  return (
    <Card
      id={recordElementId(record)}
      data-testid="record-card"
      className={cn(
        'scroll-mt-20 py-4',
        highlighted && 'ring-2 ring-primary',
        tone === 'inactive' && 'opacity-75',
      )}
    >
      <Collapsible open={open} onOpenChange={onOpenChange}>
        <CardContent className="flex flex-col gap-2">
          <CollapsibleTrigger className="w-full group cursor-pointer">
            <div className="flex items-start gap-3">
              <Icon className="h-5 w-5 text-primary flex-shrink-0 mt-0.5" />
              <div className="flex-1 text-left">
                <p className="text-sm font-medium leading-relaxed">
                  {record.title}
                </p>
                <div className="mt-1 flex flex-wrap items-center gap-1.5">
                  <Badge variant="outline">
                    {record.kind === 'spec' ? 'Spec' : 'ADR'}
                  </Badge>
                  {record.status && (
                    <Badge
                      variant={tone === 'neutral' ? 'outline' : 'secondary'}
                      className={TONE_CLASSES[tone]}
                    >
                      {record.status}
                    </Badge>
                  )}
                  {sourceLabel && (
                    <Badge variant="outline" className="text-muted-foreground">
                      {sourceLabel}
                    </Badge>
                  )}
                  {record.date && (
                    <span className="text-xs text-muted-foreground">
                      {record.date}
                    </span>
                  )}
                  <span className="text-xs font-mono text-muted-foreground">
                    {record.id}
                  </span>
                </div>
              </div>
              <ChevronRight className="h-4 w-4 text-muted-foreground transition-transform duration-200 group-data-[state=open]:rotate-90 flex-shrink-0 mt-0.5" />
            </div>
          </CollapsibleTrigger>
          {(record.tasks || hasLinks(record)) && (
            <div className="ml-8 flex flex-col gap-1.5">
              {record.tasks && (
                <TaskProgress
                  done={record.tasks.done}
                  total={record.tasks.total}
                />
              )}
              <LinkChips
                record={record}
                featurePath={featurePath}
                index={index}
              />
            </div>
          )}
        </CardContent>
        <CollapsibleContent>
          <CardContent className="pt-3 pb-1">
            <p className="mb-2 text-xs font-mono text-muted-foreground">
              {record.path}
            </p>
            <RecordBody record={record} />
          </CardContent>
        </CollapsibleContent>
      </Collapsible>
    </Card>
  )
}

const hasLinks = (record: DocRecord) =>
  LINK_ORDER.some((key) => record.links[key]?.length > 0)
