import useSWR from 'swr'

type Metadata = {
  version: string
}

const fetcher = async (url: string): Promise<Metadata> => {
  const response = await fetch(url)
  const data = await response.json()
  return data
}

export function VersionIndicator() {
  const { data } = useSWR<Metadata>('./metadata.json', fetcher)

  const version = data?.version || ''

  if (!version) return null

  return (
    <div>
      <span className="text-muted-foreground/60 text-xs">v{version}</span>
    </div>
  )
}
