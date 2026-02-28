import { useCallback, useContext, useEffect, useMemo, useRef, useState } from 'react'
import { ArrowUpRight, FolderTree, User } from 'lucide-react'
import { Link } from 'react-router'
import { FeatureOwner } from '@/components/feature-owner'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { FeaturesContext } from '@/lib/features-context'
import { formatFeatureName } from '@/lib/format-feature-name'
import type { Dependency, Feature } from '@/models/feature'

const NODE_RADIUS = 22
const LAYOUT_W = 1000
const LAYOUT_H = 750

interface GraphNode {
  id: string
  label: string
  owner: string
  x: number
  y: number
  vx: number
  vy: number
}

interface GraphEdge {
  source: string
  target: string
  type: 'parent' | 'child' | 'sibling'
}

function hashStr(s: string): number {
  let h = 5381
  for (let i = 0; i < s.length; i++) h = ((h << 5) + h) ^ s.charCodeAt(i)
  return Math.abs(h)
}

function ownerColor(owner: string): string {
  if (!owner) return 'hsl(0 0% 55%)'
  return `hsl(${hashStr(owner) % 360} 55% 55%)`
}

function edgeStroke(type: GraphEdge['type']): string {
  return (
    { parent: 'hsl(25 80% 55%)', child: 'hsl(210 80% 55%)', sibling: 'hsl(280 60% 55%)' }[
      type
    ] ?? 'hsl(0 0% 50%)'
  )
}

function buildGraph(features: Feature[]): { nodes: GraphNode[]; edges: GraphEdge[] } {
  const nodeMap = new Map<string, GraphNode>()
  const edgeMap = new Map<string, GraphEdge>()

  function visit(f: Feature) {
    if (!nodeMap.has(f.path)) {
      nodeMap.set(f.path, {
        id: f.path,
        label: f.name,
        owner: f.owner,
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
      })
    }
    for (const dep of f.dependencies) {
      const key = `${f.path}=>${dep.featurePath}`
      if (f.path !== dep.featurePath && !edgeMap.has(key)) {
        edgeMap.set(key, { source: f.path, target: dep.featurePath, type: dep.type })
      }
    }
    for (const child of f.features ?? []) visit(child)
  }

  for (const f of features) visit(f)

  const nodes = Array.from(nodeMap.values())
  const count = nodes.length
  if (count > 0) {
    const r = Math.min(LAYOUT_W, LAYOUT_H) * 0.38
    nodes.forEach((n, i) => {
      const a = (2 * Math.PI * i) / count - Math.PI / 2
      n.x = LAYOUT_W / 2 + r * Math.cos(a)
      n.y = LAYOUT_H / 2 + r * Math.sin(a)
    })
  }

  const edges = Array.from(edgeMap.values()).filter(
    (e) => nodeMap.has(e.source) && nodeMap.has(e.target),
  )
  return { nodes, edges }
}

function simulateStep(
  nodes: GraphNode[],
  nodeMap: Map<string, GraphNode>,
  edges: GraphEdge[],
) {
  const rep = 18000,
    spring = 0.03,
    restLen = 260,
    damp = 0.78,
    grav = 0.06
  const cx = LAYOUT_W / 2,
    cy = LAYOUT_H / 2

  for (let i = 0; i < nodes.length; i++) {
    for (let j = i + 1; j < nodes.length; j++) {
      const a = nodes[i],
        b = nodes[j]
      const dx = b.x - a.x,
        dy = b.y - a.y
      const d = Math.max(Math.sqrt(dx * dx + dy * dy), 1)
      const f = rep / (d * d)
      a.vx -= (f * dx) / d
      a.vy -= (f * dy) / d
      b.vx += (f * dx) / d
      b.vy += (f * dy) / d
    }
  }

  for (const e of edges) {
    const s = nodeMap.get(e.source),
      t = nodeMap.get(e.target)
    if (!s || !t) continue
    const dx = t.x - s.x,
      dy = t.y - s.y
    const d = Math.max(Math.sqrt(dx * dx + dy * dy), 1)
    const f = spring * (d - restLen)
    s.vx += (f * dx) / d
    s.vy += (f * dy) / d
    t.vx -= (f * dx) / d
    t.vy -= (f * dy) / d
  }

  for (const n of nodes) {
    n.vx += grav * (cx - n.x)
    n.vy += grav * (cy - n.y)
    n.vx *= damp
    n.vy *= damp
    n.x += n.vx
    n.y += n.vy
  }
}

type Transform = { x: number; y: number; s: number }

