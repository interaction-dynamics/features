import { FlaskConical } from 'lucide-react'
import { cn } from '@/lib/utils'

type ExperimentalBadgeProps = {
  className?: string
  /** Only show the icon (e.g. in the sidebar) */
  compact?: boolean
}

/** Marks a page or a link of the dashboard as experimental */
export function ExperimentalBadge({
  className,
  compact = false,
}: ExperimentalBadgeProps) {
  return (
    <span
      title="Experimental: this page may change or be removed"
      className={cn(
        'inline-flex items-center gap-1 rounded-md border border-amber-500/40 bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide text-amber-700 dark:text-amber-400',
        className,
      )}
    >
      <FlaskConical className="h-3 w-3" aria-hidden="true" />
      {compact ? <span className="sr-only">Experimental</span> : 'Experimental'}
    </span>
  )
}
