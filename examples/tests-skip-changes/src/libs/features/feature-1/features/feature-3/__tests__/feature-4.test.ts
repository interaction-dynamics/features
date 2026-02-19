import { describe, expect, it } from 'vitest'
import { createFeature3 } from '../feature-4'

describe('Feature 3', () => {
  it('should be created', () => {
    const feature = createFeature3()
    expect(feature).toBeDefined()
  })
})
