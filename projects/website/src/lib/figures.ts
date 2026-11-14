/*
 * Isometric figures of the home page, in the spirit of Linear's: a few
 * solid objects with rounded corners, thin lines and almost no text, all
 * telling the codebase as electronics, each feature a chip. Each figure
 * builds itself when it scrolls into view, then loops a small story.
 *
 * x runs to the front right, y to the front left, z up. Times are in
 * milliseconds; the animation classes live in IsoFigure.astro.
 */
import {
  box,
  circle,
  dot,
  group,
  line,
  rect,
  slab,
  type Node,
  type Point3,
  type Shape,
  type Stroke,
} from './iso'

type Owner = 'a' | 'b' | 'c'

/** What tells owners apart, printed on a chip: a dot, a ring or a square. */
const mark = ([x, y, z]: Point3, owner: Owner, size = 0.35): Shape => {
  if (owner === 'a')
    return circle([x, y, z], size, { fill: 'solid', stroke: 'strong' })
  if (owner === 'b')
    return circle([x, y, z], size, { fill: 'none', stroke: 'strong' })
  return rect([x - size, y - size, z], [size * 2, size * 2], {
    stroke: 'strong',
  })
}

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

const ping = (center: Point3, size: number, delay: number) =>
  circle(center, size, {
    fill: 'none',
    stroke: 'strong',
    anim: 'anim-iso-ping',
    delay,
  })

/**
 * Every feature, mapped: the layers of a board stacked up, the chip on top
 * printed with a grid of features. In a loop, features are picked one by
 * one and stand out of the grid.
 */
const map = (): Node[] => {
  const size = 8
  const radius = 0.6
  const layers = 3
  const gap = 1.0
  const top = layers * gap
  const height = 0.7
  const surface = top + height
  const nodes: Node[] = []

  // Dotted guides through the corners of the stack, seen between the layers.
  const corner = (angle: number, [cx, cy]: [number, number]): Point3 => [
    cx + radius * Math.cos((angle * Math.PI) / 180),
    cy + radius * Math.sin((angle * Math.PI) / 180),
    0.15,
  ]
  nodes.push(
    group(
      [
        corner(-45, [size - radius, radius]),
        corner(45, [size - radius, size - radius]),
        corner(135, [radius, size - radius]),
      ].map(([x, y, z]) =>
        line(
          [
            [x, y, z],
            [x, y, top],
          ],
          { stroke: 'faint', dashed: true },
        ),
      ),
      { anim: 'anim-iso-fade', delay: 700 },
    ),
  )

  // The layers, bottom up, then the chip.
  for (let index = 0; index < layers; index++)
    nodes.push(
      group(
        slab([0, 0, index * gap], [size, size, 0.35], {
          radius,
          stroke: 'faint',
        }),
        {
          anim: 'anim-iso-drop',
          delay: 150 + index * 90,
        },
      ),
    )
  nodes.push(
    group(slab([0, 0, top], [size, size, height], { radius, pins: true }), {
      anim: 'anim-iso-drop',
      delay: 150 + layers * 90,
    }),
  )

  // The features, a grid on the chip; three of them get picked in turn.
  const cell = 1.6
  const pitch = 2
  const start = (size - (pitch * 2 + cell)) / 2
  const picks: Record<string, number> = {
    '0,0': 2600,
    '2,1': 5100,
    '1,2': 7600,
  }
  const cells = [0, 1, 2].flatMap((row) =>
    [0, 1, 2].map((col) => ({ row, col })),
  )
  cells.forEach(({ row, col }, index) => {
    nodes.push(
      group(
        [
          rect(
            [start + col * pitch, start + row * pitch, surface],
            [cell, cell],
            {
              stroke: 'faint',
            },
          ),
        ],
        { anim: 'anim-iso-pop', delay: 800 + index * 50 },
      ),
    )
  })
  cells.forEach(({ row, col }) => {
    const pick = picks[`${col},${row}`]
    if (pick === undefined) return
    nodes.push(
      group(
        slab(
          [start + col * pitch, start + row * pitch, surface],
          [cell, cell, 0.5],
          {
            radius: 0.25,
            stroke: 'strong',
          },
        ),
        { anim: 'anim-iso-select', delay: pick },
      ),
    )
  })
  return nodes
}

/**
 * Owned and connected: features as chips standing side by side, each marked
 * with its owner. A pull request hovers over the one it changes; in a loop,
 * the change goes out and the chips importing it light up.
 */
