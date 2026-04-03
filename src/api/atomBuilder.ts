/**
 * Atom Structure Builder API
 * V1.5-003: 原子结构建模器 - 晶体/非晶/界面/碳纳米管/团簇/位错
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type CrystalStructure = 'FCC' | 'BCC' | 'HCP' | 'diamond' | 'SC' | 'custom'

export interface Element {
  symbol: string
  name: string
  atomic_number: number
  mass_amu: number
  lattice_constant_A: number
  crystal_structure: CrystalStructure
  covalent_radius_A: number
  color: string
}

export interface SupercellConfig {
  nx: number
  ny: number
  nz: number
  crystal_structure: CrystalStructure
  element: Element
  lattice_constant_A: number
  orientation: { x: number; y: number; z: number }
  vacuum_A: number
}

export interface AmorphousConfig {
  element: Element
  num_atoms: number
  density_g_cm3: number
  box_size_A: number
  random_seed: number
  quench_rate_K_ps: number
}

export interface InterfaceConfig {
  element_bottom: Element
  element_top: Element
  crystal_bottom: CrystalStructure
  crystal_top: CrystalStructure
  orientation_bottom: { x: number; y: number; z: number }
  orientation_top: { x: number; y: number; z: number }
  mismatch_tolerance: number
  vacuum_A: number
  num_layers_bottom: number
  num_layers_top: number
}

export interface DefectConfig {
  base_config: SupercellConfig
  defect_type: 'vacancy' | 'interstitial' | 'substitution' | 'dislocation_edge' | 'dislocation_screw' | 'grain_boundary'
  defect_density: number
  defect_positions: { x: number; y: number; z: number }[]
}

export interface CNTConfig {
  chirality_n: number
  chirality_m: number
  length_A: number
  element: Element
  cap_end: boolean
  vacancy_defect_density: number
}

export interface ClusterConfig {
  element: Element
  cluster_type: 'icosahedron' | 'decahedron' | 'fcc_cube' | 'bcc_cube' | 'sphere'
  num_atoms: number
  lattice_constant_A: number
}

export interface MdAtom {
  id: number
  type: number
  symbol: string
  x: number
  y: number
  z: number
  fx: number
  fy: number
  fz: number
}

export interface AtomBuilderResult {
  success: boolean
  atoms: MdAtom[]
  box: { lx: number; ly: number; lz: number }
  num_atoms: number
  num_types: number
  density: number
  volume_A3: number
  mass_amu: number
  visualization_data: {
    bonds: Array<{ i: number; j: number }>
    colors: Record<string, string>
  }
}

export interface AtomBuilderTemplate {
  id: string
  name: string
  name_en: string
  category: string
  description: string
  config_type: 'supercell' | 'amorphous' | 'interface' | 'defect' | 'cnt' | 'cluster'
  typical_application: string
}

// ============ API 函数 ============

/**
 * 构建超胞结构
 */
export async function buildSupercell(config: SupercellConfig): Promise<AtomBuilderResult> {
  return invoke<AtomBuilderResult>('build_supercell', { config })
}

/**
 * 构建非晶结构
 */
export async function buildAmorphous(config: AmorphousConfig): Promise<AtomBuilderResult> {
  return invoke<AtomBuilderResult>('build_amorphous', { config })
}

/**
 * 构建界面结构
 */
export async function buildInterface(config: InterfaceConfig): Promise<AtomBuilderResult> {
  return invoke<AtomBuilderResult>('build_interface', { config })
}

/**
 * 构建缺陷结构
 */
export async function buildDefect(config: DefectConfig): Promise<AtomBuilderResult> {
  return invoke<AtomBuilderResult>('build_defect', { config })
}

/**
 * 构建碳纳米管
 */
export async function buildCNT(config: CNTConfig): Promise<AtomBuilderResult> {
  return invoke<AtomBuilderResult>('build_cnt', { config })
}

/**
 * 构建团簇结构
 */
export async function buildCluster(config: ClusterConfig): Promise<AtomBuilderResult> {
  return invoke<AtomBuilderResult>('build_cluster', { config })
}

/**
 * 获取元素库
 */
export async function getElementLibrary(): Promise<Element[]> {
  return invoke<Element[]>('get_element_library')
}

/**
 * 计算密度
 */
export async function calculateDensity(
  atoms: MdAtom[],
  box: { lx: number; ly: number; lz: number }
): Promise<number> {
  return invoke<number>('calculate_density', { atoms, box })
}

/**
 * 获取原子结构建模器模板
 */
export async function getAtomBuilderTemplates(): Promise<AtomBuilderTemplate[]> {
  return invoke<AtomBuilderTemplate[]>('get_atom_builder_templates')
}
