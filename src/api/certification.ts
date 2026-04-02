/**
 * ISO/ASME Certification Report Generator API
 * V1.4-011 - 认证报告生成器后端API封装
 */

import { invoke } from '@tauri-apps/api/core'

// ============ 类型定义 ============

export type CertificationStandard =
  | 'ASME_Y14_5'
  | 'ISO_2553'
  | 'ASME_BPVC_VIII'
  | 'ISO_15614'
  | 'ASME_B31_3'
  | 'ISO_9001'

export interface StressResult {
  location: string
  stress_type: 'membrane' | 'bending' | 'peak' | 'primary' | 'secondary'
  value: number
  allowable: number
  utilization_ratio: number
  status: 'pass' | 'fail' | 'warning'
}

export interface SafetyFactorResult {
  component_name: string
  safety_factor: number
  minimum_required: number
  status: 'pass' | 'fail' | 'warning'
  critical_location: string
}

export interface ComplianceItem {
  clause_id: string
  description: string
  requirement: string
  evidence: string
  status: 'compliant' | 'non_compliant' | 'not_applicable' | 'pending'
  notes: string
}

export interface OperatingConditions {
  temperature: number
  pressure: number
  cyclic_loading: boolean
}

export interface CertificationConfig {
  project_id: string
  standard: CertificationStandard
  component_name: string
  material_spec: string
  design_code: string
  operating_conditions: OperatingConditions
  stress_results: StressResult[]
  safety_factors: SafetyFactorResult[]
  inspector_name: string
  report_number: string
}

export interface ComplianceSummary {
  total: number
  compliant: number
  non_compliant: number
  pending: number
}

export interface CertificationResult {
  success: boolean
  overall_status: 'pass' | 'fail' | 'conditional'
  compliance_summary: ComplianceSummary
  stress_check_results: StressResult[]
  safety_factor_results: SafetyFactorResult[]
  compliance_items: ComplianceItem[]
  report_generated_at: string
  report_file_path: string
  recommendations: string[]
}

export interface CertificationTemplate {
  id: string
  name: string
  standard: CertificationStandard
  description: string
}

// ============ API 函数 ============

/**
 * 生成认证报告
 */
export async function generateCertificationReport(
  config: CertificationConfig
): Promise<CertificationResult> {
  return invoke<CertificationResult>('generate_certification_report', { config })
}

/**
 * 检查应力合规性
 */
export async function checkStressCompliance(
  stressResults: StressResult[],
  standard: CertificationStandard
): Promise<StressResult[]> {
  return invoke<StressResult[]>('check_stress_compliance', { stressResults, standard })
}

/**
 * 计算安全系数
 */
export async function calculateSafetyFactors(
  loads: Record<string, unknown>,
  material: Record<string, unknown>,
  standard: CertificationStandard
): Promise<SafetyFactorResult[]> {
  return invoke<SafetyFactorResult[]>('calculate_safety_factors', { loads, material, standard })
}

/**
 * 获取合规检查清单
 */
export async function getComplianceChecklist(
  standard: CertificationStandard
): Promise<ComplianceItem[]> {
  return invoke<ComplianceItem[]>('get_compliance_checklist', { standard })
}

/**
 * 导出认证PDF报告
 */
export async function exportCertificationPDF(
  result: CertificationResult
): Promise<string> {
  return invoke<string>('export_certification_pdf', { result })
}

/**
 * 获取认证模板列表
 */
export async function getCertificationTemplates(): Promise<CertificationTemplate[]> {
  return invoke<CertificationTemplate[]>('get_certification_templates')
}
