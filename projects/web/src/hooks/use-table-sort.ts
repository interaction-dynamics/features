import { useMemo, useState } from 'react'

export type SortDirection = 'asc' | 'desc' | null

export type SortConfig<T> = {
  field: keyof T | string
  direction: SortDirection
}

export type UseSortReturn<T> = {
  sortedData: T[]
  sortConfig: SortConfig<T> | null
  requestSort: (field: keyof T | string) => void
  clearSort: () => void
}

/**
 * Get a nested value from an object using dot notation
 */
function getNestedValue(obj: unknown, path: string): unknown {
  const keys = path.split('.')
  let value: unknown = obj

  for (const key of keys) {
    if (value === null || value === undefined) {
      return undefined
    }
    value = (value as Record<string, unknown>)[key]
  }

  return value
}

/**
 * Compare two values for sorting
 */
function compareValues(
  a: unknown,
  b: unknown,
  direction: SortDirection,
): number {
  // Handle null/undefined
  if (a === null || a === undefined) return direction === 'asc' ? 1 : -1
  if (b === null || b === undefined) return direction === 'asc' ? -1 : 1

  // Handle dates
  if (a instanceof Date && b instanceof Date) {
    return direction === 'asc'
      ? a.getTime() - b.getTime()
      : b.getTime() - a.getTime()
  }

  // Try to parse as date strings
  const dateA = new Date(a as string)
  const dateB = new Date(b as string)
  if (!Number.isNaN(dateA.getTime()) && !Number.isNaN(dateB.getTime())) {
    return direction === 'asc'
      ? dateA.getTime() - dateB.getTime()
      : dateB.getTime() - dateA.getTime()
  }

  // Handle numbers
  if (typeof a === 'number' && typeof b === 'number') {
    return direction === 'asc' ? a - b : b - a
  }

  // Handle strings
  const strA = String(a).toLowerCase()
  const strB = String(b).toLowerCase()

  if (strA < strB) return direction === 'asc' ? -1 : 1
  if (strA > strB) return direction === 'asc' ? 1 : -1
  return 0
}

/**
 * Reusable hook for sorting table data
 *
 * @param data - Array of items to sort
 * @param defaultSort - Optional default sort configuration
 *
 * @example
 * const { sortedData, sortConfig, requestSort } = useTableSort(features, {
 *   field: 'name',
 *   direction: 'asc'
 * })
 */
export function useTableSort<T extends Record<string, unknown>>(
  data: T[],
  defaultSort?: SortConfig<T>,
): UseSortReturn<T> {
  const [sortConfig, setSortConfig] = useState<SortConfig<T> | null>(
    defaultSort || null,
  )

  const sortedData = useMemo(() => {
    if (!sortConfig || !sortConfig.direction) {
      return data
    }

    const sorted = [...data].sort((a, b) => {
      const aValue = getNestedValue(a, String(sortConfig.field))
      const bValue = getNestedValue(b, String(sortConfig.field))

      return compareValues(aValue, bValue, sortConfig.direction)
    })

    return sorted
  }, [data, sortConfig])

  const requestSort = (field: keyof T | string) => {
    let direction: SortDirection = 'asc'

    if (
      sortConfig &&
      sortConfig.field === field &&
      sortConfig.direction === 'asc'
    ) {
      direction = 'desc'
    } else if (
      sortConfig &&
      sortConfig.field === field &&
      sortConfig.direction === 'desc'
    ) {
      direction = null
    }

    setSortConfig(direction ? { field, direction } : null)
  }

  const clearSort = () => {
    setSortConfig(null)
  }

  return {
    sortedData,
    sortConfig,
    requestSort,
    clearSort,
  }
}
