<template>
  <div class="h-full overflow-y-auto p-6">
    <!-- ============================================ -->
    <!-- Step 1: 欢迎区域 -->
    <!-- ============================================ -->
    <div class="mb-8">
      <div class="flex items-center gap-3 mb-2">
        <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center">
          <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
          </svg>
        </div>
        <div>
          <h1 class="text-xl font-bold">{{ t('installer.title') }}</h1>
          <p class="text-sm text-[var(--text-secondary)]">{{ t('installer.subtitle') }}</p>
        </div>
      </div>

      <!-- 四个尺度预览卡片 -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-3 mt-4">
        <div v-for="scale in scaleCards" :key="scale.key"
          class="rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] p-3 flex items-center gap-2">
          <span class="text-xl">{{ scale.icon }}</span>
          <div>
            <div class="text-xs font-semibold">{{ scale.name }}</div>
            <div class="text-xs text-[var(--text-secondary)]">{{ scale.label }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- ============================================ -->
    <!-- Step 2: 组件选择器 -->
    <!-- ============================================ -->
    <div class="mb-8">
      <!-- 操作栏 -->
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">{{ t('installer.title') }}</h2>
        <div class="flex items-center gap-2">
          <button
            @click="solverStore.selectAll()"
            class="px-3 py-1.5 text-xs rounded border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors"
          >
            {{ t('installer.selectAll') }}
          </button>
          <button
            @click="solverStore.deselectAll()"
            class="px-3 py-1.5 text-xs rounded border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors"
          >
            {{ t('installer.deselectAll') }}
          </button>
          <button
            @click="refreshStatus"
            :disabled="solverStore.loading"
            class="px-3 py-1.5 text-xs rounded border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors flex items-center gap-1 disabled:opacity-50"
          >
            <svg class="w-3 h-3" :class="{ 'animate-spin': solverStore.loading }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            {{ t('installer.refresh') }}
          </button>
        </div>
      </div>

      <!-- 加载骨架屏 -->
      <div v-if="solverStore.loading && solverStore.solvers.length === 0" class="space-y-3">
        <div v-for="i in 4" :key="i" class="skeleton h-32 rounded-lg"></div>
      </div>

      <!-- 求解器卡片列表 -->
      <div v-else class="space-y-3">
        <div
          v-for="solver in solverStore.solvers"
          :key="solver.name"
          class="rounded-lg border transition-all duration-200"
          :class="[
            solver.installed
              ? 'border-[var(--accent-green)] bg-[var(--bg-surface)]'
              : 'border-[var(--border-subtle)] bg-[var(--bg-surface)]',
            solverStore.selectedSolvers.has(solver.name) && !solver.installed
              ? 'ring-1 ring-[var(--primary)] ring-opacity-30'
              : '',
            solverStore.installing === solver.name
              ? 'ring-2 ring-[var(--primary)] shadow-md'
              : '',
          ]"
        >
          <div class="p-4">
            <div class="flex items-start justify-between gap-4">
              <!-- 左侧：图标 + 信息 -->
              <div class="flex items-start gap-3 flex-1 min-w-0">
                <!-- 选中复选框 -->
                <input
                  type="checkbox"
                  :checked="solverStore.selectedSolvers.has(solver.name)"
                  @change="solverStore.toggleSelect(solver.name)"
                  :disabled="solver.installed"
                  class="mt-1 w-4 h-4 rounded border-[var(--border-default)] text-[var(--primary)] focus:ring-[var(--primary)] disabled:opacity-40"
                />

                <!-- 图标 -->
                <div class="w-10 h-10 rounded-lg flex items-center justify-center text-lg shrink-0"
                  :class="getSolverIconBg(solver.scale)">
                  {{ getSolverIcon(solver.scale) }}
                </div>

                <!-- 信息 -->
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 flex-wrap">
                    <span class="font-semibold text-sm">{{ solver.display_name }}</span>
                    <!-- 状态徽章 -->
                    <span
                      class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
                      :class="getStatusBadgeClass(solver)"
                    >
                      <span v-if="solverStore.installing === solver.name" class="animate-spin">&#x1f504;</span>
                      <span v-else-if="solver.installed">&#x2705;</span>
                      <span v-else>&#x2b1c;</span>
                      {{ getStatusText(solver) }}
                    </span>
                    <span v-if="solver.version" class="text-xs text-[var(--text-muted)]">
                      v{{ solver.version }}
                    </span>
                  </div>
                  <p class="text-xs text-[var(--text-secondary)] mt-1">
                    {{ getSolverDescription(solver.name) }}
                  </p>
                  <p v-if="solver.path" class="text-xs text-[var(--text-muted)] mt-0.5 truncate">
                    {{ solver.path }}
                  </p>
                </div>
              </div>

              <!-- 右侧：操作按钮 -->
              <div class="flex items-center gap-2 shrink-0">
                <!-- 安装方法选择 -->
                <select
                  v-if="!solver.installed && solverStore.installing !== solver.name"
                  v-model="selectedMethods[solver.name]"
                  class="text-xs p-1.5 rounded border border-[var(--border-subtle)] bg-[var(--bg-base)] focus:outline-none focus:ring-1 focus:ring-[var(--primary)]"
                >
                  <option v-for="m in solverStore.installMethods" :key="m.name" :value="m.name" :disabled="!m.available">
                    {{ m.display_name }}
                  </option>
                </select>

                <!-- 安装按钮 -->
                <button
                  v-if="!solver.installed && solverStore.installing !== solver.name"
                  @click="handleInstall(solver.name)"
                  class="px-3 py-1.5 text-xs rounded bg-[var(--primary)] text-white hover:opacity-90 transition-opacity"
                >
                  {{ t('installer.install') }}
                </button>

                <!-- 安装中指示 -->
                <span v-if="solverStore.installing === solver.name"
                  class="px-3 py-1.5 text-xs rounded bg-[var(--primary)] text-white opacity-80">
                  {{ t('installer.installing') }}...
                </span>

                <!-- 已安装：卸载 + 验证 -->
                <template v-if="solver.installed">
                  <button
                    @click="handleVerify(solver.name)"
                    :disabled="verifying === solver.name"
                    class="px-3 py-1.5 text-xs rounded border border-[var(--border-subtle)] hover:bg-[var(--bg-hover)] transition-colors disabled:opacity-50"
                  >
                    {{ verifying === solver.name ? t('installer.verifying') + '...' : t('installer.verify') }}
                  </button>
                  <button
                    @click="handleUninstall(solver.name)"
                    class="px-3 py-1.5 text-xs rounded border border-[var(--accent-red)] text-[var(--accent-red)] hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                  >
                    {{ t('installer.uninstall') }}
                  </button>
                </template>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 一键安装按钮 -->
      <div v-if="solverStore.selectedCount > 0" class="mt-4 flex items-center gap-3">
        <button
          @click="handleInstallSelected"
          :disabled="solverStore.isInstalling"
          class="px-4 py-2 text-sm rounded-lg bg-[var(--primary)] text-white hover:opacity-90 transition-opacity flex items-center gap-2 disabled:opacity-50"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
          {{ t('installer.installSelected') }} ({{ solverStore.selectedCount }})
        </button>
      </div>
    </div>

    <!-- ============================================ -->
    <!-- Step 3: 安装进度 -->
    <!-- ============================================ -->
    <div v-if="solverStore.isInstalling || solverStore.installLogs.length > 0" class="mb-8">
      <h2 class="text-lg font-semibold mb-3">{{ t('installer.installing') }}</h2>
      <div class="rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] overflow-hidden">
        <!-- 进度条 -->
        <div class="h-2 bg-[var(--bg-elevated)]">
          <div
            class="h-full bg-gradient-to-r from-blue-500 to-indigo-500 transition-all duration-500"
            :style="{ width: installProgressPercent + '%' }"
          ></div>
        </div>

        <!-- 进度消息 -->
        <div v-if="solverStore.installProgress" class="px-4 py-2 text-sm border-b border-[var(--border-subtle)]">
          {{ solverStore.installProgress }}
        </div>

        <!-- 日志输出 -->
        <div class="p-4 max-h-48 overflow-y-auto font-mono text-xs text-[var(--text-secondary)] space-y-1">
          <div v-for="(log, idx) in solverStore.installLogs" :key="idx" class="flex gap-2">
            <span class="text-[var(--text-muted)] shrink-0">[{{ idx + 1 }}]</span>
            <span :class="log.includes('FAILED') || log.includes('ERROR') ? 'text-[var(--accent-red)]' : ''">{{ log }}</span>
          </div>
          <div v-if="solverStore.isInstalling" class="flex gap-2 animate-pulse">
            <span class="text-[var(--text-muted)] shrink-0">...</span>
            <span>{{ t('installer.installing') }}...</span>
          </div>
        </div>
      </div>
    </div>

    <!-- ============================================ -->
    <!-- Step 4: 验证结果 -->
    <!-- ============================================ -->
    <div v-if="Object.keys(solverStore.verifyResults).length > 0" class="mb-8">
      <h2 class="text-lg font-semibold mb-3">{{ t('installer.verifySuccess') }}</h2>
      <div class="rounded-lg border border-[var(--border-subtle)] bg-[var(--bg-surface)] overflow-hidden">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
              <th class="text-left px-4 py-2 font-medium text-[var(--text-secondary)]">{{ t('installer.status') }}</th>
              <th class="text-left px-4 py-2 font-medium text-[var(--text-secondary)]">{{ t('installer.version') }}</th>
              <th class="text-left px-4 py-2 font-medium text-[var(--text-secondary)]">{{ t('installer.path') }}</th>
              <th class="text-left px-4 py-2 font-medium text-[var(--text-secondary)]">{{ t('installer.status') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(result, name) in solverStore.verifyResults"
              :key="name"
              class="border-b border-[var(--border-subtle)] last:border-b-0"
            >
              <td class="px-4 py-2 font-medium">{{ getDisplayName(name) }}</td>
              <td class="px-4 py-2 text-[var(--text-secondary)]">
                {{ result.version || '-' }}
              </td>
              <td class="px-4 py-2 text-[var(--text-muted)] font-mono text-xs truncate max-w-xs">
                {{ result.error_message || '-' }}
              </td>
              <td class="px-4 py-2">
                <span
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium"
                  :class="result.works
                    ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
                    : 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'"
                >
                  {{ result.works ? t('installer.verifySuccess') : t('installer.verifyFailed') }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 错误提示 -->
    <div v-if="solverStore.error" class="mb-6">
      <div class="rounded-lg border border-[var(--accent-red)] bg-red-50 dark:bg-red-900/20 p-4 text-sm text-[var(--accent-red)]">
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span>{{ solverStore.error }}</span>
          <button @click="solverStore.clearError()" class="ml-auto hover:opacity-70">&times;</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSolverStore } from '@/stores/solverStore'

const { t } = useI18n()
const solverStore = useSolverStore()

/** 正在验证的求解器 */
const verifying = ref<string | null>(null)

/** 每个求解器选择的安装方法 */
const selectedMethods = reactive<Record<string, string>>({})

/** 安装进度百分比 */
const installProgressPercent = computed(() => {
  if (!solverStore.isInstalling) return 100
  const total = solverStore.selectedCount
  if (total === 0) return 0
  const logs = solverStore.installLogs.length
  return Math.min(Math.round((logs / total) * 100), 95)
})

/** 四个尺度预览卡片 */
const scaleCards = computed(() => [
  { key: 'md', icon: '\u269B\uFE0F', name: 'MD', label: t('installer.md.name') },
  { key: 'dft', icon: '\u2699\uFE0F', name: 'DFT', label: t('installer.dft.name') },
  { key: 'phaseField', icon: '\uD83C\uDF0A', name: t('installer.phaseField.name'), label: 'Phase Field' },
  { key: 'fe', icon: '\uD83D\uDEE0\uFE0F', name: t('installer.fe.name'), label: 'FE' },
])

/** 获取求解器图标 */
function getSolverIcon(scale: string): string {
  const icons: Record<string, string> = {
    md: '\u269B\uFE0F',
    dft: '\u2699\uFE0F',
    phase_field: '\uD83C\uDF0A',
    fe: '\uD83D\uDEE0\uFE0F',
  }
  return icons[scale] || '\uD83D\uDD27'
}

/** 获取求解器图标背景色 */
function getSolverIconBg(scale: string): string {
  const bgs: Record<string, string> = {
    md: 'bg-gradient-to-br from-blue-500 to-cyan-500',
    dft: 'bg-gradient-to-br from-purple-500 to-pink-500',
    phase_field: 'bg-gradient-to-br from-green-500 to-teal-500',
    fe: 'bg-gradient-to-br from-orange-500 to-red-500',
  }
  return bgs[scale] || 'bg-gradient-to-br from-gray-500 to-gray-600'
}

/** 获取求解器描述 */
function getSolverDescription(name: string): string {
  const descs: Record<string, string> = {
    lammps: t('installer.md.desc'),
    quantum_espresso: t('installer.dft.desc'),
    fipy: t('installer.phaseField.desc'),
    calculix: t('installer.fe.desc'),
  }
  return descs[name] || ''
}

/** 获取显示名称 */
function getDisplayName(name: string): string {
  const solver = solverStore.solvers.find(s => s.name === name)
  return solver?.display_name || name
}

/** 获取状态徽章样式 */
function getStatusBadgeClass(solver: { installed: boolean; name: string }): string {
  if (solverStore.installing === solver.name) {
    return 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
  }
  if (solver.installed) {
    return 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
  }
  return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
}

/** 获取状态文本 */
function getStatusText(solver: { installed: boolean; name: string }): string {
  if (solverStore.installing === solver.name) {
    return t('installer.installing')
  }
  if (solver.installed) {
    return t('installer.installed')
  }
  return t('installer.notInstalled')
}

/** 刷新状态 */
async function refreshStatus() {
  await solverStore.detect()
  await solverStore.loadInstallMethods()
}

/** 安装单个求解器 */
async function handleInstall(solverName: string) {
  const method = selectedMethods[solverName] || solverStore.installMethods[0]?.name || 'apt'
  await solverStore.install(solverName, method)
}

/** 批量安装选中求解器 */
async function handleInstallSelected() {
  await solverStore.installSelected((name: string) => {
    return selectedMethods[name] || solverStore.installMethods[0]?.name || 'apt'
  })
}

/** 验证求解器 */
async function handleVerify(solverName: string) {
  verifying.value = solverName
  await solverStore.verify(solverName)
  verifying.value = null
}

/** 卸载求解器 */
async function handleUninstall(solverName: string) {
  const result = await solverStore.uninstall(solverName)
  if (!result.success) {
    console.error('Uninstall failed:', result.message)
  }
}

/** 初始化 */
onMounted(async () => {
  await Promise.all([
    solverStore.detect(),
    solverStore.loadInstallMethods(),
  ])
  // 设置默认安装方法
  solverStore.installMethods.forEach(m => {
    if (m.available) {
      solverStore.solvers.forEach(s => {
        if (!selectedMethods[s.name]) {
          selectedMethods[s.name] = m.name
        }
      })
    }
  })
})
</script>
