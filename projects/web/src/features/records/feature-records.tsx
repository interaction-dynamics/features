import type { Feature } from '@/models/feature'
import { RecordsEmptyState } from './records-empty-state'
import { RecordsTimeline } from './records-timeline'
import { featureRecords } from './utils'

interface FeatureRecordsProps {
  feature: Feature
}

/** "Specs & Decisions" tab of a feature */
export default function FeatureRecords({ feature }: FeatureRecordsProps) {
  const records = featureRecords(feature)
  if (records.length === 0) {
    return <RecordsEmptyState folder={`${feature.path}/__docs__`} />
  }
  return <RecordsTimeline records={records} />
}
