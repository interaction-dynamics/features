import { Badge } from '@/components/ui/badge'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

interface MetadataItem {
  [key: string]: string
}

interface FeatureMetadataTableProps {
  data: MetadataItem[]
  metadataKey: string
}

export function FeatureMetadataTable({
  data,
  metadataKey,
}: FeatureMetadataTableProps) {
  if (!data || data.length === 0) {
    return (
      <div className="rounded-lg border border-dashed py-12 text-center">
        <p className="text-sm text-muted-foreground">
          No {metadataKey.replace(/-/g, ' ')} metadata found
        </p>
      </div>
    )
  }

  // Extract all unique keys from all items
  const allKeys = Array.from(
    new Set(data.flatMap((item) => Object.keys(item))),
  ).filter((key) => key !== 'feature') // Exclude 'feature' key as it's redundant

  return (
    <div className="space-y-2">
      <div className="rounded-lg border">
        <Table>
          <TableHeader>
            <TableRow className="hover:bg-transparent">
              {allKeys.map((key) => (
                <TableHead
                  key={key}
                  className="h-10 font-semibold text-foreground"
                >
                  {key
                    .split('_')
                    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
                    .join(' ')}
                </TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {data.map((item, index) => (
              // biome-ignore lint/suspicious/noArrayIndexKey: we have no other option
              <TableRow key={index}>
                {allKeys.map((key) => (
                  <TableCell key={key} className="py-3">
                    {item[key] ? (
                      key === 'type' ||
                      key === 'status' ||
                      key === 'environment' ? (
                        <Badge
                          variant="secondary"
                          className="font-mono text-xs font-normal"
                        >
                          {item[key]}
                        </Badge>
                      ) : key === 'enabled' ? (
                        <Badge
                          variant={
                            item[key] === 'true' ? 'default' : 'secondary'
                          }
                          className="font-mono text-xs font-normal"
                        >
                          {item[key]}
                        </Badge>
                      ) : (
                        <span className="text-sm text-muted-foreground">
                          {item[key]}
                        </span>
                      )
                    ) : (
                      <span className="text-sm text-muted-foreground/40">
                        —
                      </span>
                    )}
                  </TableCell>
                ))}
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
      <div className="text-xs text-muted-foreground">
        {data.length} {data.length === 1 ? 'entry' : 'entries'}
      </div>
    </div>
  )
}
