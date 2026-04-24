<template>
  <div class="ml-prediction-panel">
    <!-- 模型选择 -->
    <div class="panel-header">
      <h3 class="panel-title">🧠 ML 材料性能预测</h3>
      <div class="model-selector">
        <select v-model="selectedModel" class="model-select" :disabled="isPredicting">
          <option v-for="model in models" :key="model.name" :value="model.name">
            {{ model.description }}
          </option>
        </select>
        <span v-if="currentModel?.isLoaded" class="model-status loaded">● 已加载</span>
        <span v-else class="model-status">○ 未加载</span>
      </div>
    </div>

    <!-- 成分输入 -->
    <div class="composition-section">
      <div class="section-label">材料成分 (at%)</div>
      <div class="composition-grid">
        <div v-for="(value, element) in composition" :key="element" class="composition-item">
          <label class="element-label">{{ element }}</label>
          <input
            type="number"
            :value="value"
            @input="updateComposition(element, $event)"
            class="element-input"
            :disabled="isPredicting"
            min="0"
            max="100"
            step="0.1"
          />
          <span class="element-unit">%</span>
          <button class="remove-btn" @click="removeElement(element)" :disabled="isPredicting">✕</button>
        </div>
      </div>
      <div class="add-element">
        <input
          v-model="newElement"
          placeholder="元素符号 (如 Fe, Al, Ti)"
          class="add-input"
          @keyup.enter="addElement"
          :disabled="isPredicting"
        />
        <button class="add-btn" @click="addElement" :disabled="isPredicting || !newElement.trim()">
          + 添加
        </button>
      </div>
    </div>

    <!-- 预测目标选择 -->
    <div class="target-section">
      <div class="section-label">预测属性</div>
      <div class="target-chips">
        <label
          v-for="prop in availableProperties"
          :key="prop.key"
          class="chip"
          :class="{ active: selectedProperties.includes(prop.key) }"
        >
          <input
            type="checkbox"
            :value="prop.key"
            v-model="selectedProperties"
            :disabled="isPredicting"
          />
          {{ prop.label }}
        </label>
      </div>
    </div>

    <!-- 预测按钮 -->
    <button
      class="predict-btn"
      @click="runPrediction"
      :disabled="isPredicting || selectedProperties.length === 0 || Object.keys(composition).length === 0"
    >
      <span v-if="isPredicting" class="loading-spinner">⟳</span>
      <span v-else>⚡ 运行预测</span>
    </button>

    <!-- 预测结果 -->
    <div v-if="predictionResult" class="results-section">
      <div class="results-header">
        <span class="results-title">预测结果</span>
        <span class="results-meta">
          {{ predictionResult.modelName }} · {{ predictionResult.inferenceTimeMs }}ms
          <span v-if="predictionResult.isMock" class="mock-badge">Mock</span>
        </span>
      </div>

      <div class="prediction-cards">
        <div
          v-for="pred in predictionResult.predictions"
          :key="pred.propertyName"
          class="prediction-card"
        >
          <div class="card-header">
            <span class="property-name">{{ getPropertyLabel(pred.propertyName) }}</span>
            <span class="property-unit">{{ pred.unit }}</span>
          </div>
          <div class="card-value">
            {{ formatValue(pred.predictedValue, pred.propertyName) }}
          </div>
          <div class="card-confidence">
            <div class="confidence-bar">
              <div
                class="confidence-fill"
                :style="{
                  left: getConfidencePosition(pred) + '%',
                  width: getConfidenceWidth(pred) + '%'
                }"
              ></div>
              <div
                class="confidence-marker"
                :style="{ left: getConfidencePosition(pred) + '%' }"
              ></div>
            </div>
            <div class="confidence-label">
              ± {{ formatUncertainty(pred) }}
            </div>
          </div>
        </div>
      </div>

      <!-- 一键验证按钮 -->
      <button class="verify-btn" @click="$emit('verify', predictionResult)">
        🔬 一键 CAELab 验证
      </button>
    </div>

    <!-- 错误提示 -->
    <div v-if="errorMsg" class="error-msg">{{ errorMsg }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  predictMaterialProperties,
  listMlModels,
  type PredictionResponse,
  type ModelInfo,
  type Composition,
} from '../../api/mlPredict'

const emit = defineEmits<{
  verify: [result: PredictionResponse]
}>()

