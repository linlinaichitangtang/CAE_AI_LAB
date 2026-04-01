/**
 * 标准算例库 - 用于验证求解器精度
 * 包含经典梁理论算例，可自动计算理论解并与数值解对比
 */

export interface StandardCase {
  id: string
  name: string
  nameEn: string
  description: string
  category: 'structural' | 'modal' | 'thermal' | 'buckling'
  // 几何参数
  geometry: {
    type: 'beam' | 'plate' | 'cylinder'
    length: number    // m
    width: number     // m
    height: number    // m
    dimension: '2d' | '3d'
  }
  // 网格参数
  mesh: {
    x_div: number
    y_div: number
    z_div: number
    element_type: string
  }
  // 材料参数
  material: {
    name: string
    elastic_modulus: number  // Pa
    poisson_ratio: number
    density: number         // kg/m³
  }
  // 边界条件
  boundaryConditions: {
    fixed_face: string      // 'x_min' | 'y_min' | 'z_min'
    load_type: 'point' | 'uniform' | 'none'
    load_face: string
    load_magnitude: number  // N or Pa
    load_direction: string  // 'Y' | 'X' | 'Z'
  }
  // 理论解
  theoretical: {
    max_displacement: number  // m (理论最大位移)
    max_stress: number        // Pa (理论最大应力)
    formula: string           // 理论公式
    reference: string         // 参考文献
    natural_frequencies?: number[]  // Hz (模态分析：前 N 阶固有频率)
  }
  // 验收标准
  acceptance: {
    displacement_error_limit: number  // % 位移误差上限
    stress_error_limit: number        // % 应力误差上限
    frequency_error_limit?: number    // % 频率误差上限（模态分析）
  }
}

export interface ValidationReport {
  caseId: string
  caseName: string
  timestamp: string
  theoretical: {
    max_displacement: number
    max_stress: number
    natural_frequencies?: number[]
  }
  numerical: {
    max_displacement: number
    max_stress: number
    natural_frequencies?: number[]
  }
  errors: {
    displacement_error: number  // %
    stress_error: number        // %
    frequency_errors?: number[] // % (各阶频率误差)
  }
  acceptance: {
    displacement_error_limit: number
    stress_error_limit: number
    frequency_error_limit?: number
  }
  result: 'PASS' | 'FAIL'
  details: string
}

// ========== 辅助计算函数 ==========

/**
 * 计算矩形截面惯性矩 I = b*h³/12
 */
function calculateMomentOfInertia(width: number, height: number): number {
  return width * Math.pow(height, 3) / 12
}

/**
 * 计算截面模量 W = b*h²/6
 */
function calculateSectionModulus(width: number, height: number): number {
  return width * Math.pow(height, 2) / 6
}

// ========== 标准算例数据 ==========

/**
 * 算例1：悬臂梁 - 端点集中载荷
 *
 * 几何：L=1m, b=0.1m, h=0.1m
 * 材料：钢 E=200GPa, ν=0.3, ρ=7850 kg/m³
 * BC：左端固定，右端顶部 Y 方向集中力 F=1000N
 *
 * 理论解：
 *   I = bh³/12 = 0.1 × 0.1³/12 = 8.333e-6 m⁴
 *   δ = PL³/(3EI) = 1000×1³/(3×200e9×8.333e-6) = 2.0e-3 m
 *   σ = M*c/I = (F*L)*(h/2)/I = (1000×1)×0.05/8.333e-6 = 60e6 Pa
 */
const cantileverPointLoad: StandardCase = {
  id: 'cantilever-point-load',
  name: '悬臂梁 - 端点集中载荷',
  nameEn: 'Cantilever Beam - Tip Point Load',
  description: '经典悬臂梁算例：左端固定，右端自由端施加集中力F=1000N（Y方向），验证结构分析求解器的位移和应力精度。',
  category: 'structural',
  geometry: {
    type: 'beam',
    length: 1.0,    // m
    width: 0.1,     // m (b)
    height: 0.1,    // m (h)
    dimension: '3d'
  },
  mesh: {
    x_div: 20,
    y_div: 2,
    z_div: 2,
    element_type: 'C3D8'
  },
  material: {
    name: '钢 (Steel)',
    elastic_modulus: 200e9,   // Pa (200 GPa)
    poisson_ratio: 0.3,
    density: 7850             // kg/m³
  },
  boundaryConditions: {
    fixed_face: 'x_min',
    load_type: 'point',
    load_face: 'x_max_top',
    load_magnitude: 1000,     // N
    load_direction: 'Y'
  },
  theoretical: {
    max_displacement: 2.0e-3,  // m = 2.0 mm
    max_stress: 60e6,          // Pa = 60 MPa
    formula: 'δ = PL³/(3EI), σ = Mc/I = FL(h/2)/I, I = bh³/12',
    reference: 'Timoshenko, S.P. "Strength of Materials", Part I, Elementary Theory and Problems'
  },
  acceptance: {
    displacement_error_limit: 5,  // %
    stress_error_limit: 5         // %
  }
}

