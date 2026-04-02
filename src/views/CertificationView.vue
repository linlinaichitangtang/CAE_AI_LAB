<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">ISO/ASME 认证报告生成器</h2>
        <p class="text-sm text-[var(--text-muted)]">应力云图 + 安全系数 + 合规性声明，符合 ASME Y14.5 / ISO 2553</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板按钮 -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="btn btn-ghost text-xs"
        >
          {{ tpl.name }}
        </button>
        <!-- 操作按钮 -->
        <button @click="checkCompliance" class="btn btn-ghost text-xs" :disabled="!config.standard">
          检查合规性
        </button>
        <button @click="addStressRow" class="btn btn-ghost text-xs">
          添加应力数据
        </button>
        <button @click="generateReport" :disabled="generating" class="btn btn-primary text-xs">
          {{ generating ? '生成中...' : '生成报告' }}
        </button>
        <button
          v-if="result"
          @click="exportPDF"
          class="btn btn-ghost text-xs"
          style="color: var(--accent-green); border-color: var(--accent-green);"
        >
          导出PDF
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">

        <!-- Step 1: 标准选择 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">1</span>
            标准选择
          </h4>
          <div class="flex flex-col gap-1.5">
            <button
              v-for="std in standards"
              :key="std.value"
              @click="config.standard = std.value"
              :class="['px-3 py-2 rounded text-xs text-left transition border',
                config.standard === std.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
            >
              <div class="font-medium">{{ std.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ std.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: 项目信息 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">2</span>
            项目信息
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">部件名称</label>
              <input v-model="config.component_name" type="text" class="input w-full text-xs" placeholder="例: 压力容器筒体" />
            </div>
            <div>
              <label class="label">材料规格</label>
              <input v-model="config.material_spec" type="text" class="input w-full text-xs" placeholder="例: SA-516 Gr.70" />
            </div>
            <div>
              <label class="label">设计规范</label>
              <input v-model="config.design_code" type="text" class="input w-full text-xs" placeholder="例: ASME BPVC Section VIII Div.1" />
            </div>
            <div>
              <label class="label">检验员</label>
              <input v-model="config.inspector_name" type="text" class="input w-full text-xs" placeholder="例: 张工" />
            </div>
            <div>
              <label class="label">报告编号</label>
              <input v-model="config.report_number" type="text" class="input w-full text-xs" placeholder="例: CERT-2026-0011" />
            </div>
          </div>
        </div>

        <!-- Step 3: 工况条件 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">3</span>
            工况条件
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">操作温度 (°C)</label>
              <input v-model.number="config.operating_conditions.temperature" type="number" step="1" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">操作压力 (MPa)</label>
              <input v-model.number="config.operating_conditions.pressure" type="number" step="0.1" class="input w-full text-xs" />
            </div>
            <div class="flex items-center gap-2">
              <label class="label mb-0">循环载荷</label>
              <div
                @click="config.operating_conditions.cyclic_loading = !config.operating_conditions.cyclic_loading"
                :class="['toggle', config.operating_conditions.cyclic_loading ? 'active' : '']"
              >
                <div class="toggle-knob"></div>
              </div>
              <span class="text-xs text-[var(--text-muted)]">
                {{ config.operating_conditions.cyclic_loading ? '已启用' : '已关闭' }}
              </span>
            </div>
          </div>
        </div>

        <!-- Step 4: 应力结果导入 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">4</span>
            应力结果导入
          </h4>
          <div class="space-y-2">
            <div v-for="(row, idx) in config.stress_results" :key="idx" class="p-2 rounded border border-[var(--border-default)] bg-[var(--bg-elevated)] space-y-1.5">
              <div class="flex items-center justify-between">
                <span class="text-[10px] text-[var(--text-muted)]">数据行 {{ idx + 1 }}</span>
                <button @click="removeStressRow(idx)" class="text-[var(--accent-red)] text-xs hover:underline">删除</button>
              </div>
              <div class="grid grid-cols-2 gap-1.5">
                <div>
                  <label class="label text-[10px]">位置</label>
                  <input v-model="row.location" type="text" class="input w-full text-xs" placeholder="例: 筒体中部" />
                </div>
                <div>
                  <label class="label text-[10px]">应力类型</label>
                  <select v-model="row.stress_type" class="input w-full text-xs">
                    <option value="membrane">膜应力</option>
                    <option value="bending">弯曲应力</option>
                    <option value="peak">峰值应力</option>
                    <option value="primary">一次应力</option>
                    <option value="secondary">二次应力</option>
                  </select>
                </div>
                <div>
                  <label class="label text-[10px]">应力值 (MPa)</label>
                  <input v-model.number="row.value" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label text-[10px]">许用值 (MPa)</label>
                  <input v-model.number="row.allowable" type="number" step="1" class="input w-full text-xs" />
                </div>
              </div>
              <div>
                <label class="label text-[10px]">利用率: {{ row.allowable > 0 ? ((row.value / row.allowable) * 100).toFixed(1) : '0.0' }}%</label>
              </div>
            </div>
            <button @click="addStressRow" class="btn btn-ghost text-xs w-full">+ 添加应力数据行</button>
          </div>
        </div>

        <!-- Step 5: 安全系数 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">5</span>
            安全系数
          </h4>
          <div class="space-y-2">
            <div v-for="(sf, idx) in config.safety_factors" :key="idx" class="p-2 rounded border border-[var(--border-default)] bg-[var(--bg-elevated)] space-y-1.5">
              <div class="flex items-center justify-between">
                <span class="text-[10px] text-[var(--text-muted)]">部件 {{ idx + 1 }}</span>
                <button @click="removeSafetyFactor(idx)" class="text-[var(--accent-red)] text-xs hover:underline">删除</button>
              </div>
              <div class="grid grid-cols-2 gap-1.5">
                <div>
                  <label class="label text-[10px]">部件名称</label>
                  <input v-model="sf.component_name" type="text" class="input w-full text-xs" placeholder="例: 封头" />
                </div>
                <div>
                  <label class="label text-[10px]">安全系数</label>
                  <input v-model.number="sf.safety_factor" type="number" step="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label text-[10px]">最小要求</label>
                  <input v-model.number="sf.minimum_required" type="number" step="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label text-[10px]">关键位置</label>
                  <input v-model="sf.critical_location" type="text" class="input w-full text-xs" placeholder="例: 接管焊缝" />
                </div>
              </div>
              <div class="flex items-center gap-2">
                <span class="text-[10px] text-[var(--text-muted)]">状态:</span>
                <span
                  :class="['text-xs font-medium',
                    sf.safety_factor >= sf.minimum_required ? 'text-[var(--accent-green)]' : 'text-[var(--accent-red)]']"
                >
                  {{ sf.safety_factor >= sf.minimum_required ? '通过' : '不通过' }}
                </span>
              </div>
            </div>
            <button @click="addSafetyFactor" class="btn btn-ghost text-xs w-full">+ 添加安全系数</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- 总体合规状态卡片 -->
        <div v-if="result" class="rounded-lg border p-6 transition-all duration-200"
          :class="{
            'border-[var(--accent-green)] bg-green-50': result.overall_status === 'pass',
            'border-[var(--accent-yellow)] bg-yellow-50': result.overall_status === 'conditional',
            'border-[var(--accent-red)] bg-red-50': result.overall_status === 'fail'
          }"
        >
          <div class="flex items-center gap-4">
            <div class="text-5xl">
              <span v-if="result.overall_status === 'pass'">&#10004;</span>
              <span v-else-if="result.overall_status === 'conditional'">&#9888;</span>
              <span v-else>&#10008;</span>
            </div>
            <div>
              <h3 class="text-xl font-bold"
                :class="{
                  'text-[var(--accent-green)]': result.overall_status === 'pass',
                  'text-yellow-600': result.overall_status === 'conditional',
                  'text-[var(--accent-red)]': result.overall_status === 'fail'
                }"
              >
                {{ overallStatusLabel }}
              </h3>
              <p class="text-sm text-[var(--text-secondary)] mt-1">
                合规项: {{ result.compliance_summary.compliant }} / {{ result.compliance_summary.total }}
                &nbsp;|&nbsp;
                不合规: {{ result.compliance_summary.non_compliant }}
                &nbsp;|&nbsp;
                待定: {{ result.compliance_summary.pending }}
              </p>
            </div>
          </div>
        </div>

        <!-- 无结果时的占位 -->
        <div v-if="!result" class="flex items-center justify-center h-64 text-[var(--text-muted)]">
          <div class="text-center">
            <div class="text-4xl mb-2">&#128196;</div>
            <div>配置参数后点击「生成报告」查看认证结果</div>
          </div>
        </div>

        <!-- 应力校核结果表 -->
        <div v-if="result && result.stress_check_results.length > 0" class="rounded-lg border border-[var(--border-default)] bg-[var(--bg-surface)] overflow-hidden">
          <div class="px-4 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">应力校核结果</h4>
          </div>
          <div class="overflow-x-auto">
            <table class="w-full text-xs">
              <thead>
                <tr class="border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">位置</th>
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">应力类型</th>
                  <th class="px-3 py-2 text-right text-[var(--text-secondary)] font-medium">应力值 (MPa)</th>
                  <th class="px-3 py-2 text-right text-[var(--text-secondary)] font-medium">许用值 (MPa)</th>
                  <th class="px-3 py-2 text-right text-[var(--text-secondary)] font-medium">利用率</th>
                  <th class="px-3 py-2 text-center text-[var(--text-secondary)] font-medium">状态</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(row, idx) in result.stress_check_results"
                  :key="idx"
                  class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-elevated)] transition-colors"
                >
                  <td class="px-3 py-2 text-[var(--text-primary)]">{{ row.location }}</td>
                  <td class="px-3 py-2 text-[var(--text-secondary)]">{{ stressTypeLabel(row.stress_type) }}</td>
                  <td class="px-3 py-2 text-right text-[var(--text-primary)] font-mono">{{ row.value.toFixed(1) }}</td>
                  <td class="px-3 py-2 text-right text-[var(--text-secondary)] font-mono">{{ row.allowable.toFixed(1) }}</td>
                  <td class="px-3 py-2 text-right font-mono">
                    <span
                      :class="{
                        'text-[var(--accent-green)]': row.utilization_ratio <= 0.8,
                        'text-yellow-600': row.utilization_ratio > 0.8 && row.utilization_ratio <= 1.0,
                        'text-[var(--accent-red)]': row.utilization_ratio > 1.0
                      }"
                    >
                      {{ (row.utilization_ratio * 100).toFixed(1) }}%
                    </span>
                  </td>
                  <td class="px-3 py-2 text-center">
                    <span
                      :class="['inline-flex items-center justify-center w-6 h-6 rounded-full text-white text-[10px] font-bold',
                        row.status === 'pass' ? 'bg-[var(--accent-green)]' :
                        row.status === 'warning' ? 'bg-yellow-500' : 'bg-[var(--accent-red)]']"
                    >
                      {{ row.status === 'pass' ? 'P' : row.status === 'warning' ? 'W' : 'F' }}
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- 安全系数结果表 -->
        <div v-if="result && result.safety_factor_results.length > 0" class="rounded-lg border border-[var(--border-default)] bg-[var(--bg-surface)] overflow-hidden">
          <div class="px-4 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">安全系数结果</h4>
          </div>
          <div class="overflow-x-auto">
            <table class="w-full text-xs">
              <thead>
                <tr class="border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">部件</th>
                  <th class="px-3 py-2 text-right text-[var(--text-secondary)] font-medium">安全系数</th>
                  <th class="px-3 py-2 text-right text-[var(--text-secondary)] font-medium">最小要求</th>
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">关键位置</th>
                  <th class="px-3 py-2 text-center text-[var(--text-secondary)] font-medium">状态</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(sf, idx) in result.safety_factor_results"
                  :key="idx"
                  class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-elevated)] transition-colors"
                >
                  <td class="px-3 py-2 text-[var(--text-primary)]">{{ sf.component_name }}</td>
                  <td class="px-3 py-2 text-right font-mono font-medium"
                    :class="sf.status === 'pass' ? 'text-[var(--accent-green)]' : 'text-[var(--accent-red)]'">
                    {{ sf.safety_factor.toFixed(2) }}
                  </td>
                  <td class="px-3 py-2 text-right text-[var(--text-secondary)] font-mono">{{ sf.minimum_required.toFixed(2) }}</td>
                  <td class="px-3 py-2 text-[var(--text-secondary)]">{{ sf.critical_location }}</td>
                  <td class="px-3 py-2 text-center">
                    <span
                      :class="['inline-flex items-center justify-center px-2 py-0.5 rounded text-[10px] font-bold text-white',
                        sf.status === 'pass' ? 'bg-[var(--accent-green)]' :
                        sf.status === 'warning' ? 'bg-yellow-500' : 'bg-[var(--accent-red)]']"
                    >
                      {{ sf.status === 'pass' ? '通过' : sf.status === 'warning' ? '警告' : '不通过' }}
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- 合规检查清单 -->
        <div v-if="result && result.compliance_items.length > 0" class="rounded-lg border border-[var(--border-default)] bg-[var(--bg-surface)] overflow-hidden">
          <div class="px-4 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)] flex items-center justify-between">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">合规检查清单</h4>
            <span class="text-xs text-[var(--text-muted)]">
              {{ result.compliance_items.filter(i => i.status === 'compliant').length }} / {{ result.compliance_items.length }} 项合规
            </span>
          </div>
          <div class="overflow-x-auto">
            <table class="w-full text-xs">
              <thead>
                <tr class="border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium w-8">状态</th>
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">条款号</th>
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">描述</th>
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">要求</th>
                  <th class="px-3 py-2 text-left text-[var(--text-secondary)] font-medium">证据</th>
                  <th class="px-3 py-2 text-center text-[var(--text-secondary)] font-medium">合规</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(item, idx) in result.compliance_items"
                  :key="idx"
                  class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-elevated)] transition-colors"
                >
                  <td class="px-3 py-2 text-center">
                    <input
                      type="checkbox"
                      :checked="item.status === 'compliant'"
                      @change="toggleCompliance(idx)"
                      class="w-4 h-4 rounded cursor-pointer accent-blue-600"
                    />
                  </td>
                  <td class="px-3 py-2 text-[var(--text-primary)] font-mono font-medium">{{ item.clause_id }}</td>
                  <td class="px-3 py-2 text-[var(--text-secondary)]">{{ item.description }}</td>
                  <td class="px-3 py-2 text-[var(--text-secondary)] max-w-[200px] truncate">{{ item.requirement }}</td>
                  <td class="px-3 py-2 text-[var(--text-muted)] max-w-[150px] truncate">{{ item.evidence }}</td>
                  <td class="px-3 py-2 text-center">
                    <span
                      :class="['inline-flex items-center justify-center px-2 py-0.5 rounded text-[10px] font-bold text-white',
                        item.status === 'compliant' ? 'bg-[var(--accent-green)]' :
                        item.status === 'non_compliant' ? 'bg-[var(--accent-red)]' :
                        item.status === 'pending' ? 'bg-yellow-500' : 'bg-gray-400']"
                    >
                      {{ complianceStatusLabel(item.status) }}
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- 改进建议列表 -->
        <div v-if="result && result.recommendations.length > 0" class="rounded-lg border border-[var(--border-default)] bg-[var(--bg-surface)] overflow-hidden">
          <div class="px-4 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">改进建议</h4>
          </div>
          <ul class="divide-y divide-[var(--border-subtle)]">
            <li
              v-for="(rec, idx) in result.recommendations"
              :key="idx"
              class="px-4 py-3 flex items-start gap-3 hover:bg-[var(--bg-elevated)] transition-colors"
            >
              <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-100 text-blue-700 text-[10px] font-bold mt-0.5 shrink-0">
                {{ idx + 1 }}
              </span>
              <span class="text-xs text-[var(--text-secondary)] leading-relaxed">{{ rec }}</span>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import type {
  CertificationStandard,
  StressResult,
  SafetyFactorResult,
  ComplianceItem,
  CertificationConfig,
  CertificationResult,
  CertificationTemplate
} from '../api/certification'

