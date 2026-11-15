import { ChevronDown } from 'lucide-react'
import { useMemo, useState } from 'react'
import { useNavigate } from 'react-router'
import {
  CartesianGrid,
  ReferenceLine,
  Scatter,
  ScatterChart,
  Tooltip,
  XAxis,
  YAxis,
  ZAxis,
} from 'recharts'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { type ChartConfig, ChartContainer } from '@/components/ui/chart'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { useMetadata } from '@/hooks/use-metadata'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import {
  availableMetrics,
  buildRiskPoints,
  defaultAxes,
  formatMetricTick,
  formatMetricValue,
  median,
  metricsForAxis,
  type Quadrant,
  quadrantLabels,
  quadrantOf,
  type RiskMetric,
  type RiskMetricId,
  type RiskPoint,
} from './risk-metrics'

const chartConfig = {
  features: { label: 'Features', color: 'var(--foreground)' },
} satisfies ChartConfig

type RiskMatrixProps = {
  features: Feature[]
}

type ChartPoint = RiskPoint & { quadrant: Quadrant }

export function RiskMatrix({ features }: RiskMatrixProps) {
  const { metadata } = useMetadata()
  const skipChanges = metadata?.skipChanges ?? false
  const navigate = useNavigate()

  const metrics = useMemo(
    () => availableMetrics(features, { skipChanges }),
    [features, skipChanges],
  )
  const defaults = defaultAxes(metrics)

  const [xId, setXId] = useState<RiskMetricId | undefined>()
  const [yId, setYId] = useState<RiskMetricId | undefined>()

  // Fall back on the defaults when nothing is picked or the pick disappeared
  const x = metrics.find((metric) => metric.id === xId) ?? defaults.x
  const y = metrics.find((metric) => metric.id === yId) ?? defaults.y

  const chart = useMemo(() => {
    if (!x || !y) return undefined
    const { points, hidden } = buildRiskPoints(features, x, y)
    const medianX = median(points.map((point) => point.x)) ?? 0
    const medianY = median(points.map((point) => point.y)) ?? 0
    const data: ChartPoint[] = points.map((point) => ({
      ...point,
      quadrant: quadrantOf(point, medianX, medianY),
    }))
    return { data, hidden, medianX, medianY }
  }, [features, x, y])

  if (!x || !y || !chart) {
    return <RiskMatrixEmptyState hasRisk={!!x} hasActivity={!!y} />
  }

  const labels = quadrantLabels(x)
  const focus = chart.data.filter((point) => point.quadrant === 'top-right')
  const others = chart.data.filter((point) => point.quadrant !== 'top-right')

  const openFeature = (point: unknown) => {
    const path = (point as { payload?: ChartPoint })?.payload?.feature?.path
    if (path) navigate(`/?feature=${path}`)
  }

  return (
    <Card className="gap-4" data-testid="risk-matrix">
      <CardHeader className="flex flex-wrap items-start justify-between gap-4">
        <div className="space-y-1.5">
          <CardTitle>Risk matrix</CardTitle>
          <CardDescription>
            One dot per feature, sized by lines of code. Dashed lines are the
            medians. Click a dot to open the feature.
          </CardDescription>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <AxisPicker
            label="Risk"
            value={x}
            options={metricsForAxis(metrics, 'risk')}
            onChange={setXId}
          />
          <span className="text-xs text-muted-foreground">vs</span>
          <AxisPicker
            label="Activity"
            value={y}
            options={metricsForAxis(metrics, 'activity')}
            onChange={setYId}
          />
        </div>
      </CardHeader>
      <CardContent className="space-y-2">
        {chart.data.length === 0 ? (
          <p className="py-16 text-center text-sm text-muted-foreground">
            No feature has a value for both {x.label.toLowerCase()} and{' '}
            {y.label.toLowerCase()}.
          </p>
        ) : (
          <div className="relative">
            <ChartContainer
              config={chartConfig}
              className="aspect-auto h-[420px] w-full"
            >
              <ScatterChart
                margin={{ top: 28, right: 16, bottom: 24, left: 8 }}
              >
                <CartesianGrid className="stroke-muted" strokeDasharray="0" />
                <XAxis
                  type="number"
                  dataKey="x"
                  name={x.label}
                  tickLine={false}
                  axisLine={false}
                  tickFormatter={(value: number) => formatMetricTick(x, value)}
                  label={{
                    value: `${x.label} →`,
                    position: 'insideBottom',
                    offset: -16,
                    className: 'fill-muted-foreground text-xs',
                  }}
                />
                <YAxis
                  type="number"
                  dataKey="y"
                  name={y.label}
                  tickLine={false}
                  axisLine={false}
                  width={56}
                  tickFormatter={(value: number) => formatMetricTick(y, value)}
                  label={{
                    value: `${y.label} →`,
                    angle: -90,
                    position: 'insideLeft',
                    offset: 4,
                    style: { textAnchor: 'middle' },
                    className: 'fill-muted-foreground text-xs',
                  }}
                />
                <ZAxis type="number" dataKey="size" range={[40, 400]} />
                <ReferenceLine
                  x={chart.medianX}
                  stroke="var(--muted-foreground)"
                  strokeOpacity={0.5}
                  strokeDasharray="4 4"
                />
                <ReferenceLine
                  y={chart.medianY}
                  stroke="var(--muted-foreground)"
                  strokeOpacity={0.5}
                  strokeDasharray="4 4"
                />
                <Tooltip
                  cursor={false}
                  isAnimationActive={false}
                  content={<RiskTooltip x={x} y={y} labels={labels} />}
                />
                <Scatter
                  name="Features"
                  data={others}
                  fill="var(--foreground)"
                  fillOpacity={0.25}
                  stroke="var(--foreground)"
                  strokeOpacity={0.4}
                  className="cursor-pointer"
                  onClick={openFeature}
                  isAnimationActive={false}
                />
                <Scatter
                  name={labels['top-right']}
                  data={focus}
                  fill="var(--foreground)"
                  fillOpacity={0.85}
                  stroke="var(--foreground)"
                  className="cursor-pointer"
                  onClick={openFeature}
                  isAnimationActive={false}
                />
              </ScatterChart>
            </ChartContainer>
            <QuadrantLabel className="top-0 right-4 text-foreground">
              {labels['top-right']}
            </QuadrantLabel>
            <QuadrantLabel className="top-0 left-16">
              {labels['top-left']}
            </QuadrantLabel>
            <QuadrantLabel className="right-4 bottom-14">
              {labels['bottom-right']}
            </QuadrantLabel>
            <QuadrantLabel className="bottom-14 left-16">
              {labels['bottom-left']}
            </QuadrantLabel>
          </div>
        )}
        {chart.hidden > 0 && (
          <p className="text-xs text-muted-foreground">
            {chart.hidden} feature{chart.hidden === 1 ? '' : 's'} without{' '}
            {chart.hidden === 1 ? 'a value' : 'values'} for these axes{' '}
            {chart.hidden === 1 ? 'is' : 'are'} hidden.
          </p>
        )}
      </CardContent>
    </Card>
  )
}

