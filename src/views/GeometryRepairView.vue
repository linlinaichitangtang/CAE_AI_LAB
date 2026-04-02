<template>
  <div class="geometry-repair-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">几何清理与修复</h2>
        <p class="text-xs text-[var(--text-muted)]">CAD 修复：裂缝填补/小孔移除/边界统一，STL/STEP 修复</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板选择 -->
        <select v-model="selectedTemplate" @change="loadTemplate" class="input w-auto text-xs">
          <option value="">选择模板</option>
          <option v-for="t in templates" :key="t.id" :value="t.id">{{ t.name }}</option>
        </select>
        <!-- 重置 -->
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x27F3;</span> 重置
        </button>
        <!-- 导出 -->
        <button v-if="repairResult" @click="exportResult" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x2B07;</span> 导出
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">

        <!-- 文件导入区 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">1. 文件导入</h3>
          </div>
          <!-- 拖拽上传区 -->
          <div
            @dragover.prevent="onDragOver"
            @dragleave="onDragLeave"
            @drop.prevent="onDrop"
            :class="['border-2 border-dashed rounded-lg p-4 text-center transition cursor-pointer',
              isDragging ? 'border-[var(--primary)] bg-[var(--primary-glow)]' : 'border-[var(--border-subtle)] hover:border-[var(--border-default)]']"
            @click="triggerFileInput"
          >
            <input ref="fileInputRef" type="file" :accept="acceptFormats" class="hidden" @change="onFileSelected" />
            <div class="text-2xl mb-1">&#x1F4C1;</div>
            <p class="text-xs text-[var(--text-secondary)]">拖拽文件到此处或点击选择</p>
            <p class="text-[10px] text-[var(--text-muted)] mt-1">支持 STL / STEP / IGES / BREP</p>
          </div>
          <!-- 已选文件信息 -->
          <div v-if="config.file_path" class="mt-2 p-2 rounded bg-[var(--bg-base)]">
            <div class="flex items-center justify-between">
              <span class="text-xs text-[var(--text-primary)] truncate max-w-[180px]">{{ fileName }}</span>
              <span class="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">{{ config.file_format }}</span>
            </div>
          </div>
          <!-- 文件格式选择 -->
          <div class="mt-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">文件格式</label>
            <select v-model="config.file_format" class="input w-full text-xs">
              <option value="STL">STL</option>
              <option value="STEP">STEP</option>
              <option value="IGES">IGES</option>
              <option value="BREP">BREP</option>
            </select>
          </div>
        </div>

        <!-- 检测设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">2. 检测设置</h3>
          </div>
          <!-- 容差 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">容差 (mm)</label>
            <input v-model.number="config.tolerance" type="number" step="0.001" min="0.0001" class="input w-full text-xs" />
          </div>
          <!-- 合并距离 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">合并距离 (mm)</label>
            <input v-model.number="config.merge_distance" type="number" step="0.01" min="0" class="input w-full text-xs" />
          </div>
          <!-- 最小面面积 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">最小面面积 (mm&sup2;)</label>
            <input v-model.number="config.min_face_area" type="number" step="0.1" min="0" class="input w-full text-xs" />
          </div>
          <!-- 最小孔径 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">最小孔径 (mm)</label>
            <input v-model.number="config.min_hole_diameter" type="number" step="0.1" min="0" class="input w-full text-xs" />
          </div>
          <!-- 最大边长 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">最大边长 (mm)</label>
            <input v-model.number="config.max_edge_length" type="number" step="0.1" min="0" class="input w-full text-xs" />
          </div>
        </div>

        <!-- 问题列表 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex-1 overflow-y-auto">
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <span class="step-dot active"></span>
              <h3 class="text-sm font-semibold text-[var(--text-primary)]">3. 问题列表</h3>
            </div>
            <div class="flex items-center gap-1">
              <span class="text-[10px] px-1.5 py-0.5 rounded" :class="criticalCount > 0 ? 'bg-red-500/20 text-[var(--accent-red)]' : 'bg-[var(--bg-elevated)] text-[var(--text-muted)]'">
                {{ criticalCount }} 严重
              </span>
              <span class="text-[10px] px-1.5 py-0.5 rounded" :class="warningCount > 0 ? 'bg-yellow-500/20 text-[var(--accent-yellow)]' : 'bg-[var(--bg-elevated)] text-[var(--text-muted)]'">
                {{ warningCount }} 警告
              </span>
              <span class="text-[10px] px-1.5 py-0.5 rounded" :class="infoCount > 0 ? 'bg-blue-500/20 text-[var(--primary)]' : 'bg-[var(--bg-elevated)] text-[var(--text-muted)]'">
                {{ infoCount }} 提示
              </span>
            </div>
          </div>

          <!-- 空状态 -->
          <div v-if="issues.length === 0" class="text-center py-6">
            <div class="text-2xl mb-1">&#x1F50D;</div>
            <p class="text-xs text-[var(--text-muted)]">导入文件后点击"检测问题"</p>
          </div>

          <!-- Critical 问题 -->
          <div v-if="criticalIssues.length > 0" class="mb-3">
            <div class="flex items-center gap-1 mb-1.5">
              <span class="w-2 h-2 rounded-full bg-[var(--accent-red)]"></span>
              <span class="text-xs font-medium text-[var(--accent-red)]">严重问题</span>
            </div>
            <div v-for="(issue, idx) in criticalIssues" :key="'c-' + idx" class="issue-item mb-1">
              <label class="flex items-start gap-2 cursor-pointer">
                <input type="checkbox" v-model="issue.selected" class="mt-0.5" />
                <div class="flex-1 min-w-0">
                  <div class="text-xs text-[var(--text-primary)] truncate">{{ issueLabel(issue.type) }}</div>
                  <div class="text-[10px] text-[var(--text-muted)] truncate">{{ issue.description }}</div>
                  <div class="text-[10px] text-[var(--text-muted)]">
                    面: {{ issue.face_ids.length }} | 边: {{ issue.edge_ids.length }}
                  </div>
                </div>
              </label>
            </div>
          </div>

          <!-- Warning 问题 -->
          <div v-if="warningIssues.length > 0" class="mb-3">
            <div class="flex items-center gap-1 mb-1.5">
              <span class="w-2 h-2 rounded-full bg-[var(--accent-yellow)]"></span>
              <span class="text-xs font-medium text-[var(--accent-yellow)]">警告</span>
            </div>
            <div v-for="(issue, idx) in warningIssues" :key="'w-' + idx" class="issue-item mb-1">
              <label class="flex items-start gap-2 cursor-pointer">
                <input type="checkbox" v-model="issue.selected" class="mt-0.5" />
                <div class="flex-1 min-w-0">
                  <div class="text-xs text-[var(--text-primary)] truncate">{{ issueLabel(issue.type) }}</div>
                  <div class="text-[10px] text-[var(--text-muted)] truncate">{{ issue.description }}</div>
                  <div class="text-[10px] text-[var(--text-muted)]">
                    面: {{ issue.face_ids.length }} | 边: {{ issue.edge_ids.length }}
                  </div>
                </div>
              </label>
            </div>
          </div>

          <!-- Info 问题 -->
          <div v-if="infoIssues.length > 0" class="mb-3">
            <div class="flex items-center gap-1 mb-1.5">
              <span class="w-2 h-2 rounded-full bg-[var(--primary)]"></span>
              <span class="text-xs font-medium text-[var(--primary)]">提示</span>
            </div>
            <div v-for="(issue, idx) in infoIssues" :key="'i-' + idx" class="issue-item mb-1">
              <label class="flex items-start gap-2 cursor-pointer">
                <input type="checkbox" v-model="issue.selected" class="mt-0.5" />
                <div class="flex-1 min-w-0">
                  <div class="text-xs text-[var(--text-primary)] truncate">{{ issueLabel(issue.type) }}</div>
                  <div class="text-[10px] text-[var(--text-muted)] truncate">{{ issue.description }}</div>
                  <div class="text-[10px] text-[var(--text-muted)]">
                    面: {{ issue.face_ids.length }} | 边: {{ issue.edge_ids.length }}
                  </div>
                </div>
              </label>
            </div>
          </div>
        </div>

        <!-- 修复操作队列 -->
        <div v-if="repairQueue.length > 0" class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-2">
            <span class="text-xs font-medium text-[var(--text-primary)]">修复队列</span>
            <span class="text-[10px] text-[var(--text-muted)]">{{ repairQueue.length }} 项</span>
          </div>
          <div class="max-h-32 overflow-y-auto">
            <div v-for="(op, idx) in repairQueue" :key="idx" class="flex items-center gap-2 mb-1 text-xs">
              <span
                :class="['w-1.5 h-1.5 rounded-full flex-shrink-0', op.status === 'success' ? 'bg-[var(--accent-green)]' : op.status === 'failed' ? 'bg-[var(--accent-red)]' : 'bg-[var(--text-muted)]']"
              ></span>
              <span class="text-[var(--text-secondary)] truncate flex-1">{{ operationLabel(op.type) }}</span>
              <span v-if="op.status === 'success'" class="text-[10px] text-[var(--accent-green)]">成功</span>
              <span v-else-if="op.status === 'failed'" class="text-[10px] text-[var(--accent-red)]">失败</span>
              <span v-else class="text-[10px] text-[var(--text-muted)]">等待</span>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="px-4 py-3 mt-auto border-t border-[var(--border-subtle)] space-y-2">
          <button
            @click="handleImport"
            :disabled="!config.file_path || importing"
            :class="['btn w-full text-xs', importing ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="importing" class="mr-1 animate-spin">&#x27F3;</span>
            {{ importing ? '导入中...' : '&#x1F4E5; 导入几何' }}
          </button>
          <button
            @click="handleDetect"
            :disabled="!imported || detecting"
            :class="['btn w-full text-xs', detecting ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="detecting" class="mr-1 animate-spin">&#x27F3;</span>
            {{ detecting ? '检测中...' : '&#x1F50D; 检测问题' }}
          </button>
          <button
            @click="handleRepair"
            :disabled="issues.length === 0 || repairing"
            :class="['btn w-full text-xs', repairing ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="repairing" class="mr-1 animate-spin">&#x27F3;</span>
            {{ repairing ? '修复中...' : '&#x1F527; 自动修复' }}
          </button>
          <button
            @click="handleExport"
            :disabled="!repairResult"
            class="btn w-full text-xs"
            :class="repairResult ? 'btn-primary' : 'btn-loading'"
          >
            &#x1F4E4; 导出修复文件
          </button>
        </div>
      </div>

      <!-- Right Area -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- 3D 几何预览 -->
        <div class="flex-1 relative">
          <div class="w-full h-full">
            <canvas ref="geometryCanvas" class="w-full h-full"></canvas>
          </div>
          <!-- 视图切换 -->
          <div class="absolute top-3 right-3 flex items-center gap-1">
            <button
              @click="viewMode = 'original'"
              :class="['px-2 py-1 text-[10px] rounded transition', viewMode === 'original' ? 'bg-[var(--primary)] text-white' : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
            >
              原始模型
            </button>
            <button
              @click="viewMode = 'repaired'"
              :class="['px-2 py-1 text-[10px] rounded transition', viewMode === 'repaired' ? 'bg-[var(--primary)] text-white' : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
            >
              修复后
            </button>
            <button
              @click="viewMode = 'compare'"
              :class="['px-2 py-1 text-[10px] rounded transition', viewMode === 'compare' ? 'bg-[var(--primary)] text-white' : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
            >
              对比
            </button>
          </div>
          <!-- 空状态 -->
          <div v-if="!imported && !repairResult" class="absolute inset-0 flex items-center justify-center pointer-events-none">
            <div class="text-center text-[var(--text-muted)]">
              <div class="text-4xl mb-2">&#x1F4D0;</div>
              <p class="text-sm">导入几何文件开始修复</p>
            </div>
          </div>
        </div>

        <!-- 修复统计面板 -->
        <div v-if="repairResult" class="bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] p-4">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">修复统计</h3>
            <span
              :class="['text-xs px-2 py-0.5 rounded', repairResult.validation_passed ? 'bg-green-500/20 text-[var(--accent-green)]' : 'bg-red-500/20 text-[var(--accent-red)]']"
            >
              {{ repairResult.validation_passed ? '验证通过' : '验证未通过' }}
            </span>
          </div>

          <div class="grid grid-cols-5 gap-3 mb-3">
            <!-- 面数变化 -->
            <div class="stat-card text-center">
              <div class="text-[10px] text-[var(--text-muted)]">面数</div>
              <div class="text-sm font-bold text-[var(--text-primary)]">
                {{ repairResult.stats.total_faces_after }}
              </div>
              <div class="text-[10px]" :class="faceChangeClass">
                {{ faceChangeText }}
              </div>
            </div>
            <!-- 边数变化 -->
            <div class="stat-card text-center">
              <div class="text-[10px] text-[var(--text-muted)]">边数</div>
              <div class="text-sm font-bold text-[var(--text-primary)]">
                {{ repairResult.stats.total_edges_after }}
              </div>
              <div class="text-[10px]" :class="edgeChangeClass">
                {{ edgeChangeText }}
              </div>
            </div>
            <!-- 顶点变化 -->
            <div class="stat-card text-center">
              <div class="text-[10px] text-[var(--text-muted)]">顶点</div>
              <div class="text-sm font-bold text-[var(--text-primary)]">
                {{ repairResult.stats.total_vertices_after }}
              </div>
              <div class="text-[10px]" :class="vertexChangeClass">
                {{ vertexChangeText }}
              </div>
            </div>
            <!-- 水密性 -->
            <div class="stat-card text-center">
              <div class="text-[10px] text-[var(--text-muted)]">水密性</div>
              <div class="text-sm font-bold" :class="repairResult.stats.watertight_after ? 'text-[var(--accent-green)]' : 'text-[var(--accent-red)]'">
                {{ repairResult.stats.watertight_after ? '是' : '否' }}
              </div>
              <div class="text-[10px] text-[var(--text-muted)]">
                {{ repairResult.stats.watertight_before ? '是' : '否' }} &#x2192; {{ repairResult.stats.watertight_after ? '是' : '否' }}
              </div>
            </div>
            <!-- 体积变化 -->
            <div class="stat-card text-center">
              <div class="text-[10px] text-[var(--text-muted)]">体积 (mm&sup3;)</div>
              <div class="text-sm font-bold text-[var(--text-primary)]">
                {{ repairResult.stats.volume_after.toFixed(1) }}
              </div>
              <div class="text-[10px]" :class="volumeChangeClass">
                {{ volumeChangeText }}
              </div>
            </div>
          </div>

          <!-- 修复前后对比数据 -->
          <div class="grid grid-cols-2 gap-3">
            <div class="p-2 rounded bg-[var(--bg-base)]">
              <div class="text-[10px] text-[var(--text-muted)] mb-1">修复前</div>
              <div class="grid grid-cols-3 gap-2 text-[10px]">
                <div>
                  <span class="text-[var(--text-muted)]">面:</span>
                  <span class="text-[var(--text-secondary)]">{{ repairResult.stats.total_faces_before }}</span>
                </div>
                <div>
                  <span class="text-[var(--text-muted)]">边:</span>
                  <span class="text-[var(--text-secondary)]">{{ repairResult.stats.total_edges_before }}</span>
                </div>
                <div>
                  <span class="text-[var(--text-muted)]">顶点:</span>
                  <span class="text-[var(--text-secondary)]">{{ repairResult.stats.total_vertices_before }}</span>
                </div>
              </div>
              <div class="text-[10px] mt-1">
                <span class="text-[var(--text-muted)]">体积:</span>
                <span class="text-[var(--text-secondary)]">{{ repairResult.stats.volume_before.toFixed(1) }} mm&sup3;</span>
              </div>
            </div>
            <div class="p-2 rounded bg-[var(--bg-base)]">
              <div class="text-[10px] text-[var(--text-muted)] mb-1">修复后</div>
              <div class="grid grid-cols-3 gap-2 text-[10px]">
                <div>
                  <span class="text-[var(--text-muted)]">面:</span>
                  <span class="text-[var(--text-secondary)]">{{ repairResult.stats.total_faces_after }}</span>
                </div>
                <div>
                  <span class="text-[var(--text-muted)]">边:</span>
                  <span class="text-[var(--text-secondary)]">{{ repairResult.stats.total_edges_after }}</span>
                </div>
                <div>
                  <span class="text-[var(--text-muted)]">顶点:</span>
                  <span class="text-[var(--text-secondary)]">{{ repairResult.stats.total_vertices_after }}</span>
                </div>
              </div>
              <div class="text-[10px] mt-1">
                <span class="text-[var(--text-muted)]">体积:</span>
                <span class="text-[var(--text-secondary)]">{{ repairResult.stats.volume_after.toFixed(1) }} mm&sup3;</span>
              </div>
            </div>
          </div>

          <!-- 修复操作摘要 -->
          <div class="mt-3">
            <div class="text-[10px] text-[var(--text-muted)] mb-1">执行的操作</div>
            <div class="flex flex-wrap gap-1">
              <span
                v-for="(op, idx) in repairResult.operations_performed"
                :key="idx"
                :class="['text-[10px] px-1.5 py-0.5 rounded',
                  op.status === 'success' ? 'bg-green-500/10 text-[var(--accent-green)]' : 'bg-red-500/10 text-[var(--accent-red)]']"
              >
                {{ operationLabel(op.type) }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type {
  GeometryIssue,
  GeometryRepairConfig,
  GeometryRepairResult,
  GeometryTemplate,
  RepairOperation,
  ImportedGeometry,
  GeometryIssueType,
  RepairOperationType,
} from '@/api/geometryRepair'

// ============ 类型扩展 ============

interface SelectableIssue extends GeometryIssue {
  selected: boolean
}

// ============ 常量 ============

const acceptFormats = '.stl,.step,.stp,.iges,.igs,.brep,.brp'

const issueTypeLabels: Record<GeometryIssueType, string> = {
  sliver_face: '狭长面',
  narrow_face: '窄面',
  small_hole: '小孔',
  gap: '缝隙',
  overlap: '重叠',
  non_manifold: '非流形',
  duplicate_face: '重复面',
  short_edge: '短边',
  t_junction: 'T型交叉',
  self_intersection: '自相交',
}

const operationTypeLabels: Record<RepairOperationType, string> = {
  fill_sliver: '填补狭长面',
  remove_hole: '移除小孔',
  close_gap: '闭合缝隙',
  remove_overlap: '移除重叠',
  fix_non_manifold: '修复非流形',
  merge_faces: '合并面',
  remove_duplicate: '移除重复',
  extend_short_edge: '延伸短边',
  fix_t_junction: '修复T型交叉',
  fix_self_intersection: '修复自相交',
}

const issueToOperation: Record<GeometryIssueType, RepairOperationType> = {
  sliver_face: 'fill_sliver',
  narrow_face: 'merge_faces',
  small_hole: 'remove_hole',
  gap: 'close_gap',
  overlap: 'remove_overlap',
  non_manifold: 'fix_non_manifold',
  duplicate_face: 'remove_duplicate',
  short_edge: 'extend_short_edge',
  t_junction: 'fix_t_junction',
  self_intersection: 'fix_self_intersection',
}

// ============ 模板数据 ============

const templateData: GeometryTemplate[] = [
  {
    id: 'stl_print',
    name: '3D打印 STL 修复',
    name_en: '3D Print STL Repair',
    category: 'manufacturing',
    description: '针对3D打印的STL模型修复，处理法线反转、非流形边、小孔等问题',
    file_format: 'STL',
    typical_issues: ['non_manifold', 'small_hole', 'gap', 'duplicate_face', 'self_intersection'],
  },
  {
    id: 'step_import',
    name: 'STEP 导入清理',
    name_en: 'STEP Import Cleanup',
    category: 'cad_import',
    description: '清理STEP文件导入后产生的狭长面、T型交叉和微小几何特征',
    file_format: 'STEP',
    typical_issues: ['sliver_face', 'narrow_face', 't_junction', 'short_edge'],
  },
  {
    id: 'iges_legacy',
    name: 'IGES 遗留修复',
    name_en: 'IGES Legacy Repair',
    category: 'legacy',
    description: '修复旧版IGES文件中的缝隙、重叠和自相交问题',
    file_format: 'IGES',
    typical_issues: ['gap', 'overlap', 'self_intersection', 'non_manifold'],
  },
  {
    id: 'brep_precise',
    name: 'BREP 精确修复',
    name_en: 'BREP Precise Repair',
    category: 'precision',
    description: 'BREP边界表示的精确几何修复，处理拓扑错误和几何偏差',
    file_format: 'BREP',
    typical_issues: ['sliver_face', 't_junction', 'short_edge', 'narrow_face'],
  },
]

// ============ 状态 ============

const importing = ref(false)
const detecting = ref(false)
const repairing = ref(false)
const imported = ref(false)
const isDragging = ref(false)
const viewMode = ref<'original' | 'repaired' | 'compare'>('original')
const selectedTemplate = ref('')
const templates = ref<GeometryTemplate[]>([])
const issues = ref<SelectableIssue[]>([])
const repairQueue = ref<RepairOperation[]>([])
const repairResult = ref<GeometryRepairResult | null>(null)
const importedData = ref<ImportedGeometry | null>(null)

// Canvas ref
const geometryCanvas = ref<HTMLCanvasElement | null>(null)
const fileInputRef = ref<HTMLInputElement | null>(null)

// ============ 配置 ============

const config = reactive<GeometryRepairConfig>({
  file_path: '',
  file_format: 'STL',
  repair_operations: [],
  tolerance: 0.01,
  merge_distance: 0.1,
  min_face_area: 0.5,
  min_hole_diameter: 1.0,
  max_edge_length: 5.0,
})

// ============ 计算属性 ============

const fileName = computed(() => {
  if (!config.file_path) return ''
  return config.file_path.split(/[/\\]/).pop() || config.file_path
})

const criticalIssues = computed(() => issues.value.filter(i => i.severity === 'critical'))
const warningIssues = computed(() => issues.value.filter(i => i.severity === 'warning'))
const infoIssues = computed(() => issues.value.filter(i => i.severity === 'info'))

const criticalCount = computed(() => criticalIssues.value.length)
const warningCount = computed(() => warningIssues.value.length)
const infoCount = computed(() => infoIssues.value.length)

const faceChangeClass = computed(() => {
  if (!repairResult.value) return 'text-[var(--text-muted)]'
  const diff = repairResult.value.stats.total_faces_after - repairResult.value.stats.total_faces_before
  return diff < 0 ? 'text-[var(--accent-green)]' : diff > 0 ? 'text-[var(--accent-yellow)]' : 'text-[var(--text-muted)]'
})

const faceChangeText = computed(() => {
  if (!repairResult.value) return ''
  const before = repairResult.value.stats.total_faces_before
  const after = repairResult.value.stats.total_faces_after
  return `${before} \u2192 ${after}`
})

const edgeChangeClass = computed(() => {
  if (!repairResult.value) return 'text-[var(--text-muted)]'
  const diff = repairResult.value.stats.total_edges_after - repairResult.value.stats.total_edges_before
  return diff < 0 ? 'text-[var(--accent-green)]' : diff > 0 ? 'text-[var(--accent-yellow)]' : 'text-[var(--text-muted)]'
})

const edgeChangeText = computed(() => {
  if (!repairResult.value) return ''
  const before = repairResult.value.stats.total_edges_before
  const after = repairResult.value.stats.total_edges_after
  return `${before} \u2192 ${after}`
})

const vertexChangeClass = computed(() => {
  if (!repairResult.value) return 'text-[var(--text-muted)]'
  const diff = repairResult.value.stats.total_vertices_after - repairResult.value.stats.total_vertices_before
  return diff < 0 ? 'text-[var(--accent-green)]' : diff > 0 ? 'text-[var(--accent-yellow)]' : 'text-[var(--text-muted)]'
})

const vertexChangeText = computed(() => {
  if (!repairResult.value) return ''
  const before = repairResult.value.stats.total_vertices_before
  const after = repairResult.value.stats.total_vertices_after
  return `${before} \u2192 ${after}`
})

const volumeChangeClass = computed(() => {
  if (!repairResult.value) return 'text-[var(--text-muted)]'
  const diff = repairResult.value.stats.volume_after - repairResult.value.stats.volume_before
  const pct = repairResult.value.stats.volume_before > 0 ? Math.abs(diff / repairResult.value.stats.volume_before * 100) : 0
  return pct < 1 ? 'text-[var(--accent-green)]' : 'text-[var(--accent-yellow)]'
})

const volumeChangeText = computed(() => {
  if (!repairResult.value) return ''
  const before = repairResult.value.stats.volume_before
  const after = repairResult.value.stats.volume_after
  const diff = after - before
  const sign = diff >= 0 ? '+' : ''
  return `${sign}${diff.toFixed(1)} mm\u00B3`
})

// ============ 方法 ============

function issueLabel(type: GeometryIssueType): string {
  return issueTypeLabels[type] || type
}

function operationLabel(type: RepairOperationType): string {
  return operationTypeLabels[type] || type
}

function triggerFileInput() {
  fileInputRef.value?.click()
}

function onDragOver() {
  isDragging.value = true
}

function onDragLeave() {
  isDragging.value = false
}

function onDrop(e: DragEvent) {
  isDragging.value = false
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    handleFile(files[0])
  }
}

