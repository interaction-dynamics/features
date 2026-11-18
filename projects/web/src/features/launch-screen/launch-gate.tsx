import { Suspense, useCallback, useEffect, useState } from 'react'
import { LaunchScreen } from './launch-screen'

/** Shortest time the intro plays, so the logo animation can finish */
export const INTRO_MIN_MS = 1400
/** Length of the exit animation, before the launch screen is removed */
export const EXIT_MS = 900

type Phase = 'intro' | 'exit' | 'done'

const prefersReducedMotion = () =>
  typeof window !== 'undefined' &&
  window.matchMedia?.('(prefers-reduced-motion: reduce)').matches

/** Browsers driven by automation (end-to-end tests) skip the animation */
const isAutomated = () =>
  typeof navigator !== 'undefined' && navigator.webdriver === true

function ReadySignal({ onReady }: { onReady: () => void }) {
  useEffect(() => {
    onReady()
  }, [onReady])
  return null
}

/**
 * Shows the launch screen until its children (data and first route) are
 * loaded and the intro has played, then reveals them.
 */
export function LaunchGate({ children }: { children: React.ReactNode }) {
  const [ready, setReady] = useState(false)
  const [introPlayed, setIntroPlayed] = useState(false)
  const [done, setDone] = useState(isAutomated)

  const onReady = useCallback(() => setReady(true), [])

  useEffect(() => {
    const timeout = setTimeout(
      () => setIntroPlayed(true),
      prefersReducedMotion() ? 0 : INTRO_MIN_MS,
    )
    return () => clearTimeout(timeout)
  }, [])

  // The exit starts once the data is loaded and the intro has played
  const exiting = !done && ready && introPlayed

  useEffect(() => {
    if (!exiting) return
    const timeout = setTimeout(
      () => setDone(true),
      prefersReducedMotion() ? 200 : EXIT_MS,
    )
    return () => clearTimeout(timeout)
  }, [exiting])

  const phase: Phase = done ? 'done' : exiting ? 'exit' : 'intro'

  const appClassName =
    phase === 'intro'
      ? 'launch-app-hidden'
      : phase === 'exit'
        ? 'launch-app-reveal'
        : undefined

  return (
    <>
      <div className={appClassName}>
        <Suspense fallback={null}>
          <ReadySignal onReady={onReady} />
          {children}
        </Suspense>
      </div>
      {phase !== 'done' && <LaunchScreen exiting={phase === 'exit'} />}
    </>
  )
}
