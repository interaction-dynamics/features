import type { Feature } from '@/models/feature'
import { recordsOf } from './record-utils'
import { RecordsTimeline } from './records-timeline'

export default function FeatureRecords({ feature }: { feature: Feature }) {
  return (
    <RecordsTimeline records={recordsOf(feature)} featurePath={feature.path} />
  )
}