function onFileSelected(e: Event) {
  const input = e.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    handleFile(input.files[0])
  }
}

function handleFile(file: File) {
  config.file_path = file.name
  const ext = file.name.split('.').pop()?.toUpperCase() || ''
  if (['STL', 'STEP', 'STP', 'IGES', 'IGS', 'BREP', 'BRP'].includes(ext)) {
    if (ext === 'STP') config.file_format = 'STEP'
    else if (ext === 'IGS') config.file_format = 'IGES'
    else if (ext === 'BRP') config.file_format = 'BREP'
    else config.file_format = ext as GeometryRepairConfig['file_format']
  }
}

function loadTemplate() {
  if (!selectedTemplate.value) return
  const tmpl = templateData.find(t => t.id === selectedTemplate.value)
  if (!tmpl) return

  config.file_format = tmpl.file_format

  // 根据模板调整检测参数
  switch (tmpl.id) {
    case 'stl_print':
      config.tolerance = 0.05
      config.merge_distance = 0.2
      config.min_face_area = 0.1
      config.min_hole_diameter = 2.0
      config.max_edge_length = 3.0
      break
    case 'step_import':
      config.tolerance = 0.001
      config.merge_distance = 0.01
      config.min_face_area = 0.01
      config.min_hole_diameter = 0.5
      config.max_edge_length = 1.0
      break
    case 'iges_legacy':
      config.tolerance = 0.1
      config.merge_distance = 0.5
      config.min_face_area = 1.0
      config.min_hole_diameter = 3.0
      config.max_edge_length = 10.0
      break
    case 'brep_precise':
      config.tolerance = 0.0001
      config.merge_distance = 0.001
      config.min_face_area = 0.001
      config.min_hole_diameter = 0.1
      config.max_edge_length = 0.5
      break
  }
}

