import type { Feature } from '@/models/feature'
import { HelpButton } from './help-button'
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip'

type FeatureOwnerProps = {
  feature: Feature
}

export function FeatureOwner({ feature }: FeatureOwnerProps) {
  return (
    <div className="flex items-center gap-2">
      <span>{feature.owner || 'Unknown'}</span>
      {feature.is_owner_inherited && (
        <Tooltip>
          <TooltipTrigger>
            <span className="text-xs px-1.5 py-0.5 bg-amber-50 text-amber-700 dark:bg-amber-900/20 dark:text-amber-400 rounded text-[10px] font-medium">
              inherited
            </span>
          </TooltipTrigger>
          <TooltipContent>
            The owner is inherited from a parent feature since the owner is not
            defined in this feature
          </TooltipContent>
        </Tooltip>
      )}
      {feature.owner === '' && (
        <HelpButton
          tooltip="Learn how to add an owner"
          url="https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-an-owner"
          size="small"
        />
      )}
    </div>
  )
}
