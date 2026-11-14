import type { ExternalValue, Feature } from '@/models/feature'

/** Kind of external tool, used to pick an icon for a link */
export type ToolCategory =
  | 'analytics'
  | 'errors'
  | 'monitoring'
  | 'design'
  | 'issues'
  | 'code'
  | 'docs'
  | 'chat'
  | 'other'

type KnownTool = { label: string; category: ToolCategory }

const KNOWN_TOOLS: Record<string, KnownTool> = {
  posthog: { label: 'PostHog', category: 'analytics' },
  amplitude: { label: 'Amplitude', category: 'analytics' },
  mixpanel: { label: 'Mixpanel', category: 'analytics' },
  ga: { label: 'Google Analytics', category: 'analytics' },
  plausible: { label: 'Plausible', category: 'analytics' },
  sentry: { label: 'Sentry', category: 'errors' },
  bugsnag: { label: 'Bugsnag', category: 'errors' },
  rollbar: { label: 'Rollbar', category: 'errors' },
  datadog: { label: 'Datadog', category: 'monitoring' },
  grafana: { label: 'Grafana', category: 'monitoring' },
  newrelic: { label: 'New Relic', category: 'monitoring' },
  honeycomb: { label: 'Honeycomb', category: 'monitoring' },
  figma: { label: 'Figma', category: 'design' },
  linear: { label: 'Linear', category: 'issues' },
  jira: { label: 'Jira', category: 'issues' },
  github: { label: 'GitHub', category: 'code' },
  gitlab: { label: 'GitLab', category: 'code' },
  notion: { label: 'Notion', category: 'docs' },
  confluence: { label: 'Confluence', category: 'docs' },
  docs: { label: 'Docs', category: 'docs' },
  slack: { label: 'Slack', category: 'chat' },
  teams: { label: 'Teams', category: 'chat' },
}

/** Readable label and category of a link name, e.g. `posthog` → PostHog */
export function describeLink(name: string): KnownTool {
  const known = KNOWN_TOOLS[name.toLowerCase().replace(/[\s_-]/g, '')]
  if (known) {
    return known
  }
  return { label: humanize(name), category: 'other' }
}

/** `weekly_users` → `Weekly users` */
export function humanize(name: string): string {
  const words = name.replace(/[_-]+/g, ' ').trim()
  return words.charAt(0).toUpperCase() + words.slice(1)
}

/** Only http(s) URLs are rendered as links, never `javascript:` and the like */
export function isHttpUrl(value: string): boolean {
  try {
    const url = new URL(value)
    return url.protocol === 'http:' || url.protocol === 'https:'
  } catch {
    return false
  }
}

/** Display an external metric: numbers with thousands separators, booleans as yes/no */
export function formatExternalValue(value: ExternalValue | undefined): string {
  if (value === undefined) {
    return ''
  }
  if (typeof value === 'number') {
    return value.toLocaleString('en-US')
  }
  if (typeof value === 'boolean') {
    return value ? 'yes' : 'no'
  }
  return value
}

/** Sorted names of every external metric found in `features` */
export function getExternalMetricKeys(features: Feature[]): string[] {
  const keys = new Set<string>()
  for (const feature of features) {
    for (const key of Object.keys(feature.external ?? {})) {
      keys.add(key)
    }
  }
  return Array.from(keys).sort()
}
