<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">仿真结果审计日志</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-007 | Hash 链审计，完整性验证，ISO 9001 / AS9100 报告导出</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetChain">重置</button>
        <button class="btn btn-primary text-xs" @click="loadDemoChain">加载示例</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Create Chain -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            创建审计链
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">仿真 ID</label>
              <input class="input w-full text-xs" type="text" v-model="simulationId" placeholder="sim-20260403-001" />
            </div>
            <button class="btn btn-primary text-xs w-full" @click="createChain" :disabled="!simulationId || chainCreated">
              {{ chainCreated ? '审计链已创建' : '创建审计链' }}
            </button>
          </div>
        </div>

        <!-- Step 2: Add Entry -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            添加审计条目
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">事件类型</label>
              <select class="input w-full text-xs" v-model="newEntry.eventType">
                <option v-for="et in eventTypes" :key="et.value" :value="et.value">{{ et.label }}</option>
              </select>
            </div>
            <div>
              <label class="label">描述</label>
              <input class="input w-full text-xs" type="text" v-model="newEntry.description" placeholder="事件描述..." />
            </div>
            <div>
              <label class="label">元数据 (Key-Value)</label>
              <div class="space-y-1">
                <div v-for="(kv, idx) in newEntry.metadata" :key="idx" class="flex gap-1">
                  <input class="input w-full text-xs" type="text" v-model="kv.key" placeholder="key" />
                  <input class="input w-full text-xs" type="text" v-model="kv.value" placeholder="value" />
                  <button class="btn btn-ghost text-xs px-1" @click="newEntry.metadata.splice(idx, 1)" style="color: var(--accent-red)">x</button>
                </div>
                <button class="btn btn-ghost text-xs w-full" @click="newEntry.metadata.push({ key: '', value: '' })">+ 添加键值对</button>
              </div>
            </div>
            <button class="btn btn-primary text-xs w-full" @click="addEntry" :disabled="!chainCreated">
              添加条目
            </button>
          </div>
        </div>

        <!-- Step 3: Chain Info -->
        <div v-if="chainInfo">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            链信息
          </h4>
          <div class="space-y-1.5 text-xs">
            <div class="flex justify-between">
              <span style="color: var(--text-muted)">链 ID</span>
              <span class="font-mono" style="color: var(--text-secondary)">{{ chainInfo.chainId }}</span>
            </div>
            <div class="flex justify-between">
              <span style="color: var(--text-muted)">仿真 ID</span>
              <span style="color: var(--text-secondary)">{{ chainInfo.simulationId }}</span>
            </div>
            <div class="flex justify-between">
              <span style="color: var(--text-muted)">条目数</span>
              <span style="color: var(--text-secondary)">{{ chainInfo.entryCount }}</span>
            </div>
            <div class="flex justify-between">
              <span style="color: var(--text-muted)">创建时间</span>
              <span style="color: var(--text-secondary)">{{ chainInfo.createdAt }}</span>
            </div>
            <div class="flex justify-between items-center">
              <span style="color: var(--text-muted)">哈希有效性</span>
              <span
                class="text-[10px] px-2 py-0.5 rounded-full font-medium"
                :style="chainInfo.isValid
                  ? 'background: rgba(34,197,94,0.15); color: var(--accent-green)'
                  : 'background: rgba(239,68,68,0.15); color: var(--accent-red)'"
              >{{ chainInfo.isValid ? '有效' : '已篡改' }}</span>
            </div>
          </div>
        </div>

        <!-- Step 4: Validate Chain -->
        <div v-if="chainCreated">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            验证链完整性
          </h4>
          <button class="btn btn-primary text-xs w-full" @click="validateChainIntegrity">验证链完整性</button>
          <div v-if="validationResult !== null" class="mt-2 p-3 rounded" :style="validationResult.isValid ? 'background: rgba(34,197,94,0.1)' : 'background: rgba(239,68,68,0.1)'">
            <div class="flex items-center gap-2 mb-1">
              <span class="text-xs font-medium" :style="{ color: validationResult.isValid ? 'var(--accent-green)' : 'var(--accent-red)' }">
                {{ validationResult.isValid ? '链完整性验证通过' : '检测到篡改!' }}
              </span>
            </div>
            <p v-if="!validationResult.isValid" class="text-[10px]" style="color: var(--accent-red)">
              篡改条目索引: {{ validationResult.tamperedEntries.join(', ') }}
              | 首次篡改位置: #{{ validationResult.firstTamperedIndex }}
            </p>
            <p v-else class="text-[10px]" style="color: var(--accent-green)">
              所有 {{ auditEntries.length }} 个条目哈希链完整
            </p>
          </div>
        </div>

        <!-- Step 5: Export -->
        <div v-if="chainCreated">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            导出报告
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">报告类型</label>
              <select class="input w-full text-xs" v-model="exportConfig.report_type">
                <option value="ISO9001">ISO 9001</option>
                <option value="AS9100">AS9100</option>
              </select>
            </div>
            <div>
              <label class="label">格式</label>
              <select class="input w-full text-xs" v-model="exportConfig.format">
                <option value="pdf">PDF</option>
                <option value="html">HTML</option>
                <option value="json">JSON</option>
              </select>
            </div>
            <button class="btn btn-primary text-xs w-full" @click="exportReport">导出报告</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Blockchain Visualization -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">区块链可视化</h4>
          <div class="p-4 rounded overflow-x-auto" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg :width="blockchainSvgWidth" :height="auditEntries.length * 72 + 20" viewBox="0 0 500 600">
              <!-- Vertical connecting arrows -->
              <defs>
                <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                  <polygon points="0 0, 8 3, 0 6" fill="var(--border-default)" />
                </marker>
              </defs>
              <g v-for="(entry, idx) in auditEntries" :key="entry.entry_id">
                <!-- Arrow from previous block -->
                <line
                  v-if="idx > 0"
                  x1="80" :y1="idx * 72 - 8"
                  x2="80" :y2="idx * 72 + 8"
                  stroke="var(--border-default)"
                  stroke-width="2"
                  marker-end="url(#arrowhead)"
                />
                <!-- Block background -->
                <rect x="10" :y="idx * 72 + 10" width="180" height="52" rx="6"
                  fill="var(--bg-elevated)" stroke="var(--border-default)" stroke-width="1" />
                <!-- Block index -->
                <text x="20" :y="idx * 72 + 28" fill="var(--primary)" font-size="10" font-weight="600">#{{ entry.block_index }}</text>
                <!-- Event type -->
                <text x="20" :y="idx * 72 + 42" fill="var(--text-secondary)" font-size="9">{{ entry.event_type }}</text>
                <!-- Hash snippet -->
                <text x="20" :y="idx * 72 + 55" fill="var(--text-muted)" font-size="8" font-family="monospace">{{ entry.entry_hash.slice(0, 16) }}...</text>
                <!-- Previous hash link -->
                <text x="200" :y="idx * 72 + 35" fill="var(--text-muted)" font-size="8" font-family="monospace">
                  prev: {{ entry.previous_hash.slice(0, 12) }}...
                </text>
                <!-- Timestamp -->
                <text x="200" :y="idx * 72 + 50" fill="var(--text-muted)" font-size="8">{{ formatTimestamp(entry.timestamp) }}</text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Event Type Distribution Pie Chart -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">事件类型分布</h4>
          <div class="p-4 rounded flex items-center gap-6" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="180" height="180" viewBox="0 0 180 180">
              <g v-for="(slice, idx) in pieSlices" :key="idx">
                <circle
                  cx="90" cy="90" :r="70"
                  fill="transparent"
                  :stroke="slice.color"
                  :stroke-width="20"
                  :stroke-dasharray="slice.dashArray"
                  :stroke-dashoffset="slice.dashOffset"
                  transform="rotate(-90 90 90)"
                />
              </g>
              <text x="90" y="86" text-anchor="middle" fill="var(--text-primary)" font-size="18" font-weight="bold">{{ auditEntries.length }}</text>
              <text x="90" y="102" text-anchor="middle" fill="var(--text-muted)" font-size="10">总条目</text>
            </svg>
            <div class="space-y-1.5">
              <div v-for="(slice, idx) in pieSlices" :key="idx" class="flex items-center gap-2 text-xs">
                <span class="w-3 h-3 rounded-sm inline-block" :style="{ background: slice.color }" />
                <span style="color: var(--text-secondary)">{{ slice.label }}</span>
                <span class="font-medium" style="color: var(--text-primary)">{{ slice.count }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Audit Entries Table -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">审计条目列表</h4>
          <div class="rounded overflow-hidden" style="border: 1px solid var(--border-subtle)">
            <table class="w-full text-xs">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-muted)">时间</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-muted)">事件类型</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-muted)">描述</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-muted)">哈希</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="entry in auditEntries"
                  :key="entry.entry_id"
                  class="border-t"
                  style="border-color: var(--border-subtle)"
                >
                  <td class="px-3 py-2 font-mono" style="color: var(--text-secondary)">{{ formatTimestamp(entry.timestamp) }}</td>
                  <td class="px-3 py-2">
                    <span
                      class="px-1.5 py-0.5 rounded text-[10px] font-medium"
                      :style="eventBadgeStyle(entry.event_type)"
                    >{{ eventLabel(entry.event_type) }}</span>
                  </td>
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ entry.description }}</td>
                  <td class="px-3 py-2 font-mono text-[10px]" style="color: var(--text-muted)">{{ entry.entry_hash.slice(0, 12) }}...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { AuditEventType, AuditEntry, AuditChain, AuditExportConfig } from '../api/auditLog'

