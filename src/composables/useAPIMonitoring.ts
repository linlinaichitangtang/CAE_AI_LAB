/**
 * CAELab V3.5-006: API Monitoring Dashboard
 * CI/CD 集成状态监控，错误告警
 */
import { ref, computed } from 'vue'

export type AlertLevel = 'info' | 'warning' | 'error' | 'critical'
export type MetricType = 'response_time' | 'error_rate' | 'requests_per_min' | 'cpu_usage' | 'memory_usage'

export interface APIEndpoint {
  id: string
  name: string
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  path: string
  description: string
}

export interface MetricDataPoint {
  timestamp: number
  value: number
}

export interface Alert {
  id: string
  level: AlertLevel
  message: string
  endpoint?: string
  timestamp: Date
  acknowledged: boolean
  resolved: boolean
}

export interface EndpointHealth {
  endpoint: APIEndpoint
  status: 'healthy' | 'degraded' | 'down'
  uptime: number
  avgResponseTime: number
  errorRate: number
  requestsLastHour: number
  metrics: Record<MetricType, MetricDataPoint[]>
}

const isMonitoring = ref(false)
const endpoints = ref<APIEndpoint[]>([])
const endpointHealth = ref<Map<string, EndpointHealth>>(new Map())
const alerts = ref<Alert[]>([])
const metricsHistory = ref<Map<string, MetricDataPoint[]>>(new Map())
const refreshInterval = ref(30) // seconds
const retentionPeriod = ref(24) // hours
const alertThresholds = ref({
  responseTime: 1000, // ms
  errorRate: 5, // percent
  cpuUsage: 80, // percent
  memoryUsage: 85 // percent
})

const healthCheckStatuses = ref<Map<string, 'up' | 'down' | 'unknown'>>(new Map())
const lastHealthCheck = ref<Map<string, Date>>(new Map())

