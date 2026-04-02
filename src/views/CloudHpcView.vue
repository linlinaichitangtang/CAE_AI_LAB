<!--
  云端 HPC 提交视图 - CloudHpcView.vue
  V1.4-005: 仿真任务上传 → 云端排队 → 进度实时推送 → 结果下载
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">云端 HPC 提交</h2>
        <p class="text-sm text-[var(--text-secondary)]">仿真任务上传 → 云端排队 → 进度实时推送 → 结果下载</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="refreshAll" class="btn btn-ghost text-xs">刷新状态</button>
        <button @click="showTemplates = !showTemplates" class="btn btn-ghost text-xs">模板</button>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧面板 -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto flex-shrink-0">
        <!-- Step 1: 任务配置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('config')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">Step 1: 任务配置</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.config ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.config" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">任务名称</label>
              <input v-model="submitConfig.job_name" type="text" class="input w-full text-xs" placeholder="输入任务名称" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">求解器类型</label>
              <select v-model="submitConfig.solver_type" class="input w-full text-xs">
                <option value="calculix">CalculiX (结构分析)</option>
                <option value="openfoam">OpenFOAM (CFD)</option>
                <option value="custom">自定义求解器</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: 资源配置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('resource')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">Step 2: 资源配置</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.resource ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.resource" class="px-4 pb-4 space-y-3">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">节点数</label>
                <input v-model.number="submitConfig.node_count" type="number" min="1" max="64" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">每节点核心数</label>
                <input v-model.number="submitConfig.cores_per_node" type="number" min="1" max="128" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">内存 (GB)</label>
                <input v-model.number="submitConfig.memory_gb" type="number" min="1" max="1024" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">墙钟时间 (h)</label>
                <input v-model.number="submitConfig.wall_time_hours" type="number" min="1" max="168" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">优先级 (1-10)</label>
              <div class="flex items-center gap-2">
                <input v-model.number="submitConfig.priority" type="range" min="1" max="10" class="flex-1" />
                <span class="text-xs text-[var(--text-primary)] w-6 text-right">{{ submitConfig.priority }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: 文件上传 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('files')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">Step 3: 文件上传</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.files ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.files" class="px-4 pb-4 space-y-3">
            <div
              @dragover.prevent="onDragOver"
              @dragleave="onDragLeave"
              @drop.prevent="onDrop"
              :class="['border-2 border-dashed rounded-lg p-4 text-center transition-colors cursor-pointer',
                isDragging ? 'border-[var(--primary)] bg-[var(--primary)]/5' : 'border-[var(--border-subtle)] hover:border-[var(--border-default)]']"
              @click="triggerFileInput"
            >
              <div class="text-[var(--text-muted)] text-xs">
                <div class="text-lg mb-1">📁</div>
                <div>拖拽文件到此处或点击上传</div>
                <div class="text-[10px] mt-1">支持 .inp, .dat, .stl, .vtk 等格式</div>
              </div>
              <input ref="fileInputRef" type="file" multiple class="hidden" @change="onFileSelected" />
            </div>
            <div v-if="submitConfig.input_files.length > 0" class="space-y-1">
              <div v-for="(file, idx) in submitConfig.input_files" :key="idx"
                class="flex items-center justify-between px-2 py-1 bg-[var(--bg-elevated)] rounded text-xs">
                <span class="text-[var(--text-primary)] truncate flex-1">{{ file }}</span>
                <button @click="removeFile(idx)" class="text-[var(--accent-red)] hover:opacity-80 ml-2">✕</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 4: 通知设置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('notify')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">Step 4: 通知设置</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.notify ? '▼' : '▶' }}</span>
          </button>
          <div v-show="sections.notify" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">回调 URL</label>
              <input v-model="submitConfig.callback_url" type="text" class="input w-full text-xs" placeholder="https://your-server.com/callback" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">邮件通知</label>
              <input v-model="submitConfig.notification_email" type="email" class="input w-full text-xs" placeholder="user@example.com" />
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="px-4 py-3 space-y-2">
          <button @click="handleSubmit" :disabled="submitting" class="w-full btn btn-primary text-xs">
            {{ submitting ? '提交中...' : '提交任务' }}
          </button>
        </div>

        <!-- 快捷模板 -->
        <div v-if="showTemplates" class="px-4 pb-4 space-y-2 border-t border-[var(--border-subtle)]">
          <h4 class="text-xs font-semibold text-[var(--text-primary)] mt-3">快捷模板</h4>
          <button v-for="tpl in templates" :key="tpl.id" @click="applyTemplate(tpl)"
            class="w-full text-left px-3 py-2 bg-[var(--bg-elevated)] hover:bg-[var(--bg-base)] rounded-lg transition-colors">
            <div class="text-xs text-[var(--text-primary)] font-medium">{{ tpl.name }}</div>
            <div class="text-[10px] text-[var(--text-muted)]">{{ tpl.description }}</div>
            <div class="text-[10px] text-[var(--text-secondary)] mt-1">
              {{ tpl.recommended_nodes }} 节点 / {{ tpl.recommended_cores }} 核心 / {{ tpl.solver_type }}
            </div>
          </button>
        </div>
      </div>

      <!-- 右侧区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- 集群概览卡片 -->
        <div class="grid grid-cols-4 gap-3 px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">总节点</div>
            <div class="text-lg font-semibold text-[var(--text-primary)]">{{ clusterInfo.total_nodes }}</div>
          </div>
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">在线节点</div>
            <div class="text-lg font-semibold text-[var(--accent-green)]">{{ clusterInfo.online_nodes }}</div>
          </div>
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">可用核心</div>
            <div class="text-lg font-semibold text-[var(--primary)]">{{ clusterInfo.available_cores }}</div>
          </div>
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">队列长度</div>
            <div class="text-lg font-semibold" :class="clusterInfo.queue_length > 10 ? 'text-[var(--accent-red)]' : 'text-[var(--accent-yellow)]'">
              {{ clusterInfo.queue_length }}
            </div>
          </div>
        </div>

        <!-- 任务列表 -->
        <div class="flex-1 overflow-auto px-4 py-3">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">任务列表</h3>
            <div class="flex items-center gap-2">
              <select v-model="statusFilter" class="input text-xs py-1 px-2">
                <option value="">全部状态</option>
                <option value="queued">排队中</option>
                <option value="running">运行中</option>
                <option value="completed">已完成</option>
                <option value="failed">失败</option>
                <option value="cancelled">已取消</option>
              </select>
            </div>
          </div>

          <table class="w-full text-xs">
            <thead>
              <tr class="text-[var(--text-muted)] border-b border-[var(--border-subtle)]">
                <th class="text-left py-2 px-2 font-medium">ID</th>
                <th class="text-left py-2 px-2 font-medium">名称</th>
                <th class="text-left py-2 px-2 font-medium">状态</th>
                <th class="text-left py-2 px-2 font-medium">进度</th>
                <th class="text-left py-2 px-2 font-medium">提交时间</th>
                <th class="text-left py-2 px-2 font-medium">操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="job in filteredJobs" :key="job.id"
                @click="selectJob(job)"
                :class="['border-b border-[var(--border-subtle)] cursor-pointer transition-colors',
                  selectedJob?.id === job.id ? 'bg-[var(--primary)]/10' : 'hover:bg-[var(--bg-elevated)]']">
                <td class="py-2 px-2 text-[var(--text-secondary)] font-mono">{{ job.id.slice(0, 8) }}</td>
                <td class="py-2 px-2 text-[var(--text-primary)]">{{ job.name }}</td>
                <td class="py-2 px-2">
                  <span :class="['px-2 py-0.5 rounded-full text-[10px] font-medium', statusClass(job.status)]">
                    {{ statusLabel(job.status) }}
                  </span>
                </td>
                <td class="py-2 px-2">
                  <div class="flex items-center gap-2">
                    <div class="w-16 h-1.5 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                      <div class="h-full rounded-full transition-all duration-300"
                        :class="progressBarClass(job.status)"
                        :style="{ width: job.progress + '%' }"></div>
                    </div>
                    <span class="text-[var(--text-muted)]">{{ job.progress }}%</span>
                  </div>
                </td>
                <td class="py-2 px-2 text-[var(--text-muted)]">{{ formatTime(job.submit_time) }}</td>
                <td class="py-2 px-2">
                  <div class="flex items-center gap-1">
                    <button v-if="job.status === 'queued' || job.status === 'running'"
                      @click.stop="handleCancel(job.id)"
                      class="px-2 py-0.5 text-[10px] rounded border border-[var(--accent-red)] text-[var(--accent-red)] hover:bg-[var(--accent-red)]/10 transition-colors">
                      取消
                    </button>
                    <button v-if="job.status === 'completed'"
                      @click.stop="handleDownload(job.id)"
                      class="px-2 py-0.5 text-[10px] rounded border border-[var(--primary)] text-[var(--primary)] hover:bg-[var(--primary)]/10 transition-colors">
                      下载
                    </button>
                  </div>
                </td>
              </tr>
              <tr v-if="filteredJobs.length === 0">
                <td colspan="6" class="py-8 text-center text-[var(--text-muted)]">暂无任务</td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- 任务详情面板 -->
        <div v-if="selectedJob" class="border-t border-[var(--border-subtle)] bg-[var(--bg-surface)] px-4 py-3 max-h-64 overflow-y-auto">
          <div class="flex items-center justify-between mb-2">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">任务详情: {{ selectedJob.name }}</h4>
            <button @click="selectedJob = null" class="text-[var(--text-muted)] hover:text-[var(--text-primary)] text-xs">✕</button>
          </div>
          <div class="grid grid-cols-3 gap-4 text-xs">
            <div>
              <div class="text-[var(--text-muted)] mb-1">资源</div>
              <div class="text-[var(--text-secondary)]">
                {{ selectedJob.node_count }} 节点 x {{ selectedJob.cores_per_node }} 核心<br />
                {{ selectedJob.memory_gb }} GB 内存<br />
                墙钟时间: {{ selectedJob.wall_time_hours }}h
              </div>
            </div>
            <div>
              <div class="text-[var(--text-muted)] mb-1">时间</div>
              <div class="text-[var(--text-secondary)]">
                提交: {{ formatTime(selectedJob.submit_time) }}<br />
                开始: {{ selectedJob.start_time ? formatTime(selectedJob.start_time) : '--' }}<br />
                结束: {{ selectedJob.end_time ? formatTime(selectedJob.end_time) : '--' }}
              </div>
            </div>
            <div>
              <div class="text-[var(--text-muted)] mb-1">进度</div>
              <div class="w-full h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden mb-1">
                <div class="h-full rounded-full transition-all duration-500"
                  :class="progressBarClass(selectedJob.status)"
                  :style="{ width: selectedJob.progress + '%' }"></div>
              </div>
              <div class="text-[var(--text-secondary)]">{{ selectedJob.progress }}% - 预估 {{ selectedJob.estimated_time }}</div>
            </div>
          </div>
          <div v-if="selectedJob.error_message" class="mt-2 p-2 bg-[var(--accent-red)]/10 rounded text-[var(--accent-red)] text-xs">
            错误: {{ selectedJob.error_message }}
          </div>
          <div v-if="selectedJob.log_url" class="mt-2">
            <a :href="selectedJob.log_url" target="_blank" class="text-xs text-[var(--primary)] hover:underline">查看日志 →</a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { HpcJob, HpcSubmitConfig, HpcClusterInfo, HpcTemplate } from '../api/cloudHpc'

