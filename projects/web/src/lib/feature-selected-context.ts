import { createContext } from 'react'
import type { Feature } from '@/models/feature'

export interface FeatureSelectedContextValue {
  selectedFeature: Feature | null
  selectFeature: (f: Feature | null) => void
}

export const FeatureSelectedContext =
  createContext<FeatureSelectedContextValue>({
    selectedFeature: null,
    selectFeature: () => {},
  })
