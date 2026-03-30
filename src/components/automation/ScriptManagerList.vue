<template>
  <div class="script-manager-list">
    <!-- 搜索和筛选 -->
    <div class="search-bar">
      <input 
        v-model="searchQuery"
        type="text"
        placeholder="搜索脚本..."
        class="search-input"
      />
      <select v-model="filterType" class="filter-select">
        <option value="all">全部类型</option>
        <option value="user">用户脚本</option>
        <option value="project">项目脚本</option>
      </select>
    </div>

    <!-- 脚本列表 -->
    <div v-if="filteredScripts.length > 0" class="script-list">
      <div 
        v-for="script in filteredScripts" 
        :key="script.id"
        class="script-card"
        @click="selectScript(script)"
      >
        <div class="script-card-header">
          <span class="script-name">{{ script.name }}</span>
          <span 
            class="script-badge"
            :class="script.isTemplate ? 'badge-template' : 'badge-user'"
          >
            {{ script.isTemplate ? '模板' : '脚本' }}
          </span>
        </div>
        
        <p v-if="script.description" class="script-desc">
          {{ script.description }}
        </p>
        
        <div class="script-meta">
          <span class="meta-item">
            {{ script.steps.length }} 个步骤
          </span>
          <span class="meta-item">
            {{ formatDate(script.updatedAt) }}
          </span>
        </div>
        
        <div class="script-actions">
          <button 
            @click.stop="editScript(script)"
            class="action-btn"
          >
            编辑
          </button>
          <button 
            @click.stop="runScript(script)"
            class="action-btn action-btn-primary"
          >
            运行
          </button>
          <button 
            @click.stop="deleteScriptConfirm(script)"
            class="action-btn action-btn-danger"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="empty-state">
      <p class="text-sm text-gray-500">暂无脚本</p>
      <button 
        @click="$emit('createScript')"
        class="create-btn"
      >
        创建新脚本
      </button>
    </div>

    <!-- 删除确认对话框 -->
    <div v-if="showDeleteConfirm" class="delete-confirm-overlay">
      <div class="delete-confirm-dialog">
        <h4 class="dialog-title">确认删除</h4>
        <p class="dialog-content">
          确定要删除脚本 "{{ scriptToDelete?.name }}" 吗？此操作不可恢复。
        </p>
        <div class="dialog-actions">
          <button 
            @click="showDeleteConfirm = false"
            class="cancel-btn"
          >
            取消
          </button>
          <button 
            @click="confirmDelete"
            class="confirm-delete-btn"
          >
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAutomationStore, type Script } from '@/stores/automation'

// Store
const automationStore = useAutomationStore()

// 状态
const searchQuery = ref('')
const filterType = ref<'all' | 'user' | 'project'>('all')
const showDeleteConfirm = ref(false)
const scriptToDelete = ref<Script | null>(null)

// 计算属性
const filteredScripts = computed(() => {
  let scripts = automationStore.scripts
  
  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    scripts = scripts.filter(s => 
      s.name.toLowerCase().includes(query) ||
      s.description?.toLowerCase().includes(query)
    )
  }
  
  // 类型过滤
  if (filterType.value === 'user') {
    scripts = scripts.filter(s => !s.isTemplate && !s.projectId)
  } else if (filterType.value === 'project') {
    scripts = scripts.filter(s => !!s.projectId)
  }
  
  // 按更新时间排序
  return scripts.sort((a, b) => 
    new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
  )
})

// 方法
function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function selectScript(script: Script) {
  automationStore.loadScript(script.id)
}

function editScript(script: Script) {
  automationStore.loadScript(script.id)
  // 触发编辑事件，后续可以跳转到编辑标签页
}

function runScript(script: Script) {
  automationStore.loadScript(script.id)
  // 触发运行事件，后续可以跳转到回放标签页
}

function deleteScriptConfirm(script: Script) {
  scriptToDelete.value = script
  showDeleteConfirm.value = true
}

function confirmDelete() {
  if (scriptToDelete.value) {
    automationStore.deleteScript(scriptToDelete.value.id)
  }
  showDeleteConfirm.value = false
  scriptToDelete.value = null
}

// 事件
defineEmits<{
  (e: 'createScript'): void
}>()
</script>

<style scoped>
.script-manager-list {
  padding: 12px;
}

.search-bar {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  font-size: 13px;
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
}

.search-input:focus {
  outline: none;
  border-color: var(--primary);
}

.filter-select {
  padding: 8px 12px;
  font-size: 13px;
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  background: white;
}

.script-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.script-card {
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: 8px;
  border: 1px solid var(--border-subtle);
  cursor: pointer;
  transition: all 0.2s;
}

.script-card:hover {
  border-color: var(--primary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.script-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.script-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.script-badge {
  padding: 2px 8px;
  font-size: 10px;
  border-radius: 10px;
}

.badge-template {
  background: #fef3c7;
  color: #b45309;
}

.badge-user {
  background: #dbeafe;
  color: #1d4ed8;
}

.script-desc {
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.script-meta {
  display: flex;
  gap: 12px;
  margin-bottom: 8px;
}

.meta-item {
  font-size: 11px;
  color: var(--text-muted);
}

.script-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  flex: 1;
  padding: 6px 12px;
  font-size: 11px;
  border: 1px solid var(--border-subtle);
  border-radius: 4px;
  background: white;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background: var(--bg-hover);
}

.action-btn-primary {
  border-color: var(--primary);
  background: var(--primary);
  color: white;
}

.action-btn-primary:hover {
  background: var(--primary-dark);
}

.action-btn-danger {
  border-color: #fee2e2;
  color: #dc2626;
}

.action-btn-danger:hover {
  background: #fee2e2;
}

.empty-state {
  padding: 24px;
  text-align: center;
}

.create-btn {
  margin-top: 12px;
  padding: 8px 16px;
  font-size: 13px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

.create-btn:hover {
  background: var(--primary-dark);
}

.delete-confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.delete-confirm-dialog {
  background: white;
  padding: 20px;
  border-radius: 8px;
  max-width: 300px;
}

.dialog-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 8px;
}

.dialog-content {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.cancel-btn {
  padding: 8px 16px;
  font-size: 13px;
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  background: white;
  cursor: pointer;
}

.confirm-delete-btn {
  padding: 8px 16px;
  font-size: 13px;
  background: #dc2626;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}
</style>