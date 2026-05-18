/**
 * useComplianceChecker.ts — V3.5-001 ASME/NRC/IEC 标准模板 & V3.5-002 自动合规检查
 * 行业标准报告模板，一键合规检查
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type StandardType = 'asme' | 'nrc' | 'iec' | 'iso' | 'gb' | 'custom'
export type StandardVersion = 'yo14_5' | 'yo5_1' | '10cfr50' | 'ieee603' | 'iec61511' | 'iso9001' | 'asme_v'
export type CheckSeverity = 'critical' | 'major' | 'minor' | 'info'
export type CheckStatus = 'passed' | 'failed' | 'warning' | 'skipped' | 'not_applicable'

export interface Standard {
  id: string
  name: string
  code: string
  version: string
  description: string
  category: StandardType
  effectiveDate: string
  requirements: StandardRequirement[]
}

export interface StandardRequirement {
  id: string
  section: string  // e.g., "Y14.5-2018 Section 5.3"
  title: string
  description: string
  checkRules: CheckRule[]
  severity: CheckSeverity
  category: string
}

export interface CheckRule {
  id: string
  type: 'tolerance' | 'dimension' | 'material' | 'stress' | 'factor_of_safety' | 'documentation' | 'format'
  params: Record<string, any>
  errorMessage: string
  recommendation?: string
}

export interface ComplianceCheck {
  id: string
  projectId: string
  projectName: string
  standard: string
  checkDate: string
  status: 'in_progress' | 'completed'
  results: CheckResult[]
  summary: CheckSummary
}

export interface CheckResult {
  id: string
  requirementId: string
  requirementTitle: string
  section: string
  status: CheckStatus
  severity: CheckSeverity
  details?: string
  measuredValue?: number
  tolerance?: number
  passCriteria?: string
  failedElements?: string[]
  recommendations?: string[]
  screenshots?: string[]
}

export interface CheckSummary {
  totalChecks: number
  passed: number
  failed: number
  warnings: number
  notApplicable: number
  criticalFailed: number
  passRate: number
  complianceLevel: 'full' | 'major' | 'partial' | 'non_compliant'
}

export interface ComplianceReport {
  id: string
  projectId: string
  projectName: string
  standards: string[]
  checkDate: string
  summary: CheckSummary
  results: CheckResult[]
  approvedBy?: string
  approvalDate?: string
  digitalSignature?: string
}

export interface ToleranceTable {
  standard: string
  dimensionRange: { min: number; max: number }
  toleranceGrade: string
  itGrade?: string
  values: Record<string, { value: number; unit: string }>
}

// ============ 标准数据 ============

export const STANDARDS: Standard[] = [
  {
    id: 'asme_y14_5_2018',
    name: 'ASME Y14.5-2018',
    code: 'ASME Y14.5',
    version: '2018',
    description: 'Geometric Dimensioning and Tolerancing (GD&T)',
    category: 'asme',
    effectiveDate: '2019-01-01',
    requirements: [
      {
        id: 'y14_5_1_1',
        section: 'Y14.5-2018 Section 1.4',
        title: 'General Tolerancing',
        description: '除非另有说明，默认线性公差适用于所有尺寸',
        checkRules: [
          { id: 'ch1', type: 'tolerance', params: { standard: 'asme', grade: 'ISO 2768-m' }, errorMessage: '尺寸超出默认公差范围' }
        ],
        severity: 'major',
        category: 'dimensioning'
      },
      {
        id: 'y14_5_2_3',
        section: 'Y14.5-2018 Section 2.3',
        title: 'Datums and Datum Features',
        description: '零件必须正确识别和标注基准',
        checkRules: [
          { id: 'ch2', type: 'documentation', params: { requireDatum: true }, errorMessage: '缺少必要的基准标注' }
        ],
        severity: 'critical',
        category: 'datums'
      },
      {
        id: 'y14_5_3_5',
        section: 'Y14.5-2018 Section 3.5',
        title: 'Profile Tolerances',
        description: '轮廓度公差用于控制表面轮廓',
        checkRules: [
          { id: 'ch3', type: 'tolerance', params: { profileControl: true }, errorMessage: '轮廓度公差不合格' }
        ],
        severity: 'major',
        category: 'tolerance'
      }
    ]
  },
  {
    id: 'nrc_10cfr50',
    name: 'NRC 10 CFR 50',
    code: 'NRC',
    version: 'App A',
    description: 'Nuclear Power Plant Equipment Qualification',
    category: 'nrc',
    effectiveDate: '2020-01-01',
    requirements: [
      {
        id: 'nrc_app_a_100',
        section: '10CFR50 App A Section 100',
        title: 'Design Basis Conditions',
        description: '核设备必须满足设计基准条件',
        checkRules: [
          { id: 'nrc1', type: 'factor_of_safety', params: { minFOS: 3.0 }, errorMessage: '安全系数低于 NRC 要求', recommendation: '增加结构壁厚或降低工作应力' }
        ],
        severity: 'critical',
        category: 'safety'
      },
      {
        id: 'nrc_app_a_200',
        section: '10CFR50 App A Section 200',
        title: 'Material Requirements',
        description: '核级材料必须符合 ASME 规范',
        checkRules: [
          { id: 'nrc2', type: 'material', params: { requireNuclearGrade: true }, errorMessage: '材料不符合核级要求', recommendation: '使用经认证的核级材料' }
        ],
        severity: 'critical',
        category: 'material'
      }
    ]
  },
  {
    id: 'iec_61511',
    name: 'IEC 61511',
    code: 'IEC',
    version: '2016',
    description: 'Functional Safety - Safety Instrumented Systems',
    category: 'iec',
    effectiveDate: '2017-01-01',
    requirements: [
      {
        id: 'iec_61511_1_3',
        section: 'IEC 61511-1 Clause 11.3',
        title: 'SIS Hardware Requirements',
        description: '安全仪表系统的硬件完整性要求',
        checkRules: [
          { id: 'iec1', type: 'stress', params: { maxStressRatio: 0.5 }, errorMessage: '应力比超过SIS安全阈值' }
        ],
        severity: 'critical',
        category: 'hardware'
      }
    ]
  },
  {
    id: 'asme_v_2019',
    name: 'ASME V Article 1',
    code: 'ASME V',
    version: '2019',
    description: 'Nondestructive Examination',
    category: 'asme',
    effectiveDate: '2019-07-01',
    requirements: [
      {
        id: 'asme_v_1_10',
        section: 'ASME V Section 1.10',
        title: 'UT Examination Requirements',
        description: '超声检测人员资格和设备要求',
        checkRules: [
          { id: 'asme_v1', type: 'documentation', params: { requireCert: true }, errorMessage: '缺少NDT人员资格证明' }
        ],
        severity: 'major',
        category: 'ndt'
      }
    ]
  },
  {
    id: 'gb_50017_2017',
    name: 'GB 50017-2017',
    code: 'GB 50017',
    version: '2017',
    description: '钢结构设计标准 (Code for Design of Steel Structures)',
    category: 'gb',
    effectiveDate: '2018-07-01',
    requirements: [
      {
        id: 'gb50017_6_1',
        section: 'GB 50017-2017 第6.1条',
        title: '轴心受力构件强度验算',
        description: '轴心受拉/受压构件净截面应力不超过钢材强度设计值：N/A_n ≤ f',
        checkRules: [
          { id: 'gb_s1', type: 'stress', params: { maxStressRatio: 1.0, designStrength: true }, errorMessage: '轴心受力构件应力超过强度设计值 f', recommendation: '增大截面面积或选用更高强度钢材' }
        ],
        severity: 'critical',
        category: 'strength'
      },
      {
        id: 'gb50017_6_2',
        section: 'GB 50017-2017 第6.2条',
        title: '受弯构件强度验算',
        description: '受弯构件正应力不超过强度设计值：M/(γ·W_nx) ≤ f，γ为截面塑性发展系数',
        checkRules: [
          { id: 'gb_s2', type: 'stress', params: { maxStressRatio: 1.0, designStrength: true, plasticFactor: 1.05 }, errorMessage: '受弯构件弯曲应力超过强度设计值', recommendation: '增大截面模量或选用更高强度钢材' }
        ],
        severity: 'critical',
        category: 'strength'
      },
      {
        id: 'gb50017_6_3',
        section: 'GB 50017-2017 第6.3条',
        title: '受弯构件挠度验算',
        description: '受弯构件挠度不超过限值：δ ≤ L/250（一般楼盖梁）或 L/400（有悬挂起重机）',
        checkRules: [
          { id: 'gb_s3', type: 'dimension', params: { deflectionLimit: 250 }, errorMessage: '挠度超出 L/250 限值', recommendation: '增大截面惯性矩或减小跨度' }
        ],
        severity: 'major',
        category: 'serviceability'
      },
      {
        id: 'gb50017_7_1',
        section: 'GB 50017-2017 第7.1条',
        title: '轴心受压构件整体稳定性',
        description: '轴心受压构件稳定性验算：N/(φ·A) ≤ f，φ为轴心受压构件稳定系数',
        checkRules: [
          { id: 'gb_s4', type: 'factor_of_safety', params: { minFOS: 1.0, stabilityCheck: true }, errorMessage: '压杆稳定应力超过强度设计值', recommendation: '减小长细比、增大截面回转半径或选用更高强度钢材' }
        ],
        severity: 'critical',
        category: 'stability'
      },
      {
        id: 'gb50017_7_2',
        section: 'GB 50017-2017 第7.2条',
        title: '受弯构件整体稳定性',
        description: '受弯构件整体稳定验算：M_x/(φ_b·W_x) ≤ f',
        checkRules: [
          { id: 'gb_s5', type: 'factor_of_safety', params: { minFOS: 1.0, lateralTorsionalBuckling: true }, errorMessage: '受弯构件整体稳定性不满足要求', recommendation: '增设侧向支撑或增大受压翼缘宽度' }
        ],
        severity: 'critical',
        category: 'stability'
      },
      {
        id: 'gb50017_8_1',
        section: 'GB 50017-2017 第8.1条',
        title: '焊缝连接强度验算',
        description: '对接焊缝/角焊缝强度验算：σ ≤ f_w^t / f_w^f',
        checkRules: [
          { id: 'gb_s6', type: 'stress', params: { weldCheck: true }, errorMessage: '焊缝强度不满足要求', recommendation: '增大焊缝尺寸或改用更高等级焊材' }
        ],
        severity: 'critical',
        category: 'connection'
      },
      {
        id: 'gb50017_16_1',
        section: 'GB 50017-2017 第16.1条',
        title: '疲劳验算（常幅疲劳）',
        description: '常幅疲劳验算：Δσ ≤ [Δσ] = (C/n_f)^(1/β)',
        checkRules: [
          { id: 'gb_s7', type: 'factor_of_safety', params: { fatigueCheck: true }, errorMessage: '疲劳应力幅超出容许值', recommendation: '降低应力幅或提高构造细节等级' }
        ],
        severity: 'critical',
        category: 'fatigue'
      }
    ]
  },
  {
    id: 'gb_t_150_2011',
    name: 'GB/T 150-2011',
    code: 'GB/T 150',
    version: '2011',
    description: '压力容器 (Pressure Vessels)',
    category: 'gb',
    effectiveDate: '2012-03-01',
    requirements: [
      {
        id: 'gb150_3_1',
        section: 'GB/T 150.3 第3.1条',
        title: '内压圆筒壁厚计算',
        description: '计算壁厚 δ = P·D_i/(2·[σ]·φ - P)，φ为焊接接头系数',
        checkRules: [
          { id: 'gb_p1', type: 'factor_of_safety', params: { pressureVesselWall: true }, errorMessage: '圆筒壁厚不满足强度要求', recommendation: '增加壁厚或降低设计压力' }
        ],
        severity: 'critical',
        category: 'pressure'
      },
      {
        id: 'gb150_3_2',
        section: 'GB/T 150.3 第3.2条',
        title: '椭圆形封头壁厚计算',
        description: '标准椭圆封头 δ_h = P·D_i/(2·[σ]·φ - 0.5P)',
        checkRules: [
          { id: 'gb_p2', type: 'factor_of_safety', params: { headWall: true }, errorMessage: '封头壁厚不满足强度要求', recommendation: '增加封头壁厚' }
        ],
        severity: 'critical',
        category: 'pressure'
      },
      {
        id: 'gb150_3_3',
        section: 'GB/T 150.3 第6条',
        title: '开孔补强验算',
        description: '等面积补强法：A_r ≥ A_needed，开孔直径不超过限值',
        checkRules: [
          { id: 'gb_p3', type: 'dimension', params: { reinforcementCheck: true }, errorMessage: '开孔补强面积不足', recommendation: '增加补强板面积或减小开孔直径' }
        ],
        severity: 'critical',
        category: 'reinforcement'
      },
      {
        id: 'gb150_4_1',
        section: 'GB/T 150.4 第10.4条',
        title: '水压试验验算',
        description: '试验压力 P_T = 1.25·P·[σ]_T/[σ]，试验应力不超过 0.9σ_s',
        checkRules: [
          { id: 'gb_p4', type: 'stress', params: { hydroTest: true }, errorMessage: '水压试验应力超过屈服强度的 90%', recommendation: '降低试验压力或增加壁厚' }
        ],
        severity: 'major',
        category: 'testing'
      }
    ]
  }
]

export const TOLERANCE_TABLES: ToleranceTable[] = [
  {
    standard: 'ASME Y14.5',
    dimensionRange: { min: 0, max: 6 },
    toleranceGrade: 'T1',
    values: {
      'IT01': { value: 0.06, unit: 'μm' },
      'IT0': { value: 0.10, unit: 'μm' },
      'IT1': { value: 0.15, unit: 'μm' },
      'IT2': { value: 0.25, unit: 'μm' }
    }
  },
  {
    standard: 'ISO 2768-1',
    dimensionRange: { min: 0, max: 4000 },
    toleranceGrade: 'f (fine)',
    values: {
      'k': { value: 0.020, unit: 'mm' },
      'm': { value: 0.05, unit: 'mm' }
    }
  },
  {
    standard: 'GB 50017',
    dimensionRange: { min: 0, max: 60000 },
    toleranceGrade: '设计强度值 (MPa)',
    values: {
      'Q235_f': { value: 215, unit: 'MPa' },
      'Q345_f': { value: 305, unit: 'MPa' },
      'Q390_f': { value: 335, unit: 'MPa' },
      'Q420_f': { value: 360, unit: 'MPa' },
      'Q460_f': { value: 390, unit: 'MPa' }
    }
  },
  {
    standard: 'GB 50017 (稳定系数φ)',
    dimensionRange: { min: 0, max: 250 },
    toleranceGrade: 'b类截面 (Q345)',
    values: {
      'λ40': { value: 0.897, unit: '-' },
      'λ60': { value: 0.807, unit: '-' },
      'λ80': { value: 0.688, unit: '-' },
      'λ100': { value: 0.563, unit: '-' },
      'λ120': { value: 0.449, unit: '-' },
      'λ150': { value: 0.306, unit: '-' },
      'λ200': { value: 0.164, unit: '-' }
    }
  },
  {
    standard: 'GB/T 150',
    dimensionRange: { min: 0, max: 1000 },
    toleranceGrade: 'Q345R 许用应力 (MPa)',
    values: {
      'T_20': { value: 185, unit: 'MPa' },
      'T_100': { value: 185, unit: 'MPa' },
      'T_150': { value: 185, unit: 'MPa' },
      'T_200': { value: 181, unit: 'MPa' },
      'T_250': { value: 172, unit: 'MPa' },
      'T_300': { value: 163, unit: 'MPa' },
      'T_350': { value: 153, unit: 'MPa' },
      'T_400': { value: 143, unit: 'MPa' }
    }
  }
]

// ============ 存储键 ============

const CHECKS_KEY = 'caelab_compliance_checks'
const REPORTS_KEY = 'caelab_compliance_reports'

// ============ 工具函数 ============

function generateId(): string {
  return `comp_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function getSeverityWeight(severity: CheckSeverity): number {
  switch (severity) {
    case 'critical': return 4
    case 'major': return 3
    case 'minor': return 2
    case 'info': return 1
  }
}

function calculateComplianceLevel(passRate: number, criticalFailed: number): CheckSummary['complianceLevel'] {
  if (criticalFailed > 0) return 'non_compliant'
  if (passRate >= 95) return 'full'
  if (passRate >= 80) return 'major'
  return 'partial'
}

// ============ 主 Composable ============

export function useComplianceChecker() {
  const standards = ref<Standard[]>(STANDARDS)
  const checks = ref<ComplianceCheck[]>([])
  const reports = ref<ComplianceReport[]>([])

  const activeCheck = ref<ComplianceCheck | null>(null)
  const isChecking = ref(false)
  const checkProgress = ref(0)

  // ============ 初始化 ============

  function loadData(): void {
    const storedChecks = getStorage<ComplianceCheck[]>(CHECKS_KEY)
    if (storedChecks) checks.value = storedChecks

    const storedReports = getStorage<ComplianceReport[]>(REPORTS_KEY)
    if (storedReports) reports.value = storedReports
  }

  // ============ 标准管理 ============

  function getStandard(standardId: string): Standard | undefined {
    return standards.value.find(s => s.id === standardId)
  }

  function getStandardsByCategory(category: StandardType): Standard[] {
    return standards.value.filter(s => s.category === category)
  }

  // ============ 合规检查 ============

  /**
   * 启动合规检查
   */
  async function startCheck(
    projectId: string,
    projectName: string,
    standardIds: string[],
    projectData?: {
      dimensions?: Array<{ id: string; value: number; tolerance?: number }>
      materials?: Array<{ id: string; grade: string; certified: boolean }>
      stressResults?: Array<{ component: string; value: number; limit: number }>
      documentation?: Record<string, boolean>
    }
  ): Promise<ComplianceCheck> {
    const check: ComplianceCheck = {
      id: generateId(),
      projectId,
      projectName,
      standard: standardIds.join(', '),
      checkDate: new Date().toISOString(),
      status: 'in_progress',
      results: [],
      summary: {
        totalChecks: 0,
        passed: 0,
        failed: 0,
        warnings: 0,
        notApplicable: 0,
        criticalFailed: 0,
        passRate: 0,
        complianceLevel: 'full'
      }
    }

    checks.value.unshift(check)
    activeCheck.value = check
    isChecking.value = true
    checkProgress.value = 0

    // 收集所有检查项
    const allRequirements: Array<{ standard: Standard; requirement: StandardRequirement }> = []

    for (const standardId of standardIds) {
      const standard = getStandard(standardId)
      if (standard) {
        for (const req of standard.requirements) {
          allRequirements.push({ standard, requirement: req })
        }
      }
    }

    const totalItems = allRequirements.length
    let completedItems = 0

    // 执行每项检查
    for (const { standard, requirement } of allRequirements) {
      const result = await performCheck(requirement, projectData)

      check.results.push(result)
      completedItems++
      checkProgress.value = Math.round((completedItems / totalItems) * 100)
    }

    // 计算汇总
    check.summary = calculateSummary(check.results)
    check.status = 'completed'
    activeCheck.value = null
    isChecking.value = false

    saveChecks()

    return check
  }

  /**
   * 执行单次检查
   */
  async function performCheck(
    requirement: StandardRequirement,
    projectData?: {
      dimensions?: Array<{ id: string; value: number; tolerance?: number }>
      materials?: Array<{ id: string; grade: string; certified: boolean }>
      stressResults?: Array<{ component: string; value: number; limit: number }>
      documentation?: Record<string, boolean>
    }
  ): Promise<CheckResult> {
    // 模拟检查过程
    await new Promise(resolve => setTimeout(resolve, 100))

    const result: CheckResult = {
      id: generateId(),
      requirementId: requirement.id,
      requirementTitle: requirement.title,
      section: requirement.section,
      status: 'passed',
      severity: requirement.severity,
      recommendations: []
    }

    // 根据规则类型执行检查
    for (const rule of requirement.checkRules) {
      switch (rule.type) {
        case 'tolerance':
          if (projectData?.dimensions) {
            for (const dim of projectData.dimensions) {
              if (dim.tolerance !== undefined) {
                const toleranceTable = TOLERANCE_TABLES[0]
                const maxTolerance = Object.values(toleranceTable.values)[0].value
                if (dim.tolerance > maxTolerance) {
                  result.status = 'failed'
                  result.details = `尺寸 ${dim.id} 公差 ${dim.tolerance} 超出标准值 ${maxTolerance}`
                  result.measuredValue = dim.tolerance
                  result.tolerance = maxTolerance
                }
              }
            }
          }
          break

        case 'factor_of_safety':
          if (projectData?.stressResults) {
            for (const stress of projectData.stressResults) {
              const fos = stress.limit / stress.value
              const minFOS = rule.params.minFOS || 3.0
              if (fos < minFOS) {
                result.status = 'failed'
                result.details = `组件 ${stress.component} 安全系数 ${fos.toFixed(2)} 低于要求值 ${minFOS}`
                result.measuredValue = fos
                result.passCriteria = `最小安全系数 ${minFOS}`
                result.recommendations = [rule.recommendation || '增加结构强度']
              }
            }
          }
          break

        case 'material':
          if (projectData?.materials) {
            const nuclearGrade = projectData.materials.filter(m => m.certified)
            if (nuclearGrade.length < projectData.materials.length) {
              result.status = 'failed'
              result.details = '部分材料未获得核级认证'
              result.recommendations = [rule.recommendation || '使用经认证的材料']
            }
          }
          break

        case 'documentation':
          if (projectData?.documentation) {
            const missing = Object.entries(projectData.documentation)
              .filter(([, exists]) => !exists)
              .map(([doc]) => doc)

            if (missing.length > 0) {
              result.status = 'failed'
              result.details = `缺少必需文档: ${missing.join(', ')}`
              result.recommendations = ['补充完整文档']
            }
          }
          break
      }

      // 如果任何规则失败，整体失败
      if (result.status === 'failed') break
    }

    return result
  }

  /**
   * 计算检查汇总
   */
  function calculateSummary(results: CheckResult[]): CheckSummary {
    const summary: CheckSummary = {
      totalChecks: results.length,
      passed: 0,
      failed: 0,
      warnings: 0,
      notApplicable: 0,
      criticalFailed: 0,
      passRate: 0,
      complianceLevel: 'full'
    }

    for (const result of results) {
      switch (result.status) {
        case 'passed':
          summary.passed++
          break
        case 'failed':
          summary.failed++
          if (result.severity === 'critical') {
            summary.criticalFailed++
          }
          break
        case 'warning':
          summary.warnings++
          break
        case 'not_applicable':
        case 'skipped':
          summary.notApplicable++
          break
      }
    }

    const applicableChecks = summary.totalChecks - summary.notApplicable
    if (applicableChecks > 0) {
      summary.passRate = Math.round((summary.passed / applicableChecks) * 100)
    }

    summary.complianceLevel = calculateComplianceLevel(summary.passRate, summary.criticalFailed)

    return summary
  }

  // ============ 报告生成 ============

  /**
   * 生成合规报告
   */
  function generateReport(
    checkId: string,
    options?: {
      includeDetails?: boolean
      includeScreenshots?: boolean
      approvedBy?: string
    }
  ): ComplianceReport | null {
    const check = checks.value.find(c => c.id === checkId)
    if (!check) return null

    const report: ComplianceReport = {
      id: generateId(),
      projectId: check.projectId,
      projectName: check.projectName,
      standards: check.standard.split(','),
      checkDate: check.checkDate,
      summary: check.summary,
      results: options?.includeDetails !== false ? check.results : [],
      approvedBy: options?.approvedBy,
      approvalDate: options?.approvedBy ? new Date().toISOString() : undefined
    }

    reports.value.unshift(report)
    saveReports()

    return report
  }

  /**
   * 导出报告为 PDF 格式（模拟）
   */
  function exportReportAsPDF(reportId: string): Blob {
    const report = reports.value.find(r => r.id === reportId)
    if (!report) return new Blob()

    const content = generateReportText(report, true)
    return new Blob([content], { type: 'text/html' })
  }

  /**
   * 生成报告文本
   */
  function generateReportText(report: ComplianceReport, asHTML: boolean = false): string {
    const divider = asHTML ? '<hr>' : '='.repeat(60)
    const subDivider = asHTML ? '<br>' : '-'.repeat(60)

    let text = `${divider}
COMPLIANCE VERIFICATION REPORT
${divider}

Project: ${report.projectName}
Report ID: ${report.id}
Date: ${new Date(report.checkDate).toLocaleDateString()}
Standards: ${report.standards.join(', ')}

${subDivider}
EXECUTIVE SUMMARY
${subDivider}

Total Checks: ${report.summary.totalChecks}
Passed: ${report.summary.passed}
Failed: ${report.summary.failed}
Warnings: ${report.summary.warnings}
Not Applicable: ${report.summary.notApplicable}

Pass Rate: ${report.summary.passRate}%
Compliance Level: ${report.summary.complianceLevel.toUpperCase()}
${report.summary.criticalFailed > 0 ? `⚠ CRITICAL FAILURES: ${report.summary.criticalFailed}` : ''}

${subDivider}
DETAILED RESULTS
${subDivider}
`

    for (const result of report.results) {
      const statusIcon = result.status === 'passed' ? '✓' : result.status === 'failed' ? '✗' : '⚠'
      const severityLabel = `[${result.severity.toUpperCase()}]`

      text += `
${statusIcon} ${result.requirementTitle} ${severityLabel}
   Section: ${result.section}
   Status: ${result.status.toUpperCase()}
`

      if (result.details) {
        text += `   Details: ${result.details}\n`
      }
      if (result.measuredValue !== undefined) {
        text += `   Measured: ${result.measuredValue}`
        if (result.tolerance !== undefined) {
          text += ` (Tolerance: ${result.tolerance})\n`
        }
      }
      if (result.recommendations && result.recommendations.length > 0) {
        text += `   Recommendations:\n`
        for (const rec of result.recommendations) {
          text += `     - ${rec}\n`
        }
      }
    }

    if (report.approvedBy) {
      text += `
${subDivider}
APPROVAL
${subDivider}
Approved By: ${report.approvedBy}
Approval Date: ${report.approvalDate ? new Date(report.approvalDate).toLocaleDateString() : 'N/A'}
`
    }

    text += `\n${divider}\n`

    return text
  }

  // ============ 合规模板 ============

  /**
   * 获取标准模板
   */
  function getStandardTemplate(standardId: string): {
    sections: string[]
    requiredFields: string[]
    toleranceTable: ToleranceTable | null
  } | null {
    const standard = getStandard(standardId)
    if (!standard) return null

    const sections = standard.requirements.map(r => r.section)
    const requiredFields: string[] = []

    for (const req of standard.requirements) {
      for (const rule of req.checkRules) {
        if (rule.type === 'documentation') {
          requiredFields.push(rule.id)
        }
      }
    }

    const toleranceTable = TOLERANCE_TABLES.find(t => t.standard === standard.code) || null

    return { sections, requiredFields, toleranceTable }
  }

  /**
   * 验证尺寸公差
   */
  function validateTolerance(
    dimension: number,
    tolerance: number,
    standardId: string
  ): { valid: boolean; itGrade?: string; message?: string } {
    const standard = getStandard(standardId)
    if (!standard) return { valid: false, message: 'Unknown standard' }

    const toleranceTable = TOLERANCE_TABLES.find(t => t.standard === standard.code)
    if (!toleranceTable) return { valid: true }

    // 简化验证逻辑
    for (const [grade, { value }] of Object.entries(toleranceTable.values)) {
      if (tolerance <= value) {
        return { valid: true, itGrade: grade }
      }
    }

    return { valid: false, message: 'Tolerance exceeds standard limits' }
  }

  // ============ 查询 ============

  function getCheck(checkId: string): ComplianceCheck | undefined {
    return checks.value.find(c => c.id === checkId)
  }

  function getReport(reportId: string): ComplianceReport | undefined {
    return reports.value.find(r => r.id === reportId)
  }

  function getProjectChecks(projectId: string): ComplianceCheck[] {
    return checks.value.filter(c => c.projectId === projectId)
  }

  // ============ 统计 ============

  const stats = computed(() => ({
    totalChecks: checks.value.length,
    totalReports: reports.value.length,
    averagePassRate: checks.value.length > 0
      ? Math.round(checks.value.reduce((sum, c) => sum + c.summary.passRate, 0) / checks.value.length)
      : 0,
    criticalFailures: checks.value.reduce((sum, c) => sum + c.summary.criticalFailed, 0),
    compliantProjects: checks.value.filter(c => c.summary.complianceLevel === 'full').length
  }))

  // ============ 持久化 ============

  function saveChecks(): void {
    setStorage(CHECKS_KEY, checks.value)
  }

  function saveReports(): void {
    setStorage(REPORTS_KEY, reports.value)
  }

  function deleteCheck(checkId: string): boolean {
    const index = checks.value.findIndex(c => c.id === checkId)
    if (index === -1) return false
    checks.value.splice(index, 1)
    saveChecks()
    return true
  }

  // 初始化
  loadData()

  return {
    // 状态
    standards,
    checks,
    reports,
    activeCheck,
    isChecking,
    checkProgress,

    // 标准管理
    getStandard,
    getStandardsByCategory,
    getStandardTemplate,

    // 合规检查
    startCheck,
    performCheck,
    validateTolerance,

    // 报告
    generateReport,
    exportReportAsPDF,
    generateReportText,

    // 查询
    getCheck,
    getReport,
    getProjectChecks,

    // 统计
    stats,

    // 持久化
    deleteCheck,

    // 数据
    STANDARDS,
    TOLERANCE_TABLES
  }
}
