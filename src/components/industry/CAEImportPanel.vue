<script setup lang="ts">
/**
 * CAEImportPanel.vue — V3.1-005/006 ANSYS/ABAQUS 导入 UI
 * 支持 .wbpj (ANSYS Workbench)、.cdb (ANSYS Classic)、.cae (ABAQUS)、.inp (Abaqus)
 */
import { ref, computed } from 'vue'
import {
  useCAEImport,
  detectCAEFormat,
  type CAEImportResult,
  type CAEPreview
} from '@/composables/useCAEImport'

// ============ 状态 ============
const {
  isImporting,
  importError,
  importResult,
  supportedFormats,
  importFile,
  clearResult
} = useCAEImport()

// 文件选择
const selectedFile = ref<File | null>(null)
const preview = ref<CAEPreview | null>(null)
const isPreviewLoading = ref(false)

// 导入历史
const importHistory = ref<Array<{
  fileName: string
  format: string
  date: string
  success: boolean
}>>([])

// ============ 计算属性 ============
const canImport = computed(() => selectedFile.value !== null && !isImporting.value)
const formatIcon = computed(() => {
  if (!selectedFile.value) return '📄'
  const ext = selectedFile.value.name.toLowerCase().split('.').pop()
  switch (ext) {
    case 'wbpj': return '🔧'
    case 'cdb': return '⚙️'
    case 'cae': return '🔵'
    case 'inp': return '📋'
    default: return '📄'
  }
})

// ============ 文件处理 ============
async function handleFileSelect(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files?.length) return

  const file = input.files[0]
  selectedFile.value = file
  preview.value = null
  clearResult()

  // 自动检测格式并预览
  await loadPreview(file)
}

async function loadPreview(file: File) {
  isPreviewLoading.value = true

  try {
    const format = detectCAEFormat(file.name)
    const content = await file.text()

    let previewData: CAEPreview | null = null

    switch (format) {
      case 'inp':
        previewData = parseINPPreview(content)
        break
      case 'cdb':
        previewData = parseCDBPreview(content)
        break
      case 'wbpj':
        previewData = parseWBPJPreview(content)
        break
      case 'cae':
        previewData = parseCAEPreview(file)
        break
    }

    preview.value = previewData
  } catch (e: any) {
    console.error('Preview failed:', e)
  } finally {
    isPreviewLoading.value = false
  }
}

// 解析 INP 文件
function parseINPPreview(content: string): CAEPreview {
  const lines = content.split('\n')
  let nodes = 0
  let elements = 0
  let materials = new Set<string>()
  let steps = 0
  let boundaries = 0
  let loads = 0
  let inNodes = false
  let inElements = false

  for (const line of lines) {
    const trimmed = line.trim().toUpperCase()

    if (trimmed.startsWith('*NODE')) {
      inNodes = true
      inElements = false
    } else if (trimmed.startsWith('*ELEMENT')) {
      inElements = true
      inNodes = false
    } else if (trimmed.startsWith('*') && trimmed !== '*NODE' && trimmed !== '*ELEMENT') {
      inNodes = false
      inElements = false
    }

    if (inNodes && !trimmed.startsWith('*')) nodes++
    if (inElements && !trimmed.startsWith('*')) elements++

    if (trimmed.startsWith('*MATERIAL')) {
      const match = trimmed.match(/\$*NAME\s*=\s*(.+)/i) || trimmed.match(/\s*(.+)\s*$/)
      if (match) materials.add(match[1].trim())
    }
    if (trimmed.startsWith('*STEP')) steps++
    if (trimmed.startsWith('*BOUNDARY')) boundaries++
    if (trimmed.startsWith('*CLOAD') || trimmed.startsWith('*DLOAD') || trimmed.startsWith('*LOAD')) loads++
  }

  return { nodes, elements, materials: materials.size, steps, boundaries, loads }
}

// 解析 CDB 文件
function parseCDBPreview(content: string): CAEPreview {
  const lines = content.split('\n')
  let nodes = 0
  let elements = 0
  let materials = 0

  for (const line of lines) {
    const trimmed = line.trim()
    const upper = trimmed.toUpperCase()

    if (upper.startsWith('N,') || /^\s*\d+\s*,\s*[\d.eE+-]+\s*,\s*[\d.eE+-]+\s*,\s*[\d.eE+-]+/.test(trimmed)) {
      nodes++
    }
    if (upper.startsWith('EN,') || /^\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,/.test(trimmed)) {
      elements++
    }
    if (upper.startsWith('MAT,') || /^\s*MAT\s*,\s*\d+/.test(upper)) {
      materials++
    }
  }

  return {
    nodes,
    elements,
    materials,
    geometryInfo: { hasSolid: elements > 0, hasShell: false, hasBeam: false, partCount: 1 }
  }
}

