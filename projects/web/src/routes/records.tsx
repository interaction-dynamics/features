import { useContext } from 'react'
import { Header } from '@/components/header'
import { RecordsTimeline } from '@/features/records'
import { FeaturesContext } from '@/lib/features-context'

export default function ProjectRecords() {
  const { projectRecords } = useContext(FeaturesContext)

  return (
    <div className="absolute inset-0 overflow-auto">
      <Header>Specs &amp; Decisions</Header>
      <div className="flex flex-col gap-4 p-4">
        <p className="text-sm text-muted-foreground">
          Project-level specs and decisions: records at the root of the project
          that are not attached to a feature. Add{' '}
          <code className="font-mono text-xs">feature: &lt;name&gt;</code> to a
          record's frontmatter to show it on that feature instead.
        </p>
        <RecordsTimeline records={projectRecords} featurePath={null} />
      </div>
    </div>
  )
}
