/*
 * Isometric figures of the home page. Each one compares an idea of Features
 * to the real world: features as buildings on a city map, teams as plots of
 * land wired to each other, specs as the blueprint kept next to a building.
 * Each figure builds itself when it scrolls into view (plates rise, blocks
 * drop, lines draw), then loops a small story.
 *
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
  type Fill,
  type Node,
  type Point3,
  type Shape,
  type Stroke,
} from './iso'

type LineOptions = Parameters<typeof line>[1]

/** Any polygon in space, filled like a face. */
const face = (
  points: Point3[],
  fill: Fill = 'side',
  stroke: Stroke = 'base',
  dashed = false,
): Shape => ({ kind: 'polygon', fill, stroke, dashed, points })

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

/** A rectangle standing on the front face (y = constant) of a building. */
const opening = (
  x: number,
  y: number,
  z: number,
  [w, h]: [number, number],
  stroke: Stroke = 'faint',
): Shape =>
  face(
    [
      [x, y, z],
      [x + w, y, z],
      [x + w, y, z + h],
      [x, y, z + h],
    ],
    'none',
    stroke,
  )

/** A rectangle standing on the right face (x = constant) of a building. */
const sideOpening = (
  x: number,
  y: number,
  z: number,
  [d, h]: [number, number],
  stroke: Stroke = 'faint',
): Shape =>
  face(
    [
      [x, y, z],
      [x, y + d, z],
      [x, y + d, z + h],
      [x, y, z + h],
    ],
    'none',
    stroke,
  )

/** Rows of windows on both visible faces of a building. */
const windows = (
  [x, y, z]: Point3,
  [w, d, h]: Point3,
  floors: number,
): Shape[] => {
  const shapes: Shape[] = []
  const story = h / floors
  for (let floor = 0; floor < floors; floor++) {
    const bottom = z + floor * story + story * 0.3
    const size = story * 0.4
    for (let u = 0.4; u + 0.5 <= w - 0.2; u += 0.8)
      shapes.push(opening(x + u, y + d, bottom, [0.4, size]))
    for (let v = 0.4; v + 0.5 <= d - 0.2; v += 0.8)
      shapes.push(sideOpening(x + w, y + v, bottom, [0.4, size]))
  }
  return shapes
}

/** A house: walls, a gable roof with its ridge along x, and a door. */
const house = (
  [x, y, z]: Point3,
  [w, d, h]: Point3,
  { rise = 0.9, roof = 'base' as Stroke } = {},
): Shape[] => {
  const top = z + h
  const ridge = top + rise
  return [
    ...box([x, y, z], [w, d, h]),
    face(
      [
        [x, y, top],
        [x + w, y, top],
        [x + w, y + d / 2, ridge],
        [x, y + d / 2, ridge],
      ],
      'top',
      roof,
    ),
    face(
      [
        [x, y + d, top],
        [x + w, y + d, top],
        [x + w, y + d / 2, ridge],
        [x, y + d / 2, ridge],
      ],
      'top',
      roof,
    ),
    face([
      [x + w, y, top],
      [x + w, y + d, top],
      [x + w, y + d / 2, ridge],
    ]),
    opening(x + w * 0.3, y + d, z, [w * 0.22, h * 0.55], 'base'),
    sideOpening(x + w, y + d * 0.35, z + h * 0.35, [d * 0.3, h * 0.35]),
  ]
}

/** The middle of the ridge of a house, where its cables are tied. */
const ridgeOf = ([x, y, z]: Point3, [w, d, h]: Point3, rise = 0.9): Point3 => [
  x + w / 2,
  y + d / 2,
  z + h + rise,
]

/** A flag on its pole, the flag waving to the right. */
const flag = ([x, y, z]: Point3, height: number): Shape[] => [
  line(
    [
      [x, y, z],
      [x, y, z + height],
    ],
    { stroke: 'strong' },
  ),
  face(
    [
      [x, y, z + height],
      [x + 1.5, y, z + height - 0.45],
      [x, y, z + height - 0.9],
    ],
    'side',
    'strong',
  ),
]

/** A tree: a trunk and a round crown seen as a short cylinder. */
const tree = ([x, y, z]: Point3, radius: number): Node[] => [
  line(
    [
      [x, y, z],
      [x, y, z + 0.5],
    ],
    { stroke: 'faint' },
  ),
  ...disc([x, y, z + 0.5], radius, radius * 0.9),
]

/**
 * A map pin standing up and facing the viewer, its tip at (x, y, z). Drawn in
 * the vertical plane that runs across the screen.
 */
