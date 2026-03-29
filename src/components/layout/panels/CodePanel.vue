<template>
  <div class="space-y-4">
    <!-- File Info -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">当前文件</h4>
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">文件名</span>
          <span class="text-[var(--text-primary)]">-</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">语言</span>
          <span class="text-[var(--text-primary)]">-</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">编码</span>
          <span class="text-[var(--text-primary)]">UTF-8</span>
        </div>
      </div>
    </div>

    <!-- Cursor Info -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">光标位置</h4>
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-1 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">行</span>
          <span class="text-[var(--text-primary)]">1</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">列</span>
          <span class="text-[var(--text-primary)]">1</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">缩进</span>
          <span class="text-[var(--text-primary)]">4 空格</span>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">快捷操作</h4>
      <div class="space-y-2">
        <button class="btn btn-secondary w-full text-left justify-start text-xs">
          <span class="mr-2">▶</span> 运行脚本
        </button>
        <button class="btn btn-secondary w-full text-left justify-start text-xs">
          <span class="mr-2">🐛</span> 调试
        </button>
        <button class="btn btn-secondary w-full text-left justify-start text-xs">
          <span class="mr-2">✨</span> 格式化代码
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Placeholder for code properties
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