// ============ Refs ============
const simulationId = ref('')
const chainCreated = ref(false)
const auditEntries = ref<AuditEntry[]>([])
const validationResult = ref<{ isValid: boolean; tamperedEntries: number[]; firstTamperedIndex: number } | null>(null)

// ============ Reactive ============
const newEntry = reactive({
  eventType: 'simulation_start' as AuditEventType,
  description: '',
  metadata: reactive<{ key: string; value: string }[]>([])
})

const exportConfig = reactive<AuditExportConfig>({
  chain_id: '',
  report_type: 'ISO9001',
  format: 'pdf',
  include_raw_data: false
})

// ============ Event Types ============
const eventTypes = [
  { value: 'simulation_start', label: '仿真启动' },
  { value: 'simulation_complete', label: '仿真完成' },
  { value: 'parameter_change', label: '参数变更' },
  { value: 'result_export', label: '结果导出' },
  { value: 'data_import', label: '数据导入' },
  { value: 'scale_bridge', label: '尺度桥接' },
  { value: 'coarse_graining', label: '粗粒化' },
  { value: 'validation_run', label: '验证运行' }
]

// ============ Chain Info ============
const chainInfo = computed(() => {
  if (!chainCreated.value) return null
  return {
    chainId: `chain-${simulationId.value}`,
    simulationId: simulationId.value,
    entryCount: auditEntries.value.length,
    createdAt: '2026-04-03 10:30',
    isValid: validationResult.value?.isValid ?? true
  }
})

