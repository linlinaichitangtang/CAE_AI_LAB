/**
 * HPC Cluster Management API - HPC 集群管理
 * V1.4-006: 节点管理、队列管理、集群监控、告警处理
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export interface ClusterNode {
  id: string
  hostname: string
  ip: string
  status: 'online' | 'offline' | 'busy' | 'idle' | 'maintenance'
  cpu_model: string
  cpu_cores: number
  cpu_usage: number // 0-100
  memory_total_gb: number
  memory_used_gb: number
  gpu_count: number
  gpu_model: string
  gpu_usage: number // 0-100
  disk_total_gb: number
  disk_used_gb: number
  network_bandwidth_gbps: number
  temperature: number
  power_consumption_w: number
  jobs_running: string[]
  last_heartbeat: string
}

export interface QueueJob {
  id: string
  name: string
  user: string
  priority: number
  submit_time: string
  estimated_start: string
  wall_time_requested: number
  status: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled'
  dependencies: string[]
}

export interface ClusterMetrics {
  cpu_utilization: number
  memory_utilization: number
  gpu_utilization: number
  disk_io_read_mbps: number
  disk_io_write_mbps: number
  network_in_mbps: number
  network_out_mbps: number
  jobs_completed_today: number
  jobs_failed_today: number
  avg_queue_wait_minutes: number
}

export interface ClusterAlert {
  id: string
  severity: 'info' | 'warning' | 'critical'
  message: string
  node_id: string
  timestamp: string
  acknowledged: boolean
}

// ============ API 函数 ============

/** 获取集群所有节点列表 */
export async function getClusterNodes(): Promise<ClusterNode[]> {
  return await invoke<ClusterNode[]>('get_cluster_nodes')
}

/** 获取集群资源指标 */
export async function getClusterMetrics(): Promise<ClusterMetrics> {
  return await invoke<ClusterMetrics>('get_cluster_metrics')
}

/** 获取队列中的作业列表 */
export async function getQueueJobs(): Promise<QueueJob[]> {
  return await invoke<QueueJob[]>('get_queue_jobs')
}

/** 获取单个节点详情 */
export async function getNodeDetail(nodeId: string): Promise<ClusterNode> {
  return await invoke<ClusterNode>('get_node_detail', { nodeId })
}

/** 获取集群告警列表 */
export async function getClusterAlerts(): Promise<ClusterAlert[]> {
  return await invoke<ClusterAlert[]>('get_cluster_alerts')
}

/** 确认告警 */
export async function acknowledgeAlert(alertId: string): Promise<boolean> {
  return await invoke<boolean>('acknowledge_alert', { alertId })
}

/** 重启节点 */
export async function restartNode(nodeId: string): Promise<boolean> {
  return await invoke<boolean>('restart_node', { nodeId })
}

/** 排空节点 (不再分配新任务) */
export async function drainNode(nodeId: string): Promise<boolean> {
  return await invoke<boolean>('drain_node', { nodeId })
}
