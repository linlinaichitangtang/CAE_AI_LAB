<template>
  <div class="space-y-4">
    <!-- Note Info -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">笔记信息</h4>
      <div class="space-y-2">
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase">标题</label>
          <input 
            v-model="noteTitle" 
            type="text" 
            class="input w-full mt-1"
            placeholder="笔记标题"
          />
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase">创建时间</label>
          <div class="text-sm text-[var(--text-primary)] mt-1">-</div>
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase">最后修改</label>
          <div class="text-sm text-[var(--text-primary)] mt-1">-</div>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">快捷操作</h4>
      <div class="space-y-2">
        <button class="btn btn-secondary w-full text-left justify-start">
          <span class="mr-2">📋</span> 复制为 Markdown
        </button>
        <button class="btn btn-secondary w-full text-left justify-start">
          <span class="mr-2">📤</span> 导出为 PDF
        </button>
        <button class="btn btn-secondary w-full text-left justify-start">
          <span class="mr-2">🔗</span> 生成分享链接
        </button>
      </div>
    </div>

    <!-- Statistics -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">统计</h4>
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2">
        <div class="flex justify-between text-xs">
          <span class="text-[var(--text-muted)]">字符数</span>
          <span class="text-[var(--text-primary)] font-medium">0</span>
        </div>
        <div class="flex justify-between text-xs">
          <span class="text-[var(--text-muted)]">图片数</span>
          <span class="text-[var(--text-primary)] font-medium">0</span>
        </div>
        <div class="flex justify-between text-xs">
          <span class="text-[var(--text-muted)]">公式数</span>
          <span class="text-[var(--text-primary)] font-medium">0</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const noteTitle = ref('')
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
</style>