<!--
  HPC 集群管理视图 - HpcClusterView.vue
  V1.4-006: 节点状态 / 队列排名 / 预估完成时间，中断恢复
-->
<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">HPC 集群管理</h2>
        <p class="text-sm text-[var(--text-secondary)]">节点状态 / 队列排名 / 预估完成时间，中断恢复</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="refreshCluster" class="btn btn-ghost text-xs">刷新</button>
      </div>
    </div>

    <!-- 顶部统计卡片行 -->
    <div class="grid grid-cols-6 gap-3 px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
        <div class="text-[10px] text-[var(--text-muted)]">在线节点</div>
        <div class="text-lg font-semibold text-[var(--accent-green)]">{{ metricsStat.onlineNodes }}</div>
      </div>
      <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
        <div class="text-[10px] text-[var(--text-muted)]">总核心</div>
        <div class="text-lg font-semibold text-[var(--text-primary)]">{{ metricsStat.totalCores }}</div>
      </div>
      <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
        <div class="text-[10px] text-[var(--text-muted)]">队列长度</div>
        <div class="text-lg font-semibold" :class="metricsStat.queueLength > 10 ? 'text-[var(--accent-red)]' : 'text-[var(--accent-yellow)]'">
          {{ metricsStat.queueLength }}
        </div>
      </div>
      <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
        <div class="text-[10px] text-[var(--text-muted)]">今日完成</div>
        <div class="text-lg font-semibold text-[var(--accent-green)]">{{ metricsStat.completedToday }}</div>
      </div>
      <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
        <div class="text-[10px] text-[var(--text-muted)]">今日失败</div>
        <div class="text-lg font-semibold" :class="metricsStat.failedToday > 0 ? 'text-[var(--accent-red)]' : 'text-[var(--text-primary)]'">
          {{ metricsStat.failedToday }}
        </div>
      </div>
      <div class="bg-[var(--bg-elevated)] rounded-lg px-3 py-2">
        <div class="text-[10px] text-[var(--text-muted)]">平均等待</div>
        <div class="text-lg font-semibold text-[var(--text-primary)]">{{ metricsStat.avgWait }}min</div>
      </div>
    </div>

    <!-- 主内容区域 -->
    <div class="flex-1 flex overflow-hidden">
      <!-- 左侧: 节点列表 -->
      <div class="w-72 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto flex-shrink-0">
        <div class="px-3 py-2 border-b border-[var(--border-subtle)]">
          <h3 class="text-xs font-semibold text-[var(--text-primary)]">计算节点</h3>
        </div>
        <div class="p-2 space-y-2">
          <div v-for="node in nodes" :key="node.id"
            @click="selectNode(node)"
            :class="['rounded-lg border p-3 cursor-pointer transition-colors',
              selectedNode?.id === node.id
                ? 'border-[var(--primary)] bg-[var(--primary)]/5'
                : 'border-[var(--border-subtle)] hover:border-[var(--border-default)] bg-[var(--bg-elevated)]']">
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs font-medium text-[var(--text-primary)]">{{ node.hostname }}</span>
              <span :class="['text-[10px] px-1.5 py-0.5 rounded-full font-medium', nodeStatusClass(node.status)]">
                {{ nodeStatusLabel(node.status) }}
              </span>
            </div>
            <div class="text-[10px] text-[var(--text-muted)] mb-2">{{ node.ip }}</div>
            <!-- CPU 使用率 -->
            <div class="mb-1.5">
              <div class="flex items-center justify-between text-[10px] mb-0.5">
                <span class="text-[var(--text-muted)]">CPU</span>
                <span class="text-[var(--text-secondary)]">{{ node.cpu_usage }}%</span>
              </div>
              <div class="w-full h-1 bg-[var(--bg-base)] rounded-full overflow-hidden">
                <div class="h-full rounded-full transition-all duration-300"
                  :class="usageColorClass(node.cpu_usage)"
                  :style="{ width: node.cpu_usage + '%' }"></div>
              </div>
            </div>
            <!-- GPU 使用率 -->
            <div v-if="node.gpu_count > 0" class="mb-1.5">
              <div class="flex items-center justify-between text-[10px] mb-0.5">
                <span class="text-[var(--text-muted)]">GPU</span>
                <span class="text-[var(--text-secondary)]">{{ node.gpu_usage }}%</span>
              </div>
              <div class="w-full h-1 bg-[var(--bg-base)] rounded-full overflow-hidden">
                <div class="h-full rounded-full transition-all duration-300"
                  :class="usageColorClass(node.gpu_usage)"
                  :style="{ width: node.gpu_usage + '%' }"></div>
              </div>
            </div>
            <!-- 内存 -->
            <div class="mb-1.5">
              <div class="flex items-center justify-between text-[10px] mb-0.5">
                <span class="text-[var(--text-muted)]">内存</span>
                <span class="text-[var(--text-secondary)]">{{ node.memory_used_gb }}/{{ node.memory_total_gb }} GB</span>
              </div>
              <div class="w-full h-1 bg-[var(--bg-base)] rounded-full overflow-hidden">
                <div class="h-full rounded-full bg-blue-400 transition-all duration-300"
                  :style="{ width: (node.memory_used_gb / node.memory_total_gb * 100) + '%' }"></div>
              </div>
            </div>
            <!-- 底部信息 -->
            <div class="flex items-center justify-between text-[10px] text-[var(--text-muted)] mt-2">
              <span>{{ node.temperature }}°C</span>
              <span>{{ node.jobs_running.length }} 任务</span>
              <span>{{ node.cpu_cores }} 核</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧区域 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- 右侧上: 队列管理 -->
        <div class="flex-1 overflow-auto px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">队列管理</h3>
            <span class="text-xs text-[var(--text-muted)]">{{ queueJobs.length }} 个任务</span>
          </div>
          <table class="w-full text-xs">
            <thead>
              <tr class="text-[var(--text-muted)] border-b border-[var(--border-subtle)]">
                <th class="text-left py-2 px-2 font-medium w-12">#</th>
                <th class="text-left py-2 px-2 font-medium">任务名</th>
                <th class="text-left py-2 px-2 font-medium">用户</th>
                <th class="text-left py-2 px-2 font-medium w-12">优先级</th>
                <th class="text-left py-2 px-2 font-medium">提交时间</th>
                <th class="text-left py-2 px-2 font-medium">预估开始</th>
                <th class="text-left py-2 px-2 font-medium">状态</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(job, idx) in queueJobs" :key="job.id"
                class="border-b border-[var(--border-subtle)] hover:bg-[var(--bg-elevated)] transition-colors">
                <td class="py-2 px-2 text-[var(--text-muted)]">{{ idx + 1 }}</td>
                <td class="py-2 px-2 text-[var(--text-primary)]">{{ job.name }}</td>
                <td class="py-2 px-2 text-[var(--text-secondary)]">{{ job.user }}</td>
                <td class="py-2 px-2">
                  <span :class="['px-1.5 py-0.5 rounded text-[10px] font-medium',
                    job.priority >= 8 ? 'bg-[var(--accent-red)]/20 text-[var(--accent-red)]' :
                    job.priority >= 5 ? 'bg-[var(--accent-yellow)]/20 text-[var(--accent-yellow)]' :
                    'bg-gray-500/20 text-gray-400']">
                    P{{ job.priority }}
                  </span>
                </td>
                <td class="py-2 px-2 text-[var(--text-muted)]">{{ formatTime(job.submit_time) }}</td>
                <td class="py-2 px-2 text-[var(--text-secondary)]">{{ job.estimated_start ? formatTime(job.estimated_start) : '--' }}</td>
                <td class="py-2 px-2">
                  <span :class="['px-2 py-0.5 rounded-full text-[10px] font-medium', queueStatusClass(job.status)]">
                    {{ queueStatusLabel(job.status) }}
                  </span>
                </td>
              </tr>
              <tr v-if="queueJobs.length === 0">
                <td colspan="7" class="py-8 text-center text-[var(--text-muted)]">队列为空</td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- 右侧下: 集群告警 -->
        <div class="h-48 overflow-auto px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">集群告警</h3>
            <span class="text-xs text-[var(--text-muted)]">{{ alerts.filter(a => !a.acknowledged).length }} 条未确认</span>
          </div>
          <div class="space-y-2">
            <div v-for="alert in alerts" :key="alert.id"
              :class="['flex items-start justify-between px-3 py-2 rounded-lg border transition-colors',
                alertSeverityBorder(alert.severity),
                alert.acknowledged ? 'opacity-50' : '']">
              <div class="flex items-start gap-2 flex-1">
                <span :class="['text-xs mt-0.5', alertSeverityIcon(alert.severity)]">
                  {{ alert.severity === 'critical' ? '●' : alert.severity === 'warning' ? '●' : '●' }}
                </span>
                <div class="flex-1">
                  <div class="text-xs text-[var(--text-primary)]">{{ alert.message }}</div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-0.5">
                    节点: {{ alert.node_id }} | {{ formatTime(alert.timestamp) }}
                  </div>
                </div>
              </div>
              <button v-if="!alert.acknowledged" @click="ackAlert(alert.id)"
                class="btn btn-ghost text-[10px] py-0.5 px-2 flex-shrink-0 ml-2">
                确认
              </button>
              <span v-else class="text-[10px] text-[var(--text-muted)] flex-shrink-0 ml-2">已确认</span>
            </div>
            <div v-if="alerts.length === 0" class="py-4 text-center text-xs text-[var(--text-muted)]">暂无告警</div>
          </div>
        </div>

        <!-- 底部: 资源使用趋势图 -->
        <div class="h-48 px-4 py-3 flex-shrink-0">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">资源使用趋势</h3>
          <canvas ref="chartCanvas" class="w-full h-32 rounded-lg bg-[var(--bg-elevated)]"></canvas>
        </div>
      </div>
    </div>

    <!-- 节点详情弹窗 -->
    <div v-if="selectedNode" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="selectedNode = null">
      <div class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] shadow-xl w-[480px] max-h-[80vh] overflow-y-auto">
        <div class="flex items-center justify-between px-4 py-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)]">节点详情: {{ selectedNode.hostname }}</h3>
          <button @click="selectedNode = null" class="text-[var(--text-muted)] hover:text-[var(--text-primary)]">✕</button>
        </div>
        <div class="px-4 py-3 space-y-4">
          <!-- 基本信息 -->
          <div class="grid grid-cols-2 gap-3 text-xs">
            <div>
              <div class="text-[var(--text-muted)]">主机名</div>
              <div class="text-[var(--text-primary)]">{{ selectedNode.hostname }}</div>
            </div>
            <div>
              <div class="text-[var(--text-muted)]">IP 地址</div>
              <div class="text-[var(--text-primary)] font-mono">{{ selectedNode.ip }}</div>
            </div>
            <div>
              <div class="text-[var(--text-muted)]">状态</div>
              <span :class="['px-2 py-0.5 rounded-full text-[10px] font-medium', nodeStatusClass(selectedNode.status)]">
                {{ nodeStatusLabel(selectedNode.status) }}
              </span>
            </div>
            <div>
              <div class="text-[var(--text-muted)]">最后心跳</div>
              <div class="text-[var(--text-primary)]">{{ formatTime(selectedNode.last_heartbeat) }}</div>
            </div>
          </div>

          <!-- CPU -->
          <div>
            <div class="text-xs text-[var(--text-muted)] mb-1">CPU: {{ selectedNode.cpu_model }}</div>
            <div class="flex items-center gap-2">
              <div class="flex-1 h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
                <div class="h-full rounded-full transition-all duration-300"
                  :class="usageColorClass(selectedNode.cpu_usage)"
                  :style="{ width: selectedNode.cpu_usage + '%' }"></div>
              </div>
              <span class="text-xs text-[var(--text-secondary)]">{{ selectedNode.cpu_usage }}% ({{ selectedNode.cpu_cores }} 核)</span>
            </div>
          </div>

          <!-- 内存 -->
          <div>
            <div class="text-xs text-[var(--text-muted)] mb-1">内存</div>
            <div class="flex items-center gap-2">
              <div class="flex-1 h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
                <div class="h-full rounded-full bg-blue-400 transition-all duration-300"
                  :style="{ width: (selectedNode.memory_used_gb / selectedNode.memory_total_gb * 100) + '%' }"></div>
              </div>
              <span class="text-xs text-[var(--text-secondary)]">{{ selectedNode.memory_used_gb }} / {{ selectedNode.memory_total_gb }} GB</span>
            </div>
          </div>

          <!-- GPU -->
          <div v-if="selectedNode.gpu_count > 0">
            <div class="text-xs text-[var(--text-muted)] mb-1">GPU: {{ selectedNode.gpu_model }} x{{ selectedNode.gpu_count }}</div>
            <div class="flex items-center gap-2">
              <div class="flex-1 h-2 bg-[var(--bg-base)] rounded-full overflow-hidden">
                <div class="h-full rounded-full bg-purple-400 transition-all duration-300"
                  :style="{ width: selectedNode.gpu_usage + '%' }"></div>
              </div>
              <span class="text-xs text-[var(--text-secondary)]">{{ selectedNode.gpu_usage }}%</span>
            </div>
          </div>

          <!-- 磁盘 & 网络 -->
          <div class="grid grid-cols-2 gap-3 text-xs">
            <div>
              <div class="text-[var(--text-muted)]">磁盘</div>
              <div class="text-[var(--text-primary)]">{{ selectedNode.disk_used_gb }} / {{ selectedNode.disk_total_gb }} GB</div>
            </div>
            <div>
              <div class="text-[var(--text-muted)]">网络带宽</div>
              <div class="text-[var(--text-primary)]">{{ selectedNode.network_bandwidth_gbps }} Gbps</div>
            </div>
            <div>
              <div class="text-[var(--text-muted)]">温度</div>
              <div class="text-[var(--text-primary)]" :class="selectedNode.temperature > 80 ? 'text-[var(--accent-red)]' : ''">
                {{ selectedNode.temperature }}°C
              </div>
            </div>
            <div>
              <div class="text-[var(--text-muted)]">功耗</div>
              <div class="text-[var(--text-primary)]">{{ selectedNode.power_consumption_w }} W</div>
            </div>
          </div>

          <!-- 运行任务 -->
          <div>
            <div class="text-xs text-[var(--text-muted)] mb-1">运行中的任务 ({{ selectedNode.jobs_running.length }})</div>
            <div v-if="selectedNode.jobs_running.length > 0" class="space-y-1">
              <div v-for="jid in selectedNode.jobs_running" :key="jid"
                class="px-2 py-1 bg-[var(--bg-elevated)] rounded text-[10px] text-[var(--text-secondary)] font-mono">
                {{ jid }}
              </div>
            </div>
            <div v-else class="text-[10px] text-[var(--text-muted)]">无运行任务</div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex items-center gap-2 pt-2 border-t border-[var(--border-subtle)]">
            <button @click="handleDrain(selectedNode.id)"
              :disabled="selectedNode.status === 'offline' || selectedNode.status === 'maintenance'"
              class="btn btn-ghost text-xs flex-1">
              排空节点
            </button>
            <button @click="handleRestart(selectedNode.id)"
              class="btn btn-ghost text-xs flex-1 text-[var(--accent-red)]">
              重启节点
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from 'vue'
import type { ClusterNode, QueueJob, ClusterMetrics, ClusterAlert } from '../api/hpcCluster'

