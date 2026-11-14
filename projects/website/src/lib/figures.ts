/*
 * Isometric figures of the home page, each one a view of the dashboard drawn
 * as a circuit board: features are chips, folders are the zones printed on
 * the board, imports are traces, and specs and decisions are the datasheets
 * shipped with a chip. Each figure builds itself when it scrolls into view
 * (the board rises, chips drop, traces draw), then loops a small story.
 *
 * Boards are laid out like pages: x runs to the right of the page, y down it.
 * Times are in milliseconds; the animation classes live in IsoFigure.astro.
 */
import {
  box,
  circle,
  dot,
  group,
  line,
  rect,
  shape,
  type Node,
  type Point3,
  type Shape,
  type Stroke,
} from './iso'

type LineOptions = Parameters<typeof line>[1]

/** A short horizontal stroke standing for a line of text, on the plane z. */
const textLine = (
  [x, y, z]: Point3,
  length: number,
  options: LineOptions = {},
): Shape =>
  line(
    [
      [x, y, z],
      [x + length, y, z],
    ],
    { stroke: 'faint', ...options },
  )

/** A line of text that writes itself at `delay`. */
const write = (
  at: Point3,
  length: number,
  delay: number,
  stroke: Stroke = 'faint',
): Shape =>
  textLine(at, length, { stroke, draw: true, anim: 'anim-iso-draw', delay })

/** A folder icon, its tab at the back, on the plane z. */
const folder = (at: Point3, options: LineOptions = {}): Shape =>
  shape(
    at,
    [
      [0, 0],
      [0.22, 0],
      [0.3, 0.1],
      [0.6, 0.1],
      [0.6, 0.48],
      [0, 0.48],
    ],
    { stroke: 'faint', ...options },
  )

/** A chevron pointing right, at the end of a clickable row. */
const chevron = ([x, y, z]: Point3): Shape =>
  line(
    [
      [x - 0.15, y - 0.3, z],
      [x + 0.15, y, z],
      [x - 0.15, y + 0.3, z],
    ],
    { stroke: 'faint' },
  )

/** How far the legs of a chip reach out of its body, on the board. */
const LEG = 0.35

/**
 * A chip standing on the board at z: its legs at the back, its body, then
 * its legs at the front, so the body hides what it should. `top` draws the
 * markings printed on it, at the height of its top face.
 */
const chip = (
  [x, y, z]: Point3,
  [w, d]: [number, number],
  {
    height = 0.45,
    stroke = 'base',
    top = stroke,
    markings = () => [],
  }: {
    height?: number
    stroke?: Stroke
    top?: Stroke
    markings?: (top: number) => Node[]
  } = {},
): Node[] => {
  const pitch = 0.5
  const knee = z + 0.15
  /** Positions of the legs along a side, centered on it. */
  const along = (length: number) => {
    const count = Math.max(1, Math.floor((length - 0.3) / pitch))
    const start = (length - (count - 1) * pitch) / 2
    return Array.from({ length: count }, (_, index) => start + index * pitch)
  }
  const leg = (from: Point3, [ux, uy]: [number, number]) =>
    line(
      [
        from,
        [from[0] + ux * LEG * 0.6, from[1] + uy * LEG * 0.6, knee],
        [from[0] + ux * LEG, from[1] + uy * LEG, z],
      ],
      { stroke: 'faint' },
    )
  const xs = along(w)
  const ys = along(d)
  return [
    ...xs.map((u) => leg([x + u, y, knee], [0, -1])),
    ...ys.map((v) => leg([x, y + v, knee], [-1, 0])),
    ...box([x, y, z], [w, d, height], { stroke, top }),
    ...xs.map((u) => leg([x + u, y + d, knee], [0, 1])),
    ...ys.map((v) => leg([x + w, y + v, knee], [1, 0])),
    ...markings(z + height),
  ]
}

/** The board every figure is drawn on, with a mounting hole in each corner. */
const board = (width: number, depth: number, height: number): Node =>
  group(
    [
      ...box([0, 0, 0], [width, depth, height], { stroke: 'faint' }),
      ...[
        [0.45, 0.45],
        [width - 0.45, 0.45],
        [0.45, depth - 0.45],
        [width - 0.45, depth - 0.45],
      ].map(([u, v]) =>
        circle([u, v, height], 0.18, { fill: 'none', stroke: 'faint' }),
      ),
    ],
    { anim: 'anim-iso-rise' },
  )

