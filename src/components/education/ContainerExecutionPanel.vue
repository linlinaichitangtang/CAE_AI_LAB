<script setup lang="ts">
/**
 * ContainerExecutionPanel.vue — V3.0-004 容器化执行环境 UI
 * Docker 化仿真环境，学生环境一致，结果可复现
 */
import { ref, computed, onMounted } from 'vue'
import {
  useContainerizedExecution,
  PREDEFINED_ENVIRONMENTS,
  getEnvironmentIcon,
  type ExecutionEnvironment,
  type ExecutionResult
} from '@/composables/useContainerizedExecution'

// ============ 状态 ============
const {
  availableImages,
  runningContainers,
  currentEnvironment,
  isDockerAvailable,
  isLoading,
  lastError,
  isExecuting,
  executionHistory,
  checkDockerAvailability,
  listImages,
  pullImage,
  startContainer,
  stopContainer,
  executeCode,
  getEnvironments,
  cleanupStoppedContainers,
  clearExecutionHistory
} = useContainerizedExecution()

// 代码编辑
const codeInput = ref('')
const selectedLanguage = ref<'python' | 'julia' | 'octave' | 'fortran'>('python')
const executionTimeout = ref(60)

// 执行结果
const lastResult = ref<ExecutionResult | null>(null)

// UI 状态
const showEnvDetails = ref(false)
const selectedEnvForDetails = ref<ExecutionEnvironment | null>(null)
const isStarting = ref(false)

// ============ 计算属性 ============
const environments = computed(() => getEnvironments())
const canExecute = computed(() => runningContainers.value.some(c => c.status === 'running') && codeInput.value.trim())
const currentContainer = computed(() => runningContainers.value.find(c => c.status === 'running'))

// ============ 工具函数 ============
function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
  return `${Math.floor(ms / 60000)}m ${((ms % 60000) / 1000).toFixed(0)}s`
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// ============ 操作函数 ============
async function handleStartEnvironment(env: ExecutionEnvironment) {
  isStarting.value = true
  try {
    const container = await startContainer(env)
    if (container) {
      lastError.value = null
    }
  } finally {
    isStarting.value = false
  }
}

async function handleStopEnvironment(containerId: string) {
  await stopContainer(containerId)
}

async function handleExecute() {
  if (!canExecute.value) return

  lastResult.value = null
  const result = await executeCode({
    code: codeInput.value,
    language: selectedLanguage.value,
    timeout: executionTimeout.value
  })
  lastResult.value = result
}

function handleSelectEnv(env: ExecutionEnvironment) {
  selectedEnvForDetails.value = env
  showEnvDetails.value = true
}

async function handleCleanup() {
  const cleaned = await cleanupStoppedContainers()
  alert(`已清理 ${cleaned} 个停止的容器`)
}

function handleClearHistory() {
  if (confirm('确定清空执行历史？')) {
    clearExecutionHistory()
  }
}

// 代码模板
const codeTemplates = [
  {
    name: '基础计算',
    language: 'python' as const,
    code: `import numpy as np

# 基础有限元计算示例
nodes = np.array([
    [0, 0],
    [1, 0],
    [1, 1],
    [0, 1]
])

elements = np.array([
    [0, 1, 2, 3]
])

print(f"节点数: {len(nodes)}")
print(f"单元数: {len(elements)}")
print("环境验证成功 ✅")`
  },
  {
    name: '矩阵运算',
    language: 'python' as const,
    code: `import numpy as np
from scipy.linalg import inv

# 刚度矩阵组装
K = np.array([
    [4, -2, 0],
    [-2, 4, -2],
    [0, -2, 4]
], dtype=float)

print("刚度矩阵 K:")
print(K)

# 求逆验证
K_inv = inv(K)
print("\\n逆矩阵验证 (K @ K^-1 = I):")
print(np.round(K @ K_inv, 6))`
  },
  {
    name: '后处理',
    language: 'python' as const,
    code: `import numpy as np

# 仿真结果后处理
max_displacement = 0.0234  # m
max_stress = 156.7  # MPa

print("=" * 40)
print("仿真结果摘要")
print("=" * 40)
print(f"最大位移: {max_displacement:.6f} m")
print(f"最大应力: {max_stress:.2f} MPa")
print(f"安全系数: {250 / max_stress:.2f}")
print("=" * 40)

# 应力分布可视化数据
stress_values = np.linspace(0, max_stress, 50)
print(f"\\n应力采样点: {len(stress_values)}")
print(f"应力范围: [{stress_values.min():.2f}, {stress_values.max():.2f}] MPa")`
  },
  {
    name: '参数扫描',
    language: 'python' as const,
    code: `import numpy as np

# 参数化扫描分析
E_values = np.linspace(70e9, 210e9, 10)  # 弹性模量范围
results = []

for i, E in enumerate(E_values):
    stress = 100e6  # 施加应力
    displacement = stress / E * 0.1  # 简化计算
    results.append({
        'index': i + 1,
        'E_GPa': E / 1e9,
        'displacement_mm': displacement * 1000
    })

print("参数扫描结果:")
print("-" * 40)
for r in results:
    print(f"E={r['E_GPa']:.1f} GPa → δ={r['displacement_mm']:.4f} mm")
print("-" * 40)
print(f"完成 {len(results)} 组计算")`
  }
]

