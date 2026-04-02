<template>
  <div class="h-full overflow-y-auto p-6">
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <div>
          <h2 class="text-xl font-bold text-[var(--text-primary)]">{{ t('plugin.title') }}</h2>
          <p class="text-sm text-[var(--text-secondary)] mt-1">{{ t('plugin.installed') }}</p>
        </div>
        <div class="flex items-center gap-2">
          <button
            @click="triggerLoadPlugin"
            class="btn btn-primary flex items-center gap-2 text-sm"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span>{{ t('plugin.loadPlugin') }}</span>
          </button>
        </div>
        <input
          ref="fileInput"
          type="file"
          class="hidden"
          accept=".so,.dll,.dylib"
          @change="handleFileSelect"
        />
      </div>

      <!-- Plugin List -->
      <div v-if="plugins.length > 0" class="space-y-3 mb-8">
        <div
          v-for="plugin in plugins"
          :key="plugin.name"
          class="bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] overflow-hidden"
        >
          <!-- Plugin Header -->
          <div class="px-4 py-3 flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-purple-500 to-indigo-600 flex items-center justify-center">
                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z" />
                </svg>
              </div>
              <div>
                <h3 class="text-sm font-semibold text-[var(--text-primary)]">{{ plugin.name }}</h3>
                <p class="text-xs text-[var(--text-muted)]">{{ plugin.description }}</p>
              </div>
            </div>
            <div class="flex items-center gap-3">
              <span class="text-xs px-2 py-1 rounded-full" :class="plugin.loaded ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400' : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400'">
                {{ plugin.loaded ? t('plugin.loaded') : t('plugin.unloaded') }}
              </span>
              <span class="text-xs text-[var(--text-muted)]">v{{ plugin.version }}</span>
              <button
                @click="handleUnload(plugin.name)"
                class="text-xs px-3 py-1 rounded border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
              >
                {{ t('plugin.uninstall') }}
              </button>
            </div>
          </div>

          <!-- Plugin Details (expandable) -->
          <div v-if="selectedPlugin === plugin.name" class="border-t border-[var(--border-subtle)] px-4 py-3">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-xs">
              <div v-if="plugin.registered_solvers.length > 0">
                <div class="text-[var(--text-muted)] mb-1">{{ t('plugin.solvers') }}</div>
                <div v-for="s in plugin.registered_solvers" :key="s" class="text-[var(--text-primary)]">{{ s }}</div>
              </div>
              <div v-if="plugin.registered_importers.length > 0">
                <div class="text-[var(--text-muted)] mb-1">{{ t('plugin.importers') }}</div>
                <div v-for="imp in plugin.registered_importers" :key="imp" class="text-[var(--text-primary)]">{{ imp }}</div>
              </div>
              <div v-if="plugin.registered_exporters.length > 0">
                <div class="text-[var(--text-muted)] mb-1">{{ t('plugin.exporters') }}</div>
                <div v-for="exp in plugin.registered_exporters" :key="exp" class="text-[var(--text-primary)]">{{ exp }}</div>
              </div>
              <div v-if="plugin.registered_materials.length > 0">
                <div class="text-[var(--text-muted)] mb-1">{{ t('plugin.materials') }}</div>
                <div v-for="mat in plugin.registered_materials" :key="mat" class="text-[var(--text-primary)]">{{ mat }}</div>
              </div>
            </div>
            <div v-if="plugin.path" class="mt-2 text-xs text-[var(--text-muted)]">
              {{ t('plugin.path') }}: {{ plugin.path }}
            </div>
          </div>

          <!-- Click to expand -->
          <div
            v-if="hasDetails(plugin)"
            class="px-4 py-2 border-t border-[var(--border-subtle)] cursor-pointer hover:bg-[var(--bg-hover)] transition-colors"
            @click="toggleDetails(plugin.name)"
          >
            <span class="text-xs text-[var(--accent-primary)]">
              {{ selectedPlugin === plugin.name ? '- Collapse' : '+ ' + t('plugin.details') }}
            </span>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-16">
        <div class="w-16 h-16 rounded-full bg-[var(--bg-elevated)] flex items-center justify-center mx-auto mb-4">
          <svg class="w-8 h-8 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z" />
          </svg>
        </div>
        <p class="text-sm text-[var(--text-muted)]">{{ t('plugin.noPlugins') }}</p>
        <p class="text-xs text-[var(--text-muted)] mt-1">{{ t('plugin.selectPluginFile') }}</p>
      </div>

      <!-- Plugin Marketplace Placeholder -->
      <div class="mt-8 bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] p-6 text-center">
        <div class="w-12 h-12 rounded-full bg-gradient-to-br from-amber-400 to-orange-500 flex items-center justify-center mx-auto mb-3">
          <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 100 4 2 2 0 000-4z" />
          </svg>
        </div>
        <h3 class="text-base font-semibold text-[var(--text-primary)] mb-1">{{ t('plugin.marketplace') }}</h3>
        <p class="text-sm text-[var(--text-muted)]">{{ t('plugin.marketplaceSoon') }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'

interface PluginInfo {
  name: string
  version: string
  description: string
  author: string | null
  loaded: boolean
  path: string | null
  registered_solvers: string[]
  registered_importers: string[]
  registered_exporters: string[]
  registered_materials: string[]
}

const { t } = useI18n()

const plugins = ref<PluginInfo[]>([])
const selectedPlugin = ref<string | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)

onMounted(() => {
  loadPlugins()
})

async function loadPlugins() {
  try {
    plugins.value = await invoke<PluginInfo[]>('list_plugins')
  } catch (e) {
    console.error('Failed to load plugins:', e)
  }
}

function triggerLoadPlugin() {
  fileInput.value?.click()
}

async function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  try {
    const info = await invoke<PluginInfo>('load_plugin', { path: (file as any).path || file.name })
    plugins.value.push(info)
  } catch (e: any) {
    alert(typeof e === 'string' ? e : e.message || t('plugin.loadFailed'))
  }

  // Reset input
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}

async function handleUnload(name: string) {
  try {
    await invoke('unload_plugin', { name })
    plugins.value = plugins.value.filter(p => p.name !== name)
    if (selectedPlugin.value === name) {
      selectedPlugin.value = null
    }
  } catch (e: any) {
    alert(typeof e === 'string' ? e : e.message || t('plugin.unloadFailed'))
  }
}

function toggleDetails(name: string) {
  selectedPlugin.value = selectedPlugin.value === name ? null : name
}

function hasDetails(plugin: PluginInfo): boolean {
  return plugin.registered_solvers.length > 0
    || plugin.registered_importers.length > 0
    || plugin.registered_exporters.length > 0
    || plugin.registered_materials.length > 0
    || plugin.path !== null
}
</script>
