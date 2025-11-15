import { useMetadata } from '@/hooks/use-metadata'

export function VersionIndicator() {
  const { metadata } = useMetadata()

  const version = metadata?.version || ''

  if (!version) return null

  return (
    <div>
      <span className="text-muted-foreground/60 text-xs">v{version}</span>
    </div>
  )
}