// 解析 WBPJ 文件
function parseWBPJPreview(content: string): CAEPreview {
  try {
    const parser = new DOMParser()
    const doc = parser.parseFromString(content, 'text/xml')

    const projectName = doc.querySelector('ProjectName')?.textContent || ''
    const systems = doc.querySelectorAll('System')

    let meshNodes = 0
    let meshElements = 0

    const meshInfo = doc.querySelectorAll('Mesh')
    meshInfo.forEach(mesh => {
      const nodeCount = mesh.getAttribute('NodeCount')
      const elemCount = mesh.getAttribute('ElementCount')
      if (nodeCount) meshNodes = Math.max(meshNodes, parseInt(nodeCount))
      if (elemCount) meshElements = Math.max(meshElements, parseInt(elemCount))
    })

    return {
      nodes: meshNodes,
      elements: meshElements,
      geometryInfo: { hasSolid: true, hasShell: false, hasBeam: false, partCount: systems.length },
      warnings: [`项目名称: ${projectName}`, `包含 ${systems.length} 个系统`]
    }
  } catch {
    return { nodes: 0, elements: 0, warnings: ['解析失败'] }
  }
}

// 解析 CAE 文件
function parseCAEPreview(file: File): CAEPreview {
  // .cae 是 SQLite 格式，需要后端解析
  return {
    warnings: [
      'ABAQUS .cae 文件 (SQLite 格式)',
      '完整解析需要在后端进行',
      '请使用 File > Import 功能'
    ],
    geometryInfo: { hasSolid: true, hasShell: true, hasBeam: false, partCount: 1 }
  }
}