function resetAll() {
  config.file_path = ''
  config.file_format = 'STL'
  config.tolerance = 0.01
  config.merge_distance = 0.1
  config.min_face_area = 0.5
  config.min_hole_diameter = 1.0
  config.max_edge_length = 5.0
  config.repair_operations = []
  imported.value = false
  issues.value = []
  repairQueue.value = []
  repairResult.value = null
  importedData.value = null
  selectedTemplate.value = ''
  viewMode.value = 'original'
}

/** 生成模拟导入数据 */
function generateMockImportData(): ImportedGeometry {
  const nodes: ImportedGeometry['nodes'] = []
  const elements: ImportedGeometry['elements'] = []
  const faces: ImportedGeometry['faces'] = []

  // 生成一个简单的立方体网格
  const size = 20
  const step = 2
  let nodeId = 0
  const grid: number[][][] = []

  for (let x = 0; x <= size; x += step) {
    const slice: number[][] = []
    for (let y = 0; y <= size; y += step) {
      const row: number[] = []
      for (let z = 0; z <= size; z += step) {
        nodes.push({
          x: x + (Math.random() - 0.5) * 0.05,
          y: y + (Math.random() - 0.5) * 0.05,
          z: z + (Math.random() - 0.5) * 0.05,
        })
        row.push(nodeId++)
      }
      slice.push(row)
    }
    grid.push(slice)
  }

  let elemId = 0
  const nx = grid.length
  const ny = grid[0].length
  const nz = grid[0][0].length

  for (let i = 0; i < nx - 1; i++) {
    for (let j = 0; j < ny - 1; j++) {
      for (let k = 0; k < nz - 1; k++) {
        const n0 = grid[i][j][k]
        const n1 = grid[i + 1][j][k]
        const n2 = grid[i + 1][j + 1][k]
        const n3 = grid[i][j + 1][k]
        const n4 = grid[i][j][k + 1]
        const n5 = grid[i + 1][j][k + 1]
        const n6 = grid[i + 1][j + 1][k + 1]
        const n7 = grid[i][j + 1][k + 1]

        elements.push({ id: elemId++, node_ids: [n0, n1, n2, n3, n4, n5, n6, n7], type: 'HEX8' })

        // 6 faces per hex
        const faceDefs = [
          [n0, n1, n2, n3],
          [n4, n5, n6, n7],
          [n0, n1, n5, n4],
          [n2, n3, n7, n6],
          [n0, n3, n7, n4],
          [n1, n2, n6, n5],
        ]
        const normals = [
          { x: 0, y: 0, z: -1 },
          { x: 0, y: 0, z: 1 },
          { x: 0, y: -1, z: 0 },
          { x: 0, y: 1, z: 0 },
          { x: -1, y: 0, z: 0 },
          { x: 1, y: 0, z: 0 },
        ]
        for (let f = 0; f < 6; f++) {
          faces.push({
            id: faces.length,
            node_ids: faceDefs[f],
            normal: normals[f],
          })
        }
      }
    }
  }

  return { nodes, elements, faces }
}

