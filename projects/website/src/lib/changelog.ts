// Single source of truth shared with the dashboard's "What's new" menu
// (projects/web/src/features/app-menu/changelog.ts). See docs/specs/011-website-changelog.md.
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

export interface Release {
  version: string
  date: string
  summary?: string
  /** Release page: the GitHub release, or the npm version when `githubRelease` is false. */
  url: string
  releaseLabel: string
  /** Compare view with the previous release, when there is one. */
  compareUrl?: string
  /** Anchor on the changelog page: `0.15.0` → `v0-15-0`. */
  anchor: string
  changes: Change[]
}

const fail = (message: string): never => {
  throw new Error(`docs/changelog.json: ${message}`)
}

const isChangeType = (type: string): type is ChangeType =>
  (changeTypes as readonly string[]).includes(type)

/** Validated at build time, so a typo in the JSON breaks the build instead of the page. */
const parse = (): Release[] =>
  data.releases.map((release, index, releases) => {
    const { version, date } = release
    if (!/^\d+\.\d+\.\d+$/.test(version)) fail(`invalid version "${version}"`)
    if (!/^\d{4}-\d{2}-\d{2}$/.test(date)) fail(`invalid date for ${version}`)
    const previous = releases[index + 1]
    if (previous && previous.date > date) {
      fail(
        `${version} must be listed before ${previous.version} (newest first)`,
      )
    }

    const changes = release.changes.map((change): Change => {
      if (!isChangeType(change.type)) {
        fail(`unknown change type "${change.type}" in ${version}`)
      }
      return change as Change
    })

    // Without a GitHub release (no tag), link the npm version and skip the compare view.
    const tagged = !('githubRelease' in release) || release.githubRelease
    return {
      version,
      date,
      summary: 'summary' in release ? release.summary : undefined,
      url: tagged
        ? `${site.repository}/releases/tag/v${version}`
        : `${site.npm}/v/${version}`,
      releaseLabel: tagged ? 'GitHub release' : 'npm package',
      compareUrl:
        tagged && previous
          ? `${site.repository}/compare/v${previous.version}...v${version}`
          : undefined,
      anchor: `v${version.replaceAll('.', '-')}`,
      changes,
    }
  })

export const releases = parse()

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
