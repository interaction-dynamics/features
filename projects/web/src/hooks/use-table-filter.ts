import { useMemo, useState } from 'react'
import {
  filterByQuery,
  type ParsedQuery,
  parseQuery,
} from '@/lib/smart-query-parser'

export type UseFilterReturn<T> = {
  filteredData: T[]
  query: string
  setQuery: (query: string) => void
  clearQuery: () => void
  parsedQuery: ParsedQuery
}

/**
 * Reusable hook for filtering table data with smart query syntax
 *
 * Supports GitHub-like query syntax:
 * - field:value - Exact match
 * - field:>value - Greater than
 * - field:<value - Less than
 * - field:>=value - Greater than or equal
 * - field:<=value - Less than or equal
 * - field:!=value - Not equal
 * - "quoted value" - Exact phrase match
 * - word1 AND word2 - AND search
 * - word1 OR word2 - OR search
 *
 * @param data - Array of items to filter
 * @param searchableFields - Optional array of field names to search when no field is specified
 * @param initialQuery - Optional initial query string
 *
 * @example
 * const { filteredData, query, setQuery } = useTableFilter(features, [
 *   'name',
 *   'owner',
 *   'description'
 * ])
 *
 * @example
 * // Advanced usage with nested fields
 * const { filteredData, query, setQuery } = useTableFilter(features, [
 *   'name',
 *   'owner',
 *   'stats.lines_count',
 *   'stats.files_count'
 * ])
 */
export function useTableFilter<T extends Record<string, unknown>>(
  data: T[],
  searchableFields?: (keyof T | string)[],
  initialQuery = '',
): UseFilterReturn<T> {
  const [query, setQuery] = useState<string>(initialQuery)

  const parsedQuery = useMemo(() => {
    return parseQuery(query)
  }, [query])

  const filteredData = useMemo(() => {
    if (parsedQuery.groups.length === 0) {
      return data
    }

    return filterByQuery(data, parsedQuery, searchableFields as (keyof T)[])
  }, [data, parsedQuery, searchableFields])

  const clearQuery = () => {
    setQuery('')
  }

  return {
    filteredData,
    query,
    setQuery,
    clearQuery,
    parsedQuery,
  }
}
