import { ArrowDown, ArrowUp, ArrowUpDown } from 'lucide-react'
import { TableHead } from '@/components/ui/table'
import type { SortConfig, SortDirection } from '@/hooks/use-table-sort'
import { cn } from '@/lib/utils'

type SortableTableHeaderProps<T> = {
  field: keyof T | string
  label: string
  sortConfig: SortConfig<T> | null
  onSort: (field: keyof T | string) => void
  className?: string
  align?: 'left' | 'right' | 'center'
}

export function SortableTableHeader<T>({
  field,
  label,
  sortConfig,
  onSort,
  className,
  align = 'left',
}: SortableTableHeaderProps<T>) {
  const isSorted = sortConfig?.field === field
  const direction: SortDirection = isSorted ? sortConfig.direction : null

  const getSortIcon = () => {
    if (!isSorted || direction === null) {
      return <ArrowUpDown className="h-4 w-4" />
    }
    if (direction === 'asc') {
      return <ArrowUp className="h-4 w-4" />
    }
    return <ArrowDown className="h-4 w-4" />
  }

  return (
    <TableHead
      className={cn(
        'cursor-pointer select-none transition-colors hover:bg-muted/50 group',
        align === 'right' && 'text-right',
        align === 'center' && 'text-center',
        className,
      )}
      onClick={() => onSort(field)}
    >
      <div
        className={cn(
          'flex items-center gap-2',
          align === 'right' && 'justify-end',
          align === 'center' && 'justify-center',
        )}
      >
        <span>{label}</span>
        <span
          className={cn(
            'transition-opacity',
            isSorted ? 'opacity-100' : 'opacity-0 group-hover:opacity-50',
          )}
        >
          {getSortIcon()}
        </span>
      </div>
    </TableHead>
  )
}
