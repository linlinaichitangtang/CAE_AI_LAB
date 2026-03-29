<template>
  <div class="modeling-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">3D建模</h2>
        <p class="text-xs text-[var(--text-muted)]">几何建模、外部模型导入、参数化编辑</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs">
          <span class="mr-1">📁</span> 导入
        </button>
        <button class="btn btn-primary text-xs">
          <span class="mr-1">+</span> 添加几何体
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Scene Tree -->
      <div class="w-56 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          场景树
        </div>
        <div class="flex-1 overflow-y-auto p-3">
          <!-- Empty state -->
          <div v-if="true" class="text-center py-8">
            <div class="text-4xl mb-2 opacity-30">📐</div>
            <p class="text-xs text-[var(--text-muted)]">暂无几何体</p>
            <p class="text-[10px] text-[var(--text-muted)] mt-1">点击"添加几何体"开始</p>
          </div>
        </div>
        <!-- Add geometry button -->
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <select class="input w-full text-xs">
            <option value="">添加基本体...</option>
            <option value="box">立方体</option>
            <option value="sphere">球体</option>
            <option value="cylinder">圆柱体</option>
            <option value="cone">圆锥体</option>
            <option value="torus">圆环体</option>
          </select>
        </div>
      </div>

      <!-- Center: 3D Viewport -->
      <div class="flex-1 relative flex flex-col">
        <!-- Viewport placeholder -->
        <div class="flex-1 flex items-center justify-center bg-[var(--bg-base)]">
          <div class="text-center">
            <div class="w-32 h-32 mx-auto mb-4 rounded-2xl bg-[var(--bg-surface)] flex items-center justify-center">
              <span class="text-6xl opacity-30">🎨</span>
            </div>
            <h3 class="text-base font-semibold text-[var(--text-primary)] mb-2">3D建模模块</h3>
            <p class="text-sm text-[var(--text-muted)] max-w-xs">支持几何建模、参数化编辑、材质贴图等功能</p>
            <button class="btn btn-primary mt-4">
              <span class="mr-2">+</span> 创建第一个模型
            </button>
          </div>
        </div>
        
        <!-- Toolbar -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase tracking-wider">工具</span>
          <div class="flex items-center gap-1">
            <button class="icon-btn w-8 h-8 active" title="选择 (S)">
              <span class="text-sm">⊹</span>
            </button>
            <button class="icon-btn w-8 h-8" title="移动 (M)">
              <span class="text-sm">✥</span>
            </button>
            <button class="icon-btn w-8 h-8" title="旋转 (R)">
              <span class="text-sm">↻</span>
            </button>
            <button class="icon-btn w-8 h-8" title="缩放 (E)">
              <span class="text-sm">⤢</span>
            </button>
          </div>
          <div class="flex-1"></div>
          <span class="text-[10px] text-[var(--text-muted)]">网格: 开启</span>
        </div>
      </div>

      <!-- Right: Properties Panel -->
      <div class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex flex-col">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          属性
        </div>
        <div class="flex-1 overflow-y-auto p-4">
          <!-- No selection state -->
          <div class="text-center py-8">
            <div class="text-3xl mb-2 opacity-30">ℹ️</div>
            <p class="text-xs text-[var(--text-muted)]">选择对象以查看属性</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Placeholder for 3D modeling view
// Full implementation with Three.js/WebGL will be in Phase 2
</script>