/**
 * 算例2：悬臂梁 - 均布载荷
 *
 * 几何：同上
 * BC：左端固定，顶面均布载荷 q=10000 N/m (= 100000 Pa)
 *
 * 理论解：
 *   I = 8.333e-6 m⁴
 *   q = 10000 N/m (线载荷密度)
 *   δ = qL⁴/(8EI) = 10000×1⁴/(8×200e9×8.333e-6) = 7.5e-4 m
 *   σ_max = qL²(h/2)/I = 10000×1²×0.05/8.333e-6 = 60e6 Pa
 */
const cantileverUniformLoad: StandardCase = {
  id: 'cantilever-uniform-load',
  name: '悬臂梁 - 均布载荷',
  nameEn: 'Cantilever Beam - Uniform Load',
  description: '悬臂梁均布载荷算例：左端固定，顶面施加均布载荷q=10000 N/m（Y方向），验证均布载荷下的位移和应力精度。',
  category: 'structural',
  geometry: {
    type: 'beam',
    length: 1.0,    // m
    width: 0.1,     // m (b)
    height: 0.1,    // m (h)
    dimension: '3d'
  },
  mesh: {
    x_div: 20,
    y_div: 2,
    z_div: 2,
    element_type: 'C3D8'
  },
  material: {
    name: '钢 (Steel)',
    elastic_modulus: 200e9,   // Pa (200 GPa)
    poisson_ratio: 0.3,
    density: 7850             // kg/m³
  },
  boundaryConditions: {
    fixed_face: 'x_min',
    load_type: 'uniform',
    load_face: 'top',
    load_magnitude: 100000,   // Pa (= 10000 N/m / 0.1m width)
    load_direction: 'Y'
  },
  theoretical: {
    max_displacement: 7.5e-4,  // m = 0.75 mm
    max_stress: 60e6,          // Pa = 60 MPa
    formula: 'δ = qL⁴/(8EI), σ_max = qL²c/(2I), I = bh³/12',
    reference: 'Timoshenko, S.P. "Strength of Materials", Part I, Elementary Theory and Problems'
  },
  acceptance: {
    displacement_error_limit: 5,  // %
    stress_error_limit: 5         // %
  }
}

/**
 * 算例3：简支梁 - 中心集中载荷
 *
 * 几何：L=1m, b=0.1m, h=0.1m
 * BC：两端铰支（Y方向固定），中心点 Y 方向集中力 F=1000N
 *
 * 理论解：
 *   I = 8.333e-6 m⁴
 *   δ = FL³/(48EI) = 1000×1³/(48×200e9×8.333e-6) = 1.25e-4 m
 *   σ_max = FLc/(4I) = 1000×1×0.05/(4×8.333e-6) = 15e6 Pa
 */
const simplySupportedPointLoad: StandardCase = {
  id: 'simply-supported-center-load',
  name: '简支梁 - 中心集中载荷',
  nameEn: 'Simply Supported Beam - Center Point Load',
  description: '简支梁中心集中载荷算例：两端铰支（Y方向固定），跨中施加集中力F=1000N（Y方向），验证简支梁的位移和应力精度。',
  category: 'structural',
  geometry: {
    type: 'beam',
    length: 1.0,    // m
    width: 0.1,     // m (b)
    height: 0.1,    // m (h)
    dimension: '3d'
  },
  mesh: {
    x_div: 20,
    y_div: 2,
    z_div: 2,
    element_type: 'C3D8'
  },
  material: {
    name: '钢 (Steel)',
    elastic_modulus: 200e9,   // Pa (200 GPa)
    poisson_ratio: 0.3,
    density: 7850             // kg/m³
  },
  boundaryConditions: {
    fixed_face: 'x_min_x_max_y',
    load_type: 'point',
    load_face: 'center_top',
    load_magnitude: 1000,     // N
    load_direction: 'Y'
  },
  theoretical: {
    max_displacement: 1.25e-4,  // m = 0.125 mm
    max_stress: 15e6,           // Pa = 15 MPa
    formula: 'δ = FL³/(48EI), σ_max = FLc/(4I), I = bh³/12',
    reference: 'Timoshenko, S.P. "Strength of Materials", Part I, Elementary Theory and Problems'
  },
  acceptance: {
    displacement_error_limit: 5,  // %
    stress_error_limit: 5         // %
  }
}

