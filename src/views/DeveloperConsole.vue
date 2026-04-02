<template>
  <div class="h-full overflow-y-auto">
    <!-- Header -->
    <div class="sticky top-0 z-10 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center">
            <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
            </svg>
          </div>
          <div>
            <h2 class="text-lg font-bold">Developer Console</h2>
            <p class="text-xs text-[var(--text-secondary)]">REST API Open Platform</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <span
            class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium"
            :class="serverStatus.is_running ? 'bg-green-500/10 text-green-500' : 'bg-[var(--bg-hover)] text-[var(--text-secondary)]'"
          >
            <span
              class="w-1.5 h-1.5 rounded-full"
              :class="serverStatus.is_running ? 'bg-green-500 animate-pulse' : 'bg-[var(--text-secondary)]'"
            ></span>
            {{ serverStatus.is_running ? 'Running' : 'Stopped' }}
          </span>
        </div>
      </div>
    </div>

    <div class="p-6 space-y-6">
      <!-- Server Control Panel -->
      <section class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
        <div class="px-5 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-hover)]">
          <h3 class="text-sm font-semibold flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2" />
            </svg>
            API Server Control
          </h3>
        </div>
        <div class="p-5 space-y-4">
          <!-- Port Configuration -->
          <div class="flex items-end gap-3">
            <div class="flex-1">
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-1">Port</label>
              <input
                v-model.number="port"
                type="number"
                min="1024"
                max="65535"
                :disabled="serverStatus.is_running"
                class="w-full px-3 py-2 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-base)] font-mono text-sm focus:outline-none focus:ring-2 focus:ring-green-500/50 disabled:opacity-50"
              />
            </div>
            <button
              @click="toggleServer"
              :disabled="isToggling"
              class="px-5 py-2 rounded-lg font-medium text-sm transition-all duration-200"
              :class="serverStatus.is_running
                ? 'bg-red-500/10 text-red-500 hover:bg-red-500/20 border border-red-500/20'
                : 'bg-green-500/10 text-green-500 hover:bg-green-500/20 border border-green-500/20'"
            >
              {{ isToggling ? 'Please wait...' : (serverStatus.is_running ? 'Stop Server' : 'Start Server') }}
            </button>
          </div>

          <!-- Server Info -->
          <div v-if="serverStatus.is_running" class="grid grid-cols-2 gap-3">
            <div class="bg-[var(--bg-base)] rounded-lg p-3">
              <div class="text-xs text-[var(--text-secondary)]">Base URL</div>
              <code class="text-sm font-mono text-green-500 mt-1 block">{{ serverStatus.base_url }}</code>
            </div>
            <div class="bg-[var(--bg-base)] rounded-lg p-3">
              <div class="text-xs text-[var(--text-secondary)]">Swagger UI</div>
              <a
                :href="serverStatus.docs_url || undefined"
                target="_blank"
                class="text-sm font-mono text-[var(--accent-primary)] hover:underline mt-1 block"
              >
                {{ serverStatus.docs_url }}
              </a>
            </div>
          </div>
        </div>
      </section>

      <!-- API Key Management -->
      <section class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
        <div class="px-5 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-hover)] flex items-center justify-between">
          <h3 class="text-sm font-semibold flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
            </svg>
            API Keys
          </h3>
          <button
            @click="showCreateKey = true"
            class="text-xs px-3 py-1.5 rounded-md bg-[var(--accent-primary)] text-white hover:opacity-90 transition-opacity"
          >
            + New Key
          </button>
        </div>
        <div class="p-5">
          <!-- Create Key Form -->
          <div v-if="showCreateKey" class="mb-4 p-4 bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] space-y-3">
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-1">Key Name</label>
              <input
                v-model="newKeyName"
                type="text"
                placeholder="e.g., My Script, CI/CD Pipeline"
                class="w-full px-3 py-2 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]/50"
              />
            </div>
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-1">Permissions</label>
              <select
                v-model="newKeyPermissions"
                class="w-full px-3 py-2 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]/50"
              >
                <option value="read">Read Only</option>
                <option value="read_write">Read & Write</option>
                <option value="admin">Full Access</option>
              </select>
            </div>
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-1">Expires (days)</label>
              <input
                v-model.number="newKeyExpires"
                type="number"
                min="1"
                max="365"
                placeholder="90"
                class="w-full px-3 py-2 rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]/50"
              />
            </div>
            <div class="flex justify-end gap-2">
              <button
                @click="showCreateKey = false"
                class="px-3 py-1.5 text-xs rounded-md border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)]"
              >
                Cancel
              </button>
              <button
                @click="createApiKey"
                :disabled="!newKeyName.trim() || isCreatingKey"
                class="px-3 py-1.5 text-xs rounded-md bg-[var(--accent-primary)] text-white hover:opacity-90 disabled:opacity-50"
              >
                {{ isCreatingKey ? 'Creating...' : 'Create Key' }}
              </button>
            </div>
          </div>

          <!-- New Key Display (shown once after creation) -->
          <div v-if="newlyCreatedKey" class="mb-4 p-4 bg-yellow-500/5 rounded-lg border border-yellow-500/20 space-y-2">
            <div class="flex items-center gap-2 text-yellow-600 text-sm font-medium">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
              </svg>
              Save this API key now! You will not be able to see it again.
            </div>
            <div class="flex items-center gap-2">
              <code class="flex-1 px-3 py-2 bg-[var(--bg-base)] rounded-lg text-sm font-mono text-yellow-600 select-all break-all">{{ newlyCreatedKey.key }}</code>
              <button
                @click="copyToClipboard(newlyCreatedKey.key)"
                class="px-3 py-2 rounded-lg bg-[var(--bg-base)] hover:bg-[var(--bg-hover)] text-xs transition-colors"
                title="Copy"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
              </button>
            </div>
            <button
              @click="newlyCreatedKey = null"
              class="text-xs text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
            >
              Done
            </button>
          </div>

          <!-- API Keys List -->
          <div v-if="apiKeys.length === 0" class="text-center py-8 text-[var(--text-secondary)] text-sm">
            <svg class="w-10 h-10 mx-auto mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
            </svg>
            No API keys yet. Create one to get started.
          </div>
          <div v-else class="space-y-2">
            <div
              v-for="key in apiKeys"
              :key="key.id"
              class="flex items-center justify-between p-3 bg-[var(--bg-base)] rounded-lg group"
            >
              <div class="flex-1 min-w-0">
                <div class="text-sm font-medium truncate">{{ key.name }}</div>
                <div class="flex items-center gap-3 mt-1">
                  <code class="text-xs font-mono text-[var(--text-secondary)]">{{ key.key_prefix }}...</code>
                  <span class="text-xs px-1.5 py-0.5 rounded bg-[var(--bg-hover)] text-[var(--text-secondary)]">{{ key.permissions }}</span>
                  <span class="text-xs text-[var(--text-secondary)]">{{ key.call_count }} calls</span>
                </div>
              </div>
              <div class="flex items-center gap-2 ml-3">
                <span
                  v-if="key.expires_at"
                  class="text-xs text-[var(--text-secondary)]"
                  :class="{ 'text-yellow-500': isExpiringSoon(key.expires_at) }"
                >
                  Expires: {{ formatDate(key.expires_at) }}
                </span>
                <button
                  @click="deleteApiKey(key.id)"
                  class="p-1.5 rounded-md text-[var(--text-secondary)] hover:text-red-500 hover:bg-red-500/10 opacity-0 group-hover:opacity-100 transition-all"
                  title="Delete"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- API Endpoints Reference -->
      <section class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
        <div class="px-5 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-hover)]">
          <h3 class="text-sm font-semibold flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            API Endpoints Reference
          </h3>
        </div>
        <div class="p-5">
          <div class="overflow-x-auto">
            <table class="w-full text-sm">
              <thead>
                <tr class="text-left text-xs text-[var(--text-secondary)] border-b border-[var(--border-subtle)]">
                  <th class="pb-2 pr-4 font-medium">Method</th>
                  <th class="pb-2 pr-4 font-medium">Endpoint</th>
                  <th class="pb-2 font-medium">Description</th>
                </tr>
              </thead>
              <tbody class="font-mono text-xs">
                <tr v-for="ep in endpoints" :key="ep.method + ep.path" class="border-b border-[var(--border-subtle)]/50">
                  <td class="py-2 pr-4">
                    <span
                      class="px-1.5 py-0.5 rounded text-xs font-bold"
                      :class="methodColor(ep.method)"
                    >{{ ep.method }}</span>
                  </td>
                  <td class="py-2 pr-4 text-[var(--text-primary)]">{{ ep.path }}</td>
                  <td class="py-2 text-[var(--text-secondary)] font-sans">{{ ep.description }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <!-- Usage Statistics -->
      <section class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
        <div class="px-5 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-hover)]">
          <h3 class="text-sm font-semibold flex items-center gap-2">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
            Usage Statistics
          </h3>
        </div>
        <div class="p-5">
          <div class="grid grid-cols-3 gap-4">
            <div class="bg-[var(--bg-base)] rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-[var(--accent-primary)]">{{ totalCalls }}</div>
              <div class="text-xs text-[var(--text-secondary)] mt-1">Total API Calls</div>
            </div>
            <div class="bg-[var(--bg-base)] rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-green-500">{{ apiKeys.length }}</div>
              <div class="text-xs text-[var(--text-secondary)] mt-1">Active API Keys</div>
            </div>
            <div class="bg-[var(--bg-base)] rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-blue-500">{{ serverStatus.is_running ? serverStatus.port : '-' }}</div>
              <div class="text-xs text-[var(--text-secondary)] mt-1">Server Port</div>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ServerStatus, ApiKeyInfo, ApiKeyCreateResponse } from '@/api/restApi'

