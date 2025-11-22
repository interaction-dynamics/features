import { Info } from 'lucide-react'
import { formatDate } from '@/lib/format-date'
import type { Feature } from '@/models/feature'
import { MetaValue } from './meta-value'

interface FeatureMetaProps {
  feature: Feature
}

function findCreationDate(feature: Feature): string {
  const creation_date = feature?.meta?.creation_date

  if (
    creation_date &&
    typeof creation_date === 'string' &&
    /\d{4}-\d{2}-\d{2}/.test(creation_date)
  ) {
    return creation_date
  }

  return feature.changes[0].date
}

export function FeatureMeta({ feature }: FeatureMetaProps) {
  const creationDate = findCreationDate(feature)

  const metas = [
    creationDate ? ['creation_date', formatDate(creationDate)] : null,
    ...Object.entries(feature?.meta ?? {}).filter(
      ([key]) => key !== 'creation_date',
    ),
  ].filter(Boolean)

  return (
    <div className="flex items-start gap-3">
      <Info className="h-4 w-4 text-muted-foreground mt-0.5" />
      <div className="flex-1">
        <p className="text-sm font-medium text-foreground mb-1">Meta</p>
        <div className="text-xs font-mono text-muted-foreground flex flex-wrap gap-2">
          {metas.map(([key, value]) => (
            <MetaValue key={key} metaKey={key} value={value} />
          ))}
        </div>
      </div>
    </div>
  )
}