// ============ 响应式状态 ============

const selectedNode = ref<ClusterNode | null>(null)
const chartCanvas = ref<HTMLCanvasElement | null>(null)

// ============ 模拟数据 ============

const nodes = ref<ClusterNode[]>([
  {
    id: 'node-001',
    hostname: 'hpc-node-001',
    ip: '10.0.1.101',
    status: 'busy',
    cpu_model: 'AMD EPYC 7763',
    cpu_cores: 64,
    cpu_usage: 92,
    memory_total_gb: 256,
    memory_used_gb: 224,
    gpu_count: 4,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 78,
    disk_total_gb: 2048,
    disk_used_gb: 856,
    network_bandwidth_gbps: 100,
    temperature: 72,
    power_consumption_w: 850,
    jobs_running: ['hpc-job-0001', 'hpc-job-0005'],
    last_heartbeat: '2026-04-02T10:28:55Z'
  },
  {
    id: 'node-002',
    hostname: 'hpc-node-002',
    ip: '10.0.1.102',
    status: 'busy',
    cpu_model: 'AMD EPYC 7763',
    cpu_cores: 64,
    cpu_usage: 87,
    memory_total_gb: 256,
    memory_used_gb: 198,
    gpu_count: 4,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 65,
    disk_total_gb: 2048,
    disk_used_gb: 742,
    network_bandwidth_gbps: 100,
    temperature: 68,
    power_consumption_w: 790,
    jobs_running: ['hpc-job-0002'],
    last_heartbeat: '2026-04-02T10:28:53Z'
  },
  {
    id: 'node-003',
    hostname: 'hpc-node-003',
    ip: '10.0.1.103',
    status: 'idle',
    cpu_model: 'Intel Xeon Platinum 8380',
    cpu_cores: 40,
    cpu_usage: 5,
    memory_total_gb: 128,
    memory_used_gb: 18,
    gpu_count: 2,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 0,
    disk_total_gb: 1024,
    disk_used_gb: 312,
    network_bandwidth_gbps: 50,
    temperature: 35,
    power_consumption_w: 280,
    jobs_running: [],
    last_heartbeat: '2026-04-02T10:28:58Z'
  },
  {
    id: 'node-004',
    hostname: 'hpc-node-004',
    ip: '10.0.1.104',
    status: 'busy',
    cpu_model: 'Intel Xeon Platinum 8380',
    cpu_cores: 40,
    cpu_usage: 76,
    memory_total_gb: 128,
    memory_used_gb: 96,
    gpu_count: 2,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 55,
    disk_total_gb: 1024,
    disk_used_gb: 523,
    network_bandwidth_gbps: 50,
    temperature: 61,
    power_consumption_w: 620,
    jobs_running: ['hpc-job-0007', 'hpc-job-0008'],
    last_heartbeat: '2026-04-02T10:28:50Z'
  },
  {
    id: 'node-005',
    hostname: 'hpc-node-005',
    ip: '10.0.1.105',
    status: 'maintenance',
    cpu_model: 'AMD EPYC 7763',
    cpu_cores: 64,
    cpu_usage: 0,
    memory_total_gb: 256,
    memory_used_gb: 0,
    gpu_count: 4,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 0,
    disk_total_gb: 2048,
    disk_used_gb: 645,
    network_bandwidth_gbps: 100,
    temperature: 28,
    power_consumption_w: 45,
    jobs_running: [],
    last_heartbeat: '2026-04-01T22:00:00Z'
  },
  {
    id: 'node-006',
    hostname: 'hpc-node-006',
    ip: '10.0.1.106',
    status: 'online',
    cpu_model: 'Intel Xeon Platinum 8380',
    cpu_cores: 40,
    cpu_usage: 12,
    memory_total_gb: 128,
    memory_used_gb: 24,
    gpu_count: 0,
    gpu_model: '',
    gpu_usage: 0,
    disk_total_gb: 1024,
    disk_used_gb: 198,
    network_bandwidth_gbps: 50,
    temperature: 38,
    power_consumption_w: 310,
    jobs_running: [],
    last_heartbeat: '2026-04-02T10:28:57Z'
  },
  {
    id: 'node-007',
    hostname: 'hpc-node-007',
    ip: '10.0.1.107',
    status: 'offline',
    cpu_model: 'AMD EPYC 7763',
    cpu_cores: 64,
    cpu_usage: 0,
    memory_total_gb: 256,
    memory_used_gb: 0,
    gpu_count: 4,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 0,
    disk_total_gb: 2048,
    disk_used_gb: 910,
    network_bandwidth_gbps: 100,
    temperature: 0,
    power_consumption_w: 0,
    jobs_running: [],
    last_heartbeat: '2026-03-30T15:00:00Z'
  },
  {
    id: 'node-008',
    hostname: 'hpc-node-008',
    ip: '10.0.1.108',
    status: 'idle',
    cpu_model: 'Intel Xeon Platinum 8380',
    cpu_cores: 40,
    cpu_usage: 3,
    memory_total_gb: 128,
    memory_used_gb: 12,
    gpu_count: 2,
    gpu_model: 'NVIDIA A100',
    gpu_usage: 0,
    disk_total_gb: 1024,
    disk_used_gb: 267,
    network_bandwidth_gbps: 50,
    temperature: 33,
    power_consumption_w: 265,
    jobs_running: [],
    last_heartbeat: '2026-04-02T10:28:56Z'
  }
])

