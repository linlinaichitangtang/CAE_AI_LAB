<template>
  <div class="inp-import-preview" v-if="visible">
    <!-- Modal Overlay -->
    <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center" @click.self="$emit('close')">
      <div class="bg-[var(--bg-surface)] rounded-xl shadow-2xl border border-[var(--border-subtle)] w-full max-w-3xl max-h-[85vh] flex flex-col mx-4">
        <!-- Header -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center">
              <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
              </svg>
            </div>
            <div>
              <h3 class="text-base font-semibold text-[var(--text-primary)]">{{ t('inpImport.title') }}</h3>
              <p class="text-xs text-[var(--text-muted)]" v-if="fileName">{{ fileName }}</p>
            </div>
          </div>
          <button @click="$emit('close')" class="icon-btn">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto px-6 py-4">
          <!-- Loading State -->
          <div v-if="loading" class="flex flex-col items-center justify-center py-12">
            <div class="w-10 h-10 border-3 border-[var(--primary)] border-t-transparent rounded-full animate-spin mb-4"></div>
            <p class="text-sm text-[var(--text-secondary)]">{{ t('common.loading') }}</p>
          </div>

          <!-- Error State -->
          <div v-else-if="error" class="flex flex-col items-center justify-center py-12">
            <div class="w-12 h-12 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <p class="text-sm text-red-500 mb-2">{{ t('common.error') }}</p>
            <p class="text-xs text-[var(--text-muted)]">{{ error }}</p>
          </div>

          <!-- Preview Content -->
          <div v-else-if="result">
            <!-- Summary Cards -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
              <div class="bg-[var(--bg-base)] rounded-lg p-3 border border-[var(--border-subtle)]">
                <div class="text-xs text-[var(--text-muted)] mb-1">{{ t('inpImport.nodes') }}</div>
                <div class="text-lg font-bold text-[var(--text-primary)]">{{ formatNumber(result.num_nodes) }}</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded-lg p-3 border border-[var(--border-subtle)]">
                <div class="text-xs text-[var(--text-muted)] mb-1">{{ t('inpImport.elements') }}</div>
                <div class="text-lg font-bold text-[var(--text-primary)]">{{ formatNumber(result.num_elements) }}</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded-lg p-3 border border-[var(--border-subtle)]">
                <div class="text-xs text-[var(--text-muted)] mb-1">{{ t('inpImport.materials') }}</div>
                <div class="text-lg font-bold text-[var(--text-primary)]">{{ result.num_materials }}</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded-lg p-3 border border-[var(--border-subtle)]">
                <div class="text-xs text-[var(--text-muted)] mb-1">{{ t('inpImport.steps') }}</div>
                <div class="text-lg font-bold text-[var(--text-primary)]">{{ result.num_steps }}</div>
              </div>
            </div>

            <!-- Detailed Sections -->
            <div class="space-y-4">
              <!-- Materials -->
              <div v-if="result.model.materials.length > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.materials') }} ({{ result.model.materials.length }})</h4>
                </div>
                <div class="divide-y divide-[var(--border-subtle)]">
                  <div v-for="mat in result.model.materials" :key="mat.name" class="px-4 py-2.5">
                    <div class="flex items-center justify-between mb-1">
                      <span class="text-sm font-medium text-[var(--text-primary)]">{{ mat.name }}</span>
                      <span class="text-xs px-2 py-0.5 rounded-full bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400">
                        {{ mat.elastic_modulus ? 'Elastic' : 'Custom' }}
                      </span>
                    </div>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-2 text-xs text-[var(--text-secondary)]">
                      <div v-if="mat.elastic_modulus">
                        <span class="text-[var(--text-muted)]">E:</span> {{ mat.elastic_modulus.toExponential(2) }}
                      </div>
                      <div v-if="mat.poisson_ratio">
                        <span class="text-[var(--text-muted)]">v:</span> {{ mat.poisson_ratio }}
                      </div>
                      <div v-if="mat.density">
                        <span class="text-[var(--text-muted)]">rho:</span> {{ mat.density }}
                      </div>
                      <div v-if="mat.yield_stress">
                        <span class="text-[var(--text-muted)]">sigma_y:</span> {{ mat.yield_stress }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Sections -->
              <div v-if="result.model.sections.length > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.sections') }} ({{ result.model.sections.length }})</h4>
                </div>
                <div class="divide-y divide-[var(--border-subtle)]">
                  <div v-for="(sec, idx) in result.model.sections" :key="idx" class="px-4 py-2 flex items-center justify-between text-xs">
                    <span class="text-[var(--text-primary)]">{{ sec.section_type }}</span>
                    <span class="text-[var(--text-secondary)]">{{ sec.material }}</span>
                    <span v-if="sec.element_set" class="text-[var(--text-muted)]">{{ sec.element_set }}</span>
                  </div>
                </div>
              </div>

              <!-- Boundary Conditions -->
              <div v-if="result.model.boundaries.length > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.boundaries') }} ({{ result.model.boundaries.length }})</h4>
                </div>
                <div class="px-4 py-2 text-xs text-[var(--text-secondary)]">
                  <div v-for="(bc, idx) in result.model.boundaries.slice(0, 10)" :key="idx" class="flex items-center gap-2 py-0.5">
                    <span class="text-[var(--text-muted)]">{{ bc.node_set || `Node ${bc.node_id}` }}</span>
                    <span class="text-[var(--text-primary)]">DOF={{ bc.dof }}</span>
                    <span v-if="bc.magnitude !== null && bc.magnitude !== undefined">= {{ bc.magnitude }}</span>
                  </div>
                  <div v-if="result.model.boundaries.length > 10" class="text-[var(--text-muted)] mt-1">
                    ... {{ t('inpImport.andMore', { count: result.model.boundaries.length - 10 }) }}
                  </div>
                </div>
              </div>

              <!-- Loads -->
              <div v-if="result.model.loads.length > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.loads') }} ({{ result.model.loads.length }})</h4>
                </div>
                <div class="px-4 py-2 text-xs text-[var(--text-secondary)]">
                  <div v-for="(ld, idx) in result.model.loads.slice(0, 10)" :key="idx" class="flex items-center gap-2 py-0.5">
                    <span class="px-1.5 py-0.5 rounded bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400">{{ ld.load_type }}</span>
                    <span class="text-[var(--text-muted)]">{{ ld.node_set || ld.element_set || `Node ${ld.node_id}` }}</span>
                    <span v-if="ld.dof" class="text-[var(--text-primary)]">DOF={{ ld.dof }}</span>
                    <span class="text-[var(--text-primary)]">{{ ld.magnitude }}</span>
                  </div>
                  <div v-if="result.model.loads.length > 10" class="text-[var(--text-muted)] mt-1">
                    ... {{ t('inpImport.andMore', { count: result.model.loads.length - 10 }) }}
                  </div>
                </div>
              </div>

              <!-- Steps -->
              <div v-if="result.model.steps.length > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.steps') }} ({{ result.model.steps.length }})</h4>
                </div>
                <div class="divide-y divide-[var(--border-subtle)]">
                  <div v-for="(step, idx) in result.model.steps" :key="idx" class="px-4 py-2 flex items-center justify-between text-xs">
                    <span class="text-[var(--text-primary)]">{{ step.name }}</span>
                    <span class="px-2 py-0.5 rounded-full bg-indigo-100 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400">
                      {{ step.step_type }}
                    </span>
                    <span v-if="step.nlgeom" class="text-orange-500">NLGEOM</span>
                  </div>
                </div>
              </div>

              <!-- Contact Pairs -->
              <div v-if="result.model.contact_pairs.length > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.contactPairs') }} ({{ result.model.contact_pairs.length }})</h4>
                </div>
                <div class="divide-y divide-[var(--border-subtle)]">
                  <div v-for="(cp, idx) in result.model.contact_pairs" :key="idx" class="px-4 py-2 text-xs">
                    <span class="text-[var(--text-primary)]">{{ cp.master_surface }}</span>
                    <span class="text-[var(--text-muted)] mx-2"> <-> </span>
                    <span class="text-[var(--text-primary)]">{{ cp.slave_surface }}</span>
                    <span class="text-[var(--text-muted)] ml-2">({{ cp.interaction }})</span>
                  </div>
                </div>
              </div>

              <!-- Sets -->
              <div v-if="result.num_node_sets > 0 || result.num_element_sets > 0" class="bg-[var(--bg-base)] rounded-lg border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-4 py-2.5 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
                  <h4 class="text-sm font-medium text-[var(--text-primary)]">{{ t('inpImport.sets') }}</h4>
                </div>
                <div class="px-4 py-2 text-xs text-[var(--text-secondary)]">
                  <span>{{ t('inpImport.nodeSets') }}: {{ result.num_node_sets }}</span>
                  <span class="mx-3">|</span>
                  <span>{{ t('inpImport.elementSets') }}: {{ result.num_element_sets }}</span>
                </div>
              </div>

              <!-- Warnings -->
              <div v-if="result.warnings.length > 0" class="bg-yellow-50 dark:bg-yellow-900/20 rounded-lg border border-yellow-200 dark:border-yellow-800 overflow-hidden">
                <div class="px-4 py-2.5 border-b border-yellow-200 dark:border-yellow-800">
                  <h4 class="text-sm font-medium text-yellow-700 dark:text-yellow-400">{{ t('common.warning') }}</h4>
                </div>
                <div class="px-4 py-2">
                  <div v-for="(w, idx) in result.warnings" :key="idx" class="text-xs text-yellow-600 dark:text-yellow-400 py-0.5">
                    {{ w }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div v-if="result" class="flex items-center justify-end gap-3 px-6 py-4 border-t border-[var(--border-subtle)]">
          <button @click="$emit('close')" class="px-4 py-2 rounded-lg border border-[var(--border-subtle)] text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] transition-colors">
            {{ t('common.cancel') }}
          </button>
          <button
            @click="confirmImport"
            :disabled="!result.success"
            class="px-4 py-2 rounded-lg bg-[var(--primary)] text-white text-sm hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ t('common.confirm') }} {{ t('common.import') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'

interface InpNode {
  id: number
  x: number
  y: number
  z: number
}

interface InpElement {
  id: number
  element_type: string
  node_ids: number[]
}

interface InpMaterial {
  name: string
  elastic_modulus: number | null
  poisson_ratio: number | null
  density: number | null
  thermal_expansion: number | null
  thermal_conductivity: number | null
  specific_heat: number | null
  yield_stress: number | null
  plastic_strain: number | null
  raw_properties: Record<string, number[]>
}

interface InpSection {
  section_type: string
  material: string
  element_set: string | null
  thickness: number | null
}

interface InpBoundary {
  node_set: string | null
  node_id: number | null
  dof: string
  magnitude: number | null
  step_name: string | null
}

interface InpLoad {
  load_type: string
  node_set: string | null
  node_id: number | null
  element_set: string | null
  dof: string | null
  magnitude: number
  step_name: string | null
  amplitude: string | null
}

interface InpStep {
  name: string
  step_type: string
  nlgeom: boolean
  description: string | null
}

interface InpContactPair {
  master_surface: string
  slave_surface: string
  interaction: string
  step_name: string | null
}

interface AbaqusInpModel {
  nodes: InpNode[]
  elements: InpElement[]
  materials: InpMaterial[]
  sections: InpSection[]
  boundaries: InpBoundary[]
  loads: InpLoad[]
  steps: InpStep[]
  contact_pairs: InpContactPair[]
  node_sets: Record<string, number[]>
  element_sets: Record<string, number[]>
  amplitudes: any[]
  initial_conditions: any[]
  warnings: string[]
}

interface AbaqusInpImportResult {
  success: boolean
  model: AbaqusInpModel
  num_nodes: number
  num_elements: number
  num_materials: number
  num_sections: number
  num_boundaries: number
  num_loads: number
  num_steps: number
  num_contact_pairs: number
  num_node_sets: number
  num_element_sets: number
  bounding_box: number[] | null
  warnings: string[]
  error: string | null
}

const { t } = useI18n()

const props = defineProps<{
  visible: boolean
  filePath: string | null
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'imported', result: AbaqusInpImportResult): void
}>()

const loading = ref(false)
const error = ref<string | null>(null)
const result = ref<AbaqusInpImportResult | null>(null)
const fileName = ref<string | null>(null)

watch(() => props.filePath, async (newPath) => {
  if (newPath && props.visible) {
    await loadPreview(newPath)
  }
})

watch(() => props.visible, async (isVisible) => {
  if (isVisible && props.filePath) {
    await loadPreview(props.filePath)
  }
  if (!isVisible) {
    result.value = null
    error.value = null
    loading.value = false
  }
})

async function loadPreview(filePath: string) {
  loading.value = true
  error.value = null
  result.value = null
  fileName.value = filePath.split(/[\\/]/).pop() || filePath

  try {
    const res = await invoke<AbaqusInpImportResult>('import_abaqus_inp', { filePath })
    result.value = res
    if (!res.success) {
      error.value = res.error || 'Import failed'
    }
  } catch (e: any) {
    error.value = typeof e === 'string' ? e : e.message || 'Unknown error'
  } finally {
    loading.value = false
  }
}

function confirmImport() {
  if (result.value && result.value.success) {
    emit('imported', result.value)
    emit('close')
  }
}

function formatNumber(n: number): string {
  if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M'
  if (n >= 1000) return (n / 1000).toFixed(1) + 'K'
  return n.toString()
}
</script>

<style scoped>
.inp-import-preview {
  position: fixed;
  inset: 0;
  z-index: 50;
}

.border-3 {
  border-width: 3px;
}
</style>