// 模型相关
const models = ref<ModelInfo[]>([])
const selectedModel = ref('mock_chgnet')
const currentModel = computed(() => models.value.find(m => m.name === selectedModel.value))

// 成分输入
const composition = ref<Composition>({
  Fe: 70.0,
  Cr: 19.0,
  Ni: 9.0,
})
const newElement = ref('')

// 预测目标
const availableProperties = [
  { key: 'elastic_modulus', label: '弹性模量' },
  { key: 'yield_strength', label: '屈服强度' },
  { key: 'thermal_conductivity', label: '热导率' },
  { key: 'density', label: '密度' },
  { key: 'poissons_ratio', label: '泊松比' },
  { key: 'shear_modulus', label: '剪切模量' },
  { key: 'bulk_modulus', label: '体积模量' },
]
const selectedProperties = ref<string[]>(['elastic_modulus', 'yield_strength', 'thermal_conductivity'])

// 状态
const isPredicting = ref(false)
const predictionResult = ref<PredictionResponse | null>(null)
const errorMsg = ref('')

// 属性标签映射
const propertyLabels: Record<string, string> = {
  elastic_modulus: '弹性模量',
  yield_strength: '屈服强度',
  thermal_conductivity: '热导率',
  density: '密度',
  poissons_ratio: '泊松比',
  shear_modulus: '剪切模量',
  bulk_modulus: '体积模量',
}

function getPropertyLabel(key: string): string {
  return propertyLabels[key] || key
}

// 格式化预测值
function formatValue(value: number, property: string): string {
  if (property === 'poissons_ratio') return value.toFixed(3)
  if (value >= 1e9) return (value / 1e9).toFixed(2) + ' GPa'
  if (value >= 1e6) return (value / 1e6).toFixed(1) + ' MPa'
  if (value >= 1e3) return (value / 1e3).toFixed(1) + ' kPa'
  return value.toFixed(2)
}

// 格式化不确定性
function formatUncertainty(pred: { uncertainty: number; propertyName: string }): string {
  const u = pred.uncertainty
  if (pred.propertyName === 'poissons_ratio') return u.toFixed(4)
  if (u >= 1e9) return (u / 1e9).toFixed(2) + ' GPa'
  if (u >= 1e6) return (u / 1e6).toFixed(1) + ' MPa'
  if (u >= 1e3) return (u / 1e3).toFixed(1) + ' kPa'
  return u.toFixed(2)
}

// 置信区间可视化
function getConfidencePosition(pred: { predictedValue: number; confidenceLower: number; confidenceUpper: number }): number {
  const range = pred.confidenceUpper - pred.confidenceLower
  if (range === 0) return 50
  return ((pred.predictedValue - pred.confidenceLower) / range) * 100
}

function getConfidenceWidth(pred: { predictedValue: number; confidenceLower: number; confidenceUpper: number }): number {
  const range = pred.confidenceUpper - pred.confidenceLower
  if (range === 0) return 0
  return ((pred.predictedValue - pred.confidenceLower) / range) * 100
}

// 成分操作
function updateComposition(element: string, event: Event) {
  const target = event.target as HTMLInputElement
  composition.value[element] = parseFloat(target.value) || 0
}

function addElement() {
  const el = newElement.value.trim()
  if (el && !(el in composition.value)) {
    composition.value[el] = 0.0
    newElement.value = ''
  }
}

function removeElement(element: string) {
  delete composition.value[element]
  composition.value = { ...composition.value }
}

// 运行预测
async function runPrediction() {
  isPredicting.value = true
  errorMsg.value = ''
  predictionResult.value = null

  try {
    const result = await predictMaterialProperties({
      composition: composition.value,
      targetProperties: selectedProperties.value,
      modelName: selectedModel.value,
    })
    predictionResult.value = result
  } catch (e) {
    errorMsg.value = `预测失败: ${e}`
  } finally {
    isPredicting.value = false
  }
}

// 加载模型列表
onMounted(async () => {
  try {
    models.value = await listMlModels()
  } catch (e) {
    errorMsg.value = `加载模型列表失败: ${e}`
  }
})
</script>

<style scoped>
.ml-prediction-panel {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 12px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.panel-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
}

.model-selector {
  display: flex;
  align-items: center;
  gap: 8px;
}

