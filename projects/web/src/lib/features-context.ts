import { createContext } from 'react'
import type { DocRecord, Feature, ReconciliationError } from '@/models/feature'

export interface FeaturesContextValue {
  features: Feature[]
  featuresMap: Record<string, Feature>
  isLoading: boolean
  isValidating: boolean
  /** True when features.json is an aggregated report of several repositories */
  isAggregated: boolean
  /** Repositories of an aggregated report */
  repositories: string[]
  /** Reconciliation issues of an aggregated report */
  reconciliationErrors: ReconciliationError[]
  /** Root specs and decisions not attached to a feature (records.json) */
  projectRecords: DocRecord[]
}

export const FeaturesContext = createContext<FeaturesContextValue>({
  features: [],
  featuresMap: {},
  isLoading: false,
  isValidating: false,
  isAggregated: false,
  repositories: [],
  reconciliationErrors: [],
  projectRecords: [],
})
