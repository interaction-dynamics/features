import { box, circle, line, rect, type Point3, type Shape } from './iso'

/** Every feature of a folder as a block on a plate, its height standing for its size. */
const map = (): Shape[] => {
  const blocks: { at: [number, number]; size: Point3; strong?: boolean }[] = [
    { at: [0.6, 0.6], size: [5, 6.4, 3.4], strong: true },
    { at: [6.2, 0.6], size: [5.2, 3, 1.8] },
    { at: [0.6, 7.6], size: [3.4, 3.8, 1.4] },
    { at: [6.2, 4.2], size: [2.4, 2.8, 2.6] },
    { at: [4.6, 7.6], size: [3, 3.8, 2.2] },
    { at: [9, 4.2], size: [2.4, 2.8, 1.2] },
    { at: [8.2, 7.6], size: [3.2, 3.8, 0.7] },
  ]
  const base = 0.6
  const highlight = blocks[0]
  const top = base + highlight.size[2]
  return [
    ...box([0, 0, 0], [12, 12, base], { stroke: 'faint' }),
    ...blocks.flatMap(({ at: [x, y], size, strong }) =>
      box([x, y, base], size, { top: strong ? 'strong' : 'base' }),
    ),
    // Hatching on the highlighted feature, as if it were selected.
    ...[1.6, 2.6, 3.6, 4.6].map((offset) =>
      line(
        [
          [highlight.at[0] + offset, highlight.at[1] + 0.8, top],
          [highlight.at[0] + offset, highlight.at[1] + 5.6, top],
        ],
        { stroke: 'faint' },
      ),
    ),
  ]
}

/** Features grouped on one plate per team, with the imports between them. */
const owners = (): Shape[] => {
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
  return [
    // Team plates, back to front.
    ...box([0, 0, 0], [6.4, 5.8, plate]),
    circle([5.4, 0.9, plate], 0.4, { stroke: 'strong' }),
    ...box([8.4, 0.4, 0], [4.6, 4.6, plate]),
    circle([12.1, 1.2, plate], 0.4),
    ...box([2.4, 8.6, 0], [5, 4, plate]),
    circle([6.6, 9.4, plate], 0.4),
    // Features.
    ...box([0.9, 0.9, plate], [2.4, 2.4, 2.6]),
    ...box([3.8, 3, plate], [1.8, 1.8, 1.4]),
    ...box([9.4, 1.4, plate], [2.6, 2.6, 3.2], { top: 'strong' }),
    ...box([3.2, 9.4, plate], [2.2, 2.2, 1.8]),
    // Imports between features, drawn over everything like a wire.
    line([a, c], { dashed: true }),
    line([a, d], { dashed: true }),
    line([b, c], { dashed: true }),
    circle(c, 0.35, { stroke: 'strong' }),
  ]
}

/** A spec on top of older records: text, then a task list half done. */
const specs = (): Shape[] => {
  const size: Point3 = [7, 9, 0.25]
  const sheets = [0, 1, 2].map((index): Point3 => [index * 0.9, 0, index * 1.9])
  const shapes: Shape[] = []
  sheets.forEach((at, index) => {
    const next = sheets[index + 1]
    shapes.push(
      ...box(at, size, {
        top: index === sheets.length - 1 ? 'strong' : 'base',
      }),
    )
    if (!next) return
    // Dotted guides between the front corners of two sheets.
    for (const [dx, dy] of [
      [size[0], 0],
      [size[0], size[1]],
      [0, size[1]],
    ]) {
      shapes.push(
        line(
          [
            [at[0] + dx, at[1] + dy, at[2] + size[2]],
            [next[0] + dx, next[1] + dy, next[2]],
          ],
          { dashed: true, stroke: 'faint' },
        ),
      )
    }
  })

  const [x, y, z0] = sheets[sheets.length - 1]
  const z = z0 + size[2]
  const text = (
    row: number,
    from: number,
    to: number,
    stroke: Shape['stroke'] = 'faint',
  ) =>
    line(
      [
        [x + from, y + row, z],
        [x + to, y + row, z],
      ],
      { stroke },
    )
  shapes.push(text(1.3, 1, 4.6, 'strong'), text(2.4, 1, 6), text(3.2, 1, 5.2))
  ;[
    { row: 4.8, to: 5.6, done: true },
    { row: 6.1, to: 4.8, done: true },
    { row: 7.4, to: 5.2, done: false },
  ].forEach(({ row, to, done }) => {
    shapes.push(
      rect([x + 1, y + row - 0.4, z], [0.8, 0.8], {
        stroke: done ? 'strong' : 'base',
      }),
      text(row, 2.3, to, done ? 'base' : 'faint'),
    )
    if (done) {
      shapes.push(
        line(
          [
            [x + 1.15, y + row - 0.05, z],
            [x + 1.45, y + row + 0.25, z],
            [x + 1.75, y + row - 0.55, z],
          ],
          { stroke: 'strong' },
        ),
      )
    }
  })
  return shapes
}

export const figures = { map, owners, specs }
export type FigureName = keyof typeof figures
