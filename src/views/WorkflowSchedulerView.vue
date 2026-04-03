<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">工作流调度器</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-003 | DAG 工作流执行、监控与重试</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetAll">重置</button>
        <button class="btn btn-primary text-xs" @click="refreshStatus">刷新状态</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Select Graph -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            选择工作流图
          </h4>
          <div class="space-y-2">
            <select class="input w-full text-xs" v-model="selectedGraphId">
              <option value="">-- 选择已保存的图 --</option>
              <option v-for="g in savedGraphs" :key="g.graph_id" :value="g.graph_id">{{ g.name }}</option>
            </select>
          </div>
        </div>

        <!-- Step 2: Execution Config -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            执行配置
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">最大重试次数</label>
              <input class="input w-full text-xs" type="number" v-model.number="execConfig.max_retries" min="0" max="10" />
            </div>
            <div>
              <label class="label">单节点超时 (秒)</label>
              <input class="input w-full text-xs" type="number" v-model.number="execConfig.timeout_per_node_s" min="60" max="86400" />
            </div>
            <div>
              <label class="label">并行节点数</label>
              <input class="input w-full text-xs" type="number" v-model.number="execConfig.parallel_nodes" min="1" max="8" />
            </div>
            <div class="flex items-center gap-2">
              <input type="checkbox" id="enable-rollback" v-model="execConfig.enable_rollback" />
              <label for="enable-rollback" class="text-xs" style="color: var(--text-secondary)">启用回滚</label>
            </div>
          </div>
        </div>

        <!-- Step 3: Start Execution -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            执行控制
          </h4>
          <div class="space-y-2">
            <button class="btn btn-primary text-xs w-full" @click="startExecution" :disabled="executionStatus === 'running'">
              启动执行
            </button>
            <div v-if="executionStatus" class="flex items-center justify-center">
              <span
                class="text-[10px] px-2 py-1 rounded-full font-medium"
                :style="execStatusBadgeStyle(executionStatus)"
              >{{ execStatusLabel(executionStatus) }}</span>
            </div>
          </div>
        </div>

        <!-- Step 4: Node Execution List -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            节点执行列表
          </h4>
          <div class="space-y-2" v-if="nodeExecutions.length > 0">
            <div
              v-for="nodeExec in nodeExecutions"
              :key="nodeExec.node_id"
              class="p-2.5 rounded border"
              style="background: var(--bg-elevated); border-color: var(--border-default)"
            >
              <div class="flex items-center justify-between mb-1">
                <div class="flex items-center gap-1.5">
                  <span
                    class="text-[9px] px-1.5 py-0.5 rounded"
                    :style="scaleBadgeStyle(nodeExec.scale)"
                  >{{ scaleLabel(nodeExec.scale) }}</span>
                  <span class="text-xs font-medium" style="color: var(--text-primary)">{{ nodeExec.node_id }}</span>
                </div>
                <span
                  class="text-[9px] px-1.5 py-0.5 rounded"
                  :style="execStatusBadgeStyle(nodeExec.status)"
                >{{ execStatusLabel(nodeExec.status) }}</span>
              </div>
              <div class="flex items-center gap-3 text-[10px]" style="color: var(--text-muted)">
                <span>耗时: {{ nodeExec.duration_s.toFixed(1) }}s</span>
                <span>重试: {{ nodeExec.retry_count }}</span>
              </div>
              <div v-if="nodeExec.error" class="mt-1 text-[10px] p-1 rounded" style="color: var(--accent-red); background: rgba(239,68,68,0.08)">
                {{ nodeExec.error }}
              </div>
            </div>
          </div>
          <div v-else class="text-[11px] p-2 rounded" style="color: var(--text-muted); background: var(--bg-elevated)">
            暂无执行记录
          </div>
        </div>

        <!-- Step 5: Controls -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            调度控制
          </h4>
          <div class="space-y-2">
            <div class="flex gap-2">
              <button class="btn btn-ghost text-xs flex-1" @click="pauseExecution" :disabled="executionStatus !== 'running'">暂停</button>
              <button class="btn btn-ghost text-xs flex-1" @click="resumeExecution" :disabled="executionStatus !== 'paused'">恢复</button>
              <button class="btn btn-ghost text-xs flex-1" @click="cancelExecution" :disabled="executionStatus !== 'running' && executionStatus !== 'paused'" style="color: var(--accent-red)">取消</button>
            </div>
            <div class="flex gap-2">
              <select class="input w-full text-xs flex-1" v-model="retryNodeId">
                <option value="">-- 选择重试节点 --</option>
                <option v-for="n in nodeExecutions" :key="n.node_id" :value="n.node_id">{{ n.node_id }}</option>
              </select>
              <button class="btn btn-ghost text-xs shrink-0" @click="retryFromNode" :disabled="!retryNodeId">从节点重试</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- SVG DAG Execution Visualization -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">DAG 执行可视化</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="200" viewBox="0 0 720 200">
              <defs>
                <marker id="sch-arrow" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                  <polygon points="0 0, 8 3, 0 6" fill="var(--text-muted)" />
                </marker>
                <filter id="sch-glow">
                  <feGaussianBlur stdDeviation="4" result="blur" />
                  <feMerge>
                    <feMergeNode in="blur" />
                    <feMergeNode in="SourceGraphic" />
                  </feMerge>
                </filter>
              </defs>
              <!-- Edges -->
              <g v-for="(edge, idx) in dagEdges" :key="idx">
                <line
                  :x1="edge.x1" :y1="edge.y1"
                  :x2="edge.x2" :y2="edge.y2"
                  stroke="var(--text-muted)"
                  stroke-width="2"
                  marker-end="url(#sch-arrow)"
                />
              </g>
              <!-- Nodes -->
              <g v-for="nodeExec in nodeExecutions" :key="nodeExec.node_id">
                <!-- Pulse animation for running node -->
                <rect
                  v-if="nodeExec.status === 'running'"
                  :x="getNodeX(nodeExec.node_id) - 5"
                  :y="70"
                  width="150"
                  height="60"
                  rx="12"
                  fill="none"
                  stroke="var(--primary)"
                  stroke-width="2"
                  opacity="0.5"
                >
                  <animate attributeName="opacity" values="0.5;0.1;0.5" dur="1.5s" repeatCount="indefinite" />
                  <animate attributeName="rx" values="12;18;12" dur="1.5s" repeatCount="indefinite" />
                </rect>
                <rect
                  :x="getNodeX(nodeExec.node_id)"
                  :y="75"
                  width="140"
                  height="50"
                  rx="8"
                  :fill="execNodeColor(nodeExec.status)"
                  :stroke="nodeExec.status === 'running' ? 'var(--primary)' : 'transparent'"
                  stroke-width="2"
                />
                <text :x="getNodeX(nodeExec.node_id) + 70" :y="97" text-anchor="middle" fill="white" font-size="11" font-weight="600">
                  {{ scaleLabel(nodeExec.scale) }}
                </text>
                <text :x="getNodeX(nodeExec.node_id) + 70" :y="115" text-anchor="middle" fill="rgba(255,255,255,0.8)" font-size="9">
                  {{ execStatusLabel(nodeExec.status) }} | {{ nodeExec.duration_s.toFixed(0) }}s
                </text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Execution Timeline -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">执行时间线</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="160" viewBox="0 0 640 160">
              <!-- Time axis -->
              <line x1="120" y1="130" x2="620" y2="130" stroke="var(--border-subtle)" stroke-width="1" />
              <text x="370" y="150" text-anchor="middle" fill="var(--text-muted)" font-size="9">时间 (秒)</text>
              <!-- Time ticks -->
              <g v-for="tick in timelineTicks" :key="tick.value">
                <line :x1="tick.x" :y1="125" :x2="tick.x" :y2="135" stroke="var(--border-subtle)" stroke-width="1" />
                <text :x="tick.x" :y="145" text-anchor="middle" fill="var(--text-muted)" font-size="8">{{ tick.label }}</text>
              </g>
              <!-- Bars -->
              <g v-for="nodeExec in nodeExecutions" :key="nodeExec.node_id">
                <rect
                  :x="getBarX(nodeExec)"
                  :y="getBarY(nodeExec)"
                  :width="getBarWidth(nodeExec)"
                  height="20"
                  rx="3"
                  :fill="execNodeColor(nodeExec.status)"
                  opacity="0.85"
                />
                <text :x="115" :y="getBarY(nodeExec) + 14" text-anchor="end" fill="var(--text-secondary)" font-size="9">
                  {{ scaleLabel(nodeExec.scale) }}
                </text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Checkpoints List -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            检查点列表
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ checkpoints.length }})</span>
          </h4>
          <div class="rounded overflow-hidden" style="border: 1px solid var(--border-subtle)">
            <table class="w-full text-[11px]">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">节点 ID</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">步骤索引</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">时间戳</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="cp in checkpoints"
                  :key="cp.node_id + '-' + cp.step_index"
                  style="border-top: 1px solid var(--border-subtle)"
                >
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ cp.node_id }}</td>
                  <td class="px-3 py-2" style="color: var(--text-secondary)">{{ cp.step_index }}</td>
                  <td class="px-3 py-2" style="color: var(--text-muted)">{{ cp.timestamp }}</td>
                </tr>
                <tr v-if="checkpoints.length === 0">
                  <td colspan="3" class="px-3 py-4 text-center" style="color: var(--text-muted)">暂无检查点</td>
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
import { ref, reactive, computed, onMounted } from 'vue'
import type { ExecutionStatus, NodeExecution, WorkflowExecution, ExecutionConfig } from '../api/workflowScheduler'

