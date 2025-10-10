import { ExternalLink } from 'lucide-react'

interface MetaValueProps {
  metaKey: string
  value: unknown
}

export function MetaValue({ metaKey, value }: MetaValueProps) {
  const isUrl = (value: string): boolean => {
    if (typeof value !== 'string' || value.length < 4) {
      return false
    }

    // Check for common URL patterns
    const urlPattern =
      /^https?:\/\/(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&//=]*)$/

    try {
      const url = new URL(value)
      return (
        (url.protocol === 'http:' || url.protocol === 'https:') &&
        urlPattern.test(value)
      )
    } catch {
      return false
    }
  }

  const stringValue =
    typeof value === 'object' ? JSON.stringify(value) : String(value)

  if (typeof value === 'string' && isUrl(stringValue)) {
    return (
      <a
        href={stringValue}
        target="_blank"
        rel="noopener noreferrer"
        className="bg-blue-50 hover:bg-blue-100 dark:bg-blue-900/20 dark:hover:bg-blue-900/40 text-blue-700 dark:text-blue-400 border border-blue-200 dark:border-blue-800 rounded-md px-2 py-1 text-xs transition-all inline-flex items-center gap-1.5 group hover:shadow-sm"
        title={`Open ${stringValue} in new tab`}
      >
        <span className="font-semibold">{metaKey}</span>
        <ExternalLink className="h-3 w-3 opacity-70 group-hover:opacity-100 transition-opacity" />
      </a>
    )
  }

  return (
    <div className="bg-muted rounded-md px-2 py-1 text-xs">
      <span className="font-semibold">{metaKey}:</span> {stringValue}
    </div>
  )
}
