<template>
  <div class="wizard-step-content">
    <!-- 几何步骤 -->
    <div v-if="component === 'geometry'" class="step-geometry">
      <div class="param-card">
        <h4>几何参数</h4>
        <div class="param-list">
          <div
            v-for="(value, key) in template.geometry.params"
            :key="key"
            class="param-item"
          >
            <label>{{ paramLabel(key) }}</label>
            <div class="param-value">
              <input
                type="number"
                :value="value"
                @input="updateParam(key, ($event.target as HTMLInputElement).valueAsNumber)"
              >
              <span class="unit">{{ template.geometry.unit }}</span>
            </div>
          </div>
        </div>
      </div>
      <div class="geometry-preview">
        <div class="preview-placeholder">
          <span class="preview-icon">📐</span>
          <span class="preview-text">几何预览</span>
          <span class="preview-hint">{{ geometryHint }}</span>
        </div>
      </div>
    </div>

    <!-- 材料步骤 -->
    <div v-else-if="component === 'material'" class="step-material">
      <div class="material-card">
        <div class="material-header">
          <span class="material-name">{{ template.material.name }}</span>
          <span class="material-category">{{ template.material.category }}</span>
        </div>
        <div class="material-props">
          <div class="prop-item">
            <span class="prop-label">弹性模量 E</span>
            <span class="prop-value">{{ (template.material.elasticModulus / 1e9).toFixed(1) }} GPa</span>
          </div>
          <div class="prop-item">
            <span class="prop-label">泊松比 ν</span>
            <span class="prop-value">{{ template.material.poissonsRatio }}</span>
          </div>
          <div class="prop-item">
            <span class="prop-label">密度 ρ</span>
            <span class="prop-value">{{ template.material.density }} kg/m³</span>
          </div>
          <div v-if="template.material.yieldStrength" class="prop-item">
            <span class="prop-label">屈服强度 σy</span>
            <span class="prop-value">{{ (template.material.yieldStrength / 1e6).toFixed(0) }} MPa</span>
          </div>
        </div>
      </div>
      <div class="material-note">
        <p>💡 材料参数已根据案例预设。你可以在材料库中选择其他材料，或手动修改参数。</p>
      </div>
    </div>

    <!-- 网格步骤 -->
    <div v-else-if="component === 'mesh'" class="step-mesh">
      <div class="mesh-config">
        <div class="config-row">
          <label>单元类型</label>
          <select :value="template.mesh.elementType" @change="updateMesh('elementType', ($event.target as HTMLSelectElement).value)">
            <option value="tet4">Tet4（一阶四面体）</option>
            <option value="tet10">Tet10（二阶四面体）</option>
            <option value="hex8">Hex8（一阶六面体）</option>
            <option value="hex20">Hex20（二阶六面体）</option>
          </select>
        </div>
        <div class="config-row">
          <label>全局网格尺寸</label>
          <div class="input-with-unit">
            <input
              type="number"
              :value="template.mesh.globalSize"
              @input="updateMesh('globalSize', ($event.target as HTMLInputElement).valueAsNumber)"
            >
            <span>mm</span>
          </div>
        </div>
        <div class="config-row">
          <label>质量目标</label>
          <div class="quality-slider">
            <input
              type="range"
              min="0.3"
              max="0.9"
              step="0.05"
              :value="template.mesh.qualityTarget"
              @input="updateMesh('qualityTarget', ($event.target as HTMLInputElement).valueAsNumber)"
            >
            <span>{{ (template.mesh.qualityTarget * 100).toFixed(0) }}%</span>
          </div>
        </div>
      </div>
      <div v-if="template.mesh.localRefinement?.length" class="refinement-list">
        <h4>局部加密区域</h4>
        <div
          v-for="(ref, i) in template.mesh.localRefinement"
          :key="i"
          class="refinement-item"
        >
          <span class="ref-region">{{ ref.region }}</span>
          <span class="ref-size">{{ ref.size }} mm</span>
          <span class="ref-desc">{{ ref.description }}</span>
        </div>
      </div>
    </div>

    <!-- 边界条件步骤 -->
    <div v-else-if="component === 'boundary'" class="step-boundary">
      <div
        v-for="(bc, i) in template.boundaryConditions"
        :key="i"
        class="bc-card"
      >
        <div class="bc-header">
          <span class="bc-type">{{ bcTypeLabel(bc.type) }}</span>
          <span class="bc-region">{{ bc.region }}</span>
        </div>
        <p class="bc-description">{{ bc.description }}</p>
        <div v-if="Object.keys(bc.values).length > 0" class="bc-values">
          <div
            v-for="(val, key) in bc.values"
            :key="key"
            class="bc-value-item"
          >
            <span class="value-key">{{ key }}</span>
            <span class="value-val">{{ val }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 求解器步骤 -->
    <div v-else-if="component === 'solver'" class="step-solver">
      <div class="solver-config">
        <div class="config-row">
          <label>分析类型</label>
          <span class="config-value">{{ analysisTypeLabel(template.solver.analysisType) }}</span>
        </div>
        <div class="config-row">
          <label>求解器类型</label>
          <span class="config-value">{{ template.solver.solverType === 'direct' ? '直接求解器' : '迭代求解器' }}</span>
        </div>
      </div>
      <div class="solver-note">
        <p>⚙️ 求解器参数已针对本案例优化。对于初学者，建议使用默认设置。</p>
      </div>
    </div>

    <!-- 检查步骤 -->
    <div v-else-if="component === 'review'" class="step-review">
      <div class="review-checklist">
        <div class="check-item">
          <input type="checkbox" id="check-geo" :checked="true">
          <label for="check-geo">几何尺寸正确</label>
        </div>
        <div class="check-item">
          <input type="checkbox" id="check-mat" :checked="true">
          <label for="check-mat">材料参数合理</label>
        </div>
        <div class="check-item">
          <input type="checkbox" id="check-mesh" :checked="true">
          <label for="check-mesh">网格密度足够</label>
        </div>
        <div class="check-item">
          <input type="checkbox" id="check-bc" :checked="true">
          <label for="check-bc">边界条件无误</label>
        </div>
      </div>
      <div v-if="template.expectedResults" class="expected-results">
        <h4>理论参考值</h4>
        <div class="result-items">
          <div v-if="template.expectedResults.maxStress" class="result-item">
            <span>最大应力</span>
            <span>{{ (template.expectedResults.maxStress / 1e6).toFixed(1) }} MPa</span>
          </div>
          <div v-if="template.expectedResults.maxDisplacement" class="result-item">
            <span>最大位移</span>
            <span>{{ (template.expectedResults.maxDisplacement * 1000).toFixed(2) }} mm</span>
          </div>
          <div v-if="template.expectedResults.safetyFactor" class="result-item">
            <span>安全系数</span>
            <span>{{ template.expectedResults.safetyFactor.toFixed(2) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 结果步骤 -->
    <div v-else-if="component === 'result'" class="step-result">
      <div class="result-placeholder">
        <span class="result-icon">📊</span>
        <h3>仿真结果查看</h3>
        <p>点击「运行仿真」后将在此显示应力云图、位移云图和结果数据。</p>
        <p class="result-hint">向导完成后，你可以在仿真模块中查看详细结果并生成报告。</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SimulationTemplate } from '../../data/templates/simulationTemplates'

const props = defineProps<{
  component: string
  template: SimulationTemplate
  stepId: string
  stepData?: Record<string, unknown>
}>()

const emit = defineEmits<{
  update: [data: Record<string, unknown>]
}>()

function updateParam(key: string, value: number) {
  emit('update', { [key]: value })
}

function updateMesh(key: string, value: string | number) {
  emit('update', { [key]: value })
}

function paramLabel(key: string): string {
  const labels: Record<string, string> = {
    length: '长度',
    width: '宽度',
    height: '高度',
    thickness: '厚度',
    holeDiameter: '孔径',
    armLength: '臂长',
    armWidth: '臂宽',
    filletRadius: '圆角半径'
  }
  return labels[key] ?? key
}

const geometryHint = computed(() => {
  const params = props.template.geometry.params
  const dims = Object.keys(params).length
  return `${dims} 个参数 | 单位: ${props.template.geometry.unit}`
})

function bcTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    fixed: '固定约束',
    displacement: '位移约束',
    force: '集中力',
    pressure: '均布压力',
    temperature: '温度载荷',
    symmetry: '对称约束'
  }
  return labels[type] ?? type
}

function analysisTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    static: '静力分析',
    modal: '模态分析',
    thermal: '热分析',
    buckling: '屈曲分析',
    fatigue: '疲劳分析'
  }
  return labels[type] ?? type
}

import { computed } from 'vue'
</script>

<style scoped>
.wizard-step-content {
  width: 100%;
}

/* 几何步骤 */
.step-geometry {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}

.param-card {
  background: #f8fafc;
  border-radius: 8px;
  padding: 16px;
}

.param-card h4 {
  margin: 0 0 12px 0;
  font-size: 15px;
  color: var(--text-primary, #1e293b);
}

.param-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.param-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.param-item label {
  font-size: 14px;
  color: var(--text-secondary, #64748b);
}

.param-value {
  display: flex;
  align-items: center;
  gap: 6px;
}

.param-value input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  text-align: right;
  font-size: 14px;
}

.unit {
  font-size: 13px;
  color: var(--text-secondary, #94a3b8);
}

.geometry-preview {
  background: #f1f5f9;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 200px;
}

.preview-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary, #94a3b8);
}

.preview-icon {
  font-size: 48px;
}

.preview-text {
  font-size: 16px;
  font-weight: 600;
}

.preview-hint {
  font-size: 13px;
}

/* 材料步骤 */
.step-material {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.material-card {
  background: #f8fafc;
  border-radius: 8px;
  padding: 20px;
}

.material-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.material-name {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.material-category {
  padding: 4px 10px;
  border-radius: 12px;
  background: #e0f2fe;
  color: #0369a1;
  font-size: 12px;
}

.material-props {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.prop-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: white;
  border-radius: 6px;
}

.prop-label {
  font-size: 13px;
  color: var(--text-secondary, #64748b);
}

.prop-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.material-note {
  padding: 12px 16px;
  background: #eff6ff;
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-secondary, #475569);
}

/* 网格步骤 */
.step-mesh {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.mesh-config {
  background: #f8fafc;
  border-radius: 8px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.config-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.config-row label {
  font-size: 14px;
  color: var(--text-secondary, #64748b);
}

.config-row select,
.config-row input {
  padding: 6px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
}

.input-with-unit {
  display: flex;
  align-items: center;
  gap: 6px;
}

.quality-slider {
  display: flex;
  align-items: center;
  gap: 10px;
}

.refinement-list {
  background: #fefce8;
  border-radius: 8px;
  padding: 16px;
}

.refinement-list h4 {
  margin: 0 0 10px 0;
  font-size: 14px;
  color: #a16207;
}

.refinement-item {
  display: flex;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid #fef08a;
  font-size: 14px;
}

.ref-region {
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.ref-size {
  color: var(--primary-color, #3b82f6);
}

.ref-desc {
  color: var(--text-secondary, #64748b);
  flex: 1;
}

/* 边界条件 */
.step-boundary {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.bc-card {
  background: #f8fafc;
  border-radius: 8px;
  padding: 16px;
  border-left: 4px solid var(--primary-color, #3b82f6);
}

.bc-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.bc-type {
  font-weight: 600;
  color: var(--primary-color, #3b82f6);
}

.bc-region {
  font-size: 13px;
  color: var(--text-secondary, #94a3b8);
}

.bc-description {
  font-size: 14px;
  color: var(--text-secondary, #64748b);
  margin: 0 0 10px 0;
}

.bc-values {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.bc-value-item {
  background: white;
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 13px;
}

.value-key {
  color: var(--text-secondary, #94a3b8);
  margin-right: 4px;
}

.value-val {
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

/* 求解器 */
.step-solver {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.solver-config {
  background: #f8fafc;
  border-radius: 8px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.config-value {
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.solver-note {
  padding: 12px 16px;
  background: #f0fdf4;
  border-radius: 8px;
  font-size: 14px;
  color: #166534;
}

/* 检查 */
.step-review {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.review-checklist {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.check-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  background: #f8fafc;
  border-radius: 8px;
}

.check-item input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--primary-color, #3b82f6);
}

.expected-results {
  background: #eff6ff;
  border-radius: 8px;
  padding: 16px;
}

.expected-results h4 {
  margin: 0 0 12px 0;
  color: var(--primary-color, #3b82f6);
}

.result-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: white;
  border-radius: 6px;
  font-size: 14px;
}

/* 结果 */
.step-result {
  text-align: center;
  padding: 40px 20px;
}

.result-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: var(--text-secondary, #64748b);
}

.result-icon {
  font-size: 64px;
}

.result-placeholder h3 {
  color: var(--text-primary, #1e293b);
  margin: 0;
}

.result-hint {
  font-size: 13px;
  color: var(--text-secondary, #94a3b8);
}
</style>
