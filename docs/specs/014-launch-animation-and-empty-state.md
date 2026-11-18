---
status: implemented
date: 2026-11-17
---

# Launch animation and full-page empty state

## Context

The dashboard showed a blank page while `features.json` loaded (the data provider suspends with no fallback), then the app popped in. When the scan found no feature, the page only said "No feature found" in gray text. The first impression should feel polished, in the spirit of the launch screens of CleanMyMac and Raycast, and an empty project should explain how to add a first feature.

## Requirements

- Launch screen (`projects/web/src/features/launch-screen/`):
  - Shown over the whole window from the first render until the data and the first route are loaded.
  - Intro: the `{F}` logo rises in with a blur, its braces draw themselves, the F pops in, a light sheen sweeps the tile, rounded-square ripples spread out on a faint dot grid with a spotlight, the "Features" title tightens in, and a thin indeterminate progress bar runs.
  - The intro plays at least 1.4 s, so the animation is never cut, and lasts as long as the data takes to load.
  - Exit (0.9 s): the logo zooms past the viewer and blurs out, the backdrop fades, and the app is revealed with a blur-and-scale-in while the sidebar entries slide in one after the other. The launch screen ignores clicks during the exit and is removed after it.
  - Uses the theme tokens, so it works in light and dark mode; the logo tile is the brand logo.
  - `prefers-reduced-motion`: no minimum intro, no ripples, halo or sheen, only short fades.
  - Browsers driven by automation (`navigator.webdriver`, e.g. Playwright) skip it, so end-to-end tests don't wait for it.
- Empty state (`projects/web/src/features/no-features/`), on the Features page when the scan found no feature:
  - Fills the page under the header: an illustration (logo surrounded by floating ghost feature cards on a dashed orbit), a title and a one-line explanation.
  - An editor-like window with the folder tree `src/features/checkout/README.md` and the README content (frontmatter with `owner`, title, description).
  - Three steps: create a feature folder (or `feature: true` in a README frontmatter), describe it in a README, scan again with `features serve` or `features build`.
  - A "Read the guidelines" button opening the README guidelines.
  - Sections rise in one after the other; ghost cards float. Reduced motion disables the animations.
  - Responsive down to phone width without horizontal scroll.

## Out of scope

- A splash in `index.html` before the JavaScript bundle loads.
- Empty states of other pages (Insights, Outside features).
- Changing the data loading itself.

## Plan

1. `FeaturesLogo`: the logo SVG inlined as a component with animatable braces and letter.
2. `LaunchScreen` (markup + CSS keyframes) and `LaunchGate`, which wraps the app in a Suspense boundary, waits for the data and the minimum intro, plays the exit and reveals the app.
3. Wrap `FeaturesProvider` with `LaunchGate` in `index.tsx`.
4. `NoFeatures` full-page empty state, rendered by the Features route when `features` is empty.
5. Check both in the browser, in light and dark mode and at phone width, and run the end-to-end tests.
