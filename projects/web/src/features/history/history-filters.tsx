import { Search, X } from 'lucide-react'
import { Input } from '@/components/ui/input'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import type { CommitType, HistoryFilterState as Filters } from './utils'

const chipClass =
  'inline-flex items-center gap-1 rounded-md border px-2 py-0.5 text-xs font-medium transition-colors'

interface HistoryFiltersProps {
  filters: Filters
  onChange: (filters: Filters) => void
  /** Commit types present in the history, with their counts */
  types: [CommitType, number][]
  selectedFeature: Feature | undefined
}

export function HistoryFilters({
  filters,
  onChange,
  types,
  selectedFeature,
}: HistoryFiltersProps) {
  return (
    <div className="flex flex-col gap-3">
      <div className="flex flex-wrap items-center gap-2">
        <div className="relative w-full sm:w-72">
          <Search className="pointer-events-none absolute left-2.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            type="search"
            value={filters.query}
            onChange={(event) =>
              onChange({ ...filters, query: event.target.value })
            }
            placeholder="Search commits, authors, hashes…"
            aria-label="Search commits"
            className="pl-8"
          />
        </div>
        {selectedFeature && (
          <span
            className={cn(chipClass, 'bg-secondary py-1')}
            data-testid="history-feature-filter"
          >
            Feature: {formatFeatureName(selectedFeature.name)}
            <button
              type="button"
              onClick={() => onChange({ ...filters, featurePath: null })}
              className="rounded-sm text-muted-foreground hover:text-foreground"
              aria-label="Clear feature filter"
            >
              <X className="h-3 w-3" />
            </button>
          </span>
        )}
      </div>
      {types.length > 1 && (
        <fieldset className="flex flex-wrap gap-1.5">
          <legend className="sr-only">Commit type</legend>
          {types.map(([type, count]) => {
            const active = filters.type === type
            return (
              <button
                key={type}
                type="button"
                aria-pressed={active}
                onClick={() =>
                  onChange({ ...filters, type: active ? null : type })
                }
                className={cn(
                  chipClass,
                  active
                    ? 'border-primary bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:bg-muted hover:text-foreground',
                )}
              >
                {type}
                <span className="tabular-nums opacity-70">{count}</span>
              </button>
            )
          })}
        </fieldset>
      )}
    </div>
  )
}