const queueJobs = ref<QueueJob[]>([
  {
    id: 'qjob-001',
    name: '机翼结构静力分析',
    user: 'zhangwei',
    priority: 8,
    submit_time: '2026-04-02T08:30:00Z',
    estimated_start: '2026-04-02T08:35:00Z',
    wall_time_requested: 12,
    status: 'running',
    dependencies: []
  },
  {
    id: 'qjob-002',
    name: '发动机叶片热分析',
    user: 'liming',
    priority: 6,
    submit_time: '2026-04-02T09:15:00Z',
    estimated_start: '2026-04-02T09:30:00Z',
    wall_time_requested: 24,
    status: 'running',
    dependencies: []
  },
  {
    id: 'qjob-003',
    name: '管道流场瞬态分析',
    user: 'wangfang',
    priority: 7,
    submit_time: '2026-04-02T10:00:00Z',
    estimated_start: '2026-04-02T10:20:00Z',
    wall_time_requested: 8,
    status: 'queued',
    dependencies: []
  },
  {
    id: 'qjob-004',
    name: '车身NVH分析',
    user: 'chenyu',
    priority: 5,
    submit_time: '2026-04-02T10:05:00Z',
    estimated_start: '2026-04-02T10:45:00Z',
    wall_time_requested: 6,
    status: 'queued',
    dependencies: []
  },
  {
    id: 'qjob-005',
    name: '复合材料层合板分析',
    user: 'zhaolei',
    priority: 4,
    submit_time: '2026-04-02T10:10:00Z',
    estimated_start: '2026-04-02T11:00:00Z',
    wall_time_requested: 16,
    status: 'queued',
    dependencies: ['qjob-001']
  },
  {
    id: 'qjob-006',
    name: '散热器热仿真',
    user: 'sunhao',
    priority: 3,
    submit_time: '2026-04-02T10:12:00Z',
    estimated_start: '2026-04-02T11:30:00Z',
    wall_time_requested: 4,
    status: 'queued',
    dependencies: []
  },
  {
    id: 'qjob-007',
    name: '涡轮叶片参数化优化',
    user: 'liwei',
    priority: 9,
    submit_time: '2026-04-02T10:15:00Z',
    estimated_start: '2026-04-02T10:18:00Z',
    wall_time_requested: 48,
    status: 'running',
    dependencies: []
  }
])

