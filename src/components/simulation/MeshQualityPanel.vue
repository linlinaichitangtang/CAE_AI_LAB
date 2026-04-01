<template>
  <div class="mesh-quality-panel">
    <!-- 标题栏 -->
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-sm font-semibold text-[var(--text-primary)]">网格质量检查</h3>
      <button
        @click="$emit('close')"
        class="w-6 h-6 flex items-center justify-center rounded hover:bg-[var(--bg-hover)] text-[var(--text-secondary)] text-lg leading-none"
      >
        &times;
      </button>
    </div>

    <!-- 未检查时的提示 -->
    <div v-if="!hasResult" class="text-center py-6">
      <div class="text-[var(--text-secondary)] text-xs mb-3">
        点击下方按钮运行网格质量检查
      </div>
      <button
        @click="runCheck"
        :disabled="checking || !hasMesh"
        class="w-full px-3 py-2 bg-[var(--primary)] text-white rounded text-sm hover:opacity-90 disabled:opacity-50"
      >
        {{ checking ? '检查中...' : '运行质量检查' }}
      </button>
      <div v-if="!hasMesh" class="text-red-400 text-xs mt-2">
        请先生成网格
      </div>
    </div>

    <!-- 检查结果 -->
    <div v-else-if="metrics">
      <!-- 总体评分 -->
      <div class="grid grid-cols-2 gap-2 mb-3">
        <div class="bg-[var(--bg-elevated)] rounded-lg p-2">
          <div class="text-xs text-[var(--text-secondary)]">平均质量</div>
          <div
            class="text-lg font-bold"
            :style="{ color: getQualityColor(metrics.avg_quality) }"
          >
            {{ (metrics.avg_quality * 100).toFixed(1) }}%
          </div>
        </div>
        <div class="bg-[var(--bg-elevated)] rounded-lg p-2">
          <div class="text-xs text-[var(--text-secondary)]">单元总数</div>
          <div class="text-lg font-bold text-[var(--text-primary)]">
            {{ metrics.total_elements }}
          </div>
        </div>
      </div>

      <!-- 最小/最大质量 -->
      <div class="grid grid-cols-2 gap-2 mb-3">
        <div class="bg-[var(--bg-elevated)] rounded-lg p-2">
          <div class="text-xs text-[var(--text-secondary)]">最低质量</div>
          <div
            class="text-sm font-semibold"
            :style="{ color: getQualityColor(metrics.min_quality) }"
          >
            {{ (metrics.min_quality * 100).toFixed(1) }}%
          </div>
        </div>
        <div class="bg-[var(--bg-elevated)] rounded-lg p-2">
          <div class="text-xs text-[var(--text-secondary)]">最高质量</div>
          <div
            class="text-sm font-semibold"
            :style="{ color: getQualityColor(metrics.max_quality) }"
          >
            {{ (metrics.max_quality * 100).toFixed(1) }}%
          </div>
        </div>
      </div>

      <!-- 质量分布柱状图 -->
      <div class="mb-3">
        <div class="text-xs text-[var(--text-secondary)] mb-1">质量分布</div>
        <div class="flex h-4 rounded overflow-hidden bg-[var(--bg-muted)]">
          <div
            v-if="excellentPct > 0"
            class="bg-green-500 transition-all duration-300"
            :style="{ width: excellentPct + '%' }"
            :title="'优秀: ' + metrics.excellent_count"
          ></div>
          <div
            v-if="goodPct > 0"
            class="bg-blue-500 transition-all duration-300"
            :style="{ width: goodPct + '%' }"
            :title="'良好: ' + metrics.good_count"
          ></div>
          <div
            v-if="fairPct > 0"
            class="bg-yellow-500 transition-all duration-300"
            :style="{ width: fairPct + '%' }"
            :title="'一般: ' + metrics.fair_count"
          ></div>
          <div
            v-if="poorPct > 0"
            class="bg-orange-500 transition-all duration-300"
            :style="{ width: poorPct + '%' }"
            :title="'较差: ' + metrics.poor_count"
          ></div>
          <div
            v-if="badPct > 0"
            class="bg-red-500 transition-all duration-300"
            :style="{ width: badPct + '%' }"
            :title="'很差: ' + metrics.bad_count"
          ></div>
        </div>
        <div class="flex justify-between text-[10px] text-[var(--text-secondary)] mt-1">
          <span>优秀 ({{ metrics.excellent_count }})</span>
          <span>良好 ({{ metrics.good_count }})</span>
          <span>一般 ({{ metrics.fair_count }})</span>
          <span>较差 ({{ metrics.poor_count }})</span>
          <span>很差 ({{ metrics.bad_count }})</span>
        </div>
      </div>

      <!-- 问题单元列表 -->
      <div v-if="problemElements.length > 0">
        <div class="text-xs text-[var(--text-secondary)] mb-1">
          问题单元 (质量 &lt; 0.4) - 共 {{ problemElements.length }} 个
        </div>
        <div class="max-h-32 overflow-y-auto space-y-1">
          <div
            v-for="elem in problemElements"
            :key="elem.element_id"
            class="flex items-center justify-between bg-red-50 dark:bg-red-900/20 rounded px-2 py-1 text-xs"
          >
            <span class="text-[var(--text-primary)]">单元 #{{ elem.element_id }}</span>
            <div class="flex items-center gap-2">
              <span class="text-[var(--text-secondary)] font-mono">
                AR:{{ elem.aspect_ratio.toFixed(1) }}
              </span>
              <span
                class="font-mono font-semibold"
                :style="{ color: getQualityColor(elem.overall_quality) }"
              >
                {{ (elem.overall_quality * 100).toFixed(1) }}%
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- 无问题单元时的提示 -->
      <div v-else class="text-xs text-green-500 text-center py-2">
        所有单元质量良好
      </div>

      <!-- 重新检查按钮 -->
      <button
        @click="runCheck"
        :disabled="checking"
        class="w-full mt-3 px-3 py-2 bg-[var(--primary)] text-white rounded text-sm hover:opacity-90 disabled:opacity-50"
      >
        {{ checking ? '检查中...' : '重新检查' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useProjectStore } from '@/stores/project'
import { checkMeshQuality, type MeshQualityMetrics, type ElementQuality } from '@/api/cae'

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'qualityResult', metrics: MeshQualityMetrics): void
}>()