// ============ 标准列表 ============

const standards = [
  { value: 'ASME_Y14_5' as CertificationStandard, label: 'ASME Y14.5', desc: '几何尺寸与公差 (GD&T)' },
  { value: 'ISO_2553' as CertificationStandard, label: 'ISO 2553', desc: '焊接符号表示方法' },
  { value: 'ASME_BPVC_VIII' as CertificationStandard, label: 'ASME BPVC VIII', desc: '压力容器设计规范' },
  { value: 'ISO_15614' as CertificationStandard, label: 'ISO 15614', desc: '焊接工艺评定规范' },
  { value: 'ASME_B31_3' as CertificationStandard, label: 'ASME B31.3', desc: '工艺管道规范' },
  { value: 'ISO_9001' as CertificationStandard, label: 'ISO 9001', desc: '质量管理体系' }
]

// ============ 快速模板 ============

const quickTemplates: CertificationTemplate[] = [
  { id: 'tpl-pressure-vessel', name: '压力容器 (ASME BPVC VIII)', standard: 'ASME_BPVC_VIII', description: '压力容器设计与制造认证模板' },
  { id: 'tpl-piping', name: '管道 (ASME B31.3)', standard: 'ASME_B31_3', description: '工艺管道系统认证模板' },
  { id: 'tpl-welding', name: '焊接 (ISO 15614)', standard: 'ISO_15614', description: '焊接工艺评定认证模板' },
  { id: 'tpl-gdt', name: 'GD&T (ASME Y14.5)', standard: 'ASME_Y14_5', description: '几何尺寸与公差认证模板' }
]