const alerts = ref<ClusterAlert[]>([
  {
    id: 'alert-001',
    severity: 'critical',
    message: '节点 hpc-node-007 失联超过 48 小时，可能硬件故障',
    node_id: 'hpc-node-007',
    timestamp: '2026-04-01T16:00:00Z',
    acknowledged: false
  },
  {
    id: 'alert-002',
    severity: 'warning',
    message: '节点 hpc-node-001 CPU 温度达到 72°C，接近阈值 80°C',
    node_id: 'hpc-node-001',
    timestamp: '2026-04-02T10:25:00Z',
    acknowledged: false
  },
  {
    id: 'alert-003',
    severity: 'warning',
    message: '节点 hpc-node-005 维护中，预计 2026-04-03 恢复',
    node_id: 'hpc-node-005',
    timestamp: '2026-04-01T22:00:00Z',
    acknowledged: true
  },
  {
    id: 'alert-004',
    severity: 'info',
    message: '集群整体 CPU 利用率达到 78%，建议提交低优先级任务时错峰',
    node_id: '',
    timestamp: '2026-04-02T09:00:00Z',
    acknowledged: false
  }
])

const metrics = ref<ClusterMetrics>({
  cpu_utilization: 78,
  memory_utilization: 62,
  gpu_utilization: 55,
  disk_io_read_mbps: 1200,
  disk_io_write_mbps: 450,
  network_in_mbps: 3200,
  network_out_mbps: 2800,
  jobs_completed_today: 23,
  jobs_failed_today: 2,
  avg_queue_wait_minutes: 12
})