export function DependencyGraph({ features }: { features: Feature[] }) {
  const { featuresMap } = useContext(FeaturesContext)
  const svgRef = useRef<SVGSVGElement>(null)
  const [nodes, setNodes] = useState<GraphNode[]>([])
  const [edges, setEdges] = useState<GraphEdge[]>([])
  const [simulating, setSimulating] = useState(false)
  const [selected, setSelected] = useState<string | null>(null)
  const [transform, setTransform] = useState<Transform>({ x: 0, y: 0, s: 1 })
  const panRef = useRef(false)
  const didPanRef = useRef(false)
  const lastPt = useRef({ x: 0, y: 0 })

  const { nodes: initialNodes, edges: initialEdges } = useMemo(
    () => buildGraph(features),
    [features],
  )

  const fitToView = useCallback((ns: GraphNode[]) => {
    if (!svgRef.current || ns.length === 0) return
    const rect = svgRef.current.getBoundingClientRect()
    const W = rect.width,
      H = rect.height
    if (W === 0 || H === 0) return
    const pad = NODE_RADIUS + 40
    const minX = Math.min(...ns.map((n) => n.x)) - pad
    const maxX = Math.max(...ns.map((n) => n.x)) + pad
    const minY = Math.min(...ns.map((n) => n.y)) - pad
    const maxY = Math.max(...ns.map((n) => n.y)) + pad
    const gW = maxX - minX,
      gH = maxY - minY
    const s = Math.min(W / gW, H / gH, 2)
    setTransform({ x: (W - gW * s) / 2 - minX * s, y: (H - gH * s) / 2 - minY * s, s })
  }, [])

  useEffect(() => {
    if (initialNodes.length === 0) {
      setNodes([])
      setEdges([])
      setSimulating(false)
      return
    }
    const ns = initialNodes.map((n) => ({ ...n }))
    const nm = new Map(ns.map((n) => [n.id, n]))
    let raf: number
    let frames = 0
    setSimulating(true)
    setEdges(initialEdges)

    const tick = () => {
      for (let i = 0; i < 6; i++) simulateStep(ns, nm, initialEdges)
      frames++
      setNodes([...ns])
      if (frames < 70) {
        raf = requestAnimationFrame(tick)
      } else {
        setSimulating(false)
        fitToView(ns)
      }
    }
    raf = requestAnimationFrame(tick)
    return () => cancelAnimationFrame(raf)
  }, [initialNodes, initialEdges, fitToView])

  // Wheel zoom centered on pointer
  useEffect(() => {
    const el = svgRef.current
    if (!el) return
    const onWheel = (e: WheelEvent) => {
      e.preventDefault()
      const rect = el.getBoundingClientRect()
      const mx = e.clientX - rect.left,
        my = e.clientY - rect.top
      const factor = e.deltaY > 0 ? 0.9 : 1.1
      setTransform((prev) => {
        const ns = Math.max(0.1, Math.min(8, prev.s * factor))
        return {
          s: ns,
          x: mx - ((mx - prev.x) * ns) / prev.s,
          y: my - ((my - prev.y) * ns) / prev.s,
        }
      })
    }
    el.addEventListener('wheel', onWheel, { passive: false })
    return () => el.removeEventListener('wheel', onWheel)
  }, [])

  const onPointerDown = (e: React.PointerEvent<SVGSVGElement>) => {
    if ((e.target as Element).closest('[data-node]')) return
    panRef.current = true
    didPanRef.current = false
    lastPt.current = { x: e.clientX, y: e.clientY }
    e.currentTarget.setPointerCapture(e.pointerId)
  }

  const onPointerMove = (e: React.PointerEvent<SVGSVGElement>) => {
    if (!panRef.current) return
    const dx = e.clientX - lastPt.current.x,
      dy = e.clientY - lastPt.current.y
    if (Math.abs(dx) > 2 || Math.abs(dy) > 2) didPanRef.current = true
    lastPt.current = { x: e.clientX, y: e.clientY }
    setTransform((prev) => ({ ...prev, x: prev.x + dx, y: prev.y + dy }))
  }

  const onPointerUp = () => {
    panRef.current = false
  }

  const onSvgClick = () => {
    if (!didPanRef.current) setSelected(null)
  }

  const nodeMap = useMemo(() => new Map(nodes.map((n) => [n.id, n])), [nodes])

  const connected = useMemo(() => {
    if (!selected) return new Set<string>()
    const s = new Set<string>([selected])
    for (const e of edges) {
      if (e.source === selected) s.add(e.target)
      if (e.target === selected) s.add(e.source)
    }
    return s
  }, [selected, edges])

  const owners = useMemo(() => [...new Set(nodes.map((n) => n.owner))].sort(), [nodes])

  if (!simulating && nodes.length === 0) {
    return (
      <div className="flex h-96 items-center justify-center rounded-lg border">
        <p className="text-muted-foreground">No features found</p>
      </div>
    )
  }

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-3">
      {/* Stats + legend bar */}
      <div className="flex flex-wrap items-center justify-between gap-4 rounded-lg border px-4 py-2 text-sm">
        <div className="flex items-center gap-4 text-muted-foreground">
          <span>{nodes.length} features</span>
          <span>{edges.length} dependencies</span>
          {simulating && (
            <span className="animate-pulse text-primary">Laying out graph…</span>
          )}
        </div>
        <div className="flex flex-wrap items-center gap-x-5 gap-y-2 text-xs">
          {/* Edge type legend */}
          <div className="flex items-center gap-3">
            {(['parent', 'child', 'sibling'] as const).map((t) => (
              <span key={t} className="flex items-center gap-1.5">
                <span
                  className="inline-block h-px w-6 rounded-full"
                  style={{ backgroundColor: edgeStroke(t) }}
                />
                <span className="capitalize text-muted-foreground">{t}</span>
              </span>
            ))}
          </div>
          {/* Owner legend */}
          {owners.length > 0 && (
            <>
              <span className="text-border">|</span>
              <div className="flex flex-wrap items-center gap-3">
                {owners.map((o) => (
                  <span key={o} className="flex items-center gap-1.5">
                    <span
                      className="inline-block h-2.5 w-2.5 shrink-0 rounded-full"
                      style={{ backgroundColor: ownerColor(o) }}
                    />
                    <span className="text-muted-foreground">{o || '(no owner)'}</span>
                  </span>
                ))}
              </div>
            </>
          )}
        </div>
      </div>

      {/* Graph canvas */}
      <div className="relative min-h-0 flex-1 overflow-hidden rounded-lg border bg-muted/5">
        <svg
          ref={svgRef}
          className="h-full w-full cursor-grab active:cursor-grabbing"
          style={{ userSelect: 'none' }}
          onPointerDown={onPointerDown}
          onPointerMove={onPointerMove}
          onPointerUp={onPointerUp}
          onClick={onSvgClick}
        >
          <defs>
            {(['parent', 'child', 'sibling'] as const).map((t) => (
              <marker
                key={t}
                id={`arr-${t}`}
                markerWidth="8"
                markerHeight="6"
                refX="7"
                refY="3"
                orient="auto"
              >
                <path d="M0,0 L0,6 L8,3 Z" fill={edgeStroke(t)} />
              </marker>
            ))}
          </defs>

          <g transform={`translate(${transform.x},${transform.y}) scale(${transform.s})`}>
            {/* Edges */}
            {edges.map((e) => {
              const src = nodeMap.get(e.source),
                tgt = nodeMap.get(e.target)
              if (!src || !tgt) return null
              const dx = tgt.x - src.x,
                dy = tgt.y - src.y
              const d = Math.sqrt(dx * dx + dy * dy)
              if (d < 1) return null
              const sx = src.x + (dx / d) * NODE_RADIUS
              const sy = src.y + (dy / d) * NODE_RADIUS
              const tx = tgt.x - (dx / d) * (NODE_RADIUS + 7)
              const ty = tgt.y - (dy / d) * (NODE_RADIUS + 7)
              const hi = selected
                ? connected.has(e.source) && connected.has(e.target)
                : true
              return (
                <line
                  key={`${e.source}=>${e.target}`}
                  x1={sx}
                  y1={sy}
                  x2={tx}
                  y2={ty}
                  stroke={edgeStroke(e.type)}
                  strokeWidth={hi ? 1.5 : 0.8}
                  strokeOpacity={selected ? (hi ? 0.85 : 0.07) : 0.35}
                  markerEnd={`url(#arr-${e.type})`}
                />
              )
            })}

            {/* Nodes */}
            {nodes.map((n) => {
              const isSel = n.id === selected
              const dim = selected ? !connected.has(n.id) : false
              const label = formatFeatureName(n.label)
              const short = label.length > 15 ? `${label.slice(0, 13)}…` : label
              return (
                <g
                  key={n.id}
                  transform={`translate(${n.x},${n.y})`}
                  data-node={n.id}
                  onClick={(ev) => {
                    ev.stopPropagation()
                    setSelected((prev) => (prev === n.id ? null : n.id))
                  }}
                  style={{ cursor: 'pointer', opacity: dim ? 0.12 : 1 }}
                >
                  <circle
                    r={NODE_RADIUS}
                    fill={ownerColor(n.owner)}
                    stroke={isSel ? 'white' : 'rgba(0,0,0,0.15)'}
                    strokeWidth={isSel ? 2.5 : 1}
                  />
                  <text
                    textAnchor="middle"
                    y={NODE_RADIUS + 13}
                    fontSize={10}
                    fill="currentColor"
                    style={{ pointerEvents: 'none', userSelect: 'none' }}
                  >
                    {short}
                  </text>
                </g>
              )
            })}
          </g>
        </svg>

        {/* Zoom buttons */}
        <div className="absolute bottom-3 right-3 flex flex-col gap-1">
          {[
            {
              label: '+',
              title: 'Zoom in',
              fn: () => setTransform((p) => ({ ...p, s: Math.min(8, p.s * 1.2) })),
            },
            {
              label: '−',
              title: 'Zoom out',
              fn: () => setTransform((p) => ({ ...p, s: Math.max(0.1, p.s * 0.8) })),
            },
            { label: '⊡', title: 'Fit to view', fn: () => fitToView(nodes) },
          ].map(({ label, title, fn }) => (
            <button
              key={label}
              title={title}
              className="flex h-7 w-7 items-center justify-center rounded border bg-background text-sm shadow-sm hover:bg-muted"
              onClick={fn}
            >
              {label}
            </button>
          ))}
        </div>

        {/* Selected node info panel */}
        {selected &&
          (() => {
            const n = nodeMap.get(selected)
            if (!n) return null
            const feature = featuresMap[selected]
            const deps: Dependency[] = feature?.dependencies ?? []
            const incoming = edges.filter((e) => e.target === selected)
            return (
              <div className="absolute left-3 top-3 flex max-h-[calc(100%-1.5rem)] w-72 flex-col gap-3 overflow-hidden rounded-lg border bg-background/95 p-3 text-sm shadow backdrop-blur-sm">
                {/* Header */}
                <div className="flex items-start justify-between gap-2">
                  <p className="font-semibold leading-snug">{formatFeatureName(n.label)}</p>
                  <Link
                    to={`/?feature=${n.id}`}
                    className="shrink-0 rounded p-0.5 text-muted-foreground hover:text-foreground"
                    title="Open feature page"
                  >
                    <ArrowUpRight className="h-4 w-4" />
                  </Link>
                </div>

                {/* Path + Owner */}
                <div className="flex flex-col gap-2">
                  <div className="flex items-start gap-3">
                    <FolderTree className="mt-0.5 h-4 w-4 shrink-0 text-muted-foreground" />
                    <div className="min-w-0 flex-1">
                      <p className="mb-1 font-medium text-foreground">Path</p>
                      <p className="break-all font-mono text-xs text-muted-foreground">{n.id}</p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <User className="mt-0.5 h-4 w-4 shrink-0 text-muted-foreground" />
                    <div className="min-w-0 flex-1">
                      <p className="mb-1 font-medium text-foreground">Owner</p>
                      <div className="font-mono text-xs text-muted-foreground">
                        {feature ? <FeatureOwner feature={feature} /> : n.owner || '—'}
                      </div>
                    </div>
                  </div>
                </div>

                {/* Dependencies */}
                <div className="flex min-h-0 flex-col gap-1.5 border-t pt-2">
                  <p className="font-medium text-foreground">
                    Dependencies
                    <span className="ml-1.5 text-xs font-normal text-muted-foreground">
                      {deps.length} outgoing · {incoming.length} incoming
                    </span>
                  </p>
                  {deps.length === 0 ? (
                    <p className="text-xs text-muted-foreground">None</p>
                  ) : (
                    <ul className="min-h-0 overflow-y-auto">
                      {deps.map((dep, i) => {
                        const targetNode = nodeMap.get(dep.featurePath)
                        const targetLabel = targetNode
                          ? formatFeatureName(targetNode.label)
                          : dep.featurePath
                        return (
                          <Tooltip key={i}>
                            <TooltipTrigger asChild>
                              <li className="flex cursor-default items-center gap-1.5 rounded px-1 py-0.5 hover:bg-muted/50">
                                <span
                                  className="inline-block h-1.5 w-1.5 shrink-0 rounded-full"
                                  style={{ backgroundColor: edgeStroke(dep.type) }}
                                />
                                <span className="truncate text-xs text-muted-foreground">
                                  {targetLabel}
                                </span>
                              </li>
                            </TooltipTrigger>
                            <TooltipContent
                              side="right"
                              className="max-w-72 space-y-1.5 p-2.5 text-xs"
                            >
                              <div className="flex items-center gap-1.5">
                                <span
                                  className="inline-block h-1.5 w-1.5 shrink-0 rounded-full"
                                  style={{ backgroundColor: edgeStroke(dep.type) }}
                                />
                                <span className="font-medium capitalize">{dep.type} dependency</span>
                              </div>
                              <div className="space-y-1 font-mono">
                                <p className="text-background/70">From: {dep.sourceFilename}:{dep.line}</p>
                                <p className="text-background/70">To: {dep.targetFilename}</p>
                              </div>
                              {dep.content && (
                                <p className="rounded bg-background/10 px-1.5 py-1 font-mono text-background/90">
                                  {dep.content.trim()}
                                </p>
                              )}
                            </TooltipContent>
                          </Tooltip>
                        )
                      })}
                    </ul>
                  )}
                </div>
              </div>
            )
          })()}
      </div>

    </div>
  )
}
