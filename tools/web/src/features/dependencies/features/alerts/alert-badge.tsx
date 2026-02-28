import { AlertTriangle } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'

interface AlertBadgeProps {
  label: string
  /** 'sm' uses a more compact style for tight spaces (e.g. inline list items) */
  size?: 'default' | 'sm'
  className?: string
}

export function AlertBadge({
  label,
  size = 'default',
  className,
}: AlertBadgeProps) {
  const isSmall = size === 'sm'
  return (
    <Badge
      variant="destructive"
      className={cn(
        'gap-1',
        isSmall ? 'px-1.5 py-px text-[10px]' : 'text-xs',
        className,
      )}
    >
      <AlertTriangle className={isSmall ? 'h-2.5 w-2.5' : 'h-3 w-3'} />
      {label}
    </Badge>
  )
}
