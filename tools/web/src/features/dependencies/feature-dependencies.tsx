import { AlertTriangle, ChevronDown, ChevronRight, Eye } from 'lucide-react'
import { useState } from 'react'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type { Dependency, Feature } from '@/models/feature'
import {
  buildDependencyMap,
  buildNameToPathMap,
  detectAlerts,
  groupDependencies,
} from './utils'

interface FeatureDependenciesProps {
  dependencies: Dependency[]
  currentFeaturePath: string
  allFeatures: Feature[]
}

export default function FeatureDependencies({
  dependencies,
  currentFeaturePath,
  allFeatures,
}: FeatureDependenciesProps) {
  const [expandedRows, setExpandedRows] = useState<Set<string>>(new Set())

  const dependencyMap = buildDependencyMap(allFeatures)
  const nameToPath = buildNameToPathMap(allFeatures)
  const groupedArray = groupDependencies(dependencies)

  const toggleRow = (key: string) => {
    const newExpanded = new Set(expandedRows)
    if (newExpanded.has(key)) {
      newExpanded.delete(key)
    } else {
      newExpanded.add(key)
    }
    setExpandedRows(newExpanded)
  }

  const getTypeBadgeVariant = (
    type: string,
  ): 'default' | 'secondary' | 'outline' => {
    switch (type) {
      case 'parent':
        return 'default'
      case 'child':
        return 'secondary'
      case 'sibling':
        return 'outline'
      default:
        return 'default'
    }
  }

  if (dependencies.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground">
        No dependencies found
      </div>
    )
  }

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Feature Dependency</TableHead>
            <TableHead>Type</TableHead>
            <TableHead className="text-right">Count</TableHead>
            <TableHead>Alerts</TableHead>
            <TableHead className="w-12"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {groupedArray.map((group) => {
            const key = `${group.feature}-${group.type}`
            const isExpanded = expandedRows.has(key)
            const alerts = detectAlerts(
              group,
              currentFeaturePath,
              dependencyMap,
              nameToPath,
            )

            return (
              <>
                <TableRow
                  key={key}
                  className="cursor-pointer hover:bg-muted/50"
                >
                  <TableCell className="font-medium">{group.feature}</TableCell>
                  <TableCell>
                    <Badge variant={getTypeBadgeVariant(group.type)}>
                      {group.type}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-right">{group.count}</TableCell>
                  <TableCell>
                    {alerts.length > 0 && (
                      <div className="flex flex-wrap gap-1">
                        {alerts.map((alert) => (
                          <Badge
                            key={alert}
                            variant="destructive"
                            className="text-xs"
                          >
                            <AlertTriangle className="h-3 w-3 mr-1" />
                            {alert}
                          </Badge>
                        ))}
                      </div>
                    )}
                  </TableCell>
                  <TableCell>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => toggleRow(key)}
                      className="h-8 w-8 p-0"
                    >
                      {isExpanded ? (
                        <ChevronDown className="h-4 w-4" />
                      ) : (
                        <ChevronRight className="h-4 w-4" />
                      )}
                    </Button>
                  </TableCell>
                </TableRow>
                {isExpanded && (
                  <TableRow>
                    <TableCell colSpan={5} className="p-0">
                      <div className="bg-muted/20 border-t">
                        <Table>
                          <TableHeader>
                            <TableRow>
                              <TableHead className="pl-14">
                                Target File
                              </TableHead>
                              <TableHead>Line</TableHead>
                              <TableHead className="w-12"></TableHead>
                            </TableRow>
                          </TableHeader>
                          <TableBody>
                            {group.items.map((item) => (
                              <TableRow
                                key={`${key}-${item.targetFilename}-${item.line}`}
                              >
                                <TableCell className="pl-14 font-mono text-xs">
                                  {item.targetFilename}
                                </TableCell>
                                <TableCell className="font-mono text-xs">
                                  {item.line}
                                </TableCell>
                                <TableCell>
                                  <Popover>
                                    <PopoverTrigger asChild>
                                      <Button
                                        variant="ghost"
                                        size="sm"
                                        className="h-8 w-8 p-0"
                                      >
                                        <Eye className="h-4 w-4" />
                                      </Button>
                                    </PopoverTrigger>
                                    <PopoverContent className="w-auto max-w-2xl">
                                      <div className="space-y-2">
                                        <p className="text-sm font-semibold">
                                          In{' '}
                                          <code className="text-xs">
                                            {item.sourceFilename}
                                          </code>
                                          :
                                        </p>
                                        <div className="rounded-md bg-muted p-3">
                                          <pre className="text-xs font-mono overflow-x-auto">
                                            <code>{item.content}</code>
                                          </pre>
                                        </div>
                                      </div>
                                    </PopoverContent>
                                  </Popover>
                                </TableCell>
                              </TableRow>
                            ))}
                          </TableBody>
                        </Table>
                      </div>
                    </TableCell>
                  </TableRow>
                )}
              </>
            )
          })}
        </TableBody>
      </Table>
    </div>
  )
}
