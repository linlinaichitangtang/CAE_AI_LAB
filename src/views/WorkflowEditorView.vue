<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">工作流图编辑器</h2>
        <p class="text-sm" style="color: var(--text-muted)">V2.0-001 | 多尺度工作流图构建：节点编排与连线管理</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="resetAll">重置</button>
        <button class="btn btn-primary text-xs" @click="exportGraph">导出图</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Graph Management -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            图管理
          </h4>
          <div class="space-y-2">
            <button class="btn btn-primary text-xs w-full" @click="createNewGraph">新建图</button>
            <div>
              <label class="label">图名称</label>
              <input class="input w-full text-xs" type="text" v-model="graphName" placeholder="输入工作流图名称..." />
            </div>
            <div>
              <label class="label">从列表加载</label>
              <select class="input w-full text-xs" v-model="selectedGraphId" @change="loadGraph">
                <option value="">-- 选择已保存的图 --</option>
                <option v-for="g in savedGraphs" :key="g.graph_id" :value="g.graph_id">{{ g.name }}</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: Add Node -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            添加节点
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">尺度级别</label>
              <select class="input w-full text-xs" v-model="newNodeScale">
                <option value="dft">DFT</option>
                <option value="md">MD</option>
                <option value="phase_field">Phase Field</option>
                <option value="fe">FE</option>
              </select>
            </div>
            <div>
              <label class="label">节点标签</label>
              <input class="input w-full text-xs" type="text" v-model="newNodeLabel" placeholder="如: 层错能计算" />
            </div>
            <button class="btn btn-primary text-xs w-full" @click="addNode">添加节点</button>
          </div>
        </div>

        <!-- Step 3: Node List -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            节点列表
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ nodes.length }})</span>
          </h4>
          <div class="space-y-2" v-if="nodes.length > 0">
            <div
              v-for="node in nodes"
              :key="node.id"
              class="p-2.5 rounded border"
              style="background: var(--bg-elevated); border-color: var(--border-default)"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium" style="color: var(--text-primary)">{{ node.label }}</span>
                <span
                  class="text-[9px] px-1.5 py-0.5 rounded"
                  :style="scaleBadgeStyle(node.scale)"
                >{{ scaleLabel(node.scale) }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span
                  class="text-[9px] px-1.5 py-0.5 rounded"
                  :style="statusBadgeStyle(node.status)"
                >{{ statusLabel(node.status) }}</span>
                <div class="flex items-center gap-1">
                  <button class="btn btn-ghost text-[10px] px-1.5 py-0.5" @click="configureNode(node)">配置</button>
                  <button class="btn btn-ghost text-[10px] px-1.5 py-0.5" style="color: var(--accent-red)" @click="removeNode(node.id)">删除</button>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-[11px] p-2 rounded" style="color: var(--text-muted); background: var(--bg-elevated)">
            暂无节点，请在上方添加
          </div>
        </div>

        <!-- Step 4: Add Edge -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            添加连线
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">源节点</label>
              <select class="input w-full text-xs" v-model="edgeSourceId">
                <option value="">-- 选择源节点 --</option>
                <option v-for="n in nodes" :key="n.id" :value="n.id">{{ n.label }} ({{ scaleLabel(n.scale) }})</option>
              </select>
            </div>
            <div>
              <label class="label">目标节点</label>
              <select class="input w-full text-xs" v-model="edgeTargetId">
                <option value="">-- 选择目标节点 --</option>
                <option v-for="n in nodes" :key="n.id" :value="n.id">{{ n.label }} ({{ scaleLabel(n.scale) }})</option>
              </select>
            </div>
            <button class="btn btn-primary text-xs w-full" @click="addEdge">添加连线</button>
          </div>
        </div>

        <!-- Step 5: Actions & Validation -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            操作
          </h4>
          <div class="space-y-2">
            <button class="btn btn-ghost text-xs w-full" @click="validateGraph">验证图</button>
            <button class="btn btn-primary text-xs w-full" @click="saveGraph">保存图</button>
            <div v-if="validationResult" class="p-2 rounded text-[10px] space-y-1" :style="validationResult.is_valid ? 'background: rgba(34,197,94,0.1); border: 1px solid rgba(34,197,94,0.3)' : 'background: rgba(239,68,68,0.1); border: 1px solid rgba(239,68,68,0.3)'">
              <div class="font-medium" :style="validationResult.is_valid ? 'color: var(--accent-green)' : 'color: var(--accent-red)'">
                {{ validationResult.is_valid ? '验证通过' : '验证失败' }}
              </div>
              <div v-for="err in validationResult.errors" :key="err" style="color: var(--accent-red)">- {{ err }}</div>
              <div v-for="warn in validationResult.warnings" :key="warn" style="color: var(--accent-yellow)">- {{ warn }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">

        <!-- SVG Graph Visualization -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">工作流图可视化</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg width="100%" height="200" viewBox="0 0 720 200">
              <defs>
                <marker id="wf-edge-arrow" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                  <polygon points="0 0, 8 3, 0 6" fill="var(--text-muted)" />
                </marker>
                <filter id="wf-node-shadow" x="-10%" y="-10%" width="130%" height="130%">
                  <feDropShadow dx="0" dy="2" stdDeviation="3" flood-color="rgba(0,0,0,0.15)" />
                </filter>
              </defs>
              <!-- Edges -->
              <g v-for="edge in edges" :key="edge.id">
                <line
                  :x1="getNodeX(edge.source_node_id) + 80"
                  :y1="getNodeY(edge.source_node_id)"
                  :x2="getNodeX(edge.target_node_id) - 10"
                  :y2="getNodeY(edge.target_node_id)"
                  stroke="var(--text-muted)"
                  stroke-width="2"
                  stroke-dasharray="6 3"
                  marker-end="url(#wf-edge-arrow)"
                />
                <text
                  :x="(getNodeX(edge.source_node_id) + 80 + getNodeX(edge.target_node_id) - 10) / 2"
                  :y="getNodeY(edge.source_node_id) - 10"
                  text-anchor="middle"
                  fill="var(--text-muted)"
                  font-size="9"
                >{{ edge.label }}</text>
              </g>
              <!-- Nodes -->
              <g v-for="node in nodes" :key="node.id">
                <rect
                  :x="getNodeX(node.id)"
                  :y="getNodeY(node.id) - 30"
                  width="140"
                  height="60"
                  rx="8"
                  :fill="scaleColor(node.scale)"
                  :stroke="statusBorderColor(node.status)"
                  stroke-width="2"
                  filter="url(#wf-node-shadow)"
                />
                <text :x="getNodeX(node.id) + 70" :y="getNodeY(node.id) - 6" text-anchor="middle" fill="white" font-size="11" font-weight="600">
                  {{ node.label }}
                </text>
                <text :x="getNodeX(node.id) + 70" :y="getNodeY(node.id) + 14" text-anchor="middle" fill="rgba(255,255,255,0.75)" font-size="9">
                  {{ scaleLabel(node.scale) }} | {{ statusLabel(node.status) }}
                </text>
              </g>
            </svg>
          </div>
        </div>

        <!-- Edge List Table -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            连线列表
            <span class="text-[10px] ml-1" style="color: var(--text-muted)">({{ edges.length }})</span>
          </h4>
          <div class="rounded overflow-hidden" style="border: 1px solid var(--border-subtle)">
            <table class="w-full text-[11px]">
              <thead>
                <tr style="background: var(--bg-elevated)">
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">源节点</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">目标节点</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">标签</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">数据映射数</th>
                  <th class="px-3 py-2 text-left font-medium" style="color: var(--text-secondary)">操作</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="edge in edges"
                  :key="edge.id"
                  style="border-top: 1px solid var(--border-subtle)"
                >
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ getNodeLabel(edge.source_node_id) }}</td>
                  <td class="px-3 py-2" style="color: var(--text-primary)">{{ getNodeLabel(edge.target_node_id) }}</td>
                  <td class="px-3 py-2" style="color: var(--text-secondary)">{{ edge.label }}</td>
                  <td class="px-3 py-2" style="color: var(--text-secondary)">{{ edge.data_mapping.length }}</td>
                  <td class="px-3 py-2">
                    <button class="btn btn-ghost text-[10px] px-1.5 py-0.5" style="color: var(--accent-red)" @click="removeEdge(edge.id)">删除</button>
                  </td>
                </tr>
                <tr v-if="edges.length === 0">
                  <td colspan="5" class="px-3 py-4 text-center" style="color: var(--text-muted)">暂无连线</td>
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
import type { WorkflowScaleLevel, WfNode, WfEdge, WfGraph, WfGraphSave } from '../api/workflowEditor'

