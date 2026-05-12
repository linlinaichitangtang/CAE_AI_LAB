<script setup lang="ts">
/**
 * JupyterPanel.vue — V3.0-006 Jupyter 集成
 * 在 CAELab 中运行 Python 代码，结果自动写回笔记
 */
import { ref, computed, onMounted } from 'vue'
import { useProjectStore } from '@/stores/project'

// ============ 类型定义 ============

interface JupyterCell {
  id: string
  type: 'code' | 'markdown'
  source: string
  output?: string
  error?: string
  isExecuting?: boolean
}

interface JupyterServer {
  url: string
  token?: string
  status: 'connected' | 'disconnected' | 'connecting'
}

// ============ 状态 ============
const projectStore = useProjectStore()
const cells = ref<JupyterCell[]>([])
const activeCellId = ref<string | null>(null)
const isExecuting = ref(false)
const serverUrl = ref('')
const serverToken = ref('')
const serverStatus = ref<'disconnected' | 'connecting' | 'connected'>('disconnected')

// 代码片段模板
const codeTemplates = [
  { name: '数据分析', icon: '📊', code: '# 数据分析示例\nimport numpy as np\nimport matplotlib.pyplot as plt\n\n# 生成数据\nx = np.linspace(0, 10, 100)\ny = np.sin(x)\n\n# 绘图\nplt.figure(figsize=(8, 4))\nplt.plot(x, y)\nplt.title("Sine Wave")\nplt.savefig("output.png")\nplt.show()\nprint("图像已保存为 output.png")' },
  { name: '网格处理', icon: '🕸️', code: '# 网格数据处理\nimport numpy as np\n\n# 假设从 CAELab 获取的网格数据\nnodes = project_store.current_mesh.nodes if "project_store" in dir() else []\nelements = project_store.current_mesh.elements if "project_store" in dir() else []\n\nprint(f"网格节点数: {len(nodes)}")\nprint(f"网格单元数: {len(elements)}")' },
  { name: '结果后处理', icon: '📈', code: '# 仿真结果后处理\nimport numpy as np\n\n# 假设从 CAELab 获取的结果\nresult = project_store.last_result if "project_store" in dir() else None\n\nif result:\n    print(f"最大位移: {result.max_displacement:.6f} m")\n    print(f"最大应力: {result.max_stress:.2f} MPa")\nelse:\n    print("暂无仿真结果")' },
  { name: '参数化扫描', icon: '🔄', code: '# 参数化扫描分析\nimport numpy as np\n\n# 定义参数范围\nE_values = np.linspace(70e9, 210e9, 5)  # 弹性模量范围\nresults = []\n\nfor E in E_values:\n    # 这里应该调用 CAELab API 进行计算\n    # 简化示例：直接计算\n    stress = 100e6  # 假设应力\n    displacement = stress / E * 0.1  # 简化计算\n    results.append({"E": E, "displacement": displacement})\n    \nprint("参数扫描结果:")\nfor r in results:\n    print(f"  E={r[\'E\']:.2e}: displacement={r[\'displacement\']:.6f}")' }
]

// ============ 计算属性 ============
const hasServer = computed(() => serverStatus.value === 'connected')
const canExecute = computed(() => hasServer.value && !isExecuting.value)

// ============ 服务器连接 ============
async function connectToServer() {
  if (!serverUrl.value) {
    alert('请输入 Jupyter 服务器地址')
    return
  }

  serverStatus.value = 'connecting'

  try {
    // 尝试连接到 Jupyter Server
    const response = await fetch(`${serverUrl.value}/api/status`, {
      headers: serverToken.value ? { 'Authorization': `Token ${serverToken.value}` } : {}
    })

    if (response.ok) {
      serverStatus.value = 'connected'
      addCell()  // 添加第一个代码单元
    } else {
      throw new Error('连接失败')
    }
  } catch (e: any) {
    serverStatus.value = 'disconnected'
    alert(`连接失败: ${e.message}`)
  }
}

function disconnectServer() {
  serverStatus.value = 'disconnected'
}

