<template>
  <div class="space-y-3">
    <!-- Analysis Type -->
    <div class="panel-section">
      <div class="text-[10px] font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2">分析类型</div>
      <div class="analysis-tabs">
        <button 
          v-for="type in analysisTypes" 
          :key="type.id"
          class="analysis-tab"
          :class="{ 'active': activeAnalysis === type.id }"
          @click="activeAnalysis = type.id"
        >
          {{ type.label }}
        </button>
      </div>
    </div>

    <!-- Material Properties -->
    <div class="panel-section">
      <div class="collapse-panel" :class="{ 'expanded': expandedSections.material }">
        <div class="collapse-header" @click="toggleSection('material')">
          <div class="flex items-center gap-2">
            <svg class="w-4 h-4 text-[var(--text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
            </svg>
            <span class="text-xs font-medium text-[var(--text-primary)]">材料属性</span>
          </div>
          <svg class="w-4 h-4 text-[var(--text-muted)] collapse-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="6 9 12 15 18 9"/>
          </svg>
        </div>
        <div v-show="expandedSections.material" class="collapse-content">
          <div class="space-y-3">
            <!-- Material Selector -->
            <div>
              <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1.5 block">材料</label>
              <div class="flex gap-2">
                <select v-model="material.name" class="input flex-1 text-xs">
                  <option value="6061">铝合金 6061</option>
                  <option value="304">不锈钢 304</option>
                  <option value="titanium">钛合金 Ti-6Al-4V</option>
                  <option value="custom">自定义...</option>
                </select>
                <button class="btn btn-ghost text-xs px-2">📋</button>
              </div>
            </div>
            
            <!-- Material Properties Grid -->
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] mb-1 block">弹性模量 (GPa)</label>
                <input v-model="material.young" type="number" class="input text-xs" step="0.1">
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] mb-1 block">泊松比</label>
                <input v-model="material.poisson" type="number" class="input text-xs" step="0.01">
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] mb-1 block">密度 (g/cm³)</label>
                <input v-model="material.density" type="number" class="input text-xs" step="0.01">
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] mb-1 block">屈服强度 (MPa)</label>
                <input v-model="material.yield" type="number" class="input text-xs">
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Boundary Conditions -->
    <div class="panel-section">
      <div class="collapse-panel" :class="{ 'expanded': expandedSections.boundary }">
        <div class="collapse-header" @click="toggleSection('boundary')">
          <div class="flex items-center gap-2">
            <svg class="w-4 h-4 text-[var(--text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="3" y="3" width="18" height="18" rx="2"/>
              <path d="M3 9h18M9 21V9"/>
            </svg>
            <span class="text-xs font-medium text-[var(--text-primary)]">边界条件</span>
          </div>
          <svg class="w-4 h-4 text-[var(--text-muted)] collapse-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="6 9 12 15 18 9"/>
          </svg>
        </div>
        <div v-show="expandedSections.boundary" class="collapse-content">
          <div class="space-y-3">
            <!-- Fixed Support -->
            <div>
              <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1.5 block">固定约束</label>
              <div class="flex flex-wrap gap-1.5">
                <button 
                  v-for="face in ['底面', '侧面', '顶面', '自定义']" 
                  :key="face"
                  class="boundary-tag"
                  :class="{ 'active': boundary.fixed.includes(face) }"
                  @click="toggleBoundary('fixed', face)"
                >
                  {{ face }}
                </button>
              </div>
            </div>
            
            <!-- Load Type -->
            <div>
              <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1.5 block">荷载类型</label>
              <select v-model="boundary.loadType" class="input w-full text-xs">
                <option value="force">集中力</option>
                <option value="pressure">分布压力</option>
                <option value="gravity">重力</option>
                <option value="thermal">热荷载</option>
              </select>
            </div>
            
            <!-- Load Value -->
            <div>
              <label class="text-[10px] text-[var(--text-muted)] mb-1 block">荷载值</label>
              <div class="input-with-unit">
                <input v-model="boundary.loadValue" type="number" class="input text-xs">
                <span class="input-unit">N</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Mesh Settings -->
    <div class="panel-section">
      <div class="collapse-panel" :class="{ 'expanded': expandedSections.mesh }">
        <div class="collapse-header" @click="toggleSection('mesh')">
          <div class="flex items-center gap-2">
            <svg class="w-4 h-4 text-[var(--text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 2L2 7l10 5 10-5-10-5z"/>
              <path d="M2 17l10 5 10-5"/>
              <path d="M2 12l10 5 10-5"/>
            </svg>
            <span class="text-xs font-medium text-[var(--text-primary)]">网格设置</span>
          </div>
          <svg class="w-4 h-4 text-[var(--text-muted)] collapse-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="6 9 12 15 18 9"/>
          </svg>
        </div>
        <div v-show="expandedSections.mesh" class="collapse-content">
          <div class="space-y-3">
            <div>
              <label class="text-[10px] text-[var(--text-muted)] mb-1.5 block">网格尺寸</label>
              <div class="flex items-center gap-2">
                <input v-model="mesh.size" type="range" class="slider flex-1">
                <span class="text-xs text-[var(--text-secondary)] w-12 text-right">{{ mesh.size }}mm</span>
              </div>
            </div>
            <div>
              <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1.5 block">网格类型</label>
              <select v-model="mesh.type" class="input w-full text-xs">
                <option value="free">自由网格</option>
                <option value="mapped">映射网格</option>
                <option value="tet">四面体</option>
                <option value="hex">六面体</option>
              </select>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Solver Settings -->
    <div class="panel-section">
      <div class="collapse-panel" :class="{ 'expanded': expandedSections.solver }">
        <div class="collapse-header" @click="toggleSection('solver')">
          <div class="flex items-center gap-2">
            <svg class="w-4 h-4 text-[var(--text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4"/>
            </svg>
            <span class="text-xs font-medium text-[var(--text-primary)]">求解器</span>
          </div>
          <svg class="w-4 h-4 text-[var(--text-muted)] collapse-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="6 9 12 15 18 9"/>
          </svg>
        </div>
        <div v-show="expandedSections.solver" class="collapse-content">
          <div class="space-y-3">
            <div>
              <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1.5 block">求解器类型</label>
              <select v-model="solver.type" class="input w-full text-xs">
                <option value="direct">直接求解器</option>
                <option value="iterative">迭代求解器</option>
                <option value="poco">POCO求解器</option>
              </select>
            </div>
            <div>
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="solver.autoTimeStep" class="rounded">
                <span class="text-xs text-[var(--text-primary)]">自动时间步</span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Results Summary -->
    <div class="panel-section">
      <div class="text-[10px] font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2">结果预览</div>
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">节点数</span>
          <span class="text-[var(--text-primary)] font-medium">12,845</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">单元数</span>
          <span class="text-[var(--text-primary)] font-medium">6,782</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">最大应力</span>
          <span class="text-[var(--accent-amber)] font-medium">— MPa</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">最大位移</span>
          <span class="text-[var(--text-primary)] font-medium">— mm</span>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-2">
      <button class="btn btn-primary flex-1 flex items-center justify-center gap-1.5">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor">
          <polygon points="5 3 19 12 5 21 5 3"/>
        </svg>
        运行
      </button>
      <button class="btn btn-secondary flex items-center justify-center gap-1.5">
        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <path d="M9 3v18M21 9H3"/>
        </svg>
        重置
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

