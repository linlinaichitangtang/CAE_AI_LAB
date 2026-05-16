<script setup lang="ts">
/**
 * ComplianceHubView.vue — V4.1 工程可信中心
 * 聚合：合规检查、报告生成、结果置信度、标准件库、CAD同步、溯源追踪
 */
import { ref, computed } from 'vue'
import ComplianceCheckPanel from '@/components/compliance/ComplianceCheckPanel.vue'
import ReportGeneratorPanel from '@/components/compliance/ReportGeneratorPanel.vue'
import ResultConfidenceBadge from '@/components/compliance/ResultConfidenceBadge.vue'
import EnterpriseLibraryPanel from '@/components/compliance/EnterpriseLibraryPanel.vue'
import CADSyncPanel from '@/components/compliance/CADSyncPanel.vue'
import ProvenanceTracker from '@/components/compliance/ProvenanceTracker.vue'

const activeModule = ref('compliance')

const modules = [
  { id: 'compliance', label: '合规检查', icon: '📋' },
  { id: 'report', label: '报告生成', icon: '📑' },
  { id: 'confidence', label: '结果置信度', icon: '📊' },
  { id: 'library', label: '标准件库', icon: '🏭' },
  { id: 'cadsync', label: 'CAD同步', icon: '🔗' },
  { id: 'provenance', label: '溯源追踪', icon: '📜' }
]

// 模拟仿真结果（实际应从 store 或父组件传入）
const mockSimulationResults = computed(() => ({
  maxStress: 245.6,
  maxDisplacement: 0.82,
  safetyFactor: 2.15,
  meshElements: 48200,
  meshNodes: 12800,
  materialName: 'Q355B',
  analysisType: '结构静力分析'
}))

// 模拟置信度报告（实际应从 useResultConfidence 获取）
const mockConfidenceReport = {
  overallConfidence: 0.78,
  confidenceInterval: {
    lower: 220.5,
    upper: 270.7,
    unit: 'MPa'
  },
  dimensions: [
    {
      name: 'mesh_convergence',
      label: '网格收敛性',
      value: 0.75,
      errorEstimate: 5.2,
      description: '基于网格规模估算（48200 单元），未进行收敛性测试'
    },
    {
      name: 'mesh_quality',
      label: '网格质量',
      value: 0.85,
      errorEstimate: 3.1,
      description: 'Jacobian 最小值: 0.312, 最大长宽比: 4.2, 平均歪斜度: 0.18'
    },
    {
      name: 'model_uncertainty',
      label: '模型不确定性',
      value: 0.72,
      errorEstimate: 7.3,
      description: '包含材料线性假设(±3%)、小变形假设(±2%)、边界条件理想化(±5%)等误差源'
    },
    {
      name: 'bc_sensitivity',
      label: '边界条件敏感度',
      value: 0.80,
      errorEstimate: 8.4,
      description: '基于典型边界条件误差估计，固定约束通常引入 ±5% 误差，接触分析可能高达 ±15%'
    }
  ],
  recommendations: [
    '建议加密网格，当前网格密度可能不足以捕捉应力梯度',
    '边界条件可能对结果有显著影响，建议进行敏感度分析'
  ]
}
</script>

<template>
  <div class="compliance-hub">
    <div class="hub-header">
      <h1 class="hub-title">🏗️ 工程可信中心</h1>
      <p class="hub-subtitle">合规检查 · 报告生成 · 结果置信度 · 标准件库 · CAD同步 · 溯源追踪</p>
    </div>

    <div class="hub-layout">
      <!-- 侧边栏模块导航 -->
      <aside class="hub-sidebar">
        <nav class="module-nav">
          <button
            v-for="mod in modules"
            :key="mod.id"
            class="module-btn"
            :class="{ active: activeModule === mod.id }"
            @click="activeModule = mod.id"
          >
            <span class="module-icon">{{ mod.icon }}</span>
            <span class="module-label">{{ mod.label }}</span>
          </button>
        </nav>
      </aside>

      <!-- 主内容区 -->
      <main class="hub-content">
        <!-- 合规检查 -->
        <div v-if="activeModule === 'compliance'" class="module-panel">
          <ComplianceCheckPanel />
        </div>

        <!-- 报告生成 -->
        <div v-if="activeModule === 'report'" class="module-panel">
          <ReportGeneratorPanel :simulation-results="mockSimulationResults" />
        </div>

        <!-- 结果置信度 -->
        <div v-if="activeModule === 'confidence'" class="module-panel">
          <ResultConfidenceBadge :report="mockConfidenceReport" />
        </div>

        <!-- 标准件库 -->
        <div v-if="activeModule === 'library'" class="module-panel">
          <EnterpriseLibraryPanel />
        </div>

        <!-- CAD同步 -->
        <div v-if="activeModule === 'cadsync'" class="module-panel">
          <CADSyncPanel />
        </div>

        <!-- 溯源追踪 -->
        <div v-if="activeModule === 'provenance'" class="module-panel">
          <ProvenanceTracker />
        </div>
      </main>
    </div>
  </div>
</template>

<style scoped>
.compliance-hub {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-base, #f8fafc);
}

.hub-header {
  padding: 20px 24px 12px;
  background: white;
  border-bottom: 1px solid var(--border-color, #e2e8f0);
}

.hub-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary, #1e293b);
  margin: 0 0 4px 0;
}

.hub-subtitle {
  font-size: 13px;
  color: var(--text-muted, #94a3b8);
  margin: 0;
}

.hub-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.hub-sidebar {
  width: 180px;
  background: white;
  border-right: 1px solid var(--border-color, #e2e8f0);
  padding: 12px;
  overflow-y: auto;
}

.module-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.module-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  border: none;
  background: transparent;
  font-size: 13px;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.module-btn:hover {
  background: var(--bg-elevated, #f8fafc);
}

.module-btn.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  font-weight: 600;
}

.module-icon {
  font-size: 16px;
  width: 24px;
  text-align: center;
}

.module-label {
  white-space: nowrap;
}

.hub-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.module-panel {
  max-width: 900px;
  margin: 0 auto;
}

.module-panel > * {
  margin-bottom: 16px;
}

@media (max-width: 768px) {
  .hub-sidebar {
    width: 60px;
    padding: 8px;
  }

  .module-label {
    display: none;
  }

  .module-btn {
    justify-content: center;
    padding: 10px;
  }
}
</style>
