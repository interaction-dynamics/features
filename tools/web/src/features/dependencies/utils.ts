import type { Dependency, Feature } from '@/models/feature'

export interface GroupedDependency {
  feature: string
  type: 'parent' | 'child' | 'sibling'
  count: number
  items: Dependency[]
}

/**
 * Build a map of feature paths to their dependencies (recursively)
 * Also builds a name-to-path mapping to resolve feature names to paths
 */
export function buildDependencyMap(
  features: Feature[],
): Map<string, Set<string>> {
  const pathToDeps = new Map<string, Set<string>>()
  const nameToPath = new Map<string, string>()

  const collectDeps = (feature: Feature) => {
    const deps = new Set<string>()
    feature.dependencies.forEach((dep) => {
      deps.add(dep.featurePath)
    })
    pathToDeps.set(feature.path, deps)
    nameToPath.set(feature.name, feature.path)

    // Recursively collect from nested features
    feature.features?.forEach((nested) => {
      collectDeps(nested)
    })
  }

  features.forEach((feature) => {
    collectDeps(feature)
  })

  // Convert dependency names to paths
  const pathToPathDeps = new Map<string, Set<string>>()
  pathToDeps.forEach((depNames, path) => {
    const depPaths = new Set<string>()
    depNames.forEach((depName) => {
      const depPath = nameToPath.get(depName)
      if (depPath) {
        depPaths.add(depPath)
      }
    })
    pathToPathDeps.set(path, depPaths)
  })

  return pathToPathDeps
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
 * Detect alerts for a dependency group
 */
export function detectAlerts(
  group: GroupedDependency,
  currentFeaturePath: string,
  dependencyMap: Map<string, Set<string>>,
  nameToPath: Map<string, string>,
): string[] {
  const alerts: string[] = []

  // Check for circular dependency: if the feature we depend on also depends on us
  // Convert the feature name to a path first
  const targetFeaturePath = nameToPath.get(group.feature)
  if (targetFeaturePath) {
    const targetDeps = dependencyMap.get(targetFeaturePath)
    if (targetDeps && targetDeps.has(currentFeaturePath)) {
      alerts.push('Circular Dependency')
    }
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