// ============ State ============
const selectedGraphId = ref('')
const executionStatus = ref<ExecutionStatus | null>(null)
const retryNodeId = ref('')

const savedGraphs = ref<Array<{ graph_id: string; name: string }>>([])
const nodeExecutions = ref<NodeExecution[]>([])
const checkpoints = ref<Array<{ node_id: string; step_index: number; timestamp: string }>>([])

const execConfig = reactive<ExecutionConfig>({
  graph_id: '',
  max_retries: 3,
  timeout_per_node_s: 3600,
  parallel_nodes: 2,
  enable_rollback: true,
  notification_on_complete: false
})

// ============ Computed ============
const dagEdges = computed(() => {
  const edges: Array<{ x1: number; y1: number; x2: number; y2: number }> = []
  for (let i = 0; i < nodeExecutions.value.length - 1; i++) {
    const sourceX = getNodeX(nodeExecutions.value[i].node_id)
    const targetX = getNodeX(nodeExecutions.value[i + 1].node_id)
    edges.push({
      x1: sourceX + 140,
      y1: 100,
      x2: targetX,
      y2: 100
    })
  }
  return edges
})

const timelineTicks = computed(() => {
  const maxTime = Math.max(...nodeExecutions.value.map(n => {
    const started = n.started_at ? new Date(n.started_at).getTime() : 0
    return started + n.duration_s * 1000
  }), 1)
  const baseTime = nodeExecutions.value.length > 0 && nodeExecutions.value[0].started_at
    ? new Date(nodeExecutions.value[0].started_at).getTime()
    : 0
  const totalMs = maxTime - baseTime
  const tickCount = 5
  const ticks: Array<{ value: number; label: string; x: number }> = []
  for (let i = 0; i <= tickCount; i++) {
    const value = (totalMs / tickCount) * i
    const x = 120 + (value / totalMs) * 500
    ticks.push({
      value,
      label: (value / 1000).toFixed(0) + 's',
      x
    })
  }
  return ticks
})