// ============ 配置状态 ============

const config = reactive<CertificationConfig>({
  project_id: '',
  standard: 'ASME_BPVC_VIII',
  component_name: '压力容器筒体',
  material_spec: 'SA-516 Gr.70',
  design_code: 'ASME BPVC Section VIII Div.1',
  operating_conditions: {
    temperature: 250,
    pressure: 2.5,
    cyclic_loading: false
  },
  stress_results: [
    { location: '筒体中部', stress_type: 'membrane', value: 138.5, allowable: 172.5, utilization_ratio: 0.803, status: 'pass' },
    { location: '封头过渡区', stress_type: 'bending', value: 155.2, allowable: 172.5, utilization_ratio: 0.900, status: 'warning' },
    { location: '接管焊缝', stress_type: 'peak', value: 198.7, allowable: 172.5, utilization_ratio: 1.152, status: 'fail' },
    { location: '法兰连接处', stress_type: 'primary', value: 120.3, allowable: 172.5, utilization_ratio: 0.697, status: 'pass' },
    { location: '支座区域', stress_type: 'secondary', value: 95.8, allowable: 345.0, utilization_ratio: 0.278, status: 'pass' }
  ],
  safety_factors: [
    { component_name: '筒体', safety_factor: 1.85, minimum_required: 1.50, status: 'pass', critical_location: '纵焊缝' },
    { component_name: '封头', safety_factor: 1.62, minimum_required: 1.50, status: 'pass', critical_location: '过渡圆弧' },
    { component_name: '接管', safety_factor: 1.28, minimum_required: 1.50, status: 'fail', critical_location: '接管焊缝' },
    { component_name: '法兰', safety_factor: 2.10, minimum_required: 1.50, status: 'pass', critical_location: '螺栓孔' }
  ],
  inspector_name: '张工',
  report_number: 'CERT-2026-0011'
})

