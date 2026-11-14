/**
 * Builds a standalone copy of `examples/demo` with a realistic git history.
 *
 * The demo lives in this repository, where pull requests are squash merged, so
 * only a couple of commits ever touched it. This script replays its files into
 * a fresh git repository as a series of commits by several authors spread over
 * several months, so the demo shows meaningful changes, history and insights.
 *
 * Usage: bun .github/scripts/build-demo-history.ts <destination>
 *
 * The destination must not exist or be empty. The final tree is identical to
 * the tracked files of `examples/demo`.
 */

import { execFileSync } from 'node:child_process'
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'

const DEMO_DIR = resolve(import.meta.dir, '../../examples/demo')

type Author = { name: string; email: string }

const AUTHORS = {
  alice: { name: 'Alice Martin', email: 'alice.martin@example.com' },
  bilal: { name: 'Bilal Haddad', email: 'bilal.haddad@example.com' },
  chloe: { name: 'Chloé Dubois', email: 'chloe.dubois@example.com' },
  daniel: { name: 'Daniel Okafor', email: 'daniel.okafor@example.com' },
  emma: { name: 'Emma Schmidt', email: 'emma.schmidt@example.com' },
  john: { name: 'John Doe', email: 'john.doe@example.com' },
} satisfies Record<string, Author>

/**
 * A path relative to the demo root, optionally followed by `@<fraction>`.
 *
 * - A path ending with `/` selects every file under that folder.
 * - `@0.5` writes only the first half of the file's lines, so a later commit
 *   can complete it and show up as a modification.
 */
type FileSpec = string

type Step = {
  date: string
  author: keyof typeof AUTHORS
  message: string
  files: FileSpec[]
}

const F0 = 'src/libs/features/feature-0/'
const F1 = 'src/libs/features/feature-1/'
const F2 = 'src/libs/features/feature-2/'
const R1 = 'src/routes/route-1/'
const R3 = 'src/routes/route-3/'

