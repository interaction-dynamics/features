import { ExternalLink } from 'lucide-react'

interface MetaValueProps {
  metaKey: string
  value: unknown
}

interface ColorScheme {
  bg: string
  hoverBg: string
  text: string
  border: string
}

function getColorScheme(metaKey: string, value: unknown): ColorScheme {
  const key = metaKey.toLowerCase()
  const stringValue = typeof value === 'string' ? value.toLowerCase() : ''

  // Deprecated - Warning/Orange
  if (key === 'deprecated') {
    return {
      bg: 'bg-orange-50 dark:bg-orange-900/20',
      hoverBg: 'hover:bg-orange-100 dark:hover:bg-orange-900/40',
      text: 'text-orange-700 dark:text-orange-400',
      border: 'border-orange-200 dark:border-orange-800',
    }
  }

  // Status - Different colors based on value
  if (key === 'status') {
    if (stringValue === 'active' || stringValue === 'stable') {
      return {
        bg: 'bg-green-50 dark:bg-green-900/20',
        hoverBg: 'hover:bg-green-100 dark:hover:bg-green-900/40',
        text: 'text-green-700 dark:text-green-400',
        border: 'border-green-200 dark:border-green-800',
      }
    }
    if (stringValue === 'deprecated' || stringValue === 'obsolete') {
      return {
        bg: 'bg-orange-50 dark:bg-orange-900/20',
        hoverBg: 'hover:bg-orange-100 dark:hover:bg-orange-900/40',
        text: 'text-orange-700 dark:text-orange-400',
        border: 'border-orange-200 dark:border-orange-800',
      }
    }
    if (stringValue === 'experimental' || stringValue === 'beta') {
      return {
        bg: 'bg-yellow-50 dark:bg-yellow-900/20',
        hoverBg: 'hover:bg-yellow-100 dark:hover:bg-yellow-900/40',
        text: 'text-yellow-700 dark:text-yellow-400',
        border: 'border-yellow-200 dark:border-yellow-800',
      }
    }
    if (stringValue === 'inactive' || stringValue === 'archived') {
      return {
        bg: 'bg-gray-50 dark:bg-gray-900/20',
        hoverBg: 'hover:bg-gray-100 dark:hover:bg-gray-900/40',
        text: 'text-gray-700 dark:text-gray-400',
        border: 'border-gray-200 dark:border-gray-800',
      }
    }
  }

  // Experimental/Beta - Caution/Yellow
  if (key === 'experimental' || key === 'beta') {
    return {
      bg: 'bg-yellow-50 dark:bg-yellow-900/20',
      hoverBg: 'hover:bg-yellow-100 dark:hover:bg-yellow-900/40',
      text: 'text-yellow-700 dark:text-yellow-400',
      border: 'border-yellow-200 dark:border-yellow-800',
    }
  }

  // Version - Info/Blue
  if (key === 'version' || key === 'v') {
    return {
      bg: 'bg-blue-50 dark:bg-blue-900/20',
      hoverBg: 'hover:bg-blue-100 dark:hover:bg-blue-900/40',
      text: 'text-blue-700 dark:text-blue-400',
      border: 'border-blue-200 dark:border-blue-800',
    }
  }

  // URLs (Figma, GitHub, etc.) - Keep blue
  if (
    key === 'figma' ||
    key === 'github' ||
    key === 'url' ||
    key === 'link' ||
    key === 'docs'
  ) {
    return {
      bg: 'bg-blue-50 dark:bg-blue-900/20',
      hoverBg: 'hover:bg-blue-100 dark:hover:bg-blue-900/40',
      text: 'text-blue-700 dark:text-blue-400',
      border: 'border-blue-200 dark:border-blue-800',
    }
  }

  // Default - Neutral gray
  return {
    bg: 'bg-muted',
    hoverBg: 'hover:bg-muted/80',
    text: 'text-foreground',
    border: 'border-border',
  }
}

type BadgeProps = {
  colorScheme: {
    bg: string
    hoverBg: string
    text: string
    border: string
  }
  children: React.ReactNode
  className?: string
}

function Badge({ children, colorScheme }: BadgeProps) {
  return (
    <div
      className={`${colorScheme.bg} ${colorScheme.text} ${colorScheme.border} border rounded-md px-2 py-1 text-xs`}
    >
      {children}
    </div>
  )
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

  const colorScheme = getColorScheme(metaKey, value)

  if (typeof value === 'string' && isUrl(stringValue)) {
    return (
      <a
        href={stringValue}
        target="_blank"
        rel="noopener noreferrer"
        className={`${colorScheme.bg} ${colorScheme.hoverBg} ${colorScheme.text} ${colorScheme.border} border rounded-md px-2 py-1 text-xs transition-all inline-flex items-center gap-1.5 group hover:shadow-sm`}
        title={`Open ${stringValue} in new tab`}
      >
        <span className="font-semibold">{metaKey}</span>
        <ExternalLink className="h-3 w-3 opacity-70 group-hover:opacity-100 transition-opacity" />
      </a>
    )
  }

  // For arrays, show as comma-separated list
  if (Array.isArray(value)) {
    return (
      <Badge colorScheme={colorScheme}>
        <span className="font-semibold">{metaKey}:</span> {value.join(', ')}
      </Badge>
    )
  }

  return (
    <Badge colorScheme={colorScheme}>
      <span className="font-semibold">{metaKey}:</span> {stringValue}
    </Badge>
  )
}
