<template>
  <div class="automation-tabs">
    <!-- Tab 切换 -->
    <div class="tabs-header">
      <button 
        v-for="tab in tabs" 
        :key="tab.id"
        @click="activeTab = tab.id"
        class="tab-btn"
        :class="{ 'tab-active': activeTab === tab.id }"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-label">{{ tab.label }}</span>
      </button>
    </div>

    <!-- Tab 内容 -->
    <div class="tabs-content">
      <!-- 录制面板 -->
      <div v-show="activeTab === 'record'" class="tab-panel">
        <ScriptRecorder />
      </div>

      <!-- 编辑面板 -->
      <div v-show="activeTab === 'edit'" class="tab-panel">
        <ScriptEditor />
      </div>

      <!-- 回放面板 -->
      <div v-show="activeTab === 'playback'" class="tab-panel">
        <ScriptPlayer />
      </div>

      <!-- 管理面板 -->
      <div v-show="activeTab === 'manage'" class="tab-panel">
        <ScriptManagerList />
      </div>

      <!-- 模板面板 -->
      <div v-show="activeTab === 'templates'" class="tab-panel">
        <ScriptTemplatesList />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ScriptRecorder from './ScriptRecorder.vue'
import ScriptEditor from './ScriptEditor.vue'
import ScriptPlayer from './ScriptPlayer.vue'
import ScriptManagerList from './ScriptManagerList.vue'
import ScriptTemplatesList from './ScriptTemplatesList.vue'

// Tab 定义
interface Tab {
  id: string
  label: string
  icon: string
}

const tabs: Tab[] = [
  { id: 'record', label: '录制', icon: '⏺' },
  { id: 'edit', label: '编辑', icon: '✏️' },
  { id: 'playback', label: '回放', icon: '▶' },
  { id: 'manage', label: '管理', icon: '📁' },
  { id: 'templates', label: '模板', icon: '📋' }
]

// 当前激活的 Tab
const activeTab = ref<string>('record')
</script>

<style scoped>
.automation-tabs {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: white;
  border-radius: 8px;
  overflow: hidden;
}

.tabs-header {
  display: flex;
  background: var(--bg-elevated);
  border-bottom: 1px solid var(--border-subtle);
  padding: 4px;
  gap: 4px;
}

.tab-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 8px 12px;
  font-size: 11px;
  color: var(--text-secondary);
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-btn:hover {
  background: white;
  color: var(--text-primary);
}

.tab-btn.tab-active {
  background: white;
  color: var(--primary);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.tab-icon {
  font-size: 12px;
}

.tab-label {
  font-weight: 500;
}

.tabs-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.tab-panel {
  height: 100%;
}
</style>