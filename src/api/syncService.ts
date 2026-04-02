/**
 * 跨端项目同步服务
 * 支持桌面端 <-> 移动端实时同步
 *
 * 同步策略：
 * - 基于 WebSocket 的实时双向同步
 * - 操作日志（Operation Log）模式：记录每次变更操作
 * - 向量时钟（Vector Clock）用于因果排序
 * - Last-Write-Wins 作为基础冲突解决策略
 * - 三路合并（3-way merge）用于复杂冲突
 */

import { ref } from 'vue'

/** 同步操作类型 */
export type SyncOpType =
  | 'create_project' | 'delete_project' | 'rename_project'
  | 'update_material' | 'update_mesh' | 'update_boundary_condition'
  | 'update_geometry' | 'add_result' | 'delete_result'
  | 'update_note' | 'add_note' | 'delete_note'
  | 'update_setting'

/** 单个同步操作 */
export interface SyncOperation {
  id: string
  type: SyncOpType
  projectId: string
  /** 操作数据（JSON） */
  data: any
  /** 向量时钟时间戳 */
  vectorClock: Record<string, number>
  /** 设备 ID */
  deviceId: string
  /** 操作时间（ISO 字符串） */
  timestamp: string
}

/** 同步状态 */
export type SyncStatus = 'disconnected' | 'connecting' | 'connected' | 'syncing' | 'error'

/** 同步配置 */
export interface SyncConfig {
  /** 同步服务器地址 */
  serverUrl: string
  /** 设备 ID（自动生成，存储在 localStorage） */
  deviceId?: string
  /** 认证 Token */
  authToken?: string
  /** 自动同步间隔（毫秒），0 表示实时 */
  syncInterval?: number
}

/** 冲突信息 */
export interface SyncConflict {
  id: string
  projectId: string
  opType: SyncOpType
  localData: any
  remoteData: any
  localTimestamp: string
  remoteTimestamp: string
  resolved: boolean
  resolution?: 'local_wins' | 'remote_wins' | 'merged'
  mergedData?: any
}

/** 同步统计 */
export interface SyncStats {
  totalOpsSent: number
  totalOpsReceived: number
  totalConflicts: number
  totalResolved: number
  lastSyncTime: string | null
  pendingOps: number
}

/** 同步日志条目 */
export interface SyncLogEntry {
  id: string
  timestamp: string
  direction: 'sent' | 'received' | 'conflict' | 'error' | 'info'
  message: string
  opType?: SyncOpType
  projectId?: string
}

/** 冲突解决策略 */
export type ConflictResolutionStrategy = 'local_wins' | 'remote_wins' | 'ask'

class SyncService {
  private ws: WebSocket | null = null
  private config: SyncConfig
  private vectorClock: Record<string, number> = {}
  private pendingOps: SyncOperation[] = []
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null
  private syncTimer: ReturnType<typeof setInterval> | null = null
  private listeners: Map<string, Set<Function>> = new Map()
  private syncLog: SyncLogEntry[] = []
  private maxLogEntries = 500
  private reconnectAttempts = 0
  private maxReconnectAttempts = 10
  private baseReconnectDelay = 1000
  private conflictStrategy: ConflictResolutionStrategy = 'ask'
  private disposed = false

  public status = ref<SyncStatus>('disconnected')
  public conflicts = ref<SyncConflict[]>([])
  public stats = ref<SyncStats>({
    totalOpsSent: 0,
    totalOpsReceived: 0,
    totalConflicts: 0,
    totalResolved: 0,
    lastSyncTime: null,
    pendingOps: 0,
  })

  constructor(config: SyncConfig) {
    this.config = {
      syncInterval: 0, // 默认实时
      ...config,
      deviceId: config.deviceId || this.getOrCreateDeviceId(),
    }
    // 从 localStorage 恢复向量时钟
    this.loadVectorClock()
    this.loadConflictStrategy()
    this.loadPendingOps()
  }

  // ==================== 设备 ID 管理 ====================

