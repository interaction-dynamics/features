import { describe, expect, it } from 'vitest'
import {
  describeLink,
  formatExternalValue,
  getExternalMetricKeys,
  humanize,
  isHttpUrl,
} from '@/lib/external-tools'
import type { Feature } from '@/models/feature'

function feature(external?: Feature['external']): Feature {
  return {
    name: 'feature',
    path: 'feature',
    owner: '',
    is_owner_inherited: false,
    changes: [],
    decisions: [],
    dependencies: [],
    external,
  }
}

describe('describeLink', () => {
  it('should recognize well-known tools whatever the case or separators', () => {
    expect(describeLink('posthog')).toEqual({
      label: 'PostHog',
      category: 'analytics',
    })
    expect(describeLink('Sentry')).toEqual({
      label: 'Sentry',
      category: 'errors',
    })
    expect(describeLink('new_relic')).toEqual({
      label: 'New Relic',
      category: 'monitoring',
    })
  })

  it('should humanize unknown link names', () => {
    expect(describeLink('runbook_url')).toEqual({
      label: 'Runbook url',
      category: 'other',
    })
  })
})

describe('humanize', () => {
  it('should replace separators and capitalize', () => {
    expect(humanize('weekly_users')).toBe('Weekly users')
    expect(humanize('open-errors')).toBe('Open errors')
    expect(humanize('status')).toBe('Status')
  })
})

describe('isHttpUrl', () => {
  it('should accept http and https URLs only', () => {
    expect(isHttpUrl('https://eu.posthog.com/project/1')).toBe(true)
    expect(isHttpUrl('http://localhost:3000')).toBe(true)
    expect(isHttpUrl('javascript:alert(1)')).toBe(false)
    expect(isHttpUrl('ftp://example.com')).toBe(false)
    expect(isHttpUrl('not a url')).toBe(false)
  })
})

describe('formatExternalValue', () => {
  it('should format numbers, booleans and strings', () => {
    expect(formatExternalValue(1200)).toBe('1,200')
    expect(formatExternalValue(0.5)).toBe('0.5')
    expect(formatExternalValue(true)).toBe('yes')
    expect(formatExternalValue(false)).toBe('no')
    expect(formatExternalValue('healthy')).toBe('healthy')
    expect(formatExternalValue(undefined)).toBe('')
  })
})

describe('getExternalMetricKeys', () => {
  it('should list the metrics of every feature once, sorted', () => {
    expect(
      getExternalMetricKeys([
        feature({ weekly_users: 10, errors: 1 }),
        feature(),
        feature({ errors: 2, status: 'ok' }),
      ]),
    ).toEqual(['errors', 'status', 'weekly_users'])
  })
})