// ============ 结果状态 ============

const generating = ref(false)
const result = ref<CertificationResult | null>(null)

// ============ 计算属性 ============

const overallStatusLabel = computed(() => {
  if (!result.value) return ''
  const labels: Record<string, string> = {
    pass: '认证通过',
    conditional: '有条件通过',
    fail: '认证不通过'
  }
  return labels[result.value.overall_status] || result.value.overall_status
})

// ============ 辅助函数 ============

function stressTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    membrane: '膜应力',
    bending: '弯曲应力',
    peak: '峰值应力',
    primary: '一次应力',
    secondary: '二次应力'
  }
  return labels[type] || type
}

function complianceStatusLabel(status: string): string {
  const labels: Record<string, string> = {
    compliant: '合规',
    non_compliant: '不合规',
    pending: '待定',
    not_applicable: '不适用'
  }
  return labels[status] || status
}

// ============ 操作函数 ============

function addStressRow() {
  config.stress_results.push({
    location: '',
    stress_type: 'membrane',
    value: 0,
    allowable: 0,
    utilization_ratio: 0,
    status: 'pass'
  })
}

function removeStressRow(idx: number) {
  config.stress_results.splice(idx, 1)
}

function addSafetyFactor() {
  config.safety_factors.push({
    component_name: '',
    safety_factor: 0,
    minimum_required: 1.5,
    status: 'fail',
    critical_location: ''
  })
}

