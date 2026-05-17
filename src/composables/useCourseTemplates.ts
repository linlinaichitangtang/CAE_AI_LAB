/**
 * useCourseTemplates.ts — V3.3-004 教材集成模板
 * 内置主流教材（CRC/Cengel）配套模板
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export interface Textbook {
  id: string
  name: string
  author: string
  publisher: string
  isbn?: string
  year: number
  coverImage?: string
  description: string
  categories: string[]  // ['mechanics', 'thermal', 'fem']
  difficulty: 'beginner' | 'intermediate' | 'advanced'
  chapterCount: number
}

export interface CourseTemplate {
  id: string
  textbookId: string
  chapter: string
  section?: string
  title: string
  description: string
  tags: string[]
  difficulty: 'beginner' | 'intermediate' | 'advanced'

  // 模板内容
  geometry?: TemplateGeometry
  mesh?: TemplateMesh
  materials?: TemplateMaterial[]
  boundaryConditions?: TemplateBoundaryCondition[]
  simulation?: TemplateSimulation
  postprocess?: TemplatePostprocess[]

  // 元数据
  estimatedTime?: number  // 分钟
  learningObjectives?: string[]
  prerequisites?: string[]
  relatedFormulas?: Array<{ name: string; equation: string }>
}

export interface TemplateGeometry {
  type: 'beam' | 'plate' | 'shell' | '3d_model' | 'imported'
  dimensions?: {
    length?: number
    width?: number
    height?: number
    radius?: number
    thickness?: number
  }
  importUrl?: string
  cadSource?: string
}

export interface TemplateMesh {
  elementType: 'tet4' | 'tet10' | 'hex8' | 'hex20' | 'shell' | 'beam'
  meshSize?: number
  refinementZones?: Array<{
    type: 'box' | 'sphere' | 'cylinder'
    center: [number, number, number]
    size: [number, number, number]
    meshSize: number
  }>
  qualityCriteria?: {
    minQuality?: number
    maxSkewness?: number
    maxAspectRatio?: number
  }
}

export interface TemplateMaterial {
  name: string
  type: ' isotropic' | 'orthotropic' | 'anisotropic'
  model: 'linear' | 'elastic' | 'plastic' | 'viscoelastic'
  properties: Record<string, number>  // e.g., { E: 210e9, nu: 0.3, rho: 7850 }
}

export interface TemplateBoundaryCondition {
  name: string
  type: 'displacement' | 'force' | 'pressure' | 'thermal' | 'symmetry' | 'fixed'
  location: 'face' | 'edge' | 'point' | 'body'
  region?: string
  values: Record<string, number>
  direction?: 'x' | 'y' | 'z' | 'normal' | 'tangential'
}

export interface TemplateSimulation {
  analysisType: 'static' | 'modal' | 'thermal' | 'transient' | 'buckling' | 'dynamic'
  solver?: string
  convergence?: {
    maxIterations?: number
    tolerance?: number
  }
  timeStep?: {
    total?: number
    increment?: number
  }
}

export interface TemplatePostprocess {
  name: string
  type: 'contour' | 'vector' | 'deformation' | 'diagram' | 'animation'
  quantity: string  // e.g., 'stress', 'displacement', 'temperature'
  component?: 'x' | 'y' | 'z' | 'magnitude' | 'max' | 'min'
  options?: Record<string, any>
}

export interface TextbookCategory {
  id: string
  name: string
  icon: string
  description: string
}

// ============ 教材数据 ============

export const TEXTBOOKS: Textbook[] = [
  {
    id: 'crc_mechanics',
    name: 'Mechanics of Materials',
    author: 'James M. Gere & Barry J. Goodno',
    publisher: 'Cengage Learning',
    isbn: '978-1337093354',
    year: 2018,
    description: '经典材料力学教材，覆盖杆件、梁、柱的应力应变分析',
    categories: ['mechanics', 'static', 'beam'],
    difficulty: 'intermediate',
    chapterCount: 14
  },
  {
    id: 'cengel_thermal',
    name: 'Heat Transfer: A Practical Approach',
    author: 'Yunus A. Cengel',
    publisher: 'McGraw-Hill',
    isbn: '978-0073398129',
    year: 2014,
    description: '全面介绍热传导、对流和辐射的实用教材',
    categories: ['thermal', 'heat_transfer'],
    difficulty: 'intermediate',
    chapterCount: 12
  },
  {
    id: 'logan_fem',
    name: 'A First Course in the Finite Element Method',
    author: 'Daryl L. Logan',
    publisher: 'Cengage Learning',
    isbn: '978-1305635111',
    year: 2017,
    description: '有限元方法入门教材，适合初学者',
    categories: ['fem', 'static', 'thermal'],
    difficulty: 'beginner',
    chapterCount: 13
  },
  {
    id: 'rao_fem',
    name: 'The Finite Element Method in Engineering',
    author: 'Singiresu S. Rao',
    publisher: 'Butterworth-Heinemann',
    isbn: '978-0080952046',
    year: 2017,
    description: '工程有限元方法的综合教材，覆盖多种应用',
    categories: ['fem', 'dynamics', 'modal'],
    difficulty: 'advanced',
    chapterCount: 18
  },
  {
    id: 'cook_fem',
    name: 'Concepts and Applications of Finite Element Analysis',
    author: 'Robert D. Cook et al.',
    publisher: 'Wiley',
    isbn: '978-0471356059',
    year: 2001,
    description: '有限元分析概念与应用，理论与实践结合',
    categories: ['fem', 'nonlinear', 'static'],
    difficulty: 'advanced',
    chapterCount: 15
  },
  {
    id: 'boresi_fem',
    name: 'Advanced Mechanics of Materials',
    author: 'Arthur P. Boresi & Richard J. Schmidt',
    publisher: 'Wiley',
    isbn: '978-0471510598',
    year: 2003,
    description: '高级材料力学，涵盖弹塑性和断裂力学',
    categories: ['mechanics', 'plasticity', 'fracture'],
    difficulty: 'advanced',
    chapterCount: 12
  },
  {
    id: 'holt_fem',
    name: 'Fundamentals of Finite Element Analysis',
    author: 'David V. Hutton',
    publisher: 'McGraw-Hill',
    isbn: '978-0072391283',
    year: 2004,
    description: '有限元分析基础，结合工程应用',
    categories: ['fem', 'static', 'thermal'],
    difficulty: 'beginner',
    chapterCount: 10
  },
  {
    id: 'shigley_fem',
    name: 'Mechanical Engineering Design',
    author: 'Richard G. Budynas & J. Keith Nisbett',
    publisher: 'McGraw-Hill',
    isbn: '978-0073398211',
    year: 2020,
    description: '机械工程设计，涵盖疲劳和优化',
    categories: ['design', 'fatigue', 'optimization'],
    difficulty: 'intermediate',
    chapterCount: 16
  }
]

// ============ 课程模板数据 ============

export const COURSE_TEMPLATES: CourseTemplate[] = [
  // ============ Mechanics of Materials (Gere) ============
  {
    id: 'gere_ch3_axial',
    textbookId: 'crc_mechanics',
    chapter: 'Chapter 3: Axial Deformation',
    title: 'Axial Loading of a Steel Bar',
    description: 'Analyze stress and strain distribution in a prismatic bar under axial loading',
    tags: ['axial', 'stress', 'strain', 'deformation'],
    difficulty: 'beginner',
    estimatedTime: 30,
    learningObjectives: [
      'Understand axial stress and strain concepts',
      'Apply Hooke\'s law for linear elastic materials',
      'Calculate deformation of axially loaded members'
    ],
    geometry: { type: 'beam', dimensions: { length: 1, width: 0.05, height: 0.05 } },
    mesh: { elementType: 'hex8', meshSize: 0.01 },
    materials: [{ name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 210e9, nu: 0.3, rho: 7850 } }],
    boundaryConditions: [
      { name: 'Fixed End', type: 'fixed', location: 'face', region: 'x=0', values: { ux: 0, uy: 0, uz: 0 } },
      { name: 'Axial Force', type: 'force', location: 'face', region: 'x=L', values: { F: 10000 }, direction: 'x' }
    ],
    simulation: { analysisType: 'static', convergence: { maxIterations: 100, tolerance: 1e-6 } },
    postprocess: [
      { name: 'Stress Contour', type: 'contour', quantity: 'stress', component: 'magnitude' },
      { name: 'Deformation', type: 'deformation', quantity: 'displacement', component: 'magnitude' }
    ],
    relatedFormulas: [
      { name: 'Stress', equation: 'σ = F/A' },
      { name: 'Strain', equation: 'ε = δ/L' },
      { name: 'Hooke\'s Law', equation: 'σ = Eε' }
    ]
  },
  {
    id: 'gere_ch4_torsion',
    textbookId: 'crc_mechanics',
    chapter: 'Chapter 4: Torsion',
    title: 'Torsion of a Circular Shaft',
    description: 'Analyze shear stress and angle of twist in a shaft under torsion',
    tags: ['torsion', 'shear', 'shaft', 'twist'],
    difficulty: 'beginner',
    estimatedTime: 35,
    learningObjectives: [
      'Calculate shear stress in circular shafts',
      'Determine angle of twist under torsion',
      'Understand power transmission in shafts'
    ],
    geometry: { type: 'beam', dimensions: { length: 0.5, radius: 0.025 } },
    mesh: { elementType: 'tet4', meshSize: 0.005 },
    materials: [{ name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 210e9, nu: 0.3, G: 81e9, rho: 7850 } }],
    boundaryConditions: [
      { name: 'Fixed End', type: 'fixed', location: 'face', region: 'x=0', values: { ux: 0, uy: 0, uz: 0 } },
      { name: 'Torque', type: 'force', location: 'face', region: 'x=L', values: { T: 500 }, direction: 'tangential' }
    ],
    simulation: { analysisType: 'static' },
    postprocess: [
      { name: 'Shear Stress', type: 'contour', quantity: 'stress', component: 'x' },
      { name: 'Angle of Twist', type: 'deformation', quantity: 'rotation', component: 'z' }
    ],
    relatedFormulas: [
      { name: 'Shear Stress', equation: 'τ = T*r/J' },
      { name: 'Polar Moment', equation: 'J = π*d⁴/32' },
      { name: 'Angle of Twist', equation: 'φ = T*L/(G*J)' }
    ]
  },
  {
    id: 'gere_ch5_beam',
    textbookId: 'crc_mechanics',
    chapter: 'Chapter 5: Beams',
    title: 'Simply Supported Beam with Point Load',
    description: 'Analyze bending stress and deflection in a simply supported beam',
    tags: ['beam', 'bending', 'shear', 'deflection'],
    difficulty: 'beginner',
    estimatedTime: 40,
    learningObjectives: [
      'Calculate bending stress in beams',
      'Determine shear stress distribution',
      'Find beam deflection under loading'
    ],
    geometry: { type: 'beam', dimensions: { length: 2, width: 0.1, height: 0.2 } },
    mesh: { elementType: 'hex8', meshSize: 0.02 },
    materials: [{ name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 210e9, nu: 0.3, rho: 7850 } }],
    boundaryConditions: [
      { name: 'Pin Support', type: 'displacement', location: 'face', region: 'x=0', values: { ux: 0, uy: 0 } },
      { name: 'Roller Support', type: 'displacement', location: 'face', region: 'x=L', values: { uy: 0 } },
      { name: 'Point Load', type: 'force', location: 'point', region: 'x=L/2', values: { F: -10000 }, direction: 'y' }
    ],
    simulation: { analysisType: 'static' },
    postprocess: [
      { name: 'Bending Stress', type: 'contour', quantity: 'stress', component: 'x' },
      { name: 'Deflection', type: 'deformation', quantity: 'displacement', component: 'y' },
      { name: 'Shear Diagram', type: 'diagram', quantity: 'stress', component: 'x' }
    ],
    relatedFormulas: [
      { name: 'Bending Stress', equation: 'σ = M*y/I' },
      { name: 'Moment of Inertia', equation: 'I = bh³/12' },
      { name: 'Max Deflection', equation: 'δ = PL³/(48EI)' }
    ]
  },
  {
    id: 'gere_ch9_columns',
    textbookId: 'crc_mechanics',
    chapter: 'Chapter 9: Columns',
    title: 'Buckling of a Long Column',
    description: 'Analyze critical buckling load for a pin-ended column using eigenvalue analysis',
    tags: ['buckling', 'column', 'euler', 'stability'],
    difficulty: 'intermediate',
    estimatedTime: 45,
    learningObjectives: [
      'Understand column buckling concepts',
      'Calculate Euler\'s critical load',
      'Perform eigenvalue buckling analysis'
    ],
    geometry: { type: 'beam', dimensions: { length: 3, width: 0.05, height: 0.1 } },
    mesh: { elementType: 'beam', meshSize: 0.1 },
    materials: [{ name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 210e9, nu: 0.3, rho: 7850 } }],
    boundaryConditions: [
      { name: 'Pin-Pin', type: 'displacement', location: 'face', region: 'x=0', values: { ux: 0, uy: 0, uz: 0 } },
      { name: 'Axial Load', type: 'force', location: 'face', region: 'x=L', values: { F: 1 }, direction: 'x' }
    ],
    simulation: { analysisType: 'buckling', solver: 'eigenvalue' },
    postprocess: [
      { name: 'Buckling Mode', type: 'deformation', quantity: 'displacement', component: 'magnitude' },
      { name: 'Critical Load', type: 'contour', quantity: 'reaction', component: 'x' }
    ],
    relatedFormulas: [
      { name: 'Euler Load', equation: 'Pcr = π²EI/(KL)²' },
      { name: 'Effective Length', equation: 'K = 1 for pin-pin' }
    ]
  },

  // ============ Cengel Heat Transfer ============
  {
    id: 'cengel_ch1_conduct',
    textbookId: 'cengel_thermal',
    chapter: 'Chapter 1: Introduction to Heat Transfer',
    section: '1-4: Conduction',
    title: 'Steady Heat Conduction Through a Wall',
    description: 'Analyze one-dimensional steady heat conduction through a composite wall',
    tags: ['conduction', 'heat_transfer', 'thermal', 'steady_state'],
    difficulty: 'beginner',
    estimatedTime: 30,
    learningObjectives: [
      'Understand Fourier\'s law of heat conduction',
      'Calculate thermal resistance and heat transfer rate',
      'Analyze composite wall systems'
    ],
    geometry: { type: 'plate', dimensions: { length: 0.2, width: 1, height: 1 } },
    mesh: { elementType: 'hex8', meshSize: 0.01 },
    materials: [
      { name: 'Brick', type: ' isotropic', model: 'linear', properties: { k: 0.72, rho: 1920, cp: 800 } }
    ],
    boundaryConditions: [
      { name: 'Hot Side', type: 'thermal', location: 'face', region: 'x=0', values: { T: 100 } },
      { name: 'Cold Side', type: 'thermal', location: 'face', region: 'x=L', values: { T: 20 } }
    ],
    simulation: { analysisType: 'thermal' },
    postprocess: [
      { name: 'Temperature Contour', type: 'contour', quantity: 'temperature' },
      { name: 'Heat Flux', type: 'vector', quantity: 'heat_flux' }
    ],
    relatedFormulas: [
      { name: 'Fourier\'s Law', equation: 'q = -k ∇T' },
      { name: 'Thermal Resistance', equation: 'R = L/(kA)' },
      { name: 'Heat Transfer Rate', equation: 'Q = ΔT/R_total' }
    ]
  },
  {
    id: 'cengel_ch4_transient',
    textbookId: 'cengel_thermal',
    chapter: 'Chapter 4: Transient Heat Conduction',
    title: 'Lumped System Analysis',
    description: 'Analyze temperature variation in a body during transient heating/cooling',
    tags: ['transient', 'lumped', 'thermal', 'convection'],
    difficulty: 'intermediate',
    estimatedTime: 40,
    learningObjectives: [
      'Apply lumped system analysis when Biot number < 0.1',
      'Calculate time constant and temperature response',
      'Understand convective heat transfer effects'
    ],
    geometry: { type: '3d_model', dimensions: { length: 0.05, width: 0.05, height: 0.05 } },
    mesh: { elementType: 'tet4', meshSize: 0.005 },
    materials: [
      { name: 'Aluminum', type: ' isotropic', model: 'linear', properties: { k: 237, rho: 2700, cp: 900 } }
    ],
    boundaryConditions: [
      { name: 'Initial Temp', type: 'thermal', location: 'body', values: { T: 25 } },
      { name: 'Convection', type: 'thermal', location: 'face', values: { h: 50, T_inf: 100 } }
    ],
    simulation: { analysisType: 'transient', timeStep: { total: 300, increment: 1 } },
    postprocess: [
      { name: 'Temperature vs Time', type: 'diagram', quantity: 'temperature' },
      { name: 'Temperature Contour', type: 'contour', quantity: 'temperature' }
    ],
    relatedFormulas: [
      { name: 'Biot Number', equation: 'Bi = hLc/k' },
      { name: 'Time Constant', equation: 'τ = ρVc/(hA)' },
      { name: 'Lumped Solution', equation: 'T - T∞ = (Ti - T∞)e^(-t/τ)' }
    ]
  },

  // ============ Logan FEM ============
  {
    id: 'logan_ch4_truss',
    textbookId: 'logan_fem',
    chapter: 'Chapter 4: Trusses',
    title: 'Plane Truss Analysis',
    description: 'Analyze displacements and forces in a 2D truss structure',
    tags: ['truss', 'finite_element', 'structural', 'plane_stress'],
    difficulty: 'beginner',
    estimatedTime: 35,
    learningObjectives: [
      'Understand truss element formulation',
      'Apply boundary conditions correctly',
      'Interpret finite element results'
    ],
    geometry: { type: 'beam', dimensions: { length: 1, width: 0.01, height: 0.01 } },
    mesh: { elementType: 'beam', meshSize: 0.25 },
    materials: [{ name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 200e9, nu: 0.3, rho: 7850, A: 0.001 } }],
    boundaryConditions: [
      { name: 'Fixed Support', type: 'fixed', location: 'point', region: 'node_1', values: { ux: 0, uy: 0 } },
      { name: 'Roller', type: 'displacement', location: 'point', region: 'node_5', values: { uy: 0 } },
      { name: 'Load', type: 'force', location: 'point', region: 'node_3', values: { Fy: -10000 } }
    ],
    simulation: { analysisType: 'static' },
    postprocess: [
      { name: 'Deformation', type: 'deformation', quantity: 'displacement', component: 'magnitude' },
      { name: 'Axial Force', type: 'diagram', quantity: 'force', component: 'x' }
    ],
    relatedFormulas: [
      { name: 'Stiffness Matrix', equation: '[k] = (AE/L)[1 -1;-1 1]' },
      { name: 'Shape Functions', equation: 'N1 = 1 - ξ, N2 = ξ' }
    ]
  },
  {
    id: 'logan_ch6_2d',
    textbookId: 'logan_fem',
    chapter: 'Chapter 6: Two-Dimensional Problems',
    title: 'Plane Stress Analysis of a Plate',
    description: 'Analyze stress distribution in a rectangular plate with a central hole',
    tags: ['plane_stress', '2d', 'stress_concentration', 'hole'],
    difficulty: 'intermediate',
    estimatedTime: 45,
    learningObjectives: [
      'Understand plane stress assumptions',
      'Apply symmetry boundary conditions',
      'Analyze stress concentration around discontinuities'
    ],
    geometry: { type: 'plate', dimensions: { length: 0.1, width: 0.05, height: 0.005 } },
    mesh: { elementType: 'tet4', meshSize: 0.002, refinementZones: [{ type: 'cylinder', center: [0.05, 0.025, 0], size: [0.015, 0.015, 0.01], meshSize: 0.001 }] },
    materials: [{ name: 'Aluminum', type: ' isotropic', model: 'linear', properties: { E: 70e9, nu: 0.33, rho: 2700 } }],
    boundaryConditions: [
      { name: 'Symmetry X', type: 'displacement', location: 'face', region: 'x=0', values: { ux: 0 } },
      { name: 'Symmetry Y', type: 'displacement', location: 'face', region: 'y=0', values: { uy: 0 } },
      { name: 'Tension', type: 'force', location: 'face', region: 'x=L', values: { σ: 50e6 } }
    ],
    simulation: { analysisType: 'static' },
    postprocess: [
      { name: 'Stress Contour', type: 'contour', quantity: 'stress', component: 'magnitude' },
      { name: 'Deformation', type: 'deformation', quantity: 'displacement', component: 'magnitude', options: { scaleFactor: 100 } }
    ],
    relatedFormulas: [
      { name: 'Stress Concentration', equation: 'Kt = σ_max/σ_nom' },
      { name: 'Plane Stress', equation: 'σ_z = 0, ε_z ≠ 0' }
    ]
  },

  // ============ Rao FEM ============
  {
    id: 'rao_ch10_modal',
    textbookId: 'rao_fem',
    chapter: 'Chapter 10: Vibration Analysis',
    title: 'Natural Frequency Analysis of a Cantilever Beam',
    description: 'Determine natural frequencies and mode shapes of a cantilever beam',
    tags: ['modal', 'vibration', 'natural_frequency', 'mode_shape'],
    difficulty: 'intermediate',
    estimatedTime: 40,
    learningObjectives: [
      'Understand modal analysis fundamentals',
      'Extract natural frequencies and mode shapes',
      'Interpret modal participation factors'
    ],
    geometry: { type: 'beam', dimensions: { length: 1, width: 0.02, height: 0.04 } },
    mesh: { elementType: 'tet4', meshSize: 0.02 },
    materials: [{ name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 210e9, nu: 0.3, rho: 7850 } }],
    boundaryConditions: [
      { name: 'Fixed End', type: 'fixed', location: 'face', region: 'x=0', values: { ux: 0, uy: 0, uz: 0, rx: 0, ry: 0, rz: 0 } }
    ],
    simulation: { analysisType: 'modal', solver: 'lanczos', convergence: { maxIterations: 200 } },
    postprocess: [
      { name: 'First Mode', type: 'deformation', quantity: 'mode_shape', component: 'min' },
      { name: 'Second Mode', type: 'deformation', quantity: 'mode_shape', component: 'max' },
      { name: 'Frequency Table', type: 'diagram', quantity: 'frequency' }
    ],
    relatedFormulas: [
      { name: 'Eigenvalue Problem', equation: '[K]{φ} = ω²[M]{φ}' },
      { name: 'Natural Frequency', equation: 'f = ω/(2π)' }
    ]
  },

  // ============ Additional Advanced Templates ============
  {
    id: 'advanced_nonlinear',
    textbookId: 'rao_fem',
    chapter: 'Advanced Topics: Nonlinear Analysis',
    title: 'Elastic-Plastic Analysis of a Notched Specimen',
    description: 'Analyze plastic zone development and residual stresses in a notched specimen',
    tags: ['plasticity', 'nonlinear', 'residual_stress', 'notch'],
    difficulty: 'advanced',
    estimatedTime: 60,
    learningObjectives: [
      'Understand plasticity theory and yield criteria',
      'Apply incremental plasticity formulation',
      'Analyze residual stress distribution after unloading'
    ],
    geometry: { type: 'plate', dimensions: { length: 0.1, width: 0.04, height: 0.002 } },
    mesh: { elementType: 'tet4', meshSize: 0.001, refinementZones: [{ type: 'box', center: [0.03, 0.02, 0], size: [0.02, 0.01, 0.005], meshSize: 0.0005 }] },
    materials: [
      { name: 'Steel', type: ' isotropic', model: 'plastic', properties: { E: 210e9, nu: 0.3, sigma_y: 250e6, rho: 7850 } }
    ],
    boundaryConditions: [
      { name: 'Symmetry', type: 'displacement', location: 'face', region: 'x=0', values: { ux: 0 } },
      { name: 'Displacement Control', type: 'displacement', location: 'face', region: 'x=L', values: { ux: 0.002 } }
    ],
    simulation: { analysisType: 'static', solver: 'newton_raphson', convergence: { maxIterations: 50, tolerance: 1e-4 } },
    postprocess: [
      { name: 'Plastic Zone', type: 'contour', quantity: 'plastic_strain' },
      { name: 'Residual Stress', type: 'contour', quantity: 'stress', component: 'x' }
    ],
    relatedFormulas: [
      { name: 'von Mises Yield', equation: 'σ_vm = √(3J2)' },
      { name: 'Flow Rule', equation: '{ε̇_p} = λ{∂f/∂σ}' }
    ]
  },
  {
    id: 'thermal_stress_coupling',
    textbookId: 'cengel_thermal',
    chapter: 'Advanced: Thermal Stress Analysis',
    title: 'Thermal Stress in a Bi-Material Bar',
    description: 'Analyze thermal stresses induced by temperature gradient in a bi-material assembly',
    tags: ['thermal_stress', 'coupled', 'bi_material', 'expansion'],
    difficulty: 'advanced',
    estimatedTime: 55,
    learningObjectives: [
      'Understand thermal-mechanical coupling',
      'Analyze thermal mismatch stresses in composites',
      'Apply appropriate boundary conditions for thermal stress'
    ],
    geometry: { type: 'beam', dimensions: { length: 0.1, width: 0.02, height: 0.02 } },
    mesh: { elementType: 'hex8', meshSize: 0.005 },
    materials: [
      { name: 'Aluminum', type: ' isotropic', model: 'linear', properties: { E: 70e9, nu: 0.33, alpha: 23e-6, k: 237 } },
      { name: 'Steel', type: ' isotropic', model: 'linear', properties: { E: 200e9, nu: 0.3, alpha: 12e-6, k: 16 } }
    ],
    boundaryConditions: [
      { name: 'Fixed', type: 'fixed', location: 'point', region: 'x=0', values: { ux: 0 } },
      { name: 'Temperature Gradient', type: 'thermal', location: 'face', region: 'x=L', values: { T: 100 } },
      { name: 'Convection', type: 'thermal', location: 'face', region: 'x=0', values: { h: 10, T_inf: 25 } }
    ],
    simulation: { analysisType: 'thermal', solver: 'coupled' },
    postprocess: [
      { name: 'Temperature Contour', type: 'contour', quantity: 'temperature' },
      { name: 'Thermal Stress', type: 'contour', quantity: 'stress', component: 'x' }
    ],
    relatedFormulas: [
      { name: 'Thermal Strain', equation: 'ε_th = αΔT' },
      { name: 'Thermal Stress', equation: 'σ = EαΔT/(1-ν)' }
    ]
  }
]

// ============ 分类数据 ============

export const TEXTBOOK_CATEGORIES: TextbookCategory[] = [
  { id: 'mechanics', name: '材料力学', icon: '📐', description: '应力、应变、梁、柱分析' },
  { id: 'fem', name: '有限元方法', icon: '🔲', description: 'FEM 基础与应用' },
  { id: 'thermal', name: '热传导', icon: '🔥', description: '热传递与热应力' },
  { id: 'dynamics', name: '动力学', icon: '⚡', description: '振动、模态、动力响应' },
  { id: 'design', name: '工程设计', icon: '📋', description: '机械设计、优化、疲劳' },
  { id: 'nonlinear', name: '非线性分析', icon: '📈', description: '塑性、屈曲、大变形' }
]

// ============ 存储键 ============

const FAVORITES_KEY = 'caelab_course_favorites'
const RECENT_KEY = 'caelab_course_recent'

// ============ 工具函数 ============

function generateId(): string {
  return `course_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

// ============ 主 Composable ============

export function useCourseTemplates() {
  const textbooks = ref<Textbook[]>(TEXTBOOKS)
  const templates = ref<CourseTemplate[]>(COURSE_TEMPLATES)
  const categories = ref<TextbookCategory[]>(TEXTBOOK_CATEGORIES)
  const favorites = ref<string[]>([])  // template IDs
  const recentTemplates = ref<Array<{ templateId: string; usedAt: string }>>([])

  // ============ 初始化 ============

  function loadData(): void {
    const storedFavorites = getStorage<string[]>(FAVORITES_KEY)
    if (storedFavorites) favorites.value = storedFavorites

    const storedRecent = getStorage<Array<{ templateId: string; usedAt: string }>>(RECENT_KEY)
    if (storedRecent) recentTemplates.value = storedRecent
  }

  // ============ 查询 ============

  function getTextbook(textbookId: string): Textbook | undefined {
    return textbooks.value.find(t => t.id === textbookId)
  }

  function getTemplate(templateId: string): CourseTemplate | undefined {
    return templates.value.find(t => t.id === templateId)
  }

  function getTemplatesByTextbook(textbookId: string): CourseTemplate[] {
    return templates.value.filter(t => t.textbookId === textbookId)
  }

  function getTemplatesByCategory(category: string): CourseTemplate[] {
    const textbookIds = textbooks.value
      .filter(t => t.categories.includes(category))
      .map(t => t.id)
    return templates.value.filter(t => textbookIds.includes(t.textbookId))
  }

  function searchTemplates(query: string): CourseTemplate[] {
    const lowerQuery = query.toLowerCase()
    return templates.value.filter(t =>
      t.title.toLowerCase().includes(lowerQuery) ||
      t.description.toLowerCase().includes(lowerQuery) ||
      t.tags.some(tag => tag.toLowerCase().includes(lowerQuery)) ||
      t.chapter.toLowerCase().includes(lowerQuery)
    )
  }

  function filterTemplates(options: {
    textbookId?: string
    category?: string
    difficulty?: 'beginner' | 'intermediate' | 'advanced'
    tags?: string[]
    search?: string
  }): CourseTemplate[] {
    let result = templates.value

    if (options.textbookId) {
      result = result.filter(t => t.textbookId === options.textbookId)
    }

    if (options.category) {
      const textbookIds = textbooks.value
        .filter(t => t.categories.includes(options.category!))
        .map(t => t.id)
      result = result.filter(t => textbookIds.includes(t.textbookId))
    }

    if (options.difficulty) {
      result = result.filter(t => t.difficulty === options.difficulty)
    }

    if (options.tags && options.tags.length > 0) {
      result = result.filter(t =>
        options.tags!.some(tag => t.tags.includes(tag))
      )
    }

    if (options.search) {
      const lowerQuery = options.search.toLowerCase()
      result = result.filter(t =>
        t.title.toLowerCase().includes(lowerQuery) ||
        t.description.toLowerCase().includes(lowerQuery)
      )
    }

    return result
  }

  // ============ 收藏 ============

  function toggleFavorite(templateId: string): boolean {
    const index = favorites.value.indexOf(templateId)
    if (index === -1) {
      favorites.value.push(templateId)
      saveData()
      return true
    } else {
      favorites.value.splice(index, 1)
      saveData()
      return false
    }
  }

  function isFavorite(templateId: string): boolean {
    return favorites.value.includes(templateId)
  }

  function getFavoriteTemplates(): CourseTemplate[] {
    return templates.value.filter(t => favorites.value.includes(t.id))
  }

  // ============ 最近使用 ============

  function markAsUsed(templateId: string): void {
    // 移除旧的记录
    recentTemplates.value = recentTemplates.value.filter(r => r.templateId !== templateId)

    // 添加新记录到开头
    recentTemplates.value.unshift({
      templateId,
      usedAt: new Date().toISOString()
    })

    // 只保留最近 10 条
    if (recentTemplates.value.length > 10) {
      recentTemplates.value = recentTemplates.value.slice(0, 10)
    }

    saveData()
  }

  function getRecentTemplates(): CourseTemplate[] {
    return recentTemplates.value
      .map(r => templates.value.find(t => t.id === r.templateId))
      .filter((t): t is CourseTemplate => t !== undefined)
  }

  // ============ 统计 ============

  function getStats(): {
    totalTextbooks: number
    totalTemplates: number
    templatesByDifficulty: Record<string, number>
    templatesByCategory: Record<string, number>
  } {
    const templatesByDifficulty: Record<string, number> = {
      beginner: 0,
      intermediate: 0,
      advanced: 0
    }
    const templatesByCategory: Record<string, number> = {}

    for (const template of templates.value) {
      templatesByDifficulty[template.difficulty]++
    }

    for (const textbook of textbooks.value) {
      for (const category of textbook.categories) {
        templatesByCategory[category] = (templatesByCategory[category] || 0) + 1
      }
    }

    return {
      totalTextbooks: textbooks.value.length,
      totalTemplates: templates.value.length,
      templatesByDifficulty,
      templatesByCategory
    }
  }

  // ============ 导出/导入 ============

  function exportTemplate(templateId: string): string {
    const template = getTemplate(templateId)
    if (!template) return '{}'
    return JSON.stringify(template, null, 2)
  }

  function importTemplate(jsonString: string): CourseTemplate | null {
    try {
      const template = JSON.parse(jsonString) as CourseTemplate
      template.id = generateId()  // 分配新 ID
      templates.value.push(template)
      saveData()
      return template
    } catch {
      return null
    }
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(FAVORITES_KEY, favorites.value)
    setStorage(RECENT_KEY, recentTemplates.value)
  }

  // 初始化
  loadData()

  return {
    // 状态
    textbooks,
    templates,
    categories,
    favorites,
    recentTemplates,

    // 查询
    getTextbook,
    getTemplate,
    getTemplatesByTextbook,
    getTemplatesByCategory,
    searchTemplates,
    filterTemplates,

    // 收藏
    toggleFavorite,
    isFavorite,
    getFavoriteTemplates,

    // 最近使用
    markAsUsed,
    getRecentTemplates,

    // 统计
    getStats,

    // 导出导入
    exportTemplate,
    importTemplate,

    // 数据
    TEXTBOOKS,
    COURSE_TEMPLATES,
    TEXTBOOK_CATEGORIES
  }
}