/**
 * 算例4：简支矩形板 - 均布载荷（Kirchhoff 薄板理论）
 *
 * 几何：a=1m, b=1m, h=0.01m
 * 材料：钢 E=200GPa, ν=0.3, ρ=7850 kg/m³
 * BC：四边简支，顶面均布载荷 q=10000 Pa (Z方向向下)
 *
 * 理论解（Navier 级数解，四边简支方板）：
 *   D = Eh³/(12(1-ν²)) = 200e9 × 0.01³ / (12 × 0.91) = 18315.02 N·m
 *   w_max = α × q × a⁴ / D, α = 0.00406 (方板 Navier 解)
 *   w_max = 0.00406 × 10000 × 1.0 / 18315.02 = 0.002217 m ≈ 2.22 mm
 *   σ_max = β × q × a² / h², β ≈ 0.2874 (方板底面中心)
 *   σ_max = 0.2874 × 10000 × 1.0 / 0.01² = 28.74 MPa
 */
const simplySupportedPlateUniform: StandardCase = {
  id: 'simply-supported-plate-uniform',
  name: '简支矩形板 - 均布载荷',
  nameEn: 'Simply Supported Plate - Uniform Load',
  description: '四边简支矩形板受均布载荷，基于 Kirchhoff 薄板理论验证。板尺寸 1m×1m×0.01m，顶面均布载荷 q=10000 Pa。',
  category: 'structural',
  geometry: {
    type: 'plate',
    length: 1.0,    // m (X方向, a)
    width: 1.0,     // m (Y方向, b)
    height: 0.01,   // m (厚度, h)
    dimension: '3d'
  },
  mesh: {
    x_div: 20,
    y_div: 20,
    z_div: 2,
    element_type: 'C3D8R'
  },
  material: {
    name: '钢 (Steel)',
    elastic_modulus: 200e9,   // Pa (200 GPa)
    poisson_ratio: 0.3,
    density: 7850             // kg/m³
  },
  boundaryConditions: {
    fixed_face: 'z_min',      // 底面简支（Z方向固定）
    load_type: 'uniform',
    load_face: 'z_max',       // 顶面均布载荷
    load_magnitude: 10000,    // Pa
    load_direction: 'Z'       // Z方向（向下）
  },
  theoretical: {
    max_displacement: 0.002217,  // m ≈ 2.22 mm (中心最大挠度，Navier 解)
    max_stress: 28.74e6,         // Pa = 28.74 MPa (中心最大弯曲应力)
    formula: 'w_max = 0.00406 × q × a⁴ / D, D = Eh³/(12(1-ν²)), σ_max = 0.2874 × q × a² / h²',
    reference: 'Timoshenko, Theory of Plates and Shells, Eq. 29'
  },
  acceptance: {
    displacement_error_limit: 10,  // % (薄板理论误差稍大，放宽到 10%)
    stress_error_limit: 10         // %
  }
}

