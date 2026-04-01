<template>
  <div class="contact-results space-y-4">
    <!-- Result Type Tabs -->
    <div class="flex gap-1 bg-[var(--bg-elevated)] rounded-lg p-1">
      <button
        v-for="tab in resultTabs"
        :key="tab.id"
        @click="activeTab = tab.id"
        :class="[
          'px-3 py-1.5 text-xs rounded transition-colors',
          activeTab === tab.id
            ? 'bg-[var(--accent-blue)] text-white'
            : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'
        ]"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- No Data Message -->
    <div v-if="!contactResults" class="panel-section">
      <div class="text-center text-xs text-[var(--text-muted)] py-4">
        暂无接触分析结果，请运行接触分析后查看
      </div>
    </div>

    <template v-else>
      <!-- Contact Pressure -->
      <div v-if="activeTab === 'pressure'" class="panel-section">
        <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">接触压力分布</h4>
        <div class="space-y-3">
          <div class="result-summary">
            <div class="flex justify-between items-center">
              <span class="text-xs text-[var(--text-muted)]">最大压力</span>
              <span class="text-sm font-medium text-[var(--accent-blue)]">
                {{ (contactResults.summary.maxPressure ?? 0).toFixed(2) }} MPa
              </span>
            </div>
            <div class="flex justify-between items-center mt-2">
              <span class="text-xs text-[var(--text-muted)]">平均压力</span>
              <span class="text-sm text-[var(--text-primary)]">
                {{ avgPressure.toFixed(2) }} MPa
              </span>
            </div>
          </div>

          <!-- Pressure Distribution Bar -->
          <div class="pressure-bar">
            <div class="bar-track">
              <div
                class="bar-fill"
                :style="{ width: `${Math.min((contactResults.summary.maxPressure / pressureScale) * 100, 100)}%` }"
              ></div>
            </div>
            <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
              <span>0</span>
              <span>{{ pressureScale.toFixed(1) }}</span>
            </div>
          </div>

          <!-- Contour Legend -->
          <div class="contour-legend">
            <div class="gradient-bar" :style="pressureGradientStyle"></div>
            <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
              <span>0 MPa</span>
              <span>{{ (contactResults.summary.maxPressure ?? 0).toFixed(1) }} MPa</span>
            </div>
          </div>

          <!-- Pressure Data Table -->
          <div v-if="contactResults.pressure && contactResults.pressure.length > 0" class="max-h-32 overflow-y-auto">
            <table class="w-full text-[10px]">
              <thead>
                <tr class="text-[var(--text-muted)]">
                  <th class="text-left py-1">节点 ID</th>
                  <th class="text-right py-1">压力 (MPa)</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in contactResults.pressure.slice(0, 20)" :key="item.nodeId">
                  <td class="py-0.5">{{ item.nodeId }}</td>
                  <td class="text-right py-0.5">{{ item.value.toFixed(4) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Slip Distribution -->
      <div v-if="activeTab === 'slip'" class="panel-section">
        <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">滑移分布</h4>
        <div class="space-y-3">
          <div class="result-summary">
            <div class="flex justify-between items-center">
              <span class="text-xs text-[var(--text-muted)]">最大滑移</span>
              <span class="text-sm font-medium text-[var(--accent-blue)]">
                {{ (contactResults.summary.maxSlip ?? 0).toFixed(4) }} mm
              </span>
            </div>
            <div class="flex justify-between items-center mt-2">
              <span class="text-xs text-[var(--text-muted)]">平均滑移</span>
              <span class="text-sm text-[var(--text-primary)]">
                {{ avgSlip.toFixed(4) }} mm
              </span>
            </div>
          </div>

          <!-- Slip Visualization -->
          <div class="slip-visualization">
            <div class="slip-indicator" :style="slipIndicatorStyle"></div>
          </div>

          <!-- Slip Data Table -->
          <div v-if="contactResults.slip && contactResults.slip.length > 0" class="max-h-32 overflow-y-auto">
            <table class="w-full text-[10px]">
              <thead>
                <tr class="text-[var(--text-muted)]">
                  <th class="text-left py-1">节点 ID</th>
                  <th class="text-right py-1">滑移 (mm)</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in contactResults.slip.slice(0, 20)" :key="item.nodeId">
                  <td class="py-0.5">{{ item.nodeId }}</td>
                  <td class="text-right py-0.5">{{ item.value.toFixed(6) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Penetration (Gap) Check -->
      <div v-if="activeTab === 'penetration'" class="panel-section">
        <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">穿透检查</h4>
        <div class="space-y-3">
          <div class="penetration-status" :class="penetrationStatusClass">
            <span class="status-icon">{{ penetrationStatusIcon }}</span>
            <span class="status-text">{{ penetrationStatusText }}</span>
          </div>

          <div class="result-summary">
            <div class="flex justify-between items-center">
              <span class="text-xs text-[var(--text-muted)]">最大穿透量</span>
              <span class="text-sm font-medium" :class="penetrationValueClass">
                {{ (contactResults.summary.maxGap ?? 0).toFixed(4) }} mm
              </span>
            </div>
            <div class="flex justify-between items-center mt-2">
              <span class="text-xs text-[var(--text-muted)]">穿透节点数</span>
              <span class="text-sm text-[var(--text-primary)]">
                {{ penetratedNodeCount }} / {{ totalGapNodes }}
              </span>
            </div>
          </div>

          <div v-if="penetratedNodeCount > 0" class="penetration-warning">
            <span class="text-xs">存在穿透，请检查网格或增加接触刚度</span>
          </div>

          <!-- Gap Data Table -->
          <div v-if="contactResults.gap && contactResults.gap.length > 0" class="max-h-32 overflow-y-auto">
            <table class="w-full text-[10px]">
              <thead>
                <tr class="text-[var(--text-muted)]">
                  <th class="text-left py-1">节点 ID</th>
                  <th class="text-right py-1">间隙 (mm)</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in contactResults.gap.slice(0, 20)" :key="item.nodeId">
                  <td class="py-0.5">{{ item.nodeId }}</td>
                  <td class="text-right py-0.5">{{ item.value.toFixed(6) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Bolt Force -->
      <div v-if="activeTab === 'bolt'" class="panel-section">
        <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">螺栓力输出</h4>
        <div v-if="!contactResults.boltForce || contactResults.boltForce.length === 0" class="text-xs text-[var(--text-muted)] text-center py-2">
          无螺栓力数据
        </div>
        <div v-else class="space-y-3">
          <div class="bolt-list">
            <div v-for="bolt in contactResults.boltForce" :key="bolt.boltId" class="bolt-item">
              <div class="flex justify-between items-center">
                <span class="text-xs font-medium">Bolt-{{ bolt.boltId }}</span>
                <span class="text-sm text-[var(--accent-blue)]">{{ bolt.currentForce.toFixed(1) }} N</span>
              </div>

              <!-- Preload Progress -->
              <div class="bolt-progress mt-2">
                <div class="progress-bar">
                  <div
                    class="progress-fill"
                    :style="{ width: `${Math.min((bolt.currentForce / bolt.pretension) * 100, 100)}%` }"
                  ></div>
                </div>
                <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
                  <span>预紧: {{ bolt.pretension.toFixed(0) }} N</span>
                  <span>{{ ((bolt.currentForce / bolt.pretension) * 100).toFixed(0) }}%</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Contact Force Summary -->
      <div class="panel-section">
        <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">接触力汇总</h4>
        <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2">
          <div class="contact-pair-summary">
            <div class="flex justify-between">
              <span class="text-xs text-[var(--text-muted)]">总接触力</span>
              <span class="text-xs text-[var(--text-primary)]">{{ (contactResults.summary.totalContactForce ?? 0).toFixed(1) }} N</span>
            </div>
            <div class="w-full bg-[var(--bg-surface)] rounded h-1 mt-1">
              <div
                class="h-1 bg-[var(--accent-blue)] rounded"
                :style="{ width: `${Math.min((contactResults.summary.totalContactForce / (contactResults.summary.totalContactForce * 1.2 || 1)) * 100, 100)}%` }"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </template>

    <!-- Export Button -->
    <div class="panel-section">
      <button @click="exportResults" class="btn btn-secondary w-full text-xs" :disabled="!contactResults">
        <span class="mr-1">📊</span> 导出接触结果
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

/** Contact results data structure */
export interface ContactResultData {
  pressure: Array<{ nodeId: number; value: number }>
  slip: Array<{ nodeId: number; value: number }>
  gap: Array<{ nodeId: number; value: number }>
  boltForce?: Array<{ boltId: number; pretension: number; currentForce: number }>
  summary: {
    maxPressure: number
    maxSlip: number
    maxGap: number
    totalContactForce: number
  }
}

const props = defineProps<{
  contactResults?: ContactResultData
}>();

// Result tabs
const resultTabs = [
  { id: 'pressure', label: '接触压力' },
  { id: 'slip', label: '滑移分布' },
  { id: 'penetration', label: '穿透检查' },
  { id: 'bolt', label: '螺栓力' },
];

const activeTab = ref('pressure');

// Computed: average pressure
const avgPressure = computed(() => {
  if (!props.contactResults?.pressure?.length) return 0;
  const sum = props.contactResults.pressure.reduce((acc, p) => acc + p.value, 0);
  return sum / props.contactResults.pressure.length;
});

// Computed: pressure scale for bar visualization
const pressureScale = computed(() => {
  const maxP = props.contactResults?.summary?.maxPressure ?? 0;
  return maxP > 0 ? Math.ceil(maxP / 100) * 100 : 200;
});

// Computed: average slip
const avgSlip = computed(() => {
  if (!props.contactResults?.slip?.length) return 0;
  const sum = props.contactResults.slip.reduce((acc, s) => acc + s.value, 0);
  return sum / props.contactResults.slip.length;
});

// Computed: penetrated node count
const penetratedNodeCount = computed(() => {
  if (!props.contactResults?.gap?.length) return 0;
  return props.contactResults.gap.filter(g => g.value < 0).length;
});

// Computed: total gap nodes
const totalGapNodes = computed(() => {
  return props.contactResults?.gap?.length ?? 0;
});

// Computed styles
const pressureGradientStyle = computed(() => ({
  background: 'linear-gradient(to right, #1a5fb4, #26a269, #e5a50a, #c64600)',
}));

const slipIndicatorStyle = computed(() => ({
  width: `${Math.min(((props.contactResults?.summary?.maxSlip ?? 0) / 0.1) * 100, 100)}%`,
}));

// Penetration status
const penetrationStatusClass = computed(() => {
  if (penetratedNodeCount.value === 0) return 'status-ok';
  if (penetratedNodeCount.value < 10) return 'status-warning';
  return 'status-error';
});

const penetrationStatusIcon = computed(() => {
  if (penetratedNodeCount.value === 0) return '✓';
  if (penetratedNodeCount.value < 10) return '⚠';
  return '✕';
});

const penetrationStatusText = computed(() => {
  if (penetratedNodeCount.value === 0) return '无穿透';
  if (penetratedNodeCount.value < 10) return '轻微穿透';
  return '严重穿透';
});

const penetrationValueClass = computed(() => {
  const maxGap = props.contactResults?.summary?.maxGap ?? 0;
  if (maxGap < 0.01) return 'text-green-400';
  if (maxGap < 0.1) return 'text-yellow-400';
  return 'text-red-400';
});

// Methods
const exportResults = () => {
  if (!props.contactResults) return;

  const data = {
    pressure: props.contactResults.pressure,
    slip: props.contactResults.slip,
    gap: props.contactResults.gap,
    boltForce: props.contactResults.boltForce,
    summary: props.contactResults.summary,
  };

  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `contact_results_${Date.now()}.json`;
  a.click();
  URL.revokeObjectURL(url);
};
</script>

<style scoped>
.panel-section {
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-subtle);
}

.panel-section:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.result-summary {
  background: var(--bg-elevated);
  border-radius: 8px;
  padding: 12px;
}

.pressure-bar .bar-track {
  height: 8px;
  background: var(--bg-surface);
  border-radius: 4px;
  overflow: hidden;
}

.pressure-bar .bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #1a5fb4, #26a269, #e5a50a, #c64600);
  transition: width 0.3s ease;
}

.contour-legend .gradient-bar {
  height: 12px;
  border-radius: 4px;
}

.slip-visualization {
  height: 40px;
  background: var(--bg-elevated);
  border-radius: 8px;
  position: relative;
  overflow: hidden;
}

.slip-indicator {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: linear-gradient(90deg, transparent, var(--accent-blue), var(--accent-blue));
  opacity: 0.3;
}

.penetration-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 8px;
}

.penetration-status.status-ok {
  background: rgba(38, 162, 105, 0.1);
  color: #26a269;
}

.penetration-status.status-warning {
  background: rgba(229, 165, 10, 0.1);
  color: #e5a50a;
}

.penetration-status.status-error {
  background: rgba(198, 70, 0, 0.1);
  color: #c64600;
}

.penetration-status .status-icon {
  font-size: 16px;
}

.penetration-status .status-text {
  font-size: 12px;
  font-weight: 500;
}

.penetration-warning {
  padding: 8px;
  background: rgba(229, 165, 10, 0.1);
  border-radius: 6px;
  color: #e5a50a;
}

.bolt-item {
  background: var(--bg-elevated);
  border-radius: 8px;
  padding: 12px;
}

.bolt-progress .progress-bar {
  height: 6px;
  background: var(--bg-surface);
  border-radius: 3px;
  overflow: hidden;
}

.bolt-progress .progress-fill {
  height: 100%;
  background: var(--accent-blue);
  transition: width 0.3s ease;
}

.force-item {
  display: flex;
  justify-content: space-between;
  padding: 4px 0;
}

.contact-pair-summary {
  padding: 4px 0;
}
</style>
