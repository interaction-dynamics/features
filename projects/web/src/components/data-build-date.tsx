import { useMetadata } from '@/hooks/use-metadata'
import { formatDateTime } from '@/lib/format-date'

export function DataBuildDate() {
  const { metadata } = useMetadata()

  const builtAt = metadata?.builtAt

  if (!builtAt) return null

  return (
    <time
      dateTime={builtAt}
      className="hidden md:inline text-muted-foreground text-xs whitespace-nowrap"
    >
      Data built {formatDateTime(builtAt)}
    </time>
  )
}
