<template>
  <div v-if="visible" class="floating-ai-container">
    <!-- AI 助手悬浮按钮 -->
    <button
      class="floating-ai-btn"
      :class="{ 'has-notification': hasNotification }"
      @click="toggleAssistant"
      :title="`AI 助手 ${shortcutHint}`"
    >
      <svg v-if="!isExpanded" width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z" fill="currentColor"/>
      </svg>
      <svg v-else width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" fill="currentColor"/>
      </svg>
    </button>

    <!-- AI 快捷操作面板 -->
    <Transition name="slide-up">
      <div v-if="isExpanded" class="ai-quick-panel glass" @click.stop>
        <!-- 搜索栏 -->
        <div class="search-bar">
          <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none">
            <circle cx="11" cy="11" r="7" stroke="currentColor" stroke-width="2"/>
            <path d="M21 21l-4.35-4.35" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="问 AI 任何问题..."
            class="search-input"
            @keydown.enter="executeSearch"
          />
          <span v-if="searchQuery" class="kbd">⏎</span>
        </div>

        <!-- 上下文提示 -->
        <div v-if="contextHint" class="context-hint">
          <span class="hint-icon">💡</span>
          <span>{{ contextHint }}</span>
        </div>

        <!-- 标签切换 -->
        <div class="tab-bar">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            class="tab-btn"
            :class="{ active: activeTab === tab.id }"
            @click="activeTab = tab.id"
          >
            <span class="tab-icon">{{ tab.icon }}</span>
            <span class="tab-label">{{ tab.label }}</span>
          </button>
        </div>

        <!-- 内容区域 -->
        <div class="panel-content">
          <!-- 快捷操作 (默认) -->
          <div v-if="activeTab === 'quick'" class="quick-actions">
            <!-- 场景化快捷操作 -->
            <div class="action-section">
              <div class="section-title">📐 建模</div>
              <div class="action-grid">
                <button class="action-btn" @click="quickAction('create_geometry')">
                  <span class="action-icon">📦</span>
                  <span>创建几何体</span>
                </button>
                <button class="action-btn" @click="quickAction('set_material')">
                  <span class="action-icon">🧱</span>
                  <span>设置材料</span>
                </button>
                <button class="action-btn" @click="quickAction('apply_bc')">
                  <span class="action-icon">🔒</span>
                  <span>边界条件</span>
                </button>
                <button class="action-btn" @click="quickAction('generate_mesh')">
                  <span class="action-icon">🔧</span>
                  <span>生成网格</span>
                </button>
              </div>
            </div>

            <div class="action-section">
              <div class="section-title">⚡ 仿真</div>
              <div class="action-grid">
                <button class="action-btn" @click="quickAction('run_simulation')">
                  <span class="action-icon">▶️</span>
                  <span>运行仿真</span>
                </button>
                <button class="action-btn" @click="quickAction('stop_simulation')">
                  <span class="action-icon">⏹</span>
                  <span>停止仿真</span>
                </button>
                <button class="action-btn" @click="quickAction('check_convergence')">
                  <span class="action-icon">📊</span>
                  <span>检查收敛</span>
                </button>
                <button class="action-btn" @click="quickAction('optimize_params')">
                  <span class="action-icon">🎯</span>
                  <span>优化参数</span>
                </button>
              </div>
            </div>

            <div class="action-section">
              <div class="section-title">📈 后处理</div>
              <div class="action-grid">
                <button class="action-btn" @click="quickAction('analyze_results')">
                  <span class="action-icon">🔍</span>
                  <span>分析结果</span>
                </button>
                <button class="action-btn" @click="quickAction('generate_report')">
                  <span class="action-icon">📝</span>
                  <span>生成报告</span>
                </button>
                <button class="action-btn" @click="quickAction('export_data')">
                  <span class="action-icon">💾</span>
                  <span>导出数据</span>
                </button>
                <button class="action-btn" @click="quickAction('compare_runs')">
                  <span class="action-icon">⚖️</span>
                  <span>对比结果</span>
                </button>
              </div>
            </div>
          </div>

          <!-- AI 模板 -->
          <div v-if="activeTab === 'templates'" class="templates-list">
            <div class="template-category">
              <div class="section-title">🔬 仿真分析</div>
              <button class="template-item" @click="useTemplate('stress_analysis')">
                <span class="template-icon">⚙️</span>
                <div class="template-content">
                  <span class="template-name">静力学分析</span>
                  <span class="template-desc">分析应力分布和位移</span>
                </div>
              </button>
              <button class="template-item" @click="useTemplate('modal_analysis')">
                <span class="template-icon">📳</span>
                <div class="template-content">
                  <span class="template-name">模态分析</span>
                  <span class="template-desc">获取固有频率和振型</span>
                </div>
              </button>
              <button class="template-item" @click="useTemplate('thermal_analysis')">
                <span class="template-icon">🌡️</span>
                <div class="template-content">
                  <span class="template-name">热分析</span>
                  <span class="template-desc">稳态或瞬态热传导</span>
                </div>
              </button>
            </div>

            <div class="template-category">
              <div class="section-title">🤖 AI 智能体</div>
              <button class="template-item" @click="useTemplate('microstructure_analysis')">
                <span class="template-icon">🔬</span>
                <div class="template-content">
                  <span class="template-name">微观结构分析</span>
                  <span class="template-desc">SEM/TEM 图像特征提取</span>
                </div>
              </button>
              <button class="template-item" @click="useTemplate('surrogate_modeling')">
                <span class="template-icon">🧠</span>
                <div class="template-content">
                  <span class="template-name">代理模型</span>
                  <span class="template-desc">快速预测仿真结果</span>
                </div>
              </button>
              <button class="template-item" @click="useTemplate('active_learning')">
                <span class="template-icon">📈</span>
                <div class="template-content">
                  <span class="template-name">主动学习</span>
                  <span class="template-desc">智能选择样本学习</span>
                </div>
              </button>
            </div>
          </div>

          <!-- 智能建议 -->
          <div v-if="activeTab === 'suggestions'" class="suggestions-list">
            <div v-if="aiSuggestions.length === 0" class="empty-state">
              <span class="empty-icon">🤔</span>
              <span>根据当前上下文，AI 会给出建议</span>
            </div>
            <button
              v-for="(suggestion, index) in aiSuggestions"
              :key="index"
              class="suggestion-item"
              @click="useSuggestion(suggestion)"
            >
              <span class="suggestion-icon">{{ suggestion.icon }}</span>
              <div class="suggestion-content">
                <span class="suggestion-title">{{ suggestion.title }}</span>
                <span class="suggestion-desc">{{ suggestion.description }}</span>
              </div>
              <span class="suggestion-arrow">→</span>
            </button>
          </div>
        </div>

        <!-- 底部快捷键 -->
        <div class="shortcut-bar">
          <span class="shortcut-hint">
            <span class="kbd">Ctrl</span><span>+</span><span class="kbd">K</span> 唤起
          </span>
          <span class="shortcut-hint">
            <span class="kbd">Tab</span> 切换
          </span>
          <span class="shortcut-hint">
            <span class="kbd">Esc</span> 关闭
          </span>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useProjectStore } from '@/stores/project'

