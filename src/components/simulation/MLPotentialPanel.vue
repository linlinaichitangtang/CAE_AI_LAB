<template>
  <div class="ml-potential-panel">
    <!-- 势函数列表 -->
    <div class="section">
      <div class="section-header">
        <h3 class="section-title">🧬 ML 势函数库</h3>
        <button class="btn-sm" @click="loadPotentials">刷新</button>
      </div>
      <div class="potential-list">
        <div
          v-for="p in potentials"
          :key="p.name"
          class="potential-card"
          :class="{ selected: selectedPotential?.name === p.name }"
          @click="selectPotential(p)"
        >
          <div class="card-header">
            <span class="potential-name">{{ p.name }}</span>
            <span class="potential-type">{{ p.potentialType }}</span>
          </div>
          <p class="potential-desc">{{ p.description }}</p>
          <div class="card-stats">
            <span v-if="p.energyRmse" class="stat">E: {{ p.energyRmse }} meV/atom</span>
            <span v-if="p.forceRmse" class="stat">F: {{ p.forceRmse }} meV/Å</span>
            <span class="stat">cutoff: {{ p.cutoff }}Å</span>
            <span class="stat">{{ p.supportedElements.length }} 元素</span>
          </div>
          <div class="card-footer">
            <span :class="p.isReady ? 'status-ready' : 'status-not-ready'">
              {{ p.isReady ? '● 就绪' : '○ 未加载' }}
            </span>
            <span v-if="p.trainingDataSize" class="data-size">
              {{ formatDataSize(p.trainingDataSize) }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 势函数自动选择 -->
    <div class="section">
      <h3 class="section-title">🔍 智能推荐</h3>
      <div class="recommend-form">
        <div class="form-row">
          <label>元素</label>
          <input v-model="selectionReq.elementsStr" placeholder="如 Fe, Cr, Ni" class="form-input" />
        </div>
        <div class="form-row">
          <label>原子数</label>
          <input v-model.number="selectionReq.numAtoms" type="number" class="form-input" />
        </div>
        <div class="form-row">
          <label>精度</label>
          <select v-model="selectionReq.accuracy" class="form-select">
            <option value="high">高精度</option>
            <option value="medium">中等</option>
            <option value="low">快速筛选</option>
          </select>
        </div>
        <button class="btn-primary" @click="runAutoSelect" :disabled="isSelecting">
          {{ isSelecting ? '分析中...' : '推荐势函数' }}
        </button>
      </div>

      <div v-if="selectionResult" class="recommend-result">
        <div class="result-header">
          <span class="result-type">推荐: {{ selectionResult.recommendedType }}</span>
          <span :class="selectionResult.needsTraining ? 'needs-training' : 'ready'">
            {{ selectionResult.needsTraining ? '需要训练' : '可直接使用' }}
          </span>
        </div>
        <p class="result-reason">{{ selectionResult.reason }}</p>
        <div v-if="selectionResult.estimatedTrainingMinutes" class="result-meta">
          预估训练: ~{{ selectionResult.estimatedTrainingMinutes }} 分钟
        </div>
        <div v-if="selectionResult.estimatedVramGb" class="result-meta">
          显存需求: ~{{ selectionResult.estimatedVramGb }} GB
        </div>
        <div v-if="selectionResult.alternatives.length > 0" class="alternatives">
          <span class="alt-label">备选:</span>
          <span v-for="alt in selectionResult.alternatives" :key="alt.potentialType" class="alt-item">
            {{ alt.potentialType }} ({{ alt.reason }})
          </span>
        </div>
      </div>
    </div>

    <!-- 训练面板 -->
    <div class="section">
      <h3 class="section-title">⚡ 势函数训练</h3>
      <div class="train-form">
        <div class="form-row">
          <label>势函数类型</label>
          <select v-model="trainConfig.potentialType" class="form-select">
            <option value="nequip">NequIP</option>
            <option value="mace">MACE</option>
            <option value="nep">NEP</option>
            <option value="mtp">MTP</option>
            <option value="ace">ACE</option>
          </select>
        </div>
        <div class="form-row">
          <label>训练数据路径</label>
          <input v-model="trainConfig.trainingDataPath" placeholder="data/train.extxyz" class="form-input" />
        </div>
        <div class="form-grid">
          <div class="form-row">
            <label>Epochs</label>
            <input v-model.number="trainConfig.epochs" type="number" class="form-input" />
          </div>
          <div class="form-row">
            <label>Batch Size</label>
            <input v-model.number="trainConfig.batchSize" type="number" class="form-input" />
          </div>
          <div class="form-row">
            <label>学习率</label>
            <input v-model.number="trainConfig.learningRate" type="number" step="0.0001" class="form-input" />
          </div>
          <div class="form-row">
            <label>Cutoff (Å)</label>
            <input v-model.number="trainConfig.cutoff" type="number" step="0.5" class="form-input" />
          </div>
        </div>
        <div class="form-actions">
          <button class="btn-sm" @click="getHyperparams">自动推荐超参</button>
          <button class="btn-primary" @click="startTraining" :disabled="isTraining">
            {{ isTraining ? '训练中...' : '开始训练' }}
          </button>
        </div>
      </div>

      <!-- 训练结果 -->
      <div v-if="trainResult" class="train-result">
        <div class="result-header">
          <span>{{ trainResult.potentialName }}</span>
          <span :class="trainResult.success ? 'status-ready' : 'status-not-ready'">
            {{ trainResult.success ? '✓ 成功' : '✗ 失败' }}
          </span>
        </div>
        <div class="result-stats">
          <div class="stat-item">
            <span class="stat-val">{{ trainResult.finalEnergyRmse.toFixed(1) }}</span>
            <span class="stat-label">E RMSE (meV/atom)</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ trainResult.finalForceRmse.toFixed(3) }}</span>
            <span class="stat-label">F RMSE (meV/Å)</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ (trainResult.trainingTimeSec / 60).toFixed(1) }} min</span>
            <span class="stat-label">训练时间</span>
          </div>
        </div>
        <div class="loss-chart">
          <div class="chart-label">训练损失曲线</div>
          <div class="chart-area">
            <div
              v-for="point in sampledLoss"
              :key="point.epoch"
              class="chart-point"
              :style="{ left: point.x + '%', bottom: point.y + '%' }"
              :title="`Epoch ${point.epoch}: E=${point.e.toFixed(1)}, F=${point.f.toFixed(3)}`"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <!-- 验证面板 -->
    <div class="section" v-if="selectedPotential">
      <h3 class="section-title">✅ 势函数验证</h3>
      <div class="validate-form">
        <div class="form-row">
          <label>测试数据路径</label>
          <input v-model="validateReq.testDataPath" placeholder="data/test.extxyz" class="form-input" />
        </div>
        <button class="btn-primary" @click="runValidation" :disabled="isValidating">
          {{ isValidating ? '验证中...' : '运行验证' }}
        </button>
      </div>

      <div v-if="validationResult" class="validate-result">
        <div class="result-header">
          <span>{{ validationResult.potentialName }}</span>
          <span :class="validationResult.passed ? 'status-ready' : 'status-not-ready'">
            {{ validationResult.passed ? '✓ 通过' : '✗ 未通过' }}
          </span>
        </div>
        <div class="result-stats">
          <div class="stat-item">
            <span class="stat-val">{{ validationResult.energyRmse.toFixed(1) }}</span>
            <span class="stat-label">E RMSE</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ validationResult.forceRmse.toFixed(3) }}</span>
            <span class="stat-label">F RMSE</span>
          </div>
          <div class="stat-item">
            <span class="stat-val">{{ validationResult.numTestStructures }}</span>
            <span class="stat-label">测试结构</span>
          </div>
        </div>
      </div>
    </div>

    <!-- GPU 状态 -->
    <div class="section">
      <h3 class="section-title">🖥️ GPU 资源</h3>
      <div v-if="gpuStatus" class="gpu-info">
        <div v-for="gpu in gpuStatus.gpus" :key="gpu.name" class="gpu-card" :class="{ available: gpu.isAvailable }">
          <span class="gpu-name">{{ gpu.name }}</span>
          <span class="gpu-mem">{{ gpu.availableMemoryGb.toFixed(1) }} / {{ gpu.totalMemoryGb.toFixed(1) }} GB</span>
          <span :class="gpu.isAvailable ? 'status-ready' : 'status-not-ready'">
            {{ gpu.isAvailable ? '可用' : '不可用 (Mock)' }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import {
  listMlPotentials,
  autoSelectPotential,
  getGpuStatus,
  recommendTrainingHyperparams,
  submitTrainingJob,
  validateMlPotential,
  type MLPotentialInfo,
  type PotentialSelectionResult,
  type GpuResourceStatus,
  type TrainingResult,
  type ValidationResult,
  type HyperparamRecommendation,
} from '../../api/mlPotential'

// 势函数列表
const potentials = ref<MLPotentialInfo[]>([])
const selectedPotential = ref<MLPotentialInfo | null>(null)

// 智能推荐
const isSelecting = ref(false)
const selectionResult = ref<PotentialSelectionResult | null>(null)
const selectionReq = reactive({
  elementsStr: 'Fe, Cr, Ni',
  numAtoms: 10000,
  accuracy: 'medium' as 'high' | 'medium' | 'low',
})

// 训练
const isTraining = ref(false)
const trainResult = ref<TrainingResult | null>(null)
const trainConfig = reactive({
  potentialType: 'nequip',
  trainingDataPath: 'data/train.extxyz',
  epochs: 500,
  batchSize: 8,
  learningRate: 0.001,
  cutoff: 5.0,
  maxNeuron: 128,
  energyWeight: 1.0,
  forceWeight: 10.0,
  useGpu: false,
})

// 验证
const isValidating = ref(false)
const validationResult = ref<ValidationResult | null>(null)
const validateReq = reactive({
  testDataPath: 'data/test.extxyz',
})

// GPU
const gpuStatus = ref<GpuResourceStatus | null>(null)

// 采样损失曲线 (最多 50 点)
const sampledLoss = computed(() => {
  if (!trainResult.value) return []
  const history = trainResult.value.lossHistory
  if (history.length <= 50) {
    const maxE = Math.max(...history.map(h => h.energyRmse))
    return history.map(h => ({
      epoch: h.epoch,
      x: (h.epoch / (history.length - 1 || 1)) * 100,
      y: (h.energyRmse / (maxE || 1)) * 100,
      e: h.energyRmse,
      f: h.forceRmse,
    }))
  }
  const step = Math.floor(history.length / 50)
  const sampled = history.filter((_, i) => i % step === 0)
  const maxE = Math.max(...sampled.map(h => h.energyRmse))
  return sampled.map(h => ({
    epoch: h.epoch,
    x: (h.epoch / (history.length - 1 || 1)) * 100,
    y: (h.energyRmse / (maxE || 1)) * 100,
    e: h.energyRmse,
    f: h.forceRmse,
  }))
})

function formatDataSize(n: number): string {
  if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + 'M'
  if (n >= 1_000) return (n / 1_000).toFixed(0) + 'K'
  return String(n)
}

function selectPotential(p: MLPotentialInfo) {
  selectedPotential.value = p
}

async function loadPotentials() {
  try { potentials.value = await listMlPotentials() } catch (e) { console.error(e) }
}

async function runAutoSelect() {
  isSelecting.value = true
  selectionResult.value = null
  try {
    const elements = selectionReq.elementsStr.split(',').map(s => s.trim()).filter(Boolean)
    selectionResult.value = await autoSelectPotential({
      elements,
      numAtoms: selectionReq.numAtoms,
      accuracyRequirement: selectionReq.accuracy,
      hasGpu: gpuStatus.value?.gpus.some(g => g.isAvailable) ?? false,
    })
  } catch (e) { console.error(e) } finally { isSelecting.value = false }
}

async function getHyperparams() {
  try {
    const rec: HyperparamRecommendation = await recommendTrainingHyperparams({
      numAtoms: selectionReq.numAtoms,
      numSpecies: selectionReq.elementsStr.split(',').filter(Boolean).length,
      numTrainingStructures: 1000,
      potentialType: trainConfig.potentialType,
    })
    trainConfig.cutoff = rec.cutoff
    trainConfig.maxNeuron = rec.maxNeuron
    trainConfig.epochs = rec.epochs
    trainConfig.batchSize = rec.batchSize
    trainConfig.learningRate = rec.learningRate
    trainConfig.energyWeight = rec.energyWeight
    trainConfig.forceWeight = rec.forceWeight
  } catch (e) { console.error(e) }
}

async function startTraining() {
  isTraining.value = true
  trainResult.value = null
  try {
    trainResult.value = await submitTrainingJob(trainConfig)
  } catch (e) { console.error(e) } finally { isTraining.value = false }
}

async function runValidation() {
  if (!selectedPotential.value) return
  isValidating.value = true
  validationResult.value = null
  try {
    validationResult.value = await validateMlPotential({
      potentialName: selectedPotential.value.name,
      testDataPath: validateReq.testDataPath,
    })
  } catch (e) { console.error(e) } finally { isValidating.value = false }
}

onMounted(async () => {
  await Promise.all([loadPotentials(), getGpuStatus().then(s => gpuStatus.value = s)])
})
</script>

<style scoped>
.ml-potential-panel {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 16px;
}

.section {
  margin-bottom: 20px;
}
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.section-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
  margin: 0 0 12px 0;
}

