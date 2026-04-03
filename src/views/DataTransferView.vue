<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">尺度间数据传递引擎</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-002 | 跨尺度数据映射、转换与验证</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetAll">重置</button>
        <button class="btn btn-primary text-xs" @click="showHistory = !showHistory">历史记录</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Direction Selector -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            传递方向
          </h4>
          <div class="space-y-2">
            <select class="input w-full text-xs" v-model="selectedDirection" @change="onDirectionChange">
              <option value="dft_to_md">DFT -> MD</option>
              <option value="md_to_phase_field">MD -> Phase Field</option>
              <option value="phase_field_to_fe">Phase Field -> FE</option>
              <option value="md_to_fe">MD -> FE</option>
              <option value="dft_to_phase_field">DFT -> Phase Field</option>
              <option value="dft_to_fe">DFT -> FE</option>
            </select>
          </div>
        </div>

        <!-- Step 2: Source Scale Output Schema -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            源尺度输出模式
          </h4>
          <div class="rounded overflow-hidden" style="border: 1px solid var(--border-subtle)">
            <table class="w-full text-[10px]">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-2 py-1.5 text-left font-medium" style="color: var(--text-secondary)">字段名</th>
                  <th class="px-2 py-1.5 text-left font-medium" style="color: var(--text-secondary)">类型</th>
                  <th class="px-2 py-1.5 text-left font-medium" style="color: var(--text-secondary)">维度</th>
                  <th class="px-2 py-1.5 text-left font-medium" style="color: var(--text-secondary)">单位</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="field in sourceSchema.fields"
                  :key="field.name"
                  style="border-top: 1px solid var(--border-subtle)"
                >
                  <td class="px-2 py-1.5" style="color: var(--text-primary)">{{ field.name }}</td>
                  <td class="px-2 py-1.5" style="color: var(--text-secondary)">{{ field.type }}</td>
                  <td class="px-2 py-1.5" style="color: var(--text-secondary)">{{ field.dimensions.join('x') }}</td>
                  <td class="px-2 py-1.5" style="color: var(--text-muted)">{{ field.unit }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Step 3: Field Mapping Config -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            字段映射配置
          </h4>
          <div class="space-y-2">
            <div class="flex gap-2">
              <div class="flex-1">
                <label class="label">源字段</label>
                <select class="input w-full text-xs" v-model="mappingSource">
                  <option value="">-- 源 --</option>
                  <option v-for="f in sourceSchema.fields" :key="f.name" :value="f.name">{{ f.name }}</option>
                </select>
              </div>
              <div class="flex-1">
                <label class="label">目标字段</label>
                <select class="input w-full text-xs" v-model="mappingTarget">
                  <option value="">-- 目标 --</option>
                  <option v-for="f in targetSchema.fields" :key="f.name" :value="f.name">{{ f.name }}</option>
                </select>
              </div>
            </div>
            <div class="flex gap-2">
              <div class="flex-1">
                <label class="label">转换类型</label>
                <select class="input w-full text-xs" v-model="mappingTransformation">
                  <option value="direct">直接映射</option>
                  <option value="interpolation">插值转换</option>
                  <option value="homogenization">均匀化</option>
                  <option value="averaging">平均化</option>
                  <option value="custom">自定义函数</option>
                </select>
              </div>
              <div class="flex-1">
                <label class="label">单位转换因子</label>
                <input class="input w-full text-xs" type="number" v-model.number="mappingUnitFactor" placeholder="1.0" />
              </div>
            </div>
            <button class="btn btn-primary text-xs w-full" @click="addMapping">添加映射</button>
          </div>

          <!-- Current Mappings -->
          <div class="mt-3 space-y-1.5" v-if="fieldMappings.length > 0">
            <div
              v-for="(mapping, idx) in fieldMappings"
              :key="idx"
              class="flex items-center justify-between p-2 rounded text-[10px]"
              style="background: var(--bg-elevated)"
            >
              <div class="flex items-center gap-1.5">
                <span style="color: var(--text-primary)">{{ mapping.source_field }}</span>
                <span style="color: var(--text-muted)">-></span>
                <span style="color: var(--text-primary)">{{ mapping.target_field }}</span>
                <span class="px-1 rounded" style="background: var(--bg-base); color: var(--text-muted)">{{ mapping.transformation }}</span>
              </div>
              <button class="text-[10px] px-1" style="color: var(--accent-red)" @click="removeMapping(idx)">x</button>
            </div>
          </div>
        </div>

        <!-- Step 4: Actions -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            操作
          </h4>
          <div class="space-y-2">
            <button class="btn btn-ghost text-xs w-full" @click="validateTransfer">验证传递</button>
            <button class="btn btn-primary text-xs w-full" @click="executeTransfer">执行传递</button>
          </div>
        </div>

        <!-- Step 5: Transfer Result -->
        <div v-if="transferResult">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" :style="transferResult.success ? 'background: var(--accent-green)' : 'background: var(--accent-red)'">5</span>
            传递结果
          </h4>
          <div class="p-3 rounded space-y-2" style="background: var(--bg-elevated)">
            <div class="text-xs font-medium" :style="transferResult.success ? 'color: var(--accent-green)' : 'color: var(--accent-red)'">
              {{ transferResult.success ? '传递成功' : '传递失败' }}
              <span class="text-[10px] ml-1" style="color: var(--text-muted)">耗时 {{ transferResult.transfer_time_ms }}ms</span>
            </div>
            <div class="space-y-1">
              <div
                v-for="field in transferResult.transferred_fields"
                :key="field.field"
                class="flex items-center justify-between text-[10px] p-1.5 rounded"
                style="background: var(--bg-base)"
              >
                <span style="color: var(--text-primary)">{{ field.field }}</span>
                <div class="flex items-center gap-2">
                  <span style="color: var(--text-muted)">{{ field.status === 'success' ? '成功' : field.status === 'warning' ? '警告' : '错误' }}</span>
                  <span
                    class="px-1 rounded text-[9px]"
                    :style="field.status === 'success' ? 'background: rgba(34,197,94,0.15); color: var(--accent-green)' : field.status === 'warning' ? 'background: rgba(245,158,11,0.15); color: var(--accent-yellow)' : 'background: rgba(239,68,68,0.15); color: var(--accent-red)'"
                  >{{ field.status }}</span>
                </div>
              </div>
            </div>
            <div v-if="transferResult.validation_errors.length > 0" class="space-y-0.5">
              <div v-for="err in transferResult.validation_errors" :key="err" class="text-[10px]" style="color: var(--accent-red)">- {{ err }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- SVG Data Flow Diagram -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">数据流图</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="220" viewBox="0 0 640 220">
              <defs>
                <marker id="dt-flow-arrow" markerWidth="10" markerHeight="7" refX="10" refY="3.5" orient="auto">
                  <polygon points="0 0, 10 3.5, 0 7" fill="var(--primary)" />
                </marker>
                <linearGradient id="dt-flow-grad" x1="0%" y1="0%" x2="100%" y2="0%">
                  <stop offset="0%" style="stop-color: var(--primary); stop-opacity: 0.8" />
                  <stop offset="100%" style="stop-color: var(--primary); stop-opacity: 0.3" />
                </linearGradient>
              </defs>
              <!-- Source Scale Box -->
              <rect x="30" y="60" width="160" height="100" rx="8" fill="#3b82f6" stroke="#2563eb" stroke-width="1.5" />
              <text x="110" y="95" text-anchor="middle" fill="white" font-size="14" font-weight="600">{{ sourceScaleLabel }}</text>
              <text x="110" y="115" text-anchor="middle" fill="rgba(255,255,255,0.7)" font-size="10">源尺度</text>
              <text x="110" y="140" text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="9">{{ sourceSchema.fields.length }} 个输出字段</text>

              <!-- Transformation Arrow -->
              <rect x="220" y="85" width="200" height="50" rx="6" fill="url(#dt-flow-grad)" opacity="0.3" />
              <line x1="195" y1="110" x2="445" y2="110" stroke="var(--primary)" stroke-width="2" marker-end="url(#dt-flow-arrow)" />
              <text x="320" y="100" text-anchor="middle" fill="var(--primary)" font-size="10" font-weight="500">数据转换</text>
              <text x="320" y="125" text-anchor="middle" fill="var(--text-muted)" font-size="9">
                {{ fieldMappings.length }} 个字段映射
              </text>

              <!-- Target Scale Box -->
              <rect x="450" y="60" width="160" height="100" rx="8" fill="#22c55e" stroke="#16a34a" stroke-width="1.5" />
              <text x="530" y="95" text-anchor="middle" fill="white" font-size="14" font-weight="600">{{ targetScaleLabel }}</text>
              <text x="530" y="115" text-anchor="middle" fill="rgba(255,255,255,0.7)" font-size="10">目标尺度</text>
              <text x="530" y="140" text-anchor="middle" fill="rgba(255,255,255,0.5)" font-size="9">{{ targetSchema.fields.length }} 个输入字段</text>

              <!-- Field labels on arrow -->
              <g v-for="(mapping, idx) in fieldMappings.slice(0, 4)" :key="idx">
                <text :x="320" :y="145 + idx * 14" text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ mapping.source_field }} -> {{ mapping.target_field }}
                </text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Transfer Rules Table -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            传递规则表
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ fieldMappings.length }})</span>
          </h4>
          <div class="rounded overflow-hidden" style="border: 1px solid var(--border-subtle)">
            <table class="w-full text-[11px]">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">源字段</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">目标字段</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">转换方式</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">单位转换</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(mapping, idx) in fieldMappings"
                  :key="idx"
                  style="border-top: 1px solid var(--border-subtle)"
                >
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ mapping.source_field }}</td>
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ mapping.target_field }}</td>
                  <td class="px-3 py-2" style="color: var(--text-secondary)">{{ mapping.transformation }}</td>
                  <td class="px-3 py-2" style="color: var(--text-muted)">x{{ mapping.unit_conversion.factor }}</td>
                </tr>
                <tr v-if="fieldMappings.length === 0">
                  <td colspan="4" class="px-3 py-4 text-center" style="color: var(--text-muted)">暂无映射规则</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Transfer History -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            传递历史
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ transferHistory.length }})</span>
          </h4>
          <div class="space-y-2">
            <div
              v-for="(record, idx) in transferHistory"
              :key="idx"
              class="p-3 rounded"
              style="background: var(--bg-surface); border: 1px solid var(--border-subtle)"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ directionLabel(record.direction) }}</span>
                <span
                  class="text-[9px] px-1.5 py-0.5 rounded"
                  :style="record.success ? 'background: rgba(34,197,94,0.15); color: var(--accent-green)' : 'background: rgba(239,68,68,0.15); color: var(--accent-red)'"
                >{{ record.success ? '成功' : '失败' }}</span>
              </div>
              <div class="flex items-center gap-3 text-[10px]" style="color: var(--text-muted)">
                <span>{{ record.fields_count }} 个字段</span>
                <span>{{ record.transfer_time_ms }}ms</span>
                <span>{{ record.timestamp }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import type { DataTransferDirection, ScaleOutputSchema, DataTransferRule, DataTransferResult, DataTransferConfig } from '../api/dataTransfer'

// ============ State ============
const selectedDirection = ref<DataTransferDirection>('dft_to_md')
const showHistory = ref(false)
const transferResult = ref<DataTransferResult | null>(null)

const mappingSource = ref('')
const mappingTarget = ref('')
const mappingTransformation = ref('direct')
const mappingUnitFactor = ref(1.0)

const fieldMappings = ref<Array<{
  source_field: string
  target_field: string
  transformation: string
  unit_conversion: { factor: number; offset: number }
}>>([])

const transferHistory = ref<Array<{
  direction: DataTransferDirection
  fields_count: number
  success: boolean
  transfer_time_ms: number
  timestamp: string
}>>([])

// ============ Schema Data ============
const sourceSchema = reactive<ScaleOutputSchema>({
  scale: 'dft',
  version: '1.0',
  fields: [
    { name: 'stacking_fault_energy', type: 'scalar', dimensions: [1], unit: 'mJ/m^2', description: '层错能' },
    { name: 'elastic_constants', type: 'tensor', dimensions: [3, 3], unit: 'GPa', description: '弹性常数矩阵' },
    { name: 'lattice_parameter', type: 'scalar', dimensions: [1], unit: 'Angstrom', description: '晶格常数' },
    { name: 'formation_energy', type: 'scalar', dimensions: [1], unit: 'eV', description: '形成能' },
    { name: 'surface_energy', type: 'vector', dimensions: [3], unit: 'J/m^2', description: '表面能' }
  ],
  required_fields: ['stacking_fault_energy', 'elastic_constants']
})

const targetSchema = reactive<ScaleOutputSchema>({
  scale: 'md',
  version: '1.0',
  fields: [
    { name: 'sfe_input', type: 'scalar', dimensions: [1], unit: 'mJ/m^2', description: '层错能输入' },
    { name: 'elastic_params', type: 'tensor', dimensions: [3, 3], unit: 'GPa', description: '弹性参数' },
    { name: 'lattice_const', type: 'scalar', dimensions: [1], unit: 'Angstrom', description: '晶格常数' },
    { name: 'potential_params', type: 'vector', dimensions: [10], unit: 'eV', description: '势函数参数' },
    { name: 'initial_config', type: 'tensor', dimensions: [1000, 3], unit: 'Angstrom', description: '初始构型' }
  ],
  required_fields: ['sfe_input', 'elastic_params']
})

// ============ Computed ============
const sourceScaleLabel = computed(() => {
  const map: Record<string, string> = {
    dft: 'DFT',
    md: 'MD',
    phase_field: 'Phase Field',
    fe: 'FE'
  }
  return map[sourceSchema.scale] || sourceSchema.scale
})

const targetScaleLabel = computed(() => {
  const map: Record<string, string> = {
    dft: 'DFT',
    md: 'MD',
    phase_field: 'Phase Field',
    fe: 'FE'
  }
  return map[targetSchema.scale] || targetSchema.scale
})

// ============ Helpers ============
function directionLabel(dir: DataTransferDirection): string {
  const map: Record<DataTransferDirection, string> = {
    dft_to_md: 'DFT -> MD',
    md_to_phase_field: 'MD -> Phase Field',
    phase_field_to_fe: 'Phase Field -> FE',
    md_to_fe: 'MD -> FE',
    dft_to_phase_field: 'DFT -> Phase Field',
    dft_to_fe: 'DFT -> FE'
  }
  return map[dir] || dir
}

function directionToScales(dir: DataTransferDirection): { source: string; target: string } {
  const map: Record<DataTransferDirection, { source: string; target: string }> = {
    dft_to_md: { source: 'dft', target: 'md' },
    md_to_phase_field: { source: 'md', target: 'phase_field' },
    phase_field_to_fe: { source: 'phase_field', target: 'fe' },
    md_to_fe: { source: 'md', target: 'fe' },
    dft_to_phase_field: { source: 'dft', target: 'phase_field' },
    dft_to_fe: { source: 'dft', target: 'fe' }
  }
  return map[dir]
}

// ============ Actions ============
function onDirectionChange() {
  const scales = directionToScales(selectedDirection.value)
  sourceSchema.scale = scales.source as 'dft' | 'md' | 'phase_field' | 'fe'
  targetSchema.scale = scales.target as 'dft' | 'md' | 'phase_field' | 'fe'
  fieldMappings.value = []
  transferResult.value = null
  mappingSource.value = ''
  mappingTarget.value = ''
}

function addMapping() {
  if (!mappingSource.value || !mappingTarget.value) return
  const existing = fieldMappings.value.find(
    m => m.source_field === mappingSource.value && m.target_field === mappingTarget.value
  )
  if (existing) return
  fieldMappings.value.push({
    source_field: mappingSource.value,
    target_field: mappingTarget.value,
    transformation: mappingTransformation.value,
    unit_conversion: { factor: mappingUnitFactor.value, offset: 0 }
  })
  mappingSource.value = ''
  mappingTarget.value = ''
}

function removeMapping(idx: number) {
  fieldMappings.value.splice(idx, 1)
}

function validateTransfer() {
  if (fieldMappings.value.length === 0) {
    transferResult.value = {
      success: false,
      direction: selectedDirection.value,
      transferred_fields: [],
      validation_errors: ['未配置任何字段映射'],
      warnings: [],
      transfer_time_ms: 0
    }
    return
  }
  const errors: string[] = []
  const warnings: string[] = []
  for (const mapping of fieldMappings.value) {
    const sourceField = sourceSchema.fields.find(f => f.name === mapping.source_field)
    const targetField = targetSchema.fields.find(f => f.name === mapping.target_field)
    if (!sourceField) {
      errors.push(`源字段 ${mapping.source_field} 不存在`)
    }
    if (!targetField) {
      errors.push(`目标字段 ${mapping.target_field} 不存在`)
    }
    if (sourceField && targetField) {
      if (mapping.unit_conversion.factor !== 1.0 && sourceField.unit === targetField.unit) {
        warnings.push(`${mapping.source_field} -> ${mapping.target_field}: 单位相同但设置了转换因子`)
      }
    }
  }
  transferResult.value = {
    success: errors.length === 0,
    direction: selectedDirection.value,
    transferred_fields: fieldMappings.value.map(m => ({
      field: m.source_field,
      source_value: null,
      target_value: null,
      status: 'success' as const,
      message: '验证通过'
    })),
    validation_errors: errors,
    warnings,
    transfer_time_ms: 0
  }
}

function executeTransfer() {
  if (fieldMappings.value.length === 0) return
  const transferredFields = fieldMappings.value.map(m => ({
    field: m.source_field,
    source_value: generateMockValue(m.source_field),
    target_value: generateMockValue(m.target_field),
    status: 'success' as const,
    message: '传递成功'
  }))
  transferResult.value = {
    success: true,
    direction: selectedDirection.value,
    transferred_fields: transferredFields,
    validation_errors: [],
    warnings: [],
    transfer_time_ms: Math.floor(Math.random() * 200) + 50
  }
  transferHistory.value.unshift({
    direction: selectedDirection.value,
    fields_count: fieldMappings.value.length,
    success: true,
    transfer_time_ms: transferResult.value.transfer_time_ms,
    timestamp: new Date().toLocaleString('zh-CN')
  })
}

function generateMockValue(fieldName: string): unknown {
  if (fieldName.includes('energy')) return 150.5
  if (fieldName.includes('elastic')) return [[180, 80, 80], [80, 180, 80], [80, 80, 180]]
  if (fieldName.includes('lattice')) return 3.21
  if (fieldName.includes('surface')) return [1.2, 1.5, 0.8]
  if (fieldName.includes('potential')) return [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
  if (fieldName.includes('config')) return '[[0,0,0],[1,0,0]]'
  return 0
}

function resetAll() {
  selectedDirection.value = 'dft_to_md'
  fieldMappings.value = []
  transferResult.value = null
  mappingSource.value = ''
  mappingTarget.value = ''
  mappingTransformation.value = 'direct'
  mappingUnitFactor.value = 1.0
  onDirectionChange()
}

// ============ Mock Data ============
const mockMappings: Array<{
  source_field: string
  target_field: string
  transformation: string
  unit_conversion: { factor: number; offset: number }
}> = [
  { source_field: 'stacking_fault_energy', target_field: 'sfe_input', transformation: 'direct', unit_conversion: { factor: 1.0, offset: 0 } },
  { source_field: 'elastic_constants', target_field: 'elastic_params', transformation: 'direct', unit_conversion: { factor: 1.0, offset: 0 } },
  { source_field: 'lattice_parameter', target_field: 'lattice_const', transformation: 'direct', unit_conversion: { factor: 1.0, offset: 0 } },
  { source_field: 'formation_energy', target_field: 'potential_params', transformation: 'interpolation', unit_conversion: { factor: 1.602, offset: 0 } },
  { source_field: 'surface_energy', target_field: 'initial_config', transformation: 'homogenization', unit_conversion: { factor: 0.001, offset: 0 } }
]

const mockHistory: Array<{
  direction: DataTransferDirection
  fields_count: number
  success: boolean
  transfer_time_ms: number
  timestamp: string
}> = [
  { direction: 'dft_to_md', fields_count: 5, success: true, transfer_time_ms: 128, timestamp: '2026-03-28 14:30:00' },
  { direction: 'md_to_phase_field', fields_count: 3, success: true, transfer_time_ms: 256, timestamp: '2026-03-28 13:15:00' },
  { direction: 'phase_field_to_fe', fields_count: 4, success: false, transfer_time_ms: 89, timestamp: '2026-03-27 16:45:00' },
  { direction: 'dft_to_md', fields_count: 5, success: true, transfer_time_ms: 142, timestamp: '2026-03-27 10:20:00' }
]

// ============ Lifecycle ============
onMounted(() => {
  fieldMappings.value = [...mockMappings]
  transferHistory.value = [...mockHistory]
})
</script>
