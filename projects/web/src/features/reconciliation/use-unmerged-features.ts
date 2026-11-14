import { useContext, useMemo } from 'react'
import { FeaturesContext } from '@/lib/features-context'
import type { ReconciliationOccurrence } from '@/models/feature'

/**
 * Features of an aggregated report that could not be merged with the features
 * of the same name in other repositories, indexed by their aggregated path.
 */
export function useUnmergedFeatures(): Map<string, ReconciliationOccurrence> {
  const { reconciliationErrors } = useContext(FeaturesContext)

  return useMemo(() => {
    const map = new Map<string, ReconciliationOccurrence>()
    for (const error of reconciliationErrors) {
      for (const occurrence of error.occurrences) {
        map.set(occurrence.featurePath, occurrence)
      }
    }
    return map
  }, [reconciliationErrors])
}
