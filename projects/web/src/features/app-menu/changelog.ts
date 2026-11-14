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

/** Newest first. Add new entries at the top. */
export const CHANGELOG: ChangelogEntry[] = [
  {
    id: 'outside-features',
    title: 'Outside features page',
    url: `${REPOSITORY_URL}/pull/177`,
  },
  {
    id: 'history-feature-graph',
    title: 'Feature graph in History',
    url: `${REPOSITORY_URL}/pull/174`,
  },
  {
    id: 'feature-history',
    title: 'Feature history from commits',
    url: `${REPOSITORY_URL}/pull/165`,
  },
  {
    id: 'feature-keys-links',
    title: 'Feature keys, links and external data',
    url: `${REPOSITORY_URL}/pull/163`,
  },
  {
    id: 'specs-decisions',
    title: 'Specs and decisions side by side',
    url: `${REPOSITORY_URL}/pull/158`,
  },
]

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
