import { useCallback, useState } from 'react'
import { CHANGELOG, hasUnreadChangelog } from './changelog'

const STORAGE_KEY = 'features-changelog-last-seen'

function readLastSeen(): string | null {
  try {
    return localStorage.getItem(STORAGE_KEY)
  } catch {
    return null
  }
}

/**
 * Tracks whether the user has opened the menu since the latest changelog
 * entry was added. Stored per browser; without storage, entries stay unread.
 */
export function useChangelogSeen() {
  const [lastSeenId, setLastSeenId] = useState(readLastSeen)

  const markSeen = useCallback(() => {
    const latestId = CHANGELOG[0]?.id
    if (!latestId) return
    setLastSeenId(latestId)
    try {
      localStorage.setItem(STORAGE_KEY, latestId)
    } catch {
      // Storage unavailable (private mode, blocked): keep the in-memory state
    }
  }, [])

  return {
    hasUnread: hasUnreadChangelog(CHANGELOG, lastSeenId),
    markSeen,
  }
}
