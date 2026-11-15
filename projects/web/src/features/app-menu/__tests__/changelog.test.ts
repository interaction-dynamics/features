import { describe, expect, it } from 'vitest'
import {
  CHANGELOG,
  type ChangelogEntry,
  hasUnreadChangelog,
} from '../changelog'

const entries: ChangelogEntry[] = [
  { id: 'b', title: 'B', url: 'https://example.com/b' },
  { id: 'a', title: 'A', url: 'https://example.com/a' },
]

describe('hasUnreadChangelog', () => {
  it('is unread when nothing was seen yet', () => {
    expect(hasUnreadChangelog(entries, null)).toBe(true)
  })

  it('is unread when a newer entry was added since', () => {
    expect(hasUnreadChangelog(entries, 'a')).toBe(true)
  })

  it('is read when the latest entry was seen', () => {
    expect(hasUnreadChangelog(entries, 'b')).toBe(false)
  })

  it('is read when there is no entry', () => {
    expect(hasUnreadChangelog([], null)).toBe(false)
  })
})

describe('CHANGELOG', () => {
  it('has unique ids', () => {
    const ids = CHANGELOG.map((entry) => entry.id)
    expect(new Set(ids).size).toBe(ids.length)
  })

  it('keeps the ids users have already seen', () => {
    expect(CHANGELOG.map((entry) => entry.id)).toEqual(
      expect.arrayContaining([
        'outside-features',
        'history-feature-graph',
        'feature-history',
        'feature-keys-links',
        'specs-decisions',
      ]),
    )
    expect(CHANGELOG[0].id).toBe('outside-features')
  })

  it('links every entry to its pull request', () => {
    for (const entry of CHANGELOG) {
      expect(entry.url).toMatch(/\/pull\/\d+$/)
    }
  })
})
