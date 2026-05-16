/**
 * EnterpriseLibraryPanel.vue — V4.1-004 企业标准件库与检查清单
 */
<template>
  <div class="enterprise-library-panel">
    <div class="panel-header">
      <h3 class="panel-title">🏭 企业标准库</h3>
      <div class="tab-nav">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-btn"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >
          {{ tab.label }}
        </button>
      </div>
    </div>

    <!-- 标准件库 -->
    <div v-if="activeTab === 'parts'" class="tab-content">
      <div class="search-bar">
        <input v-model="searchQuery" type="text" placeholder="搜索标准件（名称/标准号/材料）">
        <span class="search-icon">🔍</span>
      </div>

      <div class="category-filters">
        <button
          v-for="cat in partCategories"
          :key="cat.key"
          class="filter-chip"
          :class="{ active: selectedCategory === cat.key }"
          @click="selectedCategory = selectedCategory === cat.key ? null : cat.key"
        >
          {{ cat.icon }} {{ cat.label }}
        </button>
      </div>

      <div class="parts-list">
        <div
          v-for="part in filteredParts"
          :key="part.id"
          class="part-card"
          @click="selectPart(part)"
        >
          <div class="part-icon">{{ part.icon || '📦' }}</div>
          <div class="part-info">
            <div class="part-name">{{ part.name }}</div>
            <div class="part-meta">
              <span class="part-standard">{{ part.standardCode }}</span>
              <span class="part-material">{{ part.material }}</span>
            </div>
          </div>
          <div class="part-specs">
            <span v-for="(val, key) in part.specs" :key="key" class="spec-tag">
              {{ key }}: {{ val }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 材料库 -->
    <div v-if="activeTab === 'materials'" class="tab-content">
      <div class="materials-list">
        <div
          v-for="mat in standardMaterials"
          :key="mat.name"
          class="material-card"
          @click="selectedMaterial = mat"
        >
          <div class="material-header">
            <span class="material-name">{{ mat.name }}</span>
            <span class="material-standard">{{ mat.standard }} {{ mat.grade }}</span>
          </div>
          <div class="material-props">
            <div class="prop">
              <span class="prop-label">密度</span>
              <span class="prop-value">{{ mat.density }} kg/m³</span>
            </div>
            <div class="prop">
              <span class="prop-label">弹性模量</span>
              <span class="prop-value">{{ (mat.elasticModulus / 1000).toFixed(0) }} GPa</span>
            </div>
            <div class="prop">
              <span class="prop-label">屈服强度</span>
              <span class="prop-value">{{ mat.yieldStrength }} MPa</span>
            </div>
            <div class="prop">
              <span class="prop-label">抗拉强度</span>
              <span class="prop-value">{{ mat.tensileStrength }} MPa</span>
            </div>
          </div>
          <div class="material-apps">
            <span v-for="app in mat.applications" :key="app" class="app-tag">{{ app }}</span>
          </div>
        </div>
      </div>

      <div v-if="selectedMaterial" class="material-detail">
        <h4>📋 {{ selectedMaterial.name }} 详细参数</h4>
        <table class="props-table">
          <tr><td>泊松比</td><td>{{ selectedMaterial.poissonRatio }}</td></tr>
          <tr><td>热膨胀系数</td><td>{{ selectedMaterial.thermalExpansion?.toExponential(1) }} /°C</td></tr>
          <tr><td>标准</td><td>{{ selectedMaterial.standard }}</td></tr>
          <tr><td>等级</td><td>{{ selectedMaterial.grade }}</td></tr>
        </table>
      </div>
    </div>

    <!-- 检查清单 -->
    <div v-if="activeTab === 'checklist'" class="tab-content">
      <div class="checklist-header">
        <select v-model="checklistType" class="type-select">
          <option value="structural">结构静力分析</option>
          <option value="fatigue">疲劳分析</option>
          <option value="thermal">热分析</option>
          <option value="modal">模态/动力学分析</option>
        </select>
        <div class="checklist-progress">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${checklistRate * 100}%` }" />
          </div>
          <span class="progress-text">{{ (checklistRate * 100).toFixed(0) }}%</span>
        </div>
      </div>

      <div class="checklist-items">
        <div
          v-for="item in checklistItems"
          :key="item.id"
          class="checklist-item"
          :class="{ required: item.required, checked: item.checked }"
        >
          <label class="item-row">
            <input v-model="item.checked" type="checkbox">
            <span class="item-badge" :class="item.category">{{ categoryLabel(item.category) }}</span>
            <span class="item-label">{{ item.label }}</span>
            <span v-if="item.required" class="required-mark">*</span>
          </label>
          <p class="item-desc">{{ item.description }}</p>
        </div>
      </div>

      <div v-if="!checklistValid.passed" class="checklist-warning">
        ⚠️ 还有 {{ checklistValid.missing.length }} 项必填检查未完成
      </div>
      <div v-else class="checklist-pass">
        ✅ 所有必填检查项已完成，可以提交仿真
      </div>
    </div>

    <!-- 设计规则 -->
    <div v-if="activeTab === 'rules'" class="tab-content">
      <div class="rules-list">
        <div
          v-for="rule in designRules"
          :key="rule.id"
          class="rule-card"
          :class="rule.severity"
        >
          <div class="rule-header">
            <span class="rule-name">{{ rule.name }}</span>
            <span class="rule-severity" :class="rule.severity">{{ severityLabel(rule.severity) }}</span>
          </div>
          <div class="rule-condition">
            {{ rule.condition }} {{ rule.operator }} {{ rule.threshold }} {{ rule.unit }}
          </div>
          <div class="rule-category">{{ rule.category }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useEnterpriseLibrary } from '@/composables/useEnterpriseLibrary'

const {
  allParts,
  standardMaterials,
  designRules,
  searchParts,
  createChecklist,
  validateChecklist
} = useEnterpriseLibrary()

const activeTab = ref('parts')
const searchQuery = ref('')
const selectedCategory = ref<string | null>(null)
const selectedMaterial = ref<any>(null)
const checklistType = ref('structural')
const checklistItems = ref(createChecklist('structural'))

const tabs = [
  { id: 'parts', label: '标准件' },
  { id: 'materials', label: '材料库' },
  { id: 'checklist', label: '检查清单' },
  { id: 'rules', label: '设计规则' }
]

const partCategories = [
  { key: 'fastener', label: '紧固件', icon: '🔩' },
  { key: 'structural', label: '结构件', icon: '工' },
  { key: 'bearing', label: '轴承', icon: '◎' },
  { key: 'seal', label: '密封件', icon: '⭕' }
]

const filteredParts = computed(() => {
  let list = searchQuery.value ? searchParts(searchQuery.value) : allParts.value
  if (selectedCategory.value) {
    list = list.filter(p => p.category === selectedCategory.value)
  }
  return list
})

const checklistValid = computed(() => validateChecklist(checklistItems.value))
const checklistRate = computed(() => checklistValid.value.rate)

watch(checklistType, (type) => {
  checklistItems.value = createChecklist(type)
})

function categoryLabel(cat: string): string {
  const map: Record<string, string> = {
    geometry: '几何',
    material: '材料',
    mesh: '网格',
    boundary: '边界',
    solver: '求解',
    post: '后处理'
  }
  return map[cat] || cat
}

function severityLabel(s: string): string {
  const map: Record<string, string> = {
    critical: '关键',
    warning: '警告',
    info: '提示'
  }
  return map[s] || s
}

function selectPart(part: any) {
  // Could emit event or open detail modal
  console.log('Selected part:', part)
}
</script>

<style scoped>
.enterprise-library-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px;
  background: white;
  border-radius: 12px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 12px;
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
  margin: 0;
}

.tab-nav {
  display: flex;
  gap: 4px;
}

.tab-btn {
  padding: 6px 14px;
  border-radius: 6px;
  border: none;
  background: transparent;
  font-size: 13px;
  color: var(--text-secondary, #64748b);
  cursor: pointer;
  transition: all 0.2s;
}

.tab-btn:hover {
  background: var(--bg-elevated, #f1f5f9);
}

.tab-btn.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  font-weight: 600;
}

.tab-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* Parts */
.search-bar {
  position: relative;
}

.search-bar input {
  width: 100%;
  padding: 8px 32px 8px 12px;
  border: 1px solid var(--border-color, #e2e8f0);
  border-radius: 8px;
  font-size: 13px;
}

.search-icon {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 14px;
}

.category-filters {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.filter-chip {
  padding: 4px 12px;
  border-radius: 16px;
  border: 1px solid var(--border-color, #e2e8f0);
  background: white;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.filter-chip.active {
  background: var(--primary-color, #3b82f6);
  color: white;
  border-color: var(--primary-color, #3b82f6);
}

.parts-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 400px;
  overflow-y: auto;
}

.part-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  cursor: pointer;
  transition: all 0.2s;
}

.part-card:hover {
  background: var(--bg-elevated, #f8fafc);
  border-color: var(--primary-color, #3b82f6);
}

.part-icon {
  font-size: 24px;
  width: 40px;
  text-align: center;
}

.part-info {
  flex: 1;
}

.part-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.part-meta {
  display: flex;
  gap: 8px;
  margin-top: 2px;
}

.part-standard {
  font-size: 11px;
  color: var(--primary-color, #3b82f6);
  background: #eff6ff;
  padding: 1px 6px;
  border-radius: 4px;
}

.part-material {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.part-specs {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  max-width: 200px;
}

.spec-tag {
  font-size: 10px;
  color: var(--text-secondary, #64748b);
  background: var(--bg-elevated, #f1f5f9);
  padding: 2px 6px;
  border-radius: 4px;
}

/* Materials */
.materials-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 12px;
}

.material-card {
  padding: 14px;
  border-radius: 10px;
  border: 1px solid var(--border-color, #e2e8f0);
  cursor: pointer;
  transition: all 0.2s;
}

.material-card:hover {
  border-color: var(--primary-color, #3b82f6);
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
}

.material-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.material-name {
  font-size: 14px;
  font-weight: 600;
}

.material-standard {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
}

.material-props {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 6px;
  margin-bottom: 10px;
}

.prop {
  display: flex;
  flex-direction: column;
}

.prop-label {
  font-size: 10px;
  color: var(--text-muted, #94a3b8);
}

.prop-value {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary, #1e293b);
}

.material-apps {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.app-tag {
  font-size: 10px;
  padding: 2px 8px;
  background: #f0fdf4;
  color: #166534;
  border-radius: 10px;
}

.material-detail {
  margin-top: 12px;
  padding: 14px;
  background: var(--bg-elevated, #f8fafc);
  border-radius: 10px;
}

.material-detail h4 {
  margin: 0 0 10px 0;
  font-size: 13px;
}

.props-table {
  width: 100%;
  font-size: 12px;
  border-collapse: collapse;
}

.props-table td {
  padding: 6px;
  border-bottom: 1px solid var(--border-color, #e2e8f0);
}

.props-table td:first-child {
  color: var(--text-secondary, #64748b);
  width: 120px;
}

/* Checklist */
.checklist-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.type-select {
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px solid var(--border-color, #e2e8f0);
  font-size: 13px;
  background: white;
}

.checklist-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.progress-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-elevated, #f1f5f9);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: #22c55e;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary, #64748b);
  min-width: 36px;
}

.checklist-items {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 400px;
  overflow-y: auto;
}

.checklist-item {
  padding: 10px 12px;
  border-radius: 8px;
  border: 1px solid var(--border-color, #e2e8f0);
  transition: all 0.2s;
}

.checklist-item.checked {
  background: #f0fdf4;
  border-color: #bbf7d0;
}

.item-row {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.item-row input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: #22c55e;
}

.item-badge {
  font-size: 10px;
  padding: 2px 8px;
  border-radius: 10px;
  font-weight: 600;
  white-space: nowrap;
}

.item-badge.geometry { background: #eff6ff; color: #1e40af; }
.item-badge.material { background: #f0fdf4; color: #166534; }
.item-badge.mesh { background: #fef3c7; color: #92400e; }
.item-badge.boundary { background: #f3e8ff; color: #6b21a8; }
.item-badge.solver { background: #fee2e2; color: #991b1b; }
.item-badge.post { background: #f1f5f9; color: #475569; }

.item-label {
  font-size: 13px;
  color: var(--text-primary, #1e293b);
}

.required-mark {
  color: #ef4444;
  font-weight: 700;
}

.item-desc {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  margin: 4px 0 0 24px;
  line-height: 1.4;
}

.checklist-warning {
  padding: 10px 14px;
  background: #fef3c7;
  color: #92400e;
  border-radius: 8px;
  font-size: 13px;
}

.checklist-pass {
  padding: 10px 14px;
  background: #dcfce7;
  color: #166534;
  border-radius: 8px;
  font-size: 13px;
}

/* Rules */
.rules-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.rule-card {
  padding: 12px;
  border-radius: 8px;
  border-left: 4px solid var(--border-color, #e2e8f0);
  background: var(--bg-elevated, #f8fafc);
}

.rule-card.critical { border-left-color: #ef4444; background: #fef2f2; }
.rule-card.warning { border-left-color: #f59e0b; background: #fffbeb; }
.rule-card.info { border-left-color: #3b82f6; background: #eff6ff; }

.rule-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.rule-name {
  font-size: 13px;
  font-weight: 600;
}

.rule-severity {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  font-weight: 600;
}

.rule-severity.critical { background: #fee2e2; color: #991b1b; }
.rule-severity.warning { background: #fef3c7; color: #92400e; }
.rule-severity.info { background: #dbeafe; color: #1e40af; }

.rule-condition {
  font-size: 12px;
  color: var(--text-secondary, #64748b);
  font-family: monospace;
}

.rule-category {
  font-size: 11px;
  color: var(--text-muted, #94a3b8);
  margin-top: 4px;
}
</style>
