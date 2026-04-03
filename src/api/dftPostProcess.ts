/**
 * DFT Post-Processing API
 * V1.7-004: DFT 结果解析器 (VASP / QE / ABINIT)
 * V1.7-005: 能量/力验证
 * V1.7-009: 能带/DOS 可视化
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type DftOutputFormat = 'vasp' | 'quantum_espresso' | 'abinit'

export interface KpointData {
  label: string
  x: number
  y: number
  z: number
}

export interface BandPoint {
  k_index: number
  energy: number
  eV: number
}

export interface BandStructureData {
  kpoints: KpointData[]
  bands: BandPoint[][]
  num_bands: number
  fermi_energy_eV: number
  band_gap_eV: number
  direct_gap_eV: number
  indirect_gap_eV: number
  high_symmetry_points: Array<{ label: string; energy: number }>
}

export interface PartialDos {
  element: string
  orbital: string
  dos: number[]
}

export interface DosData {
  energies: number[]
  total_dos: number[]
  partial_dos: PartialDos[]
  fermi_energy_eV: number
  integrated_dos: number[]
}

export interface ChargeDensityData {
  grid: { nx: number; ny: number; nz: number }
  data: number[][][]
  isosurface_values: number[]
}

export interface DftAtom {
  element: string
  index: number
  x: number
  y: number
  z: number
}

export interface DftForce {
  fx: number
  fy: number
  fz: number
  magnitude: number
}

export interface DftForcesData {
  atoms: DftAtom[]
  forces: DftForce[]
  max_force: number
  rms_force: number
}

export interface IonicStep {
  step: number
  energy: number
  drift: number
}

export interface ElectronicStep {
  step: number
  energy: number
}

export interface DftEnergyData {
  total_energy_eV: number
  energy_per_atom_eV: number
  free_energy_eV: number
  entropy: number
  zero_point_energy: number
  ionic_steps: IonicStep[]
  electronic_steps: ElectronicStep[]
}

export interface ValidationResult {
  test_name: string
  reference_energy_eV: number
  calculated_energy_eV: number
  error_meV_per_atom: number
  passed: boolean
  tolerance_meV: number
}

export interface DftParseResult {
  success: boolean
  format: DftOutputFormat
  energy: DftEnergyData
  forces: DftForcesData
  dos: DosData | null
  band_structure: BandStructureData | null
  charge_density: ChargeDensityData | null
  convergence_achieved: boolean
}

export interface ValidationTestCase {
  id: string
  name: string
  description: string
  reference_energy: number
}

// ============ API 函数 ============

/**
 * 解析 DFT 计算输出目录
 */
export async function parseDftOutput(
  directory: string,
  format: DftOutputFormat
): Promise<DftParseResult> {
  return invoke<DftParseResult>('parse_dft_output', {
    directory,
    format,
  })
}

/**
 * 解析 VASP OUTCAR 文件
 */
export async function parseVaspOutcar(filepath: string): Promise<DftEnergyData> {
  return invoke<DftEnergyData>('parse_vasp_outcar', { filepath })
}

/**
 * 解析 VASP DOSCAR 文件
 */
export async function parseVaspDoscar(filepath: string): Promise<DosData> {
  return invoke<DosData>('parse_vasp_doscar', { filepath })
}

/**
 * 解析 VASP EIGENVAL / PROCAR 文件
 */
export async function parseVaspEigenv(filepath: string): Promise<BandStructureData> {
  return invoke<BandStructureData>('parse_vasp_eigenv', { filepath })
}

/**
 * 解析 VASP CHGCAR / PARCHG 文件
 */
export async function parseVaspContcar(filepath: string): Promise<ChargeDensityData> {
  return invoke<ChargeDensityData>('parse_vasp_contcar', { filepath })
}

/**
 * 解析 Quantum ESPRESSO 输出文件
 */
export async function parseQeOutput(filepath: string): Promise<DftEnergyData> {
  return invoke<DftEnergyData>('parse_qe_output', { filepath })
}

/**
 * 解析 Quantum ESPRESSO DOS 文件
 */
export async function parseQeDos(filepath: string): Promise<DosData> {
  return invoke<DosData>('parse_qe_dos', { filepath })
}

/**
 * 运行验证算例
 */
export async function runValidation(test_case: string): Promise<ValidationResult[]> {
  return invoke<ValidationResult[]>('run_validation', { testCase: test_case })
}

/**
 * 获取验证测试套件列表
 */
export async function getValidationTestSuite(): Promise<ValidationTestCase[]> {
  return invoke<ValidationTestCase[]>('get_validation_test_suite')
}