export function useAPIMonitoring() {
  const addEndpoint = (endpoint: Omit<APIEndpoint, 'id'>) => {
    const newEndpoint: APIEndpoint = {
      id: `ep_${Date.now()}`,
      ...endpoint
    }
    endpoints.value.push(newEndpoint)

    // Initialize health tracking
    endpointHealth.value.set(newEndpoint.id, {
      endpoint: newEndpoint,
      status: 'healthy',
      uptime: 100,
      avgResponseTime: 0,
      errorRate: 0,
      requestsLastHour: 0,
      metrics: {
        response_time: [],
        error_rate: [],
        requests_per_min: [],
        cpu_usage: [],
        memory_usage: []
      }
    })

    return newEndpoint
  }

  const removeEndpoint = (id: string) => {
    endpoints.value = endpoints.value.filter(e => e.id !== id)
    endpointHealth.value.delete(id)
    metricsHistory.value.delete(id)
  }

  const startMonitoring = () => {
    isMonitoring.value = true

    // Simulate initial health checks
    endpoints.value.forEach(ep => {
      performHealthCheck(ep.id)
    })
  }

  const stopMonitoring = () => {
    isMonitoring.value = false
  }

  const performHealthCheck = (endpointId: string) => {
    const health = endpointHealth.value.get(endpointId)
    if (!health) return

    // Simulate health check
    const isUp = Math.random() > 0.05 // 95% uptime simulation
    healthCheckStatuses.value.set(endpointId, isUp ? 'up' : 'down')
    lastHealthCheck.value.set(endpointId, new Date())

    if (!isUp) {
      health.status = 'down'
      addAlert('error', `Endpoint ${health.endpoint.name} is down`, health.endpoint.name)
    } else {
      // Simulate response time
      const responseTime = Math.floor(Math.random() * 500) + 50
      health.avgResponseTime = responseTime
      health.errorRate = Math.random() * 2

      if (responseTime > alertThresholds.value.responseTime) {
        addAlert('warning', `Slow response time: ${responseTime}ms`, health.endpoint.name)
      }

      if (health.errorRate > alertThresholds.value.errorRate) {
        addAlert('error', `High error rate: ${health.errorRate.toFixed(2)}%`, health.endpoint.name)
      }
    }
  }

  const recordMetric = (endpointId: string, type: MetricType, value: number) => {
    const dataPoint: MetricDataPoint = {
      timestamp: Date.now(),
      value
    }

    // Store in history
    const key = `${endpointId}_${type}`
    if (!metricsHistory.value.has(key)) {
      metricsHistory.value.set(key, [])
    }
    metricsHistory.value.get(key)!.push(dataPoint)

    // Update current health
    const health = endpointHealth.value.get(endpointId)
    if (health) {
      health.metrics[type].push(dataPoint)

      // Keep only recent data
      const cutoff = Date.now() - retentionPeriod.value * 60 * 60 * 1000
      health.metrics[type] = health.metrics[type].filter(m => m.timestamp > cutoff)
    }
  }

  const addAlert = (level: AlertLevel, message: string, endpoint?: string) => {
    const alert: Alert = {
      id: `alert_${Date.now()}`,
      level,
      message,
      endpoint,
      timestamp: new Date(),
      acknowledged: false,
      resolved: false
    }
    alerts.value.unshift(alert)

    // Auto-acknowledge info level
    if (level === 'info') {
      alert.acknowledged = true
    }

    return alert
  }

  const acknowledgeAlert = (alertId: string) => {
    const alert = alerts.value.find(a => a.id === alertId)
    if (alert) {
      alert.acknowledged = true
    }
  }

  const resolveAlert = (alertId: string) => {
    const alert = alerts.value.find(a => a.id === alertId)
    if (alert) {
      alert.resolved = true
      alert.acknowledged = true
    }
  }

  const dismissAlert = (alertId: string) => {
    alerts.value = alerts.value.filter(a => a.id !== alertId)
  }

  const clearResolvedAlerts = () => {
    alerts.value = alerts.value.filter(a => !a.resolved)
  }

  const setRefreshInterval = (seconds: number) => {
    refreshInterval.value = seconds
  }

  const setAlertThreshold = (type: 'responseTime' | 'errorRate' | 'cpuUsage' | 'memoryUsage', value: number) => {
    alertThresholds.value[type] = value
  }

  const getEndpointHealth = (endpointId: string): EndpointHealth | null => {
    return endpointHealth.value.get(endpointId) || null
  }

  const getMetricHistory = (endpointId: string, type: MetricType): MetricDataPoint[] => {
    const key = `${endpointId}_${type}`
    return metricsHistory.value.get(key) || []
  }

  const getAlertSummary = computed(() => {
    const unacknowledged = alerts.value.filter(a => !a.acknowledged && !a.resolved).length
    const critical = alerts.value.filter(a => a.level === 'critical' && !a.resolved).length
    const resolved = alerts.value.filter(a => a.resolved).length

    return {
      total: alerts.value.length,
      unacknowledged,
      critical,
      resolved,
      byLevel: {
        info: alerts.value.filter(a => a.level === 'info').length,
        warning: alerts.value.filter(a => a.level === 'warning').length,
        error: alerts.value.filter(a => a.level === 'error').length,
        critical: alerts.value.filter(a => a.level === 'critical').length
      }
    }
  })

  const systemHealthSummary = computed(() => {
    const endpoints_list = Array.from(endpointHealth.value.values())
    const healthy = endpoints_list.filter(e => e.status === 'healthy').length
    const degraded = endpoints_list.filter(e => e.status === 'degraded').length
    const down = endpoints_list.filter(e => e.status === 'down').length

    return {
      total: endpoints_list.length,
      healthy,
      degraded,
      down,
      overallStatus: down > 0 ? 'down' : degraded > 0 ? 'degraded' : 'healthy' as 'healthy' | 'degraded' | 'down'
    }
  })

  const activeAlerts = computed(() =>
    alerts.value.filter(a => !a.resolved)
  )

  const unacknowledgedAlerts = computed(() =>
    alerts.value.filter(a => !a.acknowledged && !a.resolved)
  )

  return {
    // State
    isMonitoring: computed(() => isMonitoring.value),
    endpoints: computed(() => endpoints.value),
    alerts: computed(() => alerts.value),
    metricsHistory: computed(() => metricsHistory.value),
    refreshInterval: computed(() => refreshInterval.value),
    alertThresholds: computed(() => alertThresholds.value),
    alertSummary: getAlertSummary,
    systemHealthSummary,
    activeAlerts,
    unacknowledgedAlerts,

    // Methods
    addEndpoint,
    removeEndpoint,
    startMonitoring,
    stopMonitoring,
    performHealthCheck,
    recordMetric,
    addAlert,
    acknowledgeAlert,
    resolveAlert,
    dismissAlert,
    clearResolvedAlerts,
    setRefreshInterval,
    setAlertThreshold,
    getEndpointHealth,
    getMetricHistory
  }
}