/**
 * DFT Input Generator API
 * V1.7-001: VASP 输入生成器
 * V1.7-002: QE 输入生成器
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type DftCode = 'vasp' | 'quantum_espresso'

// ---- VASP POSCAR ----

export interface VaspLattice {
  a: number
  b: number
  c: number
  alpha: number
  beta: number
  gamma: number
}

export interface VaspPosition {
  type: string
  x: number
  y: number
  z: number
  dx: boolean
  dy: boolean
  dz: boolean
}

export interface VaspPoscar {
  comment: string
  scaling_factor: number
  lattice: VaspLattice
  atom_types: string[]
  atom_counts: number[]
  selective_dynamics: boolean[][]
  positions: VaspPosition[]
  coordinate_type: 'cartesian' | 'direct'
}

// ---- VASP INCAR ----

export interface VaspLdauParam {
  element: string
  U: number
  J: number
}

export interface VaspIncar {
  system: string
  encut: number
  ismear: number
  sigma: number
  isif: number
  ibrion: number
  nsw: number
  ediff: number
  ediffg: number
  nelm: number
  lcharg: boolean
  lwave: boolean
  icharg: number
  prec: string
  kpoints_auto: boolean
  nbands: number
  gga: string
  ldau_luj: boolean
  ldau_params: VaspLdauParam[]
  magmom: number[]
}

// ---- VASP KPOINTS ----

export interface VaspKpointPath {
  label1: string
  label2: string
  npoints: number
}

export interface VaspKpoints {
  style: 'gamma' | 'monkhorst' | 'automatic' | 'line'
  grid: { n1: number; n2: number; n3: number }
  shift: { s1: number; s2: number; s3: number }
  kpath: VaspKpointPath[]
}

// ---- Quantum ESPRESSO ----

export interface QeControl {
  calculation: string
  prefix: string
  outdir: string
  pseudo_dir: string
  etot_conv: number
  forc_conv: number
}

export interface QeSystem {
  ibrav: number
  celldm: number[]
  nat: number
  nbp: number
  ecutwfc: number
  ecutrho: number
  occupations: string
  smearing: string
  degauss: number
}

export interface QeElectrons {
  conv_thr: number
  mixing_beta: number
  electron_maxstep: number
}

export interface QeAtomicSpecies {
  name: string
  mass: number
  pseudo: string
}

export interface QeAtomicPosition {
  atom: string
  x: number
  y: number
  z: number
}

export interface QeAtomicPositions {
  type: string
  positions: QeAtomicPosition[]
}

export interface QeKPoints {
  type: string
  grid: { n1: number; n2: number; n3: number }
  offset: { s1: number; s2: number; s3: number }
}

export interface QeInput {
  control: QeControl
  system: QeSystem
  electrons: QeElectrons
  atomic_species: QeAtomicSpecies[]
  atomic_positions: QeAtomicPositions
  k_points: QeKPoints
}

// ---- 通用结果 ----

export interface DftInputResult {
  success: boolean
  code: DftCode
  files: Record<string, string>
  file_paths: Record<string, string>
}

// ---- 模板 ----

export interface VaspTemplate {
  id: string
  name: string
  description: string
  incar: VaspIncar
  poscar: VaspPoscar
  kpoints: VaspKpoints
}

export interface QeTemplate {
  id: string
  name: string
  description: string
  input: QeInput
}

// ============ API 函数 ============

/**
 * 生成 VASP 输入文件 (POSCAR / INCAR / KPOINTS)
 */
export async function generateVaspInput(
  poscar: VaspPoscar,
  incar: VaspIncar,
  kpoints: VaspKpoints
): Promise<DftInputResult> {
  return invoke<DftInputResult>('generate_vasp_input', { poscar, incar, kpoints })
}

/**
 * 生成 Quantum ESPRESSO 输入文件
 */
export async function generateQeInput(qeInput: QeInput): Promise<DftInputResult> {
  return invoke<DftInputResult>('generate_qe_input', { qeInput })
}

/**
 * 解析 VASP POSCAR 文件内容
 */
export async function parseVaspPoscar(content: string): Promise<VaspPoscar> {
  return invoke<VaspPoscar>('parse_vasp_poscar', { content })
}

/**
 * 解析 VASP INCAR 文件内容
 */
export async function parseVaspIncar(content: string): Promise<VaspIncar> {
  return invoke<VaspIncar>('parse_vasp_incar', { content })
}

/**
 * 获取 VASP 模板列表
 */
export async function getVaspTemplates(): Promise<VaspTemplate[]> {
  return invoke<VaspTemplate[]>('get_vasp_templates')
}

/**
 * 获取 Quantum ESPRESSO 模板列表
 */
export async function getQeTemplates(): Promise<QeTemplate[]> {
  return invoke<QeTemplate[]>('get_qe_templates')
}

/**
 * 导出生成的输入文件到指定目录
 */
export async function exportInputFiles(
  result: DftInputResult,
  directory: string
): Promise<void> {
  return invoke<void>('export_input_files', { result, directory })
}
