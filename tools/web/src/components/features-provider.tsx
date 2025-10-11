import useSWR from 'swr'
import { FeaturesContext } from '@/lib/features-context'
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

const fetcher = async (url: string): Promise<Feature[]> => {
  const response = await fetch(url)
  const data = await response.json()
  return addParentReferences(data)
}

export function FeaturesProvider({ children }: { children: React.ReactNode }) {
  const { data: features } = useSWR<Feature[]>('./features.json', fetcher, {
    suspense: true,
    refreshInterval: 1000,
  })

  return (
    <FeaturesContext.Provider value={features ?? []}>
      {children}
    </FeaturesContext.Provider>
  )
}
