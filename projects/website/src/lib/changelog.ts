// Single source of truth shared with the dashboard's "What's new" menu
// (projects/web/src/features/app-menu/changelog.ts). See docs/specs/012-website-changelog.md.
import data from '../../../../docs/changelog.json'
import { site } from '../data/site'

export const changeTypes = ['feature', 'fix', 'docs'] as const
export type ChangeType = (typeof changeTypes)[number]

export const changeTypeLabels: Record<ChangeType, string> = {
  feature: 'Features',
  fix: 'Fixes',
  docs: 'Docs',
}

export interface Change {
  type: ChangeType
  title: string
  description?: string
  pr?: number
  id?: string
}

/** Version of the changes merged since the last release, listed first. */
export const UNRELEASED = 'unreleased'

export interface Release {
  /** `0.15.0`, or `unreleased` for the changes merged since the last release. */
  version: string
  unreleased: boolean
  /** Release date, absent for unreleased changes. */
  date?: string
  summary?: string
  /** Release page: the GitHub release, or the npm version when `githubRelease` is false. */
  url?: string
  releaseLabel?: string
  /** Compare view with the previous release, when both have a GitHub tag. */
  compareUrl?: string
  /** Anchor on the changelog page: `0.15.0` → `v0-15-0`, `unreleased` → `unreleased`. */
  anchor: string
  changes: Change[]
}

const fail = (message: string): never => {
  throw new Error(`docs/changelog.json: ${message}`)
}

const isChangeType = (type: string): type is ChangeType =>
  (changeTypes as readonly string[]).includes(type)

// Without a GitHub release (no tag), a release links its npm version and has no compare view.
const isTagged = (release: (typeof data.releases)[number]) =>
  !('githubRelease' in release) || release.githubRelease !== false

const dateOf = (release: (typeof data.releases)[number]) =>
  'date' in release ? release.date : undefined

/** Validated at build time, so a typo in the JSON breaks the build instead of the page. */
const parse = (): Release[] =>
  data.releases.map((release, index, releases) => {
    const { version } = release
    const date = dateOf(release)
    const unreleased = version === UNRELEASED
    const previous = releases[index + 1]

    if (unreleased) {
      if (index !== 0) fail('"unreleased" must be the first entry')
      if (date) fail('"unreleased" has no date')
    } else {
      if (!/^\d+\.\d+\.\d+$/.test(version)) {
        fail(`invalid version "${version}"`)
      }
      if (!date || !/^\d{4}-\d{2}-\d{2}$/.test(date)) {
        fail(`invalid date for ${version}`)
      }
      const previousDate = previous && dateOf(previous)
      if (date && previousDate && previousDate > date) {
        fail(
          `${version} must be listed before ${previous.version} (newest first)`,
        )
      }
    }

    const changes = release.changes.map((change): Change => {
      if (!isChangeType(change.type)) {
        fail(`unknown change type "${change.type}" in ${version}`)
      }
      return change as Change
    })

    const tagged = !unreleased && isTagged(release)
    const previousTagged = previous !== undefined && isTagged(previous)
    const head = unreleased ? 'master' : `v${version}`

    return {
      version,
      unreleased,
      date,
      summary: 'summary' in release ? release.summary : undefined,
      url: unreleased
        ? undefined
        : tagged
          ? `${site.repository}/releases/tag/v${version}`
          : `${site.npm}/v/${version}`,
      releaseLabel: unreleased
        ? undefined
        : tagged
          ? `GitHub release ${version}`
          : `npm package ${version}`,
      compareUrl:
        previousTagged && (tagged || unreleased)
          ? `${site.repository}/compare/v${previous.version}...${head}`
          : undefined,
      anchor: unreleased ? UNRELEASED : `v${version.replaceAll('.', '-')}`,
      changes,
    }
  })

export const releases = parse()

/** Published releases only, for the latest version and the feed. */
export const publishedReleases = releases.filter(
  (release): release is Release & { date: string } => !release.unreleased,
)

export const pullRequestUrl = (pr: number) => `${site.repository}/pull/${pr}`

export const formatDate = (date: string) =>
  new Date(`${date}T00:00:00Z`).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    timeZone: 'UTC',
  })

/** Changes of a release grouped as Features, Fixes, Docs; empty groups are left out. */
export const groupChanges = (changes: Change[]) =>
  changeTypes
    .map((type) => ({
      type,
      label: changeTypeLabels[type],
      changes: changes.filter((change) => change.type === type),
    }))
    .filter((group) => group.changes.length > 0)

const escapeHtml = (text: string) =>
  text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')

/** Changelog text marks code with backticks: `--since` → <code>--since</code>. */
export const toHtml = (text: string) =>
  escapeHtml(text).replace(/`([^`]+)`/g, '<code>$1</code>')

/** The same text without the backticks, for the feed and structured data. */
export const toPlainText = (text: string) => text.replace(/`([^`]+)`/g, '$1')