function removeSafetyFactor(idx: number) {
  config.safety_factors.splice(idx, 1)
}

function toggleCompliance(idx: number) {
  if (!result.value) return
  const item = result.value.compliance_items[idx]
  if (item.status === 'compliant') {
    item.status = 'non_compliant'
  } else {
    item.status = 'compliant'
  }
  // 重新计算合规汇总
  const items = result.value.compliance_items
  result.value.compliance_summary = {
    total: items.length,
    compliant: items.filter(i => i.status === 'compliant').length,
    non_compliant: items.filter(i => i.status === 'non_compliant').length,
    pending: items.filter(i => i.status === 'pending').length
  }
}

function applyTemplate(tpl: CertificationTemplate) {
  config.standard = tpl.standard
  switch (tpl.standard) {
    case 'ASME_BPVC_VIII':
      config.component_name = '压力容器筒体'
      config.material_spec = 'SA-516 Gr.70'
      config.design_code = 'ASME BPVC Section VIII Div.1'
      config.operating_conditions.temperature = 250
      config.operating_conditions.pressure = 2.5
      config.operating_conditions.cyclic_loading = false
      break
    case 'ASME_B31_3':
      config.component_name = '工艺管道'
      config.material_spec = 'A106 Gr.B'
      config.design_code = 'ASME B31.3 Process Piping'
      config.operating_conditions.temperature = 200
      config.operating_conditions.pressure = 1.6
      config.operating_conditions.cyclic_loading = true
      break
    case 'ISO_15614':
      config.component_name = '焊接接头'
      config.material_spec = 'S355JR'
      config.design_code = 'ISO 15614-1'
      config.operating_conditions.temperature = 20
      config.operating_conditions.pressure = 0
      config.operating_conditions.cyclic_loading = false
      break
    case 'ASME_Y14_5':
      config.component_name = '机械零件'
      config.material_spec = '6061-T6'
      config.design_code = 'ASME Y14.5-2018'
      config.operating_conditions.temperature = 25
      config.operating_conditions.pressure = 0
      config.operating_conditions.cyclic_loading = false
      break
    default:
      break
  }
}

function checkCompliance() {
  if (!config.standard) return
  generating.value = true

  // 模拟合规检查清单数据
  const checklist = generateMockComplianceChecklist(config.standard)

  // 更新应力结果状态
  const checkedStressResults = config.stress_results.map(row => {
    const ratio = row.allowable > 0 ? row.value / row.allowable : 0
    return {
      ...row,
      utilization_ratio: ratio,
      status: (ratio <= 0.8 ? 'pass' : ratio <= 1.0 ? 'warning' : 'fail') as StressResult['status']
    }
  })

  // 更新安全系数状态
  const checkedSafetyFactors = config.safety_factors.map(sf => ({
    ...sf,
    status: (sf.safety_factor >= sf.minimum_required ? 'pass' : 'fail') as SafetyFactorResult['status']
  }))

  // 计算总体状态
  const hasFail = checkedStressResults.some(r => r.status === 'fail') || checkedSafetyFactors.some(s => s.status === 'fail')
  const hasWarning = checkedStressResults.some(r => r.status === 'warning') || checkedSafetyFactors.some(s => s.status === 'warning')
  const overallStatus = hasFail ? 'fail' as const : hasWarning ? 'conditional' as const : 'pass' as const

  const compliantCount = checklist.filter(i => i.status === 'compliant').length
  const nonCompliantCount = checklist.filter(i => i.status === 'non_compliant').length
  const pendingCount = checklist.filter(i => i.status === 'pending').length

  result.value = {
    success: overallStatus !== 'fail',
    overall_status: overallStatus,
    compliance_summary: {
      total: checklist.length,
      compliant: compliantCount,
      non_compliant: nonCompliantCount,
      pending: pendingCount
    },
    stress_check_results: checkedStressResults,
    safety_factor_results: checkedSafetyFactors,
    compliance_items: checklist,
    report_generated_at: new Date().toISOString(),
    report_file_path: '',
    recommendations: generateMockRecommendations(overallStatus, checkedStressResults, checkedSafetyFactors)
  }

  setTimeout(() => {
    generating.value = false
  }, 600)
}

