import { useContext } from 'react'
import { FeatureBreadcrumb } from '@/components/feature-breadcrumb'
import { FeatureDetails } from '@/components/feature-details'
import { Header } from '@/components/header'
import { NoFeatures } from '@/features/no-features'
import { FeatureSelectedContext } from '@/lib/feature-selected-context'
import { FeaturesContext } from '@/lib/features-context'

export default function Features() {
  const { selectFeature, selectedFeature } = useContext(FeatureSelectedContext)
  const { features } = useContext(FeaturesContext)
  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>
        <FeatureBreadcrumb
          feature={selectedFeature}
          onFeatureClick={selectFeature}
        />
      </Header>
      {features.length > 0 ? (
        <div className="flex flex-1 flex-col gap-4 p-4">
          {selectedFeature === null ? (
            <div className="flex flex-1 items-center justify-center">
              <p className="text-muted-foreground text-lg">Select a feature</p>
            </div>
          ) : (
            <FeatureDetails feature={selectedFeature} />
          )}
        </div>
      ) : (
        <NoFeatures />
      )}
    </div>
  )
}
