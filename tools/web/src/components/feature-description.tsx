import { HelpButton } from './help-button'
import { MarkdownRenderer } from './markdown-renderer'
import { Card, CardContent } from './ui/card'

interface FeatureDescriptionProps {
  description?: string
}

export function FeatureDescription({ description }: FeatureDescriptionProps) {
  if (!description) {
    return (
      <Card>
        <CardContent>
          <div className="flex flex-col items-start gap-2">
            <p className="text-sm text-muted-foreground">
              No description available
            </p>
            <HelpButton
              title="How to add a description"
              url="https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-feature-description"
            />
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardContent>
        <MarkdownRenderer markdown={description} />
      </CardContent>
    </Card>
  )
}