/**
 * 算例5：悬臂梁 - 模态分析（Euler-Bernoulli 梁理论）
 *
 * 几何：L=1m, b=0.05m, h=0.01m
 * 材料：钢 E=200GPa, ν=0.3, ρ=7850 kg/m³
 * BC：左端固定，无外载荷
 *
 * 理论解（Euler-Bernoulli 悬臂梁固有频率）：
 *   I = bh³/12 = 0.05 × 0.01³/12 = 4.167e-9 m⁴
 *   A = bh = 0.05 × 0.01 = 5e-4 m²
 *   EI = 200e9 × 4.167e-9 = 833.33 N·m²
 *   ρA = 7850 × 5e-4 = 3.925 kg/m
 *   √(EI/ρA) = √(833.33/3.925) = √212.32 = 14.57 m²/s
 *   f_n = (β_n² / (2πL²)) × √(EI/ρA)
 *   β₁=1.875, β₂=4.694, β₃=7.855, β₄=10.996, β₅=14.137
 *   f₁ = (1.875² / 6.283) × 14.57 = 8.15 Hz
 *   f₂ = (4.694² / 6.283) × 14.57 = 51.10 Hz
 *   f₃ = (7.855² / 6.283) × 14.57 = 143.1 Hz
 *   f₄ = (10.996² / 6.283) × 14.57 = 280.4 Hz
 *   f₅ = (14.137² / 6.283) × 14.57 = 463.4 Hz
 */
const cantileverModal: StandardCase = {
  id: 'cantilever-modal',
  name: '悬臂梁 - 模态分析',
  nameEn: 'Cantilever Beam - Modal Analysis',
  description: '悬臂梁前 5 阶固有频率，基于 Euler-Bernoulli 梁理论验证。L=1m, b=0.05m, h=0.01m，左端固定。',
  category: 'modal',
  geometry: {
    type: 'beam',
    length: 1.0,    // m
    width: 0.05,    // m (b)
    height: 0.01,   // m (h)
    dimension: '3d'
  },
  mesh: {
    x_div: 30,
    y_div: 2,
    z_div: 2,
    element_type: 'C3D8'
  },
  material: {
    name: '钢 (Steel)',
    elastic_modulus: 200e9,   // Pa (200 GPa)
    poisson_ratio: 0.3,
    density: 7850             // kg/m³
  },
  boundaryConditions: {
    fixed_face: 'x_min',
    load_type: 'none',
    load_face: '',
    load_magnitude: 0,
    load_direction: 'Y'
  },
  theoretical: {
    max_displacement: 0,       // 模态分析无位移
    max_stress: 0,             // 模态分析无应力
    natural_frequencies: [8.15, 51.10, 143.1, 280.4, 463.4], // Hz, 前5阶
    formula: 'f_n = (β_n² / 2πL²) × √(EI / ρA), β₁=1.875, β₂=4.694, β₃=7.855, β₄=10.996, β₅=14.137',
    reference: 'Rao, Mechanical Vibrations, 6th Ed., Table 8.4'
  },
  acceptance: {
    displacement_error_limit: 5,  // %
    stress_error_limit: 5,        // %
    frequency_error_limit: 5      // % 频率误差限
  }
}

// ========== 标准算例列表 ==========

export const standardCases: StandardCase[] = [
  cantileverPointLoad,
  cantileverUniformLoad,
  simplySupportedPointLoad,
  simplySupportedPlateUniform,
  cantileverModal
]

// ========== 导出函数 ==========

/**
 * 根据 ID 获取标准算例
 */
export function getCaseById(id: string): StandardCase | undefined {
  return standardCases.find(c => c.id === id)
}

/**
 * 计算理论最大位移
 * 根据算例类型使用不同的公式
 */
export function calculateTheoreticalDisplacement(stdCase: StandardCase): number {
  const { geometry, material, boundaryConditions } = stdCase
  const E = material.elastic_modulus
  const L = geometry.length
  const b = geometry.width
  const h = geometry.height
  const I = calculateMomentOfInertia(b, h)

  switch (stdCase.id) {
    case 'cantilever-point-load': {
      // δ = PL³/(3EI)
      const P = boundaryConditions.load_magnitude
      return (P * Math.pow(L, 3)) / (3 * E * I)
    }
    case 'cantilever-uniform-load': {
      // δ = qL⁴/(8EI)  (q 为线载荷 N/m)
      const q = boundaryConditions.load_magnitude * b // Pa * width = N/m
      return (q * Math.pow(L, 4)) / (8 * E * I)
    }
    case 'simply-supported-center-load': {
      // δ = FL³/(48EI)
      const F = boundaryConditions.load_magnitude
      return (F * Math.pow(L, 3)) / (48 * E * I)
    }
    case 'simply-supported-plate-uniform': {
      // Kirchhoff 薄板理论：w_max = α × q × a⁴ / D
      // D = Eh³/(12(1-ν²)), α = 0.00406 (四边简支方板)
      const nu = material.poisson_ratio
      const h = geometry.height
      const a = geometry.length
      const q = boundaryConditions.load_magnitude
      const D = (E * Math.pow(h, 3)) / (12 * (1 - nu * nu))
      const alpha = 0.00406
      return alpha * q * Math.pow(a, 4) / D
    }
    default: {
      // 通用公式：使用预设的理论值
      return stdCase.theoretical.max_displacement
    }
  }
}