// ============ 响应式状态 ============

const sections = ref({
  config: true,
  resource: true,
  files: true,
  notify: false
})

const showTemplates = ref(false)
const submitting = ref(false)
const isDragging = ref(false)
const statusFilter = ref('')
const selectedJob = ref<HpcJob | null>(null)
const fileInputRef = ref<HTMLInputElement | null>(null)

const submitConfig = ref<HpcSubmitConfig>({
  project_id: 'proj-default-001',
  job_name: '',
  input_files: [],
  solver_type: 'calculix',
  node_count: 2,
  cores_per_node: 16,
  memory_gb: 64,
  wall_time_hours: 24,
  priority: 5,
  callback_url: '',
  notification_email: ''
})

// ============ 模拟数据 ============

const clusterInfo = ref<HpcClusterInfo>({
  name: 'CAELab-Production-Cluster',
  total_nodes: 32,
  online_nodes: 28,
  total_cores: 1024,
  available_cores: 384,
  total_memory_gb: 4096,
  available_memory_gb: 1536,
  queue_length: 7,
  avg_wait_time_minutes: 12
})

const mockJobs = ref<HpcJob[]>([
  {
    id: 'hpc-job-0001-a3f8c2d1',
    name: '机翼结构静力分析',
    project_id: 'proj-aero-001',
    status: 'running',
    submit_time: '2026-04-02T08:30:00Z',
    start_time: '2026-04-02T08:35:00Z',
    end_time: '',
    priority: 8,
    node_count: 4,
    cores_per_node: 32,
    memory_gb: 128,
    wall_time_hours: 12,
    estimated_time: '约 2h 后完成',
    progress: 67,
    input_file_path: '/data/wing_static.inp',
    output_file_path: '',
    log_url: 'https://hpc.caelab.io/logs/hpc-job-0001',
    error_message: ''
  },
  {
    id: 'hpc-job-0002-b7e4d1f3',
    name: '发动机叶片热分析',
    project_id: 'proj-turbine-002',
    status: 'queued',
    submit_time: '2026-04-02T09:15:00Z',
    start_time: '',
    end_time: '',
    priority: 6,
    node_count: 8,
    cores_per_node: 64,
    memory_gb: 256,
    wall_time_hours: 24,
    estimated_time: '预计 15min 后开始',
    progress: 0,
    input_file_path: '/data/blade_thermal.inp',
    output_file_path: '',
    log_url: '',
    error_message: ''
  },
  {
    id: 'hpc-job-0003-c9a2e5b7',
    name: '汽车碰撞仿真',
    project_id: 'proj-auto-003',
    status: 'completed',
    submit_time: '2026-04-01T14:00:00Z',
    start_time: '2026-04-01T14:02:00Z',
    end_time: '2026-04-01T18:45:00Z',
    priority: 9,
    node_count: 16,
    cores_per_node: 32,
    memory_gb: 512,
    wall_time_hours: 8,
    estimated_time: '',
    progress: 100,
    input_file_path: '/data/crash_sim.inp',
    output_file_path: '/data/results/crash_sim_output/',
    log_url: 'https://hpc.caelab.io/logs/hpc-job-0003',
    error_message: ''
  },
  {
    id: 'hpc-job-0004-d1c8f3a5',
    name: '桥梁模态分析',
    project_id: 'proj-civil-004',
    status: 'failed',
    submit_time: '2026-04-02T07:00:00Z',
    start_time: '2026-04-02T07:01:00Z',
    end_time: '2026-04-02T07:12:00Z',
    priority: 4,
    node_count: 2,
    cores_per_node: 16,
    memory_gb: 32,
    wall_time_hours: 4,
    estimated_time: '',
    progress: 23,
    input_file_path: '/data/bridge_modal.inp',
    output_file_path: '',
    log_url: 'https://hpc.caelab.io/logs/hpc-job-0004',
    error_message: '内存不足: 分配 32GB 但求解器需要至少 48GB'
  },
  {
    id: 'hpc-job-0005-e3b6d9c1',
    name: 'CFD 管道流场分析',
    project_id: 'proj-fluid-005',
    status: 'running',
    submit_time: '2026-04-02T06:00:00Z',
    start_time: '2026-04-02T06:05:00Z',
    end_time: '',
    priority: 7,
    node_count: 4,
    cores_per_node: 32,
    memory_gb: 128,
    wall_time_hours: 16,
    estimated_time: '约 4h 后完成',
    progress: 42,
    input_file_path: '/data/pipe_cfd.inp',
    output_file_path: '',
    log_url: 'https://hpc.caelab.io/logs/hpc-job-0005',
    error_message: ''
  },
  {
    id: 'hpc-job-0006-f5a7c2e8',
    name: '参数化扫描-厚度优化',
    project_id: 'proj-opt-006',
    status: 'cancelled',
    submit_time: '2026-04-01T20:00:00Z',
    start_time: '',
    end_time: '2026-04-01T20:05:00Z',
    priority: 3,
    node_count: 2,
    cores_per_node: 8,
    memory_gb: 16,
    wall_time_hours: 48,
    estimated_time: '',
    progress: 0,
    input_file_path: '/data/param_sweep.inp',
    output_file_path: '',
    log_url: '',
    error_message: '用户手动取消'
  }
])