// ============ 计算属性 ============

const metricsStat = computed(() => ({
  onlineNodes: nodes.value.filter(n => n.status === 'online' || n.status === 'busy' || n.status === 'idle').length,
  totalCores: nodes.value.reduce((sum, n) => sum + n.cpu_cores, 0),
  queueLength: queueJobs.value.filter(j => j.status === 'queued').length,
  completedToday: metrics.value.jobs_completed_today,
  failedToday: metrics.value.jobs_failed_today,
  avgWait: metrics.value.avg_queue_wait_minutes
}))

// ============ 方法 ============

function nodeStatusClass(status: ClusterNode['status']): string {
  const map: Record<string, string> = {
    online: 'bg-[var(--accent-green)]/20 text-[var(--accent-green)]',
    offline: 'bg-[var(--accent-red)]/20 text-[var(--accent-red)]',
    busy: 'bg-[var(--primary)]/20 text-[var(--primary)]',
    idle: 'bg-gray-500/20 text-gray-400',
    maintenance: 'bg-[var(--accent-yellow)]/20 text-[var(--accent-yellow)]'
  }
  return map[status] || ''
}

function nodeStatusLabel(status: ClusterNode['status']): string {
  const map: Record<string, string> = {
    online: '在线',
    offline: '离线',
    busy: '忙碌',
    idle: '空闲',
    maintenance: '维护中'
  }
  return map[status] || status
}