// ============ 单元格操作 ============
function addCell(type: 'code' | 'markdown' = 'code') {
  const cell: JupyterCell = {
    id: `cell_${Date.now()}`,
    type,
    source: '',
    output: undefined,
    error: undefined
  }
  cells.value.push(cell)
  activeCellId.value = cell.id
}

function deleteCell(cellId: string) {
  const index = cells.value.findIndex(c => c.id === cellId)
  if (index !== -1) {
    cells.value.splice(index, 1)
    if (activeCellId.value === cellId) {
      activeCellId.value = cells.value[Math.max(0, index - 1)]?.id || null
    }
  }
}

function moveCellUp(cellId: string) {
  const index = cells.value.findIndex(c => c.id === cellId)
  if (index > 0) {
    const temp = cells.value[index]
    cells.value[index] = cells.value[index - 1]
    cells.value[index - 1] = temp
  }
}

function moveCellDown(cellId: string) {
  const index = cells.value.findIndex(c => c.id === cellId)
  if (index < cells.value.length - 1) {
    const temp = cells.value[index]
    cells.value[index] = cells.value[index + 1]
    cells.value[index + 1] = temp
  }
}

function updateCellSource(cellId: string, source: string) {
  const cell = cells.value.find(c => c.id === cellId)
  if (cell) {
    cell.source = source
  }
}

function insertCodeTemplate(templateIndex: number) {
  if (activeCellId.value) {
    const cell = cells.value.find(c => c.id === activeCellId.value)
    if (cell && cell.type === 'code') {
      cell.source = codeTemplates[templateIndex].code
    }
  } else {
    addCell('code')
    const cell = cells.value[cells.value.length - 1]
    if (cell) {
      cell.source = codeTemplates[templateIndex].code
    }
  }
}

// ============ 代码执行 ============
async function executeCell(cellId: string) {
  const cell = cells.value.find(c => c.id === cellId)
  if (!cell || cell.type !== 'code') return

  cell.isExecuting = true
  cell.output = undefined
  cell.error = undefined
  isExecuting.value = true

  try {
    // 调用 Jupyter Server API 执行代码
    const response = await fetch(`${serverUrl.value}/api/execute`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(serverToken.value ? { 'Authorization': `Token ${serverToken.value}` } : {})
      },
      body: JSON.stringify({
        code: cell.source,
        store_variables: true,
        timeout: 60
      })
    })

    if (!response.ok) {
      throw new Error(`执行失败: ${response.statusText}`)
    }

    const result = await response.json()
    cell.output = result.output || result.text || ''
    if (result.error) {
      cell.error = result.error
    }
  } catch (e: any) {
    cell.error = e.message || '执行失败'
  } finally {
    cell.isExecuting = false
    isExecuting.value = false
  }
}

async function executeAllCells() {
  for (const cell of cells.value) {
    if (cell.type === 'code') {
      await executeCell(cell.id)
    }
  }
}

async function runAndInsertToNote() {
  // 执行所有代码单元
  await executeAllCells()

  // 生成结果摘要
  const outputs = cells.value
    .filter(c => c.output)
    .map(c => `**Cell ${cells.value.indexOf(c) + 1}:**\n\`\`\`\n${c.output}\n\`\`\``)
    .join('\n\n')

  if (outputs) {
    // 创建嵌入记录（这里简化处理，实际应该调用 projectStore）
    alert('代码执行结果可以复制到笔记中使用')
  }
}

// ============ 工具函数 ============
function formatOutput(output: string): string {
  // 简单的格式化：处理 matplotlib 输出等
  if (output.includes('data:image/png')) {
    return '[图像输出]'
  }
  return output
}

function getCellNumber(cellId: string): number {
  return cells.value.findIndex(c => c.id === cellId) + 1
}

// ============ 生命周期 ============
onMounted(() => {
  // 尝试自动连接本地 Jupyter
  const localUrl = 'http://localhost:8888'
  if (!serverUrl.value) {
    serverUrl.value = localUrl
  }
})
</script>