/** A trace on the board through `points`, with a pad at each end. */
const trace = (
  points: Point3[],
  options: LineOptions & { pads?: boolean } = {},
): Shape[] => {
  const { pads = true, ...rest } = options
  return [
    line(points, rest),
    ...(pads
      ? [points[0], points[points.length - 1]].map((at) =>
          circle(at, 0.13, { stroke: rest.stroke ?? 'base' }),
        )
      : []),
  ]
}

/**
 * The Insights page as a board: every folder is a zone printed on it with
 * its name, and every feature found in it a chip, as large as the feature.
 * The dot in the corner of a chip is its owner. In a loop, features are
 * picked one by one: the chip lifts and its footprint lights up.
 */
const map = (): Node[] => {
  const z = 0.4
  const nodes: Node[] = [board(14, 11.1, z)]

  type Feature = {
    at: [number, number]
    size: [number, number]
    owner: Stroke
    pick?: number
  }
  const zones: {
    at: [number, number]
    size: [number, number]
    name: number
    features: Feature[]
  }[] = [
    {
      at: [0.5, 0.5],
      size: [6.9, 5.1],
      name: 1.6,
      features: [
        { at: [1.1, 1.7], size: [3.0, 3.2], owner: 'strong', pick: 2600 },
        { at: [4.9, 1.7], size: [1.8, 1.3], owner: 'base' },
        { at: [4.9, 3.8], size: [1.8, 1.3], owner: 'faint' },
      ],
    },
    {
      at: [7.9, 0.5],
      size: [5.6, 5.1],
      name: 1.2,
      features: [
        { at: [8.5, 1.7], size: [2.2, 3.2], owner: 'base', pick: 5100 },
        { at: [11.6, 1.7], size: [1.3, 1.3], owner: 'strong' },
        { at: [11.6, 3.8], size: [1.3, 1.3], owner: 'base' },
      ],
    },
    {
      at: [0.5, 6.2],
      size: [13.0, 4.4],
      name: 2.0,
      features: [
        { at: [1.1, 7.4], size: [2.2, 2.5], owner: 'faint' },
        { at: [4.2, 7.4], size: [3.6, 2.5], owner: 'strong', pick: 7600 },
        { at: [8.6, 7.4], size: [1.6, 2.5], owner: 'base' },
        { at: [11.0, 7.4], size: [1.9, 1.7], owner: 'base' },
      ],
    },
  ]

  // Zones printed on the board, each with its folder and its name.
  zones.forEach(({ at: [u, v], size, name }, index) => {
    const delay = 250 + index * 120
    nodes.push(
      group([rect([u, v, z], size, { stroke: 'faint', dashed: true })], {
        anim: 'anim-iso-fade',
        delay,
      }),
      group([folder([u + 0.35, v + 0.35, z], { stroke: 'base' })], {
        anim: 'anim-iso-pop',
        delay: delay + 80,
      }),
      write([u + 1.15, v + 0.6, z], name, delay + 120, 'base'),
    )
  })

  // A bus between the zones, as on any board.
  nodes.push(
    ...[5.82, 6.0].map((v, index) =>
      line(
        [
          [0.9, v, z],
          [13.1, v, z],
        ],
        {
          stroke: 'faint',
          draw: true,
          anim: 'anim-iso-draw',
          delay: 500 + index * 80,
        },
      ),
    ),
    line(
      [
        [7.65, 0.9, z],
        [7.65, 5.82, z],
      ],
      { stroke: 'faint', draw: true, anim: 'anim-iso-draw', delay: 650 },
    ),
  )

  // Chips, back to front; the footprint of a picked chip lights up under it.
  zones
    .flatMap(({ features }) => features)
    .forEach(({ at: [u, v], size: [w, d], owner, pick }, index) => {
      if (pick !== undefined)
        nodes.push(
          rect([u - 0.55, v - 0.55, z], [w + 1.1, d + 1.1], {
            stroke: 'strong',
            anim: 'anim-iso-select',
            delay: pick,
          }),
        )
      const body = chip([u, v, z], [w, d], {
        top: owner === 'strong' ? 'strong' : 'base',
        markings: (top) => [
          circle([u + 0.4, v + 0.4, top], 0.15, {
            fill: owner === 'strong' ? 'solid' : 'none',
            stroke: owner,
          }),
          textLine([u + 0.4, v + d / 2, top], Math.min(w - 0.8, 1.8), {
            stroke: 'base',
          }),
          ...(d > 2
            ? [textLine([u + 0.4, v + d / 2 + 0.5, top], (w - 0.8) * 0.6)]
            : []),
        ],
      })
      nodes.push(
        group(
          [
            pick === undefined
              ? group(body)
              : group(body, { anim: 'anim-iso-lift', delay: pick }),
          ],
          { anim: 'anim-iso-drop', delay: 700 + index * 90 },
        ),
      )
    })
  return nodes
}