const STEPS: Step[] = [
  {
    date: '2026-03-03T09:12:00Z',
    author: 'john',
    message: 'chore: scaffold demo project',
    files: [
      'package.json',
      'pnpm-lock.yaml',
      'tsconfig.json',
      'vitest.config.ts',
      'src/index.ts@0.5',
      'src/components/',
      'src/hooks/',
    ],
  },
  {
    date: '2026-03-05T14:40:00Z',
    author: 'alice',
    message: 'feat(feature-1): add Feature 1 skeleton',
    files: [`${F1}README.mdx@0.6`, `${F1}index.ts@0.5`],
  },
  {
    date: '2026-03-09T10:05:00Z',
    author: 'alice',
    message: 'feat(feature-1): add Foo component and useHook',
    files: [`${F1}components/`, `${F1}hooks/`],
  },
  {
    date: '2026-03-12T16:22:00Z',
    author: 'john',
    message: 'feat(routes): add route 1',
    files: [
      `${R1}README.md`,
      `${R1}features/feature-0/README.md`,
      `${R1}features/feature-0/index.ts@0.5`,
    ],
  },
  {
    date: '2026-03-17T11:31:00Z',
    author: 'bilal',
    message: 'feat(feature-2): add Feature 2 with Bar component',
    files: [
      `${F2}README.mdx@0.5`,
      `${F2}index.ts`,
      `${F2}components/`,
      `${F2}hooks/`,
    ],
  },
  {
    date: '2026-03-20T15:08:00Z',
    author: 'alice',
    message: 'test(feature-1): cover the Feature 1 entry point',
    files: ['tests/feature-1.test.ts@0.5', `${F1}index.ts`],
  },
  {
    date: '2026-03-26T09:47:00Z',
    author: 'chloe',
    message: 'feat(feature-0): start the Glass Material Editor',
    files: [`${F0}README.md@0.5`, `${F0}file.tsx@0.4`],
  },
  {
    date: '2026-04-01T13:15:00Z',
    author: 'chloe',
    message: 'docs(feature-0): record the decision to use technology 1',
    files: [`${F0}__docs__/decisions/001-use-technology-1.md`],
  },
  {
    date: '2026-04-06T10:52:00Z',
    author: 'john',
    message: 'feat(route-1): add route utilities',
    files: [
      `${R1}features/feature-0/route-utils.ts`,
      `${R1}features/feature-0/index.ts`,
    ],
  },
  {
    date: '2026-04-10T17:03:00Z',
    author: 'daniel',
    message: 'feat(shared): add API client and flag helpers',
    files: ['src/libs/shared/api-client.ts@0.6', 'src/libs/shared/flags.ts@0.5'],
  },
  {
    date: '2026-04-15T09:26:00Z',
    author: 'bilal',
    message: 'docs(feature-2): add an architecture decision record',
    files: [
      `${F2}__docs__/decisions/README.md`,
      `${F2}__docs__/decisions/001-architecture-choice.md`,
    ],
  },
  {
    date: '2026-04-21T14:44:00Z',
    author: 'chloe',
    message: 'feat(feature-0): render materials with canvas',
    files: [
      `${F0}file.tsx@0.6`,
      `${F0}__docs__/decisions/002-render-with-canvas.md`,
    ],
  },
  {
    date: '2026-04-28T11:19:00Z',
    author: 'emma',
    message: 'feat(feature-1): add nested Feature 4',
    files: [
      `${F1}features/feature-4/README.md`,
      `${F1}features/feature-4/feature-4.ts@0.5`,
    ],
  },
  {
    date: '2026-05-04T16:37:00Z',
    author: 'emma',
    message: 'test(feature-4): add unit tests',
    files: [
      `${F1}features/feature-4/__tests__/`,
      `${F1}features/feature-4/feature-4.ts`,
    ],
  },
  {
    date: '2026-05-08T10:11:00Z',
    author: 'john',
    message: 'feat(routes): add route 2 and Feature 20',
    files: ['src/routes/route-2/'],
  },
  {
    date: '2026-05-13T15:58:00Z',
    author: 'daniel',
    message: 'feat(scripts): add feature flag toggles',
    files: [
      'src/scripts/toggle.ts',
      'src/scripts/feature-flag.tsx',
      'src/scripts/another-flag.ts',
      'src/libs/shared/flags.ts',
    ],
  },
  {
    date: '2026-05-19T09:34:00Z',
    author: 'alice',
    message: 'feat(feature-1): detect feature flags automatically',
    files: [`${F1}auto-detected.ts`, `${F1}another-flag.ts`],
  },
  {
    date: '2026-05-26T13:49:00Z',
    author: 'bilal',
    message: 'docs(feature-2): document the database choice',
    files: [`${F2}__docs__/decisions/002-database-choice.md`],
  },
  {
    date: '2026-06-02T10:27:00Z',
    author: 'chloe',
    message: 'feat(feature-0): switch rendering to WebGL',
    files: [
      `${F0}file.tsx@0.85`,
      `${F0}__docs__/decisions/003-render-with-webgl.md`,
    ],
  },
  {
    date: '2026-06-05T15:12:00Z',
    author: 'chloe',
    message: 'test(feature-0): add rendering tests',
    files: [`${F0}__tests__/`, `${F0}__docs__/specs/001-test-assertions.md`],
  },
  {
    date: '2026-06-11T11:45:00Z',
    author: 'john',
    message: 'feat(route-1): add Feature 10 and Feature 11',
    files: [`${R1}features/feature-10/`, `${R1}features/feature-11/`],
  },
  {
    date: '2026-06-18T14:06:00Z',
    author: 'daniel',
    message: 'feat(services): add the services Feature 1',
    files: ['src/services/'],
  },
  {
    date: '2026-06-24T09:58:00Z',
    author: 'emma',
    message: 'feat(feature-1): add Feature 3 helpers',
    files: [`${F1}features/feature-3/`],
  },
  {
    date: '2026-07-01T16:21:00Z',
    author: 'bilal',
    message: 'feat(feature-2): link related features automatically',
    files: [`${F2}auto-link.js`, `${F2}README.mdx@0.8`],
  },
  {
    date: '2026-07-07T10:39:00Z',
    author: 'alice',
    message: 'fix(feature-1): keep metadata order stable',
    files: [`${F1}order-test.ts`, `${F1}override-test.ts`],
  },
  {
    date: '2026-07-15T13:24:00Z',
    author: 'john',
    message: 'feat(routes): add route 3 with the deprecated Feature 4',
    files: [
      `${R3}README.md`,
      `${R3}features/feature-4/`,
      `${R3}features/feature-1/`,
    ],
  },
  {
    date: '2026-07-22T17:10:00Z',
    author: 'daniel',
    message: 'chore(scripts): add deploy and analysis scripts',
    files: [
      'src/scripts/deploy.sh',
      'src/scripts/analyzer.cpp',
      'src/scripts/processor.rs',
      'src/scripts/utils.py',
    ],
  },
  {
    date: '2026-07-30T09:41:00Z',
    author: 'chloe',
    message: 'docs(feature-0): write the Glass Material Editor spec',
    files: [
      `${F0}__docs__/specs/002-glass-material-editor/spec.md`,
      `${F0}README.md@0.8`,
    ],
  },
  {
    date: '2026-08-06T14:53:00Z',
    author: 'bilal',
    message: 'fix(feature-2): preserve declaration order',
    files: [`${F2}order-preserve.js`, `${F2}test.js`],
  },
  {
    date: '2026-08-13T11:02:00Z',
    author: 'chloe',
    message: 'docs(feature-0): plan the Glass Material Editor tasks',
    files: [
      `${F0}__docs__/specs/002-glass-material-editor/plan.md`,
      `${F0}__docs__/specs/002-glass-material-editor/tasks.md`,
    ],
  },
  {
    date: '2026-08-20T15:36:00Z',
    author: 'alice',
    message: 'feat(feature-1): show metadata in the dashboard',
    files: [`${F1}metadata-display-test.ts`, `${F1}README.mdx@0.8`],
  },
  {
    date: '2026-08-27T10:18:00Z',
    author: 'emma',
    message: 'docs(feature-1): describe the duplicated Feature 2',
    files: [`${F1}features/feature-2/`],
  },
  {
    date: '2026-09-03T16:44:00Z',
    author: 'daniel',
    message: 'feat(scripts): run experiments behind flags',
    files: ['src/scripts/experiment.ts', 'src/libs/shared/api-client.ts'],
  },
  {
    date: '2026-09-10T09:29:00Z',
    author: 'john',
    message: 'feat(route-1): add a feature with a very long name',
    files: [
      `${R1}features/feature-12-with-a-very-long-name-that-is-longer-than-sidebar/`,
    ],
  },
  {
    date: '2026-09-17T13:57:00Z',
    author: 'alice',
    message: 'docs(feature-1): record the technology decision',
    files: [`${F1}__docs__/`],
  },
  {
    date: '2026-09-24T11:33:00Z',
    author: 'bilal',
    message: 'docs(feature-2): expand the Feature 2 README',
    files: [`${F2}README.mdx`],
  },
  {
    date: '2026-10-01T15:16:00Z',
    author: 'emma',
    message: 'feat(route-3): add Feature 5',
    files: [`${R3}features/feature-5/`],
  },
  {
    date: '2026-10-08T10:48:00Z',
    author: 'chloe',
    message: 'feat(feature-0): finish the material presets',
    files: [`${F0}file.tsx`],
  },
  {
    date: '2026-10-15T14:25:00Z',
    author: 'alice',
    message: 'test(feature-1): extend coverage',
    files: ['tests/feature-1.test.ts', `${F1}README.mdx`],
  },
  {
    date: '2026-10-22T09:07:00Z',
    author: 'john',
    message: 'chore: generate CODEOWNERS',
    files: ['CODEOWNERS'],
  },
  {
    date: '2026-10-29T16:02:00Z',
    author: 'daniel',
    message: 'refactor: export all features from the entry point',
    files: ['src/index.ts'],
  },
  {
    date: '2026-11-05T11:54:00Z',
    author: 'chloe',
    message: 'docs(feature-0): update the README metadata',
    files: [`${F0}README.md`],
  },
]

