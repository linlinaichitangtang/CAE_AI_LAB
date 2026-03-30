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

    <!-- Contact Pressure -->
    <div v-if="activeTab === 'pressure'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">接触压力分布</h4>
      <div class="space-y-3">
        <div class="result-summary">
          <div class="flex justify-between items-center">
            <span class="text-xs text-[var(--text-muted)]">最大压力</span>
            <span class="text-sm font-medium text-[var(--accent-blue)]">
              {{ contactPressureData.max.toFixed(2) }} MPa
            </span>
          </div>
          <div class="flex justify-between items-center mt-2">
            <span class="text-xs text-[var(--text-muted)]">平均压力</span>
            <span class="text-sm text-[var(--text-primary)]">
              {{ contactPressureData.avg.toFixed(2) }} MPa
            </span>
          </div>
        </div>
        
        <!-- Pressure Distribution Bar -->
        <div class="pressure-bar">
          <div class="bar-track">
            <div 
              class="bar-fill" 
              :style="{ width: `${(contactPressureData.max / contactPressureData.scale) * 100}%` }"
            ></div>
          </div>
          <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
            <span>0</span>
            <span>{{ contactPressureData.scale.toFixed(1) }}</span>
          </div>
        </div>
        
        <!-- Contour Legend -->
        <div class="contour-legend">
          <div class="gradient-bar" :style="pressureGradientStyle"></div>
          <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
            <span>0 MPa</span>
            <span>{{ contactPressureData.max.toFixed(1) }} MPa</span>
          </div>
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
              {{ slipData.max.toFixed(4) }} mm
            </span>
          </div>
          <div class="flex justify-between items-center mt-2">
            <span class="text-xs text-[var(--text-muted)]">平均滑移</span>
            <span class="text-sm text-[var(--text-primary)]">
              {{ slipData.avg.toFixed(4) }} mm
            </span>
          </div>
        </div>
        
        <!-- Slip Visualization -->
        <div class="slip-visualization">
          <div class="slip-indicator" :style="slipIndicatorStyle"></div>
        </div>
        
        <!-- Direction -->
        <div class="text-xs text-[var(--text-muted)]">
          主要滑移方向: {{ slipData.direction }}
        </div>
      </div>
    </div>

    <!-- Penetration Check -->
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
              {{ penetrationData.maxPenetration.toFixed(4) }} mm
            </span>
          </div>
          <div class="flex justify-between items-center mt-2">
            <span class="text-xs text-[var(--text-muted)]">穿透节点数</span>
            <span class="text-sm text-[var(--text-primary)]">
              {{ penetrationData.penetratedNodes }} / {{ penetrationData.totalNodes }}
            </span>
          </div>
        </div>
        
        <div v-if="penetrationData.penetratedNodes > 0" class="penetration-warning">
          <span class="text-xs">⚠️ 存在穿透，请检查网格或增加接触刚度</span>
        </div>
      </div>
    </div>

    <!-- Bolt Force -->
    <div v-if="activeTab === 'bolt'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">螺栓力输出</h4>
      <div class="space-y-3">
        <div class="bolt-list">
          <div v-for="(bolt, index) in boltForces" :key="index" class="bolt-item">
            <div class="flex justify-between items-center">
              <span class="text-xs font-medium">{{ bolt.name }}</span>
              <span class="text-sm text-[var(--accent-blue)]">{{ bolt.preload.toFixed(1) }} N</span>
            </div>
            
            <!-- Preload Progress -->
            <div class="bolt-progress mt-2">
              <div class="progress-bar">
                <div 
                  class="progress-fill" 
                  :style="{ width: `${(bolt.preload / bolt.targetPreload) * 100}%` }"
                ></div>
              </div>
              <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
                <span>目标: {{ bolt.targetPreload.toFixed(0) }} N</span>
                <span>{{ ((bolt.preload / bolt.targetPreload) * 100).toFixed(0) }}%</span>
              </div>
            </div>
            
            <!-- Additional Forces -->
            <div class="grid grid-cols-2 gap-2 mt-2 text-xs">
              <div class="force-item">
                <span class="text-[var(--text-muted)]">剪切力</span>
                <span class="text-[var(--text-primary)]">{{ bolt.shearForce.toFixed(1) }} N</span>
              </div>
              <div class="force-item">
                <span class="text-[var(--text-muted)]">弯矩</span>
                <span class="text-[var(--text-primary)]">{{ bolt.moment.toFixed(1) }} N·mm</span>
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
        <div v-for="pair in contactPairSummary" :key="pair.name" class="contact-pair-summary">
          <div class="flex justify-between">
            <span class="text-xs text-[var(--text-muted)]">{{ pair.name }}</span>
            <span class="text-xs text-[var(--text-primary)]">{{ pair.force.toFixed(1) }} N</span>
          </div>
          <div class="w-full bg-[var(--bg-surface)] rounded h-1 mt-1">
            <div 
              class="h-1 bg-[var(--accent-blue)] rounded" 
              :style="{ width: `${(pair.force / pair.maxForce) * 100}%` }"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Export Button -->
    <div class="panel-section">
      <button @click="exportResults" class="btn btn-secondary w-full text-xs">
        <span class="mr-1">📊</span> 导出接触结果
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

