/**
 * useRealTimeCollaboration.ts — V3.2-006 实时协作
 * 多人同时编辑同一项目，实时光标追踪，冲突解决
 */

import { ref, computed, onUnmounted } from 'vue'

// ============ 类型定义 ============

export interface Collaborator {
  id: string
  name: string
  avatarColor: string
  cursor?: CursorPosition
  lastActive: string
  isOnline: boolean
}

export interface CursorPosition {
  x: number
  y: number
  elementId?: string
  selection?: { start: number; end: number }
}

export interface CollaborationSession {
  id: string
  projectId: string
  createdAt: string
  collaborators: Collaborator[]
  chatMessages: ChatMessage[]
}

export interface ChatMessage {
  id: string
  authorId: string
  authorName: string
  content: string
  timestamp: string
  type: 'message' | 'annotation' | 'system'
}

export interface ChangeOperation {
  id: string
  type: 'insert' | 'delete' | 'update' | 'cursor'
  targetId: string
  userId: string
  data: any
  timestamp: string
  version: number
}

export interface ConflictResolution {
  operationId: string
  resolution: 'local' | 'remote' | 'merged'
  mergedData?: any
}

// ============ 存储键 ============

const SESSION_KEY = 'caelab_collab_session'
const CACHE_KEY = 'caelab_collab_cache'

// ============ 工具函数 ============