// ============ Helpers ============
function scaleLabel(scale: string): string {
  const map: Record<string, string> = {
    dft: 'DFT',
    md: 'MD',
    phase_field: 'Phase Field',
    fe: 'FE'
  }
  return map[scale] || scale
}

function scaleBadgeStyle(scale: string): string {
  const map: Record<string, string> = {
    dft: 'background: rgba(59,130,246,0.15); color: #3b82f6',
    md: 'background: rgba(34,197,94,0.15); color: #22c55e',
    phase_field: 'background: rgba(249,115,22,0.15); color: #f97316',
    fe: 'background: rgba(168,85,247,0.15); color: #a855f7'
  }
  return map[scale] || 'background: rgba(107,114,128,0.15); color: #6b7280'
}

function execStatusLabel(status: ExecutionStatus | string): string {
  const map: Record<string, string> = {
    pending: '等待中',
    queued: '排队中',
    running: '运行中',
    paused: '已暂停',
    completed: '已完成',
    failed: '失败',
    cancelled: '已取消',
    rollback: '回滚中'
  }
  return map[status] || status
}

function execStatusBadgeStyle(status: ExecutionStatus | string): string {
  const map: Record<string, string> = {
    pending: 'background: rgba(107,114,128,0.15); color: #6b7280',
    queued: 'background: rgba(107,114,128,0.15); color: #6b7280',
    running: 'background: rgba(59,130,246,0.15); color: #3b82f6',
    paused: 'background: rgba(245,158,11,0.15); color: #f59e0b',
    completed: 'background: rgba(34,197,94,0.15); color: #22c55e',
    failed: 'background: rgba(239,68,68,0.15); color: #ef4444',
    cancelled: 'background: rgba(107,114,128,0.15); color: #6b7280',
    rollback: 'background: rgba(245,158,11,0.15); color: #f59e0b'
  }
  return map[status] || 'background: rgba(107,114,128,0.15); color: #6b7280'
}