.potential-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 10px;
}

.potential-card {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 8px;
  padding: 12px;
  cursor: pointer;
  transition: border-color 0.2s;
}
.potential-card:hover, .potential-card.selected {
  border-color: var(--accent-color, #89b4fa);
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}
.potential-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
}
.potential-type {
  font-size: 11px;
  background: rgba(137, 180, 250, 0.15);
  color: #89b4fa;
  padding: 1px 6px;
  border-radius: 4px;
}
.potential-desc {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  margin: 0 0 8px 0;
}
.card-stats {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 6px;
}
.stat {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
}
.card-footer {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
}
.status-ready { color: #a6e3a1; }
.status-not-ready { color: #f38ba8; }
.data-size { color: var(--text-secondary, #a6adc8); }

.recommend-form, .train-form, .validate-form {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 12px;
}
.form-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}
.form-row label {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  min-width: 100px;
}
.form-input, .form-select {
  flex: 1;
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 4px;
  color: var(--text-primary, #cdd6f4);
  padding: 4px 8px;
  font-size: 13px;
}
.form-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0 12px;
}
.form-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 8px;
}

.btn-sm {
  padding: 4px 10px;
  background: transparent;
  border: 1px solid var(--border-color, #313244);
  border-radius: 4px;
  color: var(--text-secondary, #a6adc8);
  font-size: 12px;
  cursor: pointer;
}
.btn-primary {
  padding: 6px 16px;
  background: linear-gradient(135deg, #89b4fa, #74c7ec);
  border: none;
  border-radius: 6px;
  color: #11111b;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
}
.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.recommend-result, .train-result, .validate-result {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 8px;
  padding: 12px;
}
.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
}
.result-type {
  color: var(--accent-color, #89b4fa);
}
.needs-training { color: #f9e2af; font-size: 12px; font-weight: 400; }
.result-reason {
  font-size: 13px;
  color: var(--text-secondary, #a6adc8);
  margin: 0 0 8px 0;
}
.result-meta {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  margin-bottom: 4px;
}
.alternatives {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-color, #313244);
}
.alt-label {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  margin-right: 8px;
}
.alt-item {
  font-size: 11px;
  color: var(--text-primary, #cdd6f4);
  margin-right: 12px;
}

.result-stats {
  display: flex;
  gap: 16px;
  margin: 12px 0;
}
.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.stat-val {
  font-size: 18px;
  font-weight: 700;
  color: var(--accent-color, #89b4fa);
}
.stat-label {
  font-size: 10px;
  color: var(--text-secondary, #a6adc8);
}

.loss-chart {
  margin-top: 12px;
}
.chart-label {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
  margin-bottom: 4px;
}
.chart-area {
  position: relative;
  height: 80px;
  background: var(--bg-secondary, #1e1e2e);
  border-radius: 4px;
  border: 1px solid var(--border-color, #313244);
}
.chart-point {
  position: absolute;
  width: 3px;
  height: 3px;
  background: var(--accent-color, #89b4fa);
  border-radius: 50%;
  transform: translate(-50%, 50%);
}

.gpu-info {
  display: flex;
  gap: 8px;
}
.gpu-card {
  flex: 1;
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 8px;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.gpu-card.available {
  border-color: rgba(166, 227, 161, 0.3);
}
.gpu-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #cdd6f4);
}
.gpu-mem {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
}
</style>
