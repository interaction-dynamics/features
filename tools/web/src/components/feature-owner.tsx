import { resolveOwner } from '@/lib/resolve-owner'
import type { Feature } from '@/models/feature'
import { HelpButton } from './help-button'

type FeatureOwnerProps = {
  feature: Feature
}

export function FeatureOwner({ feature }: FeatureOwnerProps) {
  const resolvedOwner = resolveOwner(feature)
  const isInheritedOwner = resolvedOwner !== feature.owner

  return (
    <div className="flex items-center gap-2">
      <span>{resolvedOwner}</span>
      {isInheritedOwner && (
        <span className="text-xs px-1.5 py-0.5 bg-amber-50 text-amber-700 dark:bg-amber-900/20 dark:text-amber-400 rounded text-[10px] font-medium">
          inherited
        </span>
      )}
      {resolvedOwner === 'Unknown' && (
        <HelpButton
          tooltip="Learn how to add an owner"
          url="https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-an-owner"
          size="small"
        />
      )}
    </div>
  )
}