// Result tabs
const resultTabs = [
  { id: 'pressure', label: '接触压力' },
  { id: 'slip', label: '滑移分布' },
  { id: 'penetration', label: '穿透检查' },
  { id: 'bolt', label: '螺栓力' },
];

const activeTab = ref('pressure');

// Contact pressure data
const contactPressureData = ref({
  max: 125.5,
  avg: 45.2,
  scale: 200,
});

// Slip data
const slipData = ref({
  max: 0.0235,
  avg: 0.0089,
  direction: 'X + Y',
});

// Penetration data
const penetrationData = ref({
  maxPenetration: 0.0012,
  penetratedNodes: 3,
  totalNodes: 150,
});

// Bolt forces
const boltForces = ref([
  {
    name: 'Bolt-1',
    preload: 8500,
    targetPreload: 10000,
    shearForce: 1250,
    moment: 450,
  },
  {
    name: 'Bolt-2',
    preload: 8200,
    targetPreload: 10000,
    shearForce: 1100,
    moment: 380,
  },
]);

// Contact pair summary
const contactPairSummary = ref([
  { name: '接触对 1 (法兰)', force: 4500, maxForce: 5000 },
  { name: '接触对 2 (底座)', force: 2800, maxForce: 5000 },
]);

// Computed styles
const pressureGradientStyle = computed(() => ({
  background: 'linear-gradient(to right, #1a5fb4, #26a269, #e5a50a, #c64600)',
}));

const slipIndicatorStyle = computed(() => ({
  width: `${Math.min((slipData.value.max / 0.1) * 100, 100)}%`,
}));

// Penetration status
const penetrationStatusClass = computed(() => {
  if (penetrationData.value.penetratedNodes === 0) return 'status-ok';
  if (penetrationData.value.penetratedNodes < 10) return 'status-warning';
  return 'status-error';
});

const penetrationStatusIcon = computed(() => {
  if (penetrationData.value.penetratedNodes === 0) return '✓';
  if (penetrationData.value.penetratedNodes < 10) return '⚠';
  return '✕';
});

const penetrationStatusText = computed(() => {
  if (penetrationData.value.penetratedNodes === 0) return '无穿透';
  if (penetrationData.value.penetratedNodes < 10) return '轻微穿透';
  return '严重穿透';
});

const penetrationValueClass = computed(() => {
  if (penetrationData.value.maxPenetration < 0.01) return 'text-green-400';
  if (penetrationData.value.maxPenetration < 0.1) return 'text-yellow-400';
  return 'text-red-400';
});

// Methods
const exportResults = () => {
  const data = {
    contactPressure: contactPressureData.value,
    slip: slipData.value,
    penetration: penetrationData.value,
    bolts: boltForces.value,
    pairs: contactPairSummary.value,
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