/** 生成模拟问题数据 */
function generateMockIssues(): SelectableIssue[] {
  const allTypes: { type: GeometryIssueType; severity: 'critical' | 'warning' | 'info' }[] = [
    { type: 'non_manifold', severity: 'critical' },
    { type: 'self_intersection', severity: 'critical' },
    { type: 'gap', severity: 'critical' },
    { type: 'sliver_face', severity: 'warning' },
    { type: 'narrow_face', severity: 'warning' },
    { type: 'small_hole', severity: 'warning' },
    { type: 'overlap', severity: 'warning' },
    { type: 'short_edge', severity: 'info' },
    { type: 't_junction', severity: 'info' },
    { type: 'duplicate_face', severity: 'info' },
  ]

  const result: SelectableIssue[] = []
  const count = 5 + Math.floor(Math.random() * 6) // 5~10 issues

  for (let i = 0; i < count; i++) {
    const item = allTypes[Math.floor(Math.random() * allTypes.length)]
    const faceCount = 1 + Math.floor(Math.random() * 5)
    const edgeCount = 1 + Math.floor(Math.random() * 8)
    const faceIds = Array.from({ length: faceCount }, () => Math.floor(Math.random() * 500))
    const edgeIds = Array.from({ length: edgeCount }, () => Math.floor(Math.random() * 800))

    result.push({
      type: item.type,
      severity: item.severity,
      description: `在位置 (${(Math.random() * 20).toFixed(1)}, ${(Math.random() * 20).toFixed(1)}, ${(Math.random() * 20).toFixed(1)}) 处检测到${issueTypeLabels[item.type]}`,
      face_ids: faceIds,
      edge_ids: edgeIds,
      position: {
        x: parseFloat((Math.random() * 20).toFixed(3)),
        y: parseFloat((Math.random() * 20).toFixed(3)),
        z: parseFloat((Math.random() * 20).toFixed(3)),
      },
      area: item.type.includes('face') ? parseFloat((Math.random() * 2 + 0.01).toFixed(4)) : undefined,
      length: item.type.includes('edge') || item.type === 'gap' ? parseFloat((Math.random() * 3 + 0.01).toFixed(4)) : undefined,
      selected: true,
    })
  }

  // 确保至少有一个 critical
  if (!result.some(i => i.severity === 'critical')) {
    result[0] = {
      ...result[0],
      type: 'non_manifold',
      severity: 'critical',
      description: '在边界处检测到非流形拓扑结构，影响网格生成',
      selected: true,
    }
  }

  return result
}

