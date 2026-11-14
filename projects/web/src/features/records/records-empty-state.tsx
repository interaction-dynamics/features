import { ClipboardList, Lightbulb } from 'lucide-react'
import { HelpButton } from '@/components/help-button'
import { Card, CardContent } from '@/components/ui/card'

const HELP_URL =
  'https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-spec-or-a-decision'

interface RecordsEmptyStateProps {
  /** Where records are read from, e.g. `<feature>/__docs__` or `docs` */
  folder: string
}

export function RecordsEmptyState({ folder }: RecordsEmptyStateProps) {
  return (
    <Card>
      <CardContent className="flex flex-col gap-4">
        <p className="text-sm text-muted-foreground">
          No specs or decisions yet.
        </p>
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="flex items-start gap-3">
            <ClipboardList className="h-5 w-5 text-sky-600 dark:text-sky-400 flex-shrink-0 mt-0.5" />
            <div className="text-sm">
              <p className="font-medium">Specs: what to build</p>
              <p className="text-muted-foreground">
                Add{' '}
                <code className="font-mono">{folder}/specs/001-title.md</code>{' '}
                with a <code className="font-mono">status</code> and{' '}
                <code className="font-mono">date</code> frontmatter, or a{' '}
                <code className="font-mono">specs/001-title/</code> folder with
                a <code className="font-mono">spec.md</code>.
              </p>
            </div>
          </div>
          <div className="flex items-start gap-3">
            <Lightbulb className="h-5 w-5 text-violet-600 dark:text-violet-400 flex-shrink-0 mt-0.5" />
            <div className="text-sm">
              <p className="font-medium">Decisions: why it is built that way</p>
              <p className="text-muted-foreground">
                Add a MADR file in{' '}
                <code className="font-mono">
                  {folder}/decisions/001-title.md
                </code>
                . Link it to its spec with{' '}
                <code className="font-mono">spec: 001-title</code>.
              </p>
            </div>
          </div>
        </div>
        <p className="text-xs text-muted-foreground">
          Or run <code className="font-mono">features new spec|decision</code>.
        </p>
        <HelpButton title="How to add specs and decisions" url={HELP_URL} />
      </CardContent>
    </Card>
  )
}
