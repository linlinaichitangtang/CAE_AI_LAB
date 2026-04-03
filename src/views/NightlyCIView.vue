<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">四尺度集成 CI / Nightly Build</h2>
        <p class="text-sm" style="color: var(--text-muted)">V1.9-008 | 每日自动构建、多场景回归、失败告警与发布门禁</p>
      </div>
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost text-xs" @click="refreshBuilds">刷新构建</button>
        <button class="btn btn-primary text-xs" @click="scrollToTimeline">查看时间线</button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Nightly Config -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            Nightly 配置
          </h4>
          <div class="space-y-3">
            <div>
              <label class="label">触发时间 (Cron)</label>
              <input class="input w-full text-xs" type="text" v-model="nightlyConfig.trigger_time" placeholder="0 2 * * *" />
            </div>
            <div>
              <label class="label">最大持续时间 (秒)</label>
              <input class="input w-full text-xs" type="number" v-model.number="nightlyConfig.max_duration_s" min="600" step="300" />
            </div>
          </div>
        </div>

        <!-- Step 2: Scenario Selection -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            场景选择
          </h4>
          <div class="space-y-1.5">
            <label
              v-for="sc in scenarioOptions"
              :key="sc.id"
              class="flex items-center gap-2 px-2 py-1.5 rounded cursor-pointer text-xs transition"
              :style="nightlyConfig.scenarios.includes(sc.id)
                ? 'background: var(--bg-elevated); color: var(--text-primary)'
                : 'color: var(--text-secondary)'"
            >
              <input type="checkbox" :value="sc.id" v-model="nightlyConfig.scenarios" class="rounded" />
              <span class="flex-1">{{ sc.name }}</span>
              <span class="text-[10px]" style="color: var(--text-muted)">{{ sc.scale }}</span>
            </label>
          </div>
          <p class="text-[10px] mt-1" style="color: var(--text-muted)">已选 {{ nightlyConfig.scenarios.length }} / {{ scenarioOptions.length }}</p>
        </div>

        <!-- Step 3: Failure Handling -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            失败处理策略
          </h4>
          <div class="space-y-2">
            <label
              v-for="opt in failureOptions"
              :key="opt.value"
              class="flex items-center gap-2 px-2 py-1.5 rounded cursor-pointer text-xs transition"
              :style="nightlyConfig.on_failure === opt.value
                ? 'background: var(--bg-elevated); color: var(--text-primary)'
                : 'color: var(--text-secondary)'"
            >
              <input type="radio" :value="opt.value" v-model="nightlyConfig.on_failure" />
              <span>{{ opt.label }}</span>
            </label>
          </div>
        </div>

        <!-- Step 4: Notification Channels -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            通知渠道
          </h4>
          <div class="space-y-2">
            <div v-for="(ch, idx) in nightlyConfig.notification_channels" :key="idx" class="p-2 rounded space-y-1.5" style="background: var(--bg-elevated)">
              <div class="flex items-center gap-2">
                <select class="input text-xs flex-1" v-model="ch.type">
                  <option value="slack">Slack</option>
                  <option value="email">Email</option>
                  <option value="webhook">Webhook</option>
                </select>
                <button class="btn btn-ghost text-xs" style="padding: 2px 6px" @click="removeChannel(idx)">x</button>
              </div>
              <input class="input w-full text-xs" type="text" v-model="ch.url" placeholder="https://..." />
            </div>
            <button class="btn btn-ghost text-xs w-full" @click="addChannel">+ 添加渠道</button>
          </div>
        </div>

        <!-- Step 5: Actions -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            操作
          </h4>
          <div class="space-y-2">
            <button class="btn btn-primary text-xs w-full" :disabled="isTriggering" @click="triggerBuild">
              {{ isTriggering ? '构建中...' : '手动触发' }}
            </button>
            <button class="btn btn-ghost text-xs w-full" @click="saveConfig">保存配置</button>
          </div>
        </div>

        <!-- Step 6: Latest Build Status -->
        <div v-if="latestBuild">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">6</span>
            最近构建状态
          </h4>
          <div class="p-3 rounded space-y-2" style="background: var(--bg-elevated)">
            <div class="flex items-center justify-between">
              <span class="text-xs font-medium" style="color: var(--text-primary)">状态</span>
              <span class="text-[10px] px-2 py-0.5 rounded-full font-medium" :style="buildStatusBadgeStyle(latestBuild.status)">
                {{ buildStatusLabel(latestBuild.status) }}
              </span>
            </div>
            <div class="flex items-center justify-between text-xs">
              <span style="color: var(--text-muted)">耗时</span>
              <span style="color: var(--text-secondary)">{{ latestBuild.duration_s.toFixed(0) }}s</span>
            </div>
            <div class="grid grid-cols-3 gap-1 text-center">
              <div class="p-1 rounded" style="background: var(--bg-surface)">
                <div class="text-sm font-bold" style="color: var(--accent-green)">{{ latestBuild.summary.passed }}</div>
                <div class="text-[9px]" style="color: var(--text-muted)">通过</div>
              </div>
              <div class="p-1 rounded" style="background: var(--bg-surface)">
                <div class="text-sm font-bold" style="color: var(--accent-red)">{{ latestBuild.summary.failed }}</div>
                <div class="text-[9px]" style="color: var(--text-muted)">失败</div>
              </div>
              <div class="p-1 rounded" style="background: var(--bg-surface)">
                <div class="text-sm font-bold" style="color: var(--text-muted)">{{ latestBuild.summary.skipped }}</div>
                <div class="text-[9px]" style="color: var(--text-muted)">跳过</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" ref="timelineRef">

        <!-- Build History Timeline -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">构建历史时间线</h4>
          <div class="p-4 rounded" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
            <svg :width="timelineSvgWidth" :height="buildHistory.length * 56 + 20" viewBox="0 0 320 500">
              <!-- Vertical line -->
              <line x1="30" y1="20" :x2="30" :y2="buildHistory.length * 56" stroke="var(--border-default)" stroke-width="2" stroke-dasharray="4,4" />
              <!-- Build nodes -->
              <g v-for="(build, idx) in buildHistory" :key="build.build_id">
                <circle :cx="30" :cy="20 + idx * 56" r="8" :fill="buildDotColor(build.status)" />
                <text x="48" :y="16 + idx * 56" fill="var(--text-primary)" font-size="11" font-weight="600">{{ build.build_id }}</text>
                <text x="48" :y="30 + idx * 56" fill="var(--text-muted)" font-size="10">
                  {{ formatDate(build.triggered_at) }} | {{ build.duration_s.toFixed(0) }}s
                </text>
                <text x="48" :y="44 + idx * 56" fill="var(--text-secondary)" font-size="10">
                  {{ build.summary.passed }}/{{ build.summary.total }} 通过
                  <tspan v-if="build.issues_created > 0" :fill="'var(--accent-red)'"> | {{ build.issues_created }} issues</tspan>
                </text>
                <!-- Scenario result dots -->
                <g :transform="`translate(220, ${12 + idx * 56})`">
                  <circle
                    v-for="(sc, si) in build.scenarios"
                    :key="si"
                    :cx="si * 16"
                    :cy="4"
                    r="5"
                    :fill="scenarioDotColor(sc.status)"
                  />
                  <text x="-8" y="20" fill="var(--text-muted)" font-size="8">高强钢</text>
                  <text x="8" y="20" fill="var(--text-muted)" font-size="8">镁合金</text>
                  <text x="24" y="20" fill="var(--text-muted)" font-size="8">陶瓷</text>
                </g>
              </g>
            </svg>
          </div>
        </div>

        <!-- Scenario Results Grid -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">场景结果详情</h4>
          <div class="grid grid-cols-3 gap-3">
            <div
              v-for="scenario in latestScenarios"
              :key="scenario.scenario"
              class="p-3 rounded border"
              style="background: var(--bg-surface); border-color: var(--border-subtle)"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="text-xs font-medium truncate" style="color: var(--text-primary)">{{ scenario.scenario }}</span>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded-full font-medium"
                  :style="scenarioBadgeStyle(scenario.status)"
                >{{ scenarioStatusLabel(scenario.status) }}</span>
              </div>
              <div class="space-y-1">
                <div
                  v-for="(step, si) in scenario.steps"
                  :key="si"
                  class="flex items-center gap-1.5 text-[10px]"
                >
                  <span
                    class="inline-block w-2 h-2 rounded-full"
                    :style="{ background: stepStatusColor(step.status) }"
                  />
                  <span style="color: var(--text-secondary)">{{ step.scale }}</span>
                  <span class="flex-1" style="color: var(--text-muted)">{{ step.status }}</span>
                </div>
              </div>
              <div class="mt-2 pt-2 border-t text-[10px]" style="border-color: var(--border-subtle); color: var(--text-muted)">
                耗时 {{ scenario.duration_s.toFixed(1) }}s
              </div>
            </div>
          </div>
        </div>

        <!-- Statistics Cards -->
        <div>
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">统计概览</h4>
          <div class="grid grid-cols-5 gap-3">
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--primary)">{{ stats.total_builds }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">总构建数</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" :style="{ color: stats.pass_rate >= 80 ? 'var(--accent-green)' : 'var(--accent-yellow)' }">
                {{ stats.pass_rate.toFixed(1) }}%
              </div>
              <div class="text-[10px]" style="color: var(--text-muted)">通过率</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" style="color: var(--text-primary)">{{ stats.avg_duration_s.toFixed(0) }}s</div>
              <div class="text-[10px]" style="color: var(--text-muted)">平均耗时</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-xl font-bold" :style="{ color: stats.consecutive_failures > 0 ? 'var(--accent-red)' : 'var(--accent-green)' }">
                {{ stats.consecutive_failures }}
              </div>
              <div class="text-[10px]" style="color: var(--text-muted)">连续失败</div>
            </div>
            <div class="p-3 rounded text-center" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
              <div class="text-[10px] font-medium" style="color: var(--text-secondary)">{{ stats.last_success }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">最近成功</div>
              <div class="text-[10px] font-medium mt-1" style="color: var(--text-secondary)">{{ stats.last_failure }}</div>
              <div class="text-[10px]" style="color: var(--text-muted)">最近失败</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import type { NightlyStatus, NightlyConfig, NightlyBuild, NightlyScenarioResult } from '../api/nightlyCI'

// ============ Refs ============
const timelineRef = ref<HTMLElement | null>(null)
const isTriggering = ref(false)
const buildHistory = ref<NightlyBuild[]>([])

// ============ Scenario Options ============
const scenarioOptions = [
  { id: 'high_strength_steel_precipitate', name: '高强钢析出相', scale: '多尺度' },
  { id: 'mg_alloy_creep', name: '镁合金蠕变', scale: '宏观' },
  { id: 'ceramic_am', name: '陶瓷增材制造', scale: '介观' }
]

// ============ Failure Options ============
const failureOptions = [
  { value: 'notify_only' as const, label: '仅通知' },
  { value: 'create_issue' as const, label: '创建 Issue' },
  { value: 'block_release' as const, label: '阻止发布' }
]

// ============ Nightly Config ============
const nightlyConfig = reactive<NightlyConfig>({
  trigger_time: '0 2 * * *',
  max_duration_s: 3600,
  scenarios: ['high_strength_steel_precipitate', 'mg_alloy_creep', 'ceramic_am'],
  on_failure: 'create_issue' as 'create_issue',
  notification_channels: [
    { type: 'slack' as 'slack', url: 'https://hooks.slack.com/services/T00/B00/nightly' },
    { type: 'email' as 'email', url: 'cae-team@example.com' }
  ]
})

// ============ Statistics ============
const stats = reactive({
  total_builds: 0,
  pass_rate: 0,
  avg_duration_s: 0,
  consecutive_failures: 0,
  last_success: '--',
  last_failure: '--'
})

// ============ Mock Data Generation ============
function generateMockBuilds(): NightlyBuild[] {
  const builds: NightlyBuild[] = []
  const statusPool: NightlyStatus[] = ['completed', 'completed', 'completed', 'failed', 'partial']

  for (let i = 0; i < 8; i++) {
    const buildStatus = statusPool[i % statusPool.length]
    const scenarios: NightlyScenarioResult[] = []
    let totalPassed = 0
    let totalFailed = 0
    let totalSkipped = 0
    const baseTime = Date.now() - (8 - i) * 86400000

    const scenarioNames = ['高强钢析出相', '镁合金蠕变', '陶瓷增材制造']
    const scales = ['DFT', 'MD', 'PF', 'FE']

    for (let s = 0; s < 3; s++) {
      const scFailed = buildStatus === 'failed' && s === 1
      const scSkipped = buildStatus === 'partial' && s === 2
      const scStatus = scFailed ? 'failed' as const : scSkipped ? 'skipped' as const : 'passed' as const

      if (scStatus === 'passed') totalPassed++
      else if (scStatus === 'failed') totalFailed++
      else totalSkipped++

      const steps = scales.map((scale, si) => ({
        scale,
        status: scFailed && si === 2 ? 'error' : 'completed',
        error: scFailed && si === 2 ? 'Convergence failed at step 3' : null
      }))

      const metrics = [
        { quantity: 'yield_strength', value: 450 + Math.random() * 50, reference: 470, error_percent: 0 },
        { quantity: 'elongation', value: 12 + Math.random() * 3, reference: 14, error_percent: 0 }
      ]
      metrics.forEach(m => {
        m.error_percent = Math.abs((m.value - m.reference) / m.reference * 100)
      })

      scenarios.push({
        scenario: scenarioNames[s],
        status: scStatus,
        duration_s: 120 + Math.random() * 300,
        steps,
        metrics
      })
    }

    const durationS = scenarios.reduce((sum, sc) => sum + sc.duration_s, 0)

    builds.push({
      build_id: `nightly-${String(20260325 + i).padStart(8, '0')}`,
      status: buildStatus,
      triggered_at: new Date(baseTime).toISOString(),
      completed_at: new Date(baseTime + durationS * 1000).toISOString(),
      duration_s: durationS,
      scenarios,
      summary: {
        total: 3,
        passed: totalPassed,
        failed: totalFailed,
        skipped: totalSkipped
      },
      report_url: `/reports/nightly-${20260325 + i}.html`,
      issues_created: buildStatus === 'failed' ? 1 + Math.floor(Math.random() * 3) : 0
    })
  }

  return builds
}

// ============ Computed ============
const latestBuild = computed(() => buildHistory.value[0] || null)

const latestScenarios = computed<NightlyScenarioResult[]>(() => {
  if (latestBuild.value) return latestBuild.value.scenarios
  return []
})

const timelineSvgWidth = computed(() => 320)

// ============ Methods ============
function buildDotColor(status: NightlyStatus): string {
  const map: Record<NightlyStatus, string> = {
    scheduled: 'var(--accent-yellow)',
    running: 'var(--primary)',
    completed: 'var(--accent-green)',
    failed: 'var(--accent-red)',
    partial: 'var(--accent-yellow)'
  }
  return map[status]
}

function scenarioDotColor(status: string): string {
  const map: Record<string, string> = {
    passed: 'var(--accent-green)',
    failed: 'var(--accent-red)',
    skipped: 'var(--text-muted)'
  }
  return map[status] || 'var(--text-muted)'
}

function buildStatusLabel(status: NightlyStatus): string {
  const map: Record<NightlyStatus, string> = {
    scheduled: '已调度',
    running: '运行中',
    completed: '已完成',
    failed: '失败',
    partial: '部分完成'
  }
  return map[status]
}

function buildStatusBadgeStyle(status: NightlyStatus) {
  const map: Record<NightlyStatus, string> = {
    scheduled: 'background: rgba(234,179,8,0.15); color: var(--accent-yellow)',
    running: 'background: rgba(59,130,246,0.15); color: var(--primary)',
    completed: 'background: rgba(34,197,94,0.15); color: var(--accent-green)',
    failed: 'background: rgba(239,68,68,0.15); color: var(--accent-red)',
    partial: 'background: rgba(234,179,8,0.15); color: var(--accent-yellow)'
  }
  return map[status]
}

function scenarioStatusLabel(status: string): string {
  const map: Record<string, string> = {
    passed: '通过',
    failed: '失败',
    skipped: '跳过'
  }
  return map[status] || status
}

function scenarioBadgeStyle(status: string) {
  const map: Record<string, string> = {
    passed: 'background: rgba(34,197,94,0.15); color: var(--accent-green)',
    failed: 'background: rgba(239,68,68,0.15); color: var(--accent-red)',
    skipped: 'background: rgba(156,163,175,0.15); color: var(--text-muted)'
  }
  return map[status] || ''
}

function stepStatusColor(status: string): string {
  const map: Record<string, string> = {
    completed: 'var(--accent-green)',
    error: 'var(--accent-red)',
    running: 'var(--primary)',
    pending: 'var(--text-muted)'
  }
  return map[status] || 'var(--text-muted)'
}

function formatDate(iso: string): string {
  const d = new Date(iso)
  return `${d.getMonth() + 1}/${d.getDate()} ${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}

function updateStats() {
  const history = buildHistory.value
  stats.total_builds = history.length + 47
  const completed = history.filter(b => b.status === 'completed')
  stats.pass_rate = completed.length > 0 ? (completed.length / history.length) * 100 : 0
  stats.avg_duration_s = history.length > 0
    ? history.reduce((s, b) => s + b.duration_s, 0) / history.length
    : 0

  let consecutive = 0
  for (const b of history) {
    if (b.status === 'failed') consecutive++
    else break
  }
  stats.consecutive_failures = consecutive

  const successBuild = history.find(b => b.status === 'completed')
  const failBuild = history.find(b => b.status === 'failed')
  stats.last_success = successBuild ? formatDate(successBuild.triggered_at) : '--'
  stats.last_failure = failBuild ? formatDate(failBuild.triggered_at) : '--'
}

function addChannel() {
  nightlyConfig.notification_channels.push({ type: 'webhook' as 'webhook', url: '' })
}

function removeChannel(idx: number) {
  nightlyConfig.notification_channels.splice(idx, 1)
}

async function triggerBuild() {
  isTriggering.value = true
  await nextTick()

  // Simulate build delay
  await new Promise(resolve => setTimeout(resolve, 800))

  const newBuild = generateMockBuilds()[0]
  newBuild.build_id = `nightly-${Date.now().toString().slice(-8)}`
  newBuild.status = 'completed' as NightlyStatus
  newBuild.triggered_at = new Date().toISOString()
  newBuild.completed_at = new Date(Date.now() + 600000).toISOString()

  buildHistory.value.unshift(newBuild)
  isTriggering.value = false
  updateStats()
}

function saveConfig() {
  // Mock save
  const configSnapshot = { ...nightlyConfig }
  console.log('Nightly config saved:', configSnapshot)
}

function refreshBuilds() {
  buildHistory.value = generateMockBuilds()
  updateStats()
}

function scrollToTimeline() {
  const el = timelineRef.value
  if (el) {
    el.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

// ============ Lifecycle ============
onMounted(() => {
  buildHistory.value = generateMockBuilds()
  updateStats()
})
</script>