function git(cwd: string, args: string[], env: Record<string, string> = {}) {
  return execFileSync('git', args, {
    cwd,
    env: { ...process.env, ...env },
    encoding: 'utf8',
  })
}

function trackedDemoFiles(): string[] {
  return git(DEMO_DIR, ['ls-files', '-z', '.'])
    .split('\0')
    .filter(Boolean)
    .sort()
}

function parseSpec(spec: FileSpec): { path: string; fraction: number } {
  const at = spec.lastIndexOf('@')
  if (at === -1) return { path: spec, fraction: 1 }
  return { path: spec.slice(0, at), fraction: Number(spec.slice(at + 1)) }
}

function matchFiles(path: string, files: string[]): string[] {
  const matches = path.endsWith('/')
    ? files.filter((file) => file.startsWith(path))
    : files.filter((file) => file === path)
  if (matches.length === 0) {
    throw new Error(`No demo file matches "${path}"`)
  }
  return matches
}

/** The first `fraction` of a file's lines, so it can be completed later */
function partialContent(content: Buffer, fraction: number): Buffer {
  if (fraction >= 1) return content
  const lines = content.toString('utf8').split('\n')
  const count = Math.max(1, Math.ceil(lines.length * fraction))
  return Buffer.from(`${lines.slice(0, count).join('\n')}\n`, 'utf8')
}