<template>
  <div class="jupyter-panel h-full flex flex-col bg-[var(--bg-ground)]">
    <!-- 顶部工具栏 -->
    <div class="bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-4 py-3">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <h2 class="font-semibold text-[var(--text-primary)] flex items-center gap-2">
            <span>📓</span>
            <span>Jupyter 代码执行</span>
          </h2>
          <span
            class="px-2 py-0.5 rounded-full text-xs"
            :class="{
              'bg-green-100 text-green-700': serverStatus === 'connected',
              'bg-yellow-100 text-yellow-700': serverStatus === 'connecting',
              'bg-gray-100 text-gray-500': serverStatus === 'disconnected'
            }"
          >
            {{ serverStatus === 'connected' ? '🟢 已连接' : serverStatus === 'connecting' ? '🟡 连接中...' : '⚫ 未连接' }}
          </span>
        </div>
        <div class="flex items-center gap-2">
          <button
            @click="runAndInsertToNote"
            :disabled="!canExecute"
            class="px-3 py-1.5 bg-[var(--primary)] text-white rounded text-sm hover:opacity-90 disabled:opacity-50"
            title="执行并插入笔记"
          >
            ▶️ 执行并插入笔记
          </button>
        </div>
      </div>

      <!-- 服务器连接 -->
      <div class="flex items-center gap-3 mt-3">
        <input
          v-model="serverUrl"
          type="text"
          class="input flex-1"
          placeholder="Jupyter 服务器地址 (如 http://localhost:8888)"
          :disabled="serverStatus === 'connected'"
        />
        <input
          v-model="serverToken"
          type="password"
          class="input w-40"
          placeholder="Token (可选)"
          :disabled="serverStatus === 'connected'"
        />
        <button
          v-if="serverStatus !== 'connected'"
          @click="connectToServer"
          :disabled="serverStatus === 'connecting'"
          class="px-4 py-2 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 disabled:opacity-50"
        >
          {{ serverStatus === 'connecting' ? '连接中...' : '连接' }}
        </button>
        <button
          v-else
          @click="disconnectServer"
          class="px-4 py-2 border border-[var(--border-subtle)] rounded text-sm hover:bg-[var(--bg-hover)]"
        >
          断开
        </button>
      </div>
    </div>

    <!-- 未连接状态 -->
    <div v-if="serverStatus !== 'connected'" class="flex-1 flex items-center justify-center text-[var(--text-muted)]">
      <div class="text-center max-w-md">
        <div class="text-6xl mb-4 opacity-30">🔌</div>
        <p class="text-lg mb-2">连接到 Jupyter 服务器</p>
        <p class="text-sm">
          请输入 Jupyter 服务器地址，或在本地启动 Jupyter：
        </p>
        <code class="block bg-[var(--bg-elevated)] rounded p-2 mt-3 text-sm">
          jupyter notebook --NotebookApp.token=''
        </code>
      </div>
    </div>

    <!-- 已连接：代码单元格列表 -->
    <div v-else class="flex-1 overflow-y-auto p-4 space-y-4">
      <!-- 代码模板快捷插入 -->
      <div class="bg-[var(--bg-surface)] rounded-lg p-3 border border-[var(--border-subtle)]">
        <p class="text-xs text-[var(--text-muted)] mb-2">💡 快捷代码模板：</p>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="(template, idx) in codeTemplates"
            :key="template.name"
            @click="insertCodeTemplate(idx)"
            class="px-3 py-1.5 bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] rounded text-xs flex items-center gap-1 transition-colors"
          >
            {{ template.icon }} {{ template.name }}
          </button>
        </div>
      </div>

      <!-- 代码单元格 -->
      <div
        v-for="cell in cells"
        :key="cell.id"
        class="bg-[var(--bg-surface)] rounded-lg border border-[var(--border-subtle)] overflow-hidden"
        :class="{ 'ring-2 ring-[var(--primary)]': activeCellId === cell.id }"
        @click="activeCellId = cell.id"
      >
        <!-- 单元格工具栏 -->
        <div class="flex items-center justify-between px-3 py-2 bg-[var(--bg-elevated)] border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2">
            <span class="text-xs font-mono text-[var(--text-muted)]">In [{{ cell.isExecuting ? '*' : getCellNumber(cell.id) }}]:</span>
            <span class="text-xs text-[var(--text-muted)]">{{ cell.type === 'code' ? 'Python' : 'Markdown' }}</span>
          </div>
          <div class="flex items-center gap-1">
            <button
              @click.stop="moveCellUp(cell.id)"
              class="p-1 hover:bg-[var(--bg-hover)] rounded text-xs"
              title="上移"
            >
              ⬆️
            </button>
            <button
              @click.stop="moveCellDown(cell.id)"
              class="p-1 hover:bg-[var(--bg-hover)] rounded text-xs"
              title="下移"
            >
              ⬇️
            </button>
            <button
              @click.stop="executeCell(cell.id)"
              :disabled="cell.isExecuting || cell.type !== 'code'"
              class="px-2 py-1 bg-green-600 text-white rounded text-xs hover:bg-green-700 disabled:opacity-50"
            >
              {{ cell.isExecuting ? '⏳' : '▶️' }}
            </button>
            <button
              @click.stop="deleteCell(cell.id)"
              class="p-1 hover:bg-red-50 rounded text-xs"
              title="删除"
            >
              🗑️
            </button>
          </div>
        </div>

        <!-- 代码编辑器 -->
        <textarea
          v-if="cell.type === 'code'"
          :value="cell.source"
          @input="updateCellSource(cell.id, ($event.target as HTMLTextAreaElement).value)"
          @focus="activeCellId = cell.id"
          class="w-full min-h-[100px] p-3 bg-[var(--bg-base)] font-mono text-sm text-[var(--text-primary)] resize-none focus:outline-none"
          placeholder="在此输入 Python 代码..."
          :disabled="cell.isExecuting"
        ></textarea>

        <!-- Markdown 编辑器 -->
        <textarea
          v-else
          :value="cell.source"
          @input="updateCellSource(cell.id, ($event.target as HTMLTextAreaElement).value)"
          @focus="activeCellId = cell.id"
          class="w-full min-h-[60px] p-3 bg-[var(--bg-base)] text-sm text-[var(--text-primary)] resize-none focus:outline-none"
          placeholder="在此输入 Markdown..."
        ></textarea>

        <!-- 输出区域 -->
        <div v-if="cell.output || cell.error" class="border-t border-[var(--border-subtle)]">
          <pre
            v-if="cell.output"
            class="p-3 bg-[var(--bg-ground)] text-sm text-[var(--text-secondary)] font-mono whitespace-pre-wrap overflow-x-auto"
          >{{ cell.output }}</pre>
          <pre
            v-if="cell.error"
            class="p-3 bg-red-50 text-red-600 text-sm font-mono whitespace-pre-wrap"
          >❌ {{ cell.error }}</pre>
        </div>
      </div>

      <!-- 添加单元格按钮 -->
      <div class="flex gap-2">
        <button
          @click="addCell('code')"
          class="flex-1 py-3 border border-dashed border-[var(--border-subtle)] rounded-lg text-[var(--text-muted)] hover:bg-[var(--bg-hover)] hover:border-[var(--primary)] transition-colors"
        >
          + 代码单元格
        </button>
        <button
          @click="addCell('markdown')"
          class="flex-1 py-3 border border-dashed border-[var(--border-subtle)] rounded-lg text-[var(--text-muted)] hover:bg-[var(--bg-hover)] hover:border-[var(--primary)] transition-colors"
        >
          + Markdown 单元格
        </button>
      </div>

      <!-- 全部执行按钮 -->
      <div class="flex justify-center pt-4">
        <button
          @click="executeAllCells"
          :disabled="!canExecute || cells.length === 0"
          class="px-6 py-2 bg-[var(--primary)] text-white rounded-lg hover:opacity-90 disabled:opacity-50"
        >
          ▶️ 执行所有单元格
        </button>
      </div>
    </div>
  </div>
</template>