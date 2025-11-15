import useSWR from 'swr'

export interface Metadata {
  version: string
  repository?: string
}

const fetcher = async (url: string): Promise<Metadata> => {
  const response = await fetch(url)
  return response.json()
}

/**
 * Custom hook to fetch and cache metadata from /metadata.json
 * Uses SWR for caching and revalidation
 *
 * @returns Object containing metadata, loading state, and error
 */
export function useMetadata() {
  const { data, error, isLoading } = useSWR<Metadata>(
    './metadata.json',
    fetcher,
    {
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
      dedupingInterval: 60000, // Cache for 1 minute
    },
  )

  return {
    metadata: data,
    isLoading,
    error,
  }
}
