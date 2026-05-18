/**
 * V4.0 经典案例模板库
 * 引导式仿真向导 — 内置 50 个教学案例（首批 8 个核心案例）
 */

export type TemplateCategory = 'structural' | 'materials' | 'fatigue' | 'thermal' | 'modal' | 'buckling' | 'composite' | 'optimization' | 'steel'
export type DifficultyLevel = 'beginner' | 'intermediate' | 'advanced'

/** 向导步骤 */
export interface WizardStep {
  id: string
  title: string
  description: string
  icon: string
  component: 'geometry' | 'material' | 'mesh' | 'boundary' | 'solver' | 'review' | 'result'
  guidance?: string
  tips?: string[]
  validation?: (values: Record<string, unknown>) => boolean | string
}

/** 几何配置 */
export interface GeometryConfig {
  type: 'beam' | 'plate' | 'shell' | 'solid' | 'custom'
  params: Record<string, number>
  unit: 'mm' | 'm' | 'cm'
  preview?: string
}

/** 材料配置 */
export interface MaterialConfig {
  name: string
  category: string
  elasticModulus: number
  poissonsRatio: number
  density: number
  yieldStrength?: number
  thermalExpansion?: number
}

/** 网格配置 */
export interface MeshConfig {
  elementType: 'tet4' | 'tet10' | 'hex8' | 'hex20'
  globalSize: number
  localRefinement?: Array<{
    region: string
    size: number
    description: string
  }>
  qualityTarget: number
}

/** 边界条件 */
export interface BoundaryCondition {
  type: 'fixed' | 'displacement' | 'force' | 'pressure' | 'temperature' | 'symmetry'
  region: string
  values: Record<string, number>
  description: string
}

/** 求解器配置 */
export interface SolverConfig {
  analysisType: 'static' | 'modal' | 'thermal' | 'buckling' | 'fatigue'
  solverType: 'direct' | 'iterative'
  maxIterations?: number
  tolerance?: number
}

/** 仿真模板 */
export interface SimulationTemplate {
  id: string
  name: string
  description: string
  category: TemplateCategory
  difficulty: DifficultyLevel
  estimatedTime: number // 分钟
  tags: string[]
  image?: string
  steps: WizardStep[]
  geometry: GeometryConfig
  material: MaterialConfig
  mesh: MeshConfig
  boundaryConditions: BoundaryCondition[]
  solver: SolverConfig
  expectedResults?: {
    maxStress?: number
    maxDisplacement?: number
    safetyFactor?: number
    firstFrequency?: number
  }
}

// ============================================================================
// 核心模板 1：悬臂梁静强度分析（新手入门首选）
// ============================================================================

export const cantileverBeamTemplate: SimulationTemplate = {
  id: 'cantilever-beam-static',
  name: '悬臂梁静强度分析',
  description: '经典的悬臂梁在自由端受集中力作用，学习固定约束、点载荷施加、应力与位移结果查看。适合 CAE 入门第一课。',
  category: 'structural',
  difficulty: 'beginner',
  estimatedTime: 20,
  tags: ['静力学', '悬臂梁', '入门', '教学'],
  steps: [
    {
      id: 'intro',
      title: '案例介绍',
      description: '悬臂梁一端固定，另一端受垂直向下的集中力 F = 1000 N。梁长度 L = 1000 mm，截面为矩形 50×100 mm。材料为结构钢 Q235。',
      icon: '📖',
      component: 'geometry',
      guidance: '这是结构力学中最经典的案例。你将学习：如何创建几何体、设置材料属性、划分网格、施加边界条件、运行求解器、查看结果。',
      tips: [
        '悬臂梁的最大应力出现在固定端上表面',
        '理论解：σ_max = 6FL/(bh²)，可用来验证仿真结果',
        '最大挠度在自由端，理论值：δ_max = FL³/(3EI)'
      ]
    },
    {
      id: 'geometry',
      title: '创建几何',
      description: '创建矩形截面悬臂梁，长度 1000 mm，宽度 50 mm，高度 100 mm。',
      icon: '📐',
      component: 'geometry',
      guidance: '悬臂梁是细长结构，可以使用实体建模。注意长宽比不要过大，否则求解可能出现问题。',
      tips: [
        '可以用拉伸命令生成：先画矩形截面，再沿长度方向拉伸',
        '截面方向影响应力分布，宽度方向对应 y 轴，高度方向对应 z 轴'
      ]
    },
    {
      id: 'material',
      title: '设置材料',
      description: '选择结构钢 Q235，弹性模量 206 GPa，泊松比 0.3。',
      icon: '🔧',
      component: 'material',
      guidance: '材料属性是仿真准确性的基础。对于结构钢，弹性模量和泊松比是最关键的两个参数。',
      tips: [
        'Q235 的屈服强度为 235 MPa，仿真结果应远小于此值',
        '如果材料库中没有 Q235，可以选择类似材料如 A36 或手动输入参数'
      ]
    },
    {
      id: 'mesh',
      title: '划分网格',
      description: '全局网格尺寸 20 mm，在固定端附近局部加密到 10 mm。',
      icon: '🔲',
      component: 'mesh',
      guidance: '网格质量直接影响结果精度。固定端附近应力梯度大，需要更细的网格。',
      tips: [
        '对于初学者，建议先用较粗的网格快速验证设置，再逐步加密',
        '悬臂梁的截面方向至少分 3-4 层网格',
        '可以使用网格收敛性检查：比较不同网格密度的结果'
      ]
    },
    {
      id: 'boundary',
      title: '施加边界条件',
      description: '固定端施加全约束（固定所有自由度），自由端施加垂直向下的集中力 1000 N。',
      icon: '⚓',
      component: 'boundary',
      guidance: '边界条件是仿真中最容易出错的环节。固定端意味着完全不能移动和转动。',
      tips: [
        '集中力方向向下（-y 方向），大小 1000 N',
        '固定约束应施加在梁的整个端面，不能仅施加在端面的边缘',
        '可以用"显示边界条件"功能检查是否正确施加'
      ]
    },
    {
      id: 'solver',
      title: '运行求解',
      description: '选择静力分析，直接求解器，运行计算。',
      icon: '▶️',
      component: 'solver',
      guidance: '静力分析是最基础的仿真类型，适用于载荷不随时间变化的情况。',
      tips: [
        '静力分析默认使用直接求解器，对于小模型速度最快',
        '如果求解失败，检查是否有足够的约束（刚体位移）',
        '收敛性通常不是问题，但如果材料非线性则需要关注'
      ]
    },
    {
      id: 'review',
      title: '检查与验证',
      description: '检查结果合理性，与理论解对比。',
      icon: '✅',
      component: 'review',
      guidance: '验证是仿真最重要的一步。用理论解或经验值检查结果是否在合理范围内。',
      tips: [
        '最大应力理论值 ≈ 120 MPa（在固定端上表面）',
        '最大位移理论值 ≈ 1.45 mm（在自由端）',
        '如果结果偏差超过 20%，检查网格密度和边界条件'
      ]
    },
    {
      id: 'result',
      title: '查看结果',
      description: '查看应力云图、位移云图，导出报告。',
      icon: '📊',
      component: 'result',
      guidance: '后处理帮助你理解结构的受力状态。重点关注高应力区域和最大变形位置。',
      tips: [
        'von Mises 应力用于判断塑性屈服（与屈服强度对比）',
        '位移云图显示变形后的形状（通常需要放大变形比例）',
        '可以沿梁长度方向绘制应力/位移曲线'
      ]
    }
  ],
  geometry: {
    type: 'beam',
    params: { length: 1000, width: 50, height: 100 },
    unit: 'mm'
  },
  material: {
    name: 'Q235',
    category: 'steel',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 235e6
  },
  mesh: {
    elementType: 'tet10',
    globalSize: 20,
    localRefinement: [
      { region: 'fixed_end', size: 10, description: '固定端附近应力集中，需要加密' }
    ],
    qualityTarget: 0.7
  },
  boundaryConditions: [
    {
      type: 'fixed',
      region: 'fixed_end_face',
      values: {},
      description: '固定端完全约束所有自由度'
    },
    {
      type: 'force',
      region: 'free_end_face_center',
      values: { fy: -1000 },
      description: '自由端施加垂直向下集中力 1000 N'
    }
  ],
  solver: {
    analysisType: 'static',
    solverType: 'direct'
  },
  expectedResults: {
    maxStress: 120e6,
    maxDisplacement: 1.45e-3,
    safetyFactor: 1.96
  }
}

