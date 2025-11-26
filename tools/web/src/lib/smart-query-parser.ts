/**
 * Smart Query Parser
 *
 * Parses GitHub-like query syntax for filtering data.
 *
 * Supported syntax:
 * - field:value - Exact match
 * - field:>value - Greater than
 * - field:<value - Less than
 * - field:>=value - Greater than or equal
 * - field:<=value - Less than or equal
 * - field:!=value - Not equal
 * - "quoted value" - Exact phrase match
 * - word1 word2 - Multiple words (OR search)
 * - word1 AND word2 - AND search
 * - word1 OR word2 - OR search
 * - field:"quoted value" - Field with quoted value
 *
 * Examples:
 * - owner:john AND lines:>1000
 * - name:feature1 OR name:feature2
 * - todos:>5
 * - "react component" AND owner:jane
 */

export type ComparisonOperator = '=' | '!=' | '>' | '<' | '>=' | '<='
export type LogicalOperator = 'AND' | 'OR'

export type FilterCondition = {
  field?: string
  operator: ComparisonOperator
  value: string | number
  raw: string
}

export type QueryGroup = {
  conditions: FilterCondition[]
  operator: LogicalOperator
}

export type ParsedQuery = {
  groups: QueryGroup[]
  rawQuery: string
}

/**
 * Tokenize the query string into meaningful parts
 */
function tokenize(query: string): string[] {
  const tokens: string[] = []
  let current = ''
  let inQuotes = false
  let quoteChar = ''

  for (let i = 0; i < query.length; i++) {
    const char = query[i]

    if ((char === '"' || char === "'") && !inQuotes) {
      inQuotes = true
      quoteChar = char
      current += char
    } else if (char === quoteChar && inQuotes) {
      inQuotes = false
      quoteChar = ''
      current += char
    } else if (char === ' ' && !inQuotes) {
      if (current) {
        tokens.push(current)
        current = ''
      }
    } else {
      current += char
    }
  }

  if (current) {
    tokens.push(current)
  }

  return tokens
}

/**
 * Parse a single condition (e.g., "owner:john", "lines:>100")
 */
