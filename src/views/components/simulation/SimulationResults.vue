<script setup lang="ts">
/**
 * SimulationResults.vue — 结果浮窗模块（屈曲/频率响应/Report/ModuleGuide）
 * 从 SimulationView.vue 提取 (~280 行)
 */
import { ref, computed, watch, nextTick } from 'vue'

// ========== Buckling 结果状态 ==========
const showBucklingResultDialogFlag = ref(false)
const currentBucklingResult = ref<any>(null)
const selectedBucklingMode = ref(0)
const bucklingSafetyFactor = ref<{ safetyFactor: number; isSafe: boolean; description: string } | null>(null)

function showBucklingResultDialog(result: any) {
  currentBucklingResult.value = result
  selectedBucklingMode.value = 0
  calculateBucklingSafety()
  showBucklingResultDialogFlag.value = true
}

function calculateBucklingSafety() {
  if (!currentBucklingResult.value) return

  const multiplier = currentBucklingResult.value.critical_load_factor || 0
  // 假设参考载荷 1000N，安全系数 = 临界载荷因子 / 设计安全系数 3.0
  const designLoad = 1000
  const criticalLoad = multiplier * designLoad
  const safetyFactor = criticalLoad / (designLoad * 3) // 设计安全系数 3.0

  bucklingSafetyFactor.value = {
    safetyFactor: multiplier / 3,
    isSafe: multiplier > 3,
    description: safetyFactor >= 1
      ? `临界载荷因子 ${multiplier.toFixed(4)} > 3.0，安全`
      : `临界载荷因子 ${multiplier.toFixed(4)} < 3.0，需复核`
  }
}

function selectBucklingMode(idx: number) {
  selectedBucklingMode.value = idx
}

function getBucklingDisplayData() {
  if (!currentBucklingResult.value?.mode_shapes?.[selectedBucklingMode.value]) return null
  const mode = currentBucklingResult.value.mode_shapes[selectedBucklingMode.value]
  return {
    max_displacement: mode.max_displacement || 0,
    max_von_mises: mode.max_von_mises,
    participation_factor: mode.participation_factor
  }
}

async function showBucklingModeAnimation() {
  // TODO: 调用 ResultViewer 播放模态动画
  console.log('Playing buckling mode animation...')
}

// ========== Frequency Response 结果状态 ==========
const showFreqResponseResultDialogFlag = ref(false)
const freqResponseChartRef = ref<HTMLCanvasElement | null>(null)
const resonanceFrequency = ref<number | null>(null)
const maxFreqDisplacement = ref(0)
const freqStartFreq = ref(0)
const freqEndFreq = ref(100)
const isChartLinked = ref(false)

const frequencyResponseData = ref<{ frequency: number; displacement: number }[]>([])

function showFreqResponseResultDialog(result: any) {
  frequencyResponseData.value = result.frequency_response || []
  freqStartFreq.value = result.start_frequency || 0
  freqEndFreq.value = result.end_frequency || 100

  // 计算共振频率（最大位移处）
  let maxDisp = 0
  let resFreq = 0
  frequencyResponseData.value.forEach(item => {
    if (Math.abs(item.displacement) > maxDisp) {
      maxDisp = Math.abs(item.displacement)
      resFreq = item.frequency
    }
  })
  resonanceFrequency.value = resFreq
  maxFreqDisplacement.value = maxDisp

  showFreqResponseResultDialogFlag.value = true

  nextTick(() => drawFreqResponseChart())
}

function toggleChartLink() {
  isChartLinked.value = !isChartLinked.value
}

function drawFreqResponseChart() {
  if (!freqResponseChartRef.value) return
  const ctx = freqResponseChartRef.value.getContext('2d')
  if (!ctx) return

  const canvas = freqResponseChartRef.value
  const w = canvas.width = canvas.offsetWidth
  const h = canvas.height = canvas.offsetHeight

  ctx.clearRect(0, 0, w, h)

  const data = frequencyResponseData.value
  if (data.length < 2) return

  const maxDisp = Math.max(...data.map(d => Math.abs(d.displacement))) || 1

  const padding = 40
  const chartW = w - padding * 2
  const chartH = h - padding * 2

  // 坐标轴
  ctx.strokeStyle = '#666'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.lineTo(w - padding, h - padding)
  ctx.stroke()

  // 频率标签
  ctx.font = '10px sans-serif'
  ctx.fillStyle = '#666'
  ctx.fillText('0', padding - 8, h - padding + 4)
  ctx.fillText(freqEndFreq.value.toString(), w - padding - 10, h - padding + 4)
  ctx.fillText('频率 (Hz)', w / 2 - 20, h - 8)

  // 绘制频响曲线
  ctx.strokeStyle = '#6366f1'
  ctx.lineWidth = 2
  ctx.beginPath()

  data.forEach((item, i) => {
    const x = padding + (item.frequency / freqEndFreq.value) * chartW
    const y = h - padding - (Math.abs(item.displacement) / maxDisp) * chartH
    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  })
  ctx.stroke()

  // 标注共振点
  if (resonanceFrequency.value && resonanceFrequency.value > 0) {
    const x = padding + (resonanceFrequency.value / freqEndFreq.value) * chartW
    const maxY = h - padding - (maxFreqDisplacement.value / maxDisp) * chartH

    ctx.fillStyle = '#dc2626'
    ctx.beginPath()
    ctx.arc(x, maxY, 5, 0, Math.PI * 2)
    ctx.fill()

    ctx.font = '10px sans-serif'
    ctx.fillStyle = '#dc2626'
    ctx.fillText(`${resonanceFrequency.value.toFixed(1)} Hz`, x + 5, maxY - 5)
  }
}