// ============================================================================
// 核心模板 2：简支梁中点加载
// ============================================================================

export const simplySupportedBeamTemplate: SimulationTemplate = {
  id: 'simply-supported-beam',
  name: '简支梁中点加载',
  description: '两端简支的梁在中点受集中力，学习简支约束、对称性利用、中点挠度计算。',
  category: 'structural',
  difficulty: 'beginner',
  estimatedTime: 25,
  tags: ['静力学', '简支梁', '入门', '教学', '对称性'],
  steps: [
    {
      id: 'intro',
      title: '案例介绍',
      description: '简支梁跨度 L = 2000 mm，截面矩形 100×200 mm，中点受集中力 F = 5000 N。',
      icon: '📖',
      component: 'geometry',
      guidance: '简支梁是工程中最常见的结构形式之一。本案例学习简支约束和对称性简化。',
      tips: [
        '简支约束：约束垂直位移，但允许水平位移和转动',
        '利用对称性可以将模型简化为一半，大大节省计算时间',
        '理论解：中点挠度 δ = FL³/(48EI)'
      ]
    },
    {
      id: 'geometry',
      title: '创建几何',
      description: '创建矩形截面简支梁，长度 2000 mm，宽度 100 mm，高度 200 mm。',
      icon: '📐',
      component: 'geometry',
      guidance: '由于对称性，可以只建一半模型（长度 1000 mm），在对称面施加对称约束。',
      tips: [
        '对称约束：限制垂直于对称面的位移，允许面内位移和转动',
        '一半模型的中点载荷应为原载荷的一半（2500 N）'
      ]
    },
    {
      id: 'material',
      title: '设置材料',
      description: '选择结构钢 Q345，弹性模量 206 GPa，泊松比 0.3。',
      icon: '🔧',
      component: 'material',
      guidance: 'Q345 比 Q235 强度更高，常用于桥梁和建筑结构。',
      tips: ['Q345 屈服强度 345 MPa，适用于更高载荷的场合']
    },
    {
      id: 'mesh',
      title: '划分网格',
      description: '全局网格尺寸 30 mm，中点区域加密到 15 mm。',
      icon: '🔲',
      component: 'mesh',
      guidance: '中点处弯矩最大，应力梯度最大，需要更细的网格。',
      tips: [
        '对称模型在中点（对称面）附近应力集中',
        '至少沿高度方向分 5-6 层网格'
      ]
    },
    {
      id: 'boundary',
      title: '施加边界条件',
      description: '一端简支（约束 y、z 位移），另一端滚动（仅约束 y 位移），中点施加集中力。',
      icon: '⚓',
      component: 'boundary',
      guidance: '简支约束不能同时约束两个支座的 x 方向位移，否则会产生过约束。',
      tips: [
        '支座 A：固定 y 和 z（Ux 自由，防止刚体位移）',
        '支座 B：固定 y（允许 x 方向滑动）',
        '如果使用对称模型，对称面施加对称约束'
      ]
    },
    {
      id: 'solver',
      title: '运行求解',
      description: '静力分析，运行计算。',
      icon: '▶️',
      component: 'solver',
      guidance: '简支梁是线性问题，直接求解器即可。',
      tips: ['对于线性静力分析，求解时间主要取决于网格规模']
    },
    {
      id: 'review',
      title: '检查与验证',
      description: '与理论解对比。',
      icon: '✅',
      component: 'review',
      guidance: '简支梁中点最大挠度理论值是经典材料力学公式。',
      tips: [
        '最大应力理论值 ≈ 112.5 MPa',
        '中点挠度理论值 ≈ 0.76 mm',
        '应力沿截面高度线性分布（上压下拉）'
      ]
    },
    {
      id: 'result',
      title: '查看结果',
      description: '查看应力分布、挠度曲线。',
      icon: '📊',
      component: 'result',
      guidance: '简支梁的应力分布沿长度方向呈三角形，中点最大。',
      tips: [
        '可以沿梁长度方向绘制弯矩图和挠度曲线',
        '比较对称模型和全模型的结果，验证对称性简化是否正确'
      ]
    }
  ],
  geometry: {
    type: 'beam',
    params: { length: 2000, width: 100, height: 200 },
    unit: 'mm'
  },
  material: {
    name: 'Q345',
    category: 'steel',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6
  },
  mesh: {
    elementType: 'tet10',
    globalSize: 30,
    localRefinement: [
      { region: 'mid_span', size: 15, description: '中点处弯矩最大，需要加密' }
    ],
    qualityTarget: 0.7
  },
  boundaryConditions: [
    {
      type: 'fixed',
      region: 'support_a',
      values: { uy: 0, uz: 0 },
      description: '支座 A：约束 y 和 z 位移'
    },
    {
      type: 'displacement',
      region: 'support_b',
      values: { uy: 0 },
      description: '支座 B：仅约束 y 位移（滚动支座）'
    },
    {
      type: 'force',
      region: 'mid_point',
      values: { fy: -5000 },
      description: '中点集中力 5000 N 向下'
    }
  ],
  solver: {
    analysisType: 'static',
    solverType: 'direct'
  },
  expectedResults: {
    maxStress: 112.5e6,
    maxDisplacement: 0.76e-3,
    safetyFactor: 3.07
  }
}

