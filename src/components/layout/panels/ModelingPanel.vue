<template>
  <div class="space-y-4">
    <!-- Object Properties -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">变换</h4>
      <div class="space-y-3">
        <!-- Position -->
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-2 block">位置</label>
          <div class="grid grid-cols-3 gap-2">
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">X</span>
              <input type="number" value="0" class="input w-full text-center text-sm mt-1" />
            </div>
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">Y</span>
              <input type="number" value="0" class="input w-full text-center text-sm mt-1" />
            </div>
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">Z</span>
              <input type="number" value="0" class="input w-full text-center text-sm mt-1" />
            </div>
          </div>
        </div>
        
        <!-- Rotation -->
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-2 block">旋转</label>
          <div class="grid grid-cols-3 gap-2">
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">X</span>
              <input type="number" value="0" class="input w-full text-center text-sm mt-1" />
            </div>
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">Y</span>
              <input type="number" value="0" class="input w-full text-center text-sm mt-1" />
            </div>
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">Z</span>
              <input type="number" value="0" class="input w-full text-center text-sm mt-1" />
            </div>
          </div>
        </div>
        
        <!-- Scale -->
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-2 block">缩放</label>
          <div class="grid grid-cols-3 gap-2">
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">X</span>
              <input type="number" value="1" step="0.1" class="input w-full text-center text-sm mt-1" />
            </div>
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">Y</span>
              <input type="number" value="1" step="0.1" class="input w-full text-center text-sm mt-1" />
            </div>
            <div class="text-center">
              <span class="text-[10px] text-[var(--text-muted)]">Z</span>
              <input type="number" value="1" step="0.1" class="input w-full text-center text-sm mt-1" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Material -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">材质</h4>
      <div class="space-y-3">
        <div class="flex gap-2">
          <button class="btn btn-ghost flex-1 text-xs">金属</button>
          <button class="btn btn-ghost flex-1 text-xs">塑料</button>
          <button class="btn btn-ghost flex-1 text-xs">玻璃</button>
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">颜色</label>
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded border border-[var(--border-default)] cursor-pointer" style="background-color: #4F8CFF"></div>
            <input type="text" value="#4F8CFF" class="input flex-1 text-xs" />
          </div>
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">金属度</label>
          <input type="range" min="0" max="100" value="80" class="w-full" />
        </div>
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">粗糙度</label>
          <input type="range" min="0" max="100" value="40" class="w-full" />
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="panel-section">
      <button class="btn btn-secondary w-full">应用变换</button>
    </div>
  </div>
</template>

<script setup lang="ts">
// Placeholder for modeling properties
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