const pin = ([x, y, z]: Point3, radius: number): Shape[] => {
  // Moving u along the screen is moving (u, -u) on the ground, scaled down
  // so the pin stays round once projected.
  const scale = 1 / (2 * Math.cos(Math.PI / 6))
  const at = (u: number, v: number): Point3 => [
    x + u * scale,
    y - u * scale,
    z + v,
  ]
  const head = radius * 2.1
  const outline: Point3[] = []
  const from = -Math.PI / 5
  const to = Math.PI + Math.PI / 5
  for (let index = 0; index <= 20; index++) {
    const angle = from + ((to - from) * index) / 20
    outline.push(at(radius * Math.cos(angle), head + radius * Math.sin(angle)))
  }
  outline.reverse()
  const hole: Point3[] = Array.from({ length: 13 }, (_, index) => {
    const angle = (Math.PI * 2 * index) / 12
    return at(
      radius * 0.4 * Math.cos(angle),
      head + radius * 0.4 * Math.sin(angle),
    )
  })
  return [
    face([...outline, at(0, 0)], 'side', 'strong'),
    face(hole, 'none', 'strong'),
  ]
}

/**
 * Every feature, mapped: a city map where each feature is a building, its
 * height standing for its size. A pin bobs over the feature being looked at.
 */
const map = (): Node[] => {
  const z = 0.25
  const nodes: Node[] = [
    group(box([0, 0, 0], [12, 10, z], { stroke: 'base' }), {
      anim: 'anim-iso-rise',
    }),
    // Folds of the paper map, and the roads between the blocks.
    group(
      [
        ...[4, 8].map((x) =>
          line(
            [
              [x, 0, z],
              [x, 10, z],
            ],
            { stroke: 'faint', dashed: true },
          ),
        ),
        ...[4.6, 5.4].map((y) =>
          line(
            [
              [0, y, z],
              [12, y, z],
            ],
            { stroke: 'faint' },
          ),
        ),
        ...[7.6, 8.4].map((x) =>
          line(
            [
              [x, 0, z],
              [x, 10, z],
            ],
            { stroke: 'faint' },
          ),
        ),
        // Dashes down the middle of the main road.
        ...[0.6, 2.2, 3.8, 5.4, 9.2, 10.8].map((x) => textLine([x, 5, z], 0.8)),
      ],
      { anim: 'anim-iso-fade', delay: 250 },
    ),
  ]

  // Buildings, back to front.
  const buildings: { at: Point3; size: Point3; floors: number }[] = [
    { at: [1, 0.9, z], size: [2.8, 2.8, 3.2], floors: 4 },
    { at: [4.4, 0.9, z], size: [2.6, 1.4, 1.2], floors: 1 },
    { at: [9, 0.9, z], size: [2.2, 3, 2], floors: 2 },
    { at: [4.4, 2.7, z], size: [1.4, 1.3, 0.8], floors: 1 },
    { at: [4.6, 6.2, z], size: [2.2, 2.8, 1.6], floors: 2 },
  ]
  buildings.forEach(({ at, size, floors }, index) => {
    nodes.push(
      group([...box(at, size), ...windows(at, size, floors)], {
        anim: 'anim-iso-drop',
        delay: 450 + index * 110,
      }),
    )
  })

  // A park in the front-left block.
  ;[
    [1.4, 6.6, 0.55],
    [2.8, 6.4, 0.45],
    [1.6, 8.2, 0.5],
    [3.2, 7.8, 0.6],
  ].forEach(([x, y, radius], index) => {
    nodes.push(
      group(tree([x, y, z], radius), {
        anim: 'anim-iso-pop',
        delay: 900 + index * 70,
      }),
    )
  })

  // The feature the pin points at, then the pin bobbing over it.
  const target: Point3 = [9.2, 6.4, z]
  const targetSize: Point3 = [2, 2.6, 1]
  const roof: Point3 = [10.2, 7.7, z + targetSize[2]]
  nodes.push(
    group(
      [
        ...box(target, targetSize, { top: 'strong' }),
        ...windows(target, targetSize, 1),
      ],
      { anim: 'anim-iso-drop', delay: 1000 },
    ),
    circle(roof, 0.5, {
      fill: 'none',
      stroke: 'strong',
      anim: 'anim-iso-ping',
      delay: 2400,
    }),
    group(
      [
        group(pin([roof[0], roof[1], roof[2] + 0.4], 0.9), {
          anim: 'anim-iso-bob',
        }),
      ],
      { anim: 'anim-iso-drop', delay: 1300 },
    ),
  )
  return nodes
}

