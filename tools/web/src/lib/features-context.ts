import { createContext } from 'react'
import type { Feature } from '@/models/feature'

export interface FeaturesContextValue {
  features: Feature[]
  featuresMap: Record<string, Feature>
  isLoading: boolean
  isValidating: boolean
}

export const FeaturesContext = createContext<FeaturesContextValue>({
  features: [],
  featuresMap: {},
  isLoading: false,
  isValidating: false,
})
