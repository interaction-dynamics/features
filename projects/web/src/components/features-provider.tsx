import { useMemo } from 'react'
import useSWR from 'swr'
import {
  FeaturesContext,
  type FeaturesContextValue,
} from '@/lib/features-context'
import type {
  AggregatedReport,
  Feature,
  ReconciliationError,
} from '@/models/feature'

type FeaturesData = {
  features: Feature[]
  isAggregated: boolean
  repositories: string[]
  reconciliationErrors: ReconciliationError[]
}

const addParentReferences = (
  features: Feature[],
  parent?: Feature,
): Feature[] => {
  return features.map((feature) => {
    const featureWithParent = { ...feature, parent }
    if (feature.features && feature.features.length > 0) {
      featureWithParent.features = addParentReferences(
        feature.features,
        featureWithParent,
      )
    }
    return featureWithParent
  })
}

const buildFeaturesMap = (features: Feature[]): Record<string, Feature> => {
  const map: Record<string, Feature> = {}

  const addToMap = (feature: Feature) => {
    map[feature.path] = feature
    if (feature.features && feature.features.length > 0) {
      feature.features.forEach(addToMap)
    }
  }

  features.forEach(addToMap)
  return map
}

/**
 * features.json is either a list of features (single repository) or an
 * aggregated report `{ repositories, features, errors }` (`--aggregate`).
 */
export const parseFeaturesData = (
  data: Feature[] | AggregatedReport,
): FeaturesData => {
  if (Array.isArray(data)) {
    return {
      features: addParentReferences(data),
      isAggregated: false,
      repositories: [],
      reconciliationErrors: [],
    }
  }

  return {
    features: addParentReferences(data.features ?? []),
    isAggregated: true,
    repositories: data.repositories ?? [],
    reconciliationErrors: data.errors ?? [],
  }
}

const fetcher = async (url: string): Promise<FeaturesData> => {
  const response = await fetch(url)
  const data = await response.json()
  return parseFeaturesData(data)
}

export function FeaturesProvider({ children }: { children: React.ReactNode }) {
  const { data, isLoading, isValidating } = useSWR<FeaturesData>(
    './features.json',
    fetcher,
    {
      suspense: true,
      refreshInterval: import.meta.env.WATCH === 'on' ? 1000 : undefined,
    },
  )

  const contextValue: FeaturesContextValue = useMemo(
    () => ({
      features: data?.features ?? [],
      featuresMap: buildFeaturesMap(data?.features ?? []),
      isLoading: isLoading ?? false,
      isValidating: isValidating ?? false,
      isAggregated: data?.isAggregated ?? false,
      repositories: data?.repositories ?? [],
      reconciliationErrors: data?.reconciliationErrors ?? [],
    }),
    [data, isLoading, isValidating],
  )

  return (
    <FeaturesContext.Provider value={contextValue}>
      {children}
    </FeaturesContext.Provider>
  )
}
