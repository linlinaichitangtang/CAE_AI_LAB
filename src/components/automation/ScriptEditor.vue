<template>
  <div class="script-editor-panel">
    <!-- 脚本选择 -->
    <div class="script-selector">
      <div class="selector-header">
        <h4 class="text-sm font-medium text-gray-700">脚本列表</h4>
        <button 
          @click="createNewScript"
          class="text-xs text-blue-600 hover:text-blue-700"
        >
          + 新建脚本
        </button>
      </div>
      
      <select 
        v-model="selectedScriptId"
        @change="loadSelectedScript"
        class="w-full px-2 py-1.5 text-sm border rounded"
      >
        <option value="">请选择脚本...</option>
        <option 
          v-for="script in allScripts" 
          :key="script.id" 
          :value="script.id"
        >
          {{ script.name }}
        </option>
      </select>
    </div>

    <!-- 脚本信息 -->
    <div v-if="currentScript" class="script-info">
      <div class="info-header">
        <input 
          v-model="currentScript.name"
          @blur="saveScript"
          class="text-sm font-medium text-gray-800 bg-transparent border-b border-transparent hover:border-gray-300 focus:border-blue-500 focus:outline-none"
          placeholder="脚本名称"
        />
        <button 
          @click="duplicateScript"
          class="text-xs text-gray-500 hover:text-gray-700"
        >
          复制
        </button>
      </div>
      
      <textarea 
        v-model="currentScript.description"
        @blur="saveScript"
        class="w-full mt-2 px-2 py-1 text-xs border rounded resize-none"
        rows="2"
        placeholder="脚本描述..."
      />
    </div>

    <!-- 工具栏 -->
    <div v-if="currentScript" class="editor-toolbar">
      <button 
        @click="addStep"
        class="px-2 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700"
      >
        + 添加步骤
      </button>
      <button 
        @click="addLoop"
        class="px-2 py-1 text-xs bg-indigo-600 text-white rounded hover:bg-indigo-700"
      >
        + 添加循环
      </button>
      <button 
        @click="addVariable"
        class="px-2 py-1 text-xs bg-green-600 text-white rounded hover:bg-green-700"
      >
        + 添加变量
      </button>
      <button 
        @click="exportScript"
        class="px-2 py-1 text-xs border border-gray-300 rounded hover:bg-gray-50"
      >
        导出
      </button>
    </div>

    <!-- 变量列表 -->
    <div v-if="currentScript && Object.keys(currentScript.variables).length > 0" class="variables-section">
      <h5 class="section-title">变量</h5>
      <div class="variables-list">
        <div 
          v-for="(value, name) in currentScript.variables" 
          :key="name"
          class="variable-item"
        >
          <span class="var-name">{{ name }}</span>
          <input 
            v-model="currentScript.variables[name]"
            @change="saveScript"
            class="var-value"
          />
          <button 
            @click="deleteVariable(name as string)"
            class="var-delete"
          >
            ×
          </button>
        </div>
      </div>
    </div>

    <!-- 循环列表 -->
    <div v-if="currentScript && currentScript.loops.length > 0" class="loops-section">
      <h5 class="section-title">循环</h5>
      <div class="loops-list">
        <div 
          v-for="(loop, idx) in currentScript.loops" 
          :key="idx"
          class="loop-item"
        >
          <select 
            v-model="loop.type"
            @change="saveScript"
            class="loop-type"
          >
            <option value="repeat">重复</option>
            <option value="forEach">遍历</option>
            <option value="while">条件循环</option>
          </select>
          <input 
            v-model="loop.paramName"
            @change="saveScript"
            class="loop-param"
            placeholder="循环参数"
          />
          <button 
            @click="deleteLoop(idx)"
            class="loop-delete"
          >
            ×
          </button>
        </div>
      </div>
    </div>

    <!-- 步骤列表 -->
    <div v-if="currentScript" class="steps-section">
      <h5 class="section-title">
        步骤 ({{ currentScript.steps.length }})
      </h5>
      
      <div 
        v-if="currentScript.steps.length === 0" 
        class="empty-steps"
      >
        暂无步骤，点击"添加步骤"开始编辑
      </div>
      
      <div v-else class="steps-list">
        <div 
          v-for="(step, idx) in currentScript.steps" 
          :key="step.id"
          class="step-item"
          :class="{ 'step-disabled': !step.enabled }"
          draggable="true"
          @dragstart="dragStart(idx)"
          @dragover.prevent
          @drop="drop(idx)"
        >
          <!-- 步骤序号 -->
          <div class="step-order">{{ idx + 1 }}</div>
          
          <!-- 步骤内容 -->
          <div class="step-details">
            <!-- 类型和操作 -->
            <div class="step-header">
              <select 
                v-model="step.type"
                @change="saveScript"
                class="step-type"
              >
                <option value="mesh">网格</option>
                <option value="material">材料</option>
                <option value="boundary">边界</option>
                <option value="load">荷载</option>
                <option value="solve">求解</option>
                <option value="result">结果</option>
                <option value="wait">等待</option>
                <option value="custom">自定义</option>
              </select>
              <input 
                v-model="step.action"
                @change="saveScript"
                class="step-action"
                placeholder="操作名称"
              />
            </div>
            
            <!-- 描述 -->
            <input 
              v-model="step.description"
              @change="saveScript"
              class="step-desc-input"
              placeholder="步骤描述"
            />
            
            <!-- 参数编辑器 -->
            <div class="step-params">
              <div 
                v-for="(paramValue, paramKey) in step.params" 
                :key="paramKey"
                class="param-row"
              >
                <span class="param-key">{{ paramKey }}</span>
                <input 
                  v-model="step.params[paramKey]"
                  @change="saveScript"
                  class="param-value"
                  :placeholder="String(paramValue)"
                />
              </div>
              
              <!-- 添加参数按钮 -->
              <button 
                @click="showAddParam(step)"
                class="add-param-btn"
              >
                + 添加参数
              </button>
            </div>
            
            <!-- 启用/禁用 -->
            <div class="step-footer">
              <label class="step-enable">
                <input 
                  v-model="step.enabled"
                  @change="saveScript"
                  type="checkbox"
                />
                <span>启用</span>
              </label>
              
              <button 
                @click="deleteStep(idx)"
                class="step-delete-btn"
              >
                删除
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-if="!currentScript" class="empty-state">
      <p class="text-sm text-gray-500">请选择一个脚本进行编辑</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAutomationStore, type Script, type ScriptStep, type ScriptLoop } from '@/stores/automation'