const owners = (): Node[] => {
  const size = 3.2
  const step = 3.9
  const loop = 2400
  const chips: { at: [number, number]; height: number; owner: Owner }[] = [
    { at: [0, 0], height: 2.6, owner: 'a' },
    { at: [step, 0], height: 1.3, owner: 'b' },
    { at: [0, step], height: 1.6, owner: 'c' },
    { at: [step, step], height: 2.0, owner: 'a' },
  ]
  const center = ({ at: [x, y], height }: (typeof chips)[number]): Point3 => [
    x + size / 2,
    y + size / 2,
    height,
  ]
  const [importer, changed, , other] = chips

  const nodes: Node[] = []
  chips.forEach((chip, index) => {
    const [x, y] = chip.at
    nodes.push(
      group(
        [
          ...slab([x, y, 0], [size, size, chip.height], { pins: true }),
          mark(center(chip), chip.owner),
        ],
        { anim: 'anim-iso-rise', delay: 150 + index * 120 },
      ),
    )
    if (chip !== changed) return

    // The pull request, hovering over the chip it changes.
    const lift = chip.height + 1.3
    const inset = 0.35
    const [cx, cy] = center(chip)
    nodes.push(
      group(
        [
          ...[
            [x + size - inset, y + inset],
            [x + size - inset, y + size - inset],
            [x + inset, y + size - inset],
          ].map(([px, py]) =>
            line(
              [
                [px, py, chip.height],
                [px, py, lift],
              ],
              { stroke: 'faint', dashed: true },
            ),
          ),
          ...slab(
            [x + inset, y + inset, lift],
            [size - inset * 2, size - inset * 2, 0.2],
            {
              radius: 0.35,
              stroke: 'strong',
            },
          ),
          // Branch glyph.
          line(
            [
              [cx - 0.6, cy + 0.3, lift + 0.2],
              [cx + 0.6, cy + 0.3, lift + 0.2],
            ],
            { stroke: 'base' },
          ),
          line(
            [
              [cx - 0.2, cy + 0.3, lift + 0.2],
              [cx + 0.2, cy - 0.4, lift + 0.2],
              [cx + 0.6, cy - 0.4, lift + 0.2],
            ],
            { stroke: 'base' },
          ),
          dot([cx - 0.6, cy + 0.3, lift + 0.2], 0.13, { stroke: 'base' }),
          dot([cx + 0.6, cy + 0.3, lift + 0.2], 0.13, { stroke: 'base' }),
          dot([cx + 0.6, cy - 0.4, lift + 0.2], 0.13, { stroke: 'base' }),
        ],
        { anim: 'anim-iso-drop', delay: 900 },
      ),
    )
  })

  // The loop: the change, then the chips importing it.
  nodes.push(
    ping(center(changed), size / 2, loop),
    ping(center(importer), size / 2, loop + 900),
    ping(center(other), size / 2, loop + 900),
  )
  return nodes
}

/**
 * Intent kept in the repository: a chip and its datasheets standing behind
 * it, the spec in front with its task progress, older decision records
 * behind, the superseded one at the back. In a loop, the next task of the
 * spec gets done.
 */
const specs = (): Node[] => {
  const width = 6.4
  const thickness = 0.12
  const sheets: { height: number; stroke: Stroke; dashed?: boolean }[] = [
    { height: 3.4, stroke: 'faint', dashed: true },
    { height: 2.9, stroke: 'base' },
    { height: 2.4, stroke: 'base' },
    { height: 1.9, stroke: 'strong' },
  ]
  const spacing = 0.7
  const front = (sheets.length - 1) * spacing + thickness
  const nodes: Node[] = []

  // The trace from the chip to its datasheets, under everything else.
  nodes.push(
    group(
      [
        line(
          [
            [6.9, 2.7, 0],
            [3.2, 2.7, 0],
            [3.2, front, 0],
          ],
          { dashed: true, anim: 'anim-iso-flow' },
        ),
      ],
      { anim: 'anim-iso-fade', delay: 1200 },
    ),
  )

  sheets.forEach(({ height, stroke, dashed }, index) =>
    nodes.push(
      group(
        box([0, index * spacing, 0], [width, thickness, height], {
          stroke,
          dashed,
        }),
        {
          anim: 'anim-iso-drop',
          delay: 150 + index * 120,
        },
      ),
    ),
  )

  // On the face of the spec: its title, then its tasks, half done.
  const progress = 0.5
  nodes.push(
    group(
      [
        textLine([0.6, front, 1.45], 3.2, { stroke: 'base' }),
        textLine([0.6, front, 1.05], 4.2),
        textLine([0.6, front, progress], 5.2),
      ],
      { anim: 'anim-iso-fade', delay: 750 },
    ),
    textLine([0.6, front, progress], 2.6, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-draw',
      delay: 1000,
    }),
    textLine([3.2, front, progress], 1.2, {
      stroke: 'strong',
      draw: true,
      anim: 'anim-iso-tick',
      delay: 2600,
    }),
  )

  // The chip the datasheets describe.
  nodes.push(
    group(
      [
        ...slab([7.0, 1.0, 0], [2.6, 2.6, 0.7], { pins: true, top: 'strong' }),
        mark([8.3, 2.3, 0.7], 'a', 0.3),
      ],
      { anim: 'anim-iso-drop', delay: 650 },
    ),
  )
  return nodes
}

export const figures = { map, owners, specs }
export type FigureName = keyof typeof figures
