import type { Feature } from '@/models/feature'

// Recursively find owner by traversing parent chain
export const resolveOwner = (currentFeature: Feature): string => {
  if (currentFeature.owner !== 'Unknown') {
    return currentFeature.owner
  }
  if (currentFeature.parent) {
    return resolveOwner(currentFeature.parent)
  }
  return 'Unknown'
}
