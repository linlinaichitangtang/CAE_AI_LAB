<template>
  <div class="diff-viewer diff-viewer-root">
    <!-- 版本信息头部 -->
    <div class="diff-header">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div class="version-badge old-version">
            <span class="version-label">旧版本</span>
            <span class="version-title">{{ oldTitle }}</span>
            <span class="version-date">{{ oldDate }}</span>
          </div>
          <span class="diff-arrow">&rarr;</span>
          <div class="version-badge new-version">
            <span class="version-label">新版本</span>
            <span class="version-title">{{ newTitle }}</span>
            <span class="version-date">{{ newDate }}</span>
          </div>
        </div>
        <div class="diff-stats">
          <span class="stat-add">+{{ diffResult.stats.additions }} 行新增</span>
          <span class="stat-delete">-{{ diffResult.stats.deletions }} 行删除</span>
          <span class="stat-unchanged">{{ diffResult.stats.unchanged }} 行未变</span>
        </div>
      </div>
    </div>

    <!-- 并排对比视图 -->
    <div class="diff-content" ref="contentRef">
      <div class="diff-panels">
        <!-- 左栏：旧版本 -->
        <div class="diff-panel left-panel">
          <div class="panel-header">
            <span class="panel-title">{{ oldTitle }}</span>
          </div>
          <div
            class="panel-body"
            ref="leftPanelRef"
            @scroll="onLeftScroll"
          >
            <table class="diff-table">
              <tbody>
                <tr
                  v-for="(row, idx) in sideBySideLines"
                  :key="'left-' + idx"
                  :class="getRowClass(row.left.type)"
                >
                  <td class="line-num">{{ row.left.lineNum ?? '' }}</td>
                  <td class="line-content">
                    <span v-if="row.left.type === 'remove'" class="line-prefix">-</span>
                    <span v-else-if="row.left.type === 'empty'" class="line-prefix">&nbsp;</span>
                    <span v-else class="line-prefix">&nbsp;</span>
                    <span class="line-text">{{ row.left.content }}</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- 分割线 -->
        <div class="diff-divider"></div>

        <!-- 右栏：新版本 -->
        <div class="diff-panel right-panel">
          <div class="panel-header">
            <span class="panel-title">{{ newTitle }}</span>
          </div>
          <div
            class="panel-body"
            ref="rightPanelRef"
            @scroll="onRightScroll"
          >
            <table class="diff-table">
              <tbody>
                <tr
                  v-for="(row, idx) in sideBySideLines"
                  :key="'right-' + idx"
                  :class="getRowClass(row.right.type)"
                >
                  <td class="line-num">{{ row.right.lineNum ?? '' }}</td>
                  <td class="line-content">
                    <span v-if="row.right.type === 'add'" class="line-prefix">+</span>
                    <span v-else-if="row.right.type === 'empty'" class="line-prefix">&nbsp;</span>
                    <span v-else class="line-prefix">&nbsp;</span>
                    <span class="line-text">{{ row.right.content }}</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { computeDiff, toSideBySide, normalizeHtmlForDiff, type DiffResult, type SideBySideLine } from '@/utils/diff'

interface Props {
  oldContent: string
  newContent: string
  oldTitle: string
  newTitle: string
  oldDate: string
  newDate: string
}

const props = defineProps<Props>()

const contentRef = ref<HTMLElement | null>(null)
const leftPanelRef = ref<HTMLElement | null>(null)
const rightPanelRef = ref<HTMLElement | null>(null)

// 标记是否为程序触发的滚动（避免循环触发）
let isSyncingScroll = false

// 计算 diff 结果
const diffResult = computed<DiffResult>(() => {
  const oldNormalized = normalizeHtmlForDiff(props.oldContent)
  const newNormalized = normalizeHtmlForDiff(props.newContent)
  return computeDiff(oldNormalized, newNormalized)
})

// 并排对比行数据
const sideBySideLines = computed<SideBySideLine[]>(() => {
  return toSideBySide(diffResult.value)
})

// 获取行样式类名
function getRowClass(type: string): string {
  switch (type) {
    case 'add':
      return 'diff-add'
    case 'remove':
      return 'diff-remove'
    case 'empty':
      return 'diff-empty'
    default:
      return 'diff-unchanged'
  }
}

// 同步滚动：左栏滚动时同步右栏
function onLeftScroll() {
  if (isSyncingScroll) return
  isSyncingScroll = true
  if (leftPanelRef.value && rightPanelRef.value) {
    rightPanelRef.value.scrollTop = leftPanelRef.value.scrollTop
    rightPanelRef.value.scrollLeft = leftPanelRef.value.scrollLeft
  }
  requestAnimationFrame(() => {
    isSyncingScroll = false
  })
}