// ============================================================================
// 核心模板 3：开孔板拉伸（应力集中）
// ============================================================================

export const perforatedPlateTemplate: SimulationTemplate = {
  id: 'perforated-plate-tension',
  name: '开孔板拉伸 — 应力集中',
  description: '中心开孔的矩形板受单向拉伸，学习应力集中系数、圆孔附近的应力分布。',
  category: 'structural',
  difficulty: 'intermediate',
  estimatedTime: 30,
  tags: ['应力集中', '孔板', '进阶', '教学'],
  steps: [
    {
      id: 'intro',
      title: '案例介绍',
      description: '矩形板 200×100×5 mm，中心圆孔直径 20 mm，受 x 方向均匀拉伸应力 100 MPa。',
      icon: '📖',
      component: 'geometry',
      guidance: '开孔板是应力集中的典型案例。理论应力集中系数 K_t ≈ 3（对于无限大板中心圆孔）。',
      tips: [
        '应力集中系数 K_t = σ_max / σ_nominal',
        '对于中心圆孔，K_t ≈ 3（与孔径和板宽比有关）',
        '最大应力出现在孔边，与拉伸方向垂直的直径两端'
      ]
    },
    {
      id: 'geometry',
      title: '创建几何',
      description: '创建带中心圆孔的矩形板。',
      icon: '📐',
      component: 'geometry',
      guidance: '可以用布尔运算：先生成矩形板，再生成圆柱体，用板减去圆柱体得到孔。',
      tips: [
        '孔直径与板宽比 d/W = 20/100 = 0.2，有限宽度修正系数约 2.5',
        '板的长度应足够大（L/W > 2），避免边界效应影响应力集中区'
      ]
    },
    {
      id: 'material',
      title: '设置材料',
      description: '选择铝合金 AA 6061-T6。',
      icon: '🔧',
      component: 'material',
      guidance: '铝合金常用于航空结构，对应力集中敏感。',
      tips: ['6061-T6 屈服强度 276 MPa，注意应力集中可能导致局部屈服']
    },
    {
      id: 'mesh',
      title: '划分网格',
      description: '全局 5 mm，孔边局部加密到 1 mm。',
      icon: '🔲',
      component: 'mesh',
      guidance: '孔边是应力集中区域，网格必须非常细密才能捕捉峰值应力。',
      tips: [
        '孔边至少分 8-10 层网格',
        '可以使用扫掠网格（swept mesh）提高质量',
        '比较不同网格密度的 K_t，验证收敛性'
      ]
    },
    {
      id: 'boundary',
      title: '施加边界条件',
      description: '一端固定 x 位移，另一端施加均匀拉伸应力 100 MPa。',
      icon: '⚓',
      component: 'boundary',
      guidance: '均匀拉伸应力可以通过在端面施加均匀分布的力来实现。',
      tips: [
        '应力 σ = F/A，端面面积 A = 厚度 × (宽度 - 孔径) = 5 × 80 = 400 mm²',
        '总拉力 F = σ × A = 100 × 400 = 40000 N',
        '也可以直接施加均匀压力（更精确）'
      ]
    },
    {
      id: 'solver',
      title: '运行求解',
      description: '静力分析。',
      icon: '▶️',
      component: 'solver',
      guidance: '由于孔边网格很密，模型规模可能较大。',
      tips: ['如果求解时间太长，可以先尝试较粗的网格验证设置']
    },
    {
      id: 'review',
      title: '检查与验证',
      description: '计算应力集中系数，与理论值对比。',
      icon: '✅',
      component: 'review',
      guidance: '名义应力 σ_nominal = F / (厚度 × (宽度 - 孔径))。',
      tips: [
        '理论 K_t ≈ 2.5（考虑有限宽度修正）',
        '仿真 K_t = σ_max / σ_nominal，应在 2.4~2.6 之间',
        '如果偏差大，检查孔边网格是否足够密'
      ]
    },
    {
      id: 'result',
      title: '查看结果',
      description: '重点查看孔边应力分布。',
      icon: '📊',
      component: 'result',
      guidance: '用剖切功能查看孔边应力沿径向的衰减规律。',
      tips: [
        '沿孔边绘制应力曲线，观察峰值位置和衰减趋势',
        '与理论解 σ_θ = σ(1 + 2cos²θ) 对比',
        'θ = 90°（拉伸方向）处应力最小，θ = 0° 处最大'
      ]
    }
  ],
  geometry: {
    type: 'plate',
    params: { length: 200, width: 100, thickness: 5, holeDiameter: 20 },
    unit: 'mm'
  },
  material: {
    name: 'AA 6061-T6',
    category: 'aluminum',
    elasticModulus: 68.9e9,
    poissonsRatio: 0.33,
    density: 2700,
    yieldStrength: 276e6
  },
  mesh: {
    elementType: 'tet10',
    globalSize: 5,
    localRefinement: [
      { region: 'hole_edge', size: 1, description: '孔边应力集中，需要极密网格' }
    ],
    qualityTarget: 0.75
  },
  boundaryConditions: [
    {
      type: 'displacement',
      region: 'left_face',
      values: { ux: 0 },
      description: '左端面固定 x 方向位移'
    },
    {
      type: 'pressure',
      region: 'right_face',
      values: { pressure: 100e6 },
      description: '右端面施加均匀拉应力 100 MPa'
    }
  ],
  solver: {
    analysisType: 'static',
    solverType: 'direct'
  },
  expectedResults: {
    maxStress: 250e6,
    safetyFactor: 1.1
  }
}

