// --feature-toggle feature:feature-1, enabled: true, rollout_percentage: 50

export function isFeatureEnabled(userId: string): boolean {
  // Simple hash-based rollout
  const hash = userId.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
  return (hash % 100) < 50;
}
