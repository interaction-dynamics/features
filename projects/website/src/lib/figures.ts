/*
 * Isometric figures of the home page, each one a view of the dashboard drawn
 * as a plate: the Insights page, the dependency graph and the Specs &
 * Decisions tab of a feature. Each figure builds itself when it scrolls into
 * view (plates rise, cards drop, lines draw), then loops a small story.
 *
 * Plates are laid out like pages: x runs to the right of the page, y down it.
 * Times are in milliseconds; the animation classes live in IsoFigure.astro.
 */
import {
  box,
  circle,
  disc,
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

/** The page every figure is drawn on, rising first. */
const plate = (width: number, depth: number, height: number): Node =>
  group(box([0, 0, 0], [width, depth, height], { stroke: 'faint' }), {
    anim: 'anim-iso-rise',
  })

/**
 * The Insights page: the feature tree built from the folders in the sidebar,
 * totals on cards, and every feature in a table. In a loop, features are
 * picked one by one in the sidebar and highlighted in the table.
 */
const insights = (): Node[] => {
  const z = 0.4
  const nodes: Node[] = [plate(14, 10.6, z)]

  // Sidebar: logo, search, then folders and the features found in them.
  const tree: { v: number; folder?: boolean; length: number; pick?: number }[] =
    [
      { v: 3.2, folder: true, length: 0.9 },
      { v: 4.0, length: 1.3, pick: 2600 },
      { v: 4.8, length: 1.0, pick: 5100 },
      { v: 5.6, length: 1.5 },
      { v: 6.7, folder: true, length: 1.1 },
      { v: 7.5, length: 1.1, pick: 7600 },
      { v: 8.3, length: 0.9 },
    ]
  nodes.push(
    group(
      [
        rect([0.6, 0.55, z], [0.7, 0.7], { stroke: 'base' }),
        textLine([1.6, 0.9, z], 1.4, { stroke: 'base' }),
        rect([0.5, 1.65, z], [2.8, 0.7], { stroke: 'faint' }),
        textLine([0.9, 2.0, z], 1.2),
        line(
          [
            [3.8, 0.3, z],
            [3.8, 10.3, z],
          ],
          { stroke: 'faint' },
        ),
      ],
      { anim: 'anim-iso-fade', delay: 250 },
    ),
    group(
      [
        line(
          [
            [0.9, 3.6, z],
            [0.9, 6.0, z],
          ],
          { stroke: 'faint' },
        ),
        line(
          [
            [0.9, 7.1, z],
            [0.9, 8.7, z],
          ],
          { stroke: 'faint' },
        ),
      ],
      { anim: 'anim-iso-fade', delay: 600 },
    ),
  )
  tree.forEach(({ v, folder: isFolder, length }, index) => {
    const delay = 400 + index * 70
    if (isFolder) {
      nodes.push(
        group([folder([0.5, v - 0.25, z], { stroke: 'base' })], {
          anim: 'anim-iso-pop',
          delay,
        }),
        write([1.3, v, z], length, delay + 40, 'base'),
      )
      return
    }
    nodes.push(
      write([1.3, v, z], length, delay + 40),
      group([circle([3.2, v, z], 0.16, { stroke: 'base' })], {
        anim: 'anim-iso-pop',
        delay,
      }),
    )
  })

  // Totals: features, owners, the largest feature, features without owners.
  const values = [0.5, 0.3, 0.9, 0.2]
  values.forEach((value, index) => {
    const u = 4.4 + index * 2.35
    const top = z + 0.55
    nodes.push(
      group(
        [
          ...box([u, 0.6, z], [2.05, 2.0, 0.55]),
          textLine([u + 0.35, 1.1, top], 1.2),
          textLine([u + 0.35, 1.9, top], value + 0.3, { stroke: 'strong' }),
        ],
        { anim: 'anim-iso-drop', delay: 550 + index * 110 },
      ),
    )
  })

  // Tabs: Features (open), Ownership, Dependencies.
  nodes.push(
    group(
      [
        rect([4.4, 3.2, z], [5.4, 0.75], { stroke: 'faint' }),
        rect([4.5, 3.3, z], [1.5, 0.55], { stroke: 'base' }),
        textLine([4.75, 3.58, z], 1.0, { stroke: 'base' }),
        textLine([6.35, 3.58, z], 1.2),
        textLine([7.9, 3.58, z], 1.6),
      ],
      { anim: 'anim-iso-fade', delay: 950 },
    ),
  )

  // Table: a feature per row, with its owner and a bar for its lines.
  const rows: { name: number; owner: Stroke; lines: number; pick?: number }[] =
    [
      { name: 1.5, owner: 'strong', lines: 1.2, pick: 2600 },
      { name: 1.1, owner: 'base', lines: 2.6, pick: 5100 },
      { name: 1.8, owner: 'strong', lines: 0.6 },
      { name: 1.3, owner: 'faint', lines: 1.7, pick: 7600 },
      { name: 0.9, owner: 'base', lines: 0.9 },
    ]
  nodes.push(
    group(
      [
        rect([4.4, 4.4, z], [9.2, 5.9], { stroke: 'faint' }),
        textLine([4.75, 4.85, z], 1.4),
        textLine([7.6, 4.85, z], 0.9),
        textLine([10.4, 4.85, z], 0.8),
        line(
          [
            [4.4, 5.25, z],
            [13.6, 5.25, z],
          ],
          { stroke: 'faint' },
        ),
      ],
      { anim: 'anim-iso-fade', delay: 1050 },
    ),
  )
  rows.forEach(({ name, owner, lines, pick }, index) => {
    const v = 5.75 + index * 0.95
    const delay = 1150 + index * 90
    if (pick !== undefined)
      nodes.push(
        rect([4.5, v - 0.37, z], [9.0, 0.74], {
          stroke: 'base',
          anim: 'anim-iso-select',
          delay: pick,
        }),
      )
    nodes.push(
      write([4.75, v, z], name, delay, 'base'),
      group([circle([7.75, v, z], 0.17, { stroke: owner })], {
        anim: 'anim-iso-pop',
        delay: delay + 60,
      }),
      write([8.1, v, z], 0.9, delay + 80),
      write([10.4, v, z], lines, delay + 120, 'strong'),
    )
  })

  // Picking a feature in the sidebar.
  tree.forEach(({ v, pick }) => {
    if (pick === undefined) return
    nodes.push(
      rect([1.05, v - 0.35, z], [2.55, 0.7], {
        stroke: 'strong',
        anim: 'anim-iso-select',
        delay: pick,
      }),
    )
  })
  return nodes
}

/**
 * The dependency graph: features as nodes marked by owner, imports as dashed
 * links. In a loop, a pull request changes one feature: the change flows to
 * the features importing it, then the owners it touches light up in the
 * legend.
 */
const dependencies = (): Node[] => {
  const z = 0.3
  const height = 0.5
  const radius = 1.05
  const top = z + height
  const loop = 2400
  type Owner = 'a' | 'b' | 'c'

  const at = (u: number, v: number): Point3 => [u, v, top]
  const changed = at(6.5, 5.8)
  const graph = {
    left: at(2.9, 3.6),
    below: at(3.0, 8.6),
    right: at(10.0, 3.2),
    front: at(9.8, 8.8),
  }

  /** What tells owners apart on a node or in the legend. */
  const mark = (center: Point3, owner: Owner, size: number): Node[] => {
    if (owner === 'a') return []
    if (owner === 'b')
      return [circle(center, size * 0.45, { fill: 'none', stroke: 'base' })]
    const half = size * 0.3
    return [
      rect(
        [center[0] - half, center[1] - half, center[2]],
        [half * 2, half * 2],
        {
          stroke: 'base',
        },
      ),
    ]
  }

  const node = (center: Point3, owner: Owner, delay: number) =>
    group(
      [
        ...disc([center[0], center[1], z], radius, height, {
          top: owner === 'a' ? 'strong' : 'base',
        }),
        ...mark(center, owner, radius),
      ],
      { anim: 'anim-iso-drop', delay },
    )

  const ping = (center: Point3, size: number, delay: number) =>
    circle(center, size, {
      fill: 'none',
      stroke: 'strong',
      anim: 'anim-iso-ping',
      delay,
    })

  // Legend: the counts, then one entry per owner.
  const legend = (
    [
      ['a', 5.6],
      ['b', 8.0],
      ['c', 10.4],
    ] as const
  ).map(([owner, u]) => ({ owner, center: [u, 0.95, z] as Point3 }))

  const link = (from: Point3, to: Point3, delay: number, flow = false) =>
    group(
      [
        line([from, to], {
          dashed: true,
          anim: flow ? 'anim-iso-flow' : undefined,
        }),
      ],
      {
        anim: 'anim-iso-fade',
        delay,
      },
    )

  return [
    plate(13, 11, z),
    group(
      [
        textLine([0.8, 0.95, z], 1.3, { stroke: 'base' }),
        textLine([2.5, 0.95, z], 1.6),
        line(
          [
            [0.4, 1.6, z],
            [12.6, 1.6, z],
          ],
          { stroke: 'faint' },
        ),
        // Zoom controls in the corner of the canvas.
        ...[8.9, 9.6].map((v) =>
          rect([11.9, v, z], [0.6, 0.6], { stroke: 'faint' }),
        ),
      ],
      { anim: 'anim-iso-fade', delay: 250 },
    ),
    ...legend.map(({ owner, center }, index) =>
      group(
        [
          circle(center, 0.3, { stroke: owner === 'a' ? 'strong' : 'base' }),
          ...mark(center, owner, 0.3),
          textLine([center[0] + 0.55, 0.95, z], 1.2),
        ],
        { anim: 'anim-iso-pop', delay: 400 + index * 90 },
      ),
    ),

    // Imports, under the nodes. The change flows from the changed feature
    // to the ones importing it; the others stay still.
    link(changed, graph.right, 1300),
    link(graph.below, graph.left, 1380),
    link(changed, graph.left, 1460, true),
    link(changed, graph.front, 1540, true),

    // Nodes, back to front.
    node(graph.left, 'b', 600),
    node(graph.below, 'b', 700),
    node(changed, 'a', 800),
    node(graph.right, 'c', 900),
    node(graph.front, 'a', 1000),

    // The pull request, pinned on the feature it changes.
    group(
      [
        line([changed, [changed[0], changed[1], 3.9]], { stroke: 'base' }),
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

    // The loop: the change, its path to the importers, the owners touched.
    ping(changed, radius, loop),
    ...[graph.left, graph.front].flatMap((target) => [
      dot(changed, 0.32, {
        travel: target,
        anim: 'anim-iso-travel',
        delay: loop + 250,
      }),
      ping(target, radius, loop + 1700),
    ]),
    ...legend
      .filter(({ owner }) => owner !== 'c')
      .map(({ center }) => ping(center, 0.3, loop + 1950)),
  ]
}

/**
 * The Specs & Decisions tab of a feature: a decision record, the spec it
 * implements with its task progress, and the record it superseded, linked
 * together. In a loop, the next task of the spec gets done.
 */
const specs = (): Node[] => {
  const z = 0.3
  const card = 0.35
  const nodes: Node[] = [plate(10, 12.6, z)]

  // Header of the feature: name, path, owner.
  nodes.push(
    write([0.8, 0.9, z], 3.4, 300, 'strong'),
    group([folder([0.75, 1.45, z])], { anim: 'anim-iso-pop', delay: 400 }),
    write([1.6, 1.7, z], 2.8, 450),
    group([circle([1.05, 2.45, z], 0.2, { stroke: 'faint' })], {
      anim: 'anim-iso-pop',
      delay: 500,
    }),
    write([1.6, 2.45, z], 1.2, 550),
  )

  // Tabs, Specs & Decisions open.
  nodes.push(
    group(
      [
        rect([0.6, 3.05, z], [8.8, 0.8], { stroke: 'faint' }),
        textLine([0.95, 3.45, z], 1.0),
        textLine([2.3, 3.45, z], 0.9),
        rect([3.6, 3.15, z], [2.9, 0.6], { stroke: 'strong' }),
        textLine([3.9, 3.45, z], 1.7, { stroke: 'base' }),
        textLine([5.9, 3.45, z], 0.2, { stroke: 'base' }),
        textLine([6.9, 3.45, z], 1.0),
        textLine([8.2, 3.45, z], 0.8),
      ],
      { anim: 'anim-iso-fade', delay: 650 },
    ),
  )

  /** A record: its kind and status badges, its date and its title. */
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
    record(4.3, 2.6, 800, 'base', (top) => [
      textLine([1.0, 5.6, top], 4.4, { stroke: 'base' }),
      rect([1.0, 6.1, top], [2.7, 0.5], { stroke: 'faint' }),
      textLine([1.3, 6.35, top], 2.0),
      rect([3.9, 6.1, top], [3.3, 0.5], { stroke: 'faint' }),
      textLine([4.2, 6.35, top], 2.6),
    ]),
  )

  // The spec, its tasks half done.
  const progress = 9.25
  nodes.push(
    record(7.3, 2.75, 950, 'strong', (top) => [
      textLine([1.0, 8.6, top], 3.2, { stroke: 'base' }),
      textLine([1.0, progress, top], 3, { stroke: 'faint' }),
      textLine([4.4, progress, top], 0.5),
    ]),
    textLine([1.0, progress, z + card], 1.5, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-draw',
      delay: 1500,
    }),
    textLine([2.5, progress, z + card], 0.75, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-tick',
      delay: 2600,
    }),
  )

  // The superseded record, struck through.
  nodes.push(
    record(10.45, 1.85, 1100, 'faint', (top) => [
      textLine([1.0, 11.75, top], 3.8),
      line(
        [
          [2.75, 11.1, top],
          [4.15, 11.1, top],
        ],
        { stroke: 'faint' },
      ),
    ]),
  )

  // Links between the records, along the margin.
  nodes.push(
    group(
      [
        line(
          [
            [0.6, 5.6, z],
            [0.25, 5.6, z],
            [0.25, 8.6, z],
            [0.6, 8.6, z],
          ],
          { dashed: true, anim: 'anim-iso-flow' },
        ),
        line(
          [
            [0.6, 6.35, z],
            [0.4, 6.35, z],
            [0.4, 11.4, z],
            [0.6, 11.4, z],
          ],
          { dashed: true, stroke: 'faint' },
        ),
      ],
      { anim: 'anim-iso-fade', delay: 1350 },
    ),
  )
  return nodes
}

export const figures = { insights, dependencies, specs }
export type FigureName = keyof typeof figures
