<template>
  <div class="tc4-failure-analysis">
    <!-- 头部 -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h2 class="text-2xl font-semibold gradient-text">TC4 失效分析</h2>
        <p class="text-sm text-gray-500 mt-1">Ti-6Al-4V 多模态失效模式预测</p>
      </div>
      <div class="flex gap-2">
        <button @click="showGenerateDialog = true" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition flex items-center gap-2">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          生成合成数据
        </button>
      </div>
    </div>

    <!-- 状态指示器 -->
    <div v-if="store.error" class="mb-4 p-4 bg-red-50 border border-red-200 rounded-lg">
      <p class="text-red-600">{{ store.error }}</p>
    </div>

    <!-- 三路输入面板 -->
    <div class="grid grid-cols-3 gap-4 mb-6">
      <!-- EBSD 晶格输入 -->
      <div class="card">
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-medium flex items-center gap-2">
            <span class="w-8 h-8 rounded-full bg-purple-100 flex items-center justify-center">
              <svg class="w-4 h-4 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z" />
              </svg>
            </span>
            EBSD 晶格
          </h3>
          <span v-if="store.ebsd" class="status-badge status-success">已加载</span>
        </div>

        <div v-if="!store.ebsd" class="border-2 border-dashed border-gray-200 rounded-lg p-6 text-center">
          <input ref="ebsdInput" type="file" accept=".csv,.ctf,.ang" class="hidden" @change="handleEbsdFile" />
          <button @click="ebsdInput?.click()" class="text-gray-500 hover:text-gray-700">
            <svg class="w-8 h-8 mx-auto mb-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            <p class="text-sm">上传 EBSD 文件</p>
            <p class="text-xs text-gray-400 mt-1">.csv, .ctf, .ang</p>
          </button>
        </div>
        <div v-else class="space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">样本ID</span>
            <span class="font-mono">{{ store.ebsd.sample_id }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">晶格参数 a</span>
            <span>{{ store.ebsd.a?.toFixed(3) || '-' }} Å</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">晶格参数 c</span>
            <span>{{ store.ebsd.c?.toFixed(3) || '-' }} Å</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">c/a</span>
            <span>{{ store.ebsd.c_to_a?.toFixed(4) || '-' }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">特征维度</span>
            <span>{{ store.ebsd.features.length }} 维</span>
          </div>
          <button @click="store.ebsd = null" class="w-full mt-2 text-sm text-red-500 hover:text-red-600">
            清除数据
          </button>
        </div>
      </div>

      <!-- SEM 图像输入 -->
      <div class="card">
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-medium flex items-center gap-2">
            <span class="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
              <svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
            </span>
            SEM 图像
          </h3>
          <span v-if="store.sem" class="status-badge status-success">已加载</span>
        </div>

        <div v-if="!store.sem" class="border-2 border-dashed border-gray-200 rounded-lg p-6 text-center">
          <input ref="semInput" type="file" accept=".csv,.jpg,.png,.tif" class="hidden" @change="handleSemFile" />
          <button @click="semInput?.click()" class="text-gray-500 hover:text-gray-700">
            <svg class="w-8 h-8 mx-auto mb-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            <p class="text-sm">上传 SEM 图像</p>
            <p class="text-xs text-gray-400 mt-1">.csv, .jpg, .png, .tif</p>
          </button>
        </div>
        <div v-else class="space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">样本ID</span>
            <span class="font-mono">{{ store.sem.sample_id }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">质量评分</span>
            <span class="text-green-600">{{ (store.sem.quality_score * 100).toFixed(1) }}%</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">特征维度</span>
            <span>{{ store.sem.features.length }} 维</span>
          </div>
          <button @click="store.sem = null" class="w-full mt-2 text-sm text-red-500 hover:text-red-600">
            清除数据
          </button>
        </div>
      </div>

      <!-- Load 载荷输入 -->
      <div class="card">
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-medium flex items-center gap-2">
            <span class="w-8 h-8 rounded-full bg-orange-100 flex items-center justify-center">
              <svg class="w-4 h-4 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </span>
            Load 载荷
          </h3>
          <span v-if="store.load" class="status-badge status-success">已加载</span>
        </div>

        <div v-if="!store.load" class="border-2 border-dashed border-gray-200 rounded-lg p-6 text-center">
          <input ref="loadInput" type="file" accept=".csv" class="hidden" @change="handleLoadFile" />
          <button @click="loadInput?.click()" class="text-gray-500 hover:text-gray-700">
            <svg class="w-8 h-8 mx-auto mb-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            <p class="text-sm">上传载荷数据</p>
            <p class="text-xs text-gray-400 mt-1">.csv</p>
          </button>
        </div>
        <div v-else class="space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">样本ID</span>
            <span class="font-mono">{{ store.load.sample_id }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">载荷类型</span>
            <span>{{ store.load.load_type }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">特征维度</span>
            <span>{{ store.load.features.length }} 维</span>
          </div>
          <button @click="store.load = null" class="w-full mt-2 text-sm text-red-500 hover:text-red-600">
            清除数据
          </button>
        </div>
      </div>
    </div>

    <!-- 预测按钮 -->
    <div class="flex justify-center mb-6">
      <button
        @click="runPrediction"
        :disabled="!store.hasAllInputs || store.isLoading"
        class="px-8 py-3 bg-gradient-primary text-white rounded-xl font-medium shadow-lg hover:shadow-xl transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
      >
        <svg v-if="store.isLoading" class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
        </svg>
        <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
        </svg>
        {{ store.isLoading ? '预测中...' : '多模态预测' }}
      </button>
    </div>

    <!-- 预测结果 -->
    <div v-if="store.prediction" class="card mb-6">
      <h3 class="font-medium mb-4 flex items-center gap-2">
        <span class="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center">
          <svg class="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </span>
        预测结果
      </h3>

      <div class="grid grid-cols-4 gap-4">
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <div class="text-sm text-gray-500 mb-1">失效模式</div>
          <div class="text-2xl font-bold text-purple-600">{{ store.prediction.failure_mode }}</div>
          <div class="text-xs text-gray-500 mt-1">{{ store.failureModeLabel }}</div>
        </div>
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <div class="text-sm text-gray-500 mb-1">分类置信度</div>
          <div class="text-2xl font-bold text-blue-600">{{ (store.prediction.class_confidence * 100).toFixed(1) }}%</div>
        </div>
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <div class="text-sm text-gray-500 mb-1">预测寿命 (log)</div>
          <div class="text-2xl font-bold text-orange-600">{{ store.prediction.log_cycles.toFixed(2) }}</div>
          <div class="text-xs text-gray-500 mt-1">log₁₀(cycles)</div>
        </div>
        <div class="text-center p-4 bg-gray-50 rounded-lg">
          <div class="text-sm text-gray-500 mb-1">预测寿命</div>
          <div class="text-2xl font-bold text-green-600">{{ formatCycles(store.prediction.cycles) }}</div>
          <div class="text-xs text-gray-500 mt-1">cycles</div>
        </div>
      </div>
    </div>

    <!-- MD 验证面板 -->
    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h3 class="font-medium flex items-center gap-2">
          <span class="w-8 h-8 rounded-full bg-indigo-100 flex items-center justify-center">
            <svg class="w-4 h-4 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
            </svg>
          </span>
          LAMMPS MD 验证
        </h3>
        <button @click="showMdDialog = true" class="text-sm text-blue-500 hover:text-blue-600">
          配置参数
        </button>
      </div>

      <div v-if="store.mdResult" class="bg-gray-900 rounded-lg p-4 font-mono text-sm text-green-400 overflow-auto max-h-60">
        <pre>{{ store.mdResult }}</pre>
      </div>
      <div v-else class="text-center py-8 text-gray-400">
        <p>点击"配置参数"运行分子动力学验证</p>
      </div>
    </div>

    <!-- 生成合成数据对话框 -->
    <div v-if="showGenerateDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showGenerateDialog = false">
      <div class="bg-white rounded-xl p-6 w-96 shadow-2xl">
        <h3 class="text-lg font-medium mb-4">生成 TC4 合成数据</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">样本数量</label>
            <input v-model="generateConfig.num_samples" type="number" min="10" max="10000" class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">输出目录</label>
            <input v-model="generateConfig.output_dir" type="text" class="w-full px-3 py-2 border rounded-lg" />
          </div>
          <div class="flex items-center gap-2">
            <input v-model="generateConfig.use_tc4" type="checkbox" id="use_tc4" class="w-4 h-4" />
            <label for="use_tc4" class="text-sm text-gray-700">使用 TC4 HCP 参数</label>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">随机种子</label>
            <input v-model.number="generateConfig.seed" type="number" class="w-full px-3 py-2 border rounded-lg" />
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-6">
          <button @click="showGenerateDialog = false" class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg">取消</button>
          <button @click="runGenerateSynthetic" :disabled="store.isGenerating" class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50">
            {{ store.isGenerating ? '生成中...' : '生成' }}
          </button>
        </div>
      </div>
    </div>

    <!-- MD 验证对话框 -->
    <div v-if="showMdDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showMdDialog = false">
      <div class="bg-white rounded-xl p-6 w-96 shadow-2xl">
        <h3 class="text-lg font-medium mb-4">LAMMPS MD 验证配置</h3>
        <div class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">晶格参数 a (Å)</label>
              <input v-model.number="mdConfig.lattice_a" type="number" step="0.001" class="w-full px-3 py-2 border rounded-lg" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">晶格参数 c (Å)</label>
              <input v-model.number="mdConfig.lattice_c" type="number" step="0.001" class="w-full px-3 py-2 border rounded-lg" />
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">裂纹面</label>
            <select v-model="mdConfig.crack_plane" class="w-full px-3 py-2 border rounded-lg">
              <option value="(0001)"> basal (0001)</option>
              <option value="(10-10)"> prism {10-10}</option>
              <option value="(11-20)"> prism {11-20}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">裂纹方向</label>
            <select v-model="mdConfig.crack_direction" class="w-full px-3 py-2 border rounded-lg">
              <option value="[11-20]">[11-20]</option>
              <option value="[1-100]">[1-100]</option>
              <option value="[10-10]">[10-10]</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">验证类型</label>
            <select v-model="mdConfig.validation_type" class="w-full px-3 py-2 border rounded-lg">
              <option value="crack_propagation">裂纹扩展</option>
              <option value="dislocation_evolution">位错演化</option>
            </select>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-6">
          <button @click="showMdDialog = false" class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg">取消</button>
          <button @click="runMdValidation" :disabled="store.isLoading" class="px-4 py-2 bg-indigo-500 text-white rounded-lg hover:bg-indigo-600 disabled:opacity-50">
            {{ store.isLoading ? '验证中...' : '开始验证' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useTc4FailureStore } from '@/api/tc4Failure'

const store = useTc4FailureStore()

const ebsdInput = ref<HTMLInputElement | null>(null)
const semInput = ref<HTMLInputElement | null>(null)
const loadInput = ref<HTMLInputElement | null>(null)

const showGenerateDialog = ref(false)
const showMdDialog = ref(false)

const generateConfig = reactive({
  num_samples: 100,
  output_dir: './tc4_synthetic',
  use_tc4: true,
  seed: 42,
})

const mdConfig = reactive({
  lattice_a: 2.95,
  lattice_c: 4.68,
  crack_plane: '(0001)',
  crack_direction: '[11-20]',
  validation_type: 'crack_propagation',
})

function handleEbsdFile(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    store.loadEbsd(input.files[0].name, 'TC4')
  }
}

function handleSemFile(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    store.loadSem(input.files[0].name)
  }
}

function handleLoadFile(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    store.loadLoad(input.files[0].name)
  }
}

async function runPrediction() {
  try {
    await store.predict()
  } catch (e) {
    console.error('Prediction failed:', e)
  }
}

async function runGenerateSynthetic() {
  try {
    await store.generateSynthetic(generateConfig)
    showGenerateDialog.value = false
  } catch (e) {
    console.error('Generation failed:', e)
  }
}

async function runMdValidation() {
  try {
    await store.runMdValidation(mdConfig)
    showMdDialog.value = false
  } catch (e) {
    console.error('MD validation failed:', e)
  }
}

function formatCycles(cycles: number): string {
  if (cycles < 1000) return cycles.toFixed(0)
  if (cycles < 1e6) return (cycles / 1000).toFixed(1) + 'K'
  if (cycles < 1e9) return (cycles / 1e6).toFixed(1) + 'M'
  return (cycles / 1e9).toFixed(1) + 'B'
}
</script>

<style scoped>
.tc4-failure-analysis {
  padding: 1.5rem;
}

.card {
  @apply bg-white rounded-xl border border-gray-200 shadow-sm p-4;
}

.status-badge {
  @apply px-2 py-1 text-xs rounded-full font-medium;
}

.status-success {
  @apply bg-green-100 text-green-700;
}

.gradient-text {
  @apply bg-gradient-to-r from-purple-600 to-blue-500 bg-clip-text text-transparent;
}
</style>