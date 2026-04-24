<template>
  <div class="multiscale-view">
    <div class="view-header">
      <h1 class="view-title">🔬 微观 → 宏观性能直通</h1>
      <p class="view-desc">输入微观结构图像或工艺参数，ML 直接预测宏观等效性能</p>
    </div>

    <div class="view-content">
      <!-- 左侧: 输入面板 -->
      <div class="input-panel">
        <!-- 路径 A: 图像输入 -->
        <div class="panel-section">
          <h3 class="section-title">路径 A: 微观结构图像</h3>
          <div class="upload-area" @click="mockImageUpload" :class="{ uploaded: imageInfo }">
            <div v-if="!imageInfo" class="upload-placeholder">
              <span class="upload-icon">📷</span>
              <span>点击上传 SEM/OM/EBSD 图像</span>
              <span class="upload-hint">支持 PNG, JPG, TIFF</span>
            </div>
            <div v-else class="upload-info">
              <span>{{ imageInfo.fileName }}</span>
              <span class="image-meta">{{ imageInfo.width }}×{{ imageInfo.height }} | {{ imageInfo.imageType }}</span>
            </div>
          </div>

          <div v-if="imageInfo" class="segment-controls">
            <div class="form-row">
              <label>分割目标</label>
              <select v-model="segTarget" class="form-select">
                <option value="phase">相分割</option>
                <option value="pore">孔隙检测</option>
                <option value="grain">晶粒分割</option>
              </select>
            </div>
            <button class="btn-primary" @click="runSegmentation" :disabled="isSegmenting">
              {{ isSegmenting ? '分割中...' : '▶ UNet 分割' }}
            </button>
          </div>

          <!-- 分割结果 -->
          <div v-if="segResult" class="seg-result">
            <div class="result-header">
              <span>分割结果</span>
              <span class="iou-badge">IoU: {{ segResult.statistics.iouScore.toFixed(2) }}</span>
            </div>
            <div class="phase-bars">
              <div v-for="(frac, phase) in segResult.statistics.phaseFractions" :key="phase" class="phase-bar">
                <span class="phase-name">{{ phase }}</span>
                <div class="bar-track">
                  <div class="bar-fill" :style="{ width: (frac * 100) + '%' }"></div>
                </div>
                <span class="phase-val">{{ (frac * 100).toFixed(1) }}%</span>
              </div>
            </div>
            <div class="seg-stats">
              <span>孔隙率: {{ (segResult.statistics.porosity * 100).toFixed(1) }}%</span>
              <span v-if="segResult.statistics.averageGrainSizeUm">
                晶粒尺寸: {{ segResult.statistics.averageGrainSizeUm.toFixed(1) }} ± {{ segResult.statistics.grainSizeStdUm?.toFixed(1) }} μm
              </span>
            </div>
          </div>
        </div>

        <!-- 路径 B: 工艺参数输入 -->
        <div class="panel-section">
          <h3 class="section-title">路径 B: 工艺参数</h3>
          <div class="param-grid">
            <div class="form-row">
              <label>材料</label>
              <input v-model="processParams.material" class="form-input" placeholder="如 Al6061" />
            </div>
            <div class="form-row">
              <label>烧结温度 (°C)</label>
              <input v-model.number="processParams.temperature" type="number" class="form-input" />
            </div>
            <div class="form-row">
              <label>压力 (MPa)</label>
              <input v-model.number="processParams.pressure" type="number" class="form-input" />
            </div>
            <div class="form-row">
              <label>保温时间 (h)</label>
              <input v-model.number="processParams.holdTime" type="number" class="form-input" />
            </div>
          </div>
        </div>

        <!-- 预测按钮 -->
        <button
          class="btn-predict"
          @click="runPrediction"
          :disabled="isPredicting || (!imageInfo && !processParams.material)"
        >
          <span v-if="isPredicting" class="spinner">⟳</span>
          <span v-else>⚡ 预测宏观性能</span>
        </button>
      </div>

      <!-- 右侧: 结果面板 -->
      <div class="result-panel">
        <div v-if="predictionResult" class="prediction-section">
          <div class="result-header">
            <h3>宏观性能预测</h3>
            <span class="model-badge">{{ predictionResult.modelName }}</span>
            <span v-if="predictionResult.isMock" class="mock-badge">Mock</span>
          </div>

          <!-- 性能卡片 -->
          <div class="property-cards">
            <div
              v-for="pred in predictionResult.predictions"
              :key="pred.propertyName"
              class="property-card"
              :class="{ 'needs-verify': pred.needsVerification }"
            >
              <div class="card-header">
                <span class="prop-name">{{ getPropertyLabel(pred.propertyName) }}</span>
                <span class="prop-unit">{{ pred.unit }}</span>
              </div>
              <div class="card-value">{{ formatValue(pred.predictedValue, pred.propertyName) }}</div>
              <div class="card-uncertainty">
                <div class="uncertainty-bar">
                  <div class="uncertainty-range" :style="uncertaintyStyle(pred)"></div>
                  <div class="uncertainty-marker"></div>
                </div>
                <span class="uncertainty-text">± {{ formatUncertainty(pred) }}</span>
              </div>
              <div v-if="pred.needsVerification" class="verify-hint">
                ⚠️ 不确定性较高，建议 CAELab 验证
              </div>
            </div>
          </div>

          <!-- 一键验证 -->
          <button
            v-if="predictionResult.predictions.some(p => p.needsVerification)"
            class="btn-verify"
            @click="runCaelabVerification"
          >
            🔬 一键 CAELab 验证
          </button>
        </div>

        <!-- 模型训练 -->
        <div class="training-section">
          <h3 class="section-title">🧠 模型训练</h3>
          <div class="train-controls">
            <div class="form-row">
              <label>模型名称</label>
              <input v-model="trainConfig.modelName" class="form-input" placeholder="my_multiscale_model" />
            </div>
            <div class="form-row">
              <label>Epochs</label>
              <input v-model.number="trainConfig.epochs" type="number" class="form-input" />
            </div>
            <div class="form-row">
              <label>学习率</label>
              <input v-model.number="trainConfig.learningRate" type="number" step="0.0001" class="form-input" />
            </div>
            <div class="form-actions">
              <button class="btn-sm" @click="exportOnnx">导出 ONNX</button>
              <button class="btn-primary" @click="startTraining" :disabled="isTraining">
                {{ isTraining ? '训练中...' : '开始训练' }}
              </button>
            </div>
          </div>

          <div v-if="trainResult" class="train-result">
            <div class="result-header">
              <span>{{ trainResult.modelName }}</span>
              <span :class="trainResult.success ? 'status-ok' : 'status-fail'">
                {{ trainResult.success ? '✓ 成功' : '✗ 失败' }}
              </span>
            </div>
            <div class="val-errors">
              <div v-for="(err, prop) in trainResult.validationErrors" :key="prop" class="val-item">
                <span>{{ prop }}</span>
                <span :class="err < 8 ? 'status-ok' : 'status-fail'">{{ err.toFixed(1) }}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import {
  importMicrostructureImage,
  segmentMicrostructure,
  predictMacroProperties,
  trainMultiscaleSurrogate,
  exportMultiscaleOnnx,
  verifyWithCaelab,
  type MicrostructureImage,
  type SegmentationResult,
  type MacroPropertyPredictionResponse,
  type MultiscaleTrainingResult,
} from '../api/multiscaleSurrogate'

