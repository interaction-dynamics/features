import { Search, X } from 'lucide-react'
import { useEffect, useRef } from 'react'
import { Input } from './ui/input'

type AppSidebarSearchInputProps = {
  value: string
  onChange: (v: string) => void
}

export function AppSidebarSearchInput({
  value,
  onChange,
}: AppSidebarSearchInputProps) {
  const searchInputRef = useRef<HTMLInputElement>(null)

  // Handle Meta+K shortcut to focus search
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.metaKey && event.key === 'k') {
        event.preventDefault()
        searchInputRef.current?.focus()
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [])

  return (
    <div className="relative">
      <Search className="absolute left-2 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
      <Input
        ref={searchInputRef}
        placeholder="Search features..."
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="pl-8 pr-16"
      />
      <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1">
        {value.trim() ? (
          <button
            type="button"
            onClick={() => onChange('')}
            className="cursor-pointer inline-flex h-5 w-5 items-center justify-center rounded hover:bg-muted text-muted-foreground hover:text-foreground transition-colors"
            aria-label="Clear search"
          >
            <X className="h-3 w-3" />
          </button>
        ) : (
          <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
            <span className="text-xs">⌘</span>K
          </kbd>
        )}
      </div>
    </div>
  )
}
