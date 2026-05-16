<template>
  <Teleport to="body">
    <Transition name="insight-fade">
      <div
        v-if="visible"
        class="fixed inset-0 z-[9000] flex items-end justify-center pointer-events-none"
      >
        <!-- Backdrop with blur -->
        <div class="absolute inset-0 bg-black/30 backdrop-blur-sm pointer-events-auto" @click="$emit('close')"></div>

        <!-- Panel with glass effect -->
        <div
          class="relative pointer-events-auto w-full max-w-lg mx-4 mb-6 bg-white/95 dark:bg-gray-800/95 backdrop-blur-xl rounded-2xl shadow-2xl border border-white/20 overflow-hidden"
        >
          <!-- Header with gradient -->
          <div class="px-5 py-4 bg-gradient-to-r from-blue-600 via-indigo-600 to-purple-600 text-white">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
                  <span class="text-xl">&#x1F4CA;</span>
                </div>
                <div>
                  <div class="text-base font-semibold">仿真完成</div>
                  <div v-if="resultData.solveTime" class="text-xs opacity-80">
                    用时 {{ resultData.solveTime.toFixed(1) }}s · {{ resultData.analysisType }}
                  </div>
                </div>
              </div>
              <button
                @click="$emit('close')"
                class="w-8 h-8 rounded-full bg-white/20 hover:bg-white/30 flex items-center justify-center transition-all hover:scale-110"
              >
                <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                  <line x1="18" y1="6" x2="6" y2="18" />
                  <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Body with better spacing -->
          <div class="p-5 space-y-4">
            <!-- AI Summary with enhanced styling -->
            <div v-if="aiSummary" class="ai-summary-card">
              <div class="flex items-center gap-2 mb-2">
                <div class="w-6 h-6 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-500 flex items-center justify-center">
                  <span class="text-xs">&#x1F916;</span>
                </div>
                <span class="text-sm font-semibold text-[var(--text-primary)]">AI 智能分析</span>
              </div>
              <p class="text-sm text-[var(--text-secondary)] leading-relaxed">{{ aiSummary }}</p>
            </div>
            <div v-else-if="aiLoading" class="ai-loading-card">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 border-3 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                <div>
                  <div class="text-sm font-medium text-[var(--text-primary)]">AI 正在分析结果</div>
                  <div class="text-xs text-[var(--text-muted)]">基于仿真数据生成专业解读</div>
                </div>
              </div>
            </div>

            <!-- Metrics Grid -->
            <div class="grid grid-cols-2 gap-3">
              <!-- Displacement Card -->
              <div class="metric-card" :class="displacementBgClass">
                <div class="flex items-center gap-2 mb-2">
                  <div class="w-8 h-8 rounded-lg flex items-center justify-center text-lg" :class="displacementIconBg">
                    {{ displacementIcon }}
                  </div>
                  <span class="text-xs font-medium text-[var(--text-muted)]">最大位移</span>
                </div>
                <div class="text-xl font-bold" :class="displacementTextClass">
                  {{ formatValue(resultData.maxDisplacement) }}
                  <span class="text-xs font-normal text-[var(--text-muted)]">mm</span>
                </div>
                <div class="text-xs text-[var(--text-muted)]] mt-1">{{ displacementNote }}</div>
              </div>

              <!-- Stress Card -->
              <div class="metric-card" :class="stressBgClass">
                <div class="flex items-center gap-2 mb-2">
                  <div class="w-8 h-8 rounded-lg flex items-center justify-center text-lg" :class="stressIconBg">
                    {{ stressIcon }}
                  </div>
                  <span class="text-xs font-medium text-[var(--text-muted)]">最大应力</span>
                </div>
                <div class="text-xl font-bold" :class="stressTextClass">
                  {{ formatValue(resultData.maxVonMises) }}
                  <span class="text-xs font-normal text-[var(--text-muted)]">MPa</span>
                </div>
                <div v-if="resultData.materialName" class="text-xs text-[var(--text-muted)] mt-1">{{ resultData.materialName }}</div>
              </div>
            </div>

            <!-- Safety Assessment with enhanced visuals -->
            <div v-if="safetyFactor !== null" class="safety-card" :class="safetyBgClass">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl" :class="safetyIconBg">
                    {{ safetyIcon }}
                  </div>
                  <div>
                    <div class="text-base font-bold" :class="safetyTextClass">
                      {{ safetyLabel }}
                    </div>
                    <div class="text-xs text-[var(--text-muted)] mt-0.5">
                      安全系数 = {{ safetyFactor.toFixed(2) }}
                    </div>
                  </div>
                </div>
                <div class="text-right">
                  <div class="text-xs text-[var(--text-muted)]">设计评估</div>
                  <div class="text-sm font-medium" :class="safetyTextClass">{{ safetyLevel }}</div>
                </div>
              </div>
              <div class="mt-3 pt-3 border-t border-current/10">
                <div class="flex items-center gap-2 text-xs" :class="safetyTextClass">
                  <span>&#x2139;</span>
                  <span>{{ safetyDescription }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Footer with action buttons -->
          <div class="px-5 py-4 border-t border-gray-200/50 dark:border-gray-700/50 flex items-center justify-between">
            <div class="flex items-center gap-2">
              <span class="text-xs text-[var(--text-muted)]">快捷键</span>
              <span class="kbd">Ctrl</span><span>+</span><span class="kbd">Shift</span><span>+</span><span class="kbd">I</span>
            </div>
            <div class="flex items-center gap-2">
              <button
                @click="$emit('view-report')"
                class="btn-secondary text-sm"
              >
                详细报告
              </button>
              <button
                @click="$emit('share')"
                class="btn-premium text-sm"
              >
                分享结果
              </button>
            </div>
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

// ============ 安全等级 ============

const safetyLevel = computed(() => {
  if (safetyFactor.value === null) return 'N/A'
  if (safetyFactor.value >= 2) return '优秀'
  if (safetyFactor.value >= 1.5) return '良好'
  if (safetyFactor.value >= 1.0) return '一般'
  return '危险'
})

const safetyIcon = computed(() => {
  if (safetyFactor.value === null) return 'ℹ'
  if (safetyFactor.value >= 2) return '🟢'
  if (safetyFactor.value >= 1.5) return '🟡'
  return '🔴'
})

const safetyIconBg = computed(() => {
  if (safetyFactor.value === null) return 'bg-gray-100'
  if (safetyFactor.value >= 1.5) return 'bg-green-100'
  if (safetyFactor.value >= 1.0) return 'bg-yellow-100'
  return 'bg-red-100'
})

// ============ 位移评估 ============

const DISPLACEMENT_THRESHOLD = 1.0 // mm

const displacementIcon = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD ? '✅' : '⚠️'
})

const displacementBgClass = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD
    ? 'metric-card bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20'
    : 'metric-card bg-gradient-to-br from-yellow-50 to-orange-50 dark:from-yellow-900/20 dark:to-orange-900/20'
})

const displacementIconBg = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD
    ? 'bg-green-100 text-green-600'
    : 'bg-yellow-100 text-yellow-600'
})

const displacementTextClass = computed(() => {
  return props.resultData.maxDisplacement < DISPLACEMENT_THRESHOLD
    ? 'text-green-600 dark:text-green-400'
    : 'text-yellow-600 dark:text-yellow-400'
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

const stressIconBg = computed(() => {
  if (safetyFactor.value === null) return 'bg-gray-100 dark:bg-gray-700'
  if (safetyFactor.value >= 1.5) return 'bg-green-100 dark:bg-green-900/40'
  if (safetyFactor.value >= 1.0) return 'bg-yellow-100 dark:bg-yellow-900/40'
  return 'bg-red-100 dark:bg-red-900/40'
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
