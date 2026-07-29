import { useMemo } from 'react'
import useSWR from 'swr'
import {
  FeaturesContext,
  type FeaturesContextValue,
} from '@/lib/features-context'
import type { Feature } from '@/models/feature'

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

const fetcher = async (url: string): Promise<Feature[]> => {
  const response = await fetch(url)
  const data = await response.json()
  return addParentReferences(data)
}

export function FeaturesProvider({ children }: { children: React.ReactNode }) {
  const {
    data: features,
    isLoading,
    isValidating,
  } = useSWR<Feature[]>('./features.json', fetcher, {
    suspense: true,
    refreshInterval: import.meta.env.WATCH === 'on' ? 1000 : undefined,
  })

  const contextValue: FeaturesContextValue = useMemo(
    () => ({
      features: features ?? [],
      featuresMap: buildFeaturesMap(features ?? []),
      isLoading: isLoading ?? false,
      isValidating: isValidating ?? false,
    }),
    [features, isLoading, isValidating],
  )

  return (
    <FeaturesContext.Provider value={contextValue}>
      {children}
    </FeaturesContext.Provider>
  )
}