/** 生成模拟修复结果 */
function generateMockRepairResult(selectedIssues: SelectableIssue[]): GeometryRepairResult {
  const operations: RepairOperation[] = selectedIssues.map(issue => ({
    type: issueToOperation[issue.type],
    description: `修复${issueTypeLabels[issue.type]} (面: ${issue.face_ids.join(',')})`,
    affected_elements: [...issue.face_ids, ...issue.edge_ids],
    status: Math.random() > 0.1 ? 'success' as const : 'failed' as const,
  }))

  const facesBefore = 1200 + Math.floor(Math.random() * 300)
  const edgesBefore = 2400 + Math.floor(Math.random() * 600)
  const verticesBefore = 800 + Math.floor(Math.random() * 200)
  const volumeBefore = 8000 + Math.random() * 2000

  const successOps = operations.filter(o => o.status === 'success').length
  const faceReduction = Math.floor(successOps * (3 + Math.random() * 5))
  const edgeReduction = Math.floor(successOps * (6 + Math.random() * 10))
  const vertexReduction = Math.floor(successOps * (2 + Math.random() * 4))

  const remainingIssues = selectedIssues.filter((_, idx) => operations[idx]?.status === 'failed')

  return {
    success: remainingIssues.length === 0,
    original_issues: selectedIssues.map(({ selected: _, ...rest }) => rest),
    repaired_issues: remainingIssues.map(({ selected: _, ...rest }) => rest),
    operations_performed: operations,
    stats: {
      total_faces_before: facesBefore,
      total_faces_after: facesBefore - faceReduction,
      total_edges_before: edgesBefore,
      total_edges_after: edgesBefore - edgeReduction,
      total_vertices_before: verticesBefore,
      total_vertices_after: verticesBefore - vertexReduction,
      watertight_before: false,
      watertight_after: remainingIssues.length === 0,
      volume_before: parseFloat(volumeBefore.toFixed(1)),
      volume_after: parseFloat((volumeBefore - Math.random() * 5).toFixed(1)),
    },
    repaired_file_path: config.file_path.replace(/(\.\w+)$/, '_repaired$1'),
    validation_passed: remainingIssues.length === 0,
  }
}