/**
 * 计算理论最大应力
 * 根据算例类型使用不同的公式
 */
export function calculateTheoreticalStress(stdCase: StandardCase): number {
  const { geometry, material, boundaryConditions } = stdCase
  const L = geometry.length
  const b = geometry.width
  const h = geometry.height
  const c = h / 2
  const I = calculateMomentOfInertia(b, h)

  switch (stdCase.id) {
    case 'cantilever-point-load': {
      // σ = Mc/I = (F*L)*(h/2)/I
      const F = boundaryConditions.load_magnitude
      const M = F * L
      return (M * c) / I
    }
    case 'cantilever-uniform-load': {
      // σ_max = qL²c/(2I)  (q 为线载荷 N/m)
      const q = boundaryConditions.load_magnitude * b
      return (q * Math.pow(L, 2) * c) / (2 * I)
    }
    case 'simply-supported-center-load': {
      // σ_max = FLc/(4I)
      const F = boundaryConditions.load_magnitude
      return (F * L * c) / (4 * I)
    }
    case 'simply-supported-plate-uniform': {
      // Kirchhoff 薄板理论：σ_max = β × q × a² / h²
      // β = 0.2874 (四边简支方板底面中心弯曲应力系数)
      const beta = 0.2874
      const a = geometry.length
      const h = geometry.height
      const q = boundaryConditions.load_magnitude
      return beta * q * (a * a) / (h * h)
    }
    default: {
      return stdCase.theoretical.max_stress
    }
  }
}

/**
 * 计算百分比误差
 * @param numerical 数值解
 * @param theoretical 理论解
 * @returns 百分比误差（绝对值）
 */
export function calculateError(numerical: number, theoretical: number): number {
  if (theoretical === 0) {
    return numerical === 0 ? 0 : Infinity
  }
  return Math.abs((numerical - theoretical) / theoretical) * 100
}

/**
 * 计算理论固有频率（Euler-Bernoulli 梁理论）
 * @param stdCase 标准算例
 * @returns 各阶固有频率数组 (Hz)，若无则返回空数组
 */
export function calculateTheoreticalFrequencies(stdCase: StandardCase): number[] {
  if (!stdCase.theoretical.natural_frequencies || stdCase.theoretical.natural_frequencies.length === 0) {
    return []
  }

  const { geometry, material } = stdCase
  const E = material.elastic_modulus
  const rho = material.density
  const L = geometry.length
  const b = geometry.width
  const h = geometry.height
  const I = calculateMomentOfInertia(b, h)
  const A = b * h

  // Euler-Bernoulli 悬臂梁频率系数
  const betaCoefficients = [1.875, 4.694, 7.855, 10.996, 14.137]

  const sqrtEIrhoA = Math.sqrt((E * I) / (rho * A))

  return stdCase.theoretical.natural_frequencies.map((_, index) => {
    if (index < betaCoefficients.length) {
      const beta = betaCoefficients[index]
      return (beta * beta) / (2 * Math.PI * L * L) * sqrtEIrhoA
    }
    return 0
  })
}

/**
 * 生成验证报告
 * @param caseId 算例 ID
 * @param numericalDisp 数值解最大位移 (m)
 * @param numericalStress 数值解最大应力 (Pa)
 * @param numericalFrequencies 数值解固有频率 (Hz)，模态分析时传入
 * @returns 验证报告
 */
