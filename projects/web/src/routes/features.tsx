import { useContext } from 'react'
import { FeatureBreadcrumb } from '@/components/feature-breadcrumb'
import { FeatureDetails } from '@/components/feature-details'
import { Header } from '@/components/header'
import { HelpButton } from '@/components/help-button'
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
      <div className="flex flex-1 flex-col gap-4 p-4">
        {features.length > 0 ? (
          selectedFeature === null ? (
            <div className="flex flex-1 items-center justify-center">
              <p className="text-muted-foreground text-lg">Select a feature</p>
            </div>
          ) : (
            <FeatureDetails feature={selectedFeature} />
          )
        ) : (
          <div className="flex flex-1 items-center justify-center">
            <p className="text-muted-foreground text-lg">No feature found</p>
            <HelpButton
              className="ms-2"
              tooltip="How to add features"
              url="https://github.com/interaction-dynamics/features?tab=readme-ov-file#guidelines"
            />
          </div>
        )}
      </div>
    </div>
  )
}
