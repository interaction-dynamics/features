// --feature-experiment hypothesis: improve-ux, status: active

export function experimentalFeature() {
  // This should automatically link to feature-2 without explicit feature: property
  return "improved UX";
}

// --feature-toggle enabled: true, rollout: 100

export const FEATURE_ENABLED = true;