function usageColorClass(usage: number): string {
  if (usage >= 90) return 'bg-[var(--accent-red)]'
  if (usage >= 70) return 'bg-[var(--accent-yellow)]'
  if (usage >= 40) return 'bg-[var(--primary)]'
  return 'bg-[var(--accent-green)]'
}

function queueStatusClass(status: QueueJob['status']): string {
  const map: Record<string, string> = {
    queued: 'bg-[var(--accent-yellow)]/20 text-[var(--accent-yellow)]',
    running: 'bg-[var(--primary)]/20 text-[var(--primary)]',
    completed: 'bg-[var(--accent-green)]/20 text-[var(--accent-green)]',
    failed: 'bg-[var(--accent-red)]/20 text-[var(--accent-red)]',
    cancelled: 'bg-gray-500/20 text-gray-400'
  }
  return map[status] || ''
}

function queueStatusLabel(status: QueueJob['status']): string {
  const map: Record<string, string> = {
    queued: '排队中',
    running: '运行中',
    completed: '已完成',
    failed: '失败',
    cancelled: '已取消'
  }
  return map[status] || status
}

function alertSeverityBorder(severity: ClusterAlert['severity']): string {
  const map: Record<string, string> = {
    critical: 'border-[var(--accent-red)]/40 bg-[var(--accent-red)]/5',
    warning: 'border-[var(--accent-yellow)]/40 bg-[var(--accent-yellow)]/5',
    info: 'border-blue-400/40 bg-blue-400/5'
  }
  return map[severity] || ''
}

