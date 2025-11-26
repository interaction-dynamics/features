import { Info, Search, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Input } from '@/components/ui/input'
import { cn } from '@/lib/utils'

type SmartSearchInputProps = {
  value: string
  onChange: (value: string) => void
  onClear?: () => void
  placeholder?: string
  className?: string
  fields?: string[]
}

export function SmartSearchInput({
  value,
  onChange,
  onClear,
  placeholder = 'Search... (e.g., owner:john AND lines:>1000)',
  className,
  fields = [],
}: SmartSearchInputProps) {
  const handleClear = () => {
    onChange('')
    onClear?.()
  }

  return (
    <div className={cn('flex items-center gap-2', className)}>
      <div className="relative flex-1">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          className="pl-9 pr-9"
        />
        {value && (
          <Button
            variant="ghost"
            size="sm"
            onClick={handleClear}
            className="absolute right-1 top-1/2 h-7 w-7 -translate-y-1/2 p-0"
          >
            <X className="h-4 w-4" />
            <span className="sr-only">Clear search</span>
          </Button>
        )}
      </div>

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="sm" className="h-9 w-9 p-0">
            <Info className="h-4 w-4" />
            <span className="sr-only">Search syntax help</span>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-96" align="end">
          <div className="space-y-3 p-2">
            <div>
              <h4 className="mb-2 font-semibold">Search Syntax</h4>
              <p className="text-sm text-muted-foreground">
                Use GitHub-like syntax to filter results
              </p>
            </div>

            <div className="space-y-2">
              <div>
                <p className="text-sm font-medium">Operators</p>
                <ul className="mt-1 space-y-1 text-sm text-muted-foreground">
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      field:value
                    </code>{' '}
                    - Contains match
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      field:&gt;100
                    </code>{' '}
                    - Greater than
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      field:&lt;100
                    </code>{' '}
                    - Less than
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      field:&gt;=100
                    </code>{' '}
                    - Greater than or equal
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      field:&lt;=100
                    </code>{' '}
                    - Less than or equal
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      field:!=value
                    </code>{' '}
                    - Not equal
                  </li>
                </ul>
              </div>

              <div>
                <p className="text-sm font-medium">Logical Operators</p>
                <ul className="mt-1 space-y-1 text-sm text-muted-foreground">
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">AND</code> -
                    Match both conditions
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">OR</code> -
                    Match either condition
                  </li>
                </ul>
              </div>

              <div>
                <p className="text-sm font-medium">Examples</p>
                <ul className="mt-1 space-y-1 text-sm text-muted-foreground">
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      owner:john AND lines:&gt;1000
                    </code>
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      todos:&gt;5
                    </code>
                  </li>
                  <li>
                    <code className="rounded bg-muted px-1 py-0.5">
                      "react component"
                    </code>
                  </li>
                </ul>
              </div>

              {fields.length > 0 && (
                <div>
                  <p className="text-sm font-medium">Available Fields</p>
                  <div className="mt-1 flex flex-wrap gap-1">
                    {fields.map((field) => (
                      <code
                        key={field}
                        className="rounded bg-muted px-1 py-0.5 text-xs"
                      >
                        {field}
                      </code>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  )
}
