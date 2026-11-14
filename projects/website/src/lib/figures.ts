/*
 * Isometric figures of the home page. Each one builds itself when it scrolls
 * into view (plates rise, blocks drop, lines draw), then loops a small story.
 * Times are in milliseconds; the animation classes live in IsoFigure.astro.
 */
import {
  box,
  circle,
  dot,
  group,
  line,
  rect,
  type Node,
  type Point3,
  type Shape,
} from './iso'

/** A short horizontal stroke standing for a line of text, on the plane z. */
const textLine = (
  [x, y, z]: Point3,
  length: number,
  options: Parameters<typeof line>[1] = {},
): Shape =>
  line(
    [
      [x, y, z],
      [x + length, y, z],
    ],
    { stroke: 'faint', ...options },
  )

/** Every feature of a folder as a block on a plate, its height standing for its size. */
const map = (): Node[] => {
  const base = 0.6
  const blocks: {
    at: [number, number]
    size: Point3
    strong?: boolean
    /** Time this block starts lifting, in the loop that picks features one by one. */
    lift?: number
    details?: (top: number) => Node[]
  }[] = [
    {
      at: [0.6, 0.6],
      size: [5, 6.4, 3.4],
      strong: true,
      lift: 2600,
      // Hatching, as if the feature were selected.
      details: (top) =>
        [1.6, 2.6, 3.6, 4.6].map((offset, index) =>
          line(
            [
              [0.6 + offset, 1.4, top],
              [0.6 + offset, 6.2, top],
            ],
            {
              stroke: 'faint',
              draw: true,
              anim: 'anim-iso-draw',
              delay: 1300 + index * 90,
            },
          ),
        ),
    },
    {
      at: [6.2, 0.6],
      size: [5.2, 3, 1.8],
      // Files of the feature.
      details: (top) =>
        [7.1, 8, 8.9, 9.8].map((x, index) =>
          circle([x, 2.1, top], 0.22, {
            stroke: 'faint',
            anim: 'anim-iso-pop',
            delay: 1350 + index * 70,
          }),
        ),
    },
    {
      at: [0.6, 7.6],
      size: [3.4, 3.8, 1.4],
      details: (top) => [
        textLine([1.2, 8.6, top], 2, {
          draw: true,
          anim: 'anim-iso-draw',
          delay: 1400,
        }),
        textLine([1.2, 9.5, top], 1.4, {
          draw: true,
          anim: 'anim-iso-draw',
          delay: 1480,
        }),
      ],
    },
    {
      at: [6.2, 4.2],
      size: [2.4, 2.8, 2.6],
      lift: 5100,
      // A nested feature on top of its parent.
      details: (top) => [
        group(box([6.6, 4.6, top], [1.2, 1.3, 0.8]), {
          anim: 'anim-iso-drop',
          delay: 1450,
        }),
      ],
    },
    {
      at: [4.6, 7.6],
      size: [3, 3.8, 2.2],
      lift: 7600,
      details: (top) => [
        rect([5.1, 8.2, top], [0.9, 0.9], {
          stroke: 'faint',
          anim: 'anim-iso-pop',
          delay: 1500,
        }),
        textLine([6.3, 8.65, top], 0.9, {
          draw: true,
          anim: 'anim-iso-draw',
          delay: 1550,
        }),
        textLine([5.1, 9.8, top], 2, {
          draw: true,
          anim: 'anim-iso-draw',
          delay: 1600,
        }),
      ],
    },
    {
      at: [9, 4.2],
      size: [2.4, 2.8, 1.2],
      details: (top) => [
        circle([10.2, 5.6, top], 0.3, {
          stroke: 'faint',
          anim: 'anim-iso-pop',
          delay: 1550,
        }),
      ],
    },
    {
      at: [8.2, 7.6],
      size: [3.2, 3.8, 0.7],
      details: (top) =>
        [0, 1, 2].map((index) =>
          rect([8.7 + index * 0.85, 8.3, top], [0.6, 2.4], {
            stroke: 'faint',
            anim: 'anim-iso-pop',
            delay: 1600 + index * 60,
          }),
        ),
    },
  ]

  // Ruler ticks on the front faces of the plate.
  const ticks = Array.from({ length: 11 }, (_, index) => index + 1).flatMap(
    (n) => {
      const depth = n % 3 === 0 ? 0.36 : 0.18
      return [
        line([
          [n, 12, base],
          [n, 12, base - depth],
        ]),
        line([
          [12, n, base],
          [12, n, base - depth],
        ]),
      ]
    },
  )

  return [
    group(box([0, 0, 0], [12, 12, base], { stroke: 'faint' }), {
      anim: 'anim-iso-rise',
    }),
    group(
      [
        rect([0.25, 0.25, base], [11.5, 11.5], {
          stroke: 'faint',
          dashed: true,
        }),
        ...ticks.map((tick) => ({ ...tick, stroke: 'faint' as const })),
      ],
      { anim: 'anim-iso-fade', delay: 250 },
    ),
    ...blocks.map(({ at: [x, y], size, strong, lift, details }, index) => {
      const children = [
        ...box([x, y, base], size, { top: strong ? 'strong' : 'base' }),
        ...(details?.(base + size[2]) ?? []),
      ]
      return group(
        lift === undefined
          ? children
          : [group(children, { anim: 'anim-iso-lift', delay: lift })],
        {
          anim: 'anim-iso-drop',
          delay: 350 + index * 110,
        },
      )
    }),
  ]
}

