/**
 * useCertificationReport.ts — V3.1-002 ISO/ASME 认证报告
 * 符合 ASME Y14.5 的工程报告签名流程
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export interface SignatureRole {
  id: string
  name: string
  title: string  // 如 "设计工程师", "审核工程师", "项目经理"
  required: boolean
  order: number
}

export interface Signature {
  roleId: string
  signerName: string
  signedAt?: string
  signatureImage?: string  // Base64 或 URL
  comments?: string
  approved: boolean
}

export interface CertificationReport {
  id: string
  projectName: string
  projectId?: string
  reportNumber: string  // 报告编号
  revision: string
  createdAt: string
  updatedAt: string

  // 报告内容
  summary: string
  methodology: string
  results: ReportResults
  conclusions: string

  // 签名
  signatures: Signature[]

  // 元数据
  softwareVersion: string
  solverVersion: string
  meshInfo: {
    nodes: number
    elements: number
    type: string
  }
  materialModels: string[]
  boundaryConditions: string[]

  // 状态
  status: 'draft' | 'pending_review' | 'approved' | 'released'
  attachments?: string[]  // 附件文件路径
}

export interface ReportResults {
  maxDisplacement?: number
  maxDisplacementLocation?: string
  maxStress?: number
  maxStressLocation?: string
  safetyFactor?: number
  criticalLoadCase?: string
  naturalFrequencies?: number[]
}

// ============ 默认签名角色 ============

export const DEFAULT_SIGNATURE_ROLES: SignatureRole[] = [
  { id: 'designer', name: '设计工程师', title: 'Design Engineer', required: true, order: 1 },
  { id: 'analyst', name: '分析工程师', title: 'Analysis Engineer', required: true, order: 2 },
  { id: 'reviewer', name: '技术审核', title: 'Technical Reviewer', required: true, order: 3 },
  { id: 'approver', name: '项目经理', title: 'Project Manager', required: false, order: 4 },
  { id: 'qa', name: '质量保证', title: 'Quality Assurance', required: false, order: 5 }
]

// ============ ASME Y14.5 标准要求 ============

export const ASME_Y145_REQUIREMENTS = {
  tolerances: {
    geometryTolerance: 0.01,  // mm
    surfaceFinish: 1.6  // μm Ra
  },
  documentation: {
    revisionHistory: true,
    changeDescription: true,
    approvalSignatures: true
  },
  reportSections: [
    '1. 目的和范围',
    '2. 参考标准',
    '3. 模型描述',
    '4. 材料属性',
    '5. 边界条件和载荷',
    '6. 分析方法',
    '7. 结果摘要',
    '8. 结论',
    '9. 签名批准'
  ]
}

// ============ 工具函数 ============

function generateId(): string {
  return `cert_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function generateReportNumber(): string {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  const seq = Math.floor(Math.random() * 1000).toString().padStart(3, '0')
  return `CAE-RPT-${year}${month}${day}-${seq}`
}

function getStorage<T>(key: string): T[] {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : []
}

function setStorage<T>(key: string, data: T[]): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useCertificationReport() {
  const reports = ref<CertificationReport[]>([])
  const currentReport = ref<CertificationReport | null>(null)
  const signatureRoles = ref<SignatureRole[]>([...DEFAULT_SIGNATURE_ROLES])

  // 加载历史报告
  function loadReports(): CertificationReport[] {
    const stored = getStorage<CertificationReport>('caelab_certification_reports')
    reports.value = stored
    return stored
  }

  // 获取报告列表
  function getReports(): CertificationReport[] {
    return reports.value.sort((a, b) =>
      new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
    )
  }

  // 获取单个报告
  function getReport(reportId: string): CertificationReport | undefined {
    return reports.value.find(r => r.id === reportId)
  }

  // 创建新报告
  function createReport(data: {
    projectName: string
    projectId?: string
    summary: string
    methodology: string
    results: ReportResults
    conclusions: string
    meshInfo: CertificationReport['meshInfo']
    materialModels: string[]
    boundaryConditions: string[]
  }): CertificationReport {
    const report: CertificationReport = {
      id: generateId(),
      projectName: data.projectName,
      projectId: data.projectId,
      reportNumber: generateReportNumber(),
      revision: 'A',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      summary: data.summary,
      methodology: data.methodology,
      results: data.results,
      conclusions: data.conclusions,
      signatures: signatureRoles.value.map(role => ({
        roleId: role.id,
        signerName: '',
        approved: false
      })),
      softwareVersion: 'CAELab V2.9',
      solverVersion: 'Internal Solver V2.9',
      meshInfo: data.meshInfo,
      materialModels: data.materialModels,
      boundaryConditions: data.boundaryConditions,
      status: 'draft'
    }

    reports.value.unshift(report)
    currentReport.value = report
    saveReports()

    return report
  }

  // 更新报告
  function updateReport(reportId: string, updates: Partial<CertificationReport>): CertificationReport | null {
    const report = reports.value.find(r => r.id === reportId)
    if (!report) return null

    Object.assign(report, updates, { updatedAt: new Date().toISOString() })
    saveReports()
    return report
  }

  // 添加签名
  function addSignature(
    reportId: string,
    roleId: string,
    signerName: string,
    comments?: string
  ): boolean {
    const report = reports.value.find(r => r.id === reportId)
    if (!report) return false

    const signature = report.signatures.find(s => s.roleId === roleId)
    if (!signature) return false

    signature.signerName = signerName
    signature.signedAt = new Date().toISOString()
    signature.comments = comments
    signature.approved = true
    report.updatedAt = new Date().toISOString()

    // 检查是否所有必需签名都已完成
    const allRequiredSigned = signatureRoles.value
      .filter(r => r.required)
      .every(role => {
        const sig = report.signatures.find(s => s.roleId === role.id)
        return sig?.approved
      })

    if (allRequiredSigned && report.status === 'pending_review') {
      report.status = 'approved'
    }

    saveReports()
    return true
  }

  // 提交审核
  function submitForReview(reportId: string): boolean {
    const report = reports.value.find(r => r.id === reportId)
    if (!report) return false

    report.status = 'pending_review'
    report.updatedAt = new Date().toISOString()
    saveReports()
    return true
  }

  // 发布报告
  function releaseReport(reportId: string): boolean {
    const report = reports.value.find(r => r.id === reportId)
    if (!report) return false

    // 检查所有必需签名
    const allRequiredSigned = signatureRoles.value
      .filter(r => r.required)
      .every(role => {
        const sig = report.signatures.find(s => s.roleId === role.id)
        return sig?.approved
      })

    if (!allRequiredSigned) return false

    report.status = 'released'
    report.updatedAt = new Date().toISOString()
    saveReports()
    return true
  }

  // 导出报告为 PDF
  function exportReport(reportId: string): string | null {
    const report = getReport(reportId)
    if (!report) return null

    // 生成符合 ASME Y14.5 的报告内容
    const content = generateReportContent(report)
    return content
  }

  // 生成报告内容
  function generateReportContent(report: CertificationReport): string {
    const lines: string[] = []

    // 页眉
    lines.push('=' .repeat(80))
    lines.push('ENGINEERING ANALYSIS REPORT')
    lines.push('=' .repeat(80))
    lines.push('')

    // 报告信息
    lines.push(`Report Number: ${report.reportNumber}`)
    lines.push(`Revision: ${report.revision}`)
    lines.push(`Date: ${new Date(report.createdAt).toLocaleDateString()}`)
    lines.push(`Project: ${report.projectName}`)
    lines.push('')

    // 1. 目的和范围
    lines.push('1. PURPOSE AND SCOPE')
    lines.push('-'.repeat(80))
    lines.push(report.summary)
    lines.push('')

    // 2. 参考标准
    lines.push('2. REFERENCE STANDARDS')
    lines.push('-'.repeat(80))
    lines.push('- ASME Y14.5-2018 Geometric Dimensioning and Tolerancing')
    lines.push('- ASME Y14.5.1-2019 Mathematical Definition of GD&T')
    lines.push(`- CAELab Software Version: ${report.softwareVersion}`)
    lines.push(`- Solver Version: ${report.solverVersion}`)
    lines.push('')

    // 3. 模型描述
    lines.push('3. MODEL DESCRIPTION')
    lines.push('-'.repeat(80))
    lines.push(`Mesh Nodes: ${report.meshInfo.nodes}`)
    lines.push(`Mesh Elements: ${report.meshInfo.elements}`)
    lines.push(`Element Type: ${report.meshInfo.type}`)
    lines.push('')

    // 4. 材料属性
    lines.push('4. MATERIAL PROPERTIES')
    lines.push('-'.repeat(80))
    report.materialModels.forEach((mat, i) => {
      lines.push(`${i + 1}. ${mat}`)
    })
    lines.push('')

    // 5. 边界条件和载荷
    lines.push('5. BOUNDARY CONDITIONS AND LOADS')
    lines.push('-'.repeat(80))
    report.boundaryConditions.forEach((bc, i) => {
      lines.push(`${i + 1}. ${bc}`)
    })
    lines.push('')

    // 6. 分析方法
    lines.push('6. ANALYSIS METHODOLOGY')
    lines.push('-'.repeat(80))
    lines.push(report.methodology)
    lines.push('')

    // 7. 结果摘要
    lines.push('7. RESULTS SUMMARY')
    lines.push('-'.repeat(80))
    if (report.results.maxDisplacement !== undefined) {
      lines.push(`Maximum Displacement: ${report.results.maxDisplacement} m`)
      if (report.results.maxDisplacementLocation) {
        lines.push(`  Location: ${report.results.maxDisplacementLocation}`)
      }
    }
    if (report.results.maxStress !== undefined) {
      lines.push(`Maximum Stress: ${report.results.maxStress} MPa`)
      if (report.results.maxStressLocation) {
        lines.push(`  Location: ${report.results.maxStressLocation}`)
      }
    }
    if (report.results.safetyFactor !== undefined) {
      lines.push(`Safety Factor: ${report.results.safetyFactor}`)
    }
    if (report.results.naturalFrequencies?.length) {
      lines.push(`Natural Frequencies: ${report.results.naturalFrequencies.join(', ')} Hz`)
    }
    lines.push('')

    // 8. 结论
    lines.push('8. CONCLUSIONS')
    lines.push('-'.repeat(80))
    lines.push(report.conclusions)
    lines.push('')

    // 9. 签名批准
    lines.push('9. APPROVAL SIGNATURES')
    lines.push('-'.repeat(80))
    lines.push('')
    lines.push('Per ASME Y14.5-2018, the following personnel have reviewed and approved this report:')
    lines.push('')

    for (const sig of report.signatures) {
      const role = signatureRoles.value.find(r => r.id === sig.roleId)
      lines.push(`${role?.name || sig.roleId} (${role?.title || ''}):`)
      lines.push(`  Signed by: ${sig.signerName || '_____________'}`)
      lines.push(`  Date: ${sig.signedAt ? new Date(sig.signedAt).toLocaleDateString() : '_____________'}`)
      if (sig.comments) {
        lines.push(`  Comments: ${sig.comments}`)
      }
      lines.push(`  Status: ${sig.approved ? 'APPROVED ✓' : 'PENDING'}`)
      lines.push('')
    }

    // 页脚
    lines.push('=' .repeat(80))
    lines.push(`Report ID: ${report.id}`)
    lines.push(`Generated by CAELab on ${new Date().toISOString()}`)
    lines.push('This report is subject to the terms and conditions of the software license agreement.')

    return lines.join('\n')
  }

  // 保存报告
  function saveReports(): void {
    setStorage('caelab_certification_reports', reports.value)
  }

  // 删除报告
  function deleteReport(reportId: string): boolean {
    const index = reports.value.findIndex(r => r.id === reportId)
    if (index === -1) return false

    reports.value.splice(index, 1)
    saveReports()
    return true
  }

  // 复制报告
  function duplicateReport(reportId: string): CertificationReport | null {
    const original = getReport(reportId)
    if (!original) return null

    const duplicate = createReport({
      projectName: `${original.projectName} (Copy)`,
      summary: original.summary,
      methodology: original.methodology,
      results: { ...original.results },
      conclusions: original.conclusions,
      meshInfo: { ...original.meshInfo },
      materialModels: [...original.materialModels],
      boundaryConditions: [...original.boundaryConditions]
    })

    // 更新 revision 和状态
    duplicate.revision = 'A'
    duplicate.status = 'draft'

    return duplicate
  }

  // 初始化
  loadReports()

  return {
    reports,
    currentReport,
    signatureRoles,
    getReports,
    getReport,
    createReport,
    updateReport,
    addSignature,
    submitForReview,
    releaseReport,
    exportReport,
    deleteReport,
    duplicateReport,
    ASME_REQUIREMENTS: ASME_Y145_REQUIREMENTS
  }
}