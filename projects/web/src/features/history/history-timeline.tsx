import { ArrowUpRight, ExternalLink, Sparkles } from 'lucide-react'
import { renderTitleWithPRLinks } from '@/components/feature-changes'
import { OwnerDot } from '@/components/owner-dot'
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import { buildCommitUrl } from '@/lib/git-utils'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import { formatMonth, type HistoryCommit, type HistoryMonth } from './utils'

interface HistoryTimelineProps {
  months: HistoryMonth[]
  /** Repository URL, to link commits and pull requests */
  repository: string | undefined
  activeFeaturePath: string | null
  onFilterFeature: (feature: Feature) => void
  onOpenFeature: (feature: Feature) => void
}

export function HistoryTimeline({ months, ...props }: HistoryTimelineProps) {
  return (
    <div className="flex flex-col gap-6" data-testid="history-timeline">
      {months.map((month) => (
        <section key={month.key} aria-label={formatMonth(month.key)}>
          <h2 className="sticky top-16 z-10 -mx-4 mb-2 flex items-baseline gap-2 bg-background/95 px-4 py-2 text-sm font-semibold backdrop-blur">
            {formatMonth(month.key)}
            <span className="text-xs font-normal text-muted-foreground tabular-nums">
              {month.commits.length}{' '}
              {month.commits.length === 1 ? 'commit' : 'commits'}
            </span>
          </h2>
          <ol className="relative ml-1.5 border-l">
            {month.commits.map((commit) => (
              <CommitItem key={commit.hash} commit={commit} {...props} />
            ))}
          </ol>
        </section>
      ))}
    </div>
  )
}

function CommitItem({
  commit,
  repository,
  activeFeaturePath,
  onFilterFeature,
  onOpenFeature,
}: { commit: HistoryCommit } & Omit<HistoryTimelineProps, 'months'>) {
  const commitUrl = buildCommitUrl(repository, commit.hash)
  const introduces = commit.introduces.length > 0

  return (
    <li className="relative pb-4 pl-5 last:pb-0" data-testid="history-commit">
      <span
        className={cn(
          'absolute -left-[5px] top-1.5 size-2.5 rounded-full border-2 border-background',
          introduces ? 'bg-primary' : 'bg-muted-foreground/50',
        )}
        aria-hidden
      />
      <div className="flex flex-col gap-1.5">
        <div className="flex flex-wrap items-baseline gap-x-2 gap-y-1">
          <p className="text-sm font-medium leading-snug break-words">
            {renderTitleWithPRLinks(commit.title, repository)}
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
          <span>{commit.author_name || 'Unknown author'}</span>
          <span>{formatDate(commit.date.replace(' ', 'T'))}</span>
          {commitUrl ? (
            <a
              href={commitUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-0.5 font-mono hover:text-foreground"
              title="View commit in repository"
            >
              {commit.hash.slice(0, 7)}
              <ExternalLink className="h-2.5 w-2.5" />
            </a>
          ) : (
            <span className="font-mono">{commit.hash.slice(0, 7)}</span>
          )}
        </div>
        {introduces && (
          <p
            className="flex flex-wrap items-center gap-1 text-xs text-foreground"
            data-testid="history-introduces"
          >
            <Sparkles className="h-3 w-3 text-primary" />
            Introduces{' '}
            {commit.introduces.map((feature, index) => (
              <span key={feature.path}>
                <button
                  type="button"
                  className="font-medium hover:underline"
                  onClick={() => onOpenFeature(feature)}
                >
                  {formatFeatureName(feature.name)}
                </button>
                {index < commit.introduces.length - 1 && ','}
              </span>
            ))}
          </p>
        )}
        {commit.features.length > 0 && (
          <div className="flex flex-wrap gap-1.5">
            {commit.features.map((feature) => (
              <FeatureBadge
                key={feature.path}
                feature={feature}
                active={feature.path === activeFeaturePath}
                onFilter={() => onFilterFeature(feature)}
                onOpen={() => onOpenFeature(feature)}
              />
            ))}
          </div>
        )}
      </div>
    </li>
  )
}

function FeatureBadge({
  feature,
  active,
  onFilter,
  onOpen,
}: {
  feature: Feature
  active: boolean
  onFilter: () => void
  onOpen: () => void
}) {
  const name = formatFeatureName(feature.name)
  return (
    <span
      className={cn(
        'inline-flex items-center overflow-hidden rounded-md border text-xs',
        active && 'border-primary',
      )}
      data-testid="history-feature-badge"
    >
      <button
        type="button"
        onClick={onFilter}
        className="inline-flex items-center gap-1.5 px-2 py-0.5 hover:bg-muted"
        title={`Show the history of ${name} (${feature.path})`}
      >
        <OwnerDot owner={feature.owner} />
        {name}
      </button>
      <button
        type="button"
        onClick={onOpen}
        className="border-l px-1 py-0.5 text-muted-foreground hover:bg-muted hover:text-foreground"
        aria-label={`Open ${name}`}
        title={`Open ${name}`}
      >
        <ArrowUpRight className="h-3 w-3" />
      </button>
    </span>
  )
}
