import { Database } from 'lucide-react'
import { formatExternalValue, humanize } from '@/lib/external-tools'
import type { Feature } from '@/models/feature'

interface FeatureExternalDataProps {
  feature: Feature
}

/** Metrics imported from external tools with `features --data <file>` */
export function FeatureExternalData({ feature }: FeatureExternalDataProps) {
  const metrics = Object.entries(feature.external ?? {})

  if (metrics.length === 0) {
    return null
  }

  return (
    <div className="flex items-start gap-3">
      <Database className="h-4 w-4 text-muted-foreground mt-0.5" />
      <div className="flex-1">
        <p className="text-sm font-medium text-foreground mb-1">Data</p>
        <div className="flex flex-wrap gap-2">
          {metrics.map(([name, value]) => (
            <div
              key={name}
              className="bg-muted text-foreground border-border border rounded-md px-2 py-1 text-xs"
              title={name}
            >
              <span className="text-muted-foreground">{humanize(name)}</span>{' '}
              <span className="font-semibold tabular-nums">
                {formatExternalValue(value)}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