const port = ref(3001)
const serverStatus = ref<ServerStatus>({
  is_running: false,
  port: 3001,
  docs_url: null,
  base_url: null,
})
const isToggling = ref(false)

// API Keys
const apiKeys = ref<ApiKeyInfo[]>([])
const showCreateKey = ref(false)
const newKeyName = ref('')
const newKeyPermissions = ref('read')
const newKeyExpires = ref(90)
const isCreatingKey = ref(false)
const newlyCreatedKey = ref<ApiKeyCreateResponse | null>(null)

// Endpoints reference
const endpoints = [
  { method: 'POST', path: '/auth/login', description: 'User login' },
  { method: 'POST', path: '/auth/register', description: 'User registration' },
  { method: 'POST', path: '/auth/refresh', description: 'Refresh access token' },
  { method: 'GET', path: '/projects', description: 'List all projects' },
  { method: 'POST', path: '/projects', description: 'Create a project' },
  { method: 'GET', path: '/projects/{id}', description: 'Get project details' },
  { method: 'PUT', path: '/projects/{id}', description: 'Update project' },
  { method: 'DELETE', path: '/projects/{id}', description: 'Delete project' },
  { method: 'GET', path: '/projects/{id}/files', description: 'List project files' },
  { method: 'POST', path: '/projects/{id}/files', description: 'Create file' },
  { method: 'GET', path: '/files/{id}', description: 'Get file details' },
  { method: 'PUT', path: '/files/{id}', description: 'Update file' },
  { method: 'DELETE', path: '/files/{id}', description: 'Delete file' },
  { method: 'POST', path: '/simulations', description: 'Run simulation' },
  { method: 'GET', path: '/simulations/{id}', description: 'Get simulation status' },
  { method: 'GET', path: '/simulations/{id}/result', description: 'Get simulation result' },
  { method: 'POST', path: '/mesh/generate', description: 'Generate mesh' },
  { method: 'GET', path: '/users/me', description: 'Get user profile' },
  { method: 'PUT', path: '/users/me', description: 'Update user profile' },
  { method: 'POST', path: '/projects/{id}/share', description: 'Share project' },
  { method: 'GET', path: '/projects/{id}/shares', description: 'List project shares' },
  { method: 'GET', path: '/api-keys', description: 'List API keys' },
  { method: 'POST', path: '/api-keys', description: 'Create API key' },
  { method: 'DELETE', path: '/api-keys/{id}', description: 'Delete API key' },
  { method: 'GET', path: '/health', description: 'Health check' },
]

