import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { cn } from '@/lib/utils'
import type { HealthScore } from './health-score'

/** Health score with a small bar and the reasons in a tooltip */
export function HealthCell({ health }: { health: HealthScore }) {
  const { score, penalties, missing } = health

  return (
    <Tooltip>
      <TooltipTrigger
        className={cn(
          'inline-flex items-center justify-end gap-2 tabular-nums',
          score === undefined && 'text-muted-foreground/50',
        )}
        data-testid="health-score"
      >
        {score === undefined ? (
          '—'
        ) : (
          <>
            <span
              aria-hidden
              className="h-1.5 w-10 overflow-hidden rounded-full bg-muted"
            >
              <span
                className={cn(
                  'block h-full rounded-full',
                  score >= 80
                    ? 'bg-foreground/30'
                    : score >= 50
                      ? 'bg-foreground/60'
                      : 'bg-foreground',
                )}
                style={{ width: `${score}%` }}
              />
            </span>
            <span className={cn(score < 50 && 'font-semibold')}>{score}</span>
          </>
        )}
      </TooltipTrigger>
      <TooltipContent className="max-w-xs">
        {score === undefined ? (
          <p>Not enough data to compute a health score.</p>
        ) : (
          <>
            <p className="font-semibold">Health {score} / 100</p>
            {penalties.length === 0 ? (
              <p>No issue found.</p>
            ) : (
              <ul className="mt-1 space-y-0.5">
                {penalties.map((penalty) => (
                  <li
                    key={penalty.signal}
                    className="flex justify-between gap-4"
                  >
                    <span>{penalty.reason}</span>
                    <span className="tabular-nums">−{penalty.points}</span>
                  </li>
                ))}
              </ul>
            )}
          </>
        )}
        {missing.length > 0 && (
          <ul className="mt-1 space-y-0.5 opacity-70">
            {missing.map((item) => (
              <li key={item.signal}>{item.reason}</li>
            ))}
          </ul>
        )}
      </TooltipContent>
    </Tooltip>
  )
}