// ============================================================================
// 核心模板 4：L 型支架应力集中
// ============================================================================

export const lBracketTemplate: SimulationTemplate = {
  id: 'l-bracket-stress',
  name: 'L 型支架应力集中分析',
  description: 'L 型支架在转角处有应力集中，学习几何过渡圆角对峰值应力的影响。',
  category: 'structural',
  difficulty: 'intermediate',
  estimatedTime: 35,
  tags: ['应力集中', '圆角', '进阶', '教学'],
  steps: [
    {
      id: 'intro',
      title: '案例介绍',
      description: 'L 型支架两臂 100×50×10 mm，转角处圆角 R = 5 mm，垂直臂端固定，水平臂端受 500 N 垂直力。',
      icon: '📖',
      component: 'geometry',
      guidance: 'L 型支架转角处没有圆角时会产生理论上的无限大应力（奇点）。实际工程中必须设计圆角过渡。',
      tips: [
        '无圆角时转角处为应力奇点，应力随网格细化无限增大',
        '圆角半径越大，应力集中越缓和',
        '本案例对比 R = 5 mm 和 R = 0（尖角）的结果'
      ]
    },
    {
      id: 'geometry',
      title: '创建几何',
      description: '创建带圆角的 L 型支架。',
      icon: '📐',
      component: 'geometry',
      guidance: '可以用拉伸命令：先画 L 型截面（带圆角），再沿厚度方向拉伸。',
      tips: [
        '圆角必须与相邻边相切，避免几何不连续',
        '也可以先创建无圆角的 L 型，再用圆角命令（Fillet）添加'
      ]
    },
    {
      id: 'material',
      title: '设置材料',
      description: '选择不锈钢 SS 304。',
      icon: '🔧',
      component: 'material',
      guidance: '不锈钢用于需要耐腐蚀的支架结构。',
      tips: ['SS 304 屈服强度 215 MPa']
    },
    {
      id: 'mesh',
      title: '划分网格',
      description: '全局 3 mm，圆角区域加密到 1 mm。',
      icon: '🔲',
      component: 'mesh',
      guidance: '圆角区域是应力变化最剧烈的地方，网格必须足够细密。',
      tips: [
        '圆角处至少分 3-4 层网格',
        '使用曲面网格（curved elements）更好捕捉圆角几何'
      ]
    },
    {
      id: 'boundary',
      title: '施加边界条件',
      description: '垂直臂端面全固定，水平臂端面施加垂直向下力 500 N。',
      icon: '⚓',
      component: 'boundary',
      guidance: 'L 型支架同时承受弯曲和扭转。',
      tips: [
        '力施加在水平臂端面中心',
        '可以对比将力施加在端面上边缘（产生更大弯矩）'
      ]
    },
    {
      id: 'solver',
      title: '运行求解',
      description: '静力分析。',
      icon: '▶️',
      component: 'solver',
      guidance: '',
      tips: ['']
    },
    {
      id: 'review',
      title: '检查与验证',
      description: '检查应力集中区域。',
      icon: '✅',
      component: 'review',
      guidance: '圆角处的最大应力是设计的关键指标。',
      tips: [
        '有圆角时最大应力应小于无圆角时',
        '如果圆角处应力仍过高，考虑增大圆角半径或增加厚度'
      ]
    },
    {
      id: 'result',
      title: '查看结果',
      description: '重点查看圆角区域应力分布。',
      icon: '📊',
      component: 'result',
      guidance: '使用剖切功能查看圆角内部的应力梯度。',
      tips: [
        '沿圆角弧线绘制应力曲线',
        '对比不同圆角半径的结果（参数化扫描）'
      ]
    }
  ],
  geometry: {
    type: 'solid',
    params: { armLength: 100, armWidth: 50, thickness: 10, filletRadius: 5 },
    unit: 'mm'
  },
  material: {
    name: 'SS 304',
    category: 'steel',
    elasticModulus: 193e9,
    poissonsRatio: 0.29,
    density: 8000,
    yieldStrength: 215e6
  },
  mesh: {
    elementType: 'tet10',
    globalSize: 3,
    localRefinement: [
      { region: 'fillet', size: 1, description: '圆角区域应力集中' }
    ],
    qualityTarget: 0.75
  },
  boundaryConditions: [
    {
      type: 'fixed',
      region: 'vertical_arm_end',
      values: {},
      description: '垂直臂端面全固定'
    },
    {
      type: 'force',
      region: 'horizontal_arm_end',
      values: { fy: -500 },
      description: '水平臂端面垂直向下力 500 N'
    }
  ],
  solver: {
    analysisType: 'static',
    solverType: 'direct'
  },
  expectedResults: {
    maxStress: 150e6,
    safetyFactor: 1.43
  }
}

// ============================================================================
// 模板 5：拉伸试样静力分析（材料力学基础）
// ============================================================================