const templates = ref<HpcTemplate[]>([
  {
    id: 'tpl-001',
    name: '大型结构分析',
    name_en: 'Large Structural Analysis',
    category: 'structural',
    description: '适用于百万级自由度结构静力/动力分析',
    solver_type: 'calculix',
    recommended_nodes: 8,
    recommended_cores: 32
  },
  {
    id: 'tpl-002',
    name: 'CFD 高精度',
    name_en: 'High-Fidelity CFD',
    category: 'cfd',
    description: '高雷诺数湍流模拟，精细网格',
    solver_type: 'openfoam',
    recommended_nodes: 16,
    recommended_cores: 64
  },
  {
    id: 'tpl-003',
    name: '显式动力学',
    name_en: 'Explicit Dynamics',
    category: 'dynamics',
    description: '碰撞、冲击、爆炸等瞬态动力学分析',
    solver_type: 'calculix',
    recommended_nodes: 16,
    recommended_cores: 32
  },
  {
    id: 'tpl-004',
    name: '参数化扫描',
    name_en: 'Parametric Sweep',
    category: 'optimization',
    description: '多参数组合批量提交，自动排队',
    solver_type: 'calculix',
    recommended_nodes: 2,
    recommended_cores: 16
  }
])

// ============ 计算属性 ============