function QuadrantLabel({
  className,
  children,
}: {
  className?: string
  children: string
}) {
  return (
    <span
      className={cn(
        'pointer-events-none absolute text-xs font-medium text-muted-foreground',
        className,
      )}
    >
      {children}
    </span>
  )
}

function AxisPicker({
  label,
  value,
  options,
  onChange,
}: {
  label: string
  value: RiskMetric
  options: RiskMetric[]
  onChange: (id: RiskMetricId) => void
}) {
  const builtIn = options.filter((option) => option.axis !== 'any')
  const imported = options.filter((option) => option.axis === 'any')

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          size="sm"
          className="h-8 gap-1.5 font-normal"
          aria-label={`${label} axis`}
        >
          <span className="text-muted-foreground">{label}:</span>
          {value.label}
          <ChevronDown className="h-3.5 w-3.5 opacity-60" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="min-w-52">
        <DropdownMenuRadioGroup
          value={value.id}
          onValueChange={(id) => onChange(id as RiskMetricId)}
        >
          {builtIn.map((option) => (
            <DropdownMenuRadioItem key={option.id} value={option.id}>
              {option.label}
            </DropdownMenuRadioItem>
          ))}
          {imported.length > 0 && (
            <>
              {builtIn.length > 0 && <DropdownMenuSeparator />}
              <DropdownMenuLabel className="text-xs font-normal text-muted-foreground">
                Imported with --data
              </DropdownMenuLabel>
              {imported.map((option) => (
                <DropdownMenuRadioItem key={option.id} value={option.id}>
                  {option.label}
                </DropdownMenuRadioItem>
              ))}
            </>
          )}
        </DropdownMenuRadioGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

function RiskTooltip({
  active,
  payload,
  x,
  y,
  labels,
}: {
  active?: boolean
  payload?: { payload?: ChartPoint }[]
  x: RiskMetric
  y: RiskMetric
  labels: Record<Quadrant, string>
}) {
  const point = payload?.[0]?.payload
  if (!active || !point) return null

  return (
    <div className="grid min-w-44 gap-1.5 rounded-lg border border-border/50 bg-background px-2.5 py-1.5 text-xs shadow-xl">
      <div>
        <p className="font-medium">{formatFeatureName(point.feature.name)}</p>
        {point.feature.owner && (
          <p className="text-muted-foreground">{point.feature.owner}</p>
        )}
      </div>
      <div className="grid gap-0.5">
        <TooltipRow label={x.label} value={formatMetricValue(x, point.x)} />
        <TooltipRow label={y.label} value={formatMetricValue(y, point.y)} />
      </div>
      <p className="text-muted-foreground">{labels[point.quadrant]}</p>
    </div>
  )
}

function TooltipRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between gap-4">
      <span className="text-muted-foreground">{label}</span>
      <span className="font-mono font-medium tabular-nums">{value}</span>
    </div>
  )
}

function RiskMatrixEmptyState({
  hasRisk,
  hasActivity,
}: {
  hasRisk: boolean
  hasActivity: boolean
}) {
  return (
    <Card data-testid="risk-matrix-empty">
      <CardHeader>
        <CardTitle>Risk matrix</CardTitle>
        <CardDescription>
          The risk matrix needs a risk signal and an activity signal for each
          feature.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <ul className="list-disc space-y-1 pl-5 text-sm text-muted-foreground">
          {!hasRisk && (
            <li>
              Risk: add a coverage report (lcov or Cobertura) to the project, or
              import a metric like <code>open_errors</code> with{' '}
              <code>--data</code>.
            </li>
          )}
          {!hasActivity && (
            <li>
              Activity: scan without <code>--skip-changes</code> to read the git
              history, or import a metric like <code>weekly_users</code> with{' '}
              <code>--data</code>.
            </li>
          )}
        </ul>
      </CardContent>
    </Card>
  )
}
