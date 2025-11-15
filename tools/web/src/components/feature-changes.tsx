import { Calendar, ExternalLink, GitCommitVertical, User } from 'lucide-react'
import { Card, CardContent } from '@/components/ui/card'
import { useMetadata } from '@/hooks/use-metadata'
import { buildCommitUrl } from '@/lib/git-utils'
import type { Change } from '@/models/feature'

interface FeatureChangesProps {
  changes: Change[]
}

export function FeatureChanges({ changes }: FeatureChangesProps) {
  const { metadata } = useMetadata()

  if (changes.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-8">
        <GitCommitVertical className="h-12 w-12 text-muted-foreground mb-4" />
        <p className="text-sm text-muted-foreground">No changes found</p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      {changes.map((change, index) => (
        <Card key={`${change.hash}-${index}`}>
          <CardContent className="space-y-3">
            <div className="text-base font-semibold flex items-center gap-2">
              <GitCommitVertical className="h-4 w-4" />
              {change.title}
            </div>
            <div className="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
              <div className="flex items-center gap-1">
                <User className="h-3 w-3" />
                <span>{change.author_name}</span>
                <span className="text-xs select-all">
                  ({change.author_email})
                </span>
              </div>
              <div className="flex items-center gap-1">
                <Calendar className="h-3 w-3" />
                <span>
                  {new Date(change.date).toLocaleDateString(navigator.language)}
                </span>
              </div>
              <div className="flex items-center gap-1">
                {metadata?.repository ? (
                  <a
                    href={
                      buildCommitUrl(metadata.repository, change.hash) || '#'
                    }
                    target="_blank"
                    rel="noopener noreferrer"
                    className="bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-full px-2 py-0.5 text-xs font-mono inline-flex items-center gap-1 transition-colors"
                    title="View commit in repository"
                  >
                    #{change.hash.slice(0, 7)}
                    <ExternalLink className="h-2.5 w-2.5" />
                  </a>
                ) : (
                  <span className="bg-secondary text-secondary-foreground rounded-full px-2 py-0.5 text-xs font-mono">
                    #{change.hash.slice(0, 7)}
                  </span>
                )}
              </div>
            </div>
            {change.description && (
              <div className="text-sm">
                <p className="whitespace-pre-wrap">{change.description}</p>
              </div>
            )}
          </CardContent>
        </Card>
      ))}
    </div>
  )
}