interface Suggestion {
  icon: string
  title: string
  description: string
  action: string
}

interface Tab {
  id: 'quick' | 'templates' | 'suggestions'
  icon: string
  label: string
}

const router = useRouter()
const projectStore = useProjectStore()

const visible = ref(true)
const isExpanded = ref(false)
const hasNotification = ref(false)
const searchQuery = ref('')
const activeTab = ref<'quick' | 'templates' | 'suggestions'>('quick')

const tabs: Tab[] = [
  { id: 'quick', icon: '⚡', label: '快捷' },
  { id: 'templates', icon: '📋', label: '模板' },
  { id: 'suggestions', icon: '💡', label: '建议' },
]

const shortcutHint = computed(() => 'Ctrl+K')

// 根据当前上下文返回提示
const contextHint = computed(() => {
  if (projectStore.currentMesh) {
    const nodes = projectStore.currentMesh.nodes?.length || 0
    const elements = projectStore.currentMesh.elements?.length || 0
    return `已加载网格: ${nodes} 节点, ${elements} 单元`
  }
  if (projectStore.hasResult) {
    return '仿真结果已就绪，可询问 AI 分析'
  }
  if (projectStore.hasBoundaryConditions) {
    return '边界条件已设置，可运行仿真'
  }
  return '选择一个操作开始，或直接提问'
})