function generateReport() {
  generating.value = true

  // 模拟报告生成
  const checklist = generateMockComplianceChecklist(config.standard)
  const checkedStressResults = config.stress_results.map(row => {
    const ratio = row.allowable > 0 ? row.value / row.allowable : 0
    return {
      ...row,
      utilization_ratio: ratio,
      status: (ratio <= 0.8 ? 'pass' : ratio <= 1.0 ? 'warning' : 'fail') as StressResult['status']
    }
  })
  const checkedSafetyFactors = config.safety_factors.map(sf => ({
    ...sf,
    status: (sf.safety_factor >= sf.minimum_required ? 'pass' : 'fail') as SafetyFactorResult['status']
  }))

  const hasFail = checkedStressResults.some(r => r.status === 'fail') || checkedSafetyFactors.some(s => s.status === 'fail')
  const hasWarning = checkedStressResults.some(r => r.status === 'warning') || checkedSafetyFactors.some(s => s.status === 'warning')
  const overallStatus = hasFail ? 'fail' as const : hasWarning ? 'conditional' as const : 'pass' as const

  const compliantCount = checklist.filter(i => i.status === 'compliant').length
  const nonCompliantCount = checklist.filter(i => i.status === 'non_compliant').length
  const pendingCount = checklist.filter(i => i.status === 'pending').length

  setTimeout(() => {
    result.value = {
      success: overallStatus !== 'fail',
      overall_status: overallStatus,
      compliance_summary: {
        total: checklist.length,
        compliant: compliantCount,
        non_compliant: nonCompliantCount,
        pending: pendingCount
      },
      stress_check_results: checkedStressResults,
      safety_factor_results: checkedSafetyFactors,
      compliance_items: checklist,
      report_generated_at: new Date().toISOString(),
      report_file_path: `/reports/${config.report_number || 'CERT-REPORT'}.pdf`,
      recommendations: generateMockRecommendations(overallStatus, checkedStressResults, checkedSafetyFactors)
    }
    generating.value = false
  }, 1200)
}

function exportPDF() {
  if (!result.value) return
  // 模拟PDF导出
  const filePath = `/reports/${config.report_number || 'CERT-REPORT'}_${Date.now()}.pdf`
  result.value.report_file_path = filePath
  alert(`PDF报告已导出至: ${filePath}`)
}

// ============ 模拟数据生成 ============

