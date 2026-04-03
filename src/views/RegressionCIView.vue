<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">多尺度回归测试 CI</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.8-006 | 自动化回归测试管线，基准对比，CI/CD 集成</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="refreshHistory">刷新历史</button>
        <button class="btn btn-primary text-xs" @click="scrollToResults">查看结果</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Benchmark Selection -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            基准选择
          </h4>
          <div class="space-y-1.5 max-h-40 overflow-y-auto">
            <label
              v-for="bm in benchmarkList"
              :key="bm.id"
              class="flex items-center gap-2 px-2 py-1.5 rounded cursor-pointer text-xs transition"
              :style="selectedBenchmarks.includes(bm.id)
                ? 'background: var(--bg-elevated); color: var(--text-primary)'
                : 'color: var(--text-secondary)'"
            >
              <input
                type="checkbox"
                :value="bm.id"
                v-model="selectedBenchmarks"
                class="rounded"
              />
              <span class="flex-1">{{ bm.name }}</span>
              <span class="text-[10px]" style="color: var(--text-muted)">{{ bm.scale }}</span>
            </label>
          </div>
          <p class="text-[10px] mt-1" style="color: var(--text-muted)">已选 {{ selectedBenchmarks.length }} / {{ benchmarkList.length }}</p>
        </div>

        <!-- Step 2: CI Settings -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            CI 设置
          </h4>
          <div class="space-y-3">
            <div>
              <label class="label">容差覆盖 (%)</label>
              <input class="input w-full text-xs" type="number" v-model.number="ciSettings.toleranceOverride" min="0.01" step="0.1" />
            </div>
            <div>
              <label class="label">最大并行任务数</label>
              <input class="input w-full text-xs" type="number" v-model.number="ciSettings.maxParallel" min="1" max="16" />
            </div>
            <div>
              <label class="label">单测试超时 (秒)</label>
              <input class="input w-full text-xs" type="number" v-model.number="ciSettings.timeout" min="10" step="10" />
            </div>
          </div>
        </div>

        <!-- Step 3: Notification Config -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            通知配置
          </h4>
          <div class="space-y-3">
            <div>
              <label class="label">通知类型</label>
              <select class="input w-full text-xs" v-model="notification.type">
                <option value="slack">Slack</option>
                <option value="email">Email</option>
                <option value="webhook">Webhook</option>
              </select>
            </div>
            <div>
              <label class="label">通知 URL</label>
              <input class="input w-full text-xs" type="text" v-model="notification.url" placeholder="https://hooks.slack.com/..." />
            </div>
            <label class="flex items-center gap-2 text-xs cursor-pointer" style="color: var(--text-secondary)">
              <input type="checkbox" v-model="notification.on_failure_only" class="rounded" />
              仅失败时通知
            </label>
          </div>
        </div>

        <!-- Step 4: Trigger Pipeline -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            触发管线
          </h4>
          <button
            class="btn btn-primary text-xs w-full"
            :disabled="selectedBenchmarks.length === 0 || pipelineStatus === 'running'"
            @click="triggerPipeline"
          >
            {{ pipelineStatus === 'running' ? '管线运行中...' : '触发 CI 管线' }}
          </button>
          <!-- Pipeline Status -->
          <div v-if="pipelineStatus" class="mt-3 p-3 rounded" style="background: var(--bg-elevated)">
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs font-medium" style="color: var(--text-primary)">管线状态</span>
              <span
                class="text-[10px] px-2 py-0.5 rounded-full font-medium"
                :style="pipelineStatusStyle"
              >{{ pipelineStatusLabel }}</span>
            </div>
            <div class="w-full h-2 rounded-full overflow-hidden" style="background: var(--border-subtle)">
              <div
                class="h-full rounded-full transition-all duration-500"
                :style="{ width: pipelineProgress + '%', background: pipelineProgressColor }"
              />
            </div>
            <p class="text-[10px] mt-1" style="color: var(--text-muted)">{{ pipelineProgress }}% 完成</p>
          </div>
        </div>

        <!-- Step 5: Pipeline Results Summary -->
        <div v-if="currentPipeline">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            管线结果摘要
          </h4>
          <div class="grid grid-cols-2 gap-2">
            <div class="p-2 rounded text-center" style="background: var(--bg-elevated)">
              <div class="text-lg font-bold" style="color: var(--text-primary)">{{ currentPipeline.summary.total }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">总计</div>
            </div>
            <div class="p-2 rounded text-center" style="background: var(--bg-elevated)">
              <div class="text-lg font-bold" style="color: var(--accent-green)">{{ currentPipeline.summary.passed }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">通过</div>
            </div>
            <div class="p-2 rounded text-center" style="background: var(--bg-elevated)">
              <div class="text-lg font-bold" style="color: var(--accent-red)">{{ currentPipeline.summary.failed }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">失败</div>
            </div>
            <div class="p-2 rounded text-center" style="background: var(--bg-elevated)">
              <div class="text-lg font-bold" style="color: var(--text-muted)">{{ currentPipeline.summary.skipped }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">跳过</div>
            </div>
          </div>
          <div class="mt-2 p-2 rounded text-center" style="background: var(--bg-elevated)">
            <div class="text-sm font-medium" style="color: var(--text-primary)">{{ currentPipeline.summary.total_time_s.toFixed(1) }}s</div>
            <div class="text-[10px]" style="color: var(--text-muted)">总耗时</div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="resultsRef">

        <!-- Pipeline History Timeline -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">管线历史时间线</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg :width="timelineSvgWidth" :height="pipelineHistory.length * 60 + 20" viewBox="0 0 280 220">
              <!-- Vertical line -->
              <line x1="30" y1="20" :x2="30" :y2="pipelineHistory.length * 60" stroke="var(--border-default)" stroke-width="2" stroke-dasharray="4,4" />
              <!-- Pipeline nodes -->
              <g v-for="(pl, idx) in pipelineHistory" :key="pl.pipeline_id">
                <circle :cx="30" :cy="20 + idx * 60" r="8" :fill="statusColor(pl.status)" />
                <text x="48" :y="16 + idx * 60" class="text-[10px]" fill="var(--text-primary)" font-size="11" font-weight="600">{{ pl.pipeline_id }}</text>
                <text x="48" :y="30 + idx * 60" fill="var(--text-muted)" font-size="10">
                  {{ pl.triggered_by }} | {{ formatDate(pl.started_at) }}
                </text>
                <text x="48" :y="44 + idx * 60" fill="var(--text-secondary)" font-size="10">
                  {{ pl.summary.passed }}/{{ pl.summary.total }} 通过 | {{ pl.summary.total_time_s.toFixed(1) }}s
                </text>
                <!-- Test result dots -->
                <g :transform="`translate(200, ${12 + idx * 60})`">
                  <circle
                    v-for="(r, ri) in pl.results.slice(0, 12)"
                    :key="ri"
                    :cx="ri * 8"
                    :cy="4"
                    r="3"
                    :fill="testStatusColor(r.status)"
                  />
                </g>
              </g>
            </svg>
          </div>
        </div>

        <!-- Test Results Grid -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">测试结果详情</h4>
          <div class="grid grid-cols-2 gap-2">
            <div
              v-for="result in currentResults"
              :key="result.test_id"
              class="p-3 rounded border"
              style="background: var(--bg-surface); border-color: var(--border-subtle)"
            >
              <div class="flex items-center justify-between mb-1">
                <span class="text-xs font-medium truncate" style="color: var(--text-primary)">{{ result.benchmark_id }}</span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded-full font-medium"
                  :style="testResultBadgeStyle(result.status)"
                >{{ statusLabel(result.status) }}</span>
              </div>
              <div class="space-y-0.5">
                <div class="flex justify-between text-[10px]">
                  <span style="color: var(--text-muted)">误差</span>
                  <span :style="{ color: result.error_percent > result.tolerance_percent ? 'var(--accent-red)' : 'var(--accent-green)' }">
                    {{ result.error_percent.toFixed(3) }}%
                  </span>
                </div>
                <div class="flex justify-between text-[10px]">
                  <span style="color: var(--text-muted)">容差</span>
                  <span style="color: var(--text-secondary)">{{ result.tolerance_percent.toFixed(2) }}%</span>
                </div>
                <div class="flex justify-between text-[10px]">
                  <span style="color: var(--text-muted)">耗时</span>
                  <span style="color: var(--text-secondary)">{{ result.execution_time_s.toFixed(2) }}s</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- CI Statistics Cards -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">CI 统计</h4>
          <div class="grid grid-cols-4 gap-3">
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--primary)">{{ ciStats.totalPipelines }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">总管线数</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" :style="{ color: ciStats.passRate >= 90 ? 'var(--accent-green)' : 'var(--accent-yellow)' }">
                {{ ciStats.passRate.toFixed(1) }}%
              </div>
              <div class="text-[10px]" style="color: var(--text-muted)">通过率</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--text-primary)">{{ ciStats.avgTime.toFixed(1) }}s</div>
              <div class="text-[10px]" style="color: var(--text-muted)">平均耗时</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xs font-medium" style="color: var(--text-secondary)">{{ ciStats.lastRun }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">最近运行</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { CITestStatus, CITestResult, CIPipeline, CINotificationConfig, CITriggerConfig } from '../api/regressionCI'

// ============ Refs ============
const resultsRef = ref<HTMLElement | null>(null)
const pipelineStatus = ref<'queued' | 'running' | 'completed' | 'failed' | null>(null)
const pipelineProgress = ref(0)
const selectedBenchmarks = ref<string[]>([])
const currentPipeline = ref<CIPipeline | null>(null)

// ============ Reactive Config ============
const ciSettings = reactive({
  toleranceOverride: 0.5,
  maxParallel: 4,
  timeout: 120
})

const notification = reactive<CINotificationConfig>({
  type: 'slack',
  enabled: true,
  url: 'https://hooks.slack.com/services/T00/B00/xxx',
  on_failure_only: true
})

// ============ Benchmark List ============
const benchmarkList = [
  { id: 'bm-md-lammps-001', name: 'LAMMPS EAM Cu', scale: '原子' },
  { id: 'bm-pf-allen-cahn', name: 'Allen-Cahn 相场', scale: '介观' },
  { id: 'bm-fe-tensile-2d', name: '2D 拉伸 FE', scale: '宏观' },
  { id: 'bm-md-fe-bridge', name: 'MD-FE 桥接', scale: '跨尺度' },
  { id: 'bm-pf-crack-prop', name: '裂纹扩展相场', scale: '介观' },
  { id: 'bm-cg-mapping', name: '粗粒化映射', scale: '跨尺度' },
  { id: 'bm-homogenization', name: '均匀化 RVE', scale: '跨尺度' },
  { id: 'bm-thermal-coupling', name: '热力耦合', scale: '多物理' },
  { id: 'bm-creep-norton', name: 'Norton 蠕变', scale: '宏观' },
  { id: 'bm-fcc-elastic', name: 'FCC 弹性常数', scale: '原子' },
  { id: 'bm-dft-eos', name: 'DFT 状态方程', scale: '量子' },
  { id: 'bm-topology-opt', name: '拓扑优化', scale: '宏观' }
]

// ============ Mock Pipeline History ============
const pipelineHistory = ref<CIPipeline[]>([])

function generateMockPipelines(): CIPipeline[] {
  const statuses: Array<'queued' | 'running' | 'completed' | 'failed'> = ['completed', 'completed', 'failed']
  const testStatuses: CITestStatus[] = ['passed', 'passed', 'passed', 'passed', 'failed', 'passed', 'skipped', 'passed', 'passed', 'passed', 'timeout', 'passed']
  const pipelines: CIPipeline[] = []

  for (let p = 0; p < 3; p++) {
    const results: CITestResult[] = []
    const count = 10 + Math.floor(Math.random() * 4)
    let passed = 0
    let failed = 0
    let skipped = 0
    let totalTime = 0

    for (let i = 0; i < count; i++) {
      const status = testStatuses[i % testStatuses.length]
      const errorPercent = status === 'passed' ? Math.random() * 0.3 : status === 'failed' ? 0.5 + Math.random() * 2.0 : 0
      const execTime = 0.5 + Math.random() * 8
      totalTime += execTime

      if (status === 'passed') passed++
      else if (status === 'failed' || status === 'timeout') failed++
      else skipped++

      results.push({
        test_id: `test-${p + 1}-${String(i + 1).padStart(3, '0')}`,
        benchmark_id: benchmarkList[i % benchmarkList.length].id,
        status,
        computed: 100 + Math.random() * 50,
        reference: 100 + Math.random() * 50,
        error_percent: errorPercent,
        tolerance_percent: 0.5,
        execution_time_s: execTime,
        timestamp: new Date(Date.now() - (3 - p) * 3600000 - i * 60000).toISOString()
      })
    }

    pipelines.push({
      pipeline_id: `pipeline-${String(p + 1).padStart(4, '0')}`,
      status: statuses[p],
      triggered_by: 'user:admin',
      started_at: new Date(Date.now() - (3 - p) * 3600000).toISOString(),
      completed_at: new Date(Date.now() - (3 - p) * 3600000 + 300000).toISOString(),
      results,
      summary: { total: count, passed, failed, skipped, total_time_s: totalTime }
    })
  }

  return pipelines
}

// ============ CI Statistics ============
const ciStats = reactive({
  totalPipelines: 0,
  passRate: 0,
  avgTime: 0,
  lastRun: '--'
})

function updateCIStats() {
  const history = pipelineHistory.value
  ciStats.totalPipelines = history.length + 42
  const completedPipelines = history.filter(p => p.status === 'completed')
  const allPassed = completedPipelines.filter(p => p.summary.failed === 0)
  ciStats.passRate = completedPipelines.length > 0 ? (allPassed.length / completedPipelines.length) * 100 : 0
  ciStats.avgTime = history.length > 0 ? history.reduce((s, p) => s + p.summary.total_time_s, 0) / history.length : 0
  ciStats.lastRun = history.length > 0 ? formatDate(history[0].started_at) : '--'
}

// ============ Computed ============
const currentResults = computed(() => {
  if (currentPipeline.value) return currentPipeline.value.results
  if (pipelineHistory.value.length > 0) return pipelineHistory.value[0].results
  return []
})

const pipelineStatusLabel = computed(() => {
  const map: Record<string, string> = { queued: '排队中', running: '运行中', completed: '已完成', failed: '失败' }
  return pipelineStatus.value ? map[pipelineStatus.value] : ''
})

const pipelineStatusStyle = computed(() => {
  if (!pipelineStatus.value) return {}
  const colors: Record<string, string> = {
    queued: 'background: var(--accent-yellow); color: #000',
    running: 'background: var(--primary); color: #fff',
    completed: 'background: var(--accent-green); color: #fff',
    failed: 'background: var(--accent-red); color: #fff'
  }
  return colors[pipelineStatus.value] || ''
})

const pipelineProgressColor = computed(() => {
  if (pipelineStatus.value === 'failed') return 'var(--accent-red)'
  if (pipelineProgress.value >= 100) return 'var(--accent-green)'
  return 'var(--primary)'
})

const timelineSvgWidth = computed(() => 280)

// ============ Methods ============
function statusColor(status: string): string {
  const map: Record<string, string> = { completed: 'var(--accent-green)', failed: 'var(--accent-red)', running: 'var(--primary)', queued: 'var(--accent-yellow)' }
  return map[status] || 'var(--text-muted)'
}

function testStatusColor(status: CITestStatus): string {
  const map: Record<CITestStatus, string> = { passed: 'var(--accent-green)', failed: 'var(--accent-red)', skipped: 'var(--text-muted)', running: 'var(--primary)', timeout: 'var(--accent-yellow)' }
  return map[status]
}

function statusLabel(status: CITestStatus): string {
  const map: Record<CITestStatus, string> = { passed: '通过', failed: '失败', skipped: '跳过', running: '运行中', timeout: '超时' }
  return map[status]
}

function testResultBadgeStyle(status: CITestStatus) {
  const map: Record<CITestStatus, string> = {
    passed: 'background: rgba(34,197,94,0.15); color: var(--accent-green)',
    failed: 'background: rgba(239,68,68,0.15); color: var(--accent-red)',
    skipped: 'background: rgba(156,163,175,0.15); color: var(--text-muted)',
    running: 'background: rgba(59,130,246,0.15); color: var(--primary)',
    timeout: 'background: rgba(234,179,8,0.15); color: var(--accent-yellow)'
  }
  return map[status]
}

function formatDate(iso: string): string {
  const d = new Date(iso)
  return `${d.getMonth() + 1}/${d.getDate()} ${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

async function triggerPipeline() {
  if (selectedBenchmarks.value.length === 0) return

  const config: CITriggerConfig = {
    benchmark_ids: [...selectedBenchmarks.value],
    tolerance_override: ciSettings.toleranceOverride,
    max_parallel: ciSettings.maxParallel,
    notification: { ...notification }
  }

  pipelineStatus.value = 'queued'
  pipelineProgress.value = 0

  // Simulate pipeline execution
  await nextTick()
  pipelineStatus.value = 'running'

  const totalSteps = 20
  for (let i = 1; i <= totalSteps; i++) {
    await new Promise(resolve => setTimeout(resolve, 150))
    pipelineProgress.value = Math.round((i / totalSteps) * 100)
  }

  // Generate result
  const mockPipeline = generateMockPipelines()[0]
  mockPipeline.pipeline_id = `pipeline-${Date.now().toString().slice(-6)}`
  mockPipeline.status = 'completed'
  mockPipeline.triggered_by = 'user:admin'
  mockPipeline.started_at = new Date().toISOString()
  mockPipeline.completed_at = new Date(Date.now() + 300000).toISOString()

  pipelineHistory.value.unshift(mockPipeline)
  currentPipeline.value = mockPipeline
  pipelineStatus.value = 'completed'
  pipelineProgress.value = 100

  updateCIStats()
}

function refreshHistory() {
  pipelineHistory.value = generateMockPipelines()
  currentPipeline.value = pipelineHistory.value[0] || null
  updateCIStats()
}

function scrollToResults() {
  const el = resultsRef.value
  if (el) {
    el.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

// ============ Lifecycle ============
onMounted(() => {
  pipelineHistory.value = generateMockPipelines()
  currentPipeline.value = pipelineHistory.value[0] || null
  selectedBenchmarks.value = benchmarkList.slice(0, 6).map(b => b.id)
  updateCIStats()
})
</script>