function selectFreqResponsePoint(frequency: number, displacement: number) {
  console.log('Selected point:', frequency, displacement)
}

// ========== Report 状态 ==========
const showReportDialogFlag = ref(false)
const reportTemplate = ref('standard')
const isExportingPdf = ref(false)

function showReportDialog() {
  showReportDialogFlag.value = true
}

// ========== Module Guide 状态 ==========
const showModuleGuideFlag = ref(false)
const simulationGuideSteps = [
  { step: 1, title: '创建网格', desc: '定义网格尺寸和类型' },
  { step: 2, title: '设置材料', desc: '选择材料参数' },
  { step: 3, title: '施加载荷', desc: '定义边界条件和载荷' },
  { step: 4, title: '运行求解', desc: '执行有限元分析' },
  { step: 5, title: '查看结果', desc: '查看应力、位移分布' }
]

// ========== 暴露方法 ==========
defineExpose({
  showBucklingResultDialog,
  showFreqResponseResultDialog,
  showReportDialog,
  showModuleGuide: () => { showModuleGuideFlag.value = true },
  showBucklingResultDialogFlag,
  showFreqResponseResultDialogFlag
})
</script>

<template>
  <!-- 📊 屈曲分析结果对话框 -->
  <div v-if="showBucklingResultDialogFlag" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showBucklingResultDialogFlag = false">
    <div class="bg-white rounded-lg shadow-xl w-[700px] max-h-[85vh] flex flex-col">
      <div class="p-4 border-b flex justify-between items-center">
        <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
          <span>📊</span>
          <span>屈曲分析结果</span>
        </h3>
        <button @click="showBucklingResultDialogFlag = false" class="text-gray-500 hover:text-gray-700">✕</button>
      </div>

      <div class="p-4 flex-1 overflow-y-auto">
        <!-- 临界载荷因子 -->
        <div class="bg-purple-50 rounded-lg p-4 mb-4">
          <h4 class="text-sm font-semibold text-purple-700 mb-3 flex items-center gap-2">
            <span>⚡</span>
            <span>临界载荷因子</span>
          </h4>
          <div class="text-center">
            <div class="text-4xl font-bold text-purple-700">
              {{ currentBucklingResult?.critical_load_factor?.toFixed(4) || 'N/A' }}
            </div>
            <div class="text-sm text-gray-600 mt-1">Load Multiplier</div>
            <div class="text-xs text-gray-500 mt-2">
              临界载荷 = {{ currentBucklingResult?.critical_load_factor?.toFixed(4) || 'N/A' }} × 施加载荷
            </div>
          </div>
        </div>

        <!-- 安全系数 -->
        <div v-if="bucklingSafetyFactor" class="mb-4" :class="bucklingSafetyFactor.isSafe ? 'bg-green-50 border border-green-200' : 'bg-yellow-50 border border-yellow-200'">
          <h4 class="text-sm font-semibold mb-2 flex items-center gap-2" :class="bucklingSafetyFactor.isSafe ? 'text-green-700' : 'text-yellow-700'">
            <span>{{ bucklingSafetyFactor.isSafe ? '✅' : '⚠️' }}</span>
            <span>安全系数评估</span>
          </h4>
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-600">安全系数:</span>
              <span class="font-bold" :class="bucklingSafetyFactor.isSafe ? 'text-green-600' : 'text-yellow-600'">
                {{ bucklingSafetyFactor.safetyFactor.toFixed(4) }}
              </span>
            </div>
            <p class="text-gray-700">{{ bucklingSafetyFactor.description }}</p>
          </div>
        </div>

        <!-- 屈曲模态选择 -->
        <div class="mb-4">
          <h4 class="text-sm font-semibold text-gray-700 mb-2 flex items-center gap-2">
            <span>📐</span>
            <span>屈曲模态</span>
          </h4>
          <div class="grid grid-cols-2 gap-2">
            <div
              v-for="(mode, idx) in currentBucklingResult?.mode_shapes"
              :key="idx"
              @click="selectBucklingMode(idx)"
              class="p-3 rounded-lg border cursor-pointer transition"
              :class="selectedBucklingMode === idx ? 'border-purple-500 bg-purple-50' : 'border-gray-200 hover:border-purple-300'"
            >
              <div class="flex justify-between items-center mb-1">
                <span class="font-medium text-sm">{{ mode.description || `模态 ${idx + 1}` }}</span>
                <span v-if="idx === 0" class="text-xs bg-red-100 text-red-600 px-2 py-0.5 rounded">临界</span>
              </div>
              <div class="text-xs text-gray-500">载荷因子: {{ mode.load_multiplier?.toFixed(4) || 'N/A' }}</div>
            </div>
          </div>
        </div>

        <!-- 选中模态详细信息 -->
        <div v-if="getBucklingDisplayData()" class="bg-gray-50 rounded-lg p-4">
          <h4 class="text-sm font-semibold text-gray-700 mb-3">模态 {{ selectedBucklingMode + 1 }} 详细信息</h4>
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-600">最大位移:</span>
              <span class="font-medium">{{ getBucklingDisplayData()?.max_displacement?.toFixed(6) || 'N/A' }} mm</span>
            </div>
            <div v-if="getBucklingDisplayData()?.max_von_mises" class="flex justify-between">
              <span class="text-gray-600">最大等效应力:</span>
              <span class="font-medium">{{ getBucklingDisplayData()?.max_von_mises?.toFixed(2) || 'N/A' }} MPa</span>
            </div>
            <div v-if="getBucklingDisplayData()?.participation_factor" class="flex justify-between">
              <span class="text-gray-600">参与因子:</span>
              <span class="font-medium">{{ getBucklingDisplayData()?.participation_factor?.toFixed(4) || 'N/A' }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="p-4 border-t flex justify-end gap-3">
        <button @click="showBucklingResultDialogFlag = false" class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 text-sm">关闭</button>
        <button @click="showBucklingModeAnimation" :disabled="!currentBucklingResult?.mode_shapes?.length" class="px-4 py-2 bg-purple-600 text-white rounded hover:bg-purple-700 text-sm disabled:opacity-50 flex items-center gap-2">
          <span>▶</span>
          <span>查看模态动画</span>
        </button>
      </div>
    </div>
  </div>

  <!-- 📊 频率响应分析结果对话框 -->
  <div v-if="showFreqResponseResultDialogFlag" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showFreqResponseResultDialogFlag = false">
    <div class="bg-white rounded-lg shadow-xl w-[800px] max-h-[85vh] flex flex-col">
      <div class="p-4 border-b flex justify-between items-center">
        <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
          <span>📊</span>
          <span>频率响应分析结果</span>
        </h3>
        <button @click="showFreqResponseResultDialogFlag = false" class="text-gray-500 hover:text-gray-700">✕</button>
      </div>

      <div class="p-4 flex-1 overflow-y-auto">
        <!-- 频响曲线图 -->
        <div class="bg-indigo-50 rounded-lg p-4 mb-4">
          <div class="flex justify-between items-center mb-3">
            <h4 class="text-sm font-semibold text-indigo-700 flex items-center gap-2">
              <span>📈</span>
              <span>频响曲线</span>
            </h4>
            <button @click="toggleChartLink" class="text-xs px-2 py-1 border rounded" :class="isChartLinked ? 'bg-green-100 text-green-700 border-green-300' : 'bg-gray-100 text-gray-700 border-gray-300'">
              {{ isChartLinked ? '🔗 联动' : '📴 分离' }}
            </button>
          </div>
          <canvas ref="freqResponseChartRef" class="w-full h-48 border bg-white rounded"></canvas>
        </div>

        <!-- 关键结果摘要 -->
        <div class="grid grid-cols-3 gap-4 mb-4">
          <div class="bg-indigo-100 rounded-lg p-4 text-center">
            <div class="text-xs text-indigo-600 mb-1">最大位移</div>
            <div class="text-xl font-bold text-indigo-800">{{ maxFreqDisplacement.toFixed(4) }} mm</div>
          </div>
          <div class="bg-indigo-100 rounded-lg p-4 text-center">
            <div class="text-xs text-indigo-600 mb-1">共振频率</div>
            <div class="text-xl font-bold text-indigo-800">{{ resonanceFrequency?.toFixed(2) || '--' }} Hz</div>
          </div>
          <div class="bg-indigo-100 rounded-lg p-4 text-center">
            <div class="text-xs text-indigo-600 mb-1">频率范围</div>
            <div class="text-xl font-bold text-indigo-800">{{ freqStartFreq }}-{{ freqEndFreq }} Hz</div>
          </div>
        </div>

        <!-- 共振警告 -->
        <div v-if="resonanceFrequency" class="bg-yellow-50 border border-yellow-200 rounded-lg p-3 mb-4">
          <div class="flex items-center gap-2">
            <span class="text-yellow-600">⚠️</span>
            <span class="text-sm text-yellow-720 font-medium">检测到共振频率: {{ resonanceFrequency.toFixed(2) }} Hz</span>
          </div>
          <p class="text-xs text-yellow-700 mt-1">在该频率下结构振幅最大，设计时需特别注意避免此频率或采取减振措施。</p>
        </div>
      </div>

      <div class="p-4 border-t flex justify-end">
        <button @click="showFreqResponseResultDialogFlag = false" class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 text-sm">关闭</button>
      </div>
    </div>
  </div>
</template>