/** Features grouped on one plate per team, with the imports between them. */
const owners = (): Node[] => {
  const plate = 0.4
  const top = (
    x: number,
    y: number,
    w: number,
    d: number,
    h: number,
  ): Point3 => [x + w / 2, y + d / 2, plate + h]
  const a = top(0.9, 0.9, 2.4, 2.4, 2.6)
  const b = top(3.8, 3, 1.8, 1.8, 1.4)
  const c = top(9.4, 1.4, 2.6, 2.6, 3.2)
  const d = top(3.2, 9.4, 2.2, 2.2, 1.8)
  const e = top(5.9, 10.6, 1.1, 1.4, 1)

  // The loop: c changes, the change travels up its imports to the features
  // that use it, then the teams that own them light up.
  const loop = 2400
  const owner = (center: Point3, at: number, notified?: number) => [
    circle(center, 0.4, { stroke: 'strong', anim: 'anim-iso-pop', delay: at }),
    ...(notified === undefined
      ? []
      : [
          circle(center, 0.4, {
            fill: 'none',
            stroke: 'strong',
            anim: 'anim-iso-ping',
            delay: notified,
          }),
        ]),
  ]
  const feature = (
    at: Point3,
    size: Point3,
    delay: number,
    options: Parameters<typeof box>[2] = {},
    extra: Node[] = [],
  ) =>
    group([...box(at, size, options), ...extra], {
      anim: 'anim-iso-drop',
      delay,
    })

  return [
    // Team plates, back to front, each with its owner and a name.
    group(
      [
        ...box([0, 0, 0], [6.4, 5.8, plate]),
        textLine([0.6, 5.25, plate], 1.8, { stroke: 'base' }),
        textLine([2.7, 5.25, plate], 0.8),
      ],
      { anim: 'anim-iso-rise' },
    ),
    ...owner([5.4, 0.9, plate], 450, loop + 1700),
    group(
      [
        ...box([8.4, 0.4, 0], [4.6, 4.6, plate]),
        textLine([9, 4.5, plate], 1.4, { stroke: 'base' }),
      ],
      {
        anim: 'anim-iso-rise',
        delay: 120,
      },
    ),
    ...owner([12.1, 1.2, plate], 550),
    group(
      [
        ...box([2.4, 8.6, 0], [5, 4, plate]),
        textLine([2.9, 12.1, plate], 1.6, { stroke: 'base' }),
        textLine([4.7, 12.1, plate], 0.6),
      ],
      { anim: 'anim-iso-rise', delay: 240 },
    ),
    ...owner([6.6, 9.4, plate], 650, loop + 1900),

    // Features.
    feature([0.9, 0.9, plate], [2.4, 2.4, 2.6], 700, {}, [
      ...box([1.25, 1.25, a[2]], [0.8, 0.8, 0.45]),
      textLine([2.35, 1.5, a[2]], 0.6),
    ]),
    feature([3.8, 3, plate], [1.8, 1.8, 1.4], 800),
    feature([9.4, 1.4, plate], [2.6, 2.6, 3.2], 900, { top: 'strong' }, [
      textLine([9.9, 1.9, c[2]], 1.2),
      textLine([9.9, 2.5, c[2]], 0.7),
    ]),
    circle(c, 0.35, {
      fill: 'none',
      stroke: 'strong',
      anim: 'anim-iso-ping',
      delay: loop,
    }),
    feature([3.2, 9.4, plate], [2.2, 2.2, 1.8], 1000),
    feature([5.9, 10.6, plate], [1.1, 1.4, 1], 1080),

    // Imports between features, drawn over everything like a wire. The ones
    // reaching c flow from it, toward the features that import it.
    group([line([a, d], { dashed: true })], {
      anim: 'anim-iso-fade',
      delay: 1400,
    }),
    ...[a, b, e].map((target, index) =>
      group([line([c, target], { dashed: true, anim: 'anim-iso-flow' })], {
        anim: 'anim-iso-fade',
        delay: 1400 + index * 120,
      }),
    ),
    circle(c, 0.35, { stroke: 'strong', anim: 'anim-iso-pop', delay: 1300 }),
    ...[a, b, e].map((target) =>
      dot(c, 0.32, {
        travel: target,
        anim: 'anim-iso-travel',
        delay: loop + 250,
      }),
    ),
  ]
}