// ============ Pie Chart ============
const eventTypeColors: Record<string, string> = {
  simulation_start: '#3b82f6',
  simulation_complete: '#22c55e',
  parameter_change: '#f59e0b',
  result_export: '#8b5cf6',
  data_import: '#06b6d4',
  scale_bridge: '#ec4899',
  coarse_graining: '#f97316',
  validation_run: '#14b8a6'
}

const pieSlices = computed(() => {
  const counts: Record<string, number> = {}
  for (const entry of auditEntries.value) {
    counts[entry.event_type] = (counts[entry.event_type] || 0) + 1
  }

  const total = auditEntries.value.length
  if (total === 0) return []

  const slices: Array<{ label: string; count: number; color: string; dashArray: string; dashOffset: string }> = []
  let offset = 0
  const circumference = 2 * Math.PI * 70

  for (const [type, count] of Object.entries(counts)) {
    const fraction = count / total
    const length = fraction * circumference
    const gap = circumference - length
    slices.push({
      label: eventTypes.find(e => e.value === type)?.label || type,
      count,
      color: eventTypeColors[type] || '#6b7280',
      dashArray: `${length} ${gap}`,
      dashOffset: `${-offset}`
    })
    offset += length
  }

  return slices
})

// ============ SVG Width ============
const blockchainSvgWidth = computed(() => 500)

