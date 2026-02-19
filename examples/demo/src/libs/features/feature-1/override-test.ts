// --feature-flag feature: feature-2, type: cross-reference, source: feature-1

// This comment is inside feature-1 folder but explicitly links to feature-2
export function sharedWithFeature2() {
  return "shared";
}
