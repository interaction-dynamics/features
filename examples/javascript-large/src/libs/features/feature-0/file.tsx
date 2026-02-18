// --feature-flag name: feature-flag-1, type: rollout, status: active
// Import from feature-1 to show sibling dependency
import { Foo } from '../feature-1/components/foo';

export function createFeature0() {
  return {
    name: 'Feature 0',
    description: 'This is feature 0',
    enabled: true,
    relatedComponent: Foo,
  }
}