function loadTemplate(template: typeof codeTemplates[0]) {
  selectedLanguage.value = template.language
  codeInput.value = template.code
}

// ============ 生命周期 ============
onMounted(async () => {
  await checkDockerAvailability()
})
</script>

<template>
  <div class="container-execution h-full flex flex-col bg-[var(--bg-ground)]">
    <!-- 顶部栏 -->
    <div class="bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-xl font-bold text-[var(--text-primary)] flex items-center gap-2">
            <span>🐳</span>
            <span>容器化执行环境</span>
          </h1>
          <p class="text-sm text-[var(--text-muted)] mt-1">
            Docker 化仿真环境，所有学生环境一致，结果可复现
          </p>
        </div>
        <div class="flex items-center gap-3">
          <!-- Docker 状态 -->
          <span
            class="px-3 py-1.5 rounded-full text-sm font-medium"
            :class="isDockerAvailable
              ? 'bg-green-100 text-green-700'
              : 'bg-red-100 text-red-700'"
          >
            {{ isDockerAvailable ? '🐳 Docker 就绪' : '❌ Docker 未检测到' }}
          </span>

          <!-- 环境状态 -->
          <span
            v-if="currentContainer"
            class="px-3 py-1.5 rounded-full text-sm font-medium bg-blue-100 text-blue-700"
          >
            🟢 {{ currentContainer.imageName }}
          </span>

          <button
            @click="handleCleanup"
            class="px-3 py-1.5 border border-[var(--border-subtle)] rounded text-sm hover:bg-[var(--bg-hover)]"
          >
            🧹 清理
          </button>
        </div>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="flex-1 overflow-y-auto">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4 p-4 h-full">
        <!-- 左侧：环境选择 -->
        <div class="lg:col-span-1 space-y-4">
          <!-- 环境选择 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">📦 执行环境</h2>
            </div>

            <div class="divide-y divide-[var(--border-subtle)]">
              <div
                v-for="env in environments"
                :key="env.id"
                class="p-4 hover:bg-[var(--bg-hover)] transition-colors"
              >
                <div class="flex items-start justify-between">
                  <div class="flex items-start gap-3">
                    <div class="w-10 h-10 rounded-lg bg-[var(--primary)]/10 flex items-center justify-center text-xl">
                      {{ getEnvironmentIcon(env.id) }}
                    </div>
                    <div>
                      <p class="font-medium text-[var(--text-primary)]">{{ env.name }}</p>
                      <p class="text-xs text-[var(--text-muted)] mt-0.5">{{ env.description }}</p>
                      <div class="flex items-center gap-1 mt-1">
                        <span
                          v-for="pkg in env.packages.slice(0, 3)"
                          :key="pkg"
                          class="px-1.5 py-0.5 bg-[var(--bg-elevated)] rounded text-xs text-[var(--text-muted)]"
                        >
                          {{ pkg.split('>=')[0] }}
                        </span>
                        <span
                          v-if="env.packages.length > 3"
                          class="text-xs text-[var(--text-muted)]"
                        >
                          +{{ env.packages.length - 3 }}
                        </span>
                      </div>
                    </div>
                  </div>

                  <!-- 启动/停止按钮 -->
                  <div class="flex flex-col gap-1">
                    <button
                      v-if="!runningContainers.some(c => c.imageId === env.id && c.status === 'running')"
                      @click="handleStartEnvironment(env)"
                      :disabled="!isDockerAvailable || isStarting"
                      class="px-3 py-1 bg-green-600 text-white rounded text-xs hover:bg-green-700 disabled:opacity-50"
                    >
                      {{ isStarting ? '启动中...' : '▶️ 启动' }}
                    </button>
                    <button
                      v-else
                      @click="handleStopEnvironment(runningContainers.find(c => c.imageId === env.id && c.status === 'running')?.id || '')"
                      class="px-3 py-1 bg-red-600 text-white rounded text-xs hover:bg-red-700"
                    >
                      ⏹️ 停止
                    </button>
                    <button
                      @click="handleSelectEnv(env)"
                      class="px-3 py-1 border border-[var(--border-subtle)] rounded text-xs hover:bg-[var(--bg-hover)]"
                    >
                      📋 详情
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 快捷模板 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">💡 快捷模板</h2>
            </div>
            <div class="p-3 grid grid-cols-2 gap-2">
              <button
                v-for="template in codeTemplates"
                :key="template.name"
                @click="loadTemplate(template)"
                class="p-2 bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] rounded-lg text-xs text-left"
              >
                <span class="font-medium text-[var(--text-primary)]">{{ template.name }}</span>
              </button>
            </div>
          </div>
        </div>

        <!-- 中间：代码执行 -->
        <div class="lg:col-span-2 space-y-4">
          <!-- 语言选择和执行按钮 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] p-4">
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center gap-3">
                <select
                  v-model="selectedLanguage"
                  class="input w-32"
                >
                  <option value="python">🐍 Python</option>
                  <option value="julia">🔬 Julia</option>
                  <option value="octave">📊 Octave</option>
                  <option value="fortran">⚡ Fortran</option>
                </select>
                <select
                  v-model="executionTimeout"
                  class="input w-28"
                >
                  <option :value="30">30 秒</option>
                  <option :value="60">60 秒</option>
                  <option :value="120">2 分钟</option>
                  <option :value="300">5 分钟</option>
                </select>
              </div>
              <button
                @click="handleExecute"
                :disabled="!canExecute || isExecuting"
                class="px-6 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 disabled:opacity-50 flex items-center gap-2"
              >
                <span v-if="isExecuting" class="animate-spin">⏳</span>
                <span>{{ isExecuting ? '执行中...' : '▶️ 执行代码' }}</span>
              </button>
            </div>

            <!-- 代码输入 -->
            <textarea
              v-model="codeInput"
              class="w-full h-64 p-4 bg-[var(--bg-base)] font-mono text-sm text-[var(--text-primary)] rounded-lg border border-[var(--border-subtle)] focus:border-[var(--primary)] focus:outline-none resize-none"
              placeholder="在此输入代码..."
              :disabled="isExecuting"
            ></textarea>
          </div>

          <!-- 执行结果 -->
          <div v-if="lastResult" class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span
                  class="px-2 py-0.5 rounded-full text-xs font-medium"
                  :class="lastResult.success ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'"
                >
                  {{ lastResult.success ? '✅ 成功' : '❌ 失败' }}
                </span>
                <span class="text-sm text-[var(--text-muted)]">
                  耗时: {{ formatDuration(lastResult.duration) }}
                </span>
              </div>
              <span class="text-xs text-[var(--text-muted)]">
                退出码: {{ lastResult.exitCode }}
              </span>
            </div>

            <!-- stdout -->
            <div v-if="lastResult.stdout" class="p-4">
              <p class="text-xs font-medium text-[var(--text-secondary)] mb-2">📤 输出:</p>
              <pre class="bg-[var(--bg-base)] p-3 rounded-lg text-sm font-mono text-[var(--text-primary)] whitespace-pre-wrap overflow-x-auto max-h-48">{{ lastResult.stdout }}</pre>
            </div>

            <!-- stderr -->
            <div v-if="lastResult.stderr" class="p-4 border-t border-[var(--border-subtle)]">
              <p class="text-xs font-medium text-red-600 mb-2">📕 错误:</p>
              <pre class="bg-red-50 p-3 rounded-lg text-sm font-mono text-red-600 whitespace-pre-wrap overflow-x-auto max-h-48">{{ lastResult.stderr }}</pre>
            </div>
          </div>

          <!-- 执行历史 -->
          <div v-if="executionHistory.length > 0" class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
              <h2 class="font-semibold text-[var(--text-primary)]">📜 执行历史</h2>
              <button
                @click="handleClearHistory"
                class="text-xs text-[var(--text-muted)] hover:text-red-600"
              >
                清空历史
              </button>
            </div>
            <div class="max-h-48 overflow-y-auto">
              <div
                v-for="(item, idx) in executionHistory.slice(0, 10)"
                :key="item.id"
                class="p-3 border-b border-[var(--border-subtle)] last:border-b-0 hover:bg-[var(--bg-hover)]"
              >
                <div class="flex items-center justify-between mb-1">
                  <div class="flex items-center gap-2">
                    <span class="px-1.5 py-0.5 bg-[var(--bg-elevated)] rounded text-xs">
                      {{ item.language }}
                    </span>
                    <span class="text-xs text-[var(--text-muted)]">{{ formatDate(item.timestamp) }}</span>
                  </div>
                  <span
                    class="text-xs"
                    :class="item.result.success ? 'text-green-600' : 'text-red-600'"
                  >
                    {{ item.result.success ? '✅' : '❌' }} {{ formatDuration(item.result.duration) }}
                  </span>
                </div>
                <p class="text-xs text-[var(--text-muted)] font-mono truncate">
                  {{ item.code.slice(0, 100) }}{{ item.code.length > 100 ? '...' : '' }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 环境详情弹窗 -->
    <div v-if="showEnvDetails && selectedEnvForDetails" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEnvDetails = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-full max-w-lg p-6 shadow-2xl">
        <div class="flex items-start justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-xl bg-[var(--primary)]/10 flex items-center justify-center text-2xl">
              {{ getEnvironmentIcon(selectedEnvForDetails.id) }}
            </div>
            <div>
              <h2 class="text-lg font-bold text-[var(--text-primary)]">{{ selectedEnvForDetails.name }}</h2>
              <p class="text-sm text-[var(--text-muted)] font-mono">{{ selectedEnvForDetails.image }}</p>
            </div>
          </div>
          <button @click="showEnvDetails = false" class="text-2xl text-[var(--text-muted)] hover:text-[var(--text-primary)]">&times;</button>
        </div>

        <div class="mb-4">
          <p class="text-sm text-[var(--text-secondary)]">{{ selectedEnvForDetails.description }}</p>
        </div>

        <div class="mb-4">
          <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-2">包含的包:</h3>
          <div class="flex flex-wrap gap-2">
            <span
              v-for="pkg in selectedEnvForDetails.packages"
              :key="pkg"
              class="px-2 py-1 bg-[var(--bg-elevated)] rounded text-xs text-[var(--text-primary)]"
            >
              {{ pkg }}
            </span>
          </div>
        </div>

        <div v-if="Object.keys(selectedEnvForDetails.envVars).length > 0" class="mb-4">
          <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-2">环境变量:</h3>
          <div class="bg-[var(--bg-elevated)] rounded-lg p-3 font-mono text-xs">
            <div v-for="(value, key) in selectedEnvForDetails.envVars" :key="key" class="flex justify-between">
              <span class="text-[var(--text-muted)]">{{ key }}</span>
              <span class="text-[var(--text-primary)]">{{ value }}</span>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-3 pt-4 border-t border-[var(--border-subtle)]">
          <button
            @click="showEnvDetails = false"
            class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)]"
          >
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- 错误提示 -->
    <div v-if="lastError" class="fixed bottom-4 right-4 p-4 bg-red-50 border border-red-200 rounded-lg shadow-lg max-w-md">
      <p class="text-red-700 font-medium">⚠️ 错误</p>
      <p class="text-sm text-red-600 mt-1">{{ lastError }}</p>
    </div>
  </div>
</template>