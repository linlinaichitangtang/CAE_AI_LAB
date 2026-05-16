<script setup lang="ts">
/**
 * BenchmarkPanel.vue — V3.1-001 Benchmark 发布
 * 标准算例对比实验数据，建立工程可信度
 */
import { ref, computed } from 'vue'
import {
  useBenchmark,
  BENCHMARK_CATEGORY_INFO,
  type BenchmarkCase,
  type BenchmarkResult
} from '@/composables/useBenchmark'

// ============ 状态 ============
const {
  cases,
  results,
  getCase,
  runBenchmark,
  getStats,
  clearResults
} = useBenchmark()

// UI 状态
const selectedCategory = ref<string>('all')
const selectedCase = ref<BenchmarkCase | null>(null)
const showResultDialog = ref(false)
const lastResult = ref<BenchmarkResult | null>(null)

// 用户输入表单
const userInput = ref({
  maxDisplacement: '',
  maxStress: '',
  firstFrequency: '',
  meshNodes: '',
  meshElements: ''
})

// ============ 计算属性 ============
const filteredCases = computed(() => {
  if (selectedCategory.value === 'all') {
    return cases.value
  }
  return cases.value.filter(c => c.category === selectedCategory.value)
})

const stats = computed(() => getStats())

const categories = computed(() => [
  { id: 'all', name: '全部', icon: '🌐' },
  ...Object.entries(BENCHMARK_CATEGORY_INFO).map(([id, info]) => ({
    id,
    ...info
  }))
])

// ============ 工具函数 ============
function selectCase(benchmarkCase: BenchmarkCase) {
  selectedCase.value = benchmarkCase
  resetForm()
}

