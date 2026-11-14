import useSWR from 'swr'
import type { FeatureRecord, ProjectRecords } from '@/models/feature'

/**
 * records.json is served by `features serve` and written by `features build`.
 * It is missing for aggregated builds and older versions: no records then.
 */
const fetcher = async (url: string): Promise<FeatureRecord[]> => {
  try {
    const response = await fetch(url)
    if (!response.ok) return []
    const data = (await response.json()) as Partial<ProjectRecords>
    return Array.isArray(data.records) ? data.records : []
  } catch {
    return []
  }
}

export function useProjectRecords() {
  const { data, isLoading } = useSWR<FeatureRecord[]>(
    './records.json',
    fetcher,
    {
      refreshInterval: import.meta.env.WATCH === 'on' ? 1000 : undefined,
    },
  )
  return { records: data ?? [], isLoading }
}