// 根据上下文生成 AI 建议
const aiSuggestions = computed<Suggestion[]>(() => {
  const suggestions: Suggestion[] = []

  if (!projectStore.currentMesh) {
    suggestions.push({
      icon: '📦',
      title: '创建悬臂梁模型',
      description: '快速创建演示用的悬臂梁几何',
      action: 'create_cantilever'
    })
  } else if (!projectStore.hasBoundaryConditions) {
    suggestions.push({
      icon: '🔒',
      title: '设置边界条件',
      description: 'AI 辅助设置固定端和载荷',
      action: 'setup_bc'
    })
  } else if (!projectStore.hasResult) {
    suggestions.push({
      icon: '▶️',
      title: '运行静力学分析',
      description: '开始 10 万自由度仿真',
      action: 'run_static'
    })
  } else {
    suggestions.push({
      icon: '📊',
      title: '分析仿真结果',
      description: '获取应力分布和安全评估',
      action: 'analyze_result'
    })
  }

  // 如果有结果，添加更多建议
  if (projectStore.hasResult) {
    suggestions.push({
      icon: '📝',
      title: '生成分析报告',
      description: '导出 PDF 格式的仿真报告',
      action: 'generate_report'
    })
    suggestions.push({
      icon: '⚖️',
      title: '对比历史结果',
      description: '与之前的仿真结果对比',
      action: 'compare_results'
    })
  }

  return suggestions.slice(0, 4)
})

function toggleAssistant() {
  if (isExpanded.value) {
    isExpanded.value = false
  } else {
    isExpanded.value = true
    hasNotification.value = false
  }
}

function quickAction(action: string) {
  console.log('AI Quick Action:', action)
  isExpanded.value = false

  switch (action) {
    case 'create_geometry':
      router.push('/ai?intent=create_geometry')
      break
    case 'set_material':
      router.push('/ai?intent=material')
      break
    case 'apply_bc':
      router.push('/ai?intent=boundary_condition')
      break
    case 'generate_mesh':
      router.push('/ai?intent=mesh')
      break
    case 'run_simulation':
      router.push('/ai?intent=run_simulation')
      break
    case 'stop_simulation':
      // 停止仿真
      break
    case 'analyze_results':
      router.push('/ai?intent=analyze')
      break
    case 'generate_report':
      router.push('/ai?intent=report')
      break
    default:
      router.push(`/ai?intent=${action}`)
  }
}

function useTemplate(template: string) {
  console.log('Use Template:', template)
  isExpanded.value = false
  router.push(`/ai?template=${template}`)
}

function useSuggestion(suggestion: Suggestion) {
  console.log('Use Suggestion:', suggestion.action)
  isExpanded.value = false
  router.push(`/ai?intent=${suggestion.action}`)
}

function executeSearch() {
  if (searchQuery.value.trim()) {
    console.log('Search:', searchQuery.value)
    router.push(`/ai?q=${encodeURIComponent(searchQuery.value)}`)
    searchQuery.value = ''
    isExpanded.value = false
  }
}

// 键盘快捷键
function handleKeydown(e: KeyboardEvent) {
  // Ctrl+K 唤起面板
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    isExpanded.value = !isExpanded.value
  }

  // Tab 切换标签
  if (isExpanded.value && e.key === 'Tab') {
    e.preventDefault()
    const currentIndex = tabs.findIndex(t => t.id === activeTab.value)
    const nextIndex = (currentIndex + 1) % tabs.length
    activeTab.value = tabs[nextIndex].id
  }

  // Escape 关闭
  if (e.key === 'Escape' && isExpanded.value) {
    isExpanded.value = false
  }
}

