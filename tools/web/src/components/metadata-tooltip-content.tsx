interface MetadataTooltipContentProps {
  items: Record<string, string>[]
  metadataKey: string
}

export function MetadataTooltipContent({
  items,
  metadataKey,
}: MetadataTooltipContentProps) {
  return (
    <div className="space-y-1">
      <p className="font-semibold mb-2 capitalize">
        {metadataKey} ({items.length})
      </p>
      <div className="space-y-1 text-xs">
        {items.slice(0, 10).map((item, idx) => {
          // Check for identifier properties
          const identifierKey = ['name', 'title', 'key', 'id'].find(
            (k) => k in item,
          )

          if (identifierKey) {
            // Display only the identifier property
            const value = item[identifierKey]
            return (
              // biome-ignore lint/suspicious/noArrayIndexKey: no other choice
              <div key={idx}>
                {value.length > 50 ? `${value.substring(0, 50)}...` : value}
              </div>
            )
          }

          // Display all properties
          return (
            // biome-ignore lint/suspicious/noArrayIndexKey: no other choice
            <div key={idx}>
              {Object.entries(item)
                .filter(([k]) => k !== 'feature')
                .slice(0, 3)
                .map(
                  ([k, v]) =>
                    `${k}: ${v.length > 20 ? v.substring(0, 20) + '...' : v}`,
                )
                .join(', ')}
            </div>
          )
        })}
        {items.length > 10 && <div>+{items.length - 10} more</div>}
      </div>
    </div>
  )
}
