import { ArrowUpRight, FileText, Folder, FolderOpen } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { FeaturesLogo } from '@/features/launch-screen'
import './no-features.css'

const GUIDELINES_URL =
  'https://github.com/interaction-dynamics/features?tab=readme-ov-file#guidelines'

const STEPS = [
  {
    title: 'Create a feature folder',
    description: (
      <>
        Put it in a <code>features/</code> folder, or mark any folder with{' '}
        <code>feature: true</code> in its README frontmatter.
      </>
    ),
  },
  {
    title: 'Describe it in a README',
    description: (
      <>
        A title, one line of description and an <code>owner</code> in the
        frontmatter.
      </>
    ),
  },
  {
    title: 'Scan again',
    description: (
      <>
        Run <code>features serve</code> or <code>features build</code>: the
        feature shows up here.
      </>
    ),
  },
]

/** Ghost feature cards floating around the logo */
function NoFeaturesIllustration() {
  return (
    <div className="no-features-illustration" aria-hidden="true">
      <div className="no-features-orbit" />
      <div className="no-features-card no-features-card-left">
        <Folder className="size-4" />
        <span className="no-features-line w-14" />
      </div>
      <div className="no-features-card no-features-card-right">
        <FileText className="size-4" />
        <span className="no-features-line w-10" />
      </div>
      <div className="no-features-card no-features-card-bottom">
        <FolderOpen className="size-4" />
        <span className="no-features-line w-16" />
      </div>
      <div className="no-features-logo">
        <FeaturesLogo className="size-full" />
      </div>
    </div>
  )
}

/** Folder tree and README of a first feature, in an editor-like window */
function NoFeaturesExample() {
  return (
    <div className="no-features-window">
      <div className="no-features-window-bar">
        <span />
        <span />
        <span />
        <p>src/features/checkout/README.md</p>
      </div>
      <div className="grid sm:grid-cols-[minmax(0,10rem)_minmax(0,1fr)]">
        <ul className="no-features-tree">
          <li>
            <FolderOpen className="size-3.5" /> src
          </li>
          <li className="ps-4">
            <FolderOpen className="size-3.5" /> features
          </li>
          <li className="ps-8">
            <FolderOpen className="size-3.5" /> checkout
          </li>
          <li className="ps-12 no-features-tree-active">
            <FileText className="size-3.5" /> README.md
          </li>
        </ul>
        <pre className="no-features-code">
          <span className="text-muted-foreground">---</span>
          {'\n'}
          <span className="no-features-code-key">owner</span>: payments-team
          {'\n'}
          <span className="text-muted-foreground">---</span>
          {'\n\n'}
          <span className="no-features-code-heading"># Checkout</span>
          {'\n\n'}
          Lets customers pay for their cart.
        </pre>
      </div>
    </div>
  )
}

/** Full-page empty state of the dashboard when the scan found no feature */
export function NoFeatures() {
  return (
    <section
      className="no-features relative isolate flex min-h-[calc(100svh-4rem)] flex-col items-center justify-center overflow-hidden px-6 py-16"
      aria-labelledby="no-features-title"
    >
      <div className="no-features-backdrop" aria-hidden="true" />

      <NoFeaturesIllustration />

      <div className="no-features-reveal mt-10 flex max-w-xl flex-col items-center text-center [--delay:0.15s]">
        <h1
          id="no-features-title"
          className="text-3xl font-semibold tracking-tight text-balance"
        >
          No feature found yet
        </h1>
        <p className="mt-3 text-base text-muted-foreground text-balance">
          A feature is a folder of your codebase with a README. Add your first
          one and the dashboard maps it, with its owner, specs and decisions.
        </p>
      </div>

      <div className="no-features-reveal mt-10 w-full max-w-3xl [--delay:0.3s]">
        <NoFeaturesExample />
      </div>

      <ol className="no-features-reveal mt-8 grid w-full max-w-3xl gap-4 sm:grid-cols-3 [--delay:0.45s]">
        {STEPS.map((step, index) => (
          <li key={step.title} className="no-features-step">
            <span className="no-features-step-number">{index + 1}</span>
            <p className="font-medium">{step.title}</p>
            <p className="mt-1 text-sm text-muted-foreground">
              {step.description}
            </p>
          </li>
        ))}
      </ol>

      <div className="no-features-reveal mt-10 [--delay:0.6s]">
        <Button asChild size="lg">
          <a href={GUIDELINES_URL} target="_blank" rel="noreferrer">
            Read the guidelines
            <ArrowUpRight />
          </a>
        </Button>
      </div>
    </section>
  )
}