const totalCalls = computed(() => apiKeys.value.reduce((sum, k) => sum + k.call_count, 0))

function methodColor(method: string): string {
  switch (method) {
    case 'GET': return 'bg-green-500/10 text-green-500'
    case 'POST': return 'bg-blue-500/10 text-blue-500'
    case 'PUT': return 'bg-yellow-500/10 text-yellow-500'
    case 'DELETE': return 'bg-red-500/10 text-red-500'
    default: return 'bg-[var(--bg-hover)] text-[var(--text-secondary)]'
  }
}

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleDateString()
  } catch {
    return dateStr
  }
}

function isExpiringSoon(dateStr: string): boolean {
  try {
    const expires = new Date(dateStr).getTime()
    const now = Date.now()
    const sevenDays = 7 * 24 * 60 * 60 * 1000
    return expires - now < sevenDays
  } catch {
    return false
  }
}

async function checkServerStatus() {
  try {
    const status = await invoke<ServerStatus>('get_api_server_status')
    serverStatus.value = status
    if (status.port) {
      port.value = status.port
    }
  } catch (e) {
    console.error('Failed to check server status:', e)
  }
}

async function toggleServer() {
  isToggling.value = true
  try {
    if (serverStatus.value.is_running) {
      await invoke('stop_api_server')
    } else {
      await invoke('start_api_server_cmd', { port: port.value })
    }
    // Wait a moment for server to start/stop
    await new Promise(resolve => setTimeout(resolve, 500))
    await checkServerStatus()
  } catch (e) {
    console.error('Failed to toggle server:', e)
    alert(`Failed to ${serverStatus.value.is_running ? 'stop' : 'start'} server: ${e}`)
  } finally {
    isToggling.value = false
  }
}

