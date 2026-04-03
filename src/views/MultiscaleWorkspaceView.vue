<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度工作区</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.9-010 | DFT/MD/相场/有限元单一工作区，任务编排与数据桥接</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="refreshProjects">刷新项目</button>
        <button class="btn btn-primary text-xs" @click="scrollToGraph">查看任务图</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Project List -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            项目列表
          </h4>
          <div class="space-y-2">
            <div
              v-for="proj in projects"
              :key="proj.project_id"
              class="p-2.5 rounded border cursor-pointer transition"
              :style="selectedProjectId === proj.project_id
                ? 'background: var(--bg-elevated); border-color: var(--primary)'
                : 'background: var(--bg-surface); border-color: var(--border-subtle)'"
              @click="selectProject(proj.project_id)"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium truncate" style="color: var(--text-primary)">{{ proj.name }}</span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded-full font-medium"
                  :style="projectBadgeStyle(proj.status)"
                >{{ projectStatusLabel(proj.status) }}</span>
              </div>
              <div class="flex items-center justify-between text-[10px]" style="color: var(--text-muted)">
                <span>{{ proj.tasks.length }} 个任务</span>
                <span>{{ formatDate(proj.created_at) }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 2: New Project -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            新建项目
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">项目名称</label>
              <input class="input w-full text-xs" type="text" v-model="newProject.name" placeholder="输入项目名称" />
            </div>
            <div>
              <label class="label">描述</label>
              <input class="input w-full text-xs" type="text" v-model="newProject.description" placeholder="项目描述" />
            </div>
            <div>
              <label class="label">模板 (可选)</label>
              <select class="input w-full text-xs" v-model="newProject.template_id">
                <option value="">无模板</option>
                <option value="dft-md-fe">DFT-MD-FE 链式</option>
                <option value="md-pf">MD-相场 耦合</option>
                <option value="full-chain">全尺度链式</option>
              </select>
            </div>
            <button class="btn btn-primary text-xs w-full" @click="createProject">新建项目</button>
          </div>
        </div>

        <!-- Step 3: Selected Project Tasks -->
        <div v-if="selectedProject">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            任务列表
          </h4>
          <div class="space-y-1.5">
            <div
              v-for="task in selectedProject.tasks"
              :key="task.task_id"
              class="p-2 rounded border"
              style="background: var(--bg-elevated); border-color: var(--border-subtle)"
            >
              <div class="flex items-center gap-2 mb-1">
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded font-medium"
                  :style="scaleBadgeStyle(task.scale)"
                >{{ scaleLabel(task.scale) }}</span>
                <span class="text-xs font-medium flex-1 truncate" style="color: var(--text-primary)">{{ task.name }}</span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded-full font-medium"
                  :style="taskStatusBadgeStyle(task.status)"
                >{{ taskStatusLabel(task.status) }}</span>
              </div>
              <div class="flex items-center gap-2 text-[10px]" style="color: var(--text-muted)">
                <span v-if="task.bridge_from">from: {{ taskBridgeName(task.bridge_from) }}</span>
                <span v-if="task.bridge_to">to: {{ taskBridgeName(task.bridge_to) }}</span>
                <span v-if="!task.bridge_from && !task.bridge_to">无桥接</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: Add Task & Connect -->
        <div v-if="selectedProject">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            任务操作
          </h4>
          <div class="space-y-2">
            <div class="flex gap-2">
              <select class="input text-xs flex-1" v-model="newTask.scale">
                <option value="dft">DFT</option>
                <option value="md">MD</option>
                <option value="phase_field">相场</option>
                <option value="fe">FE</option>
              </select>
              <input class="input text-xs flex-1" type="text" v-model="newTask.name" placeholder="任务名称" />
            </div>
            <button class="btn btn-ghost text-xs w-full" @click="addTask">添加任务</button>

            <div class="border-t pt-2" style="border-color: var(--border-subtle)">
              <div class="flex gap-2">
                <select class="input text-xs flex-1" v-model="connection.from_task_id">
                  <option value="">来源任务</option>
                  <option v-for="t in selectedProject.tasks" :key="t.task_id" :value="t.task_id">{{ t.name }}</option>
                </select>
                <select class="input text-xs flex-1" v-model="connection.to_task_id">
                  <option value="">目标任务</option>
                  <option v-for="t in selectedProject.tasks" :key="t.task_id" :value="t.task_id">{{ t.name }}</option>
                </select>
              </div>
              <button class="btn btn-ghost text-xs w-full mt-2" @click="connectTasks">连接任务</button>
            </div>
          </div>
        </div>

        <!-- Step 5: Run Project -->
        <div v-if="selectedProject">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            运行项目
          </h4>
          <button class="btn btn-primary text-xs w-full" :disabled="isRunning" @click="runProject">
            {{ isRunning ? '运行中...' : '运行项目' }}
          </button>
          <div v-if="selectedProject" class="mt-2 p-2 rounded text-center" style="background: var(--bg-elevated)">
            <div class="flex items-center justify-between text-[10px]">
              <span style="color: var(--text-muted)">任务进度</span>
              <span style="color: var(--text-secondary)">{{ completedCount }}/{{ selectedProject.tasks.length }}</span>
            </div>
            <div class="w-full h-1.5 rounded-full mt-1 overflow-hidden" style="background: var(--border-subtle)">
              <div
                class="h-full rounded-full transition-all duration-500"
                :style="{ width: progressPercent + '%', background: 'var(--primary)' }"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="graphRef">

        <!-- Task Graph Visualization -->
        <div v-if="selectedProject">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">任务图可视化</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" :height="graphHeight" :viewBox="`0 0 ${graphWidth} ${graphHeight}`">
              <!-- Edges (arrows) -->
              <defs>
                <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                  <polygon points="0 0, 8 3, 0 6" fill="var(--text-muted)" />
                </marker>
              </defs>
              <g v-for="(edge, ei) in taskEdges" :key="'e-' + ei">
                <line
                  :x1="nodePosition(edge.from).x"
                  :y1="nodePosition(edge.from).y"
                  :x2="nodePosition(edge.to).x"
                  :y2="nodePosition(edge.to).y"
                  stroke="var(--text-muted)"
                  stroke-width="2"
                  stroke-dasharray="6,3"
                  marker-end="url(#arrowhead)"
                />
                <text
                  :x="(nodePosition(edge.from).x + nodePosition(edge.to).x) / 2"
                  :y="(nodePosition(edge.from).y + nodePosition(edge.to).y) / 2 - 8"
                  text-anchor="middle"
                  fill="var(--text-muted)"
                  font-size="9"
                >{{ edge.data_flow }}</text>
              </g>
              <!-- Nodes -->
              <g v-for="node in taskNodes" :key="node.id">
                <circle
                  :cx="nodePosition(node.id).x"
                  :cy="nodePosition(node.id).y"
                  r="28"
                  :fill="scaleFillColor(node.scale)"
                  :stroke="nodeStatusStroke(node.status)"
                  stroke-width="3"
                />
                <text
                  :x="nodePosition(node.id).x"
                  :y="nodePosition(node.id).y - 4"
                  text-anchor="middle"
                  fill="#fff"
                  font-size="11"
                  font-weight="600"
                >{{ scaleLabel(node.scale) }}</text>
                <text
                  :x="nodePosition(node.id).x"
                  :y="nodePosition(node.id).y + 10"
                  text-anchor="middle"
                  fill="rgba(255,255,255,0.8)"
                  font-size="8"
                >{{ node.id }}</text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Project Progress Cards -->
        <div v-if="selectedProject">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">项目进度</h4>
          <div class="grid grid-cols-5 gap-3">
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--primary)">{{ selectedProject.tasks.length }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">总任务</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--accent-green)">{{ completedCount }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">已完成</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--primary)">{{ runningCount }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">运行中</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--accent-red)">{{ failedCount }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">失败</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--text-primary)">{{ sharedDataSizeMB }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">共享数据 (MB)</div>
            </div>
          </div>
        </div>

        <!-- Shared Data List -->
        <div v-if="selectedProject">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">共享数据</h4>
          <div class="rounded border overflow-hidden" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <table class="w-full text-xs">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="text-left px-3 py-2 font-medium" style="color: var(--text-secondary)">名称</th>
                  <th class="text-left px-3 py-2 font-medium" style="color: var(--text-secondary)">类型</th>
                  <th class="text-left px-3 py-2 font-medium" style="color: var(--text-secondary)">大小</th>
                  <th class="text-left px-3 py-2 font-medium" style="color: var(--text-secondary)">来源任务</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(data, di) in selectedProject.shared_data"
                  :key="di"
                  class="border-t"
                  style="border-color: var(--border-subtle)"
                >
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ data.name }}</td>
                  <td class="px-3 py-2" style="color: var(--text-secondary)">{{ data.type }}</td>
                  <td class="px-3 py-2" style="color: var(--text-secondary)">{{ formatSize(data.size_bytes) }}</td>
                  <td class="px-3 py-2" style="color: var(--text-muted)">{{ taskBridgeName(data.source_task_id) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Empty State -->
        <div v-if="!selectedProject" class="flex items-center justify-center h-64">
          <div class="text-center">
            <p class="text-sm" style="color: var(--text-muted)">请选择或创建一个项目</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { TaskScale, ProjectTask, MultiscaleProject, ProjectDashboard } from '../api/multiscaleWorkspace'

// ============ Refs ============
const graphRef = ref<HTMLElement | null>(null)
const isRunning = ref(false)
const selectedProjectId = ref<string | null>(null)
const projects = ref<MultiscaleProject[]>([])

// ============ New Project Form ============
const newProject = reactive({
  name: '',
  description: '',
  template_id: ''
})

// ============ New Task Form ============
const newTask = reactive({
  scale: 'md' as TaskScale,
  name: ''
})

// ============ Connection Form ============
const connection = reactive({
  from_task_id: '',
  to_task_id: ''
})

// ============ Graph Dimensions ============
const graphWidth = 640
const graphHeight = 200

// ============ Mock Data ============
function generateMockProjects(): MultiscaleProject[] {
  const tasks: ProjectTask[] = [
    {
      task_id: 'task-dft-001',
      scale: 'dft' as TaskScale,
      name: 'DFT 弹性常数计算',
      status: 'completed' as 'completed',
      created_at: new Date(Date.now() - 86400000 * 5).toISOString(),
      updated_at: new Date(Date.now() - 86400000 * 4).toISOString(),
      parameters: { k_points: '12x12x12', xc_functional: 'PBE' },
      result_summary: { c11: 530, c12: 180, c44: 120 },
      bridge_to: 'task-md-001',
      bridge_from: null
    },
    {
      task_id: 'task-md-001',
      scale: 'md' as TaskScale,
      name: 'MD 分子动力学模拟',
      status: 'completed' as 'completed',
      created_at: new Date(Date.now() - 86400000 * 4).toISOString(),
      updated_at: new Date(Date.now() - 86400000 * 3).toISOString(),
      parameters: { ensemble: 'NPT', temperature: 300, potential: 'EAM' },
      result_summary: { lattice_constant: 3.615, cohesive_energy: -3.54 },
      bridge_to: 'task-pf-001',
      bridge_from: 'task-dft-001'
    },
    {
      task_id: 'task-pf-001',
      scale: 'phase_field' as TaskScale,
      name: '相场裂纹扩展模拟',
      status: 'running' as 'running',
      created_at: new Date(Date.now() - 86400000 * 3).toISOString(),
      updated_at: new Date(Date.now() - 3600000).toISOString(),
      parameters: { grid_size: '256x256', interface_width: 2.0 },
      result_summary: null,
      bridge_to: 'task-fe-001',
      bridge_from: 'task-md-001'
    },
    {
      task_id: 'task-fe-001',
      scale: 'fe' as TaskScale,
      name: 'FE 宏观拉伸模拟',
      status: 'created' as 'created',
      created_at: new Date(Date.now() - 86400000 * 2).toISOString(),
      updated_at: new Date(Date.now() - 86400000 * 2).toISOString(),
      parameters: { element_type: 'C3D8', mesh_size: 0.5 },
      result_summary: null,
      bridge_to: null,
      bridge_from: 'task-pf-001'
    }
  ]

  const sharedData = [
    { name: '弹性常数张量', type: 'tensor', size_bytes: 2048, source_task_id: 'task-dft-001' },
    { name: 'EAM 势参数', type: 'potential', size_bytes: 51200, source_task_id: 'task-dft-001' },
    { name: '晶格常数', type: 'scalar', size_bytes: 64, source_task_id: 'task-md-001' },
    { name: '断裂能', type: 'scalar', size_bytes: 64, source_task_id: 'task-md-001' },
    { name: '应力-应变曲线', type: 'curve', size_bytes: 102400, source_task_id: 'task-pf-001' },
    { name: '裂纹路径', type: 'geometry', size_bytes: 256000, source_task_id: 'task-pf-001' }
  ]

  return [
    {
      project_id: 'proj-multi-001',
      name: '多尺度材料性能预测',
      description: 'DFT-MD-相场-FE 全尺度链式模拟，预测高强钢力学性能',
      created_at: new Date(Date.now() - 86400000 * 5).toISOString(),
      updated_at: new Date(Date.now() - 3600000).toISOString(),
      tasks,
      shared_data: sharedData,
      status: 'active' as 'active'
    }
  ]
}

// ============ Computed ============
const selectedProject = computed<MultiscaleProject | null>(() => {
  if (!selectedProjectId.value) return null
  return projects.value.find(p => p.project_id === selectedProjectId.value) || null
})

const completedCount = computed(() => {
  if (!selectedProject.value) return 0
  return selectedProject.value.tasks.filter(t => t.status === 'completed').length
})

const runningCount = computed(() => {
  if (!selectedProject.value) return 0
  return selectedProject.value.tasks.filter(t => t.status === 'running').length
})

const failedCount = computed(() => {
  if (!selectedProject.value) return 0
  return selectedProject.value.tasks.filter(t => t.status === 'failed').length
})

const progressPercent = computed(() => {
  if (!selectedProject.value || selectedProject.value.tasks.length === 0) return 0
  return Math.round((completedCount.value / selectedProject.value.tasks.length) * 100)
})

const sharedDataSizeMB = computed(() => {
  if (!selectedProject.value) return 0
  const totalBytes = selectedProject.value.shared_data.reduce((s, d) => s + d.size_bytes, 0)
  return (totalBytes / (1024 * 1024)).toFixed(2)
})

const taskNodes = computed(() => {
  if (!selectedProject.value) return []
  return selectedProject.value.tasks.map(t => ({
    id: t.task_id,
    scale: t.scale,
    status: t.status
  }))
})

const taskEdges = computed(() => {
  if (!selectedProject.value) return []
  const edges: Array<{ from: string; to: string; data_flow: string }> = []
  for (const task of selectedProject.value.tasks) {
    if (task.bridge_to) {
      edges.push({
        from: task.task_id,
        to: task.bridge_to,
        data_flow: 'data'
      })
    }
  }
  return edges
})

// ============ Node Positioning ============
function nodePosition(taskId: string): { x: number; y: number } {
  const positions: Record<string, { x: number; y: number }> = {
    'task-dft-001': { x: 80, y: 100 },
    'task-md-001': { x: 240, y: 100 },
    'task-pf-001': { x: 400, y: 100 },
    'task-fe-001': { x: 560, y: 100 }
  }
  return positions[taskId] || { x: 320, y: 100 }
}

// ============ Methods ============
function scaleLabel(scale: TaskScale): string {
  const map: Record<TaskScale, string> = {
    dft: 'DFT',
    md: 'MD',
    phase_field: 'PF',
    fe: 'FE'
  }
  return map[scale]
}

function scaleFillColor(scale: TaskScale): string {
  const map: Record<TaskScale, string> = {
    dft: '#3b82f6',
    md: '#22c55e',
    phase_field: '#f97316',
    fe: '#a855f7'
  }
  return map[scale]
}

function nodeStatusStroke(status: string): string {
  const map: Record<string, string> = {
    completed: 'var(--accent-green)',
    running: 'var(--primary)',
    failed: 'var(--accent-red)',
    created: 'var(--text-muted)',
    paused: 'var(--accent-yellow)'
  }
  return map[status] || 'var(--text-muted)'
}

function taskStatusLabel(status: string): string {
  const map: Record<string, string> = {
    completed: '已完成',
    running: '运行中',
    failed: '失败',
    created: '待运行',
    paused: '已暂停'
  }
  return map[status] || status
}

function taskStatusBadgeStyle(status: string) {
  const map: Record<string, string> = {
    completed: 'background: rgba(34,197,94,0.15); color: var(--accent-green)',
    running: 'background: rgba(59,130,246,0.15); color: var(--primary)',
    failed: 'background: rgba(239,68,68,0.15); color: var(--accent-red)',
    created: 'background: rgba(156,163,175,0.15); color: var(--text-muted)',
    paused: 'background: rgba(234,179,8,0.15); color: var(--accent-yellow)'
  }
  return map[status] || ''
}

function scaleBadgeStyle(scale: TaskScale) {
  const map: Record<TaskScale, string> = {
    dft: 'background: rgba(59,130,246,0.15); color: #3b82f6',
    md: 'background: rgba(34,197,94,0.15); color: #22c55e',
    phase_field: 'background: rgba(249,115,22,0.15); color: #f97316',
    fe: 'background: rgba(168,85,247,0.15); color: #a855f7'
  }
  return map[scale]
}

function projectStatusLabel(status: string): string {
  const map: Record<string, string> = {
    active: '活跃',
    archived: '已归档',
    completed: '已完成'
  }
  return map[status] || status
}

function projectBadgeStyle(status: string) {
  const map: Record<string, string> = {
    active: 'background: rgba(34,197,94,0.15); color: var(--accent-green)',
    archived: 'background: rgba(156,163,175,0.15); color: var(--text-muted)',
    completed: 'background: rgba(59,130,246,0.15); color: var(--primary)'
  }
  return map[status] || ''
}

function taskBridgeName(taskId: string): string {
  if (!selectedProject.value) return taskId
  const task = selectedProject.value.tasks.find(t => t.task_id === taskId)
  return task ? task.name : taskId
}

function formatDate(iso: string): string {
  const d = new Date(iso)
  return `${d.getMonth() + 1}/${d.getDate()}`
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

function selectProject(projectId: string) {
  selectedProjectId.value = projectId
}

function createProject() {
  if (!newProject.name.trim()) return

  const project: MultiscaleProject = {
    project_id: `proj-${Date.now().toString().slice(-6)}`,
    name: newProject.name,
    description: newProject.description,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    tasks: [],
    shared_data: [],
    status: 'active' as 'active'
  }

  projects.value.push(project)
  selectedProjectId.value = project.project_id
  newProject.name = ''
  newProject.description = ''
  newProject.template_id = ''
}

function addTask() {
  if (!selectedProject.value || !newTask.name.trim()) return

  const task: ProjectTask = {
    task_id: `task-${Date.now().toString().slice(-6)}`,
    scale: newTask.scale,
    name: newTask.name,
    status: 'created' as 'created',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    parameters: {},
    result_summary: null,
    bridge_to: null,
    bridge_from: null
  }

  selectedProject.value.tasks.push(task)
  newTask.name = ''
}

function connectTasks() {
  if (!selectedProject.value || !connection.from_task_id || !connection.to_task_id) return
  if (connection.from_task_id === connection.to_task_id) return

  const fromTask = selectedProject.value.tasks.find(t => t.task_id === connection.from_task_id)
  if (fromTask) {
    fromTask.bridge_to = connection.to_task_id
  }

  const toTask = selectedProject.value.tasks.find(t => t.task_id === connection.to_task_id)
  if (toTask) {
    toTask.bridge_from = connection.from_task_id
  }

  connection.from_task_id = ''
  connection.to_task_id = ''
}

async function runProject() {
  if (!selectedProject.value) return
  isRunning.value = true

  await nextTick()

  // Simulate running tasks sequentially
  const pendingTasks = selectedProject.value.tasks.filter(t => t.status === 'created')
  for (const task of pendingTasks) {
    task.status = 'running' as 'running'
    await new Promise(resolve => setTimeout(resolve, 600))
    task.status = 'completed' as 'completed'
  }

  isRunning.value = false
}

function refreshProjects() {
  projects.value = generateMockProjects()
  selectedProjectId.value = projects.value[0]?.project_id || null
}

function scrollToGraph() {
  const el = graphRef.value
  if (el) {
    el.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

// ============ Lifecycle ============
onMounted(() => {
  projects.value = generateMockProjects()
  selectedProjectId.value = projects.value[0]?.project_id || null
})
</script>