function parseCondition(token: string): FilterCondition {
  // Check if it's a field:value pattern
  const colonIndex = token.indexOf(':')

  if (colonIndex === -1) {
    // No colon, treat as a general search term
    const value = token.replace(/^["']|["']$/g, '')
    return {
      operator: '=',
      value,
      raw: token,
    }
  }

  const field = token.substring(0, colonIndex)
  let valueStr = token.substring(colonIndex + 1)

  // Extract operator
  let operator: ComparisonOperator = '='
  if (valueStr.startsWith('>=')) {
    operator = '>='
    valueStr = valueStr.substring(2)
  } else if (valueStr.startsWith('<=')) {
    operator = '<='
    valueStr = valueStr.substring(2)
  } else if (valueStr.startsWith('!=')) {
    operator = '!='
    valueStr = valueStr.substring(2)
  } else if (valueStr.startsWith('>')) {
    operator = '>'
    valueStr = valueStr.substring(1)
  } else if (valueStr.startsWith('<')) {
    operator = '<'
    valueStr = valueStr.substring(1)
  }

  // Remove quotes if present
  const cleanValue = valueStr.replace(/^["']|["']$/g, '')

  // Try to parse as number
  const numValue = parseFloat(cleanValue)
  const value =
    !Number.isNaN(numValue) && cleanValue === numValue.toString()
      ? numValue
      : cleanValue

  return {
    field,
    operator,
    value,
    raw: token,
  }
}

/**
 * Parse a query string into structured filter conditions
 */
export function parseQuery(query: string): ParsedQuery {
  const trimmedQuery = query.trim()

  if (!trimmedQuery) {
    return {
      groups: [],
      rawQuery: query,
    }
  }

  const tokens = tokenize(trimmedQuery)
  const groups: QueryGroup[] = []
  let currentGroup: QueryGroup = {
    conditions: [],
    operator: 'AND',
  }

  for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i]
    const upperToken = token.toUpperCase()

    if (upperToken === 'AND') {
      if (currentGroup.conditions.length > 0) {
        groups.push(currentGroup)
        currentGroup = {
          conditions: [],
          operator: 'AND',
        }
      }
    } else if (upperToken === 'OR') {
      if (currentGroup.conditions.length > 0) {
        if (currentGroup.operator === 'AND') {
          // Start a new OR group
          groups.push(currentGroup)
          currentGroup = {
            conditions: [],
            operator: 'OR',
          }
        }
      } else {
        currentGroup.operator = 'OR'
      }
    } else {
      const condition = parseCondition(token)
      currentGroup.conditions.push(condition)
    }
  }

  if (currentGroup.conditions.length > 0) {
    groups.push(currentGroup)
  }

  return {
    groups,
    rawQuery: query,
  }
}

/**
 * Compare two values based on the operator
 */
function compareValues(
  itemValue: unknown,
  filterValue: string | number,
  operator: ComparisonOperator,
): boolean {
  // Handle null/undefined
  if (itemValue === null || itemValue === undefined) {
    return operator === '!='
      ? filterValue !== null && filterValue !== undefined
      : false
  }

  // Convert to strings for comparison if needed
  const itemStr = String(itemValue).toLowerCase()
  const filterStr = String(filterValue).toLowerCase()

  // Try numeric comparison first
  const itemNum = parseFloat(String(itemValue))
  const filterNum =
    typeof filterValue === 'number' ? filterValue : parseFloat(filterStr)

  if (!Number.isNaN(itemNum) && !Number.isNaN(filterNum)) {
    switch (operator) {
      case '=':
        return itemNum === filterNum
      case '!=':
        return itemNum !== filterNum
      case '>':
        return itemNum > filterNum
      case '<':
        return itemNum < filterNum
      case '>=':
        return itemNum >= filterNum
      case '<=':
        return itemNum <= filterNum
    }
  }

  // String comparison
  switch (operator) {
    case '=':
      return itemStr.includes(filterStr)
    case '!=':
      return !itemStr.includes(filterStr)
    case '>':
      return itemStr > filterStr
    case '<':
      return itemStr < filterStr
    case '>=':
      return itemStr >= filterStr
    case '<=':
      return itemStr <= filterStr
  }

  return false
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
 * Apply a filter condition to an item
 */
function applyCondition<T extends Record<string, unknown>>(
  item: T,
  condition: FilterCondition,
  searchableFields?: (keyof T)[],
): boolean {
  if (condition.field) {
    // Specific field filter
    const value = getNestedValue(item, condition.field)
    return compareValues(value, condition.value, condition.operator)
  } else {
    // General search - check all searchable fields
    const fieldsToSearch =
      searchableFields || (Object.keys(item) as (keyof T)[])

    return fieldsToSearch.some((field) => {
      const value = getNestedValue(item, String(field))
      return compareValues(value, condition.value, condition.operator)
    })
  }
}

/**
 * Filter an array of items based on a parsed query
 */
export function filterByQuery<T extends Record<string, unknown>>(
  items: T[],
  parsedQuery: ParsedQuery,
  searchableFields?: (keyof T)[],
): T[] {
  if (parsedQuery.groups.length === 0) {
    return items
  }

  return items.filter((item) => {
    // All groups must match (implicit AND between groups)
    return parsedQuery.groups.every((group) => {
      if (group.operator === 'AND') {
        // All conditions in the group must match
        return group.conditions.every((condition) =>
          applyCondition(item, condition, searchableFields),
        )
      } else {
        // At least one condition in the group must match
        return group.conditions.some((condition) =>
          applyCondition(item, condition, searchableFields),
        )
      }
    })
  })
}

/**
 * Get available fields from sample data for autocomplete
 */
export function extractFields<T extends Record<string, unknown>>(
  items: T[],
  maxDepth = 2,
): string[] {
  if (items.length === 0) return []

  const fields = new Set<string>()

  function extractFromObject(obj: unknown, prefix = '', depth = 0) {
    if (depth >= maxDepth) return
    if (obj === null || typeof obj !== 'object') return

    Object.keys(obj as Record<string, unknown>).forEach((key) => {
      const fieldPath = prefix ? `${prefix}.${key}` : key
      const value = (obj as Record<string, unknown>)[key]

      if (
        value !== null &&
        typeof value === 'object' &&
        !Array.isArray(value)
      ) {
        fields.add(fieldPath)
        extractFromObject(value, fieldPath, depth + 1)
      } else if (value !== null && typeof value !== 'object') {
        fields.add(fieldPath)
      }
    })
  }

  // Sample first few items
  items.slice(0, 5).forEach((item) => {
    extractFromObject(item)
  })

  return Array.from(fields).sort()
}
