import { useMemo, useState } from 'react'
import { Card, CardContent } from '@/components/ui/card'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type { Stats } from '@/models/feature'
import { HelpButton } from './help-button'

interface FeatureTestsProps {
  stats: Stats
}

type SortKey =
  | 'file'
  | 'linePercent'
  | 'branchPercent'
  | 'lineCovered'
  | 'lineTotal'
  | 'branchCovered'
  | 'branchTotal'
type SortOrder = 'asc' | 'desc'

function getCoverageColor(percent: number): string {
  if (percent >= 80) return 'text-green-600 dark:text-green-400'
  if (percent >= 60) return 'text-yellow-600 dark:text-yellow-400'
  return 'text-red-600 dark:text-red-400'
}

function getFileName(filePath: string): string {
  const parts = filePath.split('/')
  return parts[parts.length - 1]
}

function getFileDirectory(filePath: string): string {
  const parts = filePath.split('/')
  parts.pop()
  return parts.join('/')
}

export function FeatureTests({ stats }: FeatureTestsProps) {
  const [sortKey, setSortKey] = useState<SortKey>('linePercent')
  const [sortOrder, setSortOrder] = useState<SortOrder>('asc')

  const coverage = stats.coverage

  const fileData = useMemo(() => {
    if (!coverage?.files) return []

    return Object.entries(coverage.files)
      .map(([path, fileCoverage]) => ({
        path,
        fileName: getFileName(path),
        directory: getFileDirectory(path),
        linePercent: fileCoverage.line_coverage_percent,
        linesCovered: fileCoverage.lines_covered,
        linesTotal: fileCoverage.lines_total,
        linesMissed: fileCoverage.lines_missed,
        branchPercent: fileCoverage.branch_coverage_percent,
        branchesCovered: fileCoverage.branches_covered,
        branchesTotal: fileCoverage.branches_total,
      }))
      .sort((a, b) => {
        let comparison = 0

        switch (sortKey) {
          case 'file':
            comparison = a.path.localeCompare(b.path)
            break
          case 'linePercent':
            comparison = a.linePercent - b.linePercent
            break
          case 'lineCovered':
            comparison = (a.linesCovered ?? -1) - (b.linesCovered ?? -1)
            break
          case 'lineTotal':
            comparison = (a.linesTotal ?? -1) - (b.linesTotal ?? -1)
            break
          case 'branchPercent':
            comparison = (a.branchPercent ?? -1) - (b.branchPercent ?? -1)
            break
          case 'branchCovered':
            comparison = (a.branchesCovered ?? -1) - (b.branchesCovered ?? -1)
            break
          case 'branchTotal':
            comparison = (a.branchesTotal ?? -1) - (b.branchesTotal ?? -1)
            break
        }

        return sortOrder === 'asc' ? comparison : -comparison
      })
  }, [coverage?.files, sortKey, sortOrder])

  const fileDataIncludeTotal = [
    {
      fileName: 'Total',
      path: '//total',
      directory: `${Object.keys(stats.coverage?.files ?? {}).length} files`,
      linePercent: stats.coverage?.line_coverage_percent ?? 0,
      linesCovered: stats.coverage?.lines_covered,
      linesTotal: stats.coverage?.lines_total,
      branchPercent: stats.coverage?.branch_coverage_percent ?? 0,
      branchesCovered: stats.coverage?.branches_covered,
      branchesTotal: stats.coverage?.branches_total,
    },
    ...fileData,
  ]

  const handleSort = (key: SortKey) => {
    if (sortKey === key) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')
    } else {
      setSortKey(key)
      setSortOrder('asc')
    }
  }

  if (!coverage) {
    return (
      <Card>
        <CardContent>
          <div className="flex flex-col items-start gap-2">
            <p className="text-sm text-muted-foreground">
              No test coverage reports available
            </p>
            <HelpButton
              title="How to add a test coverage report"
              url="https://github.com/interaction-dynamics/features/blob/master/FAQ.md#how-can-i-add-a-test-coverage-report"
            />
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className="space-y-4">
      {/* File Coverage Table */}
      {fileDataIncludeTotal.length > 0 ? (
        <div>
          <div className="bg-card rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none"
                    onClick={() => handleSort('file')}
                  >
                    <div className="flex items-center gap-1">
                      File
                      {sortKey === 'file' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none text-right"
                    onClick={() => handleSort('linePercent')}
                  >
                    <div className="flex items-center justify-end gap-1">
                      Line Coverage %
                      {sortKey === 'linePercent' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none text-right"
                    onClick={() => handleSort('lineCovered')}
                  >
                    <div className="flex items-center justify-end gap-1">
                      Line Covered
                      {sortKey === 'lineCovered' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none text-right"
                    onClick={() => handleSort('lineTotal')}
                  >
                    <div className="flex items-center justify-end gap-1">
                      Line Total
                      {sortKey === 'lineTotal' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none text-right"
                    onClick={() => handleSort('branchPercent')}
                  >
                    <div className="flex items-center justify-end gap-1">
                      Branch Coverage %
                      {sortKey === 'branchPercent' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none text-right"
                    onClick={() => handleSort('branchCovered')}
                  >
                    <div className="flex items-center justify-end gap-1">
                      Branches Covered
                      {sortKey === 'branchCovered' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                  <TableHead
                    className="cursor-pointer hover:bg-muted/50 select-none text-right"
                    onClick={() => handleSort('branchTotal')}
                  >
                    <div className="flex items-center justify-end gap-1">
                      Branch Total
                      {sortKey === 'branchTotal' && (
                        <span className="text-xs">
                          {sortOrder === 'asc' ? '↑' : '↓'}
                        </span>
                      )}
                    </div>
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {fileDataIncludeTotal.map((file) => (
                  <TableRow key={file.path}>
                    <TableCell className="font-mono text-xs">
                      <div className="flex flex-col">
                        <span className="font-medium">{file.fileName}</span>
                        {file.directory && (
                          <span className="text-muted-foreground text-[0.65rem]">
                            {file.directory}
                          </span>
                        )}
                      </div>
                    </TableCell>
                    <TableCell className="text-right">
                      <div className="flex items-center justify-end gap-2">
                        <div className="flex flex-col items-end">
                          <span
                            className={`font-semibold ${getCoverageColor(file.linePercent)}`}
                          >
                            {file.linePercent.toFixed(1)}%
                          </span>
                        </div>
                      </div>
                    </TableCell>
                    <TableCell className="text-right">
                      {file.linesCovered}
                    </TableCell>
                    <TableCell className="text-right">
                      {file.linesTotal}
                    </TableCell>
                    <TableCell className="text-right">
                      {file.branchPercent !== undefined ? (
                        <div className="flex items-center justify-end gap-2">
                          <div className="flex flex-col items-end">
                            <span
                              className={`font-semibold ${getCoverageColor(file.branchPercent)}`}
                            >
                              {file.branchPercent.toFixed(1)}
                            </span>
                          </div>
                        </div>
                      ) : (
                        <span className="text-muted-foreground text-xs">
                          N/A
                        </span>
                      )}
                    </TableCell>
                    <TableCell className="text-right">
                      {file.branchesCovered}
                    </TableCell>
                    <TableCell className="text-right">
                      {file.branchesTotal}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>

          {/* Coverage Legend */}
          <div className="mt-4 flex items-center justify-end gap-4 text-xs text-muted-foreground">
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 rounded-full bg-green-600 dark:bg-green-400" />
              <span>≥ 80%</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 rounded-full bg-yellow-600 dark:bg-yellow-400" />
              <span>60-79%</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 rounded-full bg-red-600 dark:bg-red-400" />
              <span>&lt; 60%</span>
            </div>
          </div>
        </div>
      ) : (
        <Card>
          <CardContent className="pt-6">
            <p className="text-sm text-muted-foreground text-center">
              No individual file coverage data available. Only aggregate
              statistics are shown.
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