.model-select {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  color: var(--text-primary, #cdd6f4);
  padding: 4px 8px;
  font-size: 12px;
}

.model-status {
  font-size: 11px;
}
.model-status.loaded {
  color: #a6e3a1;
}

.section-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary, #a6adc8);
  margin-bottom: 8px;
}

.composition-section,
.target-section {
  margin-bottom: 16px;
}

.composition-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 8px;
  margin-bottom: 8px;
}

.composition-item {
  display: flex;
  align-items: center;
  gap: 4px;
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  padding: 4px 8px;
}

.element-label {
  font-weight: 600;
  font-size: 13px;
  color: var(--text-primary, #cdd6f4);
  min-width: 24px;
}

.element-input {
  width: 60px;
  background: transparent;
  border: none;
  color: var(--text-primary, #cdd6f4);
  font-size: 13px;
  text-align: right;
}
.element-input:focus {
  outline: none;
}

.element-unit {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
}

.remove-btn {
  background: none;
  border: none;
  color: var(--text-secondary, #a6adc8);
  cursor: pointer;
  padding: 0 2px;
  font-size: 12px;
}
.remove-btn:hover {
  color: #f38ba8;
}

.add-element {
  display: flex;
  gap: 8px;
}

.add-input {
  flex: 1;
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 6px;
  color: var(--text-primary, #cdd6f4);
  padding: 6px 10px;
  font-size: 13px;
}

.add-btn {
  background: var(--accent-color, #89b4fa);
  border: none;
  border-radius: 6px;
  color: var(--bg-primary, #11111b);
  padding: 6px 12px;
  font-size: 13px;
  cursor: pointer;
  font-weight: 500;
}

.target-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.chip {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border: 1px solid var(--border-color, #313244);
  border-radius: 16px;
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  cursor: pointer;
  transition: all 0.2s;
}
.chip input {
  display: none;
}
.chip.active {
  background: rgba(137, 180, 250, 0.15);
  border-color: var(--accent-color, #89b4fa);
  color: var(--accent-color, #89b4fa);
}

.predict-btn {
  width: 100%;
  padding: 10px;
  background: linear-gradient(135deg, #89b4fa, #74c7ec);
  border: none;
  border-radius: 8px;
  color: #11111b;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
  margin-bottom: 16px;
}
.predict-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.loading-spinner {
  display: inline-block;
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.results-section {
  border-top: 1px solid var(--border-color, #313244);
  padding-top: 16px;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.results-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
}

.results-meta {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
}

.mock-badge {
  display: inline-block;
  background: rgba(249, 226, 175, 0.15);
  color: #f9e2af;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 10px;
  margin-left: 4px;
}

.prediction-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px;
  margin-bottom: 12px;
}

.prediction-card {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 8px;
  padding: 12px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.property-name {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
}

.property-unit {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  opacity: 0.7;
}

.card-value {
  font-size: 22px;
  font-weight: 700;
  color: var(--accent-color, #89b4fa);
  margin-bottom: 8px;
}

.card-confidence {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.confidence-bar {
  position: relative;
  height: 4px;
  background: var(--bg-secondary, #1e1e2e);
  border-radius: 2px;
  overflow: hidden;
}

.confidence-fill {
  position: absolute;
  top: 0;
  height: 100%;
  background: rgba(137, 180, 250, 0.3);
  border-radius: 2px;
}

.confidence-marker {
  position: absolute;
  top: -2px;
  width: 2px;
  height: 8px;
  background: var(--accent-color, #89b4fa);
  border-radius: 1px;
  transform: translateX(-50%);
}

.confidence-label {
  font-size: 10px;
  color: var(--text-secondary, #a6adc8);
  text-align: center;
}

.verify-btn {
  width: 100%;
  padding: 8px;
  background: rgba(166, 227, 161, 0.15);
  border: 1px solid rgba(166, 227, 161, 0.3);
  border-radius: 8px;
  color: #a6e3a1;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}
.verify-btn:hover {
  background: rgba(166, 227, 161, 0.25);
}

.error-msg {
  margin-top: 8px;
  padding: 8px 12px;
  background: rgba(243, 139, 168, 0.1);
  border: 1px solid rgba(243, 139, 168, 0.3);
  border-radius: 6px;
  color: #f38ba8;
  font-size: 13px;
}
</style>