// 点击外部关闭
function handleClickOutside(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (!target.closest('.floating-ai-container')) {
    isExpanded.value = false
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.floating-ai-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 12px;
}

.floating-ai-btn {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--primary) 0%, var(--accent-cyan) 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 4px 20px rgba(37, 99, 235, 0.4);
  transition: all var(--duration-normal) var(--ease-spring);
  z-index: 9999;
  border: none;
}

.floating-ai-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 6px 28px rgba(37, 99, 235, 0.5);
}

.floating-ai-btn:active {
  transform: scale(0.95);
}

.floating-ai-btn.has-notification::after {
  content: '';
  position: absolute;
  top: 4px;
  right: 4px;
  width: 12px;
  height: 12px;
  background: var(--accent-red);
  border-radius: 50%;
  border: 2px solid white;
}

/* AI 快捷操作面板 */
.ai-quick-panel {
  width: 360px;
  max-height: 520px;
  overflow: hidden;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  display: flex;
  flex-direction: column;
}

/* 搜索栏 */
.search-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-subtle);
  background: var(--bg-elevated);
}

.search-icon {
  color: var(--text-muted);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 14px;
  color: var(--text-primary);
  outline: none;
}

.search-input::placeholder {
  color: var(--text-muted);
}

/* 上下文提示 */
.context-hint {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: var(--primary-glow);
  color: var(--primary);
  font-size: 12px;
}

.hint-icon {
  font-size: 14px;
}

/* 标签栏 */
.tab-bar {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.tab-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-size: 13px;
  color: var(--text-secondary);
  transition: all var(--duration-fast);
}

.tab-btn:hover {
  background: var(--bg-elevated);
}

.tab-btn.active {
  background: var(--primary-glow);
  color: var(--primary);
  font-weight: 500;
}

.tab-icon {
  font-size: 14px;
}

.tab-label {
  font-size: 13px;
}

/* 内容区域 */
.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

/* 快捷操作 */
.quick-actions {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.action-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.action-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 12px 8px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--duration-fast);
}

.action-btn:hover {
  background: var(--bg-elevated);
  border-color: var(--primary);
  transform: translateY(-2px);
}

.action-btn:active {
  transform: translateY(0);
}

.action-btn .action-icon {
  font-size: 20px;
}

.action-btn span:last-child {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 模板列表 */
.templates-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.template-category {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.template-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--duration-fast);
  text-align: left;
}

.template-item:hover {
  background: var(--bg-elevated);
  border-color: var(--primary);
}

.template-icon {
  font-size: 20px;
}

.template-content {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.template-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.template-desc {
  font-size: 11px;
  color: var(--text-muted);
}

/* 建议列表 */
.suggestions-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 24px;
  color: var(--text-muted);
  font-size: 13px;
}

.empty-icon {
  font-size: 32px;
  opacity: 0.5;
}

.suggestion-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--duration-fast);
  text-align: left;
}

.suggestion-item:hover {
  background: var(--bg-elevated);
  border-color: var(--primary);
}

.suggestion-icon {
  font-size: 20px;
}

.suggestion-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.suggestion-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.suggestion-desc {
  font-size: 11px;
  color: var(--text-muted);
}

.suggestion-arrow {
  color: var(--text-muted);
  font-size: 14px;
}

/* 底部快捷键栏 */
.shortcut-bar {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 10px;
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-elevated);
}

.shortcut-hint {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
  color: var(--text-muted);
}

.kbd {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 20px;
  height: 18px;
  padding: 0 4px;
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  border-radius: 4px;
  font-family: var(--font-code);
  font-size: 10px;
}

/* 动画 */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all var(--duration-normal) var(--ease-out);
}

.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>