/** 导入 */
async function handleImport() {
  importing.value = true
  try {
    // 尝试调用后端 API
    // importedData.value = await importGeometry(config.file_path, config.file_format)

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 800))
    importedData.value = generateMockImportData()
    imported.value = true

    await nextTick()
    drawGeometry()
  } catch (e) {
    console.error('Import failed:', e)
    // 降级为模拟数据
    await new Promise(resolve => setTimeout(resolve, 500))
    importedData.value = generateMockImportData()
    imported.value = true

    await nextTick()
    drawGeometry()
  } finally {
    importing.value = false
  }
}

/** 检测问题 */
async function handleDetect() {
  detecting.value = true
  try {
    // 尝试调用后端 API
    // const detectedIssues = await detectIssues(config)
    // issues.value = detectedIssues.map(i => ({ ...i, selected: true }))

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 1200))
    issues.value = generateMockIssues()

    await nextTick()
    drawGeometry()
  } catch (e) {
    console.error('Detection failed:', e)
    await new Promise(resolve => setTimeout(resolve, 500))
    issues.value = generateMockIssues()

    await nextTick()
    drawGeometry()
  } finally {
    detecting.value = false
  }
}

/** 自动修复 */
async function handleRepair() {
  repairing.value = true
  try {
    const selectedIssues = issues.value.filter(i => i.selected)

    // 构建修复队列
    repairQueue.value = selectedIssues.map(issue => ({
      type: issueToOperation[issue.type],
      description: `修复${issueTypeLabels[issue.type]}`,
      affected_elements: [...issue.face_ids, ...issue.edge_ids],
      status: 'pending' as const,
    }))

    // 模拟逐步修复
    for (let i = 0; i < repairQueue.value.length; i++) {
      await new Promise(resolve => setTimeout(resolve, 300 + Math.random() * 400))
      repairQueue.value[i].status = Math.random() > 0.1 ? 'success' : 'failed'
    }

    // 尝试调用后端 API
    // repairResult.value = await repairGeometry(config)

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 500))
    repairResult.value = generateMockRepairResult(selectedIssues)

    viewMode.value = 'repaired'

    await nextTick()
    drawGeometry()
  } catch (e) {
    console.error('Repair failed:', e)
    const selectedIssues = issues.value.filter(i => i.selected)
    repairResult.value = generateMockRepairResult(selectedIssues)
    viewMode.value = 'repaired'

    await nextTick()
    drawGeometry()
  } finally {
    repairing.value = false
  }
}

