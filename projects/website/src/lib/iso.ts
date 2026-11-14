/**
 * Tiny isometric drawing kit for the line illustrations ("figures") of the site.
 *
 * Scenes are described in 3D units (x to the front right, y to the front left,
 * z up) and projected to 2D. Shapes are painted in the order they are given,
 * so callers list them from back to front; faces are filled with the page
 * background so nearer shapes hide the ones behind them.
 */
export type Point3 = [number, number, number]

/** How a shape is filled: a side face, a top face, or not at all. */
export type Fill = 'side' | 'top' | 'none'
/** How a shape is stroked: regular, highlighted or barely visible. */
export type Stroke = 'base' | 'strong' | 'faint'

export interface Shape {
  kind: 'polygon' | 'polyline' | 'ellipse'
  points: Point3[]
  fill?: Fill
  stroke?: Stroke
  dashed?: boolean
  /** Radius of an ellipse, which is a circle lying flat on the ground plane. */
  radius?: number
}

interface ShapeOptions {
  stroke?: Stroke
  dashed?: boolean
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
  kind: 'polygon',
  fill: options.fill ?? 'none',
  stroke: options.stroke ?? 'base',
  dashed: options.dashed,
  points: [
    [x, y, z],
    [x + w, y, z],
    [x + w, y + d, z],
    [x, y + d, z],
  ],
})

export const line = (points: Point3[], options: ShapeOptions = {}): Shape => ({
  kind: 'polyline',
  points,
  stroke: options.stroke ?? 'base',
  dashed: options.dashed,
})

/** A circle lying flat on the plane z, centered on `center`. */
export const circle = (
  center: Point3,
  radius: number,
  options: ShapeOptions & { fill?: Fill } = {},
): Shape => ({
  kind: 'ellipse',
  points: [center],
  radius,
  fill: options.fill ?? 'top',
  stroke: options.stroke ?? 'base',
  dashed: options.dashed,
})

/** Projects the shapes and computes the view box that fits them. */
export const render = (shapes: Shape[], padding = 1) => {
  const xs: number[] = []
  const ys: number[] = []
  const items = shapes.map((shape) => {
    const points = shape.points.map(project)
    for (const [px, py] of points) {
      const rx = shape.radius ? shape.radius * Math.SQRT2 * COS30 : 0
      const ry = shape.radius ? shape.radius * Math.SQRT1_2 : 0
      xs.push(px - rx, px + rx)
      ys.push(py - ry, py + ry)
    }
    return {
      ...shape,
      d: points
        .map(([px, py]) => `${px.toFixed(2)},${py.toFixed(2)}`)
        .join(' '),
      cx: points[0][0],
      cy: points[0][1],
      rx: (shape.radius ?? 0) * Math.SQRT2 * COS30,
      ry: (shape.radius ?? 0) * Math.SQRT1_2,
    }
  })
  const minX = Math.min(...xs) - padding
  const minY = Math.min(...ys) - padding
  const width = Math.max(...xs) + padding - minX
  const height = Math.max(...ys) + padding - minY
  return {
    items,
    viewBox: [minX, minY, width, height].map((n) => n.toFixed(2)).join(' '),
  }
}
