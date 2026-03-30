<template>
  <div class="script-templates-list">
    <!-- 模板说明 -->
    <div class="templates-intro">
      <h4 class="intro-title">内置模板</h4>
      <p class="intro-desc">
        使用模板快速创建标准仿真流程，或基于模板修改为自定义脚本
      </p>
    </div>

    <!-- 模板列表 -->
    <div class="templates-grid">
      <div 
        v-for="template in templates" 
        :key="template.id"
        class="template-card"
        @click="previewTemplate(template)"
      >
        <div class="template-icon">{{ getTemplateIcon(template.id) }}</div>
        <div class="template-name">{{ template.name }}</div>
        <p class="template-desc">{{ template.description }}</p>
        
        <div class="template-meta">
          <span class="meta-badge">{{ template.steps.length }} 个步骤</span>
          <span v-if="template.loops.length > 0" class="meta-badge meta-badge-loop">
            含循环
          </span>
        </div>
        
        <div class="template-actions">
          <button 
            @click.stop="useTemplate(template)"
            class="use-template-btn"
          >
            使用模板
          </button>
        </div>
      </div>
    </div>

    <!-- 模板预览对话框 -->
    <div v-if="previewingTemplate" class="preview-overlay" @click="previewingTemplate = null">
      <div class="preview-dialog" @click.stop>
        <div class="preview-header">
          <h3 class="preview-title">{{ previewingTemplate.name }}</h3>
          <button class="close-btn" @click="previewingTemplate = null">×</button>
        </div>
        
        <p class="preview-desc">{{ previewingTemplate.description }}</p>
        
        <!-- 变量 -->
        <div v-if="Object.keys(previewingTemplate.variables).length > 0" class="preview-section">
          <h4 class="section-title">变量</h4>
          <div class="preview-vars">
            <div 
              v-for="(value, name) in previewingTemplate.variables" 
              :key="name"
              class="var-item"
            >
              <span class="var-name">{{ name }}</span>
              <span class="var-value">{{ value }}</span>
            </div>
          </div>
        </div>
        
        <!-- 步骤 -->
        <div class="preview-section">
          <h4 class="section-title">步骤</h4>
          <div class="preview-steps">
            <div 
              v-for="(step, idx) in previewingTemplate.steps" 
              :key="step.id"
              class="step-item"
            >
              <div class="step-num">{{ idx + 1 }}</div>
              <div class="step-content">
                <div class="step-type">{{ step.type }}</div>
                <div class="step-action">{{ step.action }}</div>
                <div class="step-desc">{{ step.description }}</div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 操作按钮 -->
        <div class="preview-actions">
          <button 
            class="cancel-btn"
            @click="previewingTemplate = null"
          >
            关闭
          </button>
          <button 
            class="use-btn"
            @click="useTemplate(previewingTemplate)"
          >
            使用模板
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
const previewingTemplate = ref<Script | null>(null)

// 计算属性
const templates = computed(() => automationStore.builtInTemplates)

// 方法
function getTemplateIcon(templateId: string): string {
  const icons: Record<string, string> = {
    'template-static': '📊',
    'template-parametric': '📈',
    'template-modal': '🌊'
  }
  return icons[templateId] || '📋'
}

function previewTemplate(template: Script) {
  previewingTemplate.value = template
}

function useTemplate(template: Script) {
  // 复制模板为新脚本
  const copy = automationStore.importScript(JSON.stringify({
    ...template,
    id: undefined,
    createdAt: undefined,
    updatedAt: undefined,
    isTemplate: false
  }))
  
  if (copy) {
    automationStore.loadScript(copy.id)
    // 跳转到编辑页面（通过事件或store）
  }
  
  previewingTemplate.value = null
}
</script>

<style scoped>
.script-templates-list {
  padding: 12px;
}

.templates-intro {
  margin-bottom: 16px;
}

.intro-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.intro-desc {
  font-size: 12px;
  color: var(--text-muted);
}

.templates-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.template-card {
  padding: 16px;
  background: var(--bg-elevated);
  border-radius: 8px;
  border: 1px solid var(--border-subtle);
  cursor: pointer;
  transition: all 0.2s;
}

.template-card:hover {
  border-color: var(--primary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.template-icon {
  font-size: 24px;
  margin-bottom: 8px;
}

.template-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.template-desc {
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 12px;
}

.template-meta {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.meta-badge {
  padding: 2px 8px;
  background: white;
  border: 1px solid var(--border-subtle);
  border-radius: 10px;
  font-size: 10px;
  color: var(--text-secondary);
}

.meta-badge-loop {
  background: #fef3c7;
  border-color: #fcd34d;
  color: #b45309;
}

.template-actions {
  display: flex;
  gap: 8px;
}

.use-template-btn {
  flex: 1;
  padding: 8px 16px;
  font-size: 12px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.use-template-btn:hover {
  background: var(--primary-dark);
}

/* 预览对话框 */
.preview-overlay {
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
  padding: 20px;
}

.preview-dialog {
  background: white;
  border-radius: 12px;
  max-width: 500px;
  width: 100%;
  max-height: 80vh;
  overflow-y: auto;
  padding: 20px;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.preview-title {
  font-size: 18px;
  font-weight: 600;
}

.close-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  background: var(--bg-elevated);
  border: none;
  border-radius: 50%;
  cursor: pointer;
}

.preview-desc {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.preview-section {
  margin-bottom: 16px;
}

.section-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.preview-vars {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.var-item {
  display: flex;
  gap: 8px;
  padding: 4px 8px;
  background: var(--bg-elevated);
  border-radius: 4px;
  font-size: 11px;
}

.var-name {
  color: var(--text-secondary);
}

.var-value {
  color: var(--text-primary);
  font-weight: 500;
}

.preview-steps {
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
}

.step-num {
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

.step-type {
  display: inline-block;
  padding: 1px 6px;
  background: white;
  border-radius: 3px;
  font-size: 9px;
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

.preview-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid var(--border-subtle);
}

.cancel-btn {
  padding: 8px 16px;
  font-size: 13px;
  border: 1px solid var(--border-subtle);
  border-radius: 6px;
  background: white;
  cursor: pointer;
}

.use-btn {
  padding: 8px 16px;
  font-size: 13px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}
</style>