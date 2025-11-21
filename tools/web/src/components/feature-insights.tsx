import {
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Pie,
  PieChart,
  XAxis,
  YAxis,
} from 'recharts'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  type ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from '@/components/ui/chart'
import { formatDate } from '@/lib/format-date'
import type { Stats } from '@/models/feature'
import { StatsCard } from './stats-card'

interface FeatureInsightsProps {
  stats: Stats
}

const chartConfig = {
  visitors: {
    label: 'Visitors',
  },
  desktop: {
    label: 'Desktop',
    color: 'var(--primary)',
  },
  mobile: {
    label: 'Mobile',
    color: 'var(--primary)',
  },
} satisfies ChartConfig

// Color palette for commit types
const COMMIT_TYPE_COLORS: Record<string, string> = {
  feat: '#10b981',
  fix: '#ef4444',
  docs: '#3b82f6',
  style: '#8b5cf6',
  refactor: '#f59e0b',
  perf: '#ec4899',
  test: '#14b8a6',
  build: '#6366f1',
  ci: '#84cc16',
  chore: '#64748b',
  revert: '#f97316',
  other: '#94a3b8',
}

// Get a consistent color for a commit type
const getCommitTypeColor = (type: string) => {
  return COMMIT_TYPE_COLORS[type] || COMMIT_TYPE_COLORS.other
}

export default function FeatureInsights({ stats }: FeatureInsightsProps) {
  const { commits } = stats

  // Prepare data for authors chart
  const authorsData = Object.entries(commits.authors_count || {})
    .map(([name, count]) => ({
      name: name.length > 20 ? `${name.substring(0, 20)}...` : name,
      fullName: name,
      commits: count,
    }))
    .sort((a, b) => b.commits - a.commits)
    .slice(0, 10) // Show top 10 contributors

  // Prepare data for commit types chart
  const typesData = Object.entries(commits.count_by_type || {})
    .map(([type, count]) => ({
      name: type,
      value: count,
      fill: getCommitTypeColor(type),
    }))
    .sort((a, b) => b.value - a.value)

  // Calculate some statistics
  const totalAuthors = Object.keys(commits.authors_count || {}).length
  const totalCommits = commits.total_commits || 0
  const avgCommitsPerAuthor =
    totalAuthors > 0 ? Math.round(totalCommits / totalAuthors) : 0

  return (
    <div className="space-y-4">
      {/* Overview Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4 *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card dark:*:data-[slot=card]:bg-card *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:shadow-xs">
        <StatsCard
          title="Total Commits"
          value={totalCommits}
          subtitle={`From ${formatDate(commits.first_commit_date)} to ${formatDate(commits.last_commit_date)}`}
        />

        <StatsCard
          title="Contributors"
          value={totalAuthors}
          subtitle={`Avg ${avgCommitsPerAuthor} commits per author`}
        />

        <StatsCard
          title="Number of TODOs"
          value={stats.todos_count ?? 'N/A'}
          subtitle={`Out of ${stats.files_count} files and ${stats.lines_count} lines`}
        />

        {stats.coverage ? (
          <StatsCard
            title="Test Coverage"
            value={
              `${Intl.NumberFormat('en', { maximumFractionDigits: 1 }).format(stats.coverage.line_coverage_percent)}%` ||
              'N/A'
            }
            subtitle={`${stats.coverage?.lines_covered} lines out of ${stats.coverage?.lines_total}`}
          />
        ) : (
          <StatsCard
            title="Top Contributor"
            value={authorsData[0]?.name || 'N/A'}
            subtitle={`${authorsData[0]?.commits || 0} commits (${
              authorsData[0]
                ? Math.round((authorsData[0].commits / totalCommits) * 100)
                : 0
            }%)`}
          />
        )}
      </div>

      {/* Charts */}
      <div className="grid gap-4 md:grid-cols-2">
        {/* Commit Types Chart */}
        {typesData.length > 0 && (
          <Card>
            <CardHeader>
              <CardTitle>Commits by Type</CardTitle>
              <CardDescription>
                Distribution of commits across different types
              </CardDescription>
            </CardHeader>
            <CardContent>
              <ChartContainer className="w-full h-[300px]" config={chartConfig}>
                <PieChart>
                  <Pie
                    data={typesData}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={({ name, percent }) => {
                      if (!name || typeof percent !== 'number') return ''
                      return `${name} (${(percent * 100).toFixed(0)}%)`
                    }}
                    outerRadius={80}
                    fill="#8884d8"
                    strokeWidth={0}
                    dataKey="value"
                  >
                    {typesData.map((entry) => (
                      <Cell key={`cell-${entry.name}`} fill={entry.fill} />
                    ))}
                  </Pie>
                  <ChartTooltip
                    cursor={false}
                    content={<ChartTooltipContent indicator="dot" />}
                  />
                </PieChart>
              </ChartContainer>
            </CardContent>
          </Card>
        )}

        {/* Top Contributors Chart */}
        {authorsData.length > 0 && (
          <Card>
            <CardHeader>
              <CardTitle>Top Contributors</CardTitle>
              <CardDescription>
                Most active contributors to this feature
              </CardDescription>
            </CardHeader>
            <CardContent>
              <ChartContainer className="w-full h-[300px]" config={chartConfig}>
                <BarChart
                  data={authorsData}
                  margin={{ top: 20, right: 30, left: 20, bottom: 60 }}
                >
                  <CartesianGrid
                    strokeDasharray="3 3"
                    className="stroke-muted"
                  />
                  <XAxis
                    dataKey="name"
                    angle={-45}
                    textAnchor="end"
                    height={100}
                    interval={0}
                    tick={{ fontSize: 12 }}
                  />
                  <YAxis tick={{ fontSize: 12 }} />
                  <ChartTooltip
                    cursor={false}
                    content={
                      <ChartTooltipContent
                        labelFormatter={(value) => {
                          return value.fullName
                        }}
                        indicator="dot"
                      />
                    }
                  />
                  <Bar
                    dataKey="commits"
                    fill="var(--color-desktop)"
                    stroke="var(--color-mobile)"
                    radius={[4, 4, 0, 0]}
                  />
                </BarChart>
              </ChartContainer>
            </CardContent>
          </Card>
        )}
      </div>

      {/* Commit Type Legend */}
      {typesData.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Commit Type Legend</CardTitle>
            <CardDescription>
              Conventional commit types used in this feature
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
              {Object.entries(COMMIT_TYPE_COLORS).map(([type, color]) => {
                const typeCount = commits.count_by_type?.[type] || 0
                if (typeCount === 0 && type !== 'other') return null

                return (
                  <div key={type} className="flex items-center gap-2">
                    <div
                      className="w-3 h-3 rounded-full"
                      style={{ backgroundColor: color }}
                    />
                    <span className="text-sm">
                      <span className="font-medium">{type}</span>
                      {typeCount > 0 && (
                        <span className="text-muted-foreground ml-1">
                          ({typeCount})
                        </span>
                      )}
                    </span>
                  </div>
                )
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
