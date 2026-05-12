/**
 * useOperationPlayback.ts — V3.3-002 操作步骤回放
 * 学生仿真操作记录，支持老师回放检查
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export interface PlaybackSession {
  id: string
  studentId: string
  studentName: string
  assignmentId?: string
  assignmentTitle?: string
  createdAt: string
  duration: number  // 总时长（秒）
  operations: Operation[]
}

export interface Operation {
  id: string
  timestamp: number  // 相对于会话开始的毫秒数
  type: 'modeling' | 'mesh' | 'simulation' | 'postprocess' | 'navigation' | 'settings' | 'save'
  action: string
  target?: string  // 操作目标（如几何体 ID、网格 ID）
  details?: Record<string, any>
  screenshot?: string  // Base64 截图
  stateSnapshot?: any  // 操作后的状态快照
}

export interface PlaybackState {
  isPlaying: boolean
  isPaused: boolean
  currentOperationIndex: number
  currentTime: number  // 毫秒
  playbackSpeed: number  // 1x, 2x, 4x, 8x
  totalDuration: number  // 毫秒
}

export interface OperationFilter {
  type?: Operation['type']
  startTime?: number
  endTime?: number
  searchText?: string
}

export interface OperationSummary {
  type: Operation['type']
  count: number
  totalDuration: number
  averageDuration: number
  operations: Operation[]
}

// ============ 存储键 ============

const SESSIONS_KEY = 'caelab_playback_sessions'

// ============ 工具函数 ============

function generateId(): string {
  return `playback_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function formatTime(ms: number): string {
  const seconds = Math.floor(ms / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)

  if (hours > 0) {
    return `${hours}:${String(minutes % 60).padStart(2, '0')}:${String(seconds % 60).padStart(2, '0')}`
  }
  return `${minutes}:${String(seconds % 60).padStart(2, '0')}`
}

// ============ 主 Composable ============

export function useOperationPlayback() {
  const sessions = ref<PlaybackSession[]>([])

  // 当前回放状态
  const currentSessionId = ref<string | null>(null)
  const playbackState = ref<PlaybackState>({
    isPlaying: false,
    isPaused: false,
    currentOperationIndex: 0,
    currentTime: 0,
    playbackSpeed: 1,
    totalDuration: 0
  })

  // 回放定时器
  let playbackTimer: ReturnType<typeof setInterval> | null = null

  // ============ 计算属性 ============

  const currentSession = computed(() =>
    sessions.value.find(s => s.id === currentSessionId.value) || null
  )

  const currentOperations = computed(() =>
    currentSession.value?.operations || []
  )

  const currentOperation = computed(() =>
    currentOperations.value[playbackState.value.currentOperationIndex] || null
  )

  const progress = computed(() => {
    if (playbackState.value.totalDuration === 0) return 0
    return (playbackState.value.currentTime / playbackState.value.totalDuration) * 100
  })

  // ============ 初始化 ============

  function loadData(): void {
    const storedSessions = getStorage<PlaybackSession[]>(SESSIONS_KEY)
    if (storedSessions) sessions.value = storedSessions
  }

  // ============ 会话管理 ============

  /**
   * 创建新的回放会话
   */
  function createSession(
    studentId: string,
    studentName: string,
    options?: {
      assignmentId?: string
      assignmentTitle?: string
    }
  ): PlaybackSession {
    const session: PlaybackSession = {
      id: generateId(),
      studentId,
      studentName,
      assignmentId: options?.assignmentId,
      assignmentTitle: options?.assignmentTitle,
      createdAt: new Date().toISOString(),
      duration: 0,
      operations: []
    }
    sessions.value.unshift(session)
    saveData()
    return session
  }

  /**
   * 记录操作
   */
  function recordOperation(
    sessionId: string,
    operation: Omit<Operation, 'id' | 'timestamp'>,
    startTime?: number
  ): Operation {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) throw new Error('Session not found')

    const timestamp = startTime || (session.operations.length > 0
      ? session.operations[session.operations.length - 1].timestamp + 1000
      : 0)

    const op: Operation = {
      ...operation,
      id: generateId(),
      timestamp
    }

    session.operations.push(op)

    // 更新会话时长
    const lastOp = session.operations[session.operations.length - 1]
    session.duration = Math.ceil(lastOp.timestamp / 1000)

    saveData()
    return op
  }

  /**
   * 批量记录操作
   */
  function recordOperations(
    sessionId: string,
    operations: Array<Omit<Operation, 'id' | 'timestamp'>>
  ): Operation[] {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) throw new Error('Session not found')

    let timestamp = session.operations.length > 0
      ? session.operations[session.operations.length - 1].timestamp + 1000
      : 0

    const recordedOps: Operation[] = []

    for (const op of operations) {
      recordedOps.push({
        ...op,
        id: generateId(),
        timestamp
      })
      timestamp += 500  // 默认间隔 500ms
    }

    session.operations.push(...recordedOps)

    // 更新会话时长
    const lastOp = session.operations[session.operations.length - 1]
    session.duration = Math.ceil(lastOp.timestamp / 1000)

    saveData()
    return recordedOps
  }

  /**
   * 添加截图
   */
  function addScreenshot(sessionId: string, operationId: string, screenshot: string): boolean {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) return false

    const operation = session.operations.find(op => op.id === operationId)
    if (!operation) return false

    operation.screenshot = screenshot
    saveData()
    return true
  }

  /**
   * 添加状态快照
   */
  function addStateSnapshot(sessionId: string, operationId: string, snapshot: any): boolean {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) return false

    const operation = session.operations.find(op => op.id === operationId)
    if (!operation) return false

    operation.stateSnapshot = snapshot
    saveData()
    return true
  }

  // ============ 回放控制 ============

  /**
   * 加载会话进行回放
   */
  function loadSession(sessionId: string): boolean {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) return false

    // 停止当前回放
    stopPlayback()

    currentSessionId.value = sessionId
    playbackState.value = {
      isPlaying: false,
      isPaused: false,
      currentOperationIndex: 0,
      currentTime: 0,
      playbackSpeed: 1,
      totalDuration: session.operations.length > 0
        ? session.operations[session.operations.length - 1].timestamp
        : 0
    }

    return true
  }

  /**
   * 开始播放
   */
  function play(): void {
    if (!currentSession.value) return

    playbackState.value.isPlaying = true
    playbackState.value.isPaused = false

    const interval = 1000 / playbackState.value.playbackSpeed

    playbackTimer = setInterval(() => {
      if (!playbackState.value.isPlaying || playbackState.value.isPaused) {
        return
      }

      playbackState.value.currentTime += 100

      // 更新当前操作索引
      const ops = currentOperations.value
      for (let i = 0; i < ops.length; i++) {
        if (ops[i].timestamp <= playbackState.value.currentTime) {
          playbackState.value.currentOperationIndex = i
        }
      }

      // 检查是否播放完毕
      if (playbackState.value.currentTime >= playbackState.value.totalDuration) {
        pause()
        playbackState.value.currentTime = playbackState.value.totalDuration
      }
    }, interval)
  }

  /**
   * 暂停播放
   */
  function pause(): void {
    playbackState.value.isPaused = true
    playbackState.value.isPlaying = false

    if (playbackTimer) {
      clearInterval(playbackTimer)
      playbackTimer = null
    }
  }

  /**
   * 停止播放
   */
  function stopPlayback(): void {
    pause()
    playbackState.value.currentTime = 0
    playbackState.value.currentOperationIndex = 0
  }

  /**
   * 跳转到指定时间
   */
  function seekTo(timeMs: number): void {
    playbackState.value.currentTime = Math.max(0, Math.min(timeMs, playbackState.value.totalDuration))

    // 更新当前操作索引
    const ops = currentOperations.value
    for (let i = 0; i < ops.length; i++) {
      if (ops[i].timestamp <= playbackState.value.currentTime) {
        playbackState.value.currentOperationIndex = i
      }
    }
  }

  /**
   * 跳转到指定操作
   */
  function seekToOperation(operationIndex: number): void {
    const ops = currentOperations.value
    if (operationIndex < 0 || operationIndex >= ops.length) return

    playbackState.value.currentOperationIndex = operationIndex
    playbackState.value.currentTime = ops[operationIndex].timestamp
  }

  /**
   * 播放到指定操作
   */
  function playToOperation(operationIndex: number): void {
    seekToOperation(operationIndex)
    play()
  }

  /**
   * 下一操作
   */
  function nextOperation(): void {
    const nextIndex = playbackState.value.currentOperationIndex + 1
    if (nextIndex < currentOperations.value.length) {
      seekToOperation(nextIndex)
    }
  }

  /**
   * 上一操作
   */
  function previousOperation(): void {
    const prevIndex = playbackState.value.currentOperationIndex - 1
    if (prevIndex >= 0) {
      seekToOperation(prevIndex)
    }
  }

  /**
   * 设置播放速度
   */
  function setPlaybackSpeed(speed: number): void {
    playbackState.value.playbackSpeed = speed

    // 如果正在播放，需要重启定时器
    if (playbackState.value.isPlaying && !playbackState.value.isPaused) {
      pause()
      play()
    }
  }

  // ============ 操作过滤和搜索 ============

  /**
   * 过滤操作
   */
  function filterOperations(filter: OperationFilter): Operation[] {
    if (!currentSession.value) return []

    return currentOperations.value.filter(op => {
      if (filter.type && op.type !== filter.type) return false
      if (filter.startTime && op.timestamp < filter.startTime) return false
      if (filter.endTime && op.timestamp > filter.endTime) return false
      if (filter.searchText) {
        const searchLower = filter.searchText.toLowerCase()
        return op.action.toLowerCase().includes(searchLower) ||
          (op.target?.toLowerCase().includes(searchLower)) ||
          (op.details && JSON.stringify(op.details).toLowerCase().includes(searchLower))
      }
      return true
    })
  }

  /**
   * 按类型分组操作
   */
  function groupByType(): Record<Operation['type'], OperationSummary> {
    if (!currentSession.value) return {} as Record<Operation['type'], OperationSummary>

    const groups: Record<string, Operation[]> = {}

    for (const op of currentOperations.value) {
      if (!groups[op.type]) {
        groups[op.type] = []
      }
      groups[op.type].push(op)
    }

    const result: Record<Operation['type'], OperationSummary> = {} as Record<Operation['type'], OperationSummary>

    for (const [type, ops] of Object.entries(groups)) {
      const sortedOps = [...ops].sort((a, b) => a.timestamp - b.timestamp)
      let totalDuration = 0

      for (let i = 0; i < sortedOps.length - 1; i++) {
        totalDuration += sortedOps[i + 1].timestamp - sortedOps[i].timestamp
      }

      result[type as Operation['type']] = {
        type: type as Operation['type'],
        count: ops.length,
        totalDuration,
        averageDuration: ops.length > 1 ? totalDuration / (ops.length - 1) : 0,
        operations: sortedOps
      }
    }

    return result
  }

  /**
   * 生成操作摘要
   */
  function getOperationSummary(): {
    totalOperations: number
    totalDuration: number
    typeBreakdown: Record<Operation['type'], number>
    mostCommonAction: string
    criticalOperations: Operation[]
  } {
    if (!currentSession.value) {
      return {
        totalOperations: 0,
        totalDuration: 0,
        typeBreakdown: {} as Record<Operation['type'], number>,
        mostCommonAction: '',
        criticalOperations: []
      }
    }

    const ops = currentOperations.value
    const typeBreakdown: Record<string, number> = {}
    const actionCounts: Record<string, number> = {}
    const criticalOps: Operation[] = []

    for (const op of ops) {
      typeBreakdown[op.type] = (typeBreakdown[op.type] || 0) + 1
      actionCounts[op.action] = (actionCounts[op.action] || 0) + 1

      // 标记关键操作
      if (op.type === 'simulation' || op.action.includes('run') || op.action.includes('solve')) {
        criticalOps.push(op)
      }
    }

    let mostCommonAction = ''
    let maxCount = 0
    for (const [action, count] of Object.entries(actionCounts)) {
      if (count > maxCount) {
        maxCount = count
        mostCommonAction = action
      }
    }

    return {
      totalOperations: ops.length,
      totalDuration: currentSession.value.duration,
      typeBreakdown: typeBreakdown as Record<Operation['type'], number>,
      mostCommonAction,
      criticalOperations: criticalOps
    }
  }

  // ============ 时间线视图 ============

  /**
   * 生成时间线数据
   */
  function generateTimeline(): Array<{
    time: number
    timeFormatted: string
    operation?: Operation
    isMilestone: boolean
  }> {
    if (!currentSession.value) return []

    const ops = currentOperations.value
    const milestones: Array<{ time: number; timeFormatted: string; operation?: Operation; isMilestone: boolean }> = []

    // 添加起始点
    milestones.push({
      time: 0,
      timeFormatted: '0:00',
      isMilestone: true
    })

    // 为每个操作创建时间点
    for (let i = 0; i < ops.length; i++) {
      const op = ops[i]
      milestones.push({
        time: op.timestamp,
        timeFormatted: formatTime(op.timestamp),
        operation: op,
        isMilestone: ['simulation', 'save'].includes(op.type) ||
          op.action.includes('run') ||
          op.action.includes('solve') ||
          op.action.includes('export')
      })
    }

    // 添加结束点
    milestones.push({
      time: playbackState.value.totalDuration,
      timeFormatted: formatTime(playbackState.value.totalDuration),
      isMilestone: true
    })

    return milestones
  }

  // ============ 会话查询 ============

  function getSession(sessionId: string): PlaybackSession | undefined {
    return sessions.value.find(s => s.id === sessionId)
  }

  function getStudentSessions(studentId: string): PlaybackSession[] {
    return sessions.value
      .filter(s => s.studentId === studentId)
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
  }

  function getAssignmentSessions(assignmentId: string): PlaybackSession[] {
    return sessions.value
      .filter(s => s.assignmentId === assignmentId)
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
  }

  function deleteSession(sessionId: string): boolean {
    const index = sessions.value.findIndex(s => s.id === sessionId)
    if (index === -1) return false

    sessions.value.splice(index, 1)

    if (currentSessionId.value === sessionId) {
      stopPlayback()
      currentSessionId.value = null
    }

    saveData()
    return true
  }

  // ============ 导出 ============

  /**
   * 导出操作日志为文本
   */
  function exportAsText(sessionId: string): string {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) return ''

    const lines: string[] = []
    lines.push('=' .repeat(60))
    lines.push('OPERATION PLAYBACK LOG')
    lines.push('=' .repeat(60))
    lines.push('')
    lines.push(`Student: ${session.studentName}`)
    lines.push(`Assignment: ${session.assignmentTitle || 'N/A'}`)
    lines.push(`Date: ${new Date(session.createdAt).toLocaleDateString()}`)
    lines.push(`Total Duration: ${formatTime(session.duration * 1000)}`)
    lines.push(`Total Operations: ${session.operations.length}`)
    lines.push('')

    // 按类型分组统计
    const typeGroups = groupByType()
    lines.push('STATISTICS BY TYPE:')
    for (const [type, summary] of Object.entries(typeGroups)) {
      lines.push(`  ${type}: ${summary.count} operations, avg ${Math.round(summary.averageDuration / 1000)}s each`)
    }
    lines.push('')

    // 操作详情
    lines.push('OPERATION DETAILS:')
    for (let i = 0; i < session.operations.length; i++) {
      const op = session.operations[i]
      const timeStr = formatTime(op.timestamp)
      const screenshotStr = op.screenshot ? ' [SCREENSHOT]' : ''
      const snapshotStr = op.stateSnapshot ? ' [SNAPSHOT]' : ''

      lines.push(`  [${timeStr}] ${op.type.toUpperCase()}: ${op.action}${screenshotStr}${snapshotStr}`)
      if (op.target) {
        lines.push(`      Target: ${op.target}`)
      }
      if (op.details) {
        lines.push(`      Details: ${JSON.stringify(op.details)}`)
      }
    }

    return lines.join('\n')
  }

  /**
   * 导出为 JSON
   */
  function exportAsJSON(sessionId: string): string {
    const session = sessions.value.find(s => s.id === sessionId)
    if (!session) return '{}'
    return JSON.stringify(session, null, 2)
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(SESSIONS_KEY, sessions.value)
  }

  function clearAllData(): void {
    sessions.value = []
    stopPlayback()
    currentSessionId.value = null
    saveData()
  }

  // 初始化
  loadData()

  return {
    // 状态
    sessions,
    currentSession,
    currentOperation,
    playbackState,
    progress,

    // 会话管理
    createSession,
    getSession,
    getStudentSessions,
    getAssignmentSessions,
    deleteSession,

    // 操作记录
    recordOperation,
    recordOperations,
    addScreenshot,
    addStateSnapshot,

    // 回放控制
    loadSession,
    play,
    pause,
    stopPlayback,
    seekTo,
    seekToOperation,
    playToOperation,
    nextOperation,
    previousOperation,
    setPlaybackSpeed,

    // 过滤和搜索
    filterOperations,
    groupByType,
    getOperationSummary,

    // 时间线
    generateTimeline,

    // 导出
    exportAsText,
    exportAsJSON,

    // 工具
    formatTime,

    // 持久化
    clearAllData
  }
}
