import { Link } from 'react-router'
import { useExperimentalFeatures } from '@/hooks/use-experimental-features'
import { cn } from '@/lib/utils'
import { getOwnerName } from '@/models/owner'
import { ownerPath } from './utils'

type OwnerLinkProps = {
  owner: string
  className?: string
}

/**
 * Name of an owner, linking to its page when the experimental pages are
 * enabled (the owner page is experimental), plain text otherwise
 */
export function OwnerLink({ owner, className }: OwnerLinkProps) {
  const { enabled } = useExperimentalFeatures()
  const name = getOwnerName(owner)

  if (!enabled) return <span className={className}>{name}</span>

  return (
    <Link
      to={ownerPath(owner)}
      className={cn(
        'underline-offset-4 hover:text-foreground hover:underline',
        className,
      )}
      title={`See everything owned by ${name}`}
      data-testid="owner-link"
    >
      {name}
    </Link>
  )
}
