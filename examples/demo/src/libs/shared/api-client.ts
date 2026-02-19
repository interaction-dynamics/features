// --feature-flag feature:feature-1, type: shared-dependency, location: libs/shared

export class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async get(endpoint: string): Promise<any> {
    const response = await fetch(`${this.baseUrl}${endpoint}`);
    return response.json();
  }
}

// --feature-flag feature:feature-2, type: shared-dependency, location: libs/shared

export function createApiClient(baseUrl: string): ApiClient {
  return new ApiClient(baseUrl);
}