/** 导出 */
function handleExport() {
  if (!repairResult.value) return

  const data = JSON.stringify(repairResult.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'geometry_repair_result.json'
  a.click()
  URL.revokeObjectURL(url)
}

function exportResult() {
  handleExport()
}

/** 绘制 3D 几何预览 */
function drawGeometry() {
  if (!geometryCanvas.value) return

  const container = geometryCanvas.value.parentElement
  if (!container) return

  geometryCanvas.value.width = container.clientWidth
  geometryCanvas.value.height = container.clientHeight

  const ctx = geometryCanvas.value.getContext('2d')
  if (!ctx) return

  const w = geometryCanvas.value.width
  const h = geometryCanvas.value.height

  ctx.clearRect(0, 0, w, h)

  // 背景
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  // 网格
  ctx.strokeStyle = '#1C1D24'
  ctx.lineWidth = 0.5
  const gridSize = 30
  for (let x = 0; x < w; x += gridSize) {
    ctx.beginPath()
    ctx.moveTo(x, 0)
    ctx.lineTo(x, h)
    ctx.stroke()
  }
  for (let y = 0; y < h; y += gridSize) {
    ctx.beginPath()
    ctx.moveTo(0, y)
    ctx.lineTo(w, y)
    ctx.stroke()
  }

  const cx = w / 2
  const cy = h / 2
  const cubeSize = Math.min(w, h) * 0.3

  if (viewMode.value === 'original' || viewMode.value === 'compare') {
    drawCube(ctx, cx - (viewMode.value === 'compare' ? cubeSize * 0.7 : 0), cy, cubeSize, '#4F8CFF', 0.6, issues.value)
  }

  if (viewMode.value === 'repaired' || viewMode.value === 'compare') {
    const offsetX = viewMode.value === 'compare' ? cubeSize * 0.7 : 0
    const repairedIssues = repairResult.value?.repaired_issues || []
    drawCube(ctx, cx + offsetX, cy, cubeSize, '#22C55E', 0.8, repairedIssues)
  }

  // 标签
  ctx.font = '11px sans-serif'
  if (viewMode.value === 'compare') {
    ctx.fillStyle = '#4F8CFF'
    ctx.fillText('原始模型', cx - cubeSize * 0.7 - 30, cy - cubeSize - 20)
    ctx.fillStyle = '#22C55E'
    ctx.fillText('修复后', cx + cubeSize * 0.7 - 20, cy - cubeSize - 20)
  } else if (viewMode.value === 'original') {
    ctx.fillStyle = '#4F8CFF'
    ctx.fillText('原始模型', cx - 25, cy - cubeSize - 20)
  } else {
    ctx.fillStyle = '#22C55E'
    ctx.fillText('修复后模型', cx - 30, cy - cubeSize - 20)
  }

  // 问题标注
  if (issues.value.length > 0 && (viewMode.value === 'original' || viewMode.value === 'compare')) {
    const baseX = viewMode.value === 'compare' ? cx - cubeSize * 0.7 : cx
    drawIssueMarkers(ctx, baseX, cy, cubeSize, issues.value)
  }

  if (repairResult.value && (viewMode.value === 'repaired' || viewMode.value === 'compare')) {
    const baseX = viewMode.value === 'compare' ? cx + cubeSize * 0.7 : cx
    drawIssueMarkers(ctx, baseX, cy, cubeSize, repairResult.value.repaired_issues)
  }
}

/** 绘制立方体线框 */
function drawCube(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  size: number,
  color: string,
  alpha: number,
  issueList: GeometryIssue[]
) {
  const s = size / 2
  const angle = 0.5 // 旋转角

  // 简单等轴测投影
  const cos30 = Math.cos(angle)
  const sin30 = Math.sin(angle)

  const project = (x: number, y: number, z: number) => ({
    sx: cx + (x - z) * cos30 * s * 0.7,
    sy: cy + ((x + z) * sin30 - y) * s * 0.7,
  })

  // 8 vertices
  const vertices = [
    project(-1, -1, -1), project(1, -1, -1), project(1, 1, -1), project(-1, 1, -1),
    project(-1, -1, 1), project(1, -1, 1), project(1, 1, 1), project(-1, 1, 1),
  ]

  // 12 edges
  const edges = [
    [0, 1], [1, 2], [2, 3], [3, 0],
    [4, 5], [5, 6], [6, 7], [7, 4],
    [0, 4], [1, 5], [2, 6], [3, 7],
  ]

  ctx.globalAlpha = alpha
  ctx.strokeStyle = color
  ctx.lineWidth = 1.5

  for (const [a, b] of edges) {
    ctx.beginPath()
    ctx.moveTo(vertices[a].sx, vertices[a].sy)
    ctx.lineTo(vertices[b].sx, vertices[b].sy)
    ctx.stroke()
  }

  // 顶点
  ctx.fillStyle = color
  for (const v of vertices) {
    ctx.beginPath()
    ctx.arc(v.sx, v.sy, 2, 0, Math.PI * 2)
    ctx.fill()
  }

  // 面填充 (半透明)
  const faces = [
    [0, 1, 2, 3], [4, 5, 6, 7],
    [0, 1, 5, 4], [2, 3, 7, 6],
    [0, 3, 7, 4], [1, 2, 6, 5],
  ]

  ctx.globalAlpha = 0.08
  ctx.fillStyle = color
  for (const face of faces) {
    ctx.beginPath()
    ctx.moveTo(vertices[face[0]].sx, vertices[face[0]].sy)
    for (let i = 1; i < face.length; i++) {
      ctx.lineTo(vertices[face[i]].sx, vertices[face[i]].sy)
    }
    ctx.closePath()
    ctx.fill()
  }

  ctx.globalAlpha = 1
}

/** 绘制问题标注 */
function drawIssueMarkers(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  size: number,
  issueList: GeometryIssue[]
) {
  const s = size / 2
  const cos30 = Math.cos(0.5)
  const sin30 = Math.sin(0.5)

  const project = (x: number, y: number, z: number) => ({
    sx: cx + (x - z) * cos30 * s * 0.7,
    sy: cy + ((x + z) * sin30 - y) * s * 0.7,
  })

  for (const issue of issueList) {
    const pos = project(
      (issue.position.x / 20) * 2 - 1,
      (issue.position.y / 20) * 2 - 1,
      (issue.position.z / 20) * 2 - 1
    )

    const color = issue.severity === 'critical'
      ? '#EF4444'
      : issue.severity === 'warning'
        ? '#F59E0B'
        : '#4F8CFF'

    // 标记圆圈
    ctx.beginPath()
    ctx.arc(pos.sx, pos.sy, 6, 0, Math.PI * 2)
    ctx.strokeStyle = color
    ctx.lineWidth = 1.5
    ctx.stroke()

    // 内部点
    ctx.beginPath()
    ctx.arc(pos.sx, pos.sy, 2, 0, Math.PI * 2)
    ctx.fillStyle = color
    ctx.fill()

    // 脉冲效果 (简化)
    ctx.beginPath()
    ctx.arc(pos.sx, pos.sy, 10, 0, Math.PI * 2)
    ctx.strokeStyle = color
    ctx.globalAlpha = 0.3
    ctx.lineWidth = 1
    ctx.stroke()
    ctx.globalAlpha = 1
  }
}

// ============ 生命周期 ============

onMounted(() => {
  templates.value = templateData
})
</script>

<style scoped>
.geometry-repair-view {
  --bg-base: #0A0B0E;
  --bg-surface: #14151A;
  --bg-elevated: #1C1D24;
  --bg-hover: #25262E;
  --border-subtle: #2D2E38;
  --border-default: #3D3E48;
  --text-primary: #E8E9EB;
  --text-secondary: #9CA3AF;
  --text-muted: #6B7280;
  --primary: #4F8CFF;
  --primary-glow: rgba(79, 140, 255, 0.08);
  --accent-red: #EF4444;
  --accent-green: #22C55E;
  --accent-yellow: #F59E0B;
  --radius-md: 8px;
  --radius-full: 9999px;
  --duration-fast: 150ms;
  --ease-out: cubic-bezier(0, 0, 0.2, 1);
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition;
  transition-duration: var(--duration-fast);
  transition-timing-function: var(--ease-out);
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-primary:disabled {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.btn-ghost {
  @apply text-gray-400 hover:text-white hover:bg-gray-700;
}

.btn-outline {
  @apply border border-gray-600 text-gray-300 hover:bg-gray-700;
}

.btn-loading {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.input {
  @apply px-2 py-1.5 bg-[var(--bg-base)] border border-[var(--border-subtle)] rounded text-xs text-[var(--text-primary)] focus:outline-none focus:border-[var(--primary)];
  border-radius: var(--radius-md);
  transition-duration: var(--duration-fast);
  transition-timing-function: var(--ease-out);
}

.stat-card {
  @apply p-3 rounded-lg bg-[var(--bg-base)];
  border-radius: var(--radius-md);
}

.step-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-subtle);
  flex-shrink: 0;
}

.step-dot.active {
  background: var(--primary);
  box-shadow: 0 0 0 3px rgba(79, 140, 255, 0.2);
}

.issue-item {
  padding: 6px 8px;
  border-radius: var(--radius-md);
  background: var(--bg-base);
  transition-duration: var(--duration-fast);
  transition-timing-function: var(--ease-out);
}

.issue-item:hover {
  background: var(--bg-elevated);
}
</style>
