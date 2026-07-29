import { describe, expect, it } from 'vitest'
import {
  extractFields,
  filterByQuery,
  parseQuery,
} from '@/lib/smart-query-parser'

describe('parseQuery', () => {
  it('should parse empty query', () => {
    const result = parseQuery('')
    expect(result.groups).toHaveLength(0)
  })

  it('should parse simple field:value query', () => {
    const result = parseQuery('name:john')
    expect(result.groups).toHaveLength(1)
    expect(result.groups[0].conditions).toHaveLength(1)
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'name',
      operator: '=',
      value: 'john',
    })
  })

  it('should parse greater than operator', () => {
    const result = parseQuery('age:>25')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'age',
      operator: '>',
      value: 25,
    })
  })

  it('should parse less than operator', () => {
    const result = parseQuery('score:<100')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'score',
      operator: '<',
      value: 100,
    })
  })

  it('should parse greater than or equal operator', () => {
    const result = parseQuery('lines:>=1000')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'lines',
      operator: '>=',
      value: 1000,
    })
  })

  it('should parse less than or equal operator', () => {
    const result = parseQuery('todos:<=5')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'todos',
      operator: '<=',
      value: 5,
    })
  })

  it('should parse not equal operator', () => {
    const result = parseQuery('status:!=pending')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'status',
      operator: '!=',
      value: 'pending',
    })
  })

  it('should parse quoted values', () => {
    const result = parseQuery('name:"John Doe"')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'name',
      operator: '=',
      value: 'John Doe',
    })
  })

  it('should parse general search without field', () => {
    const result = parseQuery('searchterm')
    expect(result.groups[0].conditions[0]).toMatchObject({
      operator: '=',
      value: 'searchterm',
    })
    expect(result.groups[0].conditions[0].field).toBeUndefined()
  })

  it('should parse AND operator', () => {
    const result = parseQuery('name:john AND age:>25')
    expect(result.groups).toHaveLength(2)
    expect(result.groups[0].conditions[0].field).toBe('name')
    expect(result.groups[1].conditions[0].field).toBe('age')
  })

  it('should parse OR operator', () => {
    const result = parseQuery('status:active OR status:pending')
    expect(result.groups).toHaveLength(1)
    expect(result.groups[0].operator).toBe('OR')
    expect(result.groups[0].conditions).toHaveLength(2)
  })

  it('should parse complex query', () => {
    const result = parseQuery('owner:john AND lines:>1000 AND todos:<=5')
    expect(result.groups).toHaveLength(3)
  })

  it('should handle nested field syntax', () => {
    const result = parseQuery('stats.lines_count:>1000')
    expect(result.groups[0].conditions[0]).toMatchObject({
      field: 'stats.lines_count',
      operator: '>',
      value: 1000,
    })
  })
})

describe('filterByQuery', () => {
  const testData = [
    {
      id: 1,
      name: 'John Doe',
      email: 'john@example.com',
      age: 30,
      role: 'admin',
      stats: { tasks: 50, projects: 5 },
    },
    {
      id: 2,
      name: 'Jane Smith',
      email: 'jane@example.com',
      age: 25,
      role: 'developer',
      stats: { tasks: 100, projects: 10 },
    },
    {
      id: 3,
      name: 'Bob Johnson',
      email: 'bob@example.com',
      age: 35,
      role: 'developer',
      stats: { tasks: 75, projects: 8 },
    },
  ]

  it('should filter by exact field match', () => {
    const query = parseQuery('role:admin')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('John Doe')
  })

  it('should filter by partial match', () => {
    const query = parseQuery('name:john')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2) // John Doe and Bob Johnson
  })

  it('should filter by greater than', () => {
    const query = parseQuery('age:>30')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('Bob Johnson')
  })

  it('should filter by less than', () => {
    const query = parseQuery('age:<30')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('Jane Smith')
  })

  it('should filter by greater than or equal', () => {
    const query = parseQuery('age:>=30')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2)
  })

  it('should filter by less than or equal', () => {
    const query = parseQuery('age:<=30')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2)
  })

  it('should filter by not equal', () => {
    const query = parseQuery('role:!=admin')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2)
  })

  it('should filter by nested field', () => {
    const query = parseQuery('stats.tasks:>75')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('Jane Smith')
  })

  it('should filter with AND operator', () => {
    const query = parseQuery('role:developer AND age:>30')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('Bob Johnson')
  })

  it('should filter with OR operator', () => {
    const query = parseQuery('role:admin OR age:<30')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2)
  })

  it('should filter with complex query', () => {
    const query = parseQuery('role:developer AND stats.tasks:>=75')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2)
  })

  it('should search across all fields when no field specified', () => {
    const query = parseQuery('john')
    const result = filterByQuery(testData, query, ['name', 'email'])
    expect(result).toHaveLength(2)
  })

  it('should return all data when query is empty', () => {
    const query = parseQuery('')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(3)
  })

  it('should handle quoted values', () => {
    const query = parseQuery('name:"Jane Smith"')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('Jane Smith')
  })

  it('should handle case-insensitive matching', () => {
    const query = parseQuery('name:JOHN')
    const result = filterByQuery(testData, query)
    expect(result).toHaveLength(2)
  })
})

