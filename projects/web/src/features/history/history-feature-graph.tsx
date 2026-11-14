import { GitBranch, Sparkles } from 'lucide-react'
import { useLayoutEffect, useMemo, useRef, useState } from 'react'
import { formatDate } from '@/lib/format-date'
import { formatFeatureName } from '@/lib/format-feature-name'
import { cn } from '@/lib/utils'
import type { Feature } from '@/models/feature'
import {
  buildFeatureGraph,
  dateTime,
  type FeatureLane,
  lineage,
  monthTicks,
} from './graph'
import { formatMonth } from './utils'

const ROW = 32
const AXIS = 28
const PAD_X = 24
const RADIUS = 10
const MIN_WIDTH = 480
const TICK_SPACING = 64
const COLORS = [
  'var(--chart-1)',
  'var(--chart-2)',
  'var(--chart-3)',
  'var(--chart-4)',
  'var(--chart-5)',
]

interface HistoryFeatureGraphProps {
  features: Feature[]
  onOpenFeature: (feature: Feature) => void
}

/** Width of an element, kept up to date when it is resized */
function useWidth<T extends HTMLElement>() {
  const ref = useRef<T>(null)
  const [width, setWidth] = useState(0)
  useLayoutEffect(() => {
    const element = ref.current
    if (!element) return
    setWidth(element.clientWidth)
    const observer = new ResizeObserver(() => setWidth(element.clientWidth))
    observer.observe(element)
    return () => observer.disconnect()
  }, [])
  return [ref, width] as const
}

const day = (date: string) => formatDate(date.replace(' ', 'T'))

/**
 * Features drawn like a git graph: each feature is a lane starting at the
 * commit that created it and forking from its parent, up to the last commit
 * of the report. The lane fades after the last commit of the feature.
 */
