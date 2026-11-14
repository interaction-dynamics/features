import { GitBranch, TriangleAlert } from 'lucide-react'
import { Link } from 'react-router'
import type { Feature } from '@/models/feature'
import { useUnmergedFeatures } from './use-unmerged-features'

type FeatureRepositoriesProps = {
  feature: Feature
}

/** Repositories where a feature of an aggregated report was found */
export function FeatureRepositories({ feature }: FeatureRepositoriesProps) {
  const unmerged = useUnmergedFeatures().get(feature.path)
  const repositories = feature.repositories ?? []

  if (repositories.length === 0) return null

  return (
    <div className="flex items-start gap-3">
      <GitBranch className="h-4 w-4 text-muted-foreground mt-0.5" />
      <div className="flex-1">
        <p className="text-sm font-medium text-foreground mb-1">
          {repositories.length > 1
            ? `Repositories (${repositories.length})`
            : 'Repository'}
        </p>
        <ul className="flex flex-col gap-1">
          {repositories.map((repository) => (
            <li
              key={`${repository.name}:${repository.path}`}
              className="flex flex-wrap items-center gap-2 text-xs"
            >
              <span className="rounded bg-secondary px-1.5 py-0.5 font-medium text-secondary-foreground">
                {repository.name}
              </span>
              <span className="font-mono text-muted-foreground">
                {repository.path}
              </span>
            </li>
          ))}
        </ul>
        {unmerged && (
          <p className="mt-2 flex items-center gap-1.5 text-xs text-amber-700 dark:text-amber-400">
            <TriangleAlert className="h-3.5 w-3.5" />
            Not merged with the features named "{unmerged.feature}" of other
            repositories.
            <Link to="/reconciliation" className="underline">
              See reconciliation issues
            </Link>
          </p>
        )}
      </div>
    </div>
  )
}