/**
 * The dependency graph as a board: features are chips marked by owner, and
 * imports the traces between them. In a loop, a pull request changes one
 * chip: the signal runs along the traces to the chips importing it, then
 * the owners it touches light up in the legend printed on the board.
 */
const owners = (): Node[] => {
  const z = 0.3
  const height = 0.45
  const top = z + height
  const loop = 2400
  type Owner = 'a' | 'b' | 'c'

  const at = (u: number, v: number): Point3 => [u, v, z]
  const changed = at(7.0, 6.4)
  const graph = {
    left: at(2.6, 3.4),
    below: at(2.8, 9.4),
    right: at(11.4, 3.2),
    front: at(11.2, 9.6),
  }

  /** What tells owners apart on a chip or in the legend. */
  const mark = (center: Point3, owner: Owner, size: number): Node[] => {
    if (owner === 'a')
      return [circle(center, size * 0.35, { fill: 'solid', stroke: 'strong' })]
    if (owner === 'b')
      return [circle(center, size * 0.45, { fill: 'none', stroke: 'base' })]
    const half = size * 0.35
    return [
      rect(
        [center[0] - half, center[1] - half, center[2]],
        [half * 2, half * 2],
        { stroke: 'base' },
      ),
    ]
  }

  const node = (
    [u, v]: Point3,
    owner: Owner,
    delay: number,
    half = 1.1,
  ): Node =>
    group(
      chip([u - half, v - half, z], [half * 2, half * 2], {
        height,
        top: owner === 'a' ? 'strong' : 'base',
        markings: (level) => [
          ...mark([u, v, level], owner, 1),
          textLine([u - half + 0.35, v + half - 0.4, level], half),
        ],
      }),
      { anim: 'anim-iso-drop', delay },
    )

  const ping = (center: Point3, size: number, delay: number) =>
    circle(center, size, {
      fill: 'none',
      stroke: 'strong',
      anim: 'anim-iso-ping',
      delay,
    })

  // Legend printed on the board: one entry per owner.
  const legend = (
    [
      ['a', 5.8],
      ['b', 8.2],
      ['c', 10.6],
    ] as const
  ).map(([owner, u]) => ({ owner, center: [u, 1.0, z] as Point3 }))

  // Imports, routed between the legs of the chips.
  const routes = {
    left: [at(6.6, 4.65), at(6.6, 3.6), at(4.15, 3.6)],
    front: [at(7.2, 8.15), at(7.2, 9.8), at(9.65, 9.8)],
    right: [at(8.75, 6.8), at(11.0, 6.8), at(11.0, 4.75)],
    below: [at(2.6, 7.85), at(2.6, 4.95)],
  }
  const wire = (points: Point3[], delay: number) =>
    group(trace(points, { stroke: 'base' }), {
      anim: 'anim-iso-fade',
      delay,
    })

  /** The signal of the change, running a route one segment after the other. */
  const signal = (points: Point3[], delay: number) =>
    points.slice(1).map((to, index) =>
      dot(points[index], 0.3, {
        travel: to,
        anim: 'anim-iso-travel',
        delay: delay + index * 1200,
      }),
    )
  const arrival = loop + 250 + 1200 + 1440

  return [
    board(14, 11.6, z),
    group(
      [
        textLine([0.9, 1.0, z], 1.3, { stroke: 'base' }),
        textLine([2.6, 1.0, z], 1.6),
      ],
      { anim: 'anim-iso-fade', delay: 250 },
    ),
    ...legend.map(({ owner, center }, index) =>
      group(
        [
          circle(center, 0.3, {
            fill: 'none',
            stroke: owner === 'a' ? 'strong' : 'base',
          }),
          ...mark(center, owner, 0.3),
          textLine([center[0] + 0.55, 1.0, z], 1.1),
        ],
        { anim: 'anim-iso-pop', delay: 400 + index * 90 },
      ),
    ),

    // Traces, under the chips.
    wire(routes.right, 1300),
    wire(routes.below, 1380),
    wire(routes.left, 1460),
    wire(routes.front, 1540),

    // Chips, back to front.
    node(graph.left, 'b', 600),
    node(graph.below, 'b', 700),
    node(changed, 'a', 800, 1.3),
    node(graph.right, 'c', 900),
    node(graph.front, 'a', 1000),

    // The pull request, pinned on the chip it changes.
    group(
      [
        line(
          [
            [changed[0], changed[1], top],
            [changed[0], changed[1], 3.9],
          ],
          {
            stroke: 'base',
          },
        ),
        {
          kind: 'polygon',
          fill: 'side',
          stroke: 'strong',
          points: [
            [changed[0], changed[1], 3.9],
            [changed[0] + 2.6, changed[1], 3.9],
            [changed[0] + 2.6, changed[1], 2.7],
            [changed[0], changed[1], 2.7],
          ],
        } satisfies Shape,
        // Branch glyph and title of the pull request.
        dot([changed[0] + 0.45, changed[1], 3.5], 0.14, { stroke: 'base' }),
        dot([changed[0] + 0.45, changed[1], 3.0], 0.14, { stroke: 'base' }),
        line(
          [
            [changed[0] + 0.45, changed[1], 3.5],
            [changed[0] + 0.45, changed[1], 3.0],
          ],
          { stroke: 'base' },
        ),
        textLine([changed[0] + 0.9, changed[1], 3.5], 1.4, { stroke: 'base' }),
        textLine([changed[0] + 0.9, changed[1], 3.05], 0.9),
      ],
      { anim: 'anim-iso-drop', delay: 1700 },
    ),

    // The loop: the change, the signal to the importers, the owners touched.
    ping([changed[0], changed[1], top], 1.3, loop),
    ...signal(routes.left, loop + 250),
    ...signal(routes.front, loop + 250),
    ping([graph.left[0], graph.left[1], top], 1.1, arrival),
    ping([graph.front[0], graph.front[1], top], 1.1, arrival),
    ...legend
      .filter(({ owner }) => owner !== 'c')
      .map(({ center }) => ping(center, 0.3, arrival + 250)),
  ]
}