export function HistoryFeatureGraph({
  features,
  onOpenFeature,
}: HistoryFeatureGraphProps) {
  const graph = useMemo(() => buildFeatureGraph(features), [features])
  const [containerRef, containerWidth] = useWidth<HTMLDivElement>()
  const [hovered, setHovered] = useState<number | null>(null)

  const { lanes } = graph
  const width = Math.max(containerWidth, MIN_WIDTH)
  const height = AXIS + (lanes.length + 1) * ROW

  const t0 = dateTime(graph.start)
  const t1 = dateTime(graph.end)
  const x = (time: number) =>
    t1 === t0
      ? width / 2
      : PAD_X + ((time - t0) / (t1 - t0)) * (width - 2 * PAD_X)
  const y = (row: number) => AXIS + row * ROW + ROW / 2

  const ticks = useMemo(() => {
    const months = monthTicks(graph.start, graph.end).length
    const step = Math.max(1, Math.ceil((months * TICK_SPACING) / width))
    return monthTicks(graph.start, graph.end, step)
  }, [graph.start, graph.end, width])

  // Each top-level feature and its sub-features share a color
  const colorOf = useMemo(() => {
    const roots = new Map<number, string>()
    for (const [index, lane] of lanes.entries()) {
      if (lane.parent === -1) {
        roots.set(index, COLORS[roots.size % COLORS.length])
      }
    }
    return (lane: FeatureLane) => roots.get(lane.root) ?? COLORS[0]
  }, [lanes])

  // Hovering a lane highlights its ancestors and sub-features
  const related = useMemo(
    () => (hovered === null ? null : lineage(lanes, hovered)),
    [hovered, lanes],
  )

  if (lanes.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center gap-2 py-16 text-center">
        <GitBranch className="h-12 w-12 text-muted-foreground" />
        <p className="text-sm font-medium">No feature to draw</p>
        <p className="max-w-sm text-sm text-muted-foreground">
          None of the features has commits yet.
        </p>
      </div>
    )
  }

  const xEnd = x(t1)
  const active = hovered === null ? null : lanes[hovered]
  const dim = (index: number) => related !== null && !related.has(index)

  return (
    <div className="flex flex-col gap-2" data-testid="history-feature-graph">
      <div className="flex overflow-hidden rounded-lg border">
        <div className="w-60 shrink-0 border-r">
          <div
            className="flex items-center border-b px-3 text-xs font-medium text-muted-foreground"
            style={{ height: AXIS }}
          >
            Feature
          </div>
          <div
            className="flex items-center gap-2 px-3 text-sm text-muted-foreground"
            style={{ height: ROW }}
          >
            <span className="size-2.5 shrink-0 rounded-full bg-muted-foreground/60" />
            Project
          </div>
          {lanes.map((lane, index) => (
            <button
              key={lane.feature.path}
              type="button"
              className={cn(
                'flex w-full items-center gap-2 pr-3 text-left text-sm transition-opacity hover:underline',
                hovered === index && 'bg-muted/60',
                dim(index) && 'opacity-40',
              )}
              style={{ height: ROW, paddingLeft: 12 + lane.depth * 14 }}
              onMouseEnter={() => setHovered(index)}
              onMouseLeave={() => setHovered(null)}
              onFocus={() => setHovered(index)}
              onBlur={() => setHovered(null)}
              onClick={() => onOpenFeature(lane.feature)}
              title={`Open ${formatFeatureName(lane.feature.name)} (${lane.feature.path})`}
              data-testid="history-graph-label"
            >
              <span
                className="size-2.5 shrink-0 rounded-full"
                style={{ backgroundColor: colorOf(lane) }}
              />
              <span className="truncate">
                {formatFeatureName(lane.feature.name)}
              </span>
            </button>
          ))}
        </div>
        <div ref={containerRef} className="min-w-0 flex-1 overflow-x-auto">
          <svg
            width={width}
            height={height}
            // The labels are the accessible version of the graph
            aria-hidden
          >
            <line
              x1={0}
              x2={width}
              y1={AXIS - 0.5}
              y2={AXIS - 0.5}
              className="stroke-border"
            />
            {ticks.map((key) => {
              const [year, month] = key.split('-').map(Number)
              const tx = x(Date.UTC(year, month - 1, 1))
              // Skip labels that would be cut at the edges
              if (tx < 20 || tx > width - 20) return null
              return (
                <g key={key}>
                  <line
                    x1={tx}
                    x2={tx}
                    y1={AXIS}
                    y2={height}
                    className="stroke-border"
                    strokeDasharray="2 4"
                  />
                  <text
                    x={tx}
                    y={AXIS / 2}
                    dominantBaseline="middle"
                    textAnchor="middle"
                    className="fill-muted-foreground text-[11px]"
                  >
                    {formatMonth(key, 'short')}
                  </text>
                </g>
              )
            })}

            {hovered !== null && (
              <rect
                x={0}
                y={y(hovered + 1) - ROW / 2}
                width={width}
                height={ROW}
                className="fill-muted/60"
              />
            )}

            {/* Project trunk, where top-level features fork from */}
            <line
              x1={x(t0)}
              x2={xEnd}
              y1={y(0)}
              y2={y(0)}
              strokeWidth={2}
              strokeLinecap="round"
              className="stroke-muted-foreground/60"
            />

            {lanes.map((lane, index) => {
              const parentRow = lane.parent + 1
              const parentStart =
                lane.parent === -1
                  ? x(t0)
                  : x(dateTime(lanes[lane.parent].creation.date))
              const cx = x(dateTime(lane.creation.date))
              const cy = y(index + 1)
              const py = y(parentRow)
              const r = Math.min(RADIUS, (cx - parentStart) / 2)
              const d =
                r < 3
                  ? `M ${cx} ${py} L ${cx} ${cy}`
                  : `M ${cx - 2 * r} ${py} Q ${cx - r} ${py} ${cx - r} ${py + r} L ${cx - r} ${cy - r} Q ${cx - r} ${cy} ${cx} ${cy}`
              return (
                <path
                  key={lane.feature.path}
                  d={d}
                  fill="none"
                  stroke={colorOf(lane)}
                  strokeWidth={2}
                  opacity={dim(index) ? 0.2 : 1}
                />
              )
            })}

            {lanes.map((lane, index) => {
              const color = colorOf(lane)
              const cx = x(dateTime(lane.creation.date))
              const lx = x(dateTime(lane.lastChange))
              const cy = y(index + 1)
              const faded = dim(index)
              const name = formatFeatureName(lane.feature.name)
              return (
                <g
                  key={lane.feature.path}
                  opacity={faded ? 0.2 : 1}
                  onMouseEnter={() => setHovered(index)}
                  onMouseLeave={() => setHovered(null)}
                  className="cursor-pointer"
                  onClick={() => onOpenFeature(lane.feature)}
                  data-testid="history-graph-lane"
                >
                  <title>
                    {`${name}: created ${day(lane.creation.date)}, last change ${day(lane.lastChange)}`}
                  </title>
                  <rect
                    x={0}
                    y={cy - ROW / 2}
                    width={width}
                    height={ROW}
                    fill="transparent"
                  />
                  {/* After its last commit, the feature still exists */}
                  <line
                    x1={lx}
                    x2={xEnd}
                    y1={cy}
                    y2={cy}
                    stroke={color}
                    strokeWidth={2}
                    strokeOpacity={0.3}
                  />
                  <line
                    x1={cx}
                    x2={lx}
                    y1={cy}
                    y2={cy}
                    stroke={color}
                    strokeWidth={hovered === index ? 4 : 3}
                    strokeLinecap="round"
                  />
                  <circle
                    cx={cx}
                    cy={cy}
                    r={hovered === index ? 6 : 5}
                    fill={color}
                    className="stroke-background"
                    strokeWidth={2}
                  />
                </g>
              )
            })}
          </svg>
        </div>
      </div>
      <div
        className="flex min-h-5 flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground"
        data-testid="history-graph-details"
      >
        {active ? (
          <>
            <span className="inline-flex items-center gap-1 font-medium text-foreground">
              <Sparkles className="h-3 w-3 text-primary" />
              {formatFeatureName(active.feature.name)}
            </span>
            <span>
              Created {day(active.creation.date)}
              {active.creation.author_name &&
                ` by ${active.creation.author_name}`}
            </span>
            <span className="max-w-md truncate" title={active.creation.title}>
              in “{active.creation.title}”
            </span>
            <span>
              {active.commits} {active.commits === 1 ? 'commit' : 'commits'}
            </span>
            <span>Last change {day(active.lastChange)}</span>
            {active.children > 0 && (
              <span>
                {active.children}{' '}
                {active.children === 1 ? 'sub-feature' : 'sub-features'}
              </span>
            )}
          </>
        ) : (
          <span>
            Each lane starts at the commit that created the feature and forks
            from its parent. Hover a lane for details, click to open it.
            {graph.withoutCommits > 0 &&
              ` ${graph.withoutCommits} ${graph.withoutCommits === 1 ? 'feature has' : 'features have'} no commit and ${graph.withoutCommits === 1 ? 'is' : 'are'} not shown.`}
          </span>
        )}
      </div>
    </div>
  )
}