const filteredJobs = computed(() => {
  if (!statusFilter.value) return mockJobs.value
  return mockJobs.value.filter(j => j.status === statusFilter.value)
})

// ============ 方法 ============

function toggleSection(key: keyof typeof sections.value) {
  sections.value[key] = !sections.value[key]
}

function statusClass(status: HpcJob['status']): string {
  const map: Record<string, string> = {
    queued: 'bg-[var(--accent-yellow)]/20 text-[var(--accent-yellow)]',
    running: 'bg-[var(--primary)]/20 text-[var(--primary)]',
    completed: 'bg-[var(--accent-green)]/20 text-[var(--accent-green)]',
    failed: 'bg-[var(--accent-red)]/20 text-[var(--accent-red)]',
    cancelled: 'bg-gray-500/20 text-gray-400'
  }
  return map[status] || ''
}

function statusLabel(status: HpcJob['status']): string {
  const map: Record<string, string> = {
    queued: '排队中',
    running: '运行中',
    completed: '已完成',
    failed: '失败',
    cancelled: '已取消'
  }
  return map[status] || status
}

function progressBarClass(status: HpcJob['status']): string {
  if (status === 'completed') return 'bg-[var(--accent-green)]'
  if (status === 'failed') return 'bg-[var(--accent-red)]'
  if (status === 'running') return 'bg-[var(--primary)]'
  return 'bg-[var(--accent-yellow)]'
}