function execNodeColor(status: ExecutionStatus | string): string {
  const map: Record<string, string> = {
    pending: '#6b7280',
    queued: '#9ca3af',
    running: '#3b82f6',
    paused: '#f59e0b',
    completed: '#22c55e',
    failed: '#ef4444',
    cancelled: '#9ca3af',
    rollback: '#f97316'
  }
  return map[status] || '#6b7280'
}

function getNodeX(nodeId: string): number {
  const idx = nodeExecutions.value.findIndex(n => n.node_id === nodeId)
  if (idx < 0) return 40
  return 40 + idx * 170
}

function getBarX(nodeExec: NodeExecution): number {
  if (!nodeExec.started_at) return 120
  const baseTime = nodeExecutions.value.length > 0 && nodeExecutions.value[0].started_at
    ? new Date(nodeExecutions.value[0].started_at).getTime()
    : 0
  const startTime = new Date(nodeExec.started_at).getTime()
  const maxTime = Math.max(...nodeExecutions.value.map(n => {
    const started = n.started_at ? new Date(n.started_at).getTime() : 0
    return started + n.duration_s * 1000
  }), 1)
  const totalMs = maxTime - baseTime
  if (totalMs <= 0) return 120
  return 120 + ((startTime - baseTime) / totalMs) * 500
}

function getBarY(nodeExec: NodeExecution): number {
  const idx = nodeExecutions.value.findIndex(n => n.node_id === nodeExec.node_id)
  return 10 + idx * 28
}

function getBarWidth(nodeExec: NodeExecution): number {
  const maxTime = Math.max(...nodeExecutions.value.map(n => {
    const started = n.started_at ? new Date(n.started_at).getTime() : 0
    return started + n.duration_s * 1000
  }), 1)
  const baseTime = nodeExecutions.value.length > 0 && nodeExecutions.value[0].started_at
    ? new Date(nodeExecutions.value[0].started_at).getTime()
    : 0
  const totalMs = maxTime - baseTime
  if (totalMs <= 0) return 20
  return Math.max(20, (nodeExec.duration_s * 1000 / totalMs) * 500)
}

// ============ Actions ============
function startExecution() {
  if (!selectedGraphId.value) return
  executionStatus.value = 'running'
  execConfig.graph_id = selectedGraphId.value
  // Simulate execution completing after a brief delay
  setTimeout(() => {
    executionStatus.value = 'completed'
  }, 2000)
}

function pauseExecution() {
  if (executionStatus.value !== 'running') return
  executionStatus.value = 'paused'
}

function resumeExecution() {
  if (executionStatus.value !== 'paused') return
  executionStatus.value = 'running'
}

function cancelExecution() {
  if (executionStatus.value !== 'running' && executionStatus.value !== 'paused') return
  executionStatus.value = 'cancelled'
}

function retryFromNode() {
  if (!retryNodeId.value) return
  const nodeExec = nodeExecutions.value.find(n => n.node_id === retryNodeId.value)
  if (!nodeExec) return
  nodeExec.status = 'running'
  nodeExec.retry_count += 1
  nodeExec.error = null
  executionStatus.value = 'running'
  setTimeout(() => {
    nodeExec.status = 'completed'
    executionStatus.value = 'completed'
  }, 1500)
}

function refreshStatus() {
  // Refresh mock data
}

function resetAll() {
  selectedGraphId.value = ''
  executionStatus.value = null
  retryNodeId.value = ''
  execConfig.max_retries = 3
  execConfig.timeout_per_node_s = 3600
  execConfig.parallel_nodes = 2
  execConfig.enable_rollback = true
  nodeExecutions.value = [...mockNodeExecutions]
  checkpoints.value = [...mockCheckpoints]
}

