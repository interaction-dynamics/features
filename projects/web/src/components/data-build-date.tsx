import { useMetadata } from '@/hooks/use-metadata'
import { formatDate, formatDateTime } from '@/lib/format-date'
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip'

export function DataBuildDate() {
  const { metadata } = useMetadata()

  const builtAt = metadata?.builtAt

  if (!builtAt) return null

  return (
    <Tooltip>
      <TooltipTrigger className="hidden md:inline text-muted-foreground text-xs whitespace-nowrap">
        <time dateTime={builtAt}>Data built {formatDate(builtAt)}</time>
      </TooltipTrigger>
      <TooltipContent side="bottom" sideOffset={8}>
        {formatDateTime(builtAt)}
      </TooltipContent>
    </Tooltip>
  )
}