/**
 * Owned and connected: plots of land, one per team, each with the team's
 * flag and the houses it owns, wired to each other like a power grid. In a
 * loop, one house changes: the change runs down the wires to the houses
 * connected to it, and the flags of their owners light up.
 */
const owners = (): Node[] => {
  const plate = 0.35
  const loop = 2600

  const plots = [
    { at: [0, 0], size: [6.4, 5.6], flag: [5.7, 0.7] },
    { at: [8.6, 0.4], size: [4.6, 4.6], flag: [12.6, 1.0] },
    { at: [2.2, 8.4], size: [5.4, 4.4], flag: [7.0, 9.0] },
  ] as const
  const houses: { at: Point3; size: Point3; roof?: Stroke }[] = [
    { at: [0.8, 0.8, plate], size: [2.2, 2.2, 1.4] },
    { at: [3.6, 2.8, plate], size: [2, 2, 1.1] },
    { at: [9.4, 1.4, plate], size: [2.6, 2.6, 1.7], roof: 'strong' },
    { at: [3.4, 9.2, plate], size: [2.4, 2.4, 1.2] },
  ]
  const ridges = houses.map(({ at, size }) => ridgeOf(at, size))
  const [first, importer, changed, other] = ridges

  const ping = (center: Point3, size: number, delay: number) =>
    circle(center, size, {
      fill: 'none',
      stroke: 'strong',
      anim: 'anim-iso-ping',
      delay,
    })
  const flagTop = (index: number): Point3 => {
    const [x, y] = plots[index].flag
    return [x, y, plate + 2.6]
  }

  const nodes: Node[] = []

  // Plots, their fences, and their flags.
  plots.forEach(({ at: [x, y], size: [w, d], flag: [fx, fy] }, index) => {
    nodes.push(
      group(
        [
          ...box([x, y, 0], [w, d, plate]),
          rect([x + 0.25, y + 0.25, plate], [w - 0.5, d - 0.5], {
            stroke: 'faint',
            dashed: true,
          }),
        ],
        { anim: 'anim-iso-rise', delay: index * 120 },
      ),
      group(flag([fx, fy, plate], 2.6), {
        anim: 'anim-iso-drop',
        delay: 1200 + index * 120,
      }),
    )
  })

  // Houses, back to front.
  houses.forEach(({ at, size, roof }, index) => {
    nodes.push(
      group(house(at, size, { roof }), {
        anim: 'anim-iso-drop',
        delay: 450 + index * 110,
      }),
    )
  })

  // Wires between the houses, over the roofs. The change runs out of the
  // changed house to the ones wired to it.
  nodes.push(
    group([line([first, importer], { stroke: 'base' })], {
      anim: 'anim-iso-fade',
      delay: 1000,
    }),
    ...[importer, other].map((target, index) =>
      group(
        [line([changed, target], { dashed: true, anim: 'anim-iso-flow' })],
        {
          anim: 'anim-iso-fade',
          delay: 1100 + index * 100,
        },
      ),
    ),
    ping(changed, 0.6, loop),
    ...[importer, other].flatMap((target) => [
      dot(changed, 0.32, {
        travel: target,
        anim: 'anim-iso-travel',
        delay: loop + 250,
      }),
      ping(target, 0.6, loop + 1700),
    ]),
    ...[0, 2].map((index) => ping(flagTop(index), 0.5, loop + 1950)),
  )
  return nodes
}

/**
 * Intent kept in the repository: a building going up, with its blueprint
 * and the site's checklist kept right next to it. The top floor is still
 * planned; in a loop, the next task gets ticked off.
 */
