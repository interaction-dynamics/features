import { describe, expect, it } from 'vitest'
import { compareValues } from '@/hooks/use-table-sort'

describe('compareValues', () => {
  it('should sort numbers in both directions', () => {
    expect([3, 1, 2].sort((a, b) => compareValues(a, b, 'asc'))).toEqual([
      1, 2, 3,
    ])
    expect([3, 1, 2].sort((a, b) => compareValues(a, b, 'desc'))).toEqual([
      3, 2, 1,
    ])
  })

  it('should keep missing values last in both directions', () => {
    const values = [undefined, 10, null, 1200]
    expect(
      [...values].sort((a, b) => compareValues(a, b, 'asc')).slice(0, 2),
    ).toEqual([10, 1200])
    expect(
      [...values].sort((a, b) => compareValues(a, b, 'desc')).slice(0, 2),
    ).toEqual([1200, 10])
    expect(compareValues(undefined, null, 'asc')).toBe(0)
  })
})
