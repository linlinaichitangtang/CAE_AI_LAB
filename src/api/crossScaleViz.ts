/**
 * Cross-Scale Visualization Mapping API
 * V1.8-008: 跨尺度数据可视化映射
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type VizLayer =
  | 'atoms'
  | 'phase_field'
  | 'fe_mesh'
  | 'error_bars'
  | 'streamlines'

export interface CrossScaleVizConfig {
  layers: Array<{
    type: VizLayer
    visible: boolean
    opacity: number
    color_map: string
    data_source: string
  }>
  viewport: {
    center: { x: number; y: number; z: number }
    zoom: number
    rotation: { pitch: number; yaw: number }
  }
  mapping_id: string
  overlay_mode: 'transparent' | 'contour' | 'isosurface' | 'cross_section'
}

export interface AtomColorMapping {
  field_name: string
  color_map: string
  range: { min: number; max: number }
  atom_positions: Array<{ x: number; y: number; z: number }>
  atom_values: number[]
  atom_types: string[]
}

export interface FieldOverlayConfig {
  field_name: string
  grid_size: { nx: number; ny: number; nz: number }
  field_values: number[]
  color_map: string
  contour_levels: number
  show_isosurface: boolean
  isosurface_value: number
}

export interface CrossScaleVizResult {
  success: boolean
  layers: Array<{
    type: VizLayer
    data_url: string
    bounds: { min: { x: number; y: number; z: number }; max: { x: number; y: number; z: number } }
  }>
  total_points: number
  rendering_time_ms: number
}

export interface ScaleBridgeAnimation {
  frames: Array<{
    time: number
    layers: CrossScaleVizResult['layers']
  }>
  duration_s: number
  fps: number
}

// ============ API 函数 ============

/**
 * 创建跨尺度可视化
 */
export async function createCrossScaleViz(
  config: CrossScaleVizConfig
): Promise<CrossScaleVizResult> {
  return invoke<CrossScaleVizResult>('create_cross_scale_viz', { config })
}

/**
 * 原子场着色映射
 */
export async function mapAtomToField(
  mapping: AtomColorMapping
): Promise<{ colored_atoms: Array<{ x: number; y: number; z: number; r: number; g: number; b: number }> }> {
  return invoke<{ colored_atoms: Array<{ x: number; y: number; z: number; r: number; g: number; b: number }> }>(
    'map_atom_to_field',
    { mapping }
  )
}

/**
 * 场数据叠加到网格
 */
export async function overlayFieldOnMesh(
  config: FieldOverlayConfig
): Promise<{ mesh_url: string; field_url: string }> {
  return invoke<{ mesh_url: string; field_url: string }>('overlay_field_on_mesh', { config })
}

/**
 * 生成尺度桥接动画
 */
export async function generateScaleBridgeAnimation(
  pipeline_id: string,
  fps?: number
): Promise<ScaleBridgeAnimation> {
  return invoke<ScaleBridgeAnimation>('generate_scale_bridge_animation', { pipeline_id, fps })
}

/**
 * 获取可用颜色映射表
 */
export async function getAvailableColorMaps(): Promise<
  Array<{ id: string; name: string; type: 'sequential' | 'diverging' | 'categorical' }>
> {
  return invoke<Array<{ id: string; name: string; type: 'sequential' | 'diverging' | 'categorical' }>>(
    'get_available_color_maps'
  )
}
