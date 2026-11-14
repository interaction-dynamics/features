import { createContext } from 'react'
import type { Feature, ReconciliationError } from '@/models/feature'

export interface FeaturesContextValue {
  features: Feature[]
  featuresMap: Record<string, Feature>
  isLoading: boolean
  isValidating: boolean
  /** True when features.json is an aggregated report of several repositories */
  isAggregated: boolean
  /** Repositories of an aggregated report */
  repositories: string[]
  /** Features of an aggregated report that could not be merged across repositories */
  reconciliationErrors: ReconciliationError[]
}

export const FeaturesContext = createContext<FeaturesContextValue>({
  features: [],
  featuresMap: {},
  isLoading: false,
  isValidating: false,
  isAggregated: false,
  repositories: [],
  reconciliationErrors: [],
})