const activeAnalysis = ref('static')

const analysisTypes = [
  { id: 'static', label: '静态' },
  { id: 'transient', label: '瞬态' },
  { id: 'modal', label: '模态' },
  { id: 'fatigue', label: '疲劳' },
  { id: 'thermal', label: '热耦合' }
]

const expandedSections = reactive({
  material: true,
  boundary: true,
  mesh: false,
  solver: false
})

const material = reactive({
  name: '6061',
  young: '68.9',
  poisson: '0.33',
  density: '2.70',
  yield: '276'
})

const boundary = reactive({
  fixed: ['底面'],
  loadType: 'force',
  loadValue: '1000'
})

const mesh = reactive({
  size: 2,
  type: 'tet'
})

const solver = reactive({
  type: 'direct',
  autoTimeStep: true
})

function toggleSection(section: keyof typeof expandedSections) {
  expandedSections[section] = !expandedSections[section]
}

function toggleBoundary(type: 'fixed', face: string) {
  const arr = boundary[type] as string[]
  const idx = arr.indexOf(face)
  if (idx >= 0) {
    arr.splice(idx, 1)
  } else {
    arr.push(face)
  }
}
</script>

<style scoped>
.panel-section {
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}
.panel-section:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.analysis-tabs {
  display: flex;
  gap: 2px;
  background: var(--bg-elevated);
  padding: 2px;
  border-radius: var(--radius-sm);
}
.analysis-tab {
  flex: 1;
  padding: 6px 8px;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast);
}
.analysis-tab:hover {
  color: var(--text-secondary);
}
.analysis-tab.active {
  background: var(--bg-surface);
  color: var(--primary);
  box-shadow: var(--shadow-sm);
}

.boundary-tag {
  padding: 4px 10px;
  font-size: 11px;
  color: var(--text-secondary);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-full);
  cursor: pointer;
  transition: all var(--duration-fast);
}
.boundary-tag:hover {
  border-color: var(--primary);
  color: var(--primary);
}
.boundary-tag.active {
  background: var(--primary-glow);
  border-color: var(--primary);
  color: var(--primary);
}
</style>
