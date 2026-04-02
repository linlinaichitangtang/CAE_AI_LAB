<template>
  <Teleport to="body">
    <Transition name="insight-fade">
      <div
        v-if="visible"
        class="fixed inset-0 z-[9000] flex items-end justify-center pointer-events-none"
      >
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/20 pointer-events-auto" @click="$emit('close')"></div>

        <!-- Panel -->
        <div
          class="relative pointer-events-auto w-full max-w-lg mx-4 mb-6 bg-white dark:bg-gray-800 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden"
        >
          <!-- Header -->
          <div class="px-5 py-3 bg-gradient-to-r from-blue-500 to-indigo-600 text-white">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="text-lg">&#x1F4CA;</span>
                <span class="text-sm font-semibold">仿真完成</span>
                <span v-if="resultData.solveTime" class="text-xs opacity-80">
                  &middot; 用时 {{ resultData.solveTime.toFixed(1) }}s
                </span>
              </div>
              <button
                @click="$emit('close')"
                class="w-6 h-6 rounded-full bg-white/20 hover:bg-white/30 flex items-center justify-center transition"
              >
                <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                  <line x1="18" y1="6" x2="6" y2="18" />
                  <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Body -->
          <div class="p-5 space-y-3">
            <!-- AI Summary -->
            <div v-if="aiSummary" class="text-sm text-[var(--text-secondary)] leading-relaxed bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
              <div class="flex items-center gap-1.5 mb-1">
                <span class="text-xs">&#x1F916;</span>
                <span class="text-xs font-medium text-[var(--text-primary)]">AI 总结</span>
              </div>
              {{ aiSummary }}
            </div>
            <div v-else-if="aiLoading" class="text-sm text-[var(--text-muted)] flex items-center gap-2">
              <div class="w-4 h-4 border-2 border-[var(--primary)] border-t-transparent rounded-full animate-spin"></div>
              AI 正在分析结果...
            </div>

            <!-- Displacement -->
            <div class="flex items-start gap-3 p-3 rounded-lg" :class="displacementBgClass">
              <span class="text-lg mt-0.5">{{ displacementIcon }}</span>
              <div>
                <div class="text-sm font-medium" :class="displacementTextClass">
                  最大位移：{{ formatValue(resultData.maxDisplacement) }}
                </div>
                <div class="text-xs text-[var(--text-muted)] mt-0.5">
                  {{ displacementNote }}
                </div>
              </div>
            </div>

            <!-- Stress -->
            <div class="flex items-start gap-3 p-3 rounded-lg" :class="stressBgClass">
              <span class="text-lg mt-0.5">{{ stressIcon }}</span>
              <div>
                <div class="text-sm font-medium" :class="stressTextClass">
                  最大应力：{{ formatValue(resultData.maxVonMises) }} MPa
                </div>
                <div class="text-xs text-[var(--text-muted)] mt-0.5">
                  <span v-if="resultData.materialName">材料：{{ resultData.materialName }}，</span>
                  <span v-if="safetyFactor !== null">安全系数 = {{ safetyFactor.toFixed(2) }}</span>
                  <span v-else>未提供材料屈服强度</span>
                </div>
              </div>
            </div>

            <!-- Safety Assessment -->
            <div v-if="safetyFactor !== null" class="flex items-center gap-2 p-3 rounded-lg" :class="safetyBgClass">
              <span class="text-sm font-semibold" :class="safetyTextClass">
                {{ safetyLabel }}
              </span>
              <span class="text-xs text-[var(--text-muted)]">
                {{ safetyDescription }}
              </span>
            </div>
          </div>

          <!-- Footer -->
          <div class="px-5 py-3 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-2">
            <button
              @click="$emit('view-report')"
              class="px-4 py-1.5 text-xs rounded-lg border border-gray-300 dark:border-gray-600 text-[var(--text-secondary)] hover:bg-gray-100 dark:hover:bg-gray-700 transition"
            >
              查看详细报告
            </button>
            <button
              @click="$emit('share')"
              class="px-4 py-1.5 text-xs rounded-lg bg-[var(--primary)] text-white hover:opacity-90 transition"
            >
              分享结果
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAiStore } from '@/stores/ai'

interface ResultData {
  maxDisplacement: number
  maxVonMises: number
  materialYieldStrength?: number
  materialName?: string
  solveTime?: number
  analysisType: string
}

const props = defineProps<{
  visible: boolean
  resultData: ResultData
}>()

const emit = defineEmits<{
  close: []
  'view-report': []
  share: []
}>()

const aiStore = useAiStore()
const aiSummary = ref<string | null>(null)
const aiLoading = ref(false)

// ============ 安全评估 ============

const safetyFactor = computed(() => {
  if (!props.resultData.materialYieldStrength || props.resultData.maxVonMises <= 0) return null
  return props.resultData.materialYieldStrength / props.resultData.maxVonMises
})

const safetyLabel = computed(() => {
  if (safetyFactor.value === null) return ''
  if (safetyFactor.value >= 2) return '安全'
  if (safetyFactor.value >= 1.5) return '基本安全'
  if (safetyFactor.value >= 1.0) return '需关注'
  return '不安全'
})