const imageInfo = ref<MicrostructureImage | null>(null)
const segTarget = ref('phase')
const isSegmenting = ref(false)
const segResult = ref<SegmentationResult | null>(null)

const processParams = reactive({
  material: '',
  temperature: 550,
  pressure: 50,
  holdTime: 2,
})

const isPredicting = ref(false)
const predictionResult = ref<MacroPropertyPredictionResponse | null>(null)

const isTraining = ref(false)
const trainResult = ref<MultiscaleTrainingResult | null>(null)
const trainConfig = reactive({
  modelName: 'multiscale_v1',
  epochs: 200,
  learningRate: 0.001,
})

const propertyLabels: Record<string, string> = {
  elastic_modulus: '弹性模量',
  yield_strength: '屈服强度',
  thermal_conductivity: '热导率',
  electrical_conductivity: '电导率',
  hardness: '硬度',
  fracture_toughness: '断裂韧性',
}

function getPropertyLabel(key: string) {
  return propertyLabels[key] || key
}

function formatValue(v: number, prop: string): string {
  if (v >= 1e9) return (v / 1e9).toFixed(1) + ' GPa'
  if (v >= 1e6) return (v / 1e6).toFixed(0) + ' MPa'
  if (v >= 1e3) return (v / 1e3).toFixed(1) + ' kPa'
  return v.toFixed(2)
}

function formatUncertainty(pred: { uncertainty: number; propertyName: string }): string {
  const u = pred.uncertainty
  if (pred.propertyName === 'thermal_conductivity') return u.toFixed(2) + ' W/(m·K)'
  if (u >= 1e9) return (u / 1e9).toFixed(2) + ' GPa'
  if (u >= 1e6) return (u / 1e6).toFixed(1) + ' MPa'
  return u.toFixed(2)
}

function uncertaintyStyle(pred: { predictedValue: number; confidenceLower: number; confidenceUpper: number }) {
  const range = pred.confidenceUpper - pred.confidenceLower
  const total = pred.predictedValue * 2 || 1
  const width = Math.min((range / total) * 100, 100)
  const left = Math.max(0, ((pred.confidenceLower - (pred.predictedValue - total / 2)) / total) * 100)
  return { left: left + '%', width: width + '%' }
}

