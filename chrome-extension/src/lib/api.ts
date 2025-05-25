import { Project } from './store'

// Ensure RequestInit is recognized from lib.dom.d.ts
interface ApiConfig {
  apiKey: string
  serverUrl: string
}

class ApiError extends Error {
  constructor(
    public message: string,
    public status?: number,
    public statusText?: string
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

class ContexterApi {
  private async makeRequest<T>(
    endpoint: string,
    config: ApiConfig,
    options: {
      method?: string
      headers?: Record<string, string>
      body?: string
    } = {}
  ): Promise<T> {
    const { apiKey, serverUrl } = config

    if (!apiKey || !serverUrl) {
      throw new ApiError('API Key or Server URL is missing')
    }

    const url = `${serverUrl}${endpoint}`

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          'X-API-Key': apiKey,
          'Content-Type': 'application/json',
          ...options.headers,
        },
      })

      if (!response.ok) {
        throw new ApiError(
          `Request failed: ${response.statusText}`,
          response.status,
          response.statusText
        )
      }

      return await response.json()
    } catch (error) {
      if (error instanceof ApiError) {
        throw error
      }
      throw new ApiError(
        `Network error: ${error instanceof Error ? error.message : 'Unknown error'}`
      )
    }
  }

  async fetchProjects(config: ApiConfig): Promise<Project[]> {
    const data = await this.makeRequest<{ projects: Project[] }>('/api/v1/projects', config)
    return data.projects
  }

  async fetchProjectMetadata(projectName: string, config: ApiConfig): Promise<Project> {
    return this.makeRequest<Project>(`/api/v1/projects/${encodeURIComponent(projectName)}`, config)
  }

  async fetchProjectContent(
    projectName: string,
    selectedFiles: string[],
    allFiles: string[],
    config: ApiConfig
  ): Promise<string> {
    // If all files are selected, send empty array to get everything
    const paths = selectedFiles.length === allFiles.length ? [] : selectedFiles

    const data = await this.makeRequest<{ content: string }>(
      `/api/v1/projects/${encodeURIComponent(projectName)}`,
      config,
      {
        method: 'POST',
        body: JSON.stringify({ paths }),
      }
    )
    return data.content
  }

  async validateApiKey(config: ApiConfig): Promise<boolean> {
    try {
      await this.makeRequest('/api/v1/projects', config)
      return true
    } catch {
      return false
    }
  }
}

export const api = new ContexterApi()
export { ApiError }
