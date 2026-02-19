import { describe, expect, it } from 'vitest'
import { createFeature4 } from '../feature-4'

describe('Feature 4', () => {
  it('should be created', () => {
    const feature = createFeature4()
    expect(feature).toBeDefined()
  })
})