// Store
const automationStore = useAutomationStore()

// 状态
const selectedScriptId = ref<string>('')
const draggedIndex = ref<number | null>(null)

// 计算属性
const currentScript = computed(() => automationStore.currentScript)
const allScripts = computed(() => [
  ...automationStore.builtInTemplates,
  ...automationStore.scripts
])

// 生命周期
onMounted(() => {
  automationStore.initBuiltInTemplates()
})

// 方法
function loadSelectedScript() {
  if (selectedScriptId.value) {
    automationStore.loadScript(selectedScriptId.value)
  }
}

function createNewScript() {
  const name = prompt('请输入脚本名称:')
  if (name) {
    automationStore.createScript(name)
    if (automationStore.currentScript) {
      selectedScriptId.value = automationStore.currentScript.id
    }
  }
}

function saveScript() {
  automationStore.saveCurrentScript()
}

function duplicateScript() {
  if (currentScript.value) {
    const copy = automationStore.duplicateScript(currentScript.value.id)
    if (copy) {
      selectedScriptId.value = copy.id
    }
  }
}

function exportScript() {
  if (currentScript.value) {
    const json = automationStore.exportScript(currentScript.value.id)
    if (json) {
      // 创建下载
      const blob = new Blob([json], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `${currentScript.value.name}.json`
      a.click()
      URL.revokeObjectURL(url)
    }
  }
}

function addStep() {
  automationStore.addStep({
    type: 'mesh',
    action: 'generate_mesh',
    params: {},
    description: '',
    enabled: true
  })
  saveScript()
}

function deleteStep(index: number) {
  if (currentScript.value) {
    const step = currentScript.value.steps[index]
    automationStore.deleteStep(step.id)
    saveScript()
  }
}

function addLoop() {
  automationStore.addLoop({
    type: 'repeat',
    paramName: 'i',
    start: 1,
    end: 10,
    step: 1
  })
  saveScript()
}

function deleteLoop(index: number) {
  automationStore.deleteLoop(index)
  saveScript()
}

function addVariable() {
  const name = prompt('请输入变量名:')
  if (name) {
    const valueStr = prompt('请输入变量值:', '100')
    const value = isNaN(Number(valueStr)) ? valueStr : Number(valueStr)
    if (value) {
      automationStore.setVariable(name, value)
      saveScript()
    }
  }
}

function deleteVariable(name: string) {
  automationStore.deleteVariable(name)
  saveScript()
}

function showAddParam(step: ScriptStep) {
  const key = prompt('请输入参数名:')
  if (key) {
    const value = prompt('请输入参数值:', '')
    step.params[key] = value
    saveScript()
  }
}

// 拖拽排序
function dragStart(index: number) {
  draggedIndex.value = index
}

function drop(targetIndex: number) {
  if (draggedIndex.value !== null && draggedIndex.value !== targetIndex) {
    automationStore.reorderSteps(draggedIndex.value, targetIndex)
    saveScript()
  }
  draggedIndex.value = null
}
</script>

<style scoped>
.script-editor-panel {
  background: white;
  border-radius: 8px;
  padding: 12px;
  max-height: 100%;
  overflow-y: auto;
}

.selector-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.script-info {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-subtle);
}