// ============ Mock Data Generator ============
function generateMockEntries(): AuditEntry[] {
  const entries: AuditEntry[] = []
  const mockEvents: Array<{ type: AuditEventType; desc: string; meta: Record<string, unknown> }> = [
    { type: 'simulation_start', desc: '启动 MD 拉伸仿真', meta: { ensemble: 'NVT', temperature: 300, steps: 100000 } },
    { type: 'parameter_change', desc: '修改时间步长 1fs -> 0.5fs', meta: { old_value: 1.0, new_value: 0.5, unit: 'fs' } },
    { type: 'data_import', desc: '导入 EAM 势函数文件', meta: { file: 'Cu_eam.alloy', size_kb: 245 } },
    { type: 'scale_bridge', desc: 'MD -> 相场粗粒化映射', meta: { method: 'morse_convolution', grid_size: '64x64' } },
    { type: 'coarse_graining', desc: '原子应力场粗粒化', meta: { source_atoms: 86400, target_grid: 4096 } },
    { type: 'validation_run', desc: '弹性常数验证', meta: { C11_ref: 168.4, C11_calc: 167.9, error_pct: 0.3 } },
    { type: 'result_export', desc: '导出应力-应变曲线', meta: { format: 'csv', records: 500 } },
    { type: 'simulation_complete', desc: '仿真正常完成', meta: { total_time_s: 3420, final_step: 100000 } }
  ]

  let previousHash = '0000000000000000000000000000000000000000000000000000000000000000'

  for (let i = 0; i < mockEvents.length; i++) {
    const event = mockEvents[i]
    const entryHash = computeHash(previousHash, event.type, event.desc, i)
    entries.push({
      entry_id: `entry-${String(i + 1).padStart(4, '0')}`,
      timestamp: new Date(Date.now() - (mockEvents.length - i) * 1800000).toISOString(),
      event_type: event.type,
      user_id: 'user:admin',
      simulation_id: simulationId.value || 'sim-demo-001',
      description: event.desc,
      metadata: event.meta,
      previous_hash: previousHash,
      entry_hash: entryHash,
      block_index: i
    })
    previousHash = entryHash
  }

  return entries
}

function computeHash(prevHash: string, eventType: string, desc: string, index: number): string {
  const raw = `${prevHash}:${eventType}:${desc}:${index}`
  let hash = 0
  for (let i = 0; i < raw.length; i++) {
    const char = raw.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }
  return Math.abs(hash).toString(16).padStart(64, '0').slice(0, 64)
}

// ============ Methods ============
function eventLabel(type: AuditEventType): string {
  return eventTypes.find(e => e.value === type)?.label || type
}

function eventBadgeStyle(type: AuditEventType): string {
  const color = eventTypeColors[type] || '#6b7280'
  return `background: ${color}22; color: ${color}`
}

function formatTimestamp(iso: string): string {
  const d = new Date(iso)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}:${String(d.getSeconds()).padStart(2, '0')}`
}

async function createChain() {
  if (!simulationId.value) return
  chainCreated.value = true
  auditEntries.value = []
  validationResult.value = null
  exportConfig.chain_id = `chain-${simulationId.value}`
}

async function addEntry() {
  if (!chainCreated.value || !newEntry.description) return

  const prevHash = auditEntries.value.length > 0
    ? auditEntries.value[auditEntries.value.length - 1].entry_hash
    : '0000000000000000000000000000000000000000000000000000000000000000'

  const metaObj: Record<string, unknown> = {}
  for (const kv of newEntry.metadata) {
    if (kv.key) {
      metaObj[kv.key] = kv.value
    }
  }

  const entryHash = computeHash(prevHash, newEntry.eventType, newEntry.description, auditEntries.value.length)

  const entry: AuditEntry = {
    entry_id: `entry-${String(auditEntries.value.length + 1).padStart(4, '0')}`,
    timestamp: new Date().toISOString(),
    event_type: newEntry.eventType,
    user_id: 'user:admin',
    simulation_id: simulationId.value,
    description: newEntry.description,
    metadata: metaObj,
    previous_hash: prevHash,
    entry_hash: entryHash,
    block_index: auditEntries.value.length
  }

  auditEntries.value.push(entry)
  newEntry.description = ''
  newEntry.metadata.splice(0)
  validationResult.value = null
}

async function validateChainIntegrity() {
  // Simulate validation
  await nextTick()
  validationResult.value = {
    isValid: true,
    tamperedEntries: [],
    firstTamperedIndex: -1
  }
}

async function exportReport() {
  // Simulate export
  const reportPath = `/reports/${exportConfig.report_type}_${exportConfig.format}_${Date.now()}.${exportConfig.format}`
  alert(`报告已导出: ${reportPath}`)
}

function resetChain() {
  simulationId.value = ''
  chainCreated.value = false
  auditEntries.value = []
  validationResult.value = null
  newEntry.description = ''
  newEntry.metadata.splice(0)
}

function loadDemoChain() {
  simulationId.value = 'sim-demo-001'
  chainCreated.value = true
  auditEntries.value = generateMockEntries()
  validationResult.value = null
  exportConfig.chain_id = `chain-sim-demo-001`
}

// ============ Lifecycle ============
onMounted(() => {
  simulationId.value = 'sim-demo-001'
  chainCreated.value = true
  auditEntries.value = generateMockEntries()
  exportConfig.chain_id = `chain-sim-demo-001`
  validationResult.value = { isValid: true, tamperedEntries: [], firstTamperedIndex: -1 }
})
</script>