export const tensileSpecimenTemplate: SimulationTemplate = {
  id: 'tensile-specimen-static',
  name: '拉伸试样静力分析',
  description: '标准狗骨形拉伸试样在轴向拉力作用下的应力分析，学习对称边界条件、应力集中和屈服判定。',
  category: 'structural',
  difficulty: 'beginner',
  estimatedTime: 25,
  tags: ['静力学', '拉伸', '材料试验', '入门'],
  steps: [
    {
      id: 'intro', title: '案例介绍',
      description: '标准狗骨形拉伸试样，平行段长度 50 mm，宽度 10 mm，厚度 2 mm。材料为铝合金 7075-T6，施加轴向拉力 5000 N。',
      icon: '📖', component: 'geometry',
      guidance: '拉伸试验是材料力学性能测试的基础。本案例模拟标准拉伸试样的受力过程。',
      tips: ['利用对称性只需建模 1/4 模型', '平行段是应力均匀区，用于计算弹性模量', '过渡圆弧处存在应力集中']
    },
    { id: 'geometry', title: '创建几何', description: '创建狗骨形拉伸试样，平行段 50×10×2 mm，过渡圆弧 R5 mm。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '选择铝合金 Al7075-T6，弹性模量 71.7 GPa，泊松比 0.33，屈服强度 503 MPa。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '全局尺寸 1 mm，过渡圆弧处局部加密到 0.5 mm。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '一端固定，另一端施加轴向拉力 5000 N。利用对称性可只建 1/4 模型。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '选择静力分析，直接求解器。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '验证应力分布和理论值。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '查看 Mises 应力、轴向应力、应变分布。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { gaugeLength: 50, gaugeWidth: 10, thickness: 2, transitionRadius: 5, gripLength: 25 }, unit: 'mm' },
  material: { name: 'Al7075-T6', category: '铝合金', elasticModulus: 71700, poissonsRatio: 0.33, density: 2810, yieldStrength: 503 },
  mesh: { elementType: 'hex8', globalSize: 1, localRefinement: [{ region: 'transition', size: 0.5, description: '过渡圆弧应力集中区' }], qualityTarget: 0.85 },
  boundaryConditions: [
    { type: 'fixed', region: 'grip_end', values: { ux: 0, uy: 0, uz: 0 }, description: '夹持端固定' },
    { type: 'force', region: 'grip_end_opposite', values: { fx: 5000 }, description: '轴向拉力 5000 N' }
  ],
  solver: { analysisType: 'static', solverType: 'direct' },
  expectedResults: { maxStress: 250, maxDisplacement: 0.175, safetyFactor: 2.01 }
}

// ============================================================================
// 模板 6：圆板均布压力（板壳理论验证）
// ============================================================================

export const circularPlateTemplate: SimulationTemplate = {
  id: 'circular-plate-pressure',
  name: '圆板均布压力分析',
  description: '周边固支圆板在均布压力作用下的弯曲分析，验证板壳理论解，学习轴对称建模和压力载荷施加。',
  category: 'structural',
  difficulty: 'intermediate',
  estimatedTime: 30,
  tags: ['静力学', '板壳', '压力', '轴对称'],
  steps: [
    {
      id: 'intro', title: '案例介绍',
      description: '半径 R = 100 mm、厚度 t = 5 mm 的圆板，周边固支，承受均布压力 p = 1 MPa。',
      icon: '📖', component: 'geometry',
      guidance: '圆板弯曲是板壳理论的经典问题。本案例可用轴对称简化，也可建 3D 模型。',
      tips: ['轴对称模型只需建截面，计算量小', '最大挠度在板中心', '最大应力在板上下表面中心处']
    },
    { id: 'geometry', title: '创建几何', description: '创建半径 100 mm、厚度 5 mm 的圆板。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '选择结构钢 Q355B，弹性模量 206 GPa，泊松比 0.3。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '全局尺寸 3 mm，厚度方向至少 3 层。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '周边固支，上表面施加均布压力 1 MPa。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '选择静力分析，直接求解器。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '与板壳理论解对比。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '查看挠度云图和弯曲应力分布。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { radius: 100, thickness: 5 }, unit: 'mm' },
  material: { name: 'Q355B', category: '结构钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850, yieldStrength: 355 },
  mesh: { elementType: 'hex8', globalSize: 3, localRefinement: [{ region: 'center', size: 1.5, description: '板中心挠度最大区域' }], qualityTarget: 0.8 },
  boundaryConditions: [
    { type: 'fixed', region: 'edge', values: { ux: 0, uy: 0, uz: 0, rx: 0, ry: 0, rz: 0 }, description: '周边固支' },
    { type: 'pressure', region: 'top_surface', values: { pressure: 1.0 }, description: '均布压力 1 MPa' }
  ],
  solver: { analysisType: 'static', solverType: 'direct' },
  expectedResults: { maxStress: 180, maxDisplacement: 0.15, safetyFactor: 1.97 }
}

// ============================================================================
// 模板 7：杆件稳态热传导（热分析入门）
// ============================================================================

export const heatConductionBarTemplate: SimulationTemplate = {
  id: 'heat-conduction-bar',
  name: '杆件稳态热传导',
  description: '一维杆件两端温度不同的稳态热传导分析，学习温度边界条件、热流密度计算和材料热物性设置。',
  category: 'thermal',
  difficulty: 'beginner',
  estimatedTime: 20,
  tags: ['热分析', '热传导', '稳态', '入门'],
  steps: [
    {
      id: 'intro', title: '案例介绍',
      description: '长度 L = 200 mm、截面 10×10 mm 的杆件，左端温度 100°C，右端温度 20°C。材料为铜，导热系数 400 W/(m·K)。',
      icon: '📖', component: 'geometry',
      guidance: '一维稳态热传导有解析解，非常适合验证仿真设置。',
      tips: ['温度沿杆长线性分布', '热流密度 q = k·ΔT/L', '铜的高导热系数使温度梯度较小']
    },
    { id: 'geometry', title: '创建几何', description: '创建 200×10×10 mm 的长方体杆件。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '选择铜，导热系数 400 W/(m·K)。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '全局尺寸 5 mm，长度方向适当加密。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '左端 100°C，右端 20°C，侧面绝热。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '选择稳态热分析。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '验证温度线性分布。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '查看温度云图和热流密度。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { length: 200, width: 10, height: 10 }, unit: 'mm' },
  material: { name: 'Copper', category: '金属', elasticModulus: 110000, poissonsRatio: 0.34, density: 8960, thermalExpansion: 1.65e-5 },
  mesh: { elementType: 'hex8', globalSize: 5, qualityTarget: 0.8 },
  boundaryConditions: [
    { type: 'temperature', region: 'left_end', values: { temperature: 100 }, description: '左端 100°C' },
    { type: 'temperature', region: 'right_end', values: { temperature: 20 }, description: '右端 20°C' }
  ],
  solver: { analysisType: 'thermal', solverType: 'direct' },
  expectedResults: { maxStress: undefined, maxDisplacement: undefined }
}

// ============================================================================
// 模板 8：悬臂梁模态分析（动力学入门）
// ============================================================================

export const cantileverModalTemplate: SimulationTemplate = {
  id: 'cantilever-beam-modal',
  name: '悬臂梁模态分析',
  description: '计算悬臂梁的前几阶固有频率和振型，学习模态分析设置、边界条件对频率的影响。',
  category: 'modal',
  difficulty: 'beginner',
  estimatedTime: 20,
  tags: ['模态', '动力学', '频率', '振型', '入门'],
  steps: [
    {
      id: 'intro', title: '案例介绍',
      description: '长度 L = 1000 mm、截面 50×100 mm 的悬臂梁，材料为结构钢。计算前 6 阶固有频率和振型。',
      icon: '📖', component: 'geometry',
      guidance: '模态分析是动力学仿真的基础，用于确定结构的固有振动特性。',
      tips: ['第一阶频率对应弯曲振型', '模态分析不需要载荷，只需约束', '结果可用来避免共振']
    },
    { id: 'geometry', title: '创建几何', description: '创建矩形截面悬臂梁。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '选择结构钢 Q235，密度 7850 kg/m³。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '全局尺寸 20 mm。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '固定端施加全约束，自由端无约束。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '选择模态分析，提取前 6 阶模态。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '与理论解对比。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '查看各阶频率和振型动画。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { length: 1000, width: 50, height: 100 }, unit: 'mm' },
  material: { name: 'Q235', category: '结构钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850 },
  mesh: { elementType: 'hex8', globalSize: 20, qualityTarget: 0.8 },
  boundaryConditions: [
    { type: 'fixed', region: 'fixed_end', values: { ux: 0, uy: 0, uz: 0 }, description: '固定端全约束' }
  ],
  solver: { analysisType: 'modal', solverType: 'direct', maxIterations: 100 },
  expectedResults: { firstFrequency: 4.2 }
}

// ============================================================================
// 模板 9：压杆屈曲分析（稳定性入门）
// ============================================================================

export const columnBucklingTemplate: SimulationTemplate = {
  id: 'column-buckling',
  name: '压杆屈曲分析',
  description: '细长压杆在轴向压力作用下的线性屈曲分析，学习欧拉临界载荷概念和屈曲模态查看。',
  category: 'buckling',
  difficulty: 'intermediate',
  estimatedTime: 25,
  tags: ['屈曲', '稳定性', '欧拉', '临界载荷'],
  steps: [
    {
      id: 'intro', title: '案例介绍',
      description: '长度 L = 500 mm、截面 20×20 mm 的方形杆，一端固定、一端铰支，承受轴向压力。材料为结构钢。',
      icon: '📖', component: 'geometry',
      guidance: '压杆屈曲是结构稳定性设计的经典问题。线性屈曲分析给出临界载荷的估计值。',
      tips: ['欧拉临界载荷 P_cr = π²EI/(KL)²', '一端固定一端铰支时 K = 0.7', '第一阶屈曲模态通常是弯曲']
    },
    { id: 'geometry', title: '创建几何', description: '创建 500×20×20 mm 的方形杆。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '选择结构钢 Q235。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '全局尺寸 5 mm，长度方向适当加密。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '下端固定，上端铰支（允许转动，约束平移），上端施加轴向压力。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '选择线性屈曲分析，提取前 3 阶屈曲模态。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '与欧拉公式对比。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '查看临界载荷系数和屈曲模态。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { length: 500, width: 20, height: 20 }, unit: 'mm' },
  material: { name: 'Q235', category: '结构钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850 },
  mesh: { elementType: 'hex8', globalSize: 5, qualityTarget: 0.8 },
  boundaryConditions: [
    { type: 'fixed', region: 'bottom', values: { ux: 0, uy: 0, uz: 0 }, description: '下端固定' },
    { type: 'displacement', region: 'top', values: { ux: 0, uy: 0 }, description: '上端铰支（约束水平位移）' },
    { type: 'force', region: 'top', values: { fz: -1000 }, description: '轴向压力 1000 N（负号表示压力）' }
  ],
  solver: { analysisType: 'buckling', solverType: 'direct', maxIterations: 100 },
  expectedResults: { maxStress: undefined, safetyFactor: undefined }
}

// ============================================================================
// 模板 10：法兰螺栓连接（接触分析入门）
// ============================================================================

export const boltedFlangeTemplate: SimulationTemplate = {
  id: 'bolted-flange',
  name: '法兰螺栓连接分析',
  description: '管道法兰通过螺栓预紧连接的静力分析，学习接触设置、螺栓预紧力和密封面压力评估。',
  category: 'structural',
  difficulty: 'intermediate',
  estimatedTime: 40,
  tags: ['静力学', '接触', '螺栓', '法兰', '预紧力'],
  steps: [
    {
      id: 'intro', title: '案例介绍',
      description: 'DN100 管道法兰，通过 8 个 M16 螺栓连接，螺栓预紧力 80 kN，内部压力 2 MPa。',
      icon: '📖', component: 'geometry',
      guidance: '螺栓法兰是压力容器和管道的典型连接形式。本案例学习接触分析和预紧力施加。',
      tips: ['螺栓预紧力分两步施加：先预紧，再加载', '接触设置是收敛的关键', '密封面压力必须大于内部介质压力']
    },
    { id: 'geometry', title: '创建几何', description: '创建上下法兰、螺栓、螺母和垫片几何。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '法兰为 Q345R，螺栓为 35CrMo，垫片为柔性石墨。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '全局尺寸 3 mm，螺栓和接触面局部加密到 1 mm。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '对称约束、螺栓预紧力 80 kN、内部压力 2 MPa。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '选择静力分析，迭代求解器，打开大变形选项。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '检查密封压力和螺栓应力。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '查看密封面压力分布、螺栓拉应力、法兰变形。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { flangeOD: 220, flangeID: 110, thickness: 20, boltCircle: 180, boltDiameter: 16 }, unit: 'mm' },
  material: { name: 'Q345R', category: '压力容器钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850, yieldStrength: 345 },
  mesh: { elementType: 'tet10', globalSize: 3, localRefinement: [{ region: 'bolt_hole', size: 1, description: '螺栓孔周边' }, { region: 'sealing_surface', size: 1, description: '密封面接触区' }], qualityTarget: 0.85 },
  boundaryConditions: [
    { type: 'symmetry', region: 'cut_plane', values: { ux: 0 }, description: '对称面约束' },
    { type: 'force', region: 'bolt_shank', values: { preload: 80000 }, description: '螺栓预紧力 80 kN' },
    { type: 'pressure', region: 'inner_surface', values: { pressure: 2.0 }, description: '内压 2 MPa' }
  ],
  solver: { analysisType: 'static', solverType: 'iterative', maxIterations: 200, tolerance: 1e-4 },
  expectedResults: { maxStress: 280, safetyFactor: 1.23 }
}

// ============================================================================
// 钢铁行业模板 — 连铸 / 热轧 / 焊接 / 热处理
// ============================================================================

/** 连铸坯凝固传热 */
export const continuousCastingTemplate: SimulationTemplate = {
  id: 'continuous-casting',
  name: '连铸坯凝固传热分析',
  description: '板坯连铸凝固过程热分析，模拟结晶器→二冷区→空冷区的温度场演化和凝固壳生长。适用于钢铁连铸工艺优化。',
  category: 'steel',
  difficulty: 'advanced',
  estimatedTime: 60,
  tags: ['钢铁', '连铸', '凝固', '传热', '结晶器', '工业'],
  steps: [
    { id: 'intro', title: '案例介绍', description: '1.2m×0.22m 板坯，浇铸温度 1550°C，拉速 1.2m/min。结晶器水冷+二冷喷雾+空冷三段冷却。', icon: '📖', component: 'geometry', guidance: '连铸是钢铁生产的核心工序。凝固过程的温度控制直接影响铸坯质量。', tips: ['结晶器出口壳厚约 15mm', '二冷区温度控制在 900°C 左右', '完全凝固距离约 12m'] },
    { id: 'geometry', title: '创建几何', description: '板坯 12m×1.2m×0.22m，利用对称性取四分之一模型。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: 'Q235 碳素钢，含固/液相线温度和潜热。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '表面层加密（凝固壳区域），采用 DC3D8 热传导单元。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '沿拉坯方向分 5 个冷却区，各区换热系数不同。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '瞬态热分析，时间步 0.5s，总时长 600s。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '验证凝固壳厚度和表面温度与工厂数据一致。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '温度场云图、凝固前沿推进、壳厚-距离曲线。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { length: 12000, width: 1200, height: 220 }, unit: 'mm' },
  material: { name: 'Q235', category: '碳素结构钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850, yieldStrength: 235, thermalExpansion: 12e-6 },
  mesh: { elementType: 'hex8', globalSize: 50, localRefinement: [{ region: 'surface_shell', size: 5, description: '凝固壳表面层加密' }], qualityTarget: 0.7 },
  boundaryConditions: [
    { type: 'temperature', region: 'entry', values: { temperature: 1823 }, description: '浇铸温度 1550°C' },
    { type: 'temperature', region: 'mold', values: { convection_coeff: 2000, T_fluid: 313 }, description: '结晶器水冷 h=2000' },
    { type: 'temperature', region: 'secondary_cooling', values: { convection_coeff: 500, T_fluid: 298 }, description: '二冷喷雾 h=500' },
    { type: 'temperature', region: 'air_cooling', values: { convection_coeff: 15, T_fluid: 298 }, description: '空冷 h=15' }
  ],
  solver: { analysisType: 'thermal', solverType: 'direct', maxIterations: 5000, tolerance: 1e-5 },
  expectedResults: { maxStress: 180, maxDisplacement: undefined }
}

/** 热轧板坯温度场 */
export const hotRollingTemplate: SimulationTemplate = {
  id: 'hot-rolling',
  name: '热轧板坯温度场与变形',
  description: '中厚板多道次热轧过程热-结构耦合分析，模拟温度场分布和塑性变形。适用于轧制工艺参数优化。',
  category: 'steel',
  difficulty: 'advanced',
  estimatedTime: 90,
  tags: ['钢铁', '热轧', '塑性变形', '温度场', 'Johnson-Cook', '工业'],
  steps: [
    { id: 'intro', title: '案例介绍', description: 'Q345B 板坯 220mm→25mm，5 道次轧制，初始温度 1150°C，终轧温度 ≥850°C。', icon: '📖', component: 'geometry', guidance: '热轧是将板坯减薄至目标厚度的核心工艺。温度控制对产品性能至关重要。', tips: ['终轧温度影响晶粒尺寸和力学性能', '道次间温度降需控制在合理范围', '轧制力随温度降低而增大'] },
    { id: 'geometry', title: '创建几何', description: '初始板坯 2.0m×1.2m×0.22m。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: 'Q345B 低合金钢，Johnson-Cook 高温本构模型。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '厚度方向加密至 20 层，采用 C3D8RT 热-力耦合单元。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '5 道次轧辊位移+接触传热，表面与空气对流散热。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '热-结构耦合分析，弹塑性求解。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '验证终轧温度和各道次轧制力。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '温度场、等效塑性应变、轧制力-时间曲线。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { length: 2000, width: 1200, height: 220 }, unit: 'mm' },
  material: { name: 'Q345B', category: '低合金高强钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850, yieldStrength: 345, thermalExpansion: 12e-6 },
  mesh: { elementType: 'hex8', globalSize: 30, localRefinement: [{ region: 'thickness', size: 5, description: '厚度方向加密' }], qualityTarget: 0.7 },
  boundaryConditions: [
    { type: 'temperature', region: 'initial', values: { temperature: 1423 }, description: '初始温度 1150°C' },
    { type: 'displacement', region: 'top', values: { uz: -40 }, description: '第1道次压下 40mm' },
    { type: 'temperature', region: 'surface', values: { convection_coeff: 50, T_fluid: 298 }, description: '空气对流散热' }
  ],
  solver: { analysisType: 'static', solverType: 'iterative', maxIterations: 3000, tolerance: 1e-4 },
  expectedResults: { maxStress: 470, maxDisplacement: 195 }
}

/** 埋弧焊接头分析 */
export const submergedArcWeldingTemplate: SimulationTemplate = {
  id: 'submerged-arc-welding',
  name: '埋弧焊接头热-结构分析',
  description: '厚板埋弧焊多道次焊接热-结构耦合分析，计算焊接残余应力和变形。适用于焊接结构安全评估。',
  category: 'steel',
  difficulty: 'advanced',
  estimatedTime: 120,
  tags: ['钢铁', '焊接', '埋弧焊', '残余应力', '多道次', 'Goldak', '工业'],
  steps: [
    { id: 'intro', title: '案例介绍', description: 'Q345R 厚板 30mm，V 型坡口 60°，3层5道焊接。焊接电流 550A，电压 32V。', icon: '📖', component: 'geometry', guidance: '焊接残余应力是焊接接头失效的主要原因之一。多道次焊接收缩变形需精确控制。', tips: ['焊接残余应力可达材料屈服强度', '层间温度控制在 150°C 以下', 'HAZ 是焊接接头的薄弱区域'] },
    { id: 'geometry', title: '创建几何', description: '板件 600mm×300mm×30mm，V 型坡口。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: 'Q345R 母材 + H10Mn2 焊丝填充材料。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '焊缝区 1mm，HAZ 2mm，远端 5mm。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: 'Goldak 双椭球热源，5 道焊接路径，层间温度控制。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '热-结构耦合，生死单元技术模拟焊缝填充。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '验证残余应力分布和焊接变形。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '残余应力场、温度场动画、焊接变形。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { length: 600, width: 300, height: 30 }, unit: 'mm' },
  material: { name: 'Q345R', category: '压力容器钢', elasticModulus: 206000, poissonsRatio: 0.3, density: 7850, yieldStrength: 345 },
  mesh: { elementType: 'tet10', globalSize: 5, localRefinement: [{ region: 'weld_seam', size: 1, description: '焊缝区加密' }, { region: 'HAZ', size: 2, description: '热影响区加密' }], qualityTarget: 0.8 },
  boundaryConditions: [
    { type: 'temperature', region: 'initial', values: { temperature: 298 }, description: '初始室温' },
    { type: 'force', region: 'weld_path', values: { heat_flux: 1980000 }, description: '有效热输入 1980 kJ/m' },
    { type: 'temperature', region: 'surface', values: { convection_coeff: 15, T_fluid: 298 }, description: '空气对流散热' }
  ],
  solver: { analysisType: 'thermal', solverType: 'iterative', maxIterations: 10000, tolerance: 1e-5 },
  expectedResults: { maxStress: 345, maxDisplacement: 3.0 }
}

/** 淬火热处理 */
export const quenchingTemplate: SimulationTemplate = {
  id: 'quenching-heat-treatment',
  name: '淬火热处理温度场与相变应力',
  description: '圆柱形齿轮坯淬火热处理热-结构-相变耦合分析，计算淬火残余应力和马氏体相变分布。',
  category: 'steel',
  difficulty: 'advanced',
  estimatedTime: 60,
  tags: ['钢铁', '热处理', '淬火', '马氏体', '相变', '残余应力', '42CrMo', '工业'],
  steps: [
    { id: 'intro', title: '案例介绍', description: '42CrMo 齿轮坯 Φ200×80mm，奥氏体化 850°C 后油淬。表面压应力、心部拉应力。', icon: '📖', component: 'geometry', guidance: '淬火是钢件强化的关键工艺。残余应力分布影响零件的疲劳寿命和尺寸稳定性。', tips: ['马氏体转变产生约 4% 体积膨胀', '表面先冷→先相变→产生压应力', '淬火油的换热系数随温度变化'] },
    { id: 'geometry', title: '创建几何', description: '圆柱体 Φ200mm×80mm，轴对称简化。', icon: '📐', component: 'geometry' },
    { id: 'material', title: '设置材料', description: '42CrMo 合金钢，含奥氏体/马氏体/贝氏体三相材料参数。', icon: '🔧', component: 'material' },
    { id: 'mesh', title: '划分网格', description: '径向 30 等分，轴向 16 等分，DC3D8 单元。', icon: '🔲', component: 'mesh' },
    { id: 'boundary', title: '施加边界条件', description: '分段换热系数：膜沸腾→核沸腾→对流。', icon: '⚓', component: 'boundary' },
    { id: 'solver', title: '运行求解', description: '热-结构-相变耦合分析，Koistinen-Marburger 相变模型。', icon: '▶️', component: 'solver' },
    { id: 'review', title: '检查与验证', description: '验证冷却曲线和马氏体体积分数。', icon: '✅', component: 'review' },
    { id: 'result', title: '查看结果', description: '温度场、马氏体分数分布、残余应力场。', icon: '📊', component: 'result' }
  ],
  geometry: { type: 'solid', params: { diameter: 200, height: 80 }, unit: 'mm' },
  material: { name: '42CrMo', category: '合金结构钢', elasticModulus: 210000, poissonsRatio: 0.3, density: 7850, yieldStrength: 930, thermalExpansion: 12.3e-6 },
  mesh: { elementType: 'hex8', globalSize: 5, qualityTarget: 0.8 },
  boundaryConditions: [
    { type: 'temperature', region: 'initial', values: { temperature: 1123 }, description: '奥氏体化温度 850°C' },
    { type: 'temperature', region: 'outer_surface', values: { convection_coeff: 2000, T_fluid: 298 }, description: '淬火油冷却（平均换热系数）' }
  ],
  solver: { analysisType: 'thermal', solverType: 'iterative', maxIterations: 5000, tolerance: 1e-5 },
  expectedResults: { maxStress: 1200, safetyFactor: undefined }
}

// ============================================================================
// 全部模板列表
// ============================================================================

export const allTemplates: SimulationTemplate[] = [
  cantileverBeamTemplate,
  simplySupportedBeamTemplate,
  perforatedPlateTemplate,
  lBracketTemplate,
  tensileSpecimenTemplate,
  circularPlateTemplate,
  heatConductionBarTemplate,
  cantileverModalTemplate,
  columnBucklingTemplate,
  boltedFlangeTemplate,
  continuousCastingTemplate,
  hotRollingTemplate,
  submergedArcWeldingTemplate,
  quenchingTemplate
]

/** 按分类获取模板 */
export function getTemplatesByCategory(category: TemplateCategory): SimulationTemplate[] {
  return allTemplates.filter(t => t.category === category)
}

/** 按难度获取模板 */
export function getTemplatesByDifficulty(difficulty: DifficultyLevel): SimulationTemplate[] {
  return allTemplates.filter(t => t.difficulty === difficulty)
}

/** 搜索模板 */
export function searchTemplates(query: string): SimulationTemplate[] {
  const q = query.toLowerCase()
  return allTemplates.filter(t =>
    t.name.toLowerCase().includes(q) ||
    t.description.toLowerCase().includes(q) ||
    t.tags.some(tag => tag.toLowerCase().includes(q))
  )
}

/** 获取推荐模板（新手优先） */
export function getRecommendedTemplates(): SimulationTemplate[] {
  return allTemplates.filter(t => t.difficulty === 'beginner')
}