import { HelpCircle } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Button } from './ui/button'

interface HelpButtonProps {
  className?: string
  title?: string
  url?: string
}

export function HelpButton({ className, title, url }: HelpButtonProps) {
  return (
    <Button
      variant="link"
      onClick={() => window.open(url, '_blank')}
      className={cn(
        'text-muted-foreground text-xs hover:text-foreground transition-colors flex items-center gap-1 p-0 px-0!',
        className,
      )}
      title={title}
    >
      <HelpCircle className="h-4 w-4" />
      <span className="text-sm">{title}</span>
    </Button>
  )
}
