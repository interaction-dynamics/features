import {
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Pie,
  PieChart,
  ResponsiveContainer,
  Tooltip,
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
import type { Stats } from '@/models/feature'

interface FeatureInsightsProps {
  stats: Stats
}

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

// Format date string to a more readable format
const formatDate = (dateString?: string) => {
  if (!dateString) return 'N/A'
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

// Get a consistent color for a commit type
const getCommitTypeColor = (type: string) => {
  return COMMIT_TYPE_COLORS[type] || COMMIT_TYPE_COLORS.other
}

export function FeatureInsights({ stats }: FeatureInsightsProps) {
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
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Total Commits</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalCommits}</div>
            <p className="text-xs text-muted-foreground">
              From {formatDate(commits.first_commit_date)} to{' '}
              {formatDate(commits.last_commit_date)}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Contributors</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalAuthors}</div>
            <p className="text-xs text-muted-foreground">
              Avg {avgCommitsPerAuthor} commits per author
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">
              Most Active Type
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {typesData[0]?.name || 'N/A'}
            </div>
            <p className="text-xs text-muted-foreground">
              {typesData[0]?.value || 0} commits (
              {typesData[0]
                ? Math.round((typesData[0].value / totalCommits) * 100)
                : 0}
              %)
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">
              Top Contributor
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div
              className="text-2xl font-bold truncate"
              title={authorsData[0]?.fullName}
            >
              {authorsData[0]?.name || 'N/A'}
            </div>
            <p className="text-xs text-muted-foreground">
              {authorsData[0]?.commits || 0} commits (
              {authorsData[0]
                ? Math.round((authorsData[0].commits / totalCommits) * 100)
                : 0}
              %)
            </p>
          </CardContent>
        </Card>
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
              <ResponsiveContainer width="100%" height={300}>
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
                    dataKey="value"
                  >
                    {typesData.map((entry) => (
                      <Cell key={`cell-${entry.name}`} fill={entry.fill} />
                    ))}
                  </Pie>
                  <Tooltip />
                </PieChart>
              </ResponsiveContainer>
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
              <ResponsiveContainer width="100%" height={300}>
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
                  <Tooltip
                    content={({ active, payload }) => {
                      if (active && payload && payload[0]) {
                        const data = payload[0].payload
                        return (
                          <div className="bg-background border rounded p-2 shadow-lg">
                            <p className="font-semibold text-sm">
                              {data.fullName}
                            </p>
                            <p className="text-sm">Commits: {data.commits}</p>
                          </div>
                        )
                      }
                      return null
                    }}
                  />
                  <Bar dataKey="commits" fill="#3b82f6" radius={[4, 4, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
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