// 同步滚动：右栏滚动时同步左栏
function onRightScroll() {
  if (isSyncingScroll) return
  isSyncingScroll = true
  if (leftPanelRef.value && rightPanelRef.value) {
    leftPanelRef.value.scrollTop = rightPanelRef.value.scrollTop
    leftPanelRef.value.scrollLeft = rightPanelRef.value.scrollLeft
  }
  requestAnimationFrame(() => {
    isSyncingScroll = false
  })
}
</script>

<style>
/* 全局 CSS 变量 - 在非 scoped 样式中定义以确保生效 */
.diff-viewer-root {
  --border-color: #e5e7eb;
  --header-bg: #f9fafb;
  --panel-bg: #ffffff;
  --line-num-bg: #f3f4f6;
  --line-num-color: #9ca3af;
  --text-color: #374151;
  --text-muted: #6b7280;
  --add-bg: #dcfce7;
  --add-border: #22c55e;
  --add-text: #166534;
  --remove-bg: #fee2e2;
  --remove-border: #ef4444;
  --remove-text: #991b1b;
  --empty-bg: #f9fafb;
  --divider-color: #d1d5db;
}

.dark .diff-viewer-root {
  --border-color: #374151;
  --header-bg: #1f2937;
  --panel-bg: #111827;
  --line-num-bg: #1f2937;
  --line-num-color: #4b5563;
  --text-color: #d1d5db;
  --text-muted: #9ca3af;
  --add-bg: rgba(34, 197, 94, 0.15);
  --add-border: #22c55e;
  --add-text: #86efac;
  --remove-bg: rgba(239, 68, 68, 0.15);
  --remove-border: #ef4444;
  --remove-text: #fca5a5;
  --empty-bg: #111827;
  --divider-color: #374151;
}
</style>

<style scoped>
.diff-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

/* 头部 */
.diff-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  background: var(--header-bg);
  flex-shrink: 0;
}

.version-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 13px;
}

.old-version {
  background: var(--remove-bg);
  border: 1px solid var(--remove-border);
}

.new-version {
  background: var(--add-bg);
  border: 1px solid var(--add-border);
}

.version-label {
  font-weight: 600;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-muted);
}

.version-title {
  font-weight: 500;
  color: var(--text-color);
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.version-date {
  font-size: 11px;
  color: var(--text-muted);
}

.diff-arrow {
  font-size: 18px;
  color: var(--text-muted);
  font-weight: bold;
}

.diff-stats {
  display: flex;
  gap: 12px;
  font-size: 12px;
  font-weight: 500;
}

.stat-add {
  color: var(--add-text);
}

.stat-delete {
  color: var(--remove-text);
}

.stat-unchanged {
  color: var(--text-muted);
}

/* 内容区域 */
.diff-content {
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

.diff-panels {
  display: flex;
  height: 100%;
}

.diff-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.panel-header {
  padding: 8px 16px;
  border-bottom: 1px solid var(--border-color);
  background: var(--header-bg);
  flex-shrink: 0;
}

.panel-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.panel-body {
  flex: 1;
  overflow: auto;
  background: var(--panel-bg);
}

.diff-divider {
  width: 1px;
  background: var(--divider-color);
  flex-shrink: 0;
}

/* Diff 表格 */
.diff-table {
  width: 100%;
  border-collapse: collapse;
  font-family: 'SF Mono', 'Fira Code', 'Fira Mono', Menlo, Consolas, monospace;
  font-size: 13px;
  line-height: 1.6;
}

.diff-table tr {
  border: none;
}

.diff-table td {
  padding: 0;
  vertical-align: top;
  white-space: pre-wrap;
  word-break: break-all;
}

.line-num {
  width: 50px;
  min-width: 50px;
  max-width: 50px;
  padding: 0 8px;
  text-align: right;
  background: var(--line-num-bg);
  color: var(--line-num-color);
  font-size: 12px;
  user-select: none;
  border-right: 1px solid var(--border-color);
  line-height: 1.6;
}

.line-content {
  padding: 0 12px;
  color: var(--text-color);
}

.line-prefix {
  display: inline-block;
  width: 16px;
  text-align: center;
  font-weight: bold;
  flex-shrink: 0;
}

.line-text {
  white-space: pre-wrap;
  word-break: break-all;
}

/* 行类型样式 */
.diff-add .line-content {
  background: var(--add-bg);
  border-left: 3px solid var(--add-border);
}

.diff-add .line-prefix {
  color: var(--add-text);
}

.diff-remove .line-content {
  background: var(--remove-bg);
  border-left: 3px solid var(--remove-border);
}

.diff-remove .line-prefix {
  color: var(--remove-text);
}

.diff-empty .line-content {
  background: var(--empty-bg);
}

.diff-empty .line-num {
  background: var(--empty-bg);
}

.diff-unchanged .line-content {
  background: transparent;
}

/* 滚动条样式 */
.panel-body::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.panel-body::-webkit-scrollbar-track {
  background: transparent;
}

.panel-body::-webkit-scrollbar-thumb {
  background: var(--divider-color);
  border-radius: 4px;
}

.panel-body::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}
</style>