async function loadApiKeys() {
  try {
    const token = localStorage.getItem('caelab-access-token')
    if (!token || !serverStatus.value.is_running) return

    const baseUrl = `http://127.0.0.1:${port.value}/api/v1`
    const response = await fetch(`${baseUrl}/api-keys`, {
      headers: { Authorization: `Bearer ${token}` },
    })
    if (response.ok) {
      const data = await response.json()
      if (data.success && data.data) {
        apiKeys.value = data.data
      }
    }
  } catch (e) {
    console.error('Failed to load API keys:', e)
  }
}

async function createApiKey() {
  if (!newKeyName.value.trim()) return
  isCreatingKey.value = true
  try {
    const token = localStorage.getItem('caelab-access-token')
    if (!token) {
      alert('Please log in first')
      return
    }

    const baseUrl = `http://127.0.0.1:${port.value}/api/v1`
    const response = await fetch(`${baseUrl}/api-keys`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        name: newKeyName.value,
        permissions: newKeyPermissions.value,
        expires_days: newKeyExpires.value,
      }),
    })

    const data = await response.json()
    if (data.success && data.data) {
      newlyCreatedKey.value = data.data
      showCreateKey.value = false
      newKeyName.value = ''
      await loadApiKeys()
    } else {
      alert(`Failed to create API key: ${data.error}`)
    }
  } catch (e) {
    console.error('Failed to create API key:', e)
    alert('Failed to create API key. Is the server running?')
  } finally {
    isCreatingKey.value = false
  }
}

async function deleteApiKey(id: string) {
  if (!confirm('Are you sure you want to delete this API key?')) return
  try {
    const token = localStorage.getItem('caelab-access-token')
    if (!token) return

    const baseUrl = `http://127.0.0.1:${port.value}/api/v1`
    const response = await fetch(`${baseUrl}/api-keys/${id}`, {
      method: 'DELETE',
      headers: { Authorization: `Bearer ${token}` },
    })

    if (response.ok) {
      await loadApiKeys()
    }
  } catch (e) {
    console.error('Failed to delete API key:', e)
  }
}

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text)
  } catch {
    // Fallback
    const textarea = document.createElement('textarea')
    textarea.value = text
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
  }
}

onMounted(async () => {
  await checkServerStatus()
  if (serverStatus.value.is_running) {
    await loadApiKeys()
  }
})
</script>