/** A spec on top of older records: text, then a task list half done. */
const specs = (): Node[] => {
  const size: Point3 = [7, 9, 0.25]
  const sheets = [0, 1, 2].map((index): Point3 => [index * 0.9, 0, index * 1.9])
  const nodes: Node[] = []
  sheets.forEach((at, index) => {
    const next = sheets[index + 1]
    const last = !next
    nodes.push(
      group(
        [
          ...box(at, size, { top: last ? 'strong' : 'base' }),
          // Index tabs on the older records, like a binder.
          ...(last
            ? []
            : box(
                [at[0] + size[0], at[1] + 1.2 + index * 1.8, at[2]],
                [0.5, 1.2, size[2]],
                { stroke: 'faint' },
              )),
        ],
        { anim: 'anim-iso-drop', delay: index * 160 },
      ),
    )
    if (!next) return
    // Dotted guides between the front corners of two sheets.
    nodes.push(
      group(
        (
          [
            [size[0], 0],
            [size[0], size[1]],
            [0, size[1]],
          ] as const
        ).map(([dx, dy]) =>
          line(
            [
              [at[0] + dx, at[1] + dy, at[2] + size[2]],
              [next[0] + dx, next[1] + dy, next[2]],
            ],
            { dashed: true, stroke: 'faint' },
          ),
        ),
        { anim: 'anim-iso-fade', delay: 600 + index * 120 },
      ),
    )
  })

  const [x, y, z0] = sheets[sheets.length - 1]
  const z = z0 + size[2]
  const write = (
    row: number,
    from: number,
    length: number,
    delay: number,
    stroke: Shape['stroke'] = 'faint',
  ) =>
    textLine([x + from, y + row, z], length, {
      stroke,
      draw: true,
      anim: 'anim-iso-draw',
      delay,
    })
  const check = (row: number, options: { anim: string; delay: number }) =>
    line(
      [
        [x + 1.15, y + row - 0.05, z],
        [x + 1.45, y + row + 0.25, z],
        [x + 1.75, y + row - 0.55, z],
      ],
      { stroke: 'strong', draw: true, ...options },
    )

  // Title, status badge and summary, above the front matter rule.
  nodes.push(
    write(1.3, 1, 3.4, 800, 'strong'),
    group(
      [
        rect([x + 4.9, y + 0.85, z], [1.3, 0.9], { stroke: 'base' }),
        circle([x + 5.3, y + 1.3, z], 0.16, { stroke: 'strong' }),
      ],
      { anim: 'anim-iso-pop', delay: 1000 },
    ),
    write(2.4, 1, 5, 950),
    write(3.2, 1, 4.2, 1050),
    group([textLine([x + 1, y + 3.95, z], 5, { dashed: true })], {
      anim: 'anim-iso-fade',
      delay: 1150,
    }),
  )

  // Tasks: two done, the third one ticked off in a loop.
  const tasks = [
    { row: 4.8, length: 3.3, done: true },
    { row: 6.1, length: 2.5, done: true },
    { row: 7.4, length: 2.9, done: false },
  ]
  tasks.forEach(({ row, length, done }, index) => {
    const delay = 1250 + index * 160
    nodes.push(
      rect([x + 1, y + row - 0.4, z], [0.8, 0.8], {
        stroke: done ? 'strong' : 'base',
        anim: 'anim-iso-pop',
        delay,
      }),
      write(row, 2.3, length, delay + 60, done ? 'base' : 'faint'),
      done
        ? check(row, { anim: 'anim-iso-draw', delay: 1800 + index * 200 })
        : check(row, { anim: 'anim-iso-tick', delay: 2800 }),
    )
  })

  // Progress of the tasks, the last third filling with the loop.
  nodes.push(
    group([textLine([x + 1, y + 8.4, z], 5)], {
      anim: 'anim-iso-fade',
      delay: 1600,
    }),
    textLine([x + 1, y + 8.4, z], 3.33, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-draw',
      delay: 1900,
    }),
    textLine([x + 4.33, y + 8.4, z], 1.67, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-tick',
      delay: 2900,
    }),
  )
  return nodes
}

export const figures = { map, owners, specs }
export type FigureName = keyof typeof figures