// ============ Mock Data ============
const baseTime = new Date('2026-03-28T14:00:00Z').getTime()

const mockNodeExecutions: NodeExecution[] = [
  {
    node_id: 'node-dft-001',
    scale: 'dft',
    status: 'completed',
    started_at: new Date(baseTime).toISOString(),
    completed_at: new Date(baseTime + 45 * 1000).toISOString(),
    duration_s: 45.2,
    input_data: { xc_functional: 'PBE', k_points: '6x6x6' },
    output_data: { stacking_fault_energy: 145.3, elastic_constants: [[180, 80, 80], [80, 180, 80], [80, 80, 180]] },
    error: null,
    retry_count: 0
  },
  {
    node_id: 'node-md-001',
    scale: 'md',
    status: 'completed',
    started_at: new Date(baseTime + 50 * 1000).toISOString(),
    completed_at: new Date(baseTime + 185 * 1000).toISOString(),
    duration_s: 135.8,
    input_data: { sfe_input: 145.3, temperature: 573 },
    output_data: { mobility_tensor: [0.12, 0.08, 0.15], grain_boundary_energy: 0.85 },
    error: null,
    retry_count: 0
  },
  {
    node_id: 'node-pf-001',
    scale: 'phase_field',
    status: 'completed',
    started_at: new Date(baseTime + 190 * 1000).toISOString(),
    completed_at: new Date(baseTime + 340 * 1000).toISOString(),
    duration_s: 150.3,
    input_data: { mobility_input: [0.12, 0.08, 0.15], grid_size: 256 },
    output_data: { homogenized_stress: 320.5, creep_strain_rate: 1.2e-8, effective_modulus: 68.2 },
    error: null,
    retry_count: 1
  },
  {
    node_id: 'node-fe-001',
    scale: 'fe',
    status: 'completed',
    started_at: new Date(baseTime + 345 * 1000).toISOString(),
    completed_at: new Date(baseTime + 420 * 1000).toISOString(),
    duration_s: 75.6,
    input_data: { constitutive_input: 320.5, creep_law: 1.2e-8, element_type: 'C3D8' },
    output_data: { max_stress: 285.3, max_strain: 0.023, life_cycles: 1.2e5 },
    error: null,
    retry_count: 0
  }
]

const mockCheckpoints: Array<{ node_id: string; step_index: number; timestamp: string }> = [
  { node_id: 'node-dft-001', step_index: 0, timestamp: '2026-03-28 14:00:00' },
  { node_id: 'node-dft-001', step_index: 1, timestamp: '2026-03-28 14:00:22' },
  { node_id: 'node-dft-001', step_index: 2, timestamp: '2026-03-28 14:00:45' },
  { node_id: 'node-md-001', step_index: 0, timestamp: '2026-03-28 14:00:50' },
  { node_id: 'node-md-001', step_index: 1, timestamp: '2026-03-28 14:01:45' },
  { node_id: 'node-md-001', step_index: 2, timestamp: '2026-03-28 14:03:05' },
  { node_id: 'node-pf-001', step_index: 0, timestamp: '2026-03-28 14:03:10' },
  { node_id: 'node-pf-001', step_index: 1, timestamp: '2026-03-28 14:04:30' },
  { node_id: 'node-pf-001', step_index: 2, timestamp: '2026-03-28 14:05:40' },
  { node_id: 'node-fe-001', step_index: 0, timestamp: '2026-03-28 14:05:45' },
  { node_id: 'node-fe-001', step_index: 1, timestamp: '2026-03-28 14:06:20' },
  { node_id: 'node-fe-001', step_index: 2, timestamp: '2026-03-28 14:07:00' }
]

const mockSavedGraphs: Array<{ graph_id: string; name: string }> = [
  { graph_id: 'graph-001', name: '镁合金蠕变全链条' },
  { graph_id: 'graph-002', name: '高强钢凝固流程' },
  { graph_id: 'graph-003', name: 'NiTi相变模拟' }
]

// ============ Lifecycle ============
onMounted(() => {
  savedGraphs.value = [...mockSavedGraphs]
  nodeExecutions.value = [...mockNodeExecutions]
  checkpoints.value = [...mockCheckpoints]
  selectedGraphId.value = 'graph-001'
  executionStatus.value = 'completed'
})
</script>
