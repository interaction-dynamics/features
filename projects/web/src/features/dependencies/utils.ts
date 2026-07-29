import type { Dependency, Feature } from '@/models/feature'

export interface GroupedDependency {
  feature: string
  type: 'parent' | 'child' | 'sibling'
  count: number
  items: Dependency[]
}

/**
 * Build a map of feature paths to the set of feature paths they depend on (recursively).
 * dep.featurePath is already a path, so no name-to-path conversion is needed.
 */
export function buildDependencyMap(
  features: Feature[],
): Map<string, Set<string>> {
  const dependencyMap = new Map<string, Set<string>>()

  const collectDeps = (feature: Feature) => {
    const deps = new Set<string>()
    feature.dependencies.forEach((dep) => {
      deps.add(dep.featurePath)
    })
    dependencyMap.set(feature.path, deps)

    feature.features?.forEach((nested) => {
      collectDeps(nested)
    })
  }

  features.forEach((feature) => {
    collectDeps(feature)
  })

  return dependencyMap
}

/**
 * Group dependencies by feature and type
 */
export function groupDependencies(
  dependencies: Dependency[],
): GroupedDependency[] {
  const grouped = dependencies.reduce<Record<string, GroupedDependency>>(
    (acc, dep) => {
      const key = `${dep.featurePath}-${dep.type}`
      if (!acc[key]) {
        acc[key] = {
          feature: dep.featurePath,
          type: dep.type,
          count: 0,
          items: [],
        }
      }
      acc[key].count++
      acc[key].items.push(dep)
      return acc
    },
    {},
  )

  return Object.values(grouped)
}

/**
 * Build a name-to-path mapping for all features
 */
export function buildNameToPathMap(features: Feature[]): Map<string, string> {
  const nameToPath = new Map<string, string>()

  const collectPaths = (feature: Feature) => {
    nameToPath.set(feature.name, feature.path)
    feature.features?.forEach((nested) => {
      collectPaths(nested)
    })
  }

  features.forEach((feature) => {
    collectPaths(feature)
  })

  return nameToPath
}

/**
 * Detect alerts for a dependency group.
 * group.feature is a featurePath (not a name), so we look it up directly in the dependencyMap.
 */
export function detectAlerts(
  group: GroupedDependency,
  currentFeaturePath: string,
  dependencyMap: Map<string, Set<string>>,
  _nameToPath: Map<string, string>,
): string[] {
  const alerts: string[] = []

  // Check for circular dependency: if the feature we depend on also depends on us
  const targetDeps = dependencyMap.get(group.feature)
  if (targetDeps && targetDeps.has(currentFeaturePath)) {
    alerts.push('Circular Dependency')
  }

  // Check for tight dependency (based on target files)
  const uniqueFiles = new Set(group.items.map((item) => item.targetFilename))
  const fileCount = uniqueFiles.size

  if (fileCount === 1 && group.count > 5) {
    alerts.push('Tight Dependency')
  } else if (fileCount >= 3 && group.count > 3) {
    alerts.push('Tight Dependency')
  }

  return alerts
}