// ============ State ============
const graphName = ref('')
const selectedGraphId = ref('')
const savedGraphs = ref<WfGraphSave[]>([])
const nodes = ref<WfNode[]>([])
const edges = ref<WfEdge[]>([])
const validationResult = ref<{ is_valid: boolean; errors: string[]; warnings: string[] } | null>(null)

const newNodeScale = ref<WorkflowScaleLevel>('dft')
const newNodeLabel = ref('')
const edgeSourceId = ref('')
const edgeTargetId = ref('')

// ============ Helpers ============
function scaleLabel(scale: WorkflowScaleLevel): string {
  const map: Record<WorkflowScaleLevel, string> = {
    dft: 'DFT',
    md: 'MD',
    phase_field: 'Phase Field',
    fe: 'FE'
  }
  return map[scale] || scale
}

function scaleColor(scale: WorkflowScaleLevel): string {
  const map: Record<WorkflowScaleLevel, string> = {
    dft: '#3b82f6',
    md: '#22c55e',
    phase_field: '#f97316',
    fe: '#a855f7'
  }
  return map[scale] || '#6b7280'
}

function scaleBadgeStyle(scale: WorkflowScaleLevel): string {
  return `background: ${scaleColor(scale)}22; color: ${scaleColor(scale)}`
}