describe('extractFields', () => {
  it('should extract top-level fields', () => {
    const data = [
      { name: 'John', age: 30, email: 'john@example.com' },
      { name: 'Jane', age: 25, email: 'jane@example.com' },
    ]
    const fields = extractFields(data)
    expect(fields).toContain('name')
    expect(fields).toContain('age')
    expect(fields).toContain('email')
  })

  it('should extract nested fields', () => {
    const data = [
      {
        name: 'John',
        stats: { tasks: 50, projects: 5 },
        profile: { bio: 'Developer' },
      },
    ]
    const fields = extractFields(data)
    expect(fields).toContain('name')
    expect(fields).toContain('stats')
    expect(fields).toContain('stats.tasks')
    expect(fields).toContain('stats.projects')
    expect(fields).toContain('profile')
    expect(fields).toContain('profile.bio')
  })

  it('should return empty array for empty data', () => {
    const fields = extractFields([])
    expect(fields).toHaveLength(0)
  })

  it('should limit depth', () => {
    const data = [
      {
        level1: {
          level2: {
            level3: {
              level4: 'value',
            },
          },
        },
      },
    ]
    const fields = extractFields(data, 2)
    expect(fields).toContain('level1')
    expect(fields).toContain('level1.level2')
    // Should not go deeper than maxDepth
    expect(fields.some((f) => f.includes('level3'))).toBe(false)
  })

  it('should return sorted fields', () => {
    const data = [{ zebra: 1, apple: 2, banana: 3 }]
    const fields = extractFields(data)
    expect(fields[0]).toBe('apple')
    expect(fields[1]).toBe('banana')
    expect(fields[2]).toBe('zebra')
  })
})

describe('Edge Cases', () => {
  it('should handle null values in data', () => {
    const data = [
      { name: 'John', age: null },
      { name: 'Jane', age: 25 },
    ]
    const query = parseQuery('age:!=null')
    const result = filterByQuery(data, query)
    expect(result).toHaveLength(1)
    expect(result[0].name).toBe('Jane')
  })

  it('should handle undefined values in data', () => {
    const data = [
      { name: 'John', email: undefined },
      { name: 'Jane', email: 'jane@example.com' },
    ]
    const query = parseQuery('email:jane')
    const result = filterByQuery(data, query)
    expect(result).toHaveLength(1)
  })

  it('should handle empty strings', () => {
    const data = [
      { name: 'John', description: '' },
      { name: 'Jane', description: 'Developer' },
    ]
    const query = parseQuery('description:Developer')
    const result = filterByQuery(data, query)
    expect(result).toHaveLength(1)
  })

  it('should handle special characters in values', () => {
    const data = [
      { email: 'john+test@example.com' },
      { email: 'jane@example.com' },
    ]
    const query = parseQuery('email:john+test')
    const result = filterByQuery(data, query)
    expect(result).toHaveLength(1)
  })

  it('should handle numeric strings', () => {
    const data = [{ code: '123' }, { code: '456' }, { code: '789' }]
    const query = parseQuery('code:>400')
    const result = filterByQuery(data, query)
    expect(result).toHaveLength(2)
  })
})