const projectStore = useProjectStore()

const checking = ref(false)
const metrics = ref<MeshQualityMetrics | null>(null)

const hasMesh = computed(() => {
  return projectStore.currentMesh !== null
    && projectStore.currentMesh.nodes.length > 0
    && projectStore.currentMesh.elements.length > 0
})

const hasResult = computed(() => metrics.value !== null)

// 质量分布百分比
const excellentPct = computed(() => {
  if (!metrics.value || metrics.value.total_elements === 0) return 0
  return (metrics.value.excellent_count / metrics.value.total_elements) * 100
})

const goodPct = computed(() => {
  if (!metrics.value || metrics.value.total_elements === 0) return 0
  return (metrics.value.good_count / metrics.value.total_elements) * 100
})

const fairPct = computed(() => {
  if (!metrics.value || metrics.value.total_elements === 0) return 0
  return (metrics.value.fair_count / metrics.value.total_elements) * 100
})

const poorPct = computed(() => {
  if (!metrics.value || metrics.value.total_elements === 0) return 0
  return (metrics.value.poor_count / metrics.value.total_elements) * 100
})

const badPct = computed(() => {
  if (!metrics.value || metrics.value.total_elements === 0) return 0
  return (metrics.value.bad_count / metrics.value.total_elements) * 100
})

// 问题单元（质量 < 0.4）
const problemElements = computed(() => {
  if (!metrics.value) return []
  return metrics.value.element_qualities
    .filter(eq => eq.overall_quality < 0.4)
    .sort((a, b) => a.overall_quality - b.overall_quality)
})

/**
 * 根据质量分数返回颜色
 * @param quality 0~1 的质量分数
 */
function getQualityColor(quality: number): string {
  if (quality >= 0.8) return '#22c55e'   // 绿色 - 优秀
  if (quality >= 0.6) return '#3b82f6'   // 蓝色 - 良好
  if (quality >= 0.4) return '#eab308'   // 黄色 - 一般
  if (quality >= 0.2) return '#f97316'   // 橙色 - 较差
  return '#ef4444'                        // 红色 - 很差
}

/**
 * 推断单元类型
 */
function inferElementType(): string {
  if (!projectStore.currentMesh || projectStore.currentMesh.elements.length === 0) {
    return 'C3D8'
  }
  const firstElem = projectStore.currentMesh.elements[0]
  const nodeCount = firstElem.node_ids.length

  // 检查是否有 z 坐标（判断2D还是3D）
  const nodes = projectStore.currentMesh.nodes
  let is3d = false
  for (const node of nodes) {
    if (Math.abs(node.z) > 1e-10) {
      is3d = true
      break
    }
  }

  if (is3d) {
    switch (nodeCount) {
      case 4: return 'C3D4'
      case 6: return 'C3D6'
      case 8: return 'C3D8'
      case 10: return 'C3D10'
      case 20: return 'C3D20'
      default: return 'C3D8'
    }
  } else {
    switch (nodeCount) {
      case 3: return 'S3'
      case 4: return 'S4'
      case 6: return 'S6'
      case 8: return 'S8'
      default: return 'S4'
    }
  }
}

/**
 * 运行网格质量检查
 */
async function runCheck() {
  if (!projectStore.currentMesh) return

  checking.value = true
  try {
    const elementType = inferElementType()
    const result = await checkMeshQuality(
      projectStore.currentMesh.nodes,
      projectStore.currentMesh.elements,
      elementType
    )
    metrics.value = result
    emit('qualityResult', result)
  } catch (err) {
    console.error('Mesh quality check failed:', err)
    metrics.value = null
  } finally {
    checking.value = false
  }
}
</script>

<style scoped>
.mesh-quality-panel {
  padding: 12px;
}

.mesh-quality-panel::-webkit-scrollbar {
  width: 4px;
}

.mesh-quality-panel::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 2px;
}
</style>