async function mockImageUpload() {
  try {
    imageInfo.value = await importMicrostructureImage({
      fileName: 'sem_sample_001.png',
      filePath: '/data/microstructure/sem_sample_001.png',
      imageType: 'sem',
      width: 1024,
      height: 768,
      magnification: 5000,
      scaleBarUm: 2.0,
      materialName: processParams.material || undefined,
    })
  } catch (e) { console.error(e) }
}

async function runSegmentation() {
  if (!imageInfo.value) return
  isSegmenting.value = true
  try {
    segResult.value = await segmentMicrostructure({
      imageId: imageInfo.value.id,
      segmentationTarget: segTarget.value,
      useGpu: false,
    })
  } catch (e) { console.error(e) } finally { isSegmenting.value = false }
}

async function runPrediction() {
  isPredicting.value = true
  predictionResult.value = null
  try {
    predictionResult.value = await predictMacroProperties({
      imageId: imageInfo.value?.id,
      processingParams: processParams.material ? { ...processParams } : undefined,
      targetProperties: ['elastic_modulus', 'yield_strength', 'thermal_conductivity', 'hardness', 'fracture_toughness'],
      withUncertainty: true,
    })
  } catch (e) { console.error(e) } finally { isPredicting.value = false }
}

async function startTraining() {
  isTraining.value = true
  trainResult.value = null
  try {
    trainResult.value = await trainMultiscaleSurrogate({
      modelName: trainConfig.modelName,
      trainingDataSource: 'caelab_simulation',
      targetProperties: ['elastic_modulus', 'yield_strength', 'thermal_conductivity'],
      epochs: trainConfig.epochs,
      batchSize: 16,
      learningRate: trainConfig.learningRate,
      hiddenDim: 256,
      numLayers: 4,
      useUncertainty: true,
      useGpu: false,
    })
  } catch (e) { console.error(e) } finally { isTraining.value = false }
}

async function exportOnnx() {
  try {
    const result = await exportMultiscaleOnnx(trainConfig.modelName)
    alert(`ONNX 导出成功\n文件: ${result.onnxFilePath}\n大小: ${(result.fileSizeBytes / 1e6).toFixed(1)} MB`)
  } catch (e) { console.error(e) }
}

async function runCaelabVerification() {
  if (!imageInfo.value || !predictionResult.value) return
  const props: Record<string, number> = {}
  for (const p of predictionResult.value.predictions) {
    props[p.propertyName] = p.predictedValue
  }
  try {
    const result = await verifyWithCaelab(imageInfo.value.id, props)
    alert(`CAELab 验证已提交 (Mock)\n${JSON.stringify(result, null, 2)}`)
  } catch (e) { console.error(e) }
}
</script>

