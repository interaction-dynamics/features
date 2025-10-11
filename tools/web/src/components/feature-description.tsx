import { HelpCircle } from 'lucide-react'
import { MarkdownRenderer } from './markown-renderer'
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
            <button
              type="button"
              onClick={() =>
                window.open(
                  'https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-feature-description',
                  '_blank',
                )
              }
              className="text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1"
              title="Learn how to add a decision"
            >
              <HelpCircle className="h-4 w-4" />
              <span className="text-sm">help</span>
            </button>
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
