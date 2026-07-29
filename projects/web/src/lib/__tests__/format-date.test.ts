import { describe, expect, it } from 'vitest'
import { formatDate } from '@/lib/format-date'

describe('formatDate', () => {
  it('should return "N/A" for undefined', () => {
    expect(formatDate(undefined)).toBe('N/A')
  })

  it('should return "N/A" for empty string', () => {
    expect(formatDate('')).toBe('N/A')
  })

  it('should format ISO date string', () => {
    const result = formatDate('2024-01-15T10:30:00Z')
    expect(result).toMatch(/Jan 15, 2024/)
  })

  it('should format date with year, month, and day', () => {
    const result = formatDate('2023-12-25T12:00:00Z')
    expect(result).toMatch(/Dec 25, 2023/)
  })

  it('should format date at beginning of year', () => {
    const result = formatDate('2024-01-01T12:00:00Z')
    expect(result).toMatch(/Jan 1, 2024/)
  })

  it('should format date at end of year', () => {
    const result = formatDate('2024-12-31T12:00:00Z')
    expect(result).toMatch(/Dec 31, 2024/)
  })

  it('should format date with time information', () => {
    const result = formatDate('2024-06-15T14:30:00.000Z')
    expect(result).toMatch(/Jun 15, 2024/)
  })

  it('should format date with timezone', () => {
    const result = formatDate('2024-03-20T08:00:00+05:00')
    // Note: The exact date may vary based on timezone conversion
    expect(result).toMatch(/Mar (19|20), 2024/)
  })

  it('should handle leap year date', () => {
    const result = formatDate('2024-02-29T12:00:00Z')
    expect(result).toMatch(/Feb 29, 2024/)
  })

  it('should format old dates', () => {
    const result = formatDate('2000-01-01T12:00:00Z')
    expect(result).toMatch(/Jan 1, 2000/)
  })

  it('should format future dates', () => {
    const result = formatDate('2030-07-04T12:00:00Z')
    expect(result).toMatch(/Jul 4, 2030/)
  })

  it('should use en-US locale format', () => {
    const result = formatDate('2024-05-10T12:00:00Z')
    // en-US format: Month Day, Year
    expect(result).toMatch(/^[A-Z][a-z]{2} \d{1,2}, \d{4}$/)
  })

  it('should format date with milliseconds', () => {
    const result = formatDate('2024-08-22T12:34:56.789Z')
    expect(result).toMatch(/Aug 22, 2024/)
  })
})
