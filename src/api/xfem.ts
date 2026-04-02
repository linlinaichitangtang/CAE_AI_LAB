/**
 * XFEM (Extended Finite Element Method) API
 * V1.3-006: 断裂力学扩展有限元分析 API 封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

/** 裂纹定义 */
export interface CrackDefinition {
  id: string
  type: 'edge' | 'center' | 'corner'
  initial_length: number          // m
  orientation_angle: number       // deg
  position: { x: number; y: number; z: number }
  propagation_direction: 'auto' | 'manual'
  enrichment_type: 'standard' | 'phantom_node' | 'vietnam'
}

/** XFEM 材料属性 */
export interface XfemMaterial {
  name: string
  E: number                       // Pa (弹性模量)
  nu: number                      // 泊松比
  density: number                 // kg/m^3
  yield_strength: number          // Pa
  fracture_toughness: number      // KIc, MPa*sqrt(m)
  tensile_strength: number        // Pa
}

/** XFEM 分析配置 */
export interface XfemConfig {
  project_id: string
  analysis_type: 'static' | 'fatigue_crack_growth' | 'dynamic'
  material: XfemMaterial
  crack: CrackDefinition
  mesh_size: number               // m
  max_increments: number
  enrichment_radius: number       // m
  domain_integral_type: 'interaction' | 'j_integral' | 'cstar'
  growth_criterion: 'max_circumferential_stress' | 'max_energy_release_rate' | 'wilson'
}

/** XFEM 分析结果 */
export interface XfemResult {
  success: boolean
  stress_intensity_factor: {
    KI: number                    // MPa*sqrt(m)
    KII: number                   // MPa*sqrt(m)
    KIII: number                  // MPa*sqrt(m)
  }
  j_integral: number              // N/m (J积分)
  energy_release_rate: number     // G, N/m (能量释放率)
  crack_tip_position: { x: number; y: number; z: number }
  crack_length: number            // m
  crack_path: Array<{ x: number; y: number; z: number; step: number }>
  stress_field: Array<{
    x: number
    y: number
    z: number
    sigma_xx: number
    sigma_yy: number
    tau_xy: number
  }>
  num_steps: number
  converged: boolean
}

/** XFEM 工程模板 */
export interface XfemTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  crack_type: 'edge' | 'center' | 'corner'
  default_material: Partial<XfemMaterial>
  typical_KIc_range: [number, number]
}

/** 应力强度因子计算结果 */
export interface StressIntensityResult {
  sigma_theta: number             // MPa
  tau_rtheta: number              // MPa
}

// ============ API 函数 ============

/**
 * 运行 XFEM 断裂力学分析
 */
export async function runXfemAnalysis(config: XfemConfig): Promise<XfemResult> {
  return invoke<XfemResult>('run_xfem_analysis', { config })
}

/**
 * 获取 XFEM 工程算例模板
 */
export async function getXfemTemplates(): Promise<XfemTemplate[]> {
  return invoke<XfemTemplate[]>('get_xfem_templates')
}

/**
 * 生成 XFEM 求解器输入文件 (.inp)
 */
export async function generateXfemInp(config: XfemConfig): Promise<string> {
  return invoke<string>('generate_xfem_inp', { config })
}

/**
 * 计算裂尖周向应力分量 (Williams 展开)
 * @param KI   I型应力强度因子 MPa*sqrt(m)
 * @param KII  II型应力强度因子 MPa*sqrt(m)
 * @param KIII III型应力强度因子 MPa*sqrt(m)
 * @param angle 周向角 (rad)
 */
export async function calculateStressIntensityFactor(
  KI: number,
  KII: number,
  KIII: number,
  angle: number
): Promise<StressIntensityResult> {
  return invoke<StressIntensityResult>('calculate_stress_intensity_factor', {
    KI,
    KII,
    KIII,
    angle,
  })
}

/**
 * 获取 XFEM 材料库
 */
export async function getXfemMaterialLibrary(): Promise<XfemMaterial[]> {
  return invoke<XfemMaterial[]>('get_xfem_material_library')
}
