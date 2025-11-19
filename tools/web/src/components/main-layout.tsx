import { MessageSquare } from 'lucide-react'
import { useContext, useEffect, useState } from 'react'
import { Outlet, useLocation, useNavigate, useSearchParams } from 'react-router'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { FeatureSelectedContext } from '@/lib/feature-selected-context'
import { FeaturesContext } from '@/lib/features-context'
import type { Feature } from '@/models/feature'
import { AppSidebar } from './app-sidebar'

// Helper function to find a feature by path in the features tree
const findFeatureByPath = (
  features: Feature[],
  path: string,
): Feature | null => {
  for (const feature of features) {
    if (feature.path === path) {
      return feature
    }
    if (feature.features) {
      const found = findFeatureByPath(feature.features, path)
      if (found) {
        return found
      }
    }
  }
  return null
}

export function MainLayout() {
  const [searchParams] = useSearchParams()

  const featurePath = searchParams.get('feature')

  const { features } = useContext(FeaturesContext)
  const [currentFeature, setCurrentFeature] = useState<Feature | null>(null)

  const navigate = useNavigate()

  useEffect(() => {
    if (features.length > 0) {
      if (featurePath) {
        // Find the updated version of the current feature by path
        const updatedFeature = findFeatureByPath(features, featurePath)
        if (updatedFeature) {
          setCurrentFeature(updatedFeature)
        } else {
          // Feature was deleted, clear the selection
          setCurrentFeature(null)
        }
      } else {
        setCurrentFeature(features[0])
      }
    } else {
      setCurrentFeature(null)
    }
  }, [features, featurePath])

  const onSelectFeature = (feature: Feature) => {
    setCurrentFeature(feature)
    navigate({ pathname: '/', search: `?feature=${feature.path}` })
  }

  const location = useLocation()

  return (
    <FeatureSelectedContext.Provider
      value={{
        selectFeature: setCurrentFeature,
        selectedFeature: currentFeature,
      }}
    >
      <SidebarProvider>
        <AppSidebar
          activeFeature={location.pathname === '/' ? currentFeature : null}
          onFeatureClick={onSelectFeature}
        />
        <SidebarInset>
          <Outlet />
        </SidebarInset>
      </SidebarProvider>
    </FeatureSelectedContext.Provider>
  )
}