const specs = (): Node[] => {
  const z = 0.3
  const nodes: Node[] = [
    group(box([0, 0, 0], [13, 10, z], { stroke: 'base' }), {
      anim: 'anim-iso-rise',
    }),
  ]

  // The building: two floors built, the third one planned.
  const at: Point3 = [7.6, 0.9, z]
  const story = 1.5
  const footprint: [number, number] = [4.2, 4]
  ;[0, 1].forEach((floor) => {
    const base: Point3 = [at[0], at[1], z + floor * story]
    const size: Point3 = [...footprint, story]
    nodes.push(
      group([...box(base, size), ...windows(base, size, 1)], {
        anim: 'anim-iso-drop',
        delay: 400 + floor * 150,
      }),
    )
  })
  nodes.push(
    group(
      box([at[0], at[1], z + 2 * story], [...footprint, story], {
        stroke: 'faint',
        dashed: true,
      }),
      { anim: 'anim-iso-fade', delay: 800 },
    ),
  )

  // The blueprint, lying on the ground: the floor plan of the building.
  const bp: Point3 = [0.8, 4.4, z]
  const bpz = z + 0.1
  const [bx, by] = bp
  nodes.push(
    group(
      [
        ...box(bp, [5.8, 4.8, 0.1], { top: 'strong' }),
        // Outer walls, inner walls and a door.
        rect([bx + 0.7, by + 0.7, bpz], [3.4, 3.2], { stroke: 'strong' }),
        line(
          [
            [bx + 2.3, by + 0.7, bpz],
            [bx + 2.3, by + 2.4, bpz],
          ],
          { stroke: 'base' },
        ),
        line(
          [
            [bx + 0.7, by + 2.4, bpz],
            [bx + 1.6, by + 2.4, bpz],
          ],
          { stroke: 'base' },
        ),
        line(
          [
            [bx + 2.3, by + 2.4, bpz],
            [bx + 4.1, by + 2.4, bpz],
          ],
          { stroke: 'base' },
        ),
        // Dimension line along the front wall.
        line(
          [
            [bx + 0.7, by + 4.3, bpz],
            [bx + 4.1, by + 4.3, bpz],
          ],
          { stroke: 'faint' },
        ),
        ...[0.7, 4.1].map((dx) =>
          line(
            [
              [bx + dx, by + 4.1, bpz],
              [bx + dx, by + 4.5, bpz],
            ],
            { stroke: 'faint' },
          ),
        ),
        // Title block in the corner.
        rect([bx + 4.5, by + 3.5, bpz], [1, 1], { stroke: 'base' }),
        textLine([bx + 4.6, by + 3.8, bpz], 0.8),
      ],
      { anim: 'anim-iso-drop', delay: 950 },
    ),
  )

  // Materials waiting for the next floor: bricks on a pallet.
  const pallet: Point3 = [9, 6.6, z]
  nodes.push(
    group(
      [
        ...box(pallet, [2.4, 1.8, 0.25]),
        ...[
          [0.2, 0.2, 0],
          [1.2, 0.2, 0],
          [0.2, 1, 0],
          [1.2, 1, 0],
          [0.7, 0.6, 0.5],
        ].flatMap(([dx, dy, dz]) =>
          box([pallet[0] + dx, pallet[1] + dy, z + 0.25 + dz], [1, 0.6, 0.5]),
        ),
      ],
      { anim: 'anim-iso-drop', delay: 1050 },
    ),
  )

  // The checklist on its clipboard: two tasks done, the third ticked off
  // in a loop.
  const cb: Point3 = [1, 0.8, z]
  const cbz = z + 0.15
  const [cx, cy] = cb
  nodes.push(
    group(
      [
        ...box([cx + 1, cy - 0.3, z], [1.4, 0.6, 0.35], { stroke: 'strong' }),
        ...box(cb, [3.4, 3.2, 0.15], { stroke: 'base' }),
        ...box([cx + 1, cy - 0.1, cbz], [1.4, 0.4, 0.15], { stroke: 'strong' }),
      ],
      { anim: 'anim-iso-drop', delay: 1150 },
    ),
  )
  const tasks = [1.1, 1.9, 2.7]
  tasks.forEach((row, index) => {
    const done = index < tasks.length - 1
    const delay = 1450 + index * 150
    nodes.push(
      rect([cx + 0.4, cy + row - 0.3, cbz], [0.6, 0.6], {
        stroke: done ? 'strong' : 'base',
        anim: 'anim-iso-pop',
        delay,
      }),
      textLine([cx + 1.3, cy + row, cbz], 1.7 - index * 0.3, {
        stroke: 'base',
        draw: true,
        anim: 'anim-iso-draw',
        delay: delay + 60,
      }),
      line(
        [
          [cx + 0.5, cy + row - 0.05, cbz],
          [cx + 0.72, cy + row + 0.18, cbz],
          [cx + 0.98, cy + row - 0.45, cbz],
        ],
        {
          stroke: 'strong',
          draw: true,
          anim: done ? 'anim-iso-draw' : 'anim-iso-tick',
          delay: done ? 1900 + index * 200 : 2800,
        },
      ),
    )
  })
  return nodes
}

export const figures = { map, owners, specs }
export type FigureName = keyof typeof figures