function generateMockComplianceChecklist(standard: CertificationStandard): ComplianceItem[] {
  const baseChecklists: Record<CertificationStandard, ComplianceItem[]> = {
    ASME_Y14_5: [
      { clause_id: '2.1', description: '尺寸公差标注', requirement: '所有关键尺寸须标注公差', evidence: '图纸审查记录', status: 'compliant', notes: '' },
      { clause_id: '2.2', description: '形位公差标注', requirement: '关键特征须标注形位公差', evidence: 'GD&T检查表', status: 'compliant', notes: '' },
      { clause_id: '2.5', description: '基准选择', requirement: '基准选择须符合功能要求', evidence: '基准分析报告', status: 'compliant', notes: '' },
      { clause_id: '3.1', description: '最大实体要求', requirement: 'MMC/LMC条件正确应用', evidence: '公差分析报告', status: 'pending', notes: '待补充公差分析' },
      { clause_id: '4.1', description: '轮廓度公差', requirement: '面轮廓度公差须覆盖功能面', evidence: '检测报告', status: 'compliant', notes: '' },
      { clause_id: '5.2', description: '复合位置度', requirement: '复合位置度公差框格正确', evidence: '图纸检查', status: 'non_compliant', notes: 'PLTZF与FRTZF方向标识缺失' },
      { clause_id: '6.1', description: '图纸标注完整性', requirement: '所有GD&T符号符合标准', evidence: '图纸审查', status: 'compliant', notes: '' }
    ],
    ISO_2553: [
      { clause_id: '4.1', description: '焊接符号基本要素', requirement: '焊接符号包含所有必要要素', evidence: '图纸审查记录', status: 'compliant', notes: '' },
      { clause_id: '4.3', description: '焊缝尺寸标注', requirement: '焊缝尺寸须完整标注', evidence: 'WPS文件', status: 'compliant', notes: '' },
      { clause_id: '5.1', description: '焊接方法标识', requirement: '焊接方法代号正确', evidence: 'WPS编号', status: 'compliant', notes: '' },
      { clause_id: '5.4', description: '辅助符号', requirement: '现场焊、周围焊等辅助符号正确', evidence: '图纸检查', status: 'pending', notes: '现场焊标识待确认' },
      { clause_id: '6.2', description: '箭头侧与接头侧', requirement: '箭头侧与非箭头侧区分明确', evidence: '图纸审查', status: 'compliant', notes: '' },
      { clause_id: '7.1', description: '焊缝尾标', requirement: '焊接工艺及相关标准标注完整', evidence: 'WPS引用', status: 'non_compliant', notes: '缺少ISO 15614引用' }
    ],
    ASME_BPVC_VIII: [
      { clause_id: 'UG-16', description: '材料厚度限制', requirement: '壳体最小厚度满足要求', evidence: '材料证书', status: 'compliant', notes: '' },
      { clause_id: 'UG-22', description: '载荷组合', requirement: '设计载荷组合覆盖所有工况', evidence: '载荷分析报告', status: 'compliant', notes: '' },
      { clause_id: 'UG-23', description: '许用应力', requirement: '许用应力取值符合规范', evidence: '材料许用应力表', status: 'compliant', notes: '' },
      { clause_id: 'UG-27', description: '内压壳体计算', requirement: '壳体厚度计算正确', evidence: '强度计算书', status: 'compliant', notes: '' },
      { clause_id: 'UG-32', description: '封头设计', requirement: '封头厚度满足规范要求', evidence: '封头计算书', status: 'pending', notes: '过渡区厚度余量偏小' },
      { clause_id: 'UG-37', description: '开孔补强', requirement: '开孔补强计算满足要求', evidence: '补强计算书', status: 'non_compliant', notes: '接管补强面积不足' },
      { clause_id: 'UG-99', description: '压力试验', requirement: '水压试验压力和程序符合规范', evidence: '试验程序', status: 'pending', notes: '待安排水压试验' },
      { clause_id: 'UW-11', description: '焊缝系数', requirement: '焊缝系数选取正确', evidence: 'NDE报告', status: 'compliant', notes: '' },
      { clause_id: 'UW-16', description: '接管连接', requirement: '接管连接形式符合规范', evidence: '图纸审查', status: 'compliant', notes: '' },
      { clause_id: 'UCS-66', description: '冲击试验豁免', requirement: '材料冲击试验要求评估', evidence: 'MDMT评估报告', status: 'compliant', notes: '' }
    ],
    ISO_15614: [
      { clause_id: '6.1', description: '母材分组', requirement: '母材按ISO 15608正确分组', evidence: '材料证书', status: 'compliant', notes: '' },
      { clause_id: '6.3', description: '焊接工艺参数范围', requirement: '焊接参数在评定范围内', evidence: 'WPS参数记录', status: 'compliant', notes: '' },
      { clause_id: '7.1', description: '试件制备', requirement: '试件尺寸和制备符合要求', evidence: '试件制备记录', status: 'compliant', notes: '' },
      { clause_id: '8.1', description: '外观检验', requirement: '焊缝外观质量合格', evidence: 'VT报告', status: 'compliant', notes: '' },
      { clause_id: '8.2', description: '无损检测', requirement: 'RT/UT检测结果合格', evidence: 'NDE报告', status: 'pending', notes: 'UT检测待完成' },
      { clause_id: '8.3', description: '力学性能试验', requirement: '拉伸和弯曲试验合格', evidence: '力学试验报告', status: 'compliant', notes: '' },
      { clause_id: '8.4', description: '冲击试验', requirement: '冲击韧性满足要求', evidence: '冲击试验报告', status: 'non_compliant', notes: '焊缝热影响区冲击值偏低' },
      { clause_id: '9.1', description: '评定有效期', requirement: '评定在有效期内', evidence: '评定证书', status: 'compliant', notes: '' }
    ],
    ASME_B31_3: [
      { clause_id: '302.2', description: '设计条件', requirement: '设计压力和温度确定正确', evidence: '工艺条件表', status: 'compliant', notes: '' },
      { clause_id: '302.3', description: '许用应力', requirement: '材料许用应力取值正确', evidence: 'B31.3许用应力表', status: 'compliant', notes: '' },
      { clause_id: '304.1', description: '直管壁厚计算', requirement: '直管壁厚计算正确', evidence: '壁厚计算书', status: 'compliant', notes: '' },
      { clause_id: '304.3', description: '弯管和三通', requirement: '管件应力分析满足要求', evidence: '管件分析报告', status: 'pending', notes: '弯头壁厚减薄需关注' },
      { clause_id: '306.1', description: '压力试验', requirement: '水压试验要求满足', evidence: '试验程序', status: 'pending', notes: '待安排压力试验' },
      { clause_id: '319.1', description: '柔性分析', requirement: '管道柔性分析满足要求', evidence: '应力分析报告', status: 'compliant', notes: '' },
      { clause_id: '323.1', description: '材料要求', requirement: '材料追溯和标识满足要求', evidence: '材料证书', status: 'compliant', notes: '' },
      { clause_id: '328.1', description: '焊接要求', requirement: '焊接符合WPS和规范要求', evidence: 'WPS和NDE记录', status: 'non_compliant', notes: '部分焊口NDE覆盖率不足' },
      { clause_id: '331.1', description: '检测', requirement: '检测方法和比例符合要求', evidence: 'NDE程序', status: 'compliant', notes: '' },
      { clause_id: '341.1', description: '检查与试验', requirement: '检查和试验要求满足', evidence: 'ITP文件', status: 'compliant', notes: '' }
    ],
    ISO_9001: [
      { clause_id: '4.1', description: '组织环境', requirement: '理解组织及其环境', evidence: '环境分析报告', status: 'compliant', notes: '' },
      { clause_id: '4.2', description: '相关方需求', requirement: '理解相关方的需求和期望', evidence: '相关方分析', status: 'compliant', notes: '' },
      { clause_id: '5.1', description: '领导作用', requirement: '最高管理者对质量管理体系的领导', evidence: '管理评审记录', status: 'compliant', notes: '' },
      { clause_id: '6.1', description: '风险和机遇', requirement: '确定需应对的风险和机遇', evidence: '风险登记册', status: 'pending', notes: '风险评审周期待更新' },
      { clause_id: '7.1', description: '资源', requirement: '确定并提供所需资源', evidence: '资源清单', status: 'compliant', notes: '' },
      { clause_id: '7.5', description: '成文信息', requirement: '质量管理体系文件化信息控制', evidence: '文件控制程序', status: 'compliant', notes: '' },
      { clause_id: '8.1', description: '运行策划', requirement: '运行策划和控制', evidence: '项目计划', status: 'compliant', notes: '' },
      { clause_id: '8.5', description: '生产和服务提供', requirement: '生产和服务提供的控制', evidence: '作业指导书', status: 'pending', notes: '部分作业指导书版本过期' },
      { clause_id: '9.1', description: '监视测量分析评价', requirement: '顾客满意度和内部审核', evidence: '审核报告', status: 'compliant', notes: '' },
      { clause_id: '10.1', description: '持续改进', requirement: '确定改进机会并实施', evidence: '改进计划', status: 'non_compliant', notes: '纠正措施跟踪不完整' }
    ]
  }

  return baseChecklists[standard] || baseChecklists.ASME_BPVC_VIII
}

