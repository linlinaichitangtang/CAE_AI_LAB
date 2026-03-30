<template>
  <div class="script-recorder-panel">
    <!-- 录制控制栏 -->
    <div class="recorder-header">
      <div class="flex items-center gap-2">
        <!-- 录制状态指示 -->
        <div 
          class="w-3 h-3 rounded-full"
          :class="isRecording ? 'bg-red-500 animate-pulse' : 'bg-gray-300'"
        ></div>
        <span class="text-sm font-medium text-gray-700">
          {{ isRecording ? '录制中...' : recordingLabel }}
        </span>
      </div>
      
      <div class="flex items-center gap-2">
        <!-- 开始/停止录制按钮 -->
        <button 
          v-if="!isRecording"
          @click="startRecording"
          :disabled="!canRecord"
          class="px-3 py-1.5 text-sm bg-red-500 text-white rounded hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1"
        >
          <span class="rec-icon">⏺</span>
          <span>开始录制</span>
        </button>
        
        <button 
          v-else
          @click="stopRecording"
          class="px-3 py-1.5 text-sm bg-gray-600 text-white rounded hover:bg-gray-700 flex items-center gap-1"
        >
          <span class="rec-icon">⏹</span>
          <span>停止录制</span>
        </button>
        
        <!-- 取消录制 -->
        <button 
          v-if="isRecording"
          @click="cancelRecording"
          class="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
        >
          取消
        </button>
      </div>
    </div>

    <!-- 录制步骤计数 -->
    <div v-if="isRecording || recordedSteps.length > 0" class="recorded-steps-count">
      <span class="text-xs text-gray-500">
        已录制 {{ recordedSteps.length }} 个步骤
      </span>
      <div v-if="isRecording" class="flex items-center gap-1">
        <span class="text-xs text-green-600">● 实时</span>
      </div>
    </div>

    <!-- 录制步骤列表 -->
    <div v-if="recordedSteps.length > 0" class="recorded-steps-list">
      <div 
        v-for="(step, idx) in recordedSteps" 
        :key="step.id"
        class="recorded-step-item"
        :class="{ 'last-step': idx === recordedSteps.length - 1 }"
      >
        <div class="step-number">{{ idx + 1 }}</div>
        <div class="step-content">
          <div class="step-type-badge">{{ getStepTypeLabel(step.type) }}</div>
          <div class="step-action">{{ step.action }}</div>
          <div v-if="step.description" class="step-desc">{{ step.description }}</div>
        </div>
      </div>
    </div>

    <!-- 录制说明 -->
    <div v-if="!isRecording && recordedSteps.length === 0" class="recording-hint">
      <p class="text-xs text-gray-500">
        点击"开始录制"后，执行的操作将自动记录为脚本步骤
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAutomationStore, type ScriptStep } from '@/stores/automation'

// Store
const automationStore = useAutomationStore()

// 状态
const recordedSteps = ref<ScriptStep[]>([])

// 计算属性
const isRecording = computed(() => automationStore.isRecording)
const canRecord = computed(() => automationStore.canRecord)
const recordingLabel = computed(() => 
  isRecording.value ? '录制中' : '准备录制'
)

// 方法
function getStepTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    mesh: '网格',
    material: '材料',
    boundary: '边界',
    load: '荷载',
    solve: '求解',
    result: '结果',
    wait: '等待',
    custom: '自定义'
  }
  return labels[type] || type
}

function startRecording() {
  automationStore.startRecording()
  recordedSteps.value = []
}

function stopRecording() {
  // 弹出命名对话框
  const name = prompt('请输入脚本名称:', '新脚本')
  if (name) {
    const script = automationStore.stopRecording(name)
    if (script) {
      console.log('脚本创建成功:', script)
    }
  } else {
    // 默认名称
    const script = automationStore.stopRecording('新脚本')
  }
}

function cancelRecording() {
  automationStore.cancelRecording()
  recordedSteps.value = []
}

// 监听录制步骤
// 这里可以通过store或事件监听录制的步骤
</script>

<style scoped>
.script-recorder-panel {
  background: white;
  border-radius: 8px;
  padding: 12px;
}

.recorder-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.rec-icon {
  font-size: 10px;
}

.recorded-steps-count {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.recorded-steps-list {
  max-height: 200px;
  overflow-y: auto;
  padding: 8px 0;
}

.recorded-step-item {
  display: flex;
  gap: 8px;
  padding: 8px;
  border-radius: 4px;
  margin-bottom: 4px;
  background: var(--bg-elevated);
}

.recorded-step-item.last-step {
  border-left: 2px solid #ef4444;
  background: #fef2f2;
}

.step-number {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--primary);
  color: white;
  border-radius: 50%;
  font-size: 10px;
  font-weight: 500;
  flex-shrink: 0;
}

.step-content {
  flex: 1;
  min-width: 0;
}

.step-type-badge {
  display: inline-block;
  padding: 1px 6px;
  background: var(--bg-hover);
  border-radius: 3px;
  font-size: 10px;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.step-action {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.step-desc {
  font-size: 10px;
  color: var(--text-muted);
  margin-top: 2px;
}

.recording-hint {
  padding: 16px;
  text-align: center;
}
</style>