const safetyDescription = computed(() => {
  if (safetyFactor.value === null) return ''
  if (safetyFactor.value >= 2) return '安全系数充足，设计余量较大'
  if (safetyFactor.value >= 1.5) return '满足一般工程安全要求'
  if (safetyFactor.value >= 1.0) return '接近材料极限，建议优化'
  return '超过材料屈服强度，需立即修改设计'
})

const safetyBgClass = computed(() => {
  if (safetyFactor.value === null) return ''
  if (safetyFactor.value >= 1.5) return 'bg-green-50 dark:bg-green-900/20'
  if (safetyFactor.value >= 1.0) return 'bg-yellow-50 dark:bg-yellow-900/20'
  return 'bg-red-50 dark:bg-red-900/20'
})

const safetyTextClass = computed(() => {
  if (safetyFactor.value === null) return ''
  if (safetyFactor.value >= 1.5) return 'text-green-700 dark:text-green-400'
  if (safetyFactor.value >= 1.0) return 'text-yellow-700 dark:text-yellow-400'
  return 'text-red-700 dark:text-red-400'
})

// ============ 位移评估 ============

const DISPLACEMENT_THRESHOLD = 1.0 // mm

const displacementIcon = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD ? '\u2705' : '\u26A0\uFE0F'
})

const displacementBgClass = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD
    ? 'bg-green-50 dark:bg-green-900/20'
    : 'bg-yellow-50 dark:bg-yellow-900/20'
})

const displacementTextClass = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD
    ? 'text-green-700 dark:text-green-400'
    : 'text-yellow-700 dark:text-yellow-400'
})

const displacementNote = computed(() => {
  const val = props.resultData.maxDisplacement
  if (val < DISPLACEMENT_THRESHOLD) {
    return `< ${DISPLACEMENT_THRESHOLD} mm 目标，满足刚度要求`
  }
  return `> ${DISPLACEMENT_THRESHOLD} mm 目标，建议优化结构刚度`
})

// ============ 应力评估 ============

const stressIcon = computed(() => {
  if (safetyFactor.value === null) return '\u2139\uFE0F'
  if (safetyFactor.value >= 1.5) return '\u2705'
  if (safetyFactor.value >= 1.0) return '\u26A0\uFE0F'
  return '\u274C'
})

const stressBgClass = computed(() => {
  if (safetyFactor.value === null) return 'bg-gray-50 dark:bg-gray-700/50'
  if (safetyFactor.value >= 1.5) return 'bg-green-50 dark:bg-green-900/20'
  if (safetyFactor.value >= 1.0) return 'bg-yellow-50 dark:bg-yellow-900/20'
  return 'bg-red-50 dark:bg-red-900/20'
})

const stressTextClass = computed(() => {
  if (safetyFactor.value === null) return 'text-gray-700 dark:text-gray-400'
  if (safetyFactor.value >= 1.5) return 'text-green-700 dark:text-green-400'
  if (safetyFactor.value >= 1.0) return 'text-yellow-700 dark:text-yellow-400'
  return 'text-red-700 dark:text-red-400'
})

// ============ AI 总结 ============

async function generateAISummary() {
  if (!aiStore.isConfigured) return

  aiLoading.value = true
  try {
    const prompt = `请用一句话总结以下CAE仿真结果：
- 分析类型: ${props.resultData.analysisType}
- 最大位移: ${props.resultData.maxDisplacement.toFixed(3)} mm
- 最大Von Mises应力: ${props.resultData.maxVonMises.toFixed(2)} MPa
${props.resultData.materialName ? `- 材料: ${props.resultData.materialName}` : ''}
${props.resultData.materialYieldStrength ? `- 屈服强度: ${props.resultData.materialYieldStrength} MPa` : ''}
${safetyFactor.value !== null ? `- 安全系数: ${safetyFactor.value.toFixed(2)}` : ''}

要求：简洁专业，不超过50字。`

    const response = await invoke<string>('ai_chat_completion', {
      messages: [
        { role: 'system', content: '你是CAE仿真分析专家，请用简洁专业的中文总结仿真结果。' },
        { role: 'user', content: prompt },
      ],
      config: {
        aiSource: aiStore.config.aiSource,
        apiUrl: aiStore.config.apiUrl,
        apiKey: aiStore.config.apiKey,
        modelName: aiStore.config.modelName,
        temperature: 0.3,
        maxTokens: 200,
      },
    })

    aiSummary.value = response.trim()
  } catch (e) {
    console.error('AI 结果解读失败:', e)
    aiSummary.value = null
  } finally {
    aiLoading.value = false
  }
}

// ============ 工具函数 ============

function formatValue(val: number): string {
  if (Math.abs(val) >= 100) return val.toFixed(1)
  if (Math.abs(val) >= 1) return val.toFixed(2)
  return val.toFixed(4)
}

// ============ 生命周期 ============

watch(
  () => props.visible,
  (newVal) => {
    if (newVal) {
      aiSummary.value = null
      generateAISummary()
    }
  }
)
</script>

<style scoped>
.insight-fade-enter-active {
  transition: all 0.3s ease-out;
}

.insight-fade-leave-active {
  transition: all 0.2s ease-in;
}

.insight-fade-enter-from {
  opacity: 0;
}

.insight-fade-enter-from > div:last-child {
  transform: translateY(20px);
}

.insight-fade-leave-to {
  opacity: 0;
}

.insight-fade-leave-to > div:last-child {
  transform: translateY(10px);
}
</style>
