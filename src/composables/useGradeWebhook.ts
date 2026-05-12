/**
 * useGradeWebhook.ts — V3.3-005 成绩 Webhook
 * LMS 成绩实时推送，作业提交自动同步
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type LMSProvider = 'canvas' | 'blackboard' | 'moodle' | 'brightspace' | 'custom'

export interface LMSConfig {
  id: string
  name: string
  provider: LMSProvider
  baseUrl: string
  apiToken: string
  courseId?: string
  enabled: boolean
  autoSync: boolean
  syncInterval: number  // 分钟
  lastSyncAt?: string
  webhookSecret?: string
}

export interface WebhookEvent {
  id: string
  type: 'submission' | 'grade' | 'feedback' | 'comment'
  assignmentId: string
  studentId: string
  timestamp: string
  payload: Record<string, any>
  retryCount: number
  status: 'pending' | 'sent' | 'failed' | 'verified'
  response?: {
    status: number
    body?: string
    timestamp: string
  }
  errorMessage?: string
}

export interface GradePayload {
  studentId: string
  studentEmail: string
  assignmentId: string
  assignmentName: string
  score: number
  maxScore: number
  percentage: number
  submittedAt: string
  gradedAt: string
  feedback?: string
  gradedBy?: string
}

export interface WebhookLog {
  id: string
  eventId: string
  timestamp: string
  direction: 'outbound' | 'inbound'
  status: 'success' | 'failure' | 'pending'
  requestBody?: string
  responseBody?: string
  statusCode?: number
  errorMessage?: string
}

export interface LMSSyncStatus {
  provider: LMSProvider
  lastSyncAt?: string
  totalSynced: number
  pendingCount: number
  failedCount: number
  successRate: number
}

// ============ 存储键 ============

const LMS_CONFIGS_KEY = 'caelab_lms_configs'
const WEBHOOK_EVENTS_KEY = 'caelab_webhook_events'
const WEBHOOK_LOGS_KEY = 'caelab_webhook_logs'

// ============ 工具函数 ============

function generateId(): string {
  return `webhook_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function generateSignature(payload: string, secret: string): string {
  // 简单的 HMAC-like 签名
  let hash = 0
  const combined = payload + secret
  for (let i = 0; i < combined.length; i++) {
    const char = combined.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash
  }
  return Math.abs(hash).toString(16).padStart(8, '0')
}

// ============ LMS API 封装 ============

export class LMSWebhookClient {
  private configs: Map<LMSProvider, {
    baseUrl: string
    apiToken: string
    headers: Record<string, string>
  }> = new Map()

  constructor() {
    this.initProviders()
  }

  private initProviders(): void {
    // Canvas LMS API 配置
    this.configs.set('canvas', {
      baseUrl: '',
      apiToken: '',
      headers: {
        'Authorization': 'Bearer {token}',
        'Content-Type': 'application/json'
      }
    })

    // Blackboard LMS API 配置
    this.configs.set('blackboard', {
      baseUrl: '',
      apiToken: '',
      headers: {
        'Authorization': 'Bearer {token}',
        'Content-Type': 'application/json'
      }
    })

    // Moodle API 配置
    this.configs.set('moodle', {
      baseUrl: '',
      apiToken: '',
      headers: {
        'Content-Type': 'application/json'
      }
    })
  }

  configure(config: LMSConfig): void {
    const providerConfig = this.configs.get(config.provider)
    if (providerConfig) {
      providerConfig.baseUrl = config.baseUrl
      providerConfig.apiToken = config.apiToken
      providerConfig.headers['Authorization'] = `Bearer ${config.apiToken}`
    }
  }

  // Canvas LMS
  async submitGradeCanvas(
    courseId: string,
    assignmentId: string,
    studentId: string,
    grade: GradePayload,
    config: LMSConfig
  ): Promise<{ success: boolean; error?: string }> {
    const url = `${config.baseUrl}/api/v1/courses/${courseId}/assignments/${assignmentId}/submissions/${studentId}`

    const body = {
      submission: {
        posted_grade: grade.percentage >= 0 ? `${grade.score}/${grade.maxScore}` : undefined,
        body: grade.feedback
      }
    }

    try {
      const response = await fetch(url, {
        method: 'PUT',
        headers: {
          'Authorization': `Bearer ${config.apiToken}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(body)
      })

      if (!response.ok) {
        return { success: false, error: `HTTP ${response.status}: ${response.statusText}` }
      }

      return { success: true }
    } catch (e: any) {
      return { success: false, error: e.message }
    }
  }

  // Blackboard LMS
  async submitGradeBlackboard(
    courseId: string,
    assignmentId: string,
    studentId: string,
    grade: GradePayload,
    config: LMSConfig
  ): Promise<{ success: boolean; error?: string }> {
    const url = `${config.baseUrl}/learn/api/public/v1/courses/${courseId}/gradebook/assignments/${assignmentId}/submissions/${studentId}`

    const body = {
      status: 'GRADED',
      grade: {
        scored: grade.score,
        possible: grade.maxScore
      },
      text: grade.feedback
    }

    try {
      const response = await fetch(url, {
        method: 'PATCH',
        headers: {
          'Authorization': `Bearer ${config.apiToken}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(body)
      })

      if (!response.ok) {
        return { success: false, error: `HTTP ${response.status}: ${response.statusText}` }
      }

      return { success: true }
    } catch (e: any) {
      return { success: false, error: e.message }
    }
  }

  // Moodle Webhook
  async submitGradeMoodle(
    webserviceUrl: string,
    assignmentId: string,
    studentId: string,
    grade: GradePayload,
    config: LMSConfig
  ): Promise<{ success: boolean; error?: string }> {
    // Moodle 使用 REST API
    const url = `${config.baseUrl}/webservice/rest/server.php`

    const params = new URLSearchParams({
      wstoken: config.apiToken,
      wsfunction: 'local_wstemplate_submit_grade',
      moodlewsrestformat: 'json',
      assignmentid: assignmentId,
      userid: studentId,
      grade: grade.score.toString(),
      feedback: grade.feedback || ''
    })

    try {
      const response = await fetch(`${url}?${params}`)
      const data = await response.json()

      if (data.exception) {
        return { success: false, error: data.message || 'Moodle API error' }
      }

      return { success: true }
    } catch (e: any) {
      return { success: false, error: e.message }
    }
  }

  // Brightspace API
  async submitGradeBrightspace(
    orgUnitId: string,
    assignmentId: string,
    studentId: string,
    grade: GradePayload,
    config: LMSConfig
  ): Promise<{ success: boolean; error?: string }> {
    const url = `${config.baseUrl}/api/grades/Organizations(${orgUnitId})/assignments(${assignmentId})/values`

    const body = {
      StudentId: studentId,
      PointsNumerator: grade.score,
      PointsDenominator: grade.maxScore,
      ReturnOnly: false
    }

    try {
      const response = await fetch(url, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${config.apiToken}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(body)
      })

      if (!response.ok) {
        return { success: false, error: `HTTP ${response.status}: ${response.statusText}` }
      }

      return { success: true }
    } catch (e: any) {
      return { success: false, error: e.message }
    }
  }

  // 通用 webhook
  async sendWebhook(
    url: string,
    payload: GradePayload,
    secret?: string
  ): Promise<{ success: boolean; statusCode?: number; error?: string }> {
    const body = JSON.stringify(payload)
    const signature = secret ? generateSignature(body, secret) : undefined

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'X-CAELab-Event': 'grade'
    }

    if (signature) {
      headers['X-CAELab-Signature'] = signature
    }

    try {
      const response = await fetch(url, {
        method: 'POST',
        headers,
        body
      })

      return {
        success: response.ok,
        statusCode: response.status
      }
    } catch (e: any) {
      return { success: false, error: e.message }
    }
  }
}

// ============ 主 Composable ============

export function useGradeWebhook() {
  const configs = ref<LMSConfig[]>([])
  const webhookEvents = ref<WebhookEvent[]>([])
  const webhookLogs = ref<WebhookLog[]>([])

  const client = new LMSWebhookClient()

  // 自动同步定时器
  let syncIntervalId: ReturnType<typeof setInterval> | null = null

  // ============ 计算属性 ============

  const enabledConfigs = computed(() =>
    configs.value.filter(c => c.enabled)
  )

  const pendingEvents = computed(() =>
    webhookEvents.value.filter(e => e.status === 'pending')
  )

  const failedEvents = computed(() =>
    webhookEvents.value.filter(e => e.status === 'failed')
  )

  const syncStatus = computed((): Record<LMSProvider, LMSSyncStatus> => {
    const result: Record<string, LMSSyncStatus> = {
      canvas: { provider: 'canvas', totalSynced: 0, pendingCount: 0, failedCount: 0, successRate: 100 },
      blackboard: { provider: 'blackboard', totalSynced: 0, pendingCount: 0, failedCount: 0, successRate: 100 },
      moodle: { provider: 'moodle', totalSynced: 0, pendingCount: 0, failedCount: 0, successRate: 100 },
      brightspace: { provider: 'brightspace', totalSynced: 0, pendingCount: 0, failedCount: 0, successRate: 100 },
      custom: { provider: 'custom', totalSynced: 0, pendingCount: 0, failedCount: 0, successRate: 100 }
    }

    for (const event of webhookEvents.value) {
      const config = configs.value.find(c => c.id === event.payload.configId)
      if (config) {
        const status = result[config.provider]
        status.totalSynced++

        if (event.status === 'pending') status.pendingCount++
        if (event.status === 'failed') status.failedCount++
      }
    }

    for (const provider of Object.keys(result) as LMSProvider[]) {
      const s = result[provider]
      if (s.totalSynced > 0) {
        s.successRate = Math.round(((s.totalSynced - s.failedCount) / s.totalSynced) * 100)
      }
    }

    return result as Record<LMSProvider, LMSSyncStatus>
  })

  // ============ 初始化 ============

  function loadData(): void {
    const storedConfigs = getStorage<LMSConfig[]>(LMS_CONFIGS_KEY)
    if (storedConfigs) configs.value = storedConfigs

    const storedEvents = getStorage<WebhookEvent[]>(WEBHOOK_EVENTS_KEY)
    if (storedEvents) webhookEvents.value = storedEvents

    const storedLogs = getStorage<WebhookLog[]>(WEBHOOK_LOGS_KEY)
    if (storedLogs) webhookLogs.value = storedLogs

    // 初始化客户端配置
    for (const config of configs.value) {
      client.configure(config)
    }

    // 启动自动同步
    startAutoSync()
  }

  // ============ LMS 配置管理 ============

  function addConfig(config: Omit<LMSConfig, 'id'>): LMSConfig {
    const newConfig: LMSConfig = {
      ...config,
      id: generateId()
    }
    configs.value.push(newConfig)
    client.configure(newConfig)
    saveData()
    return newConfig
  }

  function updateConfig(configId: string, updates: Partial<LMSConfig>): boolean {
    const config = configs.value.find(c => c.id === configId)
    if (!config) return false

    Object.assign(config, updates)
    client.configure(config)
    saveData()
    return true
  }

  function deleteConfig(configId: string): boolean {
    const index = configs.value.findIndex(c => c.id === configId)
    if (index === -1) return false

    configs.value.splice(index, 1)
    saveData()
    return true
  }

  function getConfig(configId: string): LMSConfig | undefined {
    return configs.value.find(c => c.id === configId)
  }

  function toggleConfig(configId: string): boolean {
    const config = configs.value.find(c => c.id === configId)
    if (!config) return false

    config.enabled = !config.enabled
    saveData()
    return config.enabled
  }

  // ============ Webhook 事件 ============

  /**
   * 创建成绩推送事件
   */
  function createGradeEvent(
    configId: string,
    payload: Omit<GradePayload, 'gradedAt' | 'gradedBy'> & { configId: string }
  ): WebhookEvent {
    const event: WebhookEvent = {
      id: generateId(),
      type: 'grade',
      assignmentId: payload.assignmentId,
      studentId: payload.studentId,
      timestamp: new Date().toISOString(),
      payload: payload as any,
      retryCount: 0,
      status: 'pending'
    }

    webhookEvents.value.unshift(event)
    saveData()

    return event
  }

  /**
   * 创建提交事件
   */
  function createSubmissionEvent(
    configId: string,
    payload: {
      assignmentId: string
      assignmentName: string
      studentId: string
      studentEmail: string
      submittedAt: string
    }
  ): WebhookEvent {
    const event: WebhookEvent = {
      id: generateId(),
      type: 'submission',
      assignmentId: payload.assignmentId,
      studentId: payload.studentId,
      timestamp: new Date().toISOString(),
      payload: payload as any,
      retryCount: 0,
      status: 'pending'
    }

    webhookEvents.value.unshift(event)
    saveData()

    return event
  }

  /**
   * 发送 Webhook
   */
  async function sendWebhook(eventId: string): Promise<boolean> {
    const event = webhookEvents.value.find(e => e.id === eventId)
    if (!event) return false

    const config = configs.value.find(c => c.id === event.payload.configId)
    if (!config || !config.enabled) return false

    const gradePayload: GradePayload = {
      studentId: event.payload.studentId,
      studentEmail: event.payload.studentEmail,
      assignmentId: event.payload.assignmentId,
      assignmentName: event.payload.assignmentName,
      score: event.payload.score,
      maxScore: event.payload.maxScore,
      percentage: (event.payload.score / event.payload.maxScore) * 100,
      submittedAt: event.payload.submittedAt,
      gradedAt: new Date().toISOString(),
      feedback: event.payload.feedback
    }

    let result: { success: boolean; statusCode?: number; error?: string }

    // 根据 provider 类型发送
    if (config.provider === 'custom') {
      // 使用通用 webhook
      result = await client.sendWebhook(
        config.baseUrl,
        gradePayload,
        config.webhookSecret
      )
    } else {
      // LMS 特定 API
      switch (config.provider) {
        case 'canvas':
          result = await client.submitGradeCanvas(
            config.courseId || '',
            event.payload.assignmentId,
            event.payload.studentId,
            gradePayload,
            config
          )
          break
        case 'blackboard':
          result = await client.submitGradeBlackboard(
            config.courseId || '',
            event.payload.assignmentId,
            event.payload.studentId,
            gradePayload,
            config
          )
          break
        case 'moodle':
          result = await client.submitGradeMoodle(
            config.baseUrl,
            event.payload.assignmentId,
            event.payload.studentId,
            gradePayload,
            config
          )
          break
        case 'brightspace':
          result = await client.submitGradeBrightspace(
            config.courseId || '',
            event.payload.assignmentId,
            event.payload.studentId,
            gradePayload,
            config
          )
          break
        default:
          result = { success: false, error: 'Unknown provider' }
      }
    }

    // 更新事件状态
    event.response = {
      status: result.statusCode || 0,
      body: result.error,
      timestamp: new Date().toISOString()
    }

    if (result.success) {
      event.status = 'sent'
    } else {
      event.status = 'failed'
      event.errorMessage = result.error
    }

    // 记录日志
    addLog({
      eventId: event.id,
      direction: 'outbound',
      status: result.success ? 'success' : 'failure',
      requestBody: JSON.stringify(gradePayload),
      responseBody: result.error,
      statusCode: result.statusCode,
      errorMessage: result.error
    })

    saveData()
    return result.success
  }

  /**
   * 重试失败的事件
   */
  async function retryEvent(eventId: string): Promise<boolean> {
    const event = webhookEvents.value.find(e => e.id === eventId)
    if (!event || event.retryCount >= 3) return false

    event.retryCount++
    return sendWebhook(eventId)
  }

  /**
   * 批量发送待处理事件
   */
  async function sendPendingEvents(): Promise<{ success: number; failed: number }> {
    const pending = webhookEvents.value.filter(e => e.status === 'pending')
    let success = 0
    let failed = 0

    for (const event of pending) {
      const result = await sendWebhook(event.id)
      if (result) success++
      else failed++
    }

    return { success, failed }
  }

  // ============ 日志 ============

  function addLog(log: Omit<WebhookLog, 'id' | 'timestamp'>): void {
    const webhookLog: WebhookLog = {
      ...log,
      id: generateId(),
      timestamp: new Date().toISOString()
    }
    webhookLogs.value.unshift(webhookLog)

    // 只保留最近 1000 条日志
    if (webhookLogs.value.length > 1000) {
      webhookLogs.value = webhookLogs.value.slice(0, 1000)
    }

    saveData()
  }

  function getLogs(options?: {
    eventId?: string
    direction?: 'outbound' | 'inbound'
    status?: 'success' | 'failure' | 'pending'
    limit?: number
  }): WebhookLog[] {
    let result = webhookLogs.value

    if (options?.eventId) {
      result = result.filter(l => l.eventId === options.eventId)
    }
    if (options?.direction) {
      result = result.filter(l => l.direction === options.direction)
    }
    if (options?.status) {
      result = result.filter(l => l.status === options.status)
    }

    return result.slice(0, options?.limit || 100)
  }

  function clearLogs(): void {
    webhookLogs.value = []
    saveData()
  }

  // ============ 自动同步 ============

  function startAutoSync(): void {
    if (syncIntervalId) {
      clearInterval(syncIntervalId)
    }

    const enabledConfigsWithSync = enabledConfigs.value.filter(c => c.autoSync && c.syncInterval > 0)

    if (enabledConfigsWithSync.length === 0) return

    // 使用最短间隔
    const minInterval = Math.min(...enabledConfigsWithSync.map(c => c.syncInterval))
    const intervalMs = minInterval * 60 * 1000

    syncIntervalId = setInterval(async () => {
      await sendPendingEvents()

      // 更新最后同步时间
      const now = new Date().toISOString()
      for (const config of enabledConfigsWithSync) {
        config.lastSyncAt = now
      }
      saveData()
    }, intervalMs)
  }

  function stopAutoSync(): void {
    if (syncIntervalId) {
      clearInterval(syncIntervalId)
      syncIntervalId = null
    }
  }

  /**
   * 手动触发同步
   */
  async function triggerSync(configId?: string): Promise<{ success: number; failed: number }> {
    if (configId) {
      const config = configs.value.find(c => c.id === configId)
      if (config) {
        config.lastSyncAt = new Date().toISOString()
        saveData()
      }
    }

    return sendPendingEvents()
  }

  // ============ Webhook 验证 ============

  /**
   * 验证 webhook 签名
   */
  function verifySignature(payload: string, signature: string, secret: string): boolean {
    const expected = generateSignature(payload, secret)
    return signature === expected
  }

  /**
   * 模拟接收 webhook
   */
  async function receiveWebhook(
    configId: string,
    body: Record<string, any>
  ): Promise<{ success: boolean; event?: WebhookEvent }> {
    const config = configs.value.find(c => c.id === configId)
    if (!config) return { success: false }

    const event: WebhookEvent = {
      id: generateId(),
      type: 'grade',
      assignmentId: body.assignmentId || '',
      studentId: body.studentId || '',
      timestamp: new Date().toISOString(),
      payload: body,
      retryCount: 0,
      status: 'verified'
    }

    webhookEvents.value.unshift(event)
    addLog({
      eventId: event.id,
      direction: 'inbound',
      status: 'success',
      requestBody: JSON.stringify(body)
    })

    saveData()
    return { success: true, event }
  }

  // ============ 工具方法 ============

  /**
   * 测试 LMS 连接
   */
  async function testConnection(configId: string): Promise<{ success: boolean; message: string }> {
    const config = configs.value.find(c => c.id === configId)
    if (!config) return { success: false, message: 'Configuration not found' }

    try {
      // 发送测试请求
      const response = await fetch(config.baseUrl, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${config.apiToken}`,
          'Content-Type': 'application/json'
        }
      })

      if (response.ok) {
        return { success: true, message: 'Connection successful' }
      } else {
        return { success: false, message: `HTTP ${response.status}: ${response.statusText}` }
      }
    } catch (e: any) {
      return { success: false, message: e.message }
    }
  }

  /**
   * 获取统计数据
   */
  function getStats(): {
    totalConfigs: number
    enabledConfigs: number
    totalEvents: number
    pendingEvents: number
    sentEvents: number
    failedEvents: number
    averageRetryCount: number
  } {
    const totalEvents = webhookEvents.value.length
    const sentEvents = webhookEvents.value.filter(e => e.status === 'sent').length
    const failedEvents = webhookEvents.value.filter(e => e.status === 'failed').length

    return {
      totalConfigs: configs.value.length,
      enabledConfigs: enabledConfigs.value.length,
      totalEvents,
      pendingEvents: pendingEvents.value.length,
      sentEvents,
      failedEvents,
      averageRetryCount: totalEvents > 0
        ? webhookEvents.value.reduce((sum, e) => sum + e.retryCount, 0) / totalEvents
        : 0
    }
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(LMS_CONFIGS_KEY, configs.value)
    setStorage(WEBHOOK_EVENTS_KEY, webhookEvents.value)
    setStorage(WEBHOOK_LOGS_KEY, webhookLogs.value)
  }

  function clearAllData(): void {
    configs.value = []
    webhookEvents.value = []
    webhookLogs.value = []
    stopAutoSync()
    saveData()
  }

  // 初始化
  loadData()

  return {
    // 状态
    configs,
    webhookEvents,
    webhookLogs,
    enabledConfigs,
    pendingEvents,
    failedEvents,
    syncStatus,

    // 配置管理
    addConfig,
    updateConfig,
    deleteConfig,
    getConfig,
    toggleConfig,

    // 事件管理
    createGradeEvent,
    createSubmissionEvent,
    sendWebhook,
    retryEvent,
    sendPendingEvents,

    // 日志
    getLogs,
    clearLogs,

    // 同步
    startAutoSync,
    stopAutoSync,
    triggerSync,

    // 验证
    verifySignature,
    receiveWebhook,
    testConnection,

    // 统计
    getStats,

    // 客户端
    client,

    // 持久化
    clearAllData
  }
}
