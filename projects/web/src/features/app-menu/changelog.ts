import changelog from '../../../../../docs/changelog.json'

export const WEBSITE_URL = 'https://features.interaction-dynamics.io'
export const REPOSITORY_URL = 'https://github.com/interaction-dynamics/features'
export const DOCS_URL = `${REPOSITORY_URL}#readme`
export const FEEDBACK_URL = `${REPOSITORY_URL}/issues/new/choose`
export const CHANGELOG_URL = `${REPOSITORY_URL}/releases`

export interface ChangelogEntry {
  /** Stable id, used to know which entries the user has already seen */
  id: string
  title: string
  url: string
}

/**
 * Newest first. Built from the changes that have an `id` in
 * `docs/changelog.json`, the changelog shared with the website's /changelog page:
 * add new entries there, not here.
 */
export const CHANGELOG: ChangelogEntry[] = changelog.releases.flatMap(
  (release) =>
    release.changes.flatMap((change) =>
      'id' in change && change.id
        ? [
            {
              id: change.id,
              title: change.title,
              url: `${REPOSITORY_URL}/pull/${change.pr}`,
            },
          ]
        : [],
    ),
)

/** Number of entries shown in the "What's new" section of the menu */
export const CHANGELOG_PREVIEW_SIZE = 3

/**
 * Whether the changelog has an entry newer than the last one the user saw.
 * Nothing seen yet counts as unread, so first-time users discover the section.
 */
export function hasUnreadChangelog(
  entries: ChangelogEntry[],
  lastSeenId: string | null,
): boolean {
  if (entries.length === 0) return false
  return entries[0].id !== lastSeenId
}