function alertSeverityIcon(severity: ClusterAlert['severity']): string {
  const map: Record<string, string> = {
    critical: 'text-[var(--accent-red)]',
    warning: 'text-[var(--accent-yellow)]',
    info: 'text-blue-400'
  }
  return map[severity] || ''
}

function formatTime(iso: string): string {
  if (!iso) return '--'
  const d = new Date(iso)
  return d.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

function selectNode(node: ClusterNode) {
  selectedNode.value = node
}

function ackAlert(alertId: string) {
  const alert = alerts.value.find(a => a.id === alertId)
  if (alert) {
    alert.acknowledged = true
  }
}

function handleDrain(nodeId: string) {
  const node = nodes.value.find(n => n.id === nodeId)
  if (node) {
    node.status = 'maintenance'
  }
  selectedNode.value = null
}

function handleRestart(nodeId: string) {
  const node = nodes.value.find(n => n.id === nodeId)
  if (node) {
    node.status = 'maintenance'
    node.cpu_usage = 0
    node.gpu_usage = 0
    node.memory_used_gb = 0
    node.jobs_running = []
    node.temperature = 28
    node.power_consumption_w = 45
  }
  selectedNode.value = null
}

function refreshCluster() {
  nodes.value.forEach(node => {
    if (node.status === 'busy') {
      node.cpu_usage = Math.min(100, Math.max(50, node.cpu_usage + Math.floor(Math.random() * 10 - 5)))
      node.gpu_usage = Math.min(100, Math.max(30, node.gpu_usage + Math.floor(Math.random() * 8 - 4)))
      node.memory_used_gb = Math.min(node.memory_total_gb, Math.max(50, node.memory_used_gb + Math.floor(Math.random() * 10 - 5)))
      node.temperature = Math.min(85, Math.max(55, node.temperature + Math.floor(Math.random() * 6 - 3)))
    }
  })
  metrics.value.cpu_utilization = Math.min(100, Math.max(40, metrics.value.cpu_utilization + Math.floor(Math.random() * 10 - 5)))
  metrics.value.memory_utilization = Math.min(100, Math.max(30, metrics.value.memory_utilization + Math.floor(Math.random() * 8 - 4)))
  metrics.value.gpu_utilization = Math.min(100, Math.max(20, metrics.value.gpu_utilization + Math.floor(Math.random() * 8 - 4)))
  drawChart()
}

// ============ 资源趋势图 ============

function drawChart() {
  const canvas = chartCanvas.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const rect = canvas.getBoundingClientRect()
  const dpr = window.devicePixelRatio || 1
  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr
  ctx.scale(dpr, dpr)

  const w = rect.width
  const h = rect.height
  const padding = { top: 10, right: 10, bottom: 20, left: 35 }
  const chartW = w - padding.left - padding.right
  const chartH = h - padding.top - padding.bottom

  ctx.clearRect(0, 0, w, h)

  const series = [
    { label: 'CPU', value: metrics.value.cpu_utilization, color: '#3b82f6' },
    { label: '内存', value: metrics.value.memory_utilization, color: '#60a5fa' },
    { label: 'GPU', value: metrics.value.gpu_utilization, color: '#a78bfa' },
    { label: '网络', value: Math.min(100, (metrics.value.network_in_mbps + metrics.value.network_out_mbps) / 100), color: '#34d399' }
  ]

  // 生成模拟历史数据 (20 个点)
  const points = 20
  const histories: number[][] = series.map(s => {
    const data: number[] = []
    let v = s.value
    for (let i = 0; i < points; i++) {
      v = Math.max(5, Math.min(100, v + (Math.random() - 0.5) * 12))
      data.push(v)
    }
    data[data.length - 1] = s.value
    return data
  })

  // 网格线
  ctx.strokeStyle = 'rgba(255,255,255,0.06)'
  ctx.lineWidth = 0.5
  for (let i = 0; i <= 4; i++) {
    const y = padding.top + (chartH / 4) * i
    ctx.beginPath()
    ctx.moveTo(padding.left, y)
    ctx.lineTo(w - padding.right, y)
    ctx.stroke()
  }

  // Y 轴标签
  ctx.fillStyle = 'rgba(255,255,255,0.3)'
  ctx.font = '9px sans-serif'
  ctx.textAlign = 'right'
  for (let i = 0; i <= 4; i++) {
    const y = padding.top + (chartH / 4) * i
    ctx.fillText(String(100 - i * 25) + '%', padding.left - 5, y + 3)
  }

  // 绘制曲线
  series.forEach((s, si) => {
    const data = histories[si]
    ctx.strokeStyle = s.color
    ctx.lineWidth = 1.5
    ctx.beginPath()
    data.forEach((v, i) => {
      const x = padding.left + (chartW / (points - 1)) * i
      const y = padding.top + chartH - (v / 100) * chartH
      if (i === 0) ctx.moveTo(x, y)
      else ctx.lineTo(x, y)
    })
    ctx.stroke()
  })

  // 图例
  ctx.font = '9px sans-serif'
  ctx.textAlign = 'left'
  series.forEach((s, i) => {
    const lx = padding.left + i * 70
    const ly = h - 5
    ctx.fillStyle = s.color
    ctx.fillRect(lx, ly - 6, 8, 8)
    ctx.fillStyle = 'rgba(255,255,255,0.5)'
    ctx.fillText(`${s.label} ${s.value}%`, lx + 12, ly + 1)
  })
}

onMounted(() => {
  nextTick(() => {
    drawChart()
  })
})

watch(selectedNode, () => {
  // 节点选择不影响图表
})
</script>
