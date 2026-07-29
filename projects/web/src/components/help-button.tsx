import { HelpCircle } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Button } from './ui/button'

interface HelpButtonProps {
  className?: string
  title?: string
  tooltip?: string
  url: string
  size?: 'medium' | 'small'
}

export function HelpButton({
  className,
  title,
  url,
  tooltip,
  size = 'medium',
}: HelpButtonProps) {
  return (
    <Button
      variant="link"
      onClick={() => window.open(url, '_blank')}
      className={cn(
        'text-muted-foreground text-xs hover:text-foreground transition-colors flex items-center gap-1 p-0 px-0!',
        className,
        size === 'small' && 'p-0 h-auto',
      )}
      title={tooltip ?? title}
    >
      <HelpCircle className={cn(size === 'medium' ? 'h-4 w-4' : 'h-3 w-3')} />
      <span className="text-sm">{title}</span>
    </Button>
  )
}