function statusLabel(status: string): string {
  const map: Record<string, string> = {
    idle: '空闲',
    configured: '已配置',
    running: '运行中',
    completed: '已完成',
    failed: '失败'
  }
  return map[status] || status
}

function statusBadgeStyle(status: string): string {
  const map: Record<string, string> = {
    idle: 'background: rgba(107,114,128,0.15); color: #6b7280',
    configured: 'background: rgba(59,130,246,0.15); color: #3b82f6',
    running: 'background: rgba(245,158,11,0.15); color: #f59e0b',
    completed: 'background: rgba(34,197,94,0.15); color: #22c55e',
    failed: 'background: rgba(239,68,68,0.15); color: #ef4444'
  }
  return map[status] || 'background: rgba(107,114,128,0.15); color: #6b7280'
}

function statusBorderColor(status: string): string {
  const map: Record<string, string> = {
    idle: '#9ca3af',
    configured: '#3b82f6',
    running: '#f59e0b',
    completed: '#22c55e',
    failed: '#ef4444'
  }
  return map[status] || '#9ca3af'
}

function getNodeX(nodeId: string): number {
  const idx = nodes.value.findIndex(n => n.id === nodeId)
  if (idx < 0) return 40
  return 40 + idx * 170
}

function getNodeY(nodeId: string): number {
  return 100
}

function getNodeLabel(nodeId: string): string {
  const node = nodes.value.find(n => n.id === nodeId)
  if (!node) return nodeId
  return node.label
}

// ============ Actions ============
function createNewGraph() {
  graphName.value = ''
  selectedGraphId.value = ''
  nodes.value = []
  edges.value = []
  validationResult.value = null
}

function loadGraph() {
  if (!selectedGraphId.value) return
  const found = savedGraphs.value.find(g => g.graph_id === selectedGraphId.value)
  if (!found) return
  graphName.value = found.name
  nodes.value = [...found.graph.nodes]
  edges.value = [...found.graph.edges]
  validationResult.value = null
}

function addNode() {
  if (!newNodeLabel.value.trim()) return
  const newNode: WfNode = {
    id: `node-${Date.now()}`,
    scale: newNodeScale.value,
    label: newNodeLabel.value.trim(),
    position: { x: 0, y: 0 },
    config: {},
    status: 'idle' as WfNode['status']
  }
  nodes.value.push(newNode)
  newNodeLabel.value = ''
}

function removeNode(nodeId: string) {
  nodes.value = nodes.value.filter(n => n.id !== nodeId)
  edges.value = edges.value.filter(e => e.source_node_id !== nodeId && e.target_node_id !== nodeId)
}

function configureNode(node: WfNode) {
  if (node.status === 'idle') {
    node.status = 'configured'
  }
}

function addEdge() {
  if (!edgeSourceId.value || !edgeTargetId.value) return
  if (edgeSourceId.value === edgeTargetId.value) return
  const sourceNode = nodes.value.find(n => n.id === edgeSourceId.value)
  const targetNode = nodes.value.find(n => n.id === edgeTargetId.value)
  if (!sourceNode || !targetNode) return
  const newEdge: WfEdge = {
    id: `edge-${Date.now()}`,
    source_node_id: edgeSourceId.value,
    target_node_id: edgeTargetId.value,
    data_mapping: [],
    label: `${scaleLabel(sourceNode.scale)} -> ${scaleLabel(targetNode.scale)}`
  }
  edges.value.push(newEdge)
  edgeSourceId.value = ''
  edgeTargetId.value = ''
}

function removeEdge(edgeId: string) {
  edges.value = edges.value.filter(e => e.id !== edgeId)
}

function validateGraph() {
  const errors: string[] = []
  const warnings: string[] = []
  if (nodes.value.length === 0) {
    errors.push('图中没有节点')
  }
  const configuredNodes = nodes.value.filter(n => n.status === 'configured' || n.status === 'completed')
  if (configuredNodes.length === 0 && nodes.value.length > 0) {
    warnings.push('所有节点均未配置')
  }
  if (edges.value.length === 0 && nodes.value.length > 1) {
    warnings.push('节点之间没有连线')
  }
  for (const edge of edges.value) {
    const sourceExists = nodes.value.some(n => n.id === edge.source_node_id)
    const targetExists = nodes.value.some(n => n.id === edge.target_node_id)
    if (!sourceExists) errors.push(`连线 ${edge.label} 的源节点不存在`)
    if (!targetExists) errors.push(`连线 ${edge.label} 的目标节点不存在`)
  }
  validationResult.value = {
    is_valid: errors.length === 0,
    errors,
    warnings
  }
}

