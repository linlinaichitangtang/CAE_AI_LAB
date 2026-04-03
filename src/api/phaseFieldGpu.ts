/**
 * Phase Field GPU Acceleration API - 相场 GPU 加速
 * V1.6-007: GPU 设备检测、基准测试、求解器配置、性能分析
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type GpuBackend = 'cuda' | 'opencl' | 'webgpu' | 'wasm_simd'

export interface GpuDeviceInfo {
  name: string
  vendor: string
  compute_capability: string
  total_memory_mb: number
  available_memory_mb: number
  max_threads_per_block: number
  max_grid_size: { x: number; y: number }
  clock_speed_mhz: number
  driver_version: string
}

export interface GpuBenchmarkResult {
  backend: GpuBackend
  grid_size: { nx: number; ny: number }
  steps_per_second: number
  memory_used_mb: number
  speedup_vs_cpu: number
  energy_consumption_mj: number
  temperature_C: number
}

export interface GpuSolverConfig {
  backend: GpuBackend
  block_size: { x: number; y: number }
  use_shared_memory: boolean
  use_texture_memory: boolean
  precision: 'float32' | 'float64'
  async_compute: boolean
  device_id: number
}

export interface GpuPerformanceProfile {
  kernel_time_ms: number
  memory_transfer_ms: number
  total_time_ms: number
  occupancy: number
  cache_hit_rate: number
  memory_bandwidth_gb_s: number
}

// ============ API 函数 ============

/** 检测系统中可用的 GPU 设备 */
export async function detectGpuDevices(): Promise<GpuDeviceInfo[]> {
  return await invoke<GpuDeviceInfo[]>('detect_gpu_devices')
}

/** 运行 GPU 基准测试 */
export async function runGpuBenchmark(
  gridSize: { nx: number; ny: number },
  steps: number
): Promise<GpuBenchmarkResult[]> {
  return await invoke<GpuBenchmarkResult[]>('run_gpu_benchmark', { gridSize, steps })
}

/** 配置 GPU 求解器参数 */
export async function configureGpuSolver(
  config: GpuSolverConfig
): Promise<{ success: boolean; warnings: string[] }> {
  return await invoke<{ success: boolean; warnings: string[] }>('configure_gpu_solver', { config })
}

/** 获取 GPU 性能分析数据 */
export async function getGpuPerformanceProfile(): Promise<GpuPerformanceProfile> {
  return await invoke<GpuPerformanceProfile>('get_gpu_performance_profile')
}

/** 估算 GPU 计算性能 */
export async function estimateGpuPerformance(
  gridSize: { nx: number; ny: number },
  steps: number
): Promise<{
  estimated_time_s: number
  memory_required_mb: number
  recommended_block_size: { x: number; y: number }
}> {
  return await invoke<{
    estimated_time_s: number
    memory_required_mb: number
    recommended_block_size: { x: number; y: number }
  }>('estimate_gpu_performance', { gridSize, steps })
}