/**
 * The Specs & Decisions tab as a chip and its datasheets: the feature, then
 * its decision record, the spec it implements with its task progress, and
 * the record it superseded, wired to the chip. In a loop, the next task of
 * the spec gets done.
 */
const specs = (): Node[] => {
  const z = 0.3
  const card = 0.35
  const nodes: Node[] = [board(10, 13.2, z)]

  // The feature: its chip, then its name, path and owner printed beside it.
  nodes.push(
    group(
      chip([0.9, 0.6, z], [3.2, 2.4], {
        height: 0.5,
        top: 'strong',
        markings: (top) => [
          circle([1.3, 1.0, top], 0.15, { fill: 'solid', stroke: 'strong' }),
          textLine([1.3, 1.8, top], 2.0, { stroke: 'base' }),
          textLine([1.3, 2.3, top], 1.2),
        ],
      }),
      { anim: 'anim-iso-drop', delay: 300 },
    ),
    write([5.0, 1.0, z], 3.4, 450, 'strong'),
    group([folder([4.95, 1.55, z])], { anim: 'anim-iso-pop', delay: 500 }),
    write([5.8, 1.8, z], 2.8, 550),
    group([circle([5.25, 2.55, z], 0.2, { stroke: 'faint' })], {
      anim: 'anim-iso-pop',
      delay: 600,
    }),
    write([5.8, 2.55, z], 1.2, 650),
  )

  // Tabs, Specs & Decisions open.
  nodes.push(
    group(
      [
        rect([0.6, 3.65, z], [8.8, 0.8], { stroke: 'faint' }),
        textLine([0.95, 4.05, z], 1.0),
        textLine([2.3, 4.05, z], 0.9),
        rect([3.6, 3.75, z], [2.9, 0.6], { stroke: 'strong' }),
        textLine([3.9, 4.05, z], 1.7, { stroke: 'base' }),
        textLine([5.9, 4.05, z], 0.2, { stroke: 'base' }),
        textLine([6.9, 4.05, z], 1.0),
        textLine([8.2, 4.05, z], 0.8),
      ],
      { anim: 'anim-iso-fade', delay: 750 },
    ),
  )

  /** A datasheet: its kind and status badges, its date and its title. */
  const record = (
    v: number,
    depth: number,
    delay: number,
    stroke: Stroke,
    extra: (top: number) => Node[] = () => [],
  ) => {
    const top = z + card
    const dashed = stroke === 'faint'
    return group(
      [
        ...box([0.6, v, z], [8.8, depth, card], { stroke, dashed }),
        rect([1.0, v + 0.35, top], [1.4, 0.6], {
          stroke: dashed ? 'faint' : 'strong',
        }),
        circle([1.35, v + 0.65, top], 0.12, {
          stroke: dashed ? 'faint' : 'base',
        }),
        textLine([1.6, v + 0.65, top], 0.55, {
          stroke: dashed ? 'faint' : 'base',
        }),
        rect([2.6, v + 0.35, top], [1.7, 0.6], { stroke: 'faint', dashed }),
        textLine([2.9, v + 0.65, top], 1.1, {
          stroke: dashed ? 'faint' : 'base',
        }),
        textLine([4.6, v + 0.65, top], 1.3),
        chevron([8.8, v + depth / 2, top]),
        ...extra(top),
      ],
      { anim: 'anim-iso-drop', delay },
    )
  }

  // The decision record, with links to its spec and to the record it supersedes.
  nodes.push(
    record(4.9, 2.6, 900, 'base', (top) => [
      textLine([1.0, 6.2, top], 4.4, { stroke: 'base' }),
      rect([1.0, 6.7, top], [2.7, 0.5], { stroke: 'faint' }),
      textLine([1.3, 6.95, top], 2.0),
      rect([3.9, 6.7, top], [3.3, 0.5], { stroke: 'faint' }),
      textLine([4.2, 6.95, top], 2.6),
    ]),
  )

  // The spec, its tasks half done.
  const progress = 9.85
  nodes.push(
    record(7.9, 2.75, 1050, 'strong', (top) => [
      textLine([1.0, 9.2, top], 3.2, { stroke: 'base' }),
      textLine([1.0, progress, top], 3, { stroke: 'faint' }),
      textLine([4.4, progress, top], 0.5),
    ]),
    textLine([1.0, progress, z + card], 1.5, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-draw',
      delay: 1600,
    }),
    textLine([2.5, progress, z + card], 0.75, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-tick',
      delay: 2700,
    }),
  )

  // The superseded record, struck through.
  nodes.push(
    record(11.05, 1.85, 1200, 'faint', (top) => [
      textLine([1.0, 12.35, top], 3.8),
      line(
        [
          [2.75, 11.7, top],
          [4.15, 11.7, top],
        ],
        { stroke: 'faint' },
      ),
    ]),
  )

  // Traces along the margin: from the chip to its decision and its spec,
  // then from the decision to the record it superseded.
  nodes.push(
    group(
      [
        line(
          [
            [0.9 - LEG, 1.8, z],
            [0.25, 1.8, z],
            [0.25, 9.2, z],
            [0.6, 9.2, z],
          ],
          { dashed: true, anim: 'anim-iso-flow' },
        ),
        line(
          [
            [0.25, 6.2, z],
            [0.6, 6.2, z],
          ],
          { dashed: true, anim: 'anim-iso-flow' },
        ),
        line(
          [
            [0.6, 6.95, z],
            [0.4, 6.95, z],
            [0.4, 12.0, z],
            [0.6, 12.0, z],
          ],
          { dashed: true, stroke: 'faint' },
        ),
      ],
      { anim: 'anim-iso-fade', delay: 1450 },
    ),
  )
  return nodes
}

export const figures = { map, owners, specs }
export type FigureName = keyof typeof figures
