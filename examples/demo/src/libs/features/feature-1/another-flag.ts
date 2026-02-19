// --feature-flag feature:feature-1, type: backend, service: api, version: 2.0

export async function fetchData(endpoint: string): Promise<any> {
  const response = await fetch(endpoint);
  return response.json();
}

// Another feature flag in the same file
// --feature-flag feature:feature-1, type: frontend, component: DataDisplay

export function DataDisplay({ data }: { data: any }) {
  return <div>{JSON.stringify(data)}</div>;
}
