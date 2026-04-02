<template>
  <div class="smart-preset-panel rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] overflow-hidden">
    <!-- Header -->
    <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="text-base">&#x1F9E0;</span>
        <h3 class="text-sm font-semibold text-[var(--text-primary)]">智能预设置</h3>
        <span
          v-if="preset"
          :class="['text-xs px-2 py-0.5 rounded-full font-medium', confidenceInfo.bgColor, confidenceInfo.color]"
        >
          置信度: {{ confidenceInfo.label }}
        </span>
      </div>
      <button
        v-if="!preset"
        @click="generateRecommendation"
        class="text-xs px-3 py-1 rounded bg-[var(--primary)] text-white hover:opacity-90 transition"
      >
        分析模型
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="p-6 flex flex-col items-center gap-2">
      <div class="w-6 h-6 border-2 border-[var(--primary)] border-t-transparent rounded-full animate-spin"></div>
      <span class="text-xs text-[var(--text-secondary)]">正在分析模型特征...</span>
    </div>

    <!-- No data -->
    <div v-else-if="!preset" class="p-6 text-center">
      <div class="text-3xl mb-2 opacity-30">&#x1F52C;</div>
      <p class="text-xs text-[var(--text-muted)]">生成网格后，点击"分析模型"获取智能推荐</p>
    </div>

    <!-- Preset Result -->
    <div v-else class="p-4 space-y-4">
      <!-- Mesh Recommendation -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <span class="w-5 h-5 rounded bg-blue-100 dark:bg-blue-900/30 text-blue-600 text-xs flex items-center justify-center font-medium">&#x2587;</span>
          <span class="text-xs font-semibold text-[var(--text-primary)]">网格设置</span>
        </div>
        <div class="bg-[var(--bg-base)] rounded p-3 space-y-2 text-xs">
          <div class="flex justify-between">
            <span class="text-[var(--text-secondary)]">单元大小</span>
            <span class="text-[var(--text-primary)] font-mono">{{ preset.mesh.elementSize.toFixed(2) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-[var(--text-secondary)]">单元阶次</span>
            <span class="text-[var(--text-primary)]">{{ preset.mesh.elementOrder === 1 ? '线性 (1阶)' : '二次 (2阶)' }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-[var(--text-secondary)]">网格类型</span>
            <span class="text-[var(--text-primary)]">{{ meshTypeLabel }}</span>
          </div>
          <div class="mt-2 pt-2 border-t border-[var(--border-subtle)]">
            <p class="text-[var(--text-muted)] leading-relaxed">{{ preset.mesh.reason }}</p>
          </div>
        </div>
      </div>

      <!-- Solver Recommendation -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <span class="w-5 h-5 rounded bg-purple-100 dark:bg-purple-900/30 text-purple-600 text-xs flex items-center justify-center font-medium">&#x2699;</span>
          <span class="text-xs font-semibold text-[var(--text-primary)]">求解器设置</span>
        </div>
        <div class="bg-[var(--bg-base)] rounded p-3 space-y-2 text-xs">
          <div class="flex justify-between">
            <span class="text-[var(--text-secondary)]">求解器类型</span>
            <span class="text-[var(--text-primary)] font-mono">{{ preset.solver.solverType }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-[var(--text-secondary)]">最大迭代</span>
            <span class="text-[var(--text-primary)] font-mono">{{ preset.solver.maxIterations }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-[var(--text-secondary)]">收敛容差</span>
            <span class="text-[var(--text-primary)] font-mono">{{ preset.solver.convergenceTolerance.toExponential(0) }}</span>
          </div>
          <div class="mt-2 pt-2 border-t border-[var(--border-subtle)]">
            <p class="text-[var(--text-muted)] leading-relaxed">{{ preset.solver.reason }}</p>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex gap-2 pt-1">
        <button
          @click="applyPreset"
          class="flex-1 px-3 py-2 text-xs rounded bg-[var(--primary)] text-white hover:opacity-90 transition font-medium"
        >
          应用推荐
        </button>
        <button
          @click="dismiss"
          class="flex-1 px-3 py-2 text-xs rounded border border-[var(--border-subtle)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)] transition"
        >
          自定义
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import {
  recommendPreset,
  extractFeatures,
  getConfidenceLabel,
  type SmartPreset,
} from '@/utils/smartPresets'

const emit = defineEmits<{
  apply: [preset: SmartPreset]
  dismiss: []
}>()

const loading = ref(false)
const preset = ref<SmartPreset | null>(null)

const confidenceInfo = computed(() => {
  if (!preset.value) return { label: '-', color: '', bgColor: '' }
  return getConfidenceLabel(preset.value.confidence)
})

const meshTypeLabel = computed(() => {
  if (!preset.value) return ''
  const labels: Record<string, string> = {
    tet: '四面体 (Tet)',
    hex: '六面体 (Hex)',
    mixed: '混合网格 (Hex+Tet)',
  }
  return labels[preset.value.mesh.meshType] || preset.value.mesh.meshType
})

/**
 * 根据当前网格数据生成推荐
 */
function generateRecommendation() {
  loading.value = true

  // 模拟短暂的分析延迟（实际分析是同步的，加一点 UX 延迟）
  setTimeout(() => {
    try {
      // 从 projectStore 获取当前网格信息
      // 这里通过 props 或 store 传入，暂时使用默认值
      const features = extractFeatures({
        nodeCount: 0,
        elementCount: 0,
        analysisType: 'static',
      })
      preset.value = recommendPreset(features)
    } catch (e) {
      console.error('智能预设置分析失败:', e)
    } finally {
      loading.value = false
    }
  }, 500)
}

/**
 * 使用外部特征数据更新推荐（供父组件调用）
 */
function updateWithFeatures(features: Parameters<typeof recommendPreset>[0]) {
  loading.value = true
  setTimeout(() => {
    preset.value = recommendPreset(features)
    loading.value = false
  }, 300)
}

function applyPreset() {
  if (preset.value) {
    emit('apply', preset.value)
  }
}

function dismiss() {
  emit('dismiss')
}

defineExpose({
  generateRecommendation,
  updateWithFeatures,
})
</script>
