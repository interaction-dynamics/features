import { useMetadata } from './use-metadata'

/**
 * Whether the experimental pages of the dashboard (History, owner pages,
 * Project records) are enabled.
 *
 * Enabled by `metadata.json` (`features serve --experimental` and
 * `features build --experimental`) or, when developing the web app, by the
 * `VITE_EXPERIMENTAL_FEATURES=true` environment variable.
 */
export function useExperimentalFeatures() {
  const { metadata, isLoading } = useMetadata()

  const enabled =
    import.meta.env.VITE_EXPERIMENTAL_FEATURES === 'true' ||
    metadata?.experimental === true

  return { enabled, isLoading: !enabled && isLoading }
}