.info-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.editor-toolbar {
  display: flex;
  gap: 4px;
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--border-subtle);
  flex-wrap: wrap;
}

.section-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  margin: 12px 0 8px;
}

.variables-list,
.loops-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.variable-item,
.loop-item {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px;
  background: var(--bg-elevated);
  border-radius: 4px;
}

.var-name,
.loop-type,
.loop-param {
  padding: 2px 4px;
  font-size: 11px;
  border: 1px solid var(--border-subtle);
  border-radius: 3px;
}

.var-value {
  flex: 1;
  padding: 2px 4px;
  font-size: 11px;
  border: 1px solid var(--border-subtle);
  border-radius: 3px;
}

.var-delete,
.loop-delete {
  padding: 2px 6px;
  font-size: 12px;
  color: var(--text-muted);
  cursor: pointer;
}

.var-delete:hover,
.loop-delete:hover {
  color: #ef4444;
}

.empty-steps {
  padding: 16px;
  text-align: center;
  font-size: 12px;
  color: var(--text-muted);
  background: var(--bg-elevated);
  border-radius: 4px;
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.step-item {
  display: flex;
  gap: 8px;
  padding: 8px;
  background: var(--bg-elevated);
  border-radius: 6px;
  border: 1px solid var(--border-subtle);
}

.step-item.step-disabled {
  opacity: 0.5;
}

.step-order {
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

.step-details {
  flex: 1;
  min-width: 0;
}

.step-header {
  display: flex;
  gap: 4px;
  margin-bottom: 4px;
}

.step-type {
  padding: 2px 4px;
  font-size: 10px;
  border: 1px solid var(--border-subtle);
  border-radius: 3px;
  background: white;
}

.step-action {
  flex: 1;
  padding: 2px 4px;
  font-size: 11px;
  border: 1px solid var(--border-subtle);
  border-radius: 3px;
}

.step-desc-input {
  width: 100%;
  padding: 2px 4px;
  font-size: 10px;
  border: 1px solid var(--border-subtle);
  border-radius: 3px;
  margin-bottom: 4px;
}

.step-params {
  background: white;
  border-radius: 4px;
  padding: 4px;
  margin-bottom: 4px;
}

.param-row {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  margin-bottom: 2px;
}

.param-key {
  color: var(--text-secondary);
  min-width: 60px;
}

.param-value {
  flex: 1;
  padding: 2px 4px;
  font-size: 10px;
  border: 1px solid var(--border-subtle);
  border-radius: 3px;
}

.add-param-btn {
  font-size: 10px;
  color: var(--primary);
  cursor: pointer;
}

.add-param-btn:hover {
  text-decoration: underline;
}

.step-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.step-enable {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  color: var(--text-secondary);
}

.step-delete-btn {
  font-size: 10px;
  color: #ef4444;
  cursor: pointer;
}

.step-delete-btn:hover {
  text-decoration: underline;
}

.empty-state {
  padding: 24px;
  text-align: center;
}
</style>