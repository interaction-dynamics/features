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
 * Intent kept in the repository: every feature is a chip in its folder,
 * its own datasheets standing right behind it and wired to it: decision
 * records, the spec in front, a superseded record dashed at the back. In a
 * loop, the next task of the middle feature's spec gets done.
 */
const specs = (): Node[] => {
  const width = 2.2
  const thickness = 0.1
  const spacing = 0.55
  const pitch = 3.2
  const chipAt = 2.0
  type Sheet = { height: number; stroke: Stroke; dashed?: boolean }
  const features: { owner: Owner; sheets: Sheet[]; focus?: boolean }[] = [
    {
      owner: 'b',
      sheets: [
        { height: 2.2, stroke: 'base' },
        { height: 1.7, stroke: 'base' },
      ],
    },
    {
      owner: 'a',
      focus: true,
      sheets: [
        { height: 2.8, stroke: 'faint', dashed: true },
        { height: 2.3, stroke: 'base' },
        { height: 1.8, stroke: 'strong' },
      ],
    },
    {
      owner: 'c',
      sheets: [
        { height: 2.0, stroke: 'base' },
        { height: 1.6, stroke: 'base' },
      ],
    },
  ]
  const nodes: Node[] = []

  features.forEach(({ owner, sheets, focus }, column) => {
    const x = column * pitch
    const delay = 150 + column * 250
    const back = chipAt - 0.45 - (sheets.length - 1) * spacing - thickness
    const front = back + (sheets.length - 1) * spacing + thickness

    // The folder of the feature, around its chip and its datasheets.
    nodes.push(
      group(
        [
          rect(
            [x - 0.35, back - 0.35, 0],
            [width + 0.7, chipAt + width + 0.35 - back + 0.35],
            {
              stroke: 'faint',
              dashed: true,
            },
          ),
        ],
        { anim: 'anim-iso-fade', delay },
      ),
      // The trace from the chip to its datasheets.
      group(
        [
          line(
            [
              [x + width / 2, chipAt, 0],
              [x + width / 2, front, 0],
            ],
            { dashed: true, anim: 'anim-iso-flow' },
          ),
        ],
        { anim: 'anim-iso-fade', delay: delay + 700 },
      ),
    )

    // The datasheets, back to front.
    sheets.forEach(({ height, stroke, dashed }, index) =>
      nodes.push(
        group(
          box([x, back + index * spacing, 0], [width, thickness, height], {
            stroke,
            dashed,
          }),
          {
            anim: 'anim-iso-drop',
            delay: delay + 100 + index * 100,
          },
        ),
      ),
    )

    // On the face of the spec: its title, then its tasks, half done.
    if (focus) {
      const progress = 0.45
      nodes.push(
        group(
          [
            textLine([x + 0.3, front, 1.3], 1.3, { stroke: 'base' }),
            textLine([x + 0.3, front, 0.95], 1.6),
            textLine([x + 0.3, front, progress], 1.6),
          ],
          { anim: 'anim-iso-fade', delay: delay + 500 },
        ),
        textLine([x + 0.3, front, progress], 0.8, {
          stroke: 'strong',
          draw: true,
          anim: 'anim-iso-draw',
          delay: delay + 700,
        }),
        textLine([x + 1.1, front, progress], 0.4, {
          stroke: 'strong',
          draw: true,
          anim: 'anim-iso-tick',
          delay: 2600,
        }),
      )
    }

    // The chip, in front of its datasheets.
    nodes.push(
      group(
        [
          ...slab([x, chipAt, 0], [width, width, 0.6], {
            pins: true,
            top: focus ? 'strong' : 'base',
          }),
          mark([x + width / 2, chipAt + width / 2, 0.6], owner, 0.28),
        ],
        { anim: 'anim-iso-drop', delay: delay + 400 },
      ),
    )
  })
  return nodes
}

export const figures = { map, owners, specs }
export type FigureName = keyof typeof figures