function formatTime(iso: string): string {
  if (!iso) return '--'
  const d = new Date(iso)
  return d.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

function selectJob(job: HpcJob) {
  selectedJob.value = job
}

function applyTemplate(tpl: HpcTemplate) {
  submitConfig.value.solver_type = tpl.solver_type
  submitConfig.value.node_count = tpl.recommended_nodes
  submitConfig.value.cores_per_node = tpl.recommended_cores
  submitConfig.value.memory_gb = tpl.recommended_cores * 4
  showTemplates.value = false
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
  if (files) {
    for (let i = 0; i < files.length; i++) {
      submitConfig.value.input_files.push(files[i].name)
    }
  }
}

function triggerFileInput() {
  fileInputRef.value?.click()
}

function onFileSelected(e: Event) {
  const target = e.target as HTMLInputElement
  const files = target.files
  if (files) {
    for (let i = 0; i < files.length; i++) {
      submitConfig.value.input_files.push(files[i].name)
    }
  }
}

function removeFile(idx: number) {
  submitConfig.value.input_files.splice(idx, 1)
}

async function handleSubmit() {
  if (!submitConfig.value.job_name) return
  submitting.value = true
  try {
    const newJob: HpcJob = {
      id: `hpc-job-${String(mockJobs.value.length + 1).padStart(4, '0')}-${Math.random().toString(36).slice(2, 10)}`,
      name: submitConfig.value.job_name,
      project_id: submitConfig.value.project_id,
      status: 'queued',
      submit_time: new Date().toISOString(),
      start_time: '',
      end_time: '',
      priority: submitConfig.value.priority,
      node_count: submitConfig.value.node_count,
      cores_per_node: submitConfig.value.cores_per_node,
      memory_gb: submitConfig.value.memory_gb,
      wall_time_hours: submitConfig.value.wall_time_hours,
      estimated_time: '计算中...',
      progress: 0,
      input_file_path: submitConfig.value.input_files[0] || '',
      output_file_path: '',
      log_url: '',
      error_message: ''
    }
    mockJobs.value.unshift(newJob)
    clusterInfo.value.queue_length += 1
    submitConfig.value.job_name = ''
    submitConfig.value.input_files = []
  } finally {
    submitting.value = false
  }
}

async function handleCancel(jobId: string) {
  const job = mockJobs.value.find(j => j.id === jobId)
  if (job) {
    job.status = 'cancelled'
    job.end_time = new Date().toISOString()
    job.error_message = '用户手动取消'
    clusterInfo.value.queue_length = Math.max(0, clusterInfo.value.queue_length - 1)
  }
}

async function handleDownload(jobId: string) {
  const job = mockJobs.value.find(j => j.id === jobId)
  if (job) {
    alert(`开始下载任务 ${job.name} 的结果文件到本地...`)
  }
}

function refreshAll() {
  clusterInfo.value = {
    name: 'CAELab-Production-Cluster',
    total_nodes: 32,
    online_nodes: 28 + Math.floor(Math.random() * 4 - 2),
    total_cores: 1024,
    available_cores: 384 + Math.floor(Math.random() * 64 - 32),
    total_memory_gb: 4096,
    available_memory_gb: 1536 + Math.floor(Math.random() * 256 - 128),
    queue_length: 7 + Math.floor(Math.random() * 4 - 2),
    avg_wait_time_minutes: 12 + Math.floor(Math.random() * 6 - 3)
  }
  clusterInfo.value.online_nodes = Math.min(clusterInfo.value.online_nodes, clusterInfo.value.total_nodes)
  clusterInfo.value.available_cores = Math.max(0, Math.min(clusterInfo.value.available_cores, clusterInfo.value.total_cores))
  clusterInfo.value.available_memory_gb = Math.max(0, Math.min(clusterInfo.value.available_memory_gb, clusterInfo.value.total_memory_gb))
  clusterInfo.value.queue_length = Math.max(0, clusterInfo.value.queue_length)
}

onMounted(() => {
  refreshAll()
})
</script>