async function handleImport() {
  if (!selectedFile.value) return

  const result = await importFile(selectedFile.value)

  importHistory.value.unshift({
    fileName: selectedFile.value.name,
    format: result.format,
    date: new Date().toLocaleString(),
    success: result.success
  })

  if (importHistory.value.length > 20) {
    importHistory.value.splice(20)
  }
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`
}

// ============ 快捷模板 ============
const importTemplates = [
  {
    name: 'ANSYS Workbench 项目',
    extensions: ['.wbpj'],
    description: 'ANSYS Workbench 项目文件 (XML 格式)'
  },
  {
    name: 'ANSYS Classic 网格',
    extensions: ['.cdb'],
    description: 'ANSYS Classic 数据库文件'
  },
  {
    name: 'ABAQUS CAE 模型',
    extensions: ['.cae'],
    description: 'ABAQUS 完整模型文件 (SQLite)'
  },
  {
    name: 'Abaqus INP 输入',
    extensions: ['.inp'],
    description: 'Abaqus 求解器输入文件'
  }
]
</script>

<template>
  <div class="cae-import-panel h-full flex flex-col bg-[var(--bg-ground)]">
    <!-- 顶部栏 -->
    <div class="bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-xl font-bold text-[var(--text-primary)] flex items-center gap-2">
            <span>📥</span>
            <span>CAE 文件导入</span>
          </h1>
          <p class="text-sm text-[var(--text-muted)] mt-1">
            导入 ANSYS/ABAQUS/Abaqus 模型和网格
          </p>
        </div>
        <div class="flex items-center gap-2">
          <span class="px-3 py-1.5 bg-blue-100 text-blue-700 rounded-full text-sm">
            支持 {{ supportedFormats.length }} 种格式
          </span>
        </div>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="flex-1 overflow-y-auto p-6">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- 左侧：文件选择和预览 -->
        <div class="space-y-4">
          <!-- 文件选择区 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">📁 选择文件</h2>
            </div>
            <div class="p-6">
              <label
                class="flex flex-col items-center justify-center w-full h-40 border-2 border-dashed border-[var(--border-subtle)] rounded-xl cursor-pointer hover:bg-[var(--bg-hover)] transition-colors"
              >
                <div class="text-4xl mb-2">{{ formatIcon }}</div>
                <p class="text-sm text-[var(--text-primary)]">点击选择 CAE 文件</p>
                <p class="text-xs text-[var(--text-muted)] mt-1">
                  支持 .wbpj, .cdb, .cae, .inp 格式
                </p>
                <input
                  type="file"
                  class="hidden"
                  accept=".wbpj,.cdb,.cae,.inp"
                  @change="handleFileSelect"
                />
              </label>

              <!-- 已选文件信息 -->
              <div v-if="selectedFile" class="mt-4 p-4 bg-[var(--bg-elevated)] rounded-lg">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-lg bg-[var(--primary)]/10 flex items-center justify-center text-xl">
                      {{ formatIcon }}
                    </div>
                    <div>
                      <p class="font-medium text-[var(--text-primary)]">{{ selectedFile.name }}</p>
                      <p class="text-xs text-[var(--text-muted)]">
                        {{ formatBytes(selectedFile.size) }}
                      </p>
                    </div>
                  </div>
                  <button
                    @click="selectedFile = null; preview = null"
                    class="p-2 hover:bg-[var(--bg-hover)] rounded text-sm"
                  >
                    ✕
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- 文件预览 -->
          <div v-if="preview || isPreviewLoading" class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">
                {{ isPreviewLoading ? '⏳ 加载中...' : '📊 文件预览' }}
              </h2>
            </div>
            <div v-if="preview" class="p-4 space-y-4">
              <!-- 网格统计 -->
              <div class="grid grid-cols-3 gap-3">
                <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
                  <p class="text-2xl font-bold text-[var(--primary)]">{{ preview.nodes || '-' }}</p>
                  <p class="text-xs text-[var(--text-muted)]">节点数</p>
                </div>
                <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
                  <p class="text-2xl font-bold text-blue-600">{{ preview.elements || '-' }}</p>
                  <p class="text-xs text-[var(--text-muted)]">单元数</p>
                </div>
                <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
                  <p class="text-2xl font-bold text-green-600">{{ preview.materials || '-' }}</p>
                  <p class="text-xs text-[var(--text-muted)]">材料数</p>
                </div>
              </div>

              <!-- 几何信息 -->
              <div v-if="preview.geometryInfo" class="space-y-2">
                <p class="text-sm font-medium text-[var(--text-secondary)]">几何类型:</p>
                <div class="flex flex-wrap gap-2">
                  <span
                    v-if="preview.geometryInfo.hasSolid"
                    class="px-2 py-1 bg-green-100 text-green-700 rounded text-xs"
                  >
                    ✅ 实体
                  </span>
                  <span
                    v-if="preview.geometryInfo.hasShell"
                    class="px-2 py-1 bg-blue-100 text-blue-700 rounded text-xs"
                  >
                    🟡 壳
                  </span>
                  <span
                    v-if="preview.geometryInfo.hasBeam"
                    class="px-2 py-1 bg-purple-100 text-purple-700 rounded text-xs"
                  >
                    🔵 梁
                  </span>
                  <span class="px-2 py-1 bg-[var(--bg-elevated)] text-[var(--text-muted)] rounded text-xs">
                    {{ preview.geometryInfo.partCount }} 个零件
                  </span>
                </div>
              </div>

              <!-- 警告信息 -->
              <div v-if="preview.warnings?.length" class="space-y-2">
                <p class="text-sm font-medium text-[var(--text-secondary)]">⚠️ 提示:</p>
                <div class="space-y-1">
                  <p
                    v-for="warning in preview.warnings"
                    :key="warning"
                    class="text-xs text-yellow-600"
                  >
                    {{ warning }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 右侧：支持的格式和导入历史 -->
        <div class="space-y-4">
          <!-- 支持的格式 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
              <h2 class="font-semibold text-[var(--text-primary)]">📋 支持的格式</h2>
            </div>
            <div class="divide-y divide-[var(--border-subtle)]">
              <div
                v-for="format in supportedFormats"
                :key="format.ext"
                class="p-4"
              >
                <div class="flex items-start gap-3">
                  <div class="w-8 h-8 rounded bg-[var(--primary)]/10 flex items-center justify-center text-sm">
                    📄
                  </div>
                  <div>
                    <p class="font-medium text-[var(--text-primary)]">{{ format.name }}</p>
                    <p class="text-xs text-[var(--text-muted)] mt-0.5">{{ format.desc }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 导入历史 -->
          <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden">
            <div class="px-4 py-3 border-b border-[var(--border-subtle)] flex items-center justify-between">
              <h2 class="font-semibold text-[var(--text-primary)]">📜 导入历史</h2>
              <button
                v-if="importHistory.length > 0"
                @click="importHistory = []"
                class="text-xs text-[var(--text-muted)] hover:text-red-600"
              >
                清空
              </button>
            </div>
            <div v-if="importHistory.length === 0" class="p-8 text-center text-[var(--text-muted)]">
              暂无导入记录
            </div>
            <div v-else class="divide-y divide-[var(--border-subtle)]">
              <div
                v-for="(item, idx) in importHistory.slice(0, 10)"
                :key="idx"
                class="p-3 flex items-center justify-between"
              >
                <div class="flex items-center gap-3">
                  <span
                    class="text-lg"
                    :class="item.success ? 'text-green-600' : 'text-red-600'"
                  >
                    {{ item.success ? '✅' : '❌' }}
                  </span>
                  <div>
                    <p class="text-sm text-[var(--text-primary)]">{{ item.fileName }}</p>
                    <p class="text-xs text-[var(--text-muted)]">{{ item.date }}</p>
                  </div>
                </div>
                <span class="text-xs text-[var(--text-muted)]">{{ item.format.toUpperCase() }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部操作栏 -->
    <div class="bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div v-if="importError" class="flex items-center gap-2 text-red-600">
            <span>❌</span>
            <span class="text-sm">{{ importError }}</span>
          </div>
          <div v-if="importResult?.success" class="flex items-center gap-2 text-green-600">
            <span>✅</span>
            <span class="text-sm">导入成功</span>
          </div>
        </div>
        <div class="flex items-center gap-3">
          <button
            @click="selectedFile = null; preview = null; clearResult()"
            class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)]"
          >
            取消
          </button>
          <button
            @click="handleImport"
            :disabled="!canImport"
            class="px-6 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 disabled:opacity-50"
          >
            {{ isImporting ? '导入中...' : '📥 导入文件' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>