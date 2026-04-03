<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">跨尺度参数映射表</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-004 | DFT/MD/Phase-Field/FE 跨尺度参数映射规则管理</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="refreshTable" class="btn btn-ghost text-xs">刷新</button>
        <button @click="exportRules" class="btn btn-primary text-xs">导出规则</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Mapping Table Overview -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            映射表概览
          </h4>
          <div class="grid grid-cols-2 gap-2">
            <div class="rounded p-2" style="background: var(--bg-elevated)">
              <div class="text-[10px]" style="color: var(--text-muted)">总规则数</div>
              <div class="text-lg font-bold" style="color: var(--primary)">{{ mappingTable.rules.length }}</div>
            </div>
            <div class="rounded p-2" style="background: var(--bg-elevated)">
              <div class="text-[10px]" style="color: var(--text-muted)">版本</div>
              <div class="text-lg font-bold" style="color: var(--text-primary)">{{ mappingTable.version }}</div>
            </div>
          </div>
          <div class="mt-2 text-[10px]" style="color: var(--text-muted)">最后更新: {{ mappingTable.last_updated }}</div>
        </div>

        <!-- Step 2: Rule List -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            映射规则列表
          </h4>
          <div class="space-y-2 max-h-60 overflow-y-auto">
            <div
              v-for="rule in mappingTable.rules" :key="rule.id"
              class="rounded p-2 border cursor-pointer transition"
              :style="editingRule && editingRule.id === rule.id
                ? 'background: var(--bg-elevated); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default)'"
              @click="selectRule(rule)"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium" style="color: var(--text-primary)">
                  {{ rule.source_scale }} → {{ rule.target_scale }}
                </span>
                <div class="flex gap-1">
                  <button @click.stop="editRule(rule)" class="text-[10px] px-1.5 py-0.5 rounded" style="background: var(--bg-base); color: var(--text-secondary)">编辑</button>
                  <button @click.stop="removeRule(rule.id)" class="text-[10px] px-1.5 py-0.5 rounded" style="background: var(--accent-red); color: white">删除</button>
                </div>
              </div>
              <div class="text-[10px]" style="color: var(--text-secondary)">
                {{ rule.source_quantity }} → {{ rule.target_parameter }}
              </div>
              <div class="text-[10px] mt-1" style="color: var(--text-muted)">转换: {{ rule.transformation }}</div>
              <div class="mt-1 h-1.5 rounded-full overflow-hidden" style="background: var(--bg-base)">
                <div
                  class="h-full rounded-full transition-all"
                  :style="{
                    width: (rule.confidence * 100) + '%',
                    background: rule.confidence > 0.8 ? 'var(--accent-green)' : rule.confidence > 0.5 ? 'var(--accent-yellow)' : 'var(--accent-red)'
                  }"
                />
              </div>
              <div class="text-[10px] mt-0.5 text-right" style="color: var(--text-muted)">置信度: {{ (rule.confidence * 100).toFixed(0) }}%</div>
            </div>
          </div>
        </div>

        <!-- Step 3: Add/Edit Rule Form -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            {{ editingRule ? '编辑规则' : '新增规则' }}
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">源尺度</label>
                <select v-model="ruleForm.source_scale" class="input w-full text-xs">
                  <option value="DFT">DFT</option>
                  <option value="MD">MD</option>
                  <option value="Phase-Field">Phase-Field</option>
                  <option value="FE">FE</option>
                </select>
              </div>
              <div>
                <label class="label">目标尺度</label>
                <select v-model="ruleForm.target_scale" class="input w-full text-xs">
                  <option value="DFT">DFT</option>
                  <option value="MD">MD</option>
                  <option value="Phase-Field">Phase-Field</option>
                  <option value="FE">FE</option>
                </select>
              </div>
            </div>
            <div>
              <label class="label">源物理量</label>
              <input v-model="ruleForm.source_quantity" class="input w-full text-xs" placeholder="如: elastic_constant_C11" />
            </div>
            <div>
              <label class="label">目标参数</label>
              <input v-model="ruleForm.target_parameter" class="input w-full text-xs" placeholder="如: youngs_modulus" />
            </div>
            <div>
              <label class="label">转换方式</label>
              <select v-model="ruleForm.transformation" class="input w-full text-xs">
                <option value="direct">直接映射</option>
                <option value="linear_fit">线性拟合</option>
                <option value="polynomial">多项式</option>
                <option value="interpolation">插值</option>
                <option value="ml_model">ML模型</option>
              </select>
            </div>
            <div>
              <label class="label">转换系数 (逗号分隔)</label>
              <input v-model="coefficientsStr" class="input w-full text-xs" placeholder="如: 1.0, 0.5" />
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">源单位</label>
                <input v-model="ruleForm.unit_conversion.from_unit" class="input w-full text-xs" placeholder="GPa" />
              </div>
              <div>
                <label class="label">目标单位</label>
                <input v-model="ruleForm.unit_conversion.to_unit" class="input w-full text-xs" placeholder="MPa" />
              </div>
              <div>
                <label class="label">转换因子</label>
                <input v-model.number="ruleForm.unit_conversion.factor" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">置信度 (0~1)</label>
              <input v-model.number="ruleForm.confidence" type="number" min="0" max="1" step="0.01" class="input w-full text-xs" />
            </div>
            <div class="flex gap-2">
              <button @click="saveRule" class="btn btn-primary text-xs flex-1">{{ editingRule ? '更新' : '添加' }}</button>
              <button v-if="editingRule" @click="cancelEdit" class="btn btn-ghost text-xs flex-1">取消</button>
            </div>
          </div>
        </div>

        <!-- Step 4: Auto Mapping -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            自动映射
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">源节点 ID</label>
              <input v-model="autoMapSource" class="input w-full text-xs" placeholder="node_dft_001" />
            </div>
            <div>
              <label class="label">目标节点 ID</label>
              <input v-model="autoMapTarget" class="input w-full text-xs" placeholder="node_md_001" />
            </div>
            <button @click="runAutoMap" class="btn btn-primary text-xs w-full">自动映射</button>
            <div v-if="autoMapResult" class="rounded p-2 mt-2" style="background: var(--bg-elevated)">
              <div class="text-[10px] font-medium mb-1" style="color: var(--accent-green)">已映射 ({{ autoMapResult.mappings.length }})</div>
              <div v-for="m in autoMapResult.mappings" :key="m.source_quantity" class="text-[10px] mb-1" style="color: var(--text-secondary)">
                {{ m.source_quantity }} → {{ m.target_parameter }} ({{ (m.confidence * 100).toFixed(0) }}%)
              </div>
              <div v-if="autoMapResult.unmapped_source_fields.length > 0" class="text-[10px] mt-2" style="color: var(--accent-yellow)">
                未映射源字段: {{ autoMapResult.unmapped_source_fields.join(', ') }}
              </div>
              <div v-if="autoMapResult.unmapped_target_params.length > 0" class="text-[10px] mt-1" style="color: var(--accent-red)">
                未映射目标参数: {{ autoMapResult.unmapped_target_params.join(', ') }}
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Sensitivity Analysis -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            灵敏度分析
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">源尺度</label>
                <select v-model="sensitivitySource" class="input w-full text-xs">
                  <option value="DFT">DFT</option>
                  <option value="MD">MD</option>
                  <option value="Phase-Field">Phase-Field</option>
                  <option value="FE">FE</option>
                </select>
              </div>
              <div>
                <label class="label">目标尺度</label>
                <select v-model="sensitivityTarget" class="input w-full text-xs">
                  <option value="DFT">DFT</option>
                  <option value="MD">MD</option>
                  <option value="Phase-Field">Phase-Field</option>
                  <option value="FE">FE</option>
                </select>
              </div>
            </div>
            <button @click="runSensitivity" class="btn btn-primary text-xs w-full">分析</button>
            <div v-if="sensitivityResults.length > 0" class="rounded p-2" style="background: var(--bg-elevated)">
              <table class="w-full text-[10px]">
                <thead>
                  <tr style="color: var(--text-muted)">
                    <th class="text-left py-1">参数</th>
                    <th class="text-right py-1">灵敏度</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="entry in sensitivityResults" :key="entry.parameter" style="color: var(--text-secondary)">
                    <td class="py-0.5">{{ entry.parameter }}</td>
                    <td class="text-right py-0.5">{{ entry.sensitivity.toFixed(3) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- Top: Parameter Flow Diagram -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">参数流向图</h4>
          <svg viewBox="0 0 800 280" class="w-full" style="max-height: 280px">
            <defs>
              <marker id="arrowPm" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                <polygon points="0 0, 8 3, 0 6" fill="var(--primary)" />
              </marker>
              <filter id="glowPm">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge><feMergeNode in="blur" /><feMergeNode in="SourceGraphic" /></feMerge>
              </filter>
            </defs>
            <!-- Scale boxes -->
            <g v-for="(scale, idx) in scaleBoxes" :key="scale.name">
              <rect :x="scale.x" :y="scale.y" width="140" height="200" rx="8"
                fill="var(--bg-elevated)" stroke="var(--border-default)" stroke-width="1" />
              <rect :x="scale.x" :y="scale.y" width="140" height="28" rx="8"
                :fill="scale.color" opacity="0.9" />
              <text :x="scale.x + 70" :y="scale.y + 18" text-anchor="middle" fill="white" font-size="12" font-weight="bold">{{ scale.name }}</text>
              <text v-for="(param, pidx) in scale.params" :key="pidx"
                :x="scale.x + 10" :y="scale.y + 48 + pidx * 22"
                fill="var(--text-secondary)" font-size="9">{{ param }}</text>
            </g>
            <!-- Mapping arrows -->
            <g v-for="(arrow, aidx) in flowArrows" :key="aidx">
              <line :x1="arrow.x1" :y1="arrow.y1" :x2="arrow.x2" :y2="arrow.y2"
                stroke="var(--primary)" stroke-width="1.5" marker-end="url(#arrowPm)"
                :stroke-dasharray="arrow.dashed ? '4,3' : 'none'" opacity="0.7" />
              <text :x="(arrow.x1 + arrow.x2) / 2" :y="(arrow.y1 + arrow.y2) / 2 - 6"
                text-anchor="middle" fill="var(--text-muted)" font-size="8">{{ arrow.label }}</text>
            </g>
          </svg>
        </div>

        <!-- Middle: Mapping Coverage Heatmap -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">映射覆盖热力图</h4>
          <svg viewBox="0 0 500 240" class="w-full" style="max-height: 240px">
            <text x="250" y="12" text-anchor="middle" fill="var(--text-muted)" font-size="10">目标尺度</text>
            <text x="10" y="130" text-anchor="middle" fill="var(--text-muted)" font-size="10"
              transform="rotate(-90, 10, 130)">源尺度</text>
            <!-- Column headers -->
            <g v-for="(col, ci) in heatmapScales" :key="'ch'+ci">
              <text :x="80 + ci * 100 + 40" :y="30" text-anchor="middle" fill="var(--text-secondary)" font-size="11">{{ col }}</text>
            </g>
            <!-- Row headers -->
            <g v-for="(row, ri) in heatmapScales" :key="'rh'+ri">
              <text :x="70" :y="55 + ri * 45 + 25" text-anchor="end" fill="var(--text-secondary)" font-size="11">{{ row }}</text>
            </g>
            <!-- Cells -->
            <g v-for="(row, ri) in heatmapData" :key="'hr'+ri">
              <g v-for="(cell, ci) in row" :key="'hc'+ri+'-'+ci">
                <rect :x="80 + ci * 100" :y="40 + ri * 45" width="80" height="35" rx="4"
                  :fill="cell === 0 ? 'var(--bg-elevated)' : cell <= 2 ? 'rgba(59,130,246,0.2)' : cell <= 4 ? 'rgba(59,130,246,0.5)' : 'rgba(59,130,246,0.8)'"
                  stroke="var(--border-default)" stroke-width="0.5" />
                <text :x="80 + ci * 100 + 40" :y="40 + ri * 45 + 22" text-anchor="middle"
                  :fill="cell === 0 ? 'var(--text-muted)' : 'white'" font-size="12" font-weight="bold">{{ cell }}</text>
              </g>
            </g>
          </svg>
        </div>

        <!-- Bottom: Sensitivity Bar Chart -->
        <div class="rounded border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">灵敏度分析结果</h4>
          <div v-if="sensitivityResults.length > 0">
            <svg viewBox="0 0 600 200" class="w-full" style="max-height: 200px">
              <g v-for="(entry, idx) in sensitivityResults" :key="entry.parameter">
                <text x="150" :y="20 + idx * 30" text-anchor="end" fill="var(--text-secondary)" font-size="10">{{ entry.parameter }}</text>
                <rect x="160" :y="10 + idx * 30" :width="Math.max(0, entry.sensitivity * 400)" height="18" rx="3"
                  :fill="entry.sensitivity > 0.7 ? 'var(--accent-red)' : entry.sensitivity > 0.4 ? 'var(--accent-yellow)' : 'var(--accent-green)'" />
                <text :x="165 + entry.sensitivity * 400" :y="23 + idx * 30" fill="var(--text-primary)" font-size="10">{{ entry.sensitivity.toFixed(3) }}</text>
              </g>
            </svg>
          </div>
          <div v-else class="text-center py-8 text-xs" style="color: var(--text-muted)">请选择尺度对并运行灵敏度分析</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { PmMappingRule, PmMappingTable, PmAutoMappingResult, PmSensitivityEntry } from '../api/paramMapping'

// ============ State ============

const mappingTable = reactive<PmMappingTable>({
  rules: [],
  version: '2.0.0',
  last_updated: '2026-04-03T10:30:00Z'
})

const editingRule = ref<PmMappingRule | null>(null)
const coefficientsStr = ref('1.0')

const ruleForm = reactive({
  source_scale: 'DFT',
  target_scale: 'MD',
  source_quantity: '',
  target_parameter: '',
  transformation: 'direct' as 'direct' | 'linear_fit' | 'polynomial' | 'interpolation' | 'ml_model',
  coefficients: [1.0],
  unit_conversion: { from_unit: '', to_unit: '', factor: 1.0 },
  description: '',
  confidence: 0.8
})

const autoMapSource = ref('')
const autoMapTarget = ref('')
const autoMapResult = ref<PmAutoMappingResult | null>(null)

const sensitivitySource = ref('DFT')
const sensitivityTarget = ref('MD')
const sensitivityResults = ref<PmSensitivityEntry[]>([])

// ============ Scale Boxes for Flow Diagram ============

const scaleBoxes = computed(() => [
  { name: 'DFT', x: 10, y: 40, color: '#3b82f6', params: ['Elastic Cij', 'Formation E', 'Band gap', 'Bader charge'] },
  { name: 'MD', x: 210, y: 40, color: '#10b981', params: ['EAM params', 'Lattice const', 'Diffusivity', 'Virial stress'] },
  { name: 'Phase-Field', x: 410, y: 40, color: '#f59e0b', params: ['Mobility', 'Gradient coeff', 'Interface energy', 'Driving force'] },
  { name: 'FE', x: 610, y: 40, color: '#ef4444', params: ['Young modulus', 'Poisson ratio', 'Yield stress', 'Hardening law'] }
])

const flowArrows = computed(() => [
  { x1: 150, y1: 120, x2: 210, y2: 120, label: 'Cij→EAM', dashed: false },
  { x1: 150, y1: 160, x2: 210, y2: 160, label: 'E_form→Lattice', dashed: true },
  { x1: 350, y1: 120, x2: 410, y2: 120, label: 'Diff→Mobility', dashed: false },
  { x1: 350, y1: 160, x2: 410, y2: 160, label: 'Stress→Driving', dashed: true },
  { x1: 550, y1: 120, x2: 610, y2: 120, label: 'Grad→Yield', dashed: false },
  { x1: 550, y1: 160, x2: 610, y2: 160, label: 'Intf→Hardening', dashed: true }
])

// ============ Heatmap ============

const heatmapScales = ['DFT', 'MD', 'Phase-Field', 'FE']

const heatmapData = computed(() => {
  const counts: number[][] = [
    [0, 3, 1, 2],
    [2, 0, 4, 1],
    [1, 3, 0, 3],
    [2, 1, 2, 0]
  ]
  return counts
})

// ============ Mock Data ============

function generateMockRules(): PmMappingRule[] {
  return [
    { id: 'pm-001', source_scale: 'DFT', target_scale: 'MD', source_quantity: 'elastic_constant_C11', target_parameter: 'eam_epsilon', transformation: 'linear_fit', coefficients: [1.2, 0.05], unit_conversion: { from_unit: 'GPa', to_unit: 'eV', factor: 0.01036 }, description: '弹性常数C11到EAM epsilon参数', confidence: 0.92 },
    { id: 'pm-002', source_scale: 'DFT', target_scale: 'MD', source_quantity: 'elastic_constant_C12', target_parameter: 'eam_sigma', transformation: 'linear_fit', coefficients: [0.8, 0.03], unit_conversion: { from_unit: 'GPa', to_unit: 'Angstrom', factor: 0.01 }, description: '弹性常数C12到EAM sigma参数', confidence: 0.85 },
    { id: 'pm-003', source_scale: 'DFT', target_scale: 'MD', source_quantity: 'formation_energy', target_parameter: 'lattice_constant', transformation: 'polynomial', coefficients: [3.6, -0.02, 0.001], unit_conversion: { from_unit: 'eV/atom', to_unit: 'Angstrom', factor: 1.0 }, description: '形成能到晶格常数', confidence: 0.78 },
    { id: 'pm-004', source_scale: 'DFT', target_scale: 'Phase-Field', source_quantity: 'interface_energy', target_parameter: 'gradient_coefficient', transformation: 'direct', coefficients: [1.0], unit_conversion: { from_unit: 'J/m2', to_unit: 'J/m', factor: 1e-10 }, description: '界面能到梯度系数', confidence: 0.88 },
    { id: 'pm-005', source_scale: 'DFT', target_scale: 'FE', source_quantity: 'elastic_constant_C11', target_parameter: 'youngs_modulus', transformation: 'linear_fit', coefficients: [0.95, 2.1], unit_conversion: { from_unit: 'GPa', to_unit: 'MPa', factor: 1000 }, description: '弹性常数到杨氏模量', confidence: 0.95 },
    { id: 'pm-006', source_scale: 'DFT', target_scale: 'FE', source_quantity: 'elastic_constant_C12', target_parameter: 'poisson_ratio', transformation: 'linear_fit', coefficients: [0.33, 0.001], unit_conversion: { from_unit: 'GPa', to_unit: 'dimensionless', factor: 0.01 }, description: '弹性常数到泊松比', confidence: 0.90 },
    { id: 'pm-007', source_scale: 'MD', target_scale: 'DFT', source_quantity: 'virial_stress', target_parameter: 'elastic_constant_C11', transformation: 'ml_model', coefficients: [1.0], unit_conversion: { from_unit: 'bar', to_unit: 'GPa', factor: 0.0001 }, description: '维里应力到弹性常数(ML)', confidence: 0.72 },
    { id: 'pm-008', source_scale: 'MD', target_scale: 'MD', source_quantity: 'diffusivity', target_parameter: 'mobility', transformation: 'direct', coefficients: [1.0], unit_conversion: { from_unit: 'm2/s', to_unit: 'm2/(J*s)', factor: 1e10 }, description: '扩散系数到迁移率', confidence: 0.82 },
    { id: 'pm-009', source_scale: 'MD', target_scale: 'Phase-Field', source_quantity: 'diffusivity', target_parameter: 'mobility', transformation: 'interpolation', coefficients: [1.5, 0.2], unit_conversion: { from_unit: 'm2/s', to_unit: 'm2/(J*s)', factor: 1e10 }, description: '扩散系数到相场迁移率', confidence: 0.86 },
    { id: 'pm-010', source_scale: 'MD', target_scale: 'Phase-Field', source_quantity: 'potential_energy', target_parameter: 'driving_force', transformation: 'linear_fit', coefficients: [0.75, 0.1], unit_conversion: { from_unit: 'eV', to_unit: 'J/m3', factor: 1.602e-19 }, description: '势能到驱动力', confidence: 0.79 },
    { id: 'pm-011', source_scale: 'MD', target_scale: 'Phase-Field', source_quantity: 'radial_distribution', target_parameter: 'interface_energy', transformation: 'ml_model', coefficients: [1.0], unit_conversion: { from_unit: 'dimensionless', to_unit: 'J/m2', factor: 0.5 }, description: '径向分布到界面能(ML)', confidence: 0.68 },
    { id: 'pm-012', source_scale: 'MD', target_scale: 'Phase-Field', source_quantity: 'mean_square_displacement', target_parameter: 'gradient_coefficient', transformation: 'polynomial', coefficients: [2.0, -0.5, 0.1], unit_conversion: { from_unit: 'Angstrom2', to_unit: 'J/m', factor: 1e-20 }, description: 'MSD到梯度系数', confidence: 0.74 }
  ]
}

function generateMockAutoMapResult(): PmAutoMappingResult {
  return {
    source_node_id: 'node_dft_001',
    target_node_id: 'node_md_001',
    mappings: [
      { source_quantity: 'elastic_constant_C11', target_parameter: 'eam_epsilon', value: 120.5, unit: 'GPa', confidence: 0.92 },
      { source_quantity: 'elastic_constant_C12', target_parameter: 'eam_sigma', value: 68.3, unit: 'GPa', confidence: 0.85 },
      { source_quantity: 'formation_energy', target_parameter: 'lattice_constant', value: -3.62, unit: 'eV/atom', confidence: 0.78 }
    ],
    unmapped_source_fields: ['band_gap', 'bader_charge'],
    unmapped_target_params: ['eam_cutoff']
  }
}

function generateMockSensitivity(): PmSensitivityEntry[] {
  return [
    { parameter: 'elastic_constant_C11', sensitivity: 0.892, source_scale: 'DFT', target_scale: 'MD' },
    { parameter: 'elastic_constant_C12', sensitivity: 0.654, source_scale: 'DFT', target_scale: 'MD' },
    { parameter: 'formation_energy', sensitivity: 0.421, source_scale: 'DFT', target_scale: 'MD' },
    { parameter: 'interface_energy', sensitivity: 0.738, source_scale: 'DFT', target_scale: 'Phase-Field' },
    { parameter: 'diffusivity', sensitivity: 0.856, source_scale: 'MD', target_scale: 'Phase-Field' },
    { parameter: 'potential_energy', sensitivity: 0.312, source_scale: 'MD', target_scale: 'Phase-Field' }
  ]
}

// ============ Actions ============

function selectRule(rule: PmMappingRule) {
  editingRule.value = rule
  ruleForm.source_scale = rule.source_scale
  ruleForm.target_scale = rule.target_scale
  ruleForm.source_quantity = rule.source_quantity
  ruleForm.target_parameter = rule.target_parameter
  ruleForm.transformation = rule.transformation
  ruleForm.coefficients = [...rule.coefficients]
  ruleForm.unit_conversion = { ...rule.unit_conversion }
  ruleForm.confidence = rule.confidence
  coefficientsStr.value = rule.coefficients.join(', ')
}

function editRule(rule: PmMappingRule) {
  selectRule(rule)
}

function cancelEdit() {
  editingRule.value = null
  ruleForm.source_scale = 'DFT'
  ruleForm.target_scale = 'MD'
  ruleForm.source_quantity = ''
  ruleForm.target_parameter = ''
  ruleForm.transformation = 'direct' as 'direct' | 'linear_fit' | 'polynomial' | 'interpolation' | 'ml_model'
  ruleForm.coefficients = [1.0]
  ruleForm.unit_conversion = { from_unit: '', to_unit: '', factor: 1.0 }
  ruleForm.confidence = 0.8
  coefficientsStr.value = '1.0'
}

function saveRule() {
  const coeffs = coefficientsStr.value.split(',').map(s => parseFloat(s.trim())).filter(n => !isNaN(n))
  if (coeffs.length === 0) return
  ruleForm.coefficients = coeffs

  if (editingRule.value) {
    const idx = mappingTable.rules.findIndex((r: PmMappingRule) => r.id === editingRule.value!.id)
    if (idx !== -1) {
      mappingTable.rules[idx] = {
        ...editingRule.value,
        source_scale: ruleForm.source_scale,
        target_scale: ruleForm.target_scale,
        source_quantity: ruleForm.source_quantity,
        target_parameter: ruleForm.target_parameter,
        transformation: ruleForm.transformation,
        coefficients: [...ruleForm.coefficients],
        unit_conversion: { ...ruleForm.unit_conversion },
        confidence: ruleForm.confidence
      }
    }
    editingRule.value = null
  } else {
    const newRule: PmMappingRule = {
      id: 'pm-' + String(mappingTable.rules.length + 1).padStart(3, '0'),
      source_scale: ruleForm.source_scale,
      target_scale: ruleForm.target_scale,
      source_quantity: ruleForm.source_quantity,
      target_parameter: ruleForm.target_parameter,
      transformation: ruleForm.transformation,
      coefficients: [...ruleForm.coefficients],
      unit_conversion: { ...ruleForm.unit_conversion },
      description: '',
      confidence: ruleForm.confidence
    }
    mappingTable.rules.push(newRule)
  }
  mappingTable.last_updated = new Date().toISOString()
  cancelEdit()
}

function removeRule(ruleId: string) {
  const idx = mappingTable.rules.findIndex((r: PmMappingRule) => r.id === ruleId)
  if (idx !== -1) {
    mappingTable.rules.splice(idx, 1)
    if (editingRule.value && editingRule.value.id === ruleId) {
      cancelEdit()
    }
  }
}

function runAutoMap() {
  if (!autoMapSource.value || !autoMapTarget.value) return
  const result = generateMockAutoMapResult()
  result.source_node_id = autoMapSource.value
  result.target_node_id = autoMapTarget.value
  autoMapResult.value = result
}

function runSensitivity() {
  if (sensitivitySource.value === sensitivityTarget.value) return
  sensitivityResults.value = generateMockSensitivity().filter(
    e => e.source_scale === sensitivitySource.value && e.target_scale === sensitivityTarget.value
  )
  if (sensitivityResults.value.length === 0) {
    sensitivityResults.value = generateMockSensitivity().slice(0, 4)
  }
}

function refreshTable() {
  mappingTable.rules = generateMockRules()
  mappingTable.last_updated = new Date().toISOString()
}

function exportRules() {
  const data = JSON.stringify(mappingTable, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'param_mapping_rules.json'
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  const rules = generateMockRules()
  mappingTable.rules = rules
  const currentCount = mappingTable.rules.length
  nextTick(() => {
    console.log('Param mapping table loaded with', currentCount, 'rules')
  })
})
</script>
