/**
 * Tiny isometric drawing kit for the line illustrations ("figures") of the site.
 *
 * Scenes are described in 3D units (x to the front right, y to the front left,
 * z up) and projected to 2D. Shapes are painted in the order they are given,
 * so callers list them from back to front; faces are filled with the page
 * background so nearer shapes hide the ones behind them.
 */
export type Point3 = [number, number, number]

/** How a shape is filled: a side face, a top face, solid ink, or not at all. */
export type Fill = 'side' | 'top' | 'solid' | 'none'
/** How a shape is stroked: regular, highlighted or barely visible. */
export type Stroke = 'base' | 'strong' | 'faint'

/**
 * How a shape or a group moves: an animation class (see IsoFigure.astro) and
 * the time it starts at, in milliseconds.
 */
export interface Motion {
  anim?: string
  delay?: number
}

export interface Shape extends Motion {
  /** A `dot` is a round marker drawn flat on the screen, whatever the angle. */
  kind: 'polygon' | 'polyline' | 'ellipse' | 'dot'
  points: Point3[]
  fill?: Fill
  stroke?: Stroke
  dashed?: boolean
  /** Radius of an ellipse (a circle lying flat on the ground) or of a dot. */
  radius?: number
  /** Normalizes the path length to 1 so the shape can draw itself. */
  draw?: boolean
  /** Where a moving shape travels to, exposed to CSS as `--dx` and `--dy`. */
  travel?: Point3
}

/** Shapes animated together, painted in place among the others. */
export interface Group extends Motion {
  kind: 'group'
  children: Node[]
}

export type Node = Shape | Group

interface ShapeOptions extends Motion {
  stroke?: Stroke
  dashed?: boolean
  draw?: boolean
}

const COS30 = Math.cos(Math.PI / 6)

export const project = ([x, y, z]: Point3): [number, number] => [
  (x - y) * COS30,
  (x + y) * 0.5 - z,
]

/** A box from its back corner (x, y, z) and its size; returns its three visible faces. */
export const box = (
  [x, y, z]: Point3,
  [w, d, h]: Point3,
  {
    stroke = 'base',
    dashed = false,
    top = stroke,
  }: ShapeOptions & { top?: Stroke } = {},
): Shape[] => [
  {
    kind: 'polygon',
    fill: 'side',
    stroke,
    dashed,
    points: [
      [x, y + d, z],
      [x + w, y + d, z],
      [x + w, y + d, z + h],
      [x, y + d, z + h],
    ],
  },
  {
    kind: 'polygon',
    fill: 'side',
    stroke,
    dashed,
    points: [
      [x + w, y, z],
      [x + w, y + d, z],
      [x + w, y + d, z + h],
      [x + w, y, z + h],
    ],
  },
  {
    kind: 'polygon',
    fill: 'top',
    stroke: top,
    dashed,
    points: [
      [x, y, z + h],
      [x + w, y, z + h],
      [x + w, y + d, z + h],
      [x, y + d, z + h],
    ],
  },
]

/** A flat rectangle lying on the plane z. */
export const rect = (
  [x, y, z]: Point3,
  [w, d]: [number, number],
  options: ShapeOptions & { fill?: Fill } = {},
): Shape => ({
  ...options,
  kind: 'polygon',
  fill: options.fill ?? 'none',
  stroke: options.stroke ?? 'base',
  points: [
    [x, y, z],
    [x + w, y, z],
    [x + w, y + d, z],
    [x, y + d, z],
  ],
})

export const line = (points: Point3[], options: ShapeOptions = {}): Shape => ({
  ...options,
  kind: 'polyline',
  points,
  stroke: options.stroke ?? 'base',
})

/** A circle lying flat on the plane z, centered on `center`. */
export const circle = (
  center: Point3,
  radius: number,
  options: ShapeOptions & { fill?: Fill } = {},
): Shape => ({
  ...options,
  kind: 'ellipse',
  points: [center],
  radius,
  fill: options.fill ?? 'top',
  stroke: options.stroke ?? 'base',
})

/** A round marker of `radius` screen units, optionally travelling to `travel`. */
export const dot = (
  center: Point3,
  radius: number,
  options: Motion & { travel?: Point3; stroke?: Stroke } = {},
): Shape => ({
  ...options,
  kind: 'dot',
  points: [center],
  radius,
  fill: 'solid',
  stroke: options.stroke ?? 'strong',
})

/** Groups shapes so they animate together. */
export const group = (children: Node[], motion: Motion = {}): Group => ({
  ...motion,
  kind: 'group',
  children,
})

export type RenderedShape = Shape & {
  d: string
  cx: number
  cy: number
  rx: number
  ry: number
  /** Screen offset to `travel`, when the shape moves. */
  dx?: number
  dy?: number
}
export type RenderedGroup = Motion & { kind: 'group'; children: Rendered[] }
export type Rendered = RenderedShape | RenderedGroup

/** Projects the shapes and computes the view box that fits them. */
export const render = (nodes: Node[], padding = 1) => {
  const xs: number[] = []
  const ys: number[] = []
  const visit = (node: Node): Rendered => {
    if (node.kind === 'group')
      return { ...node, children: node.children.map(visit) }
    const points = node.points.map(project)
    const flat = node.kind === 'ellipse'
    const rx = flat
      ? (node.radius ?? 0) * Math.SQRT2 * COS30
      : (node.radius ?? 0)
    const ry = flat ? (node.radius ?? 0) * Math.SQRT1_2 : (node.radius ?? 0)
    for (const [px, py] of points) {
      xs.push(px - rx, px + rx)
      ys.push(py - ry, py + ry)
    }
    const to = node.travel && project(node.travel)
    return {
      ...node,
      d: points
        .map(([px, py]) => `${px.toFixed(2)},${py.toFixed(2)}`)
        .join(' '),
      cx: points[0][0],
      cy: points[0][1],
      rx,
      ry,
      dx: to && to[0] - points[0][0],
      dy: to && to[1] - points[0][1],
    }
  }
  const items = nodes.map(visit)
  const minX = Math.min(...xs) - padding
  const minY = Math.min(...ys) - padding
  const width = Math.max(...xs) + padding - minX
  const height = Math.max(...ys) + padding - minY
  return {
    items,
    viewBox: [minX, minY, width, height].map((n) => n.toFixed(2)).join(' '),
  }
}
