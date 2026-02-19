// --feature-flag name: feature-flag-1, type: rollout, status: active
// Import from feature-1 to show sibling dependency
import { Foo } from '../feature-1/components/foo';
// Import from the OTHER feature-0 (in routes) to test path-based resolution
import { ROUTE_FEATURE_0_CONFIG } from '../../../routes/route-1/features/feature-0/route-utils';

export function createFeature0() {
  return {
    name: 'Feature 0',
    description: 'This is feature 0',
    enabled: true,
    relatedComponent: Foo,
    routeConfig: ROUTE_FEATURE_0_CONFIG,
  }
}
