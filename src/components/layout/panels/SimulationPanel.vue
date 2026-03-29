<template>
  <div class="space-y-4">
    <!-- Simulation Status -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">计算状态</h4>
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">状态</span>
          <span class="text-[var(--accent-green)]">就绪</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">耗时</span>
          <span class="text-[var(--text-primary)]">-</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">进度</span>
          <span class="text-[var(--text-primary)]">0%</span>
        </div>
      </div>
    </div>

    <!-- Display Settings -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">显示设置</h4>
      <div class="space-y-3">
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">颜色图例</label>
          <select class="input w-full text-xs">
            <option value="viridis">Viridis</option>
            <option value="plasma">Plasma</option>
            <option value="inferno">Inferno</option>
            <option value="jet">Jet</option>
            <option value="rainbow">Rainbow</option>
          </select>
        </div>
        <label class="flex items-center gap-2 cursor-pointer">
          <input type="checkbox" class="rounded" />
          <span class="text-xs text-[var(--text-primary)]">显示变形</span>
        </label>
        <label class="flex items-center gap-2 cursor-pointer">
          <input type="checkbox" class="rounded" />
          <span class="text-xs text-[var(--text-primary)]">显示网格</span>
        </label>
      </div>
    </div>

    <!-- Results Summary -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">结果摘要</h4>
      <div class="bg-[var(--bg-elevated)] rounded-lg p-3 space-y-2 text-xs">
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">节点数</span>
          <span class="text-[var(--text-primary)]">-</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">单元数</span>
          <span class="text-[var(--text-primary)]">-</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">最大应力</span>
          <span class="text-[var(--text-primary)]">- MPa</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[var(--text-muted)]">最大位移</span>
          <span class="text-[var(--text-primary)]">- mm</span>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="panel-section">
      <button class="btn btn-primary w-full">
        <span class="mr-2">▶</span> 运行仿真
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
// Placeholder for simulation properties
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