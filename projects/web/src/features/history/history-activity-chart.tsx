import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from 'recharts'
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
import { formatMonth, type HistoryCommit, monthlyActivity } from './utils'

const chartConfig = {
  commits: { label: 'Commits', color: 'var(--primary)' },
} satisfies ChartConfig

export function HistoryActivityChart({
  commits,
}: {
  commits: HistoryCommit[]
}) {
  const data = monthlyActivity(commits)

  if (data.length < 2) return null

  return (
    <Card data-testid="history-activity">
      <CardHeader>
        <CardTitle>Activity</CardTitle>
        <CardDescription>Commits per month</CardDescription>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig} className="h-40 w-full">
          <BarChart data={data} margin={{ top: 4, right: 4, left: -24 }}>
            <CartesianGrid vertical={false} className="stroke-muted" />
            <XAxis
              dataKey="month"
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11 }}
              tickFormatter={(month: string) => formatMonth(month, 'short')}
              minTickGap={16}
            />
            <YAxis
              allowDecimals={false}
              tickLine={false}
              axisLine={false}
              tick={{ fontSize: 11 }}
            />
            <ChartTooltip
              cursor={false}
              content={
                <ChartTooltipContent
                  labelFormatter={(month) => formatMonth(String(month))}
                  indicator="dot"
                />
              }
            />
            <Bar
              dataKey="commits"
              fill="var(--color-commits)"
              radius={[3, 3, 0, 0]}
            />
          </BarChart>
        </ChartContainer>
      </CardContent>
    </Card>
  )
}
