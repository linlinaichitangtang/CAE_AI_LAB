/**
 * Trajectory Viewer API
 * V1.5-006: 轨迹可视化 API 封装
 * V1.5-008: 应力/能量时序曲线 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 着色模式 */
export type ColoringMode =
  | 'element'
  | 'velocity'
  | 'stress'
  | 'displacement'
  | 'type'
  | 'charge'
  | 'kinetic_energy'
  | 'potential_energy'

/** 键判据 */
export type BondCriterion = 'distance' | 'angle' | 'order_parameter'

/** 原子数据 */
export interface AtomData {
  id: number
  element: string
  x: number
  y: number
  z: number
  vx: number
  vy: number
  vz: number
  fx: number
  fy: number
  fz: number
}

/** 模拟盒子 */
export interface SimBox {
  lx: number
  ly: number
  lz: number
}

/** 轨迹帧 */
export interface TrajectoryFrame {
  frame_number: number
  num_atoms: number
  atoms: AtomData[]
  box: SimBox
  time_fs: number
  temperature: number
  total_energy: number
  kinetic_energy: number
  potential_energy: number
  pressure: number
  volume: number
}

/** 帧范围 */
export interface FrameRange {
  start: number
  end: number
  step: number
}

/** 轨迹配置 */
export interface TrajectoryConfig {
  dump_file_path: string
  format: 'lammps_dump' | 'xyz' | 'pdb' | 'dcd'
  num_frames: number
  frame_range: FrameRange
  coloring_mode: ColoringMode
  bond_criterion: BondCriterion
  bond_cutoff_A: number
  show_bonds: boolean
  atom_scale: number
  box_display: boolean
}

/** 轨迹结果 */
export interface TrajectoryResult {
  success: boolean
  frames: TrajectoryFrame[]
  total_frames: number
  total_atoms: number
  box_size: SimBox
  time_range: { min: number; max: number }
  elements: string[]
}

/** 时序数据点 */
export interface TimeSeriesPoint {
  step: number
  time_fs: number
  value: number
}

/** 时序数据 */
export interface TimeSeriesData {
  label: string
  unit: string
  data: TimeSeriesPoint[]
}

/** 应力/能量时序结果 */
export interface StressEnergyResult {
  temperature_series: TimeSeriesData
  pressure_series: TimeSeriesData
  total_energy_series: TimeSeriesData
  kinetic_energy_series: TimeSeriesData
  potential_energy_series: TimeSeriesData
  volume_series: TimeSeriesData
  virial_stress_series: TimeSeriesData
}

/** 轨迹统计 */
export interface TrajectoryStats {
  avg_temp: number
  avg_pressure: number
  energy_drift: number
  density: number
}

// ============ API 函数 ============

/**
 * 加载轨迹文件
 * @param config 轨迹配置
 */
export async function loadTrajectory(config: TrajectoryConfig): Promise<TrajectoryResult> {
  return invoke<TrajectoryResult>('load_trajectory', { config })
}

/**
 * 获取指定帧
 * @param result 轨迹结果
 * @param frame_number 帧号
 */
export async function getFrame(
  result: TrajectoryResult,
  frame_number: number
): Promise<TrajectoryFrame> {
  return invoke<TrajectoryFrame>('get_frame', { result, frame_number })
}

/**
 * 解析时序数据（从 log 文件）
 * @param log_file_path log 文件路径
 */
export async function parseTimeSeries(log_file_path: string): Promise<StressEnergyResult> {
  return invoke<StressEnergyResult>('parse_time_series', { logFilePath: log_file_path })
}

/**
 * 导出帧图像
 * @param frame 轨迹帧
 * @param format 图像格式 (png/jpeg/svg)
 * @param filepath 输出路径
 */
export async function exportFrameImage(
  frame: TrajectoryFrame,
  format: string,
  filepath: string
): Promise<void> {
  return invoke<void>('export_frame_image', { frame, format, filepath })
}

/**
 * 导出时序数据为 CSV
 * @param data 应力/能量时序结果
 * @param filepath 输出路径
 */
export async function exportTimeSeriesCSV(
  data: StressEnergyResult,
  filepath: string
): Promise<void> {
  return invoke<void>('export_time_series_csv', { data, filepath })
}

/**
 * 获取轨迹统计信息
 * @param result 轨迹结果
 */
export async function getTrajectoryStats(result: TrajectoryResult): Promise<TrajectoryStats> {
  return invoke<TrajectoryStats>('get_trajectory_stats', { result })
}