function saveGraph() {
  if (!graphName.value.trim()) return
  const graph: WfGraph = {
    nodes: nodes.value,
    edges: edges.value,
    conditional_branches: []
  }
  const saveData: WfGraphSave = {
    graph_id: `graph-${Date.now()}`,
    name: graphName.value.trim(),
    description: '',
    graph,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString()
  }
  const existingIdx = savedGraphs.value.findIndex(g => g.graph_id === selectedGraphId.value)
  if (existingIdx >= 0) {
    savedGraphs.value[existingIdx] = saveData
  } else {
    savedGraphs.value.push(saveData)
  }
  selectedGraphId.value = saveData.graph_id
}

function exportGraph() {
  const graph: WfGraph = {
    nodes: nodes.value,
    edges: edges.value,
    conditional_branches: []
  }
  alert(`导出工作流图: ${graphName.value || '未命名'} (${nodes.value.length} 节点, ${edges.value.length} 连线)`)
}

function resetAll() {
  graphName.value = ''
  selectedGraphId.value = ''
  nodes.value = []
  edges.value = []
  validationResult.value = null
  newNodeScale.value = 'dft'
  newNodeLabel.value = ''
  edgeSourceId.value = ''
  edgeTargetId.value = ''
}

// ============ Mock Data ============
const mockNodes: WfNode[] = [
  {
    id: 'node-dft-001',
    scale: 'dft',
    label: '层错能计算',
    position: { x: 40, y: 100 },
    config: { xc_functional: 'PBE', k_points: '6x6x6' },
    status: 'completed'
  },
  {
    id: 'node-md-001',
    scale: 'md',
    label: '位错攀移模拟',
    position: { x: 210, y: 100 },
    config: { temperature: 573, potential: 'EAM' },
    status: 'completed'
  },
  {
    id: 'node-pf-001',
    scale: 'phase_field',
    label: '晶界滑移相场',
    position: { x: 380, y: 100 },
    config: { grid_size: 256, time_step: 0.01 },
    status: 'configured'
  },
  {
    id: 'node-fe-001',
    scale: 'fe',
    label: '宏观蠕变FE',
    position: { x: 550, y: 100 },
    config: { element_type: 'C3D8', mesh_size: 1.0 },
    status: 'idle'
  }
]

const mockEdges: WfEdge[] = [
  {
    id: 'edge-001',
    source_node_id: 'node-dft-001',
    target_node_id: 'node-md-001',
    data_mapping: [
      { source_field: 'stacking_fault_energy', target_field: 'sfe_input' },
      { source_field: 'elastic_constants', target_field: 'elastic_params' }
    ],
    label: 'DFT -> MD'
  },
  {
    id: 'edge-002',
    source_node_id: 'node-md-001',
    target_node_id: 'node-pf-001',
    data_mapping: [
      { source_field: 'mobility_tensor', target_field: 'mobility_input' },
      { source_field: 'grain_boundary_energy', target_field: 'gb_energy' }
    ],
    label: 'MD -> Phase Field'
  },
  {
    id: 'edge-003',
    source_node_id: 'node-pf-001',
    target_node_id: 'node-fe-001',
    data_mapping: [
      { source_field: 'homogenized_stress', target_field: 'constitutive_input' },
      { source_field: 'creep_strain_rate', target_field: 'creep_law' },
      { source_field: 'effective_modulus', target_field: 'material_props' }
    ],
    label: 'Phase Field -> FE'
  }
]

const mockSavedGraphs: WfGraphSave[] = [
  {
    graph_id: 'graph-001',
    name: '镁合金蠕变全链条',
    description: 'DFT-MD-PF-FE四尺度耦合工作流',
    graph: { nodes: mockNodes, edges: mockEdges, conditional_branches: [] },
    created_at: '2026-03-15T10:00:00Z',
    updated_at: '2026-03-28T14:30:00Z'
  },
  {
    graph_id: 'graph-002',
    name: '高强钢凝固流程',
    description: 'DFT-MD-PF-FE凝固模拟工作流',
    graph: { nodes: mockNodes.slice(0, 3), edges: mockEdges.slice(0, 2), conditional_branches: [] },
    created_at: '2026-03-20T08:00:00Z',
    updated_at: '2026-03-27T16:00:00Z'
  }
]

// ============ Lifecycle ============
onMounted(() => {
  savedGraphs.value = mockSavedGraphs
  nodes.value = [...mockNodes]
  edges.value = [...mockEdges]
  graphName.value = '镁合金蠕变全链条'
  selectedGraphId.value = 'graph-001'
})
</script>
