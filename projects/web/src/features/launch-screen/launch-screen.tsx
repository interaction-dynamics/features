import { FeaturesLogo } from './features-logo'
import './launch-screen.css'

interface LaunchScreenProps {
  /** True while the screen plays its exit animation and reveals the app */
  exiting: boolean
}

/**
 * Splash screen shown while the dashboard loads its data: the `{F}` logo
 * draws itself, ripples out and is swept by a sheen, then zooms away.
 */
export function LaunchScreen({ exiting }: LaunchScreenProps) {
  return (
    <output
      className="launch-screen"
      data-state={exiting ? 'exit' : 'intro'}
      aria-live="polite"
      aria-label="Loading features"
    >
      <div className="launch-backdrop" aria-hidden="true">
        <div className="launch-grid" />
        <div className="launch-glow" />
      </div>

      <div className="launch-stage">
        <div className="launch-logo-wrap" aria-hidden="true">
          <span className="launch-ring" />
          <span className="launch-ring launch-ring-delayed" />
          <span className="launch-halo" />
          <div className="launch-logo">
            <FeaturesLogo className="size-full" />
            <span className="launch-sheen" />
          </div>
        </div>

        <p className="launch-title">Features</p>

        <div className="launch-progress" aria-hidden="true">
          <span />
        </div>
      </div>
    </output>
  )
}
