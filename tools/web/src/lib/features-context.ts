import { createContext } from 'react'
import type { Feature } from '@/models/feature'

export const FeaturesContext = createContext<Feature[]>([])