function generateMockRecommendations(
  overallStatus: string,
  stressResults: StressResult[],
  safetyFactors: SafetyFactorResult[]
): string[] {
  const recommendations: string[] = []

  // 基于应力结果的建议
  const failedStress = stressResults.filter(r => r.status === 'fail')
  const warningStress = stressResults.filter(r => r.status === 'warning')

  for (const item of failedStress) {
    recommendations.push(
      `[关键] ${item.location} 处 ${stressTypeLabel(item.stress_type)} 超标 (${(item.utilization_ratio * 100).toFixed(1)}%)，建议增加壁厚或优化几何形状以降低应力集中。`
    )
  }

  for (const item of warningStress) {
    recommendations.push(
      `[注意] ${item.location} 处 ${stressTypeLabel(item.stress_type)} 利用率较高 (${(item.utilization_ratio * 100).toFixed(1)}%)，建议关注服役期间的应力监测。`
    )
  }

  // 基于安全系数的建议
  const failedSF = safetyFactors.filter(s => s.status === 'fail')
  for (const item of failedSF) {
    recommendations.push(
      `[关键] ${item.component_name} 安全系数 (${item.safety_factor.toFixed(2)}) 低于最小要求 (${item.minimum_required.toFixed(2)})，关键位置: ${item.critical_location}，建议进行结构优化或更换材料。`
    )
  }

  // 基于总体状态的建议
  if (overallStatus === 'pass') {
    recommendations.push('所有校核项目均满足规范要求，建议按计划进行压力试验和最终检验。')
    recommendations.push('建议建立定期检验计划，确保服役期间的安全裕度持续满足要求。')
  } else if (overallStatus === 'conditional') {
    recommendations.push('部分校核项目存在预警，建议在整改完成前限制操作条件（温度/压力）不超过设计值的80%。')
    recommendations.push('建议对预警区域增加无损检测覆盖范围，并在投用后进行定期监测。')
  } else {
    recommendations.push('存在不满足规范要求的项目，必须在整改完成并通过复验后方可进入下一阶段。')
    recommendations.push('建议组织设计评审会议，针对不合规项制定详细的整改方案和时间表。')
    recommendations.push('整改完成后须重新进行完整的应力分析和安全系数校核。')
  }

  return recommendations
}
</script>

<style scoped>
.label {
  display: block;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.panel-section {
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}
</style>