export function generateValidationReport(
  caseId: string,
  numericalDisp: number,
  numericalStress: number,
  numericalFrequencies?: number[]
): ValidationReport {
  const stdCase = getCaseById(caseId)
  if (!stdCase) {
    throw new Error(`未找到标准算例: ${caseId}`)
  }

  const theoreticalDisp = calculateTheoreticalDisplacement(stdCase)
  const theoreticalStress = calculateTheoreticalStress(stdCase)
  const dispError = calculateError(numericalDisp, theoreticalDisp)
  const stressError = calculateError(numericalStress, theoreticalStress)

  // 模态分析：计算频率误差
  const isModal = stdCase.category === 'modal'
  const theoreticalFreqs = isModal ? calculateTheoreticalFrequencies(stdCase) : []
  const frequencyErrors: number[] = []
  let freqPass = true

  if (isModal && numericalFrequencies && numericalFrequencies.length > 0 && theoreticalFreqs.length > 0) {
    for (let i = 0; i < Math.min(numericalFrequencies.length, theoreticalFreqs.length); i++) {
      const err = calculateError(numericalFrequencies[i], theoreticalFreqs[i])
      frequencyErrors.push(err)
      if (err > (stdCase.acceptance.frequency_error_limit ?? 5)) {
        freqPass = false
      }
    }
  }

  const dispPass = dispError <= stdCase.acceptance.displacement_error_limit
  const stressPass = stressError <= stdCase.acceptance.stress_error_limit
  const overallPass = isModal ? freqPass : (dispPass && stressPass)

  let details = ''
  if (isModal) {
    // 模态分析报告
    const freqLimit = stdCase.acceptance.frequency_error_limit ?? 5
    if (overallPass) {
      const freqDetails = frequencyErrors.map((e, i) =>
        `第${i + 1}阶 ${e.toFixed(2)}%`
      ).join('，')
      details = `模态验证通过。各阶频率误差：${freqDetails}（限值 ${freqLimit}%）。`
    } else {
      const failures: string[] = []
      frequencyErrors.forEach((e, i) => {
        if (e > freqLimit) {
          failures.push(`第${i + 1}阶频率误差 ${e.toFixed(2)}% 超过限值 ${freqLimit}%`)
        }
      })
      details = `模态验证未通过。${failures.join('；')}。`
    }
  } else {
    // 结构分析报告
    if (overallPass) {
      details = `验证通过。位移误差 ${dispError.toFixed(2)}%（限值 ${stdCase.acceptance.displacement_error_limit}%），应力误差 ${stressError.toFixed(2)}%（限值 ${stdCase.acceptance.stress_error_limit}%）。`
    } else {
      const failures: string[] = []
      if (!dispPass) {
        failures.push(`位移误差 ${dispError.toFixed(2)}% 超过限值 ${stdCase.acceptance.displacement_error_limit}%`)
      }
      if (!stressPass) {
        failures.push(`应力误差 ${stressError.toFixed(2)}% 超过限值 ${stdCase.acceptance.stress_error_limit}%`)
      }
      details = `验证未通过。${failures.join('；')}。`
    }
  }

  return {
    caseId: stdCase.id,
    caseName: stdCase.name,
    timestamp: new Date().toISOString(),
    theoretical: {
      max_displacement: theoreticalDisp,
      max_stress: theoreticalStress,
      natural_frequencies: theoreticalFreqs.length > 0 ? theoreticalFreqs : undefined
    },
    numerical: {
      max_displacement: numericalDisp,
      max_stress: numericalStress,
      natural_frequencies: numericalFrequencies && numericalFrequencies.length > 0 ? numericalFrequencies : undefined
    },
    errors: {
      displacement_error: dispError,
      stress_error: stressError,
      frequency_errors: frequencyErrors.length > 0 ? frequencyErrors : undefined
    },
    acceptance: {
      displacement_error_limit: stdCase.acceptance.displacement_error_limit,
      stress_error_limit: stdCase.acceptance.stress_error_limit,
      frequency_error_limit: stdCase.acceptance.frequency_error_limit
    },
    result: overallPass ? 'PASS' : 'FAIL',
    details
  }
}

/**
 * 获取误差等级颜色类名
 * 绿色 < 2%, 黄色 < 5%, 红色 >= 5%
 */
export function getErrorLevelClass(errorPercent: number): string {
  if (errorPercent < 2) {
    return 'text-green-600 bg-green-50'
  } else if (errorPercent < 5) {
    return 'text-yellow-600 bg-yellow-50'
  } else {
    return 'text-red-600 bg-red-50'
  }
}

/**
 * 获取误差等级文字
 */
export function getErrorLevelText(errorPercent: number): string {
  if (errorPercent < 2) {
    return '优秀'
  } else if (errorPercent < 5) {
    return '合格'
  } else {
    return '超标'
  }
}
