import type { ReactNode } from 'react'
import { Navigate } from 'react-router'
import { useExperimentalFeatures } from '@/hooks/use-experimental-features'

/** Renders an experimental page, or redirects home when experimental pages are disabled */
export function ExperimentalRoute({ children }: { children: ReactNode }) {
  const { enabled, isLoading } = useExperimentalFeatures()

  if (enabled) return children
  if (isLoading) return null

  return <Navigate to="/" replace />
}