function writeFile(destination: string, file: string, content: Buffer) {
  const target = join(destination, file)
  mkdirSync(dirname(target), { recursive: true })
  writeFileSync(target, content)
}

function commit(destination: string, step: Step): boolean {
  // --force: the demo's own .gitignore files must not hide its tracked files
  git(destination, ['add', '--all', '--force'])
  const staged = git(destination, ['diff', '--cached', '--name-only']).trim()
  if (!staged) return false

  const author = AUTHORS[step.author]
  git(destination, ['commit', '--quiet', '--message', step.message], {
    GIT_AUTHOR_NAME: author.name,
    GIT_AUTHOR_EMAIL: author.email,
    GIT_AUTHOR_DATE: step.date,
    GIT_COMMITTER_NAME: author.name,
    GIT_COMMITTER_EMAIL: author.email,
    GIT_COMMITTER_DATE: step.date,
  })
  return true
}

function main() {
  const destinationArg = process.argv[2]
  if (!destinationArg) {
    console.error('Usage: bun .github/scripts/build-demo-history.ts <destination>')
    process.exit(1)
  }

  const destination = resolve(destinationArg)
  if (existsSync(destination) && readdirSync(destination).length > 0) {
    console.error(`Destination "${destination}" is not empty`)
    process.exit(1)
  }
  mkdirSync(destination, { recursive: true })

  const files = trackedDemoFiles()
  const contents = new Map(
    files.map((file) => [file, readFileSync(join(DEMO_DIR, file))]),
  )

  git(destination, ['init', '--quiet', '--initial-branch=main'])

  let commits = 0
  for (const step of STEPS) {
    for (const spec of step.files) {
      const { path, fraction } = parseSpec(spec)
      for (const file of matchFiles(path, files)) {
        writeFile(destination, file, partialContent(contents.get(file)!, fraction))
      }
    }
    if (commit(destination, step)) commits++
  }

  // Safety net: any file the steps above missed, or left partial, lands here
  // so the final tree always matches the demo.
  for (const file of files) {
    writeFile(destination, file, contents.get(file)!)
  }
  if (
    commit(destination, {
      date: '2026-11-10T10:00:00Z',
      author: 'john',
      message: 'chore: sync demo files',
      files: [],
    })
  ) {
    commits++
    console.warn('Some demo files were not covered by the scripted history')
  }

  console.log(`Built demo history with ${commits} commits in ${destination}`)
}

main()