<style scoped>
.multiscale-view {
  padding: 20px;
  max-width: 1400px;
  margin: 0 auto;
}
.view-header { margin-bottom: 20px; }
.view-title { font-size: 24px; font-weight: 700; color: var(--text-primary, #cdd6f4); margin: 0 0 4px; }
.view-desc { font-size: 14px; color: var(--text-secondary, #a6adc8); margin: 0; }

.view-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.panel-section {
  background: var(--bg-secondary, #1e1e2e);
  border: 1px solid var(--border-color, #313244);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 12px;
}
.section-title {
  font-size: 14px; font-weight: 600; color: var(--text-primary, #cdd6f4);
  margin: 0 0 12px;
}

.upload-area {
  border: 2px dashed var(--border-color, #313244);
  border-radius: 10px;
  padding: 24px;
  text-align: center;
  cursor: pointer;
  transition: border-color 0.2s;
  margin-bottom: 12px;
}
.upload-area:hover { border-color: var(--accent-color, #89b4fa); }
.upload-area.uploaded { border-color: #a6e3a1; border-style: solid; }
.upload-placeholder { display: flex; flex-direction: column; gap: 4px; color: var(--text-secondary, #a6adc8); }
.upload-icon { font-size: 24px; }
.upload-hint { font-size: 11px; opacity: 0.6; }
.upload-info { display: flex; flex-direction: column; gap: 2px; color: var(--text-primary, #cdd6f4); font-size: 13px; }
.image-meta { font-size: 11px; color: var(--text-secondary, #a6adc8); }

.form-row { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
.form-row label { font-size: 12px; color: var(--text-secondary, #a6adc8); min-width: 100px; }
.form-input, .form-select {
  flex: 1; background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 4px; color: var(--text-primary, #cdd6f4); padding: 4px 8px; font-size: 13px;
}
.param-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0 12px; }

.btn-primary {
  width: 100%; padding: 8px; background: linear-gradient(135deg, #89b4fa, #74c7ec);
  border: none; border-radius: 6px; color: #11111b; font-size: 13px; font-weight: 600; cursor: pointer;
  margin-top: 8px;
}
.btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }
.btn-sm {
  padding: 4px 10px; background: transparent; border: 1px solid var(--border-color, #313244);
  border-radius: 4px; color: var(--text-secondary, #a6adc8); font-size: 12px; cursor: pointer;
}
.btn-predict {
  width: 100%; padding: 12px; background: linear-gradient(135deg, #a6e3a1, #94e2d5);
  border: none; border-radius: 8px; color: #11111b; font-size: 15px; font-weight: 700; cursor: pointer;
}
.btn-predict:disabled { opacity: 0.5; cursor: not-allowed; }
.spinner { display: inline-block; animation: spin 1s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

.seg-result {
  background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 8px; padding: 12px; margin-top: 12px;
}
.result-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; font-size: 13px; color: var(--text-primary, #cdd6f4); }
.iou-badge { background: rgba(166, 227, 161, 0.15); color: #a6e3a1; padding: 2px 8px; border-radius: 4px; font-size: 11px; }

.phase-bars { display: flex; flex-direction: column; gap: 6px; margin-bottom: 8px; }
.phase-bar { display: flex; align-items: center; gap: 8px; font-size: 12px; }
.phase-name { color: var(--text-secondary, #a6adc8); min-width: 70px; }
.bar-track { flex: 1; height: 6px; background: var(--bg-secondary, #1e1e2e); border-radius: 3px; }
.bar-fill { height: 100%; background: var(--accent-color, #89b4fa); border-radius: 3px; transition: width 0.3s; }
.phase-val { color: var(--text-primary, #cdd6f4); min-width: 40px; text-align: right; }
.seg-stats { display: flex; gap: 16px; font-size: 11px; color: var(--text-secondary, #a6adc8); }

.result-panel { display: flex; flex-direction: column; gap: 12px; }

.prediction-section {
  background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #313244);
  border-radius: 12px; padding: 16px;
}
.model-badge { background: rgba(137, 180, 250, 0.15); color: #89b4fa; padding: 2px 8px; border-radius: 4px; font-size: 11px; }
.mock-badge { background: rgba(249, 226, 175, 0.15); color: #f9e2af; padding: 2px 6px; border-radius: 4px; font-size: 10px; }

.property-cards { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; margin: 12px 0; }
.property-card {
  background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 8px; padding: 12px; transition: border-color 0.2s;
}
.property-card.needs-verify { border-color: rgba(249, 226, 175, 0.4); }
.card-header { display: flex; justify-content: space-between; margin-bottom: 4px; }
.prop-name { font-size: 12px; color: var(--text-secondary, #a6adc8); }
.prop-unit { font-size: 11px; color: var(--text-secondary, #a6adc8); opacity: 0.7; }
.card-value { font-size: 20px; font-weight: 700; color: var(--accent-color, #89b4fa); margin-bottom: 8px; }

.card-uncertainty { display: flex; flex-direction: column; gap: 4px; }
.uncertainty-bar { position: relative; height: 4px; background: var(--bg-secondary, #1e1e2e); border-radius: 2px; }
.uncertainty-range { position: absolute; top: 0; height: 100%; background: rgba(137, 180, 250, 0.3); border-radius: 2px; }
.uncertainty-marker { position: absolute; top: -2px; left: 50%; width: 2px; height: 8px; background: var(--accent-color, #89b4fa); border-radius: 1px; transform: translateX(-50%); }
.uncertainty-text { font-size: 10px; color: var(--text-secondary, #a6adc8); text-align: center; }
.verify-hint { font-size: 11px; color: #f9e2af; margin-top: 6px; }

.btn-verify {
  width: 100%; padding: 8px; background: rgba(166, 227, 161, 0.15);
  border: 1px solid rgba(166, 227, 161, 0.3); border-radius: 6px;
  color: #a6e3a1; font-size: 13px; cursor: pointer;
}

.training-section {
  background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #313244);
  border-radius: 12px; padding: 16px;
}
.train-controls { margin-bottom: 12px; }
.form-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 8px; }

.train-result {
  background: var(--bg-primary, #11111b); border: 1px solid var(--border-color, #313244);
  border-radius: 8px; padding: 12px;
}
.val-errors { display: flex; flex-direction: column; gap: 4px; margin-top: 8px; }
.val-item { display: flex; justify-content: space-between; font-size: 12px; padding: 2px 0; }
.status-ok { color: #a6e3a1; }
.status-fail { color: #f38ba8; }

@media (max-width: 900px) {
  .view-content { grid-template-columns: 1fr; }
  .property-cards { grid-template-columns: 1fr; }
}
</style>