function generateId(): string {
  return `collab_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function getAvatarColor(name: string): string {
  const colors = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#F97316']
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return colors[Math.abs(hash) % colors.length]
}

// ============ 主 Composable ============

export function useRealTimeCollaboration() {
  // 当前会话
  const currentSession = ref<CollaborationSession | null>(null)
  const currentUser = ref<Collaborator | null>(null)

  // 实时状态
  const isConnected = ref(false)
  const connectionStatus = ref<'connecting' | 'connected' | 'disconnected' | 'reconnecting'>('disconnected')

  // 操作历史（用于 undo/redo 和冲突检测）
  const operationHistory = ref<ChangeOperation[]>([])
  const pendingOperations = ref<ChangeOperation[]>([])
  const acknowledgedVersion = ref(0)

  // 本地缓存
  const localCache = ref<Map<string, any>>(new Map())

  // 心跳间隔
  let heartbeatInterval: ReturnType<typeof setInterval> | null = null

  // ============ 计算属性 ============

  const onlineCollaborators = computed(() =>
    currentSession.value?.collaborators.filter(c => c.isOnline) || []
  )

  const hasConflicts = computed(() =>
    pendingOperations.value.length > 0
  )

  // ============ 会话管理 ============

  /**
   * 创建新的协作会话
   */
  function createSession(projectId: string): CollaborationSession {
    const userName = localStorage.getItem('caelab_username') || `User_${Math.random().toString(36).substring(2, 6)}`

    const session: CollaborationSession = {
      id: generateId(),
      projectId,
      createdAt: new Date().toISOString(),
      collaborators: [],
      chatMessages: []
    }

    // 添加创建者为第一个协作者
    const creator: Collaborator = {
      id: generateId(),
      name: userName,
      avatarColor: getAvatarColor(userName),
      lastActive: new Date().toISOString(),
      isOnline: true
    }

    session.collaborators.push(creator)
    currentUser.value = creator
    currentSession.value = session

    // 保存到本地
    saveSession(session)

    // 模拟连接
    simulateConnect()

    return session
  }

  /**
   * 加入现有会话
   */
  function joinSession(sessionId: string, userName?: string): boolean {
    // 模拟：从本地存储加载
    const session = getStorage<CollaborationSession>(`${SESSION_KEY}_${sessionId}`)
    if (!session) return false

    const name = userName || localStorage.getItem('caelab_username') || `User_${Math.random().toString(36).substring(2, 6)}`

    // 检查是否已在会话中
    let collaborator = session.collaborators.find(c => c.name === name)
    if (!collaborator) {
      collaborator = {
        id: generateId(),
        name,
        avatarColor: getAvatarColor(name),
        lastActive: new Date().toISOString(),
        isOnline: true
      }
      session.collaborators.push(collaborator)
    } else {
      collaborator.isOnline = true
      collaborator.lastActive = new Date().toISOString()
    }

    currentUser.value = collaborator
    currentSession.value = session

    // 保存更新
    saveSession(session)

    // 模拟连接
    simulateConnect()

    // 广播用户加入
    addSystemMessage(`${name} 加入了会话`)

    return true
  }

  /**
   * 离开会话
   */
  function leaveSession(): void {
    if (!currentSession.value || !currentUser.value) return

    // 标记用户离线
    const collaborator = currentSession.value.collaborators.find(c => c.id === currentUser.value!.id)
    if (collaborator) {
      collaborator.isOnline = false
      collaborator.lastActive = new Date().toISOString()
    }

    // 添加系统消息
    addSystemMessage(`${currentUser.value.name} 离开了会话`)

    // 保存会话
    saveSession(currentSession.value)

    // 清空状态
    currentSession.value = null
    currentUser.value = null
    isConnected.value = false

    // 停止心跳
    if (heartbeatInterval) {
      clearInterval(heartbeatInterval)
      heartbeatInterval = null
    }
  }

  /**
   * 切换会话
   */
  function switchSession(sessionId: string): boolean {
    const session = getStorage<CollaborationSession>(`${SESSION_KEY}_${sessionId}`)
    if (!session) return false

    // 离开当前会话
    if (currentSession.value) {
      leaveSession()
    }

    // 加入新会话
    return joinSession(sessionId)
  }

  // ============ 光标追踪 ============

  /**
   * 更新本地光标位置
   */
  function updateCursor(position: CursorPosition): void {
    if (!currentUser.value) return

    currentUser.value.cursor = position
    currentUser.value.lastActive = new Date().toISOString()

    // 广播光标更新（模拟）
    broadcastOperation({
      id: generateId(),
      type: 'cursor',
      targetId: 'cursor',
      userId: currentUser.value.id,
      data: position,
      timestamp: new Date().toISOString(),
      version: ++acknowledgedVersion.value
    })
  }

  /**
   * 更新其他协作者的光标
   */
  function updateCollaboratorCursor(userId: string, position: CursorPosition): void {
    if (!currentSession.value) return

    const collaborator = currentSession.value.collaborators.find(c => c.id === userId)
    if (collaborator) {
      collaborator.cursor = position
      collaborator.lastActive = new Date().toISOString()
    }
  }

  // ============ 变化操作 ============

  /**
   * 记录本地变化
   */
  function recordChange(type: ChangeOperation['type'], targetId: string, data: any): void {
    if (!currentUser.value) return

    const operation: ChangeOperation = {
      id: generateId(),
      type,
      targetId,
      userId: currentUser.value.id,
      data,
      timestamp: new Date().toISOString(),
      version: ++acknowledgedVersion.value
    }

    // 添加到历史
    operationHistory.value.push(operation)

    // 限制历史长度
    if (operationHistory.value.length > 100) {
      operationHistory.value.splice(0, operationHistory.value.length - 100)
    }

    // 添加到待确认队列
    pendingOperations.value.push(operation)

    // 保存到本地缓存
    localCache.value.set(targetId, data)

    // 模拟广播
    broadcastOperation(operation)

    // 模拟服务器确认
    simulateServerAck(operation.id)
  }

  /**
   * 广播操作到其他协作者
   */
  function broadcastOperation(operation: ChangeOperation): void {
    // 模拟网络延迟后其他协作者收到更新
    setTimeout(() => {
      if (operation.type === 'cursor' && currentSession.value) {
        // 更新其他协作者的光标
        for (const collaborator of currentSession.value.collaborators) {
          if (collaborator.id !== currentUser.value?.id) {
            updateCollaboratorCursor(operation.userId, operation.data)
          }
        }
      }
    }, 50)
  }

  /**
   * 模拟服务器确认
   */
  function simulateServerAck(operationId: string): void {
    setTimeout(() => {
      const index = pendingOperations.value.findIndex(op => op.id === operationId)
      if (index !== -1) {
        pendingOperations.value.splice(index, 1)
      }
    }, 200 + Math.random() * 100)
  }

  /**
   * 应用远程操作
   */
  function applyRemoteOperation(operation: ChangeOperation): void {
    // 更新缓存
    if (operation.type !== 'cursor') {
      localCache.value.set(operation.targetId, operation.data)
    }

    // 更新历史
    const existingIndex = operationHistory.value.findIndex(op => op.id === operation.id)
    if (existingIndex === -1) {
      operationHistory.value.push(operation)
    }
  }

  // ============ 冲突检测与解决 ============

  /**
   * 检测冲突
   */
  function detectConflicts(targetId: string): ChangeOperation[] {
    const targetOps = pendingOperations.value.filter(op => op.targetId === targetId)
    if (targetOps.length <= 1) return []

    // 多个操作针对同一目标 = 冲突
    return targetOps
  }

  /**
   * 自动解决冲突
   */
  function resolveConflict(conflict: ConflictResolution): void {
    switch (conflict.resolution) {
      case 'local':
        // 保留本地，放弃远程
        break
      case 'remote':
        // 应用远程，放弃本地
        if (conflict.mergedData) {
          localCache.value.set(conflict.operationId, conflict.mergedData)
        }
        break
      case 'merged':
        // 使用合并结果
        if (conflict.mergedData) {
          localCache.value.set(conflict.operationId, conflict.mergedData)
        }
        break
    }
  }

  /**
   * 手动解决冲突
   */
  function manualResolve(operationId: string, resolution: 'local' | 'remote', mergedData?: any): void {
    resolveConflict({ operationId, resolution, mergedData })
  }

  // ============ Undo/Redo ============

  /**
   * 撤销本地操作
   */
  function undo(): ChangeOperation | null {
    // 找到最后一个本地操作
    for (let i = operationHistory.value.length - 1; i >= 0; i--) {
      const op = operationHistory.value[i]
      if (op.userId === currentUser.value?.id) {
        operationHistory.value.splice(i, 1)

        // 生成反向操作
        const reverseOp: ChangeOperation = {
          id: generateId(),
          type: op.type === 'insert' ? 'delete' : op.type === 'delete' ? 'insert' : 'update',
          targetId: op.targetId,
          userId: currentUser.value!.id,
          data: op.data, // 这里应该存储原始值
          timestamp: new Date().toISOString(),
          version: ++acknowledgedVersion.value
        }

        // 广播撤销
        broadcastOperation(reverseOp)

        return reverseOp
      }
    }
    return null
  }

  /**
   * 重做操作
   */
  function redo(): ChangeOperation | null {
    // 找到最后一个被撤销的操作
    // 这里简化处理，实际应该维护 separate redo stack
    return null
  }

  // ============ 聊天和注释 ============

  /**
   * 发送消息
   */
  function sendMessage(content: string, type: ChatMessage['type'] = 'message'): ChatMessage | null {
    if (!currentSession.value || !currentUser.value) return null

    const message: ChatMessage = {
      id: generateId(),
      authorId: currentUser.value.id,
      authorName: currentUser.value.name,
      content,
      timestamp: new Date().toISOString(),
      type
    }

    currentSession.value.chatMessages.push(message)
    saveSession(currentSession.value)

    return message
  }

  /**
   * 添加系统消息
   */
  function addSystemMessage(content: string): void {
    sendMessage(content, 'system')
  }

  /**
   * 添加注释（针对特定元素）
   */
  function addAnnotation(targetId: string, content: string): ChatMessage | null {
    const message = sendMessage(`[注释 ${targetId}]: ${content}`, 'annotation')
    return message
  }

  // ============ 连接模拟 ============

  function simulateConnect(): void {
    connectionStatus.value = 'connecting'

    setTimeout(() => {
      isConnected.value = true
      connectionStatus.value = 'connected'

      // 启动心跳
      heartbeatInterval = setInterval(() => {
        if (currentUser.value) {
          currentUser.value.lastActive = new Date().toISOString()
        }

        // 模拟定期同步
        syncWithServer()
      }, 5000)
    }, 500)
  }

  function syncWithServer(): void {
    // 模拟同步
    if (currentSession.value && currentUser.value) {
      // 检查其他协作者是否离线（超过 30 秒无活动）
      for (const collaborator of currentSession.value.collaborators) {
        if (collaborator.id !== currentUser.value.id) {
          const lastActive = new Date(collaborator.lastActive).getTime()
          const now = Date.now()
          if (now - lastActive > 30000) {
            collaborator.isOnline = false
          }
        }
      }
    }
  }

  // ============ 会话持久化 ============

  function saveSession(session: CollaborationSession): void {
    setStorage(`${SESSION_KEY}_${session.id}`, session)
  }

  function getSession(sessionId: string): CollaborationSession | null {
    return getStorage<CollaborationSession>(`${SESSION_KEY}_${sessionId}`)
  }

  function listSessions(): CollaborationSession[] {
    const sessions: CollaborationSession[] = []
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (key?.startsWith(SESSION_KEY)) {
        const session = getStorage<CollaborationSession>(key)
        if (session) sessions.push(session)
      }
    }
    return sessions
  }

  function deleteSession(sessionId: string): boolean {
    localStorage.removeItem(`${SESSION_KEY}_${sessionId}`)
    return true
  }

  // ============ 获取协作者状态 ============

  function getCollaboratorStatus(userId: string): { isOnline: boolean; lastActive: string; cursor?: CursorPosition } | null {
    if (!currentSession.value) return null

    const collaborator = currentSession.value.collaborators.find(c => c.id === userId)
    if (!collaborator) return null

    return {
      isOnline: collaborator.isOnline,
      lastActive: collaborator.lastActive,
      cursor: collaborator.cursor
    }
  }

  // ============ 清理 ============

  function cleanup(): void {
    if (heartbeatInterval) {
      clearInterval(heartbeatInterval)
      heartbeatInterval = null
    }
  }

  onUnmounted(() => {
    cleanup()
  })

  return {
    // 状态
    currentSession,
    currentUser,
    isConnected,
    connectionStatus,
    onlineCollaborators,
    hasConflicts,

    // 操作历史
    operationHistory,
    pendingOperations,

    // 会话管理
    createSession,
    joinSession,
    leaveSession,
    switchSession,
    listSessions,
    deleteSession,

    // 光标追踪
    updateCursor,
    updateCollaboratorCursor,

    // 变化操作
    recordChange,
    applyRemoteOperation,

    // 冲突解决
    detectConflicts,
    resolveConflict,
    manualResolve,

    // Undo/Redo
    undo,
    redo,

    // 聊天
    sendMessage,
    addAnnotation,
    addSystemMessage,

    // 协作者状态
    getCollaboratorStatus
  }
}