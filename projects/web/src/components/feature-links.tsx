import {
  Activity,
  BookOpen,
  Bug,
  ChartLine,
  Code,
  ExternalLink,
  Link,
  ListTodo,
  type LucideIcon,
  MessageSquare,
  PenTool,
} from 'lucide-react'
import {
  describeLink,
  isHttpUrl,
  type ToolCategory,
} from '@/lib/external-tools'
import type { Feature } from '@/models/feature'

const CATEGORY_ICONS: Record<ToolCategory, LucideIcon> = {
  analytics: ChartLine,
  errors: Bug,
  monitoring: Activity,
  design: PenTool,
  issues: ListTodo,
  code: Code,
  docs: BookOpen,
  chat: MessageSquare,
  other: Link,
}

interface FeatureLinksProps {
  feature: Feature
}

/** Links to the feature in external tools (`links` in the README front matter) */
export function FeatureLinks({ feature }: FeatureLinksProps) {
  const links = Object.entries(feature.links ?? {}).filter(([, url]) =>
    isHttpUrl(url),
  )

  if (links.length === 0) {
    return null
  }

  return (
    <div className="flex items-start gap-3">
      <Link className="h-4 w-4 text-muted-foreground mt-0.5" />
      <div className="flex-1">
        <p className="text-sm font-medium text-foreground mb-1">Links</p>
        <div className="flex flex-wrap gap-2">
          {links.map(([name, url]) => {
            const { label, category } = describeLink(name)
            const Icon = CATEGORY_ICONS[category]
            return (
              <a
                key={name}
                href={url}
                target="_blank"
                rel="noopener noreferrer"
                title={`Open ${url} in a new tab`}
                className="bg-muted hover:bg-muted/80 text-foreground border-border border rounded-md px-2 py-1 text-xs transition-all inline-flex items-center gap-1.5 group hover:shadow-sm"
              >
                <Icon className="h-3 w-3 text-muted-foreground" />
                <span className="font-semibold">{label}</span>
                <ExternalLink className="h-3 w-3 opacity-50 group-hover:opacity-100 transition-opacity" />
              </a>
            )
          })}
        </div>
      </div>
    </div>
  )
}
