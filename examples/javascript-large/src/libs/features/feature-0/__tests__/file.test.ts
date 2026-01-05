import { describe, expect, it } from 'vitest'
import { createFeature0 } from '../file'

describe('Feature 0', () => {
  it('should be created', () => {
    const feature = createFeature0()
    expect(feature).toBeDefined()
  })
})
