import { getOwnerColor } from '@/lib/owner-color'
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip'

type OwnerDotProps = {
  owner: string
}

export function OwnerDot({ owner }: OwnerDotProps) {
  const color = getOwnerColor(owner)
  const isEmpty = !owner || owner.trim() === ''

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <div
          className="size-2 rounded-full shrink-0"
          style={{ backgroundColor: color }}
        >
          <span className="sr-only">
            {isEmpty ? 'No owner' : `Owner: ${owner}`}
          </span>
        </div>
      </TooltipTrigger>
      <TooltipContent side="right" sideOffset={8}>
        {isEmpty ? 'No owner' : owner}
      </TooltipContent>
    </Tooltip>
  )
}
