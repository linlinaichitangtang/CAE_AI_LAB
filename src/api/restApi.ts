/**
 * REST API Client for CAELab
 * Provides typed access to the CAELab REST API endpoints
 */

const DEFAULT_BASE_URL = 'http://127.0.0.1:3001/api/v1'

export interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  email: string
  password: string
  nickname?: string
}

export interface CreateProjectRequest {
  name: string
  description: string
}

export interface UpdateProjectRequest {
  name?: string
  description?: string
}

export interface CreateFileRequest {
  file_type: string
  file_name: string
  content?: string
  file_path: string
}

export interface UpdateFileRequest {
  file_name?: string
  content?: string
}

export interface CreateApiKeyRequest {
  name: string
  permissions?: string
  expires_days?: number
}

export interface ShareProjectRequest {
  shared_with_name: string
  permission: 'read' | 'write' | 'admin'
}

export interface RunSimulationRequest {
  project_id: string
  job_name: string
  config: Record<string, unknown>
}

export interface GenerateMeshRequest {
  mesh_type: string
  config: Record<string, unknown>
}

export interface UpdateProfileRequest {
  nickname?: string
  company?: string
  position?: string
  avatar_url?: string
}

export interface ApiKeyInfo {
  id: string
  user_id: string
  name: string
  key_prefix: string
  permissions: string
  call_count: number
  last_used_at: string | null
  created_at: string
  expires_at: string | null
}

export interface ApiKeyCreateResponse extends ApiKeyInfo {
  key: string
  warning: string
}

export interface ServerStatus {
  is_running: boolean
  port: number
  docs_url: string | null
  base_url: string | null
}

/**
 * Make an API request to the REST API server
 */
export async function apiRequest<T>(
  method: string,
  path: string,
  body?: unknown,
  token?: string
): Promise<ApiResponse<T>> {
  const baseUrl = localStorage.getItem('caelab-api-base-url') || DEFAULT_BASE_URL
  const authToken = token || localStorage.getItem('caelab-api-token') || undefined

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  }
  if (authToken) {
    headers['Authorization'] = `Bearer ${authToken}`
  }

  const response = await fetch(`${baseUrl}${path}`, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
  })

  const data = await response.json()

  if (!response.ok) {
    return {
      success: false,
      data: null,
      error: data.error || `HTTP ${response.status}`,
    }
  }

  return data as ApiResponse<T>
}

// ============================================================================
// Auth API
// ============================================================================

export const authApi = {
  login: (req: LoginRequest) =>
    apiRequest<{ access_token: string; refresh_token: string }>('POST', '/auth/login', req),

  register: (req: RegisterRequest) =>
    apiRequest<{ access_token: string; refresh_token: string }>('POST', '/auth/register', req),

  refresh: (refreshToken: string) =>
    apiRequest<{ access_token: string; refresh_token: string }>('POST', '/auth/refresh', { refresh_token: refreshToken }),
}

// ============================================================================
// Projects API
// ============================================================================

export const projectsApi = {
  list: () =>
    apiRequest<Record<string, unknown>[]>('GET', '/projects'),

  create: (req: CreateProjectRequest) =>
    apiRequest<Record<string, unknown>>('POST', '/projects', req),

  get: (id: string) =>
    apiRequest<Record<string, unknown>>('GET', `/projects/${id}`),

  update: (id: string, req: UpdateProjectRequest) =>
    apiRequest<Record<string, unknown>>('PUT', `/projects/${id}`, req),

  delete: (id: string) =>
    apiRequest<string>('DELETE', `/projects/${id}`),
}

// ============================================================================
// Files API
// ============================================================================

export const filesApi = {
  list: (projectId: string) =>
    apiRequest<Record<string, unknown>[]>('GET', `/projects/${projectId}/files`),

  create: (projectId: string, req: CreateFileRequest) =>
    apiRequest<Record<string, unknown>>('POST', `/projects/${projectId}/files`, req),

  get: (id: string) =>
    apiRequest<Record<string, unknown>>('GET', `/files/${id}`),

  update: (id: string, req: UpdateFileRequest) =>
    apiRequest<Record<string, unknown>>('PUT', `/files/${id}`, req),

  delete: (id: string) =>
    apiRequest<string>('DELETE', `/files/${id}`),
}

// ============================================================================
// Simulations API
// ============================================================================

export const simulationsApi = {
  run: (req: RunSimulationRequest) =>
    apiRequest<Record<string, unknown>>('POST', '/simulations', req),

  getStatus: (id: string) =>
    apiRequest<Record<string, unknown>>('GET', `/simulations/${id}`),

  getResult: (id: string) =>
    apiRequest<Record<string, unknown>>('GET', `/simulations/${id}/result`),
}

// ============================================================================
// Mesh API
// ============================================================================

export const meshApi = {
  generate: (req: GenerateMeshRequest) =>
    apiRequest<Record<string, unknown>>('POST', '/mesh/generate', req),
}

// ============================================================================
// Users API
// ============================================================================

export const usersApi = {
  getProfile: () =>
    apiRequest<Record<string, unknown>>('GET', '/users/me'),

  updateProfile: (req: UpdateProfileRequest) =>
    apiRequest<Record<string, unknown>>('PUT', '/users/me', req),
}

// ============================================================================
// Collaboration API
// ============================================================================

export const collaborationApi = {
  shareProject: (projectId: string, req: ShareProjectRequest) =>
    apiRequest<Record<string, unknown>>('POST', `/projects/${projectId}/share`, req),

  listShares: (projectId: string) =>
    apiRequest<Record<string, unknown>[]>('GET', `/projects/${projectId}/shares`),
}

// ============================================================================
// API Keys API
// ============================================================================

export const apiKeysApi = {
  list: () =>
    apiRequest<ApiKeyInfo[]>('GET', '/api-keys'),

  create: (req: CreateApiKeyRequest) =>
    apiRequest<ApiKeyCreateResponse>('POST', '/api-keys', req),

  delete: (id: string) =>
    apiRequest<string>('DELETE', `/api-keys/${id}`),
}

// ============================================================================
// Health Check
// ============================================================================

export const healthApi = {
  check: () =>
    apiRequest<{ status: string; version: string; uptime: number }>('GET', '/health'),
}