  private getOrCreateDeviceId(): string {
    let id = localStorage.getItem('caelab-device-id')
    if (!id) {
      id = `device_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
      localStorage.setItem('caelab-device-id', id)
    }
    return id
  }

  /** 获取当前设备 ID */
  getDeviceId(): string {
    return this.config.deviceId!
  }

  // ==================== 向量时钟管理 ====================

  private loadVectorClock() {
    const saved = localStorage.getItem('caelab-vector-clock')
    if (saved) {
      try { this.vectorClock = JSON.parse(saved) } catch { /* ignore corrupt data */ }
    }
  }

  private saveVectorClock() {
    localStorage.setItem('caelab-vector-clock', JSON.stringify(this.vectorClock))
  }

  private tickVectorClock(): void {
    const deviceId = this.config.deviceId!
    this.vectorClock[deviceId] = (this.vectorClock[deviceId] || 0) + 1
    this.saveVectorClock()
  }

  /** 合并远程向量时钟 */
  private mergeVectorClock(remoteClock: Record<string, number>): void {
    for (const [device, time] of Object.entries(remoteClock)) {
      this.vectorClock[device] = Math.max(this.vectorClock[device] || 0, time)
    }
    this.saveVectorClock()
  }

  /** 检查操作是否可以因果排序（happened-before 关系） */
  private happenedBefore(opA: SyncOperation, opB: SyncOperation): boolean {
    const clockA = opA.vectorClock
    const clockB = opB.vectorClock
    const deviceA = opA.deviceId
    const deviceB = opB.deviceId

    // A happened-before B 当且仅当 clockA[deviceA] < clockB[deviceA]
    // 且对于所有其他设备 d，clockA[d] <= clockB[d]
    if ((clockA[deviceA] || 0) >= (clockB[deviceA] || 0)) return false

    for (const device of new Set([...Object.keys(clockA), ...Object.keys(clockB)])) {
      if (device === deviceA) continue
      if ((clockA[device] || 0) > (clockB[device] || 0)) return false
    }

    return true
  }

  /** 检查两个操作是否并发（concurrent） */
  private isConcurrent(opA: SyncOperation, opB: SyncOperation): boolean {
    return !this.happenedBefore(opA, opB) && !this.happenedBefore(opB, opA)
  }

  // ==================== 待处理操作持久化 ====================

  private loadPendingOps() {
    const saved = localStorage.getItem('caelab-pending-ops')
    if (saved) {
      try { this.pendingOps = JSON.parse(saved) } catch { /* ignore */ }
    }
    this.stats.value.pendingOps = this.pendingOps.length
  }

  private savePendingOps() {
    localStorage.setItem('caelab-pending-ops', JSON.stringify(this.pendingOps))
    this.stats.value.pendingOps = this.pendingOps.length
  }

  // ==================== 冲突策略管理 ====================

  private loadConflictStrategy() {
    const saved = localStorage.getItem('caelab-conflict-strategy')
    if (saved && ['local_wins', 'remote_wins', 'ask'].includes(saved)) {
      this.conflictStrategy = saved as ConflictResolutionStrategy
    }
  }

  /** 设置冲突解决策略 */
  setConflictStrategy(strategy: ConflictResolutionStrategy): void {
    this.conflictStrategy = strategy
    localStorage.setItem('caelab-conflict-strategy', strategy)
    this.addLog('info', `冲突解决策略已更改为: ${strategy}`)
  }

  /** 获取当前冲突解决策略 */
  getConflictStrategy(): ConflictResolutionStrategy {
    return this.conflictStrategy
  }

  // ==================== 日志管理 ====================

  private addLog(
    direction: SyncLogEntry['direction'],
    message: string,
    opType?: SyncOpType,
    projectId?: string
  ): void {
    const entry: SyncLogEntry = {
      id: `log_${Date.now()}_${Math.random().toString(36).substr(2, 6)}`,
      timestamp: new Date().toISOString(),
      direction,
      message,
      opType,
      projectId,
    }
    this.syncLog.push(entry)
    // 限制日志条目数量
    if (this.syncLog.length > this.maxLogEntries) {
      this.syncLog = this.syncLog.slice(-this.maxLogEntries)
    }
    this.emit('log', entry)
  }

  /** 获取同步日志 */
  getSyncLog(): SyncLogEntry[] {
    return [...this.syncLog]
  }

  /** 清除同步日志 */
  clearSyncLog(): void {
    this.syncLog = []
    this.addLog('info', '同步日志已清除')
  }

  // ==================== WebSocket 连接管理 ====================

  /** 连接同步服务器 */
  connect(): void {
    if (this.disposed) return
    if (this.ws?.readyState === WebSocket.OPEN) return

    this.status.value = 'connecting'
    this.addLog('info', `正在连接同步服务器: ${this.config.serverUrl}`)

    try {
      const url = this.config.serverUrl.replace(/^http/, 'ws')
      this.ws = new WebSocket(url)

      this.ws.onopen = () => {
        this.status.value = 'connected'
        this.reconnectAttempts = 0
        this.addLog('info', '已连接到同步服务器')
        // 发送认证
        this.send({ type: 'auth', deviceId: this.config.deviceId, token: this.config.authToken })
        // 发送待处理操作
        this.flushPendingOps()
        // 设置定时同步（如果配置了间隔）
        this.setupSyncInterval()
      }

      this.ws.onmessage = (event) => {
        try {
          this.handleMessage(JSON.parse(event.data))
        } catch (e) {
          this.addLog('error', `解析消息失败: ${e}`)
        }
      }

      this.ws.onclose = (event) => {
        this.status.value = 'disconnected'
        this.addLog('info', `连接已关闭 (code: ${event.code}, reason: ${event.reason || '无'})`)
        this.scheduleReconnect()
      }

      this.ws.onerror = (error) => {
        this.status.value = 'error'
        this.addLog('error', 'WebSocket 连接错误')
      }
    } catch (e) {
      this.status.value = 'error'
      this.addLog('error', `连接失败: ${e}`)
      this.scheduleReconnect()
    }
  }

  /** 断开连接 */
  disconnect(): void {
    this.disposed = true
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
      this.reconnectTimer = null
    }
    if (this.syncTimer) {
      clearInterval(this.syncTimer)
      this.syncTimer = null
    }
    if (this.ws) {
      this.ws.close(1000, '用户主动断开')
      this.ws = null
    }
    this.status.value = 'disconnected'
    this.addLog('info', '已断开同步连接')
  }

  /** 重新启用连接（在 disconnect 后调用） */
  reconnect(): void {
    this.disposed = false
    this.reconnectAttempts = 0
    this.connect()
  }

  /** 指数退避重连 */
  private scheduleReconnect(): void {
    if (this.disposed) return
    if (this.reconnectTimer) return
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      this.addLog('error', `已达到最大重连次数 (${this.maxReconnectAttempts})，停止重连`)
      return
    }

    const delay = Math.min(
      this.baseReconnectDelay * Math.pow(2, this.reconnectAttempts),
      30000 // 最大 30 秒
    )
    this.reconnectAttempts++

    this.addLog('info', `将在 ${delay / 1000} 秒后重连 (第 ${this.reconnectAttempts} 次)`)

    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null
      this.connect()
    }, delay)
  }

  /** 设置定时同步 */
  private setupSyncInterval(): void {
    if (this.syncTimer) clearInterval(this.syncTimer)
    if (this.config.syncInterval && this.config.syncInterval > 0) {
      this.syncTimer = setInterval(() => {
        this.flushPendingOps()
      }, this.config.syncInterval)
    }
  }

  // ==================== 操作推送 ====================

  /** 记录并推送操作 */
  pushOperation(op: Omit<SyncOperation, 'id' | 'vectorClock' | 'deviceId' | 'timestamp'>): void {
    this.tickVectorClock()

    const syncOp: SyncOperation = {
      ...op,
      id: `op_${Date.now()}_${Math.random().toString(36).substr(2, 6)}`,
      vectorClock: { ...this.vectorClock },
      deviceId: this.config.deviceId!,
      timestamp: new Date().toISOString(),
    }

    this.pendingOps.push(syncOp)
    this.savePendingOps()

    this.addLog('sent', `推送操作: ${op.type}`, op.type, op.projectId)

    if (this.status.value === 'connected') {
      this.flushPendingOps()
    }

    this.emit('operation', syncOp)
  }

  /** 发送待处理操作 */
  private flushPendingOps(): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN || this.pendingOps.length === 0) return

    this.status.value = 'syncing'
    const ops = [...this.pendingOps]
    this.pendingOps = []
    this.savePendingOps()

    this.send({ type: 'ops', operations: ops })
    this.stats.value.totalOpsSent += ops.length
    this.addLog('sent', `批量发送 ${ops.length} 个操作`)
  }

  /** 手动触发同步 */
  syncNow(): void {
    if (this.status.value === 'connected') {
      this.flushPendingOps()
      // 请求服务器发送最新状态
      this.send({ type: 'request_sync', vectorClock: this.vectorClock })
      this.addLog('info', '手动触发同步')
    } else {
      this.connect()
    }
  }

  // ==================== 消息处理 ====================

  /** 处理收到的消息 */
  private handleMessage(msg: any): void {
    switch (msg.type) {
      case 'ops':
        // 收到远程操作
        for (const op of msg.operations) {
          this.processRemoteOp(op as SyncOperation)
        }
        this.stats.value.totalOpsReceived += msg.operations.length
        break
      case 'conflict':
        // 收到冲突
        this.handleConflict(msg.conflict as SyncConflict)
        break
      case 'sync_complete':
        this.stats.value.lastSyncTime = new Date().toISOString()
        if (this.status.value === 'syncing') {
          this.status.value = 'connected'
        }
        this.addLog('info', '同步完成')
        break
      case 'auth_result':
        if (msg.success) {
          this.addLog('info', '认证成功')
        } else {
          this.addLog('error', `认证失败: ${msg.reason || '未知原因'}`)
        }
        break
      case 'device_list':
        this.addLog('info', `已连接设备: ${msg.devices?.length || 0} 个`)
        this.emit('device_list', msg.devices)
        break
      default:
        this.addLog('info', `收到未知消息类型: ${msg.type}`)
    }
  }

  /** 处理远程操作 */
  private processRemoteOp(op: SyncOperation): void {
    // 向量时钟更新
    this.mergeVectorClock(op.vectorClock)

    this.addLog('received', `收到远程操作: ${op.type} (设备: ${op.deviceId})`, op.type, op.projectId)
    this.emit('remote_operation', op)
  }

  /** 处理冲突 */
  private handleConflict(conflict: SyncConflict): void {
    this.stats.value.totalConflicts++
    this.conflicts.value.push(conflict)
    this.addLog('conflict', `检测到冲突: ${conflict.opType} (项目: ${conflict.projectId})`, conflict.opType, conflict.projectId)

    // 根据策略自动解决冲突
    if (this.conflictStrategy === 'local_wins') {
      this.resolveConflict(conflict.id, 'local_wins')
    } else if (this.conflictStrategy === 'remote_wins') {
      this.resolveConflict(conflict.id, 'remote_wins')
    } else {
      // 'ask' 策略：通知用户手动解决
      this.emit('conflict', conflict)
    }
  }

  /** 解决冲突 */
  resolveConflict(
    conflictId: string,
    resolution: 'local_wins' | 'remote_wins' | 'merged',
    mergedData?: any
  ): void {
    const conflict = this.conflicts.value.find(c => c.id === conflictId)
    if (!conflict) return

    conflict.resolved = true
    conflict.resolution = resolution
    conflict.mergedData = mergedData

    this.stats.value.totalResolved++
    this.conflicts.value = [...this.conflicts.value.filter(c => !c.resolved)]

    this.send({ type: 'resolve_conflict', conflictId, resolution, mergedData })
    this.addLog('info', `冲突已解决: ${resolution}`, conflict.opType, conflict.projectId)
    this.emit('conflict_resolved', conflict)
  }

  /** 使用三路合并解决冲突 */
  resolveConflictWithMerge(conflictId: string, baseData: any): void {
    const conflict = this.conflicts.value.find(c => c.id === conflictId)
    if (!conflict) return

    const merged = SyncService.threeWayMerge(baseData, conflict.localData, conflict.remoteData)
    this.resolveConflict(conflictId, 'merged', merged)
  }

  // ==================== 三路合并算法 ====================

  /**
   * 三路合并算法
   * @param base 基础版本（双方共同的祖先）
   * @param local 本地修改版本
   * @param remote 远程修改版本
   * @returns 合并后的结果
   */
  static threeWayMerge(base: any, local: any, remote: any): any {
    // 情况 1: 本地未修改，采用远程版本
    if (JSON.stringify(local) === JSON.stringify(base)) return remote
    // 情况 2: 远程未修改，采用本地版本
    if (JSON.stringify(remote) === JSON.stringify(base)) return local
    // 情况 3: 双方修改一致，采用任意一方
    if (JSON.stringify(local) === JSON.stringify(remote)) return local

    // 情况 4: 对象递归合并
    if (typeof base === 'object' && base !== null && !Array.isArray(base)) {
      const result: any = { ...base }
      const allKeys = new Set([...Object.keys(local), ...Object.keys(remote)])

      for (const key of allKeys) {
        const baseVal = base[key]
        const localVal = local[key]
        const remoteVal = remote[key]

        if (!(key in base)) {
          // 新增字段：双方都新增则冲突（取 local），仅一方新增则采用
          if (key in local && key in remote) {
            if (JSON.stringify(localVal) === JSON.stringify(remoteVal)) {
              result[key] = localVal
            } else {
              // 双方都新增了同名字段但值不同 → 冲突，保留 local
              result[key] = localVal
            }
          } else {
            result[key] = localVal ?? remoteVal
          }
        } else if (localVal === undefined || localVal === null) {
          // 本地删除了该字段
          if (remoteVal === undefined || remoteVal === null) {
            delete result[key]
          } else {
            // 本地删除，远程修改 → 保留远程（保守策略）
            result[key] = remoteVal
          }
        } else if (remoteVal === undefined || remoteVal === null) {
          // 远程删除了该字段
          if (localVal === undefined || localVal === null) {
            delete result[key]
          } else {
            // 远程删除，本地修改 → 保留本地（保守策略）
            result[key] = localVal
          }
        } else {
          // 双方都修改了该字段
          if (JSON.stringify(localVal) === JSON.stringify(baseVal)) {
            // 本地未实际修改，采用远程
            result[key] = remoteVal
          } else if (JSON.stringify(remoteVal) === JSON.stringify(baseVal)) {
            // 远程未实际修改，采用本地
            result[key] = localVal
          } else if (JSON.stringify(localVal) === JSON.stringify(remoteVal)) {
            // 双方修改一致
            result[key] = localVal
          } else if (
            typeof localVal === 'object' && localVal !== null && !Array.isArray(localVal) &&
            typeof remoteVal === 'object' && remoteVal !== null && !Array.isArray(remoteVal) &&
            typeof baseVal === 'object' && baseVal !== null && !Array.isArray(baseVal)
          ) {
            // 嵌套对象：递归合并
            result[key] = SyncService.threeWayMerge(baseVal, localVal, remoteVal)
          } else {
            // 基本类型或数组冲突 → 保留 local
            result[key] = localVal
          }
        }
      }
      return result
    }

    // 情况 5: 数组或基本类型冲突 → local wins
    return local
  }

  // ==================== 数据清理 ====================

  /** 清除所有同步数据（向量时钟、待处理操作、日志） */
  clearSyncData(): void {
    this.vectorClock = {}
    this.pendingOps = []
    this.syncLog = []
    this.conflicts.value = []
    this.stats.value = {
      totalOpsSent: 0,
      totalOpsReceived: 0,
      totalConflicts: 0,
      totalResolved: 0,
      lastSyncTime: null,
      pendingOps: 0,
    }

    localStorage.removeItem('caelab-vector-clock')
    localStorage.removeItem('caelab-pending-ops')
    localStorage.removeItem('caelab-sync-log')

    this.addLog('info', '同步数据已清除')
  }

  /** 重置设备 ID（生成新 ID） */
  resetDeviceId(): void {
    const newId = `device_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
    localStorage.setItem('caelab-device-id', newId)
    this.config.deviceId = newId
    this.addLog('info', `设备 ID 已重置: ${newId}`)
  }

  // ==================== 低层通信 ====================

  private send(data: any): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data))
    }
  }

  // ==================== 事件系统 ====================

  on(event: string, callback: Function): () => void {
    if (!this.listeners.has(event)) this.listeners.set(event, new Set())
    this.listeners.get(event)!.add(callback)
    return () => this.listeners.get(event)?.delete(callback)
  }

  off(event: string, callback: Function): void {
    this.listeners.get(event)?.delete(callback)
  }

  private emit(event: string, data: any): void {
    this.listeners.get(event)?.forEach(cb => {
      try { cb(data) } catch (e) {
        console.error(`SyncService event handler error [${event}]:`, e)
      }
    })
  }
}

// ==================== 单例管理 ====================

let instance: SyncService | null = null

/** 获取同步服务单例 */
export function useSyncService(config?: SyncConfig): SyncService {
  if (!instance && config) {
    instance = new SyncService(config)
  }
  if (!instance) {
    // 默认配置（本地模式）
    instance = new SyncService({ serverUrl: 'ws://localhost:3001/ws' })
  }
  return instance
}

/** 销毁同步服务单例 */
export function destroySyncService(): void {
  if (instance) {
    instance.disconnect()
    instance = null
  }
}

export { SyncService }
