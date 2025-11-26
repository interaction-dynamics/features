import { describe, expect, it } from 'vitest'
import { formatFeatureName } from '@/lib/format-feature-name'

describe('formatFeatureName', () => {
  it('should handle empty string', () => {
    expect(formatFeatureName('')).toBe('')
  })

  it('should handle single word', () => {
    expect(formatFeatureName('feature')).toBe('Feature')
  })

  it('should replace underscores with spaces', () => {
    expect(formatFeatureName('my_feature_name')).toBe('My Feature Name')
  })

  it('should replace hyphens with spaces', () => {
    expect(formatFeatureName('my-feature-name')).toBe('My Feature Name')
  })

  it('should handle mixed underscores and hyphens', () => {
    expect(formatFeatureName('my_feature-name')).toBe('My Feature Name')
  })

  it('should handle camelCase', () => {
    expect(formatFeatureName('myFeatureName')).toBe('My Feature Name')
  })

  it('should handle PascalCase', () => {
    expect(formatFeatureName('MyFeatureName')).toBe('My Feature Name')
  })

  it('should handle acronyms followed by lowercase', () => {
    expect(formatFeatureName('HTTPServer')).toBe('Http Server')
  })

  it('should handle acronyms at the end', () => {
    expect(formatFeatureName('myAPI')).toBe('My Api')
  })

  it('should handle all uppercase', () => {
    expect(formatFeatureName('FEATURE')).toBe('Feature')
  })

  it('should handle all lowercase', () => {
    expect(formatFeatureName('feature')).toBe('Feature')
  })

  it('should handle mixed case with underscores', () => {
    expect(formatFeatureName('my_Feature_Name')).toBe('My Feature Name')
  })

  it('should handle complex name with camelCase and separators', () => {
    expect(formatFeatureName('myFeature_nameWith-hyphen')).toBe(
      'My Feature Name With Hyphen',
    )
  })

  it('should clean up multiple spaces', () => {
    expect(formatFeatureName('feature  name')).toBe('Feature Name')
  })

  it('should trim leading and trailing spaces', () => {
    expect(formatFeatureName('  feature_name  ')).toBe('Feature Name')
  })

  it('should handle numbers', () => {
    expect(formatFeatureName('feature1')).toBe('Feature1')
  })

  it('should handle names with numbers and separators', () => {
    expect(formatFeatureName('feature_1_name')).toBe('Feature 1 Name')
  })

  it('should handle consecutive uppercase letters', () => {
    expect(formatFeatureName('XMLParser')).toBe('Xml Parser')
  })

  it('should handle single letter words', () => {
    expect(formatFeatureName('a_b_c')).toBe('A B C')
  })

  it('should handle mixed patterns', () => {
    expect(formatFeatureName('myHTTPSConnection_handler-v2')).toBe(
      'My Https Connection Handler V2',
    )
  })
})