function resetForm() {
  userInput.value = {
    maxDisplacement: '',
    maxStress: '',
    firstFrequency: '',
    meshNodes: '',
    meshElements: ''
  }
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

function getScoreColor(score: number): string {
  if (score >= 90) return 'text-green-600'
  if (score >= 70) return 'text-yellow-600'
  return 'text-red-600'
}

// Computed: filtered results for selected case
const selectedCaseResults = computed(() => {
  if (!selectedCase.value) return []
  return results.value.filter((r: BenchmarkResult) => r.caseId === selectedCase.value!.id)
})

// ============ 操作函数 ============
function handleRunBenchmark() {
  if (!selectedCase.value) return

  const userResult = {
    maxDisplacement: userInput.value.maxDisplacement ? parseFloat(userInput.value.maxDisplacement) : undefined,
    maxStress: userInput.value.maxStress ? parseFloat(userInput.value.maxStress) : undefined,
    firstFrequency: userInput.value.firstFrequency ? parseFloat(userInput.value.firstFrequency) : undefined,
    meshNodes: userInput.value.meshNodes ? parseInt(userInput.value.meshNodes) : undefined,
    meshElements: userInput.value.meshElements ? parseInt(userInput.value.meshElements) : undefined
  }

  const result = runBenchmark(selectedCase.value.id, userResult)
  if (result) {
    lastResult.value = result
    showResultDialog.value = true
  }
}

function getCategoryInfo(category: string) {
  return BENCHMARK_CATEGORY_INFO[category as keyof typeof BENCHMARK_CATEGORY_INFO] || { name: category, icon: '📋' }
}
</script>

<template>
  <div class="benchmark-panel h-full flex flex-col bg-[var(--bg-ground)]">
    <!-- 顶部栏 -->
    <div class="bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-xl font-bold text-[var(--text-primary)] flex items-center gap-2">
            <span>🎯</span>
            <span>Benchmark 验证</span>
          </h1>
          <p class="text-sm text-[var(--text-muted)] mt-1">
            标准算例对比实验数据，建立工程可信度
          </p>
        </div>
        <div class="flex items-center gap-3">
          <div class="flex items-center gap-4 text-sm">
            <span>📊 <span class="font-medium">{{ stats.totalRuns }}</span> 次运行</span>
            <span>📈 平均分 <span class="font-medium">{{ stats.averageScore }}</span></span>
            <span>✅ 通过率 <span class="font-medium text-green-600">{{ stats.passRate }}%</span></span>
          </div>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="grid grid-cols-4 gap-4 mt-4">
        <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
          <p class="text-2xl font-bold text-[var(--primary)]">{{ stats.totalCases }}</p>
          <p class="text-xs text-[var(--text-muted)]">总算例</p>
        </div>
        <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
          <p class="text-2xl font-bold text-green-600">{{ stats.verifiedCases }}</p>
          <p class="text-xs text-[var(--text-muted)]">已验证</p>
        </div>
        <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
          <p class="text-2xl font-bold text-blue-600">{{ stats.totalRuns }}</p>
          <p class="text-xs text-[var(--text-muted)]">总运行</p>
        </div>
        <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
          <p class="text-2xl font-bold" :class="getScoreColor(stats.averageScore)">{{ stats.averageScore }}</p>
          <p class="text-xs text-[var(--text-muted)]">平均分</p>
        </div>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="flex-1 overflow-y-auto p-6">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- 左侧：算例列表 -->
        <div class="lg:col-span-1 space-y-4">
          <!-- 分类筛选 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">📂 算例分类</h2>
            </div>
            <div class="p-3 grid grid-cols-2 gap-2">
              <button
                v-for="cat in categories"
                :key="cat.id"
                @click="selectedCategory = cat.id"
                class="p-3 rounded-lg text-left transition-all"
                :class="selectedCategory === cat.id
                  ? 'bg-[var(--primary)] text-white'
                  : 'bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)]'"
              >
                <span class="text-lg">{{ cat.icon }}</span>
                <span class="text-sm font-medium">{{ cat.name }}</span>
              </button>
            </div>
          </div>

          <!-- 算例列表 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">📋 标准算例</h2>
            </div>
            <div class="divide-y divide-[var(--border-subtle)]">
              <div
                v-for="benchmarkCase in filteredCases"
                :key="benchmarkCase.id"
                @click="selectCase(benchmarkCase)"
                class="p-4 hover:bg-[var(--bg-hover)] cursor-pointer transition-colors"
                :class="selectedCase?.id === benchmarkCase.id ? 'bg-[var(--primary)]/5' : ''"
              >
                <div class="flex items-start justify-between">
                  <div class="flex items-start gap-3">
                    <div class="w-10 h-10 rounded-lg bg-[var(--primary)]/10 flex items-center justify-center text-lg">
                      {{ getCategoryInfo(benchmarkCase.category).icon }}
                    </div>
                    <div>
                      <p class="font-medium text-[var(--text-primary)]">{{ benchmarkCase.name }}</p>
                      <p class="text-xs text-[var(--text-muted)] mt-1 line-clamp-2">{{ benchmarkCase.description }}</p>
                      <div class="flex items-center gap-2 mt-2">
                        <span
                          class="px-1.5 py-0.5 rounded text-xs"
                          :class="benchmarkCase.verified
                            ? 'bg-green-100 text-green-700'
                            : 'bg-yellow-100 text-yellow-700'"
                        >
                          {{ benchmarkCase.verified ? '✅ 已验证' : '⏳ 待验证' }}
                        </span>
                        <span class="text-xs text-[var(--text-muted)]">
                          {{ getCategoryInfo(benchmarkCase.category).name }}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div v-if="filteredCases.length === 0" class="p-8 text-center text-[var(--text-muted)]">
                暂无算例
              </div>
            </div>
          </div>
        </div>

        <!-- 右侧：算例详情和运行 -->
        <div class="lg:col-span-2">
          <div v-if="!selectedCase" class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] p-8 text-center">
            <div class="text-6xl mb-4 opacity-30">🎯</div>
            <p class="text-lg text-[var(--text-muted)]">选择左侧算例开始验证</p>
            <p class="text-sm text-[var(--text-muted)] mt-2">点击算例卡片查看详情并输入您的计算结果</p>
          </div>

          <div v-else class="space-y-4">
            <!-- 算例详情 -->
            <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
              <div class="px-6 py-4 border-b border-[var(--border-subtle)]">
                <div class="flex items-start justify-between">
                  <div>
                    <h2 class="text-lg font-bold text-[var(--text-primary)]">{{ selectedCase.name }}</h2>
                    <p class="text-sm text-[var(--text-muted)]">{{ selectedCase.description }}</p>
                  </div>
                  <span
                    class="px-2 py-1 rounded-full text-xs"
                    :class="selectedCase.verified
                      ? 'bg-green-100 text-green-700'
                      : 'bg-yellow-100 text-yellow-700'"
                  >
                    {{ selectedCase.verified ? '✅ 已验证' : '⏳ 待验证' }}
                  </span>
                </div>
              </div>

              <div class="p-6 grid grid-cols-2 gap-6">
                <!-- 参考数据 -->
                <div>
                  <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-3">📊 参考数据 (文献值)</h3>
                  <div class="space-y-3">
                    <div v-if="selectedCase.referenceData.maxDisplacement" class="bg-[var(--bg-elevated)] rounded-lg p-3">
                      <p class="text-xs text-[var(--text-muted)]">最大位移</p>
                      <p class="text-lg font-bold text-[var(--primary)]">
                        {{ selectedCase.referenceData.maxDisplacement }}
                        <span class="text-sm font-normal text-[var(--text-muted)]">
                          {{ selectedCase.referenceData.maxDisplacementUnit }}
                        </span>
                      </p>
                    </div>
                    <div v-if="selectedCase.referenceData.maxStress" class="bg-[var(--bg-elevated)] rounded-lg p-3">
                      <p class="text-xs text-[var(--text-muted)]">最大应力</p>
                      <p class="text-lg font-bold text-[var(--primary)]">
                        {{ selectedCase.referenceData.maxStress }}
                        <span class="text-sm font-normal text-[var(--text-muted)]">
                          {{ selectedCase.referenceData.maxStressUnit }}
                        </span>
                      </p>
                    </div>
                    <div v-if="selectedCase.referenceData.firstFrequency" class="bg-[var(--bg-elevated)] rounded-lg p-3">
                      <p class="text-xs text-[var(--text-muted)]">第一阶固有频率</p>
                      <p class="text-lg font-bold text-[var(--primary)]">
                        {{ selectedCase.referenceData.firstFrequency }}
                        <span class="text-sm font-normal text-[var(--text-muted)]">
                          {{ selectedCase.referenceData.frequencyUnit }}
                        </span>
                      </p>
                    </div>
                  </div>
                  <div class="mt-4 p-3 bg-[var(--bg-elevated)] rounded-lg">
                    <p class="text-xs text-[var(--text-muted)]">数据来源</p>
                    <p class="text-sm text-[var(--text-primary)]">{{ selectedCase.referenceData.source }}</p>
                  </div>
                </div>

                <!-- 容许误差 -->
                <div>
                  <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-3">📐 容许误差</h3>
                  <div class="space-y-3">
                    <div class="flex items-center justify-between bg-[var(--bg-elevated)] rounded-lg p-3">
                      <span class="text-sm text-[var(--text-secondary)]">位移误差</span>
                      <span class="text-sm font-medium text-[var(--primary)]">±{{ selectedCase.tolerance.displacement }}%</span>
                    </div>
                    <div class="flex items-center justify-between bg-[var(--bg-elevated)] rounded-lg p-3">
                      <span class="text-sm text-[var(--text-secondary)]">应力误差</span>
                      <span class="text-sm font-medium text-[var(--primary)]">±{{ selectedCase.tolerance.stress }}%</span>
                    </div>
                    <div class="flex items-center justify-between bg-[var(--bg-elevated)] rounded-lg p-3">
                      <span class="text-sm text-[var(--text-secondary)]">频率误差</span>
                      <span class="text-sm font-medium text-[var(--primary)]">±{{ selectedCase.tolerance.frequency }}%</span>
                    </div>
                    <div class="flex items-center justify-between bg-green-50 rounded-lg p-3">
                      <span class="text-sm text-green-700">最高容许误差</span>
                      <span class="text-sm font-bold text-green-700">≤{{ selectedCase.tolerance.maxAllowedError }}%</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 网格尺寸 -->
              <div class="px-6 py-3 bg-[var(--bg-elevated)] border-t border-[var(--border-subtle)]">
                <span class="text-xs text-[var(--text-muted)]">
                  支持网格尺寸: {{ selectedCase.meshSizes.join(' / ') }}
                </span>
              </div>
            </div>

            <!-- 用户输入 -->
            <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
              <div class="px-6 py-4 border-b border-[var(--border-subtle)]">
                <h3 class="font-semibold text-[var(--text-primary)]">📝 输入您的计算结果</h3>
              </div>
              <div class="p-6 space-y-4">
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
                      最大位移 (m)
                    </label>
                    <input
                      v-model="userInput.maxDisplacement"
                      type="number"
                      step="0.001"
                      class="input w-full"
                      placeholder="例如: 0.0285"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
                      最大应力 (MPa)
                    </label>
                    <input
                      v-model="userInput.maxStress"
                      type="number"
                      step="0.1"
                      class="input w-full"
                      placeholder="例如: 72.5"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
                      第一阶固有频率 (Hz)
                    </label>
                    <input
                      v-model="userInput.firstFrequency"
                      type="number"
                      step="0.1"
                      class="input w-full"
                      placeholder="例如: 18.2"
                    />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
                      网格节点数
                    </label>
                    <input
                      v-model="userInput.meshNodes"
                      type="number"
                      class="input w-full"
                      placeholder="例如: 5000"
                    />
                  </div>
                </div>

                <div class="flex justify-between items-center pt-4">
                  <button
                    @click="resetForm"
                    class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)]"
                  >
                    🔄 重置
                  </button>
                  <button
                    @click="handleRunBenchmark"
                    class="px-6 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90"
                  >
                    🎯 运行验证
                  </button>
                </div>
              </div>
            </div>

            <!-- 历史结果 -->
            <template v-if="selectedCase">
              <div v-if="selectedCaseResults.length > 0" class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
                <div class="px-6 py-4 border-b border-[var(--border-subtle)] flex items-center justify-between">
                  <h3 class="font-semibold text-[var(--text-primary)]">📜 历史验证结果</h3>
                  <span class="text-xs text-[var(--text-muted)]">
                    共 {{ selectedCaseResults.length }} 次
                  </span>
                </div>
                <div class="divide-y divide-[var(--border-subtle)]">
                  <div
                    v-for="result in selectedCaseResults.slice(0, 5)"
                  :key="result.executedAt"
                  class="px-6 py-3 flex items-center justify-between"
                >
                  <div>
                    <span class="text-sm text-[var(--text-secondary)]">
                      {{ formatDate(result.executedAt) }}
                    </span>
                    <span v-if="result.userResult.meshElements" class="text-xs text-[var(--text-muted)] ml-2">
                      {{ result.userResult.meshElements }} 单元
                    </span>
                  </div>
                  <div class="flex items-center gap-3">
                    <span
                      class="px-2 py-0.5 rounded text-xs font-medium"
                      :class="result.passed ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'"
                    >
                      {{ result.passed ? '✅ 通过' : '❌ 未通过' }}
                    </span>
                    <span class="text-sm font-medium" :class="getScoreColor(result.comparison.overallScore)">
                      {{ result.comparison.overallScore.toFixed(1) }} 分
                    </span>
                  </div>
                </div>
              </div>
            </div>
            </template>
          </div>
        </div>
      </div>
    </div>

    <!-- 结果弹窗 -->
    <div v-if="showResultDialog && lastResult" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showResultDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl w-full max-w-lg p-6 shadow-2xl">
        <div class="text-center mb-6">
          <div
            class="w-20 h-20 mx-auto rounded-full flex items-center justify-center text-4xl"
            :class="lastResult.passed ? 'bg-green-100' : 'bg-red-100'"
          >
            {{ lastResult.passed ? '🎉' : '😞' }}
          </div>
          <h2 class="text-xl font-bold text-[var(--text-primary)] mt-4">
            {{ lastResult.passed ? '验证通过！' : '验证未通过' }}
          </h2>
          <p class="text-4xl font-bold mt-2" :class="getScoreColor(lastResult.comparison.overallScore)">
            {{ lastResult.comparison.overallScore.toFixed(1) }}
            <span class="text-lg">/ 100</span>
          </p>
        </div>

        <div class="space-y-3 mb-6">
          <div
            v-for="detail in lastResult.comparison.details"
            :key="detail"
            class="p-3 bg-[var(--bg-elevated)] rounded-lg text-sm"
          >
            {{ detail }}
          </div>
        </div>

        <div class="flex items-center justify-between text-xs text-[var(--text-muted)] mb-4">
          <span>执行时间: {{ lastResult.executionTime }}ms</span>
          <span>{{ formatDate(lastResult.executedAt) }}</span>
        </div>

        <div class="flex justify-end gap-3">
          <button
            @click="showResultDialog = false"
            class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)]"
          >
            关闭
          </button>
          <button
            v-if="lastResult.passed"
            class="px-4 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90"
          >
            📤 导出报告
          </button>
        </div>
      </div>
    </div>
  </div>
</template>