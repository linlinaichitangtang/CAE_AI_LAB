<!--
  DFT 任务管理器视图 - DftTaskView.vue
  V1.7-003: DFT 任务队列管理 / 状态监控
  V1.7-008: DFT 云端提交 / SLURM-PBS 脚本生成
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">DFT 任务管理</h2>
        <p class="text-sm text-[var(--text-secondary)]">任务队列 / 状态监控 / SLURM/PBS 提交脚本生成</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="showTemplates = !showTemplates" class="btn btn-ghost text-xs">模板</button>
        <button @click="refreshAll" class="btn btn-ghost text-xs">刷新状态</button>
      </div>
    </div>

    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧面板 -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto flex-shrink-0">
        <!-- 提交任务 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('submit')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">提交任务</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.submit ? '&#9660;' : '&#9654;' }}</span>
          </button>
          <div v-show="sections.submit" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">任务名称</label>
              <input v-model="submitConfig.job_name" type="text" class="input w-full text-xs" placeholder="输入任务名称" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">DFT 代码</label>
              <select v-model="submitConfig.code" class="input w-full text-xs">
                <option value="vasp">VASP</option>
                <option value="quantum_espresso">Quantum ESPRESSO</option>
                <option value="abinit">ABINIT</option>
                <option value="castep">CASTEP</option>
              </select>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">输入目录</label>
              <input v-model="submitConfig.input_directory" type="text" class="input w-full text-xs" placeholder="/data/dft/calculations/" />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">核心数</label>
                <input v-model.number="submitConfig.num_cores" type="number" min="1" max="1024" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">内存 (GB)</label>
                <input v-model.number="submitConfig.memory_gb" type="number" min="1" max="2048" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">墙钟时间 (h)</label>
                <input v-model.number="submitConfig.wall_time_hours" type="number" min="1" max="168" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">优先级</label>
                <div class="flex items-center gap-1">
                  <input v-model.number="submitConfig.priority" type="range" min="1" max="10" class="flex-1" />
                  <span class="text-xs text-[var(--text-primary)] w-4 text-right">{{ submitConfig.priority }}</span>
                </div>
              </div>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">HPC 队列</label>
              <select v-model="submitConfig.hpc_queue" class="input w-full text-xs">
                <option value="slurm">SLURM</option>
                <option value="pbs">PBS</option>
                <option value="local">本地</option>
              </select>
            </div>
            <button @click="handleSubmit" :disabled="submitting" class="w-full btn btn-primary text-xs">
              {{ submitting ? '提交中...' : '提交任务' }}
            </button>
          </div>
        </div>

        <!-- 队列配置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('queue')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">队列配置</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.queue ? '&#9660;' : '&#9654;' }}</span>
          </button>
          <div v-show="sections.queue" class="px-4 pb-4 space-y-3">
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">调度器</label>
              <select v-model="queueConfig.scheduler" class="input w-full text-xs">
                <option value="slurm">SLURM</option>
                <option value="pbs">PBS</option>
                <option value="local">本地</option>
              </select>
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">分区</label>
              <input v-model="queueConfig.partition" type="text" class="input w-full text-xs" placeholder="compute" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">账户</label>
              <input v-model="queueConfig.account" type="text" class="input w-full text-xs" placeholder="research_group" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-secondary)] mb-1 block">QoS</label>
              <input v-model="queueConfig.qos" type="text" class="input w-full text-xs" placeholder="normal" />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">最大任务数</label>
                <input v-model.number="queueConfig.max_jobs" type="number" min="1" max="1000" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)]">最大核心数</label>
                <input v-model.number="queueConfig.max_cores_per_job" type="number" min="1" max="4096" class="input w-full text-xs" />
              </div>
            </div>
            <button @click="handleSaveQueueConfig" class="w-full btn btn-ghost text-xs border border-[var(--border-default)]">
              保存队列配置
            </button>
          </div>
        </div>

        <!-- 通知设置 -->
        <div class="border-b border-[var(--border-subtle)]">
          <button @click="toggleSection('notify')" class="w-full flex items-center justify-between px-4 py-3 hover:bg-[var(--bg-elevated)]">
            <span class="font-medium text-[var(--text-primary)] text-sm">通知设置</span>
            <span class="text-[var(--text-muted)] text-xs">{{ sections.notify ? '&#9660;' : '&#9654;' }}</span>
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

        <!-- 快捷模板 -->
        <div v-if="showTemplates" class="border-b border-[var(--border-subtle)]">
          <h4 class="text-xs font-semibold text-[var(--text-primary)] px-4 pt-3">快捷模板</h4>
          <div class="px-4 pb-4 space-y-2 mt-2">
            <button v-for="tpl in templates" :key="tpl.id" @click="applyTemplate(tpl)"
              class="w-full text-left px-3 py-2 bg-[var(--bg-elevated)] hover:bg-[var(--bg-base)] rounded-lg transition-colors">
              <div class="text-xs text-[var(--text-primary)] font-medium">{{ tpl.name }}</div>
              <div class="text-[10px] text-[var(--text-muted)]">{{ tpl.description }}</div>
              <div class="text-[10px] text-[var(--text-secondary)] mt-1">
                {{ tpl.code_label }} / {{ tpl.cores }} 核心 / {{ tpl.memory }} GB
              </div>
            </button>
          </div>
        </div>

        <!-- 脚本生成 -->
        <div class="px-4 py-3 space-y-2">
          <button @click="handleGenerateSlurm" class="w-full btn btn-ghost text-xs border border-[var(--border-default)]">
            生成 SLURM 脚本
          </button>
          <button @click="handleGeneratePbs" class="w-full btn btn-ghost text-xs border border-[var(--border-default)]">
            生成 PBS 脚本
          </button>
        </div>
      </div>

      <!-- 右侧区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- 统计卡片 -->
        <div class="grid grid-cols-4 gap-3 px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">运行中</div>
            <div class="text-lg font-semibold text-[var(--primary)]">{{ stats.running }}</div>
          </div>
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">排队中</div>
            <div class="text-lg font-semibold text-[var(--accent-yellow)]">{{ stats.queued }}</div>
          </div>
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">已完成</div>
            <div class="text-lg font-semibold text-[var(--accent-green)]">{{ stats.completed }}</div>
          </div>
          <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
            <div class="text-[10px] text-[var(--text-muted)]">失败</div>
            <div class="text-lg font-semibold text-[var(--accent-red)]">{{ stats.failed }}</div>
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
                <option value="submitting">提交中</option>
                <option value="completed">已完成</option>
                <option value="failed">失败</option>
                <option value="cancelled">已取消</option>
              </select>
              <select v-model="codeFilter" class="input text-xs py-1 px-2">
                <option value="">全部代码</option>
                <option value="vasp">VASP</option>
                <option value="quantum_espresso">QE</option>
                <option value="abinit">ABINIT</option>
                <option value="castep">CASTEP</option>
              </select>
            </div>
          </div>

          <table class="w-full text-xs">
            <thead>
              <tr class="text-[var(--text-muted)] border-b border-[var(--border-subtle)]">
                <th class="text-left py-2 px-2 font-medium">ID</th>
                <th class="text-left py-2 px-2 font-medium">名称</th>
                <th class="text-left py-2 px-2 font-medium">代码</th>
                <th class="text-left py-2 px-2 font-medium">状态</th>
                <th class="text-left py-2 px-2 font-medium">优先级</th>
                <th class="text-left py-2 px-2 font-medium">提交时间</th>
                <th class="text-left py-2 px-2 font-medium">耗时</th>
                <th class="text-left py-2 px-2 font-medium">操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="task in filteredTasks" :key="task.id"
                @click="selectTask(task)"
                :class="['border-b border-[var(--border-subtle)] cursor-pointer transition-colors',
                  selectedTask?.id === task.id ? 'bg-[var(--primary)]/10' : 'hover:bg-[var(--bg-elevated)]']">
                <td class="py-2 px-2 text-[var(--text-secondary)] font-mono">{{ task.id.slice(0, 8) }}</td>
                <td class="py-2 px-2 text-[var(--text-primary)]">{{ task.name }}</td>
                <td class="py-2 px-2">
                  <span :class="['px-2 py-0.5 rounded text-[10px] font-medium', codeClass(task.code)]">
                    {{ codeLabel(task.code) }}
                  </span>
                </td>
                <td class="py-2 px-2">
                  <span :class="['px-2 py-0.5 rounded-full text-[10px] font-medium', statusClass(task.status)]">
                    {{ statusLabel(task.status) }}
                  </span>
                </td>
                <td class="py-2 px-2 text-[var(--text-secondary)]">{{ task.priority }}</td>
                <td class="py-2 px-2 text-[var(--text-muted)]">{{ formatTime(task.submit_time) }}</td>
                <td class="py-2 px-2 text-[var(--text-muted)]">{{ formatWallTime(task) }}</td>
                <td class="py-2 px-2">
                  <div class="flex items-center gap-1">
                    <button v-if="task.status === 'queued' || task.status === 'running' || task.status === 'submitting'"
                      @click.stop="handleCancel(task.id)"
                      class="px-2 py-0.5 text-[10px] rounded border border-[var(--accent-red)] text-[var(--accent-red)] hover:bg-[var(--accent-red)]/10 transition-colors">
                      取消
                    </button>
                    <button v-if="task.status === 'completed' || task.status === 'failed'"
                      @click.stop="handleViewLog(task)"
                      class="px-2 py-0.5 text-[10px] rounded border border-[var(--primary)] text-[var(--primary)] hover:bg-[var(--primary)]/10 transition-colors">
                      日志
                    </button>
                  </div>
                </td>
              </tr>
              <tr v-if="filteredTasks.length === 0">
                <td colspan="8" class="py-8 text-center text-[var(--text-muted)]">暂无任务</td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- 任务详情面板 -->
        <div v-if="selectedTask" class="border-t border-[var(--border-subtle)] bg-[var(--bg-surface)] px-4 py-3 max-h-72 overflow-y-auto">
          <div class="flex items-center justify-between mb-2">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">任务详情: {{ selectedTask.name }}</h4>
            <button @click="selectedTask = null" class="text-[var(--text-muted)] hover:text-[var(--text-primary)] text-xs">&#10005;</button>
          </div>
          <div class="grid grid-cols-4 gap-4 text-xs">
            <div>
              <div class="text-[var(--text-muted)] mb-1">输入文件</div>
              <div class="text-[var(--text-secondary)] space-y-0.5">
                <div v-for="f in selectedTask.input_files" :key="f" class="truncate">{{ f }}</div>
                <div v-if="selectedTask.input_files.length === 0" class="text-[var(--text-muted)]">--</div>
              </div>
            </div>
            <div>
              <div class="text-[var(--text-muted)] mb-1">输出文件</div>
              <div class="text-[var(--text-secondary)] space-y-0.5">
                <div v-for="f in selectedTask.output_files" :key="f" class="truncate">{{ f }}</div>
                <div v-if="selectedTask.output_files.length === 0" class="text-[var(--text-muted)]">--</div>
              </div>
            </div>
            <div>
              <div class="text-[var(--text-muted)] mb-1">计算结果</div>
              <div class="text-[var(--text-secondary)]">
                总能量: {{ selectedTask.total_energy_eV !== null ? selectedTask.total_energy_eV.toFixed(4) + ' eV' : '--' }}<br />
                收敛: {{ selectedTask.convergence_achieved === null ? '--' : (selectedTask.convergence_achieved ? '是' : '否') }}<br />
                离子步数: {{ selectedTask.num_ionic_steps ?? '--' }}<br />
                电子步数: {{ selectedTask.num_electronic_steps ?? '--' }}
              </div>
            </div>
            <div>
              <div class="text-[var(--text-muted)] mb-1">运行信息</div>
              <div class="text-[var(--text-secondary)]">
                节点: {{ selectedTask.node_id || '--' }}<br />
                队列位置: {{ selectedTask.queue_position || '--' }}<br />
                耗时: {{ formatWallTime(selectedTask) }}<br />
                <a v-if="selectedTask.log_url" :href="selectedTask.log_url" target="_blank" class="text-[var(--primary)] hover:underline">查看日志 &#8594;</a>
              </div>
            </div>
          </div>
          <div v-if="selectedTask.error_message" class="mt-2 p-2 bg-[var(--accent-red)]/10 rounded text-[var(--accent-red)] text-xs">
            错误: {{ selectedTask.error_message }}
          </div>
        </div>

        <!-- SLURM/PBS 脚本预览 -->
        <div v-if="scriptPreview" class="border-t border-[var(--border-subtle)] bg-[var(--bg-surface)] px-4 py-3">
          <div class="flex items-center justify-between mb-2">
            <h4 class="text-sm font-semibold text-[var(--text-primary)]">{{ scriptPreviewTitle }}</h4>
            <div class="flex items-center gap-2">
              <button @click="copyScript" class="btn btn-ghost text-[10px] px-2 py-0.5">复制</button>
              <button @click="scriptPreview = null" class="text-[var(--text-muted)] hover:text-[var(--text-primary)] text-xs">&#10005;</button>
            </div>
          </div>
          <pre class="bg-[var(--bg-base)] border border-[var(--border-subtle)] rounded-lg p-3 text-xs text-[var(--text-secondary)] overflow-x-auto max-h-48 overflow-y-auto font-mono leading-relaxed">{{ scriptPreview }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type {
  DftTask,
  DftTaskStatus,
  DftCode,
  DftSubmitConfig,
  DftQueueConfig,
  DftTaskListResult
} from '../api/dftTask'

// ============ 响应式状态 ============

const sections = ref({
  submit: true,
  queue: false,
  notify: false
})

const showTemplates = ref(false)
const submitting = ref(false)
const statusFilter = ref('')
const codeFilter = ref('')
const selectedTask = ref<DftTask | null>(null)
const scriptPreview = ref<string | null>(null)
const scriptPreviewTitle = ref('')

const submitConfig = ref<DftSubmitConfig>({
  project_id: 'proj-dft-001',
  job_name: '',
  code: 'vasp',
  input_directory: '/data/dft/calculations/',
  num_cores: 32,
  wall_time_hours: 24,
  memory_gb: 64,
  priority: 5,
  callback_url: '',
  notification_email: '',
  hpc_queue: 'slurm'
})

const queueConfig = ref<DftQueueConfig>({
  scheduler: 'slurm',
  partition: 'compute',
  account: 'research_group',
  qos: 'normal',
  max_jobs: 100,
  max_cores_per_job: 256
})

// ============ 模拟数据 ============

const mockTasks = ref<DftTask[]>([
  {
    id: 'dft-task-0001-a3f8c2d1',
    name: 'Si-SCF 单点能计算',
    project_id: 'proj-semi-001',
    code: 'vasp',
    status: 'running',
    submit_time: '2026-04-03T06:00:00Z',
    start_time: '2026-04-03T06:02:00Z',
    end_time: '',
    priority: 8,
    input_files: ['POSCAR', 'INCAR', 'POTCAR', 'KPOINTS'],
    output_files: ['OUTCAR'],
    total_energy_eV: null,
    convergence_achieved: null,
    num_ionic_steps: null,
    num_electronic_steps: 42,
    wall_time_seconds: 1847,
    error_message: '',
    log_url: 'https://hpc.caelab.io/logs/dft-task-0001',
    node_id: 'node-compute-12',
    queue_position: 0
  },
  {
    id: 'dft-task-0002-b7e4d1f3',
    name: 'Fe2O3 结构优化',
    project_id: 'proj-oxide-002',
    code: 'vasp',
    status: 'queued',
    submit_time: '2026-04-03T07:30:00Z',
    start_time: '',
    end_time: '',
    priority: 6,
    input_files: ['POSCAR', 'INCAR', 'POTCAR', 'KPOINTS'],
    output_files: [],
    total_energy_eV: null,
    convergence_achieved: null,
    num_ionic_steps: null,
    num_electronic_steps: null,
    wall_time_seconds: null,
    error_message: '',
    log_url: '',
    node_id: '',
    queue_position: 3
  },
  {
    id: 'dft-task-0003-c9a2e5b7',
    name: 'Graphene 能带计算',
    project_id: 'proj-2d-003',
    code: 'quantum_espresso',
    status: 'completed',
    submit_time: '2026-04-02T14:00:00Z',
    start_time: '2026-04-02T14:05:00Z',
    end_time: '2026-04-02T16:32:00Z',
    priority: 7,
    input_files: ['pw.in', 'Graphene.cif', 'pseudo_upf/Si.pbe-n-rrkjus_psl.1.0.0.UPF'],
    output_files: ['pw.out', 'Graphene.bands.dat', 'Graphene.pw.wfc'],
    total_energy_eV: -456.7832,
    convergence_achieved: true,
    num_ionic_steps: 1,
    num_electronic_steps: 18,
    wall_time_seconds: 9120,
    error_message: '',
    log_url: 'https://hpc.caelab.io/logs/dft-task-0003',
    node_id: 'node-compute-05',
    queue_position: 0
  },
  {
    id: 'dft-task-0004-d1c8f3a5',
    name: 'TiO2 DOS 态密度',
    project_id: 'proj-semi-004',
    code: 'vasp',
    status: 'failed',
    submit_time: '2026-04-03T04:00:00Z',
    start_time: '2026-04-03T04:01:00Z',
    end_time: '2026-04-03T04:08:00Z',
    priority: 5,
    input_files: ['POSCAR', 'INCAR', 'POTCAR', 'KPOINTS'],
    output_files: [],
    total_energy_eV: null,
    convergence_achieved: false,
    num_ionic_steps: 0,
    num_electronic_steps: 3,
    wall_time_seconds: 420,
    error_message: 'EDDDAV: Call to ZHEGV failed. Returncode = 1 - 内存不足或 KPOINTS 网格过大',
    log_url: 'https://hpc.caelab.io/logs/dft-task-0004',
    node_id: 'node-compute-08',
    queue_position: 0
  },
  {
    id: 'dft-task-0005-e3b6d9c1',
    name: 'AlMg 合金声子谱',
    project_id: 'proj-alloy-005',
    code: 'quantum_espresso',
    status: 'running',
    submit_time: '2026-04-03T03:00:00Z',
    start_time: '2026-04-03T03:10:00Z',
    end_time: '',
    priority: 9,
    input_files: ['ph.in', 'AlMg.scf.in', 'pseudo_upf/Al.pbe-n-rrkjus_psl.1.0.0.UPF', 'pseudo_upf/Mg.pbe-n-rrkjus_psl.1.0.0.UPF'],
    output_files: ['AlMg.ph.out'],
    total_energy_eV: null,
    convergence_achieved: null,
    num_ionic_steps: null,
    num_electronic_steps: null,
    wall_time_seconds: 7200,
    error_message: '',
    log_url: 'https://hpc.caelab.io/logs/dft-task-0005',
    node_id: 'node-compute-15',
    queue_position: 0
  },
  {
    id: 'dft-task-0006-f5a7c2e8',
    name: 'ZnO 光学性质计算',
    project_id: 'proj-opt-006',
    code: 'abinit',
    status: 'cancelled',
    submit_time: '2026-04-02T20:00:00Z',
    start_time: '',
    end_time: '2026-04-02T20:05:00Z',
    priority: 3,
    input_files: ['ZnO.in', 'ZnO.files'],
    output_files: [],
    total_energy_eV: null,
    convergence_achieved: null,
    num_ionic_steps: null,
    num_electronic_steps: null,
    wall_time_seconds: null,
    error_message: '用户手动取消',
    log_url: '',
    node_id: '',
    queue_position: 0
  },
  {
    id: 'dft-task-0007-g8b3d4f6',
    name: 'Perovskite CaTiO3 弹性常数',
    project_id: 'proj-perov-007',
    code: 'castep',
    status: 'submitting',
    submit_time: '2026-04-03T08:00:00Z',
    start_time: '',
    end_time: '',
    priority: 7,
    input_files: ['CaTiO3.cell', 'CaTiO3.param', 'CaTiO3.pot'],
    output_files: [],
    total_energy_eV: null,
    convergence_achieved: null,
    num_ionic_steps: null,
    num_electronic_steps: null,
    wall_time_seconds: null,
    error_message: '',
    log_url: '',
    node_id: '',
    queue_position: 1
  },
  {
    id: 'dft-task-0008-h2c5e7a9',
    name: 'LiFePO4 电池材料 NEB',
    project_id: 'proj-battery-008',
    code: 'vasp',
    status: 'completed',
    submit_time: '2026-04-01T10:00:00Z',
    start_time: '2026-04-01T10:15:00Z',
    end_time: '2026-04-02T02:40:00Z',
    priority: 10,
    input_files: ['POSCAR', 'INCAR', 'POTCAR', 'KPOINTS', '00/POSCAR', '01/POSCAR', '02/POSCAR', '03/POSCAR', '04/POSCAR'],
    output_files: ['OUTCAR', 'vasprun.xml', 'CONTCAR'],
    total_energy_eV: -198.4521,
    convergence_achieved: true,
    num_ionic_steps: 5,
    num_electronic_steps: 156,
    wall_time_seconds: 59700,
    error_message: '',
    log_url: 'https://hpc.caelab.io/logs/dft-task-0008',
    node_id: 'node-compute-20',
    queue_position: 0
  }
])

const templates = ref([
  {
    id: 'tpl-dft-001',
    name: 'VASP-SCF 单点',
    description: '自洽场单点能计算，适用于基态能量和电荷密度',
    code: 'vasp' as DftCode,
    code_label: 'VASP',
    cores: 32,
    memory: 64,
    wall_time: 12,
    priority: 5
  },
  {
    id: 'tpl-dft-002',
    name: 'VASP-结构优化',
    description: '离子弛豫 + 晶格优化，适用于结构预测',
    code: 'vasp' as DftCode,
    code_label: 'VASP',
    cores: 64,
    memory: 128,
    wall_time: 48,
    priority: 7
  },
  {
    id: 'tpl-dft-003',
    name: 'QE-能带计算',
    description: 'Quantum ESPRESSO 能带结构和态密度计算',
    code: 'quantum_espresso' as DftCode,
    code_label: 'QE',
    cores: 48,
    memory: 96,
    wall_time: 24,
    priority: 6
  },
  {
    id: 'tpl-dft-004',
    name: '高通量批量',
    description: '批量提交多个结构的高通量 DFT 计算',
    code: 'vasp' as DftCode,
    code_label: 'VASP',
    cores: 16,
    memory: 32,
    wall_time: 8,
    priority: 4
  }
])

// ============ 计算属性 ============

const filteredTasks = computed(() => {
  let result = mockTasks.value
  if (statusFilter.value) {
    result = result.filter(t => t.status === statusFilter.value)
  }
  if (codeFilter.value) {
    result = result.filter(t => t.code === codeFilter.value)
  }
  return result
})

const stats = computed(() => {
  const tasks = mockTasks.value
  return {
    running: tasks.filter(t => t.status === 'running').length,
    queued: tasks.filter(t => t.status === 'queued' || t.status === 'submitting').length,
    completed: tasks.filter(t => t.status === 'completed').length,
    failed: tasks.filter(t => t.status === 'failed').length
  }
})

// ============ 方法 ============

function toggleSection(key: keyof typeof sections.value) {
  sections.value[key] = !sections.value[key]
}

function statusClass(status: DftTaskStatus): string {
  const map: Record<DftTaskStatus, string> = {
    queued: 'bg-[var(--accent-yellow)]/20 text-[var(--accent-yellow)]',
    running: 'bg-[var(--primary)]/20 text-[var(--primary)]',
    completed: 'bg-[var(--accent-green)]/20 text-[var(--accent-green)]',
    failed: 'bg-[var(--accent-red)]/20 text-[var(--accent-red)]',
    cancelled: 'bg-gray-500/20 text-gray-400',
    submitting: 'bg-blue-500/20 text-blue-400'
  }
  return map[status] || ''
}

function statusLabel(status: DftTaskStatus): string {
  const map: Record<DftTaskStatus, string> = {
    queued: '排队中',
    running: '运行中',
    completed: '已完成',
    failed: '失败',
    cancelled: '已取消',
    submitting: '提交中'
  }
  return map[status] || status
}

function codeClass(code: DftCode): string {
  const map: Record<DftCode, string> = {
    vasp: 'bg-orange-500/20 text-orange-400',
    quantum_espresso: 'bg-cyan-500/20 text-cyan-400',
    abinit: 'bg-purple-500/20 text-purple-400',
    castep: 'bg-emerald-500/20 text-emerald-400'
  }
  return map[code] || ''
}

function codeLabel(code: DftCode): string {
  const map: Record<DftCode, string> = {
    vasp: 'VASP',
    quantum_espresso: 'QE',
    abinit: 'ABINIT',
    castep: 'CASTEP'
  }
  return map[code] || code
}

function formatTime(iso: string): string {
  if (!iso) return '--'
  const d = new Date(iso)
  return d.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

function formatWallTime(task: DftTask): string {
  if (task.wall_time_seconds === null || task.wall_time_seconds === undefined) return '--'
  const hours = Math.floor(task.wall_time_seconds / 3600)
  const minutes = Math.floor((task.wall_time_seconds % 3600) / 60)
  const seconds = task.wall_time_seconds % 60
  if (hours > 0) {
    return `${hours}h ${minutes}m`
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds}s`
  }
  return `${seconds}s`
}

function selectTask(task: DftTask) {
  selectedTask.value = task
}

function applyTemplate(tpl: typeof templates.value[0]) {
  submitConfig.value.code = tpl.code
  submitConfig.value.num_cores = tpl.cores
  submitConfig.value.memory_gb = tpl.memory
  submitConfig.value.wall_time_hours = tpl.wall_time
  submitConfig.value.priority = tpl.priority
  showTemplates.value = false
}

async function handleSubmit() {
  if (!submitConfig.value.job_name) return
  submitting.value = true
  try {
    const newTask: DftTask = {
      id: `dft-task-${String(mockTasks.value.length + 1).padStart(4, '0')}-${Math.random().toString(36).slice(2, 10)}`,
      name: submitConfig.value.job_name,
      project_id: submitConfig.value.project_id,
      code: submitConfig.value.code,
      status: 'submitting',
      submit_time: new Date().toISOString(),
      start_time: '',
      end_time: '',
      priority: submitConfig.value.priority,
      input_files: ['POSCAR', 'INCAR', 'POTCAR', 'KPOINTS'],
      output_files: [],
      total_energy_eV: null,
      convergence_achieved: null,
      num_ionic_steps: null,
      num_electronic_steps: null,
      wall_time_seconds: null,
      error_message: '',
      log_url: '',
      node_id: '',
      queue_position: mockTasks.value.filter(t => t.status === 'queued' || t.status === 'submitting').length + 1
    }
    mockTasks.value.unshift(newTask)
    submitConfig.value.job_name = ''

    // 模拟提交完成后变为 queued
    setTimeout(() => {
      newTask.status = 'queued'
    }, 2000)
  } finally {
    submitting.value = false
  }
}

async function handleCancel(jobId: string) {
  const task = mockTasks.value.find(t => t.id === jobId)
  if (task) {
    task.status = 'cancelled'
    task.end_time = new Date().toISOString()
    task.error_message = '用户手动取消'
    task.queue_position = 0
  }
}

function handleViewLog(task: DftTask) {
  if (task.log_url) {
    window.open(task.log_url, '_blank')
  }
}

function handleSaveQueueConfig() {
  // 模拟保存队列配置
  const config = JSON.stringify(queueConfig.value, null, 2)
  scriptPreviewTitle.value = '队列配置 (JSON)'
  scriptPreview.value = config
}

function handleGenerateSlurm() {
  const cfg = submitConfig.value
  const partition = queueConfig.value.partition || 'compute'
  const account = queueConfig.value.account || 'research'
  const hours = Math.floor(cfg.wall_time_hours)
  const mins = Math.round((cfg.wall_time_hours - hours) * 60)
  const timeStr = `${String(hours).padStart(2, '0')}:${String(mins).padStart(2, '0')}:00`

  const script = `#!/bin/bash
#SBATCH --job-name=${cfg.job_name || 'dft-calc'}
#SBATCH --partition=${partition}
#SBATCH --account=${account}
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=${cfg.num_cores}
#SBATCH --cpus-per-task=1
#SBATCH --mem=${cfg.memory_gb}G
#SBATCH --time=${timeStr}
#SBATCH --output=dft_%j.out
#SBATCH --error=dft_%j.err
#SBATCH --priority=${cfg.priority}

# ============================================
# DFT Calculation Job - Generated by CAELab
# Code: ${codeLabel(cfg.code)}
# ============================================

module purge
module load ${cfg.code === 'vasp' ? 'vasp/6.5.0-intel-2023' : cfg.code === 'quantum_espresso' ? 'quantum-espresso/7.3.1-openmpi' : cfg.code === 'abinit' ? 'abinit/10.2-intel' : 'castep/24.1-intel'}

echo "Job started at: $(date)"
echo "Working directory: $SLURM_SUBMIT_DIR"
echo "Node list: $SLURM_NODELIST"
echo "Number of cores: $SLURM_NTASKS"

cd $SLURM_SUBMIT_DIR

# Run DFT calculation
${cfg.code === 'vasp' ? 'mpirun -np $SLURM_NTASKS vasp_std' : cfg.code === 'quantum_espresso' ? 'mpirun -np $SLURM_NTASKS pw.x < pw.in > pw.out' : cfg.code === 'abinit' ? 'mpirun -np $SLURM_NTASKS abinit < input.files > abinit.log' : 'mpirun -np $SLURM_NTASKS castep.mpi CaTiO3'}

echo "Job finished at: $(date)"

# Post-processing
if [ -f OUTCAR ]; then
    echo "=== Final Energy ==="
    grep "energy without" OUTCAR | tail -1
fi

if [ -f pw.out ]; then
    echo "=== Final Energy ==="
    grep "!.*total energy" pw.out | tail -1
fi`

  scriptPreviewTitle.value = 'SLURM 提交脚本'
  scriptPreview.value = script
}

function handleGeneratePbs() {
  const cfg = submitConfig.value
  const queue = queueConfig.value.partition || 'workq'
  const account = queueConfig.value.account || 'research'
  const hours = Math.floor(cfg.wall_time_hours)
  const mins = Math.round((cfg.wall_time_hours - hours) * 60)
  const timeStr = `${String(hours).padStart(2, '0')}:${String(mins).padStart(2, '0')}:00`

  const script = `#!/bin/bash
#PBS -N ${cfg.job_name || 'dft-calc'}
#PBS -q ${queue}
#PBS -A ${account}
#PBS -l nodes=1:ppn=${cfg.num_cores}
#PBS -l mem=${cfg.memory_gb}gb
#PBS -l walltime=${timeStr}
#PBS -o dft_\$PBS_JOBID.out
#PBS -e dft_\$PBS_JOBID.err
#PBS -p ${cfg.priority}

# ============================================
# DFT Calculation Job - Generated by CAELab
# Code: ${codeLabel(cfg.code)}
# ============================================

module purge
module load ${cfg.code === 'vasp' ? 'vasp/6.5.0-intel-2023' : cfg.code === 'quantum_espresso' ? 'quantum-espresso/7.3.1-openmpi' : cfg.code === 'abinit' ? 'abinit/10.2-intel' : 'castep/24.1-intel'}

echo "Job started at: $(date)"
echo "Working directory: $PBS_O_WORKDIR"
echo "Job ID: $PBS_JOBID"
echo "Number of cores: $PBS_NP"

cd $PBS_O_WORKDIR

# Run DFT calculation
${cfg.code === 'vasp' ? 'mpirun -np $PBS_NP vasp_std' : cfg.code === 'quantum_espresso' ? 'mpirun -np $PBS_NP pw.x < pw.in > pw.out' : cfg.code === 'abinit' ? 'mpirun -np $PBS_NP abinit < input.files > abinit.log' : 'mpirun -np $PBS_NP castep.mpi CaTiO3'}

echo "Job finished at: $(date)"

# Post-processing
if [ -f OUTCAR ]; then
    echo "=== Final Energy ==="
    grep "energy without" OUTCAR | tail -1
fi

if [ -f pw.out ]; then
    echo "=== Final Energy ==="
    grep "!.*total energy" pw.out | tail -1
fi`

  scriptPreviewTitle.value = 'PBS 提交脚本'
  scriptPreview.value = script
}

function copyScript() {
  if (scriptPreview.value) {
    navigator.clipboard.writeText(scriptPreview.value).then(() => {
      // 复制成功
    }).catch(() => {
      // fallback
      const textarea = document.createElement('textarea')
      textarea.value = scriptPreview.value!
      document.body.appendChild(textarea)
      textarea.select()
      document.execCommand('copy')
      document.body.removeChild(textarea)
    })
  }
}

function refreshAll() {
  // 模拟刷新: 随机更新运行中任务的电子步数和耗时
  mockTasks.value.forEach(task => {
    if (task.status === 'running') {
      task.wall_time_seconds = (task.wall_time_seconds || 0) + Math.floor(Math.random() * 120) + 30
      if (task.num_electronic_steps !== null) {
        task.num_electronic_steps += Math.floor(Math.random() * 5) + 1
      }
    }
  })
}

onMounted(() => {
  refreshAll()
})
</script>
