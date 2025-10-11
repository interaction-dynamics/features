import { createContext } from 'react'
import type { Feature } from '@/models/feature'

export interface FeaturesContextValue {
  features: Feature[]
  isLoading: boolean
  isValidating: boolean
}

export const FeaturesContext = createContext<FeaturesContextValue>({
  features: [],
  isLoading: false,
  isValidating: false,
})
