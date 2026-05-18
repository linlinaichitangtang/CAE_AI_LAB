/**
 * 钢铁材料数据库 — GB/T 标准钢号库
 * 覆盖碳素结构钢、低合金高强钢、压力容器钢、合金结构钢、耐候钢、不锈钢
 * 数据来源：GB/T 700, GB/T 1591, GB 713, GB/T 3077, GB/T 4171, GB/T 1220, GB 50017
 */

// ============ 类型定义 ============

export type SteelCategory = 'carbon' | 'low_alloy' | 'pressure_vessel' | 'alloy_structural' | 'weathering' | 'stainless'

export interface SteelComposition {
  C: { min: number; max: number }
  Si: { min: number; max: number }
  Mn: { min: number; max: number }
  P?: { max: number }
  S?: { max: number }
  Cr?: { min?: number; max: number }
  Ni?: { min?: number; max: number }
  Mo?: { min?: number; max: number }
  V?: { min?: number; max: number }
  Ti?: { max: number }
  Cu?: { min?: number; max: number }
  Al?: { min: number; max: number }
  Nb?: { max: number }
}

export interface CreepDataPoint {
  temperature: number   // K
  stress: number        // Pa
  ruptureTime: number   // hours
}

export interface SteelMaterialGB {
  id: string
  grade: string
  standard: string
  category: SteelCategory
  displayName: string

  // 力学性能（室温）
  elasticModulus: number        // Pa
  poissonsRatio: number
  density: number               // kg/m³
  yieldStrength: number         // Pa (室温屈服)
  tensileStrength: number       // Pa (室温抗拉)
  designStrength?: number       // Pa (GB 50017 强度设计值 f)

  // 热学性能
  thermalConductivity?: number  // W/(m·K)
  specificHeat?: number         // J/(kg·K)
  thermalExpansion?: number     // 1/K

  // 疲劳
  fatigueLimit?: number         // Pa (10^7 cycle, 对称循环)

  // 蠕变
  creepData?: CreepDataPoint[]

  // 化学成分 (wt%)
  composition?: SteelComposition

  // 高温力学性能 (可选)
  yieldStrengthHighTemp?: { temperature: number; value: number }[]
}

// ============ 碳素结构钢 (GB/T 700-2006) ============

const carbonSteels: SteelMaterialGB[] = [
  {
    id: 'gb-q195',
    grade: 'Q195',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q195 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 195e6,
    tensileStrength: 315e6,
    designStrength: 170e6,
    thermalConductivity: 58,
    specificHeat: 480,
    thermalExpansion: 11.5e-6,
    fatigueLimit: 140e6,
    composition: { C: { min: 0.06, max: 0.12 }, Si: { min: 0, max: 0.30 }, Mn: { min: 0.25, max: 0.50 }, P: { max: 0.045 }, S: { max: 0.050 } }
  },
  {
    id: 'gb-q215',
    grade: 'Q215',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q215 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 215e6,
    tensileStrength: 335e6,
    designStrength: 185e6,
    thermalConductivity: 55,
    specificHeat: 480,
    thermalExpansion: 11.7e-6,
    fatigueLimit: 155e6,
    composition: { C: { min: 0.09, max: 0.15 }, Si: { min: 0, max: 0.30 }, Mn: { min: 0.25, max: 0.55 }, P: { max: 0.045 }, S: { max: 0.050 } }
  },
  {
    id: 'gb-q235a',
    grade: 'Q235A',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q235A 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 235e6,
    tensileStrength: 370e6,
    designStrength: 215e6,
    thermalConductivity: 50,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 170e6,
    composition: { C: { min: 0.14, max: 0.22 }, Si: { min: 0, max: 0.30 }, Mn: { min: 0.30, max: 0.65 }, P: { max: 0.045 }, S: { max: 0.050 } }
  },
  {
    id: 'gb-q235b',
    grade: 'Q235B',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q235B 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 235e6,
    tensileStrength: 370e6,
    designStrength: 215e6,
    thermalConductivity: 50,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 170e6,
    composition: { C: { min: 0.12, max: 0.20 }, Si: { min: 0, max: 0.30 }, Mn: { min: 0.30, max: 0.70 }, P: { max: 0.045 }, S: { max: 0.045 } }
  },
  {
    id: 'gb-q235c',
    grade: 'Q235C',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q235C 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 235e6,
    tensileStrength: 370e6,
    designStrength: 215e6,
    thermalConductivity: 50,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 170e6,
    composition: { C: { min: 0.12, max: 0.18 }, Si: { min: 0, max: 0.30 }, Mn: { min: 0.30, max: 0.70 }, P: { max: 0.040 }, S: { max: 0.040 } }
  },
  {
    id: 'gb-q235d',
    grade: 'Q235D',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q235D 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 235e6,
    tensileStrength: 370e6,
    designStrength: 215e6,
    thermalConductivity: 50,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 170e6,
    composition: { C: { min: 0.10, max: 0.17 }, Si: { min: 0, max: 0.30 }, Mn: { min: 0.30, max: 0.70 }, P: { max: 0.035 }, S: { max: 0.035 } }
  },
  {
    id: 'gb-q275',
    grade: 'Q275',
    standard: 'GB/T 700-2006',
    category: 'carbon',
    displayName: 'Q275 碳素结构钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 275e6,
    tensileStrength: 410e6,
    designStrength: 250e6,
    thermalConductivity: 48,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 195e6,
    composition: { C: { min: 0.15, max: 0.24 }, Si: { min: 0, max: 0.35 }, Mn: { min: 0.40, max: 0.80 }, P: { max: 0.045 }, S: { max: 0.050 } }
  }
]

// ============ 低合金高强度结构钢 (GB/T 1591-2018) ============

const lowAlloySteels: SteelMaterialGB[] = [
  {
    id: 'gb-q345a',
    grade: 'Q345A',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q345A 低合金高强度钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6,
    tensileStrength: 470e6,
    designStrength: 305e6,
    thermalConductivity: 42,
    specificHeat: 540,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 230e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.60 }, P: { max: 0.040 }, S: { max: 0.040 }, Nb: { max: 0.07 }, V: { max: 0.15 }, Ti: { max: 0.20 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q345b',
    grade: 'Q345B',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q345B 低合金高强度钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6,
    tensileStrength: 470e6,
    designStrength: 305e6,
    thermalConductivity: 42,
    specificHeat: 540,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 230e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.60 }, P: { max: 0.040 }, S: { max: 0.040 }, Nb: { max: 0.07 }, V: { max: 0.15 }, Ti: { max: 0.20 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q345c',
    grade: 'Q345C',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q345C 低合金高强度钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6,
    tensileStrength: 470e6,
    designStrength: 305e6,
    thermalConductivity: 42,
    specificHeat: 540,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 230e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.60 }, P: { max: 0.035 }, S: { max: 0.035 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q345d',
    grade: 'Q345D',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q345D 低合金高强度钢（低温冲击）',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6,
    tensileStrength: 470e6,
    designStrength: 305e6,
    thermalConductivity: 42,
    specificHeat: 540,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 230e6,
    composition: { C: { min: 0, max: 0.18 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.60 }, P: { max: 0.030 }, S: { max: 0.030 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q345e',
    grade: 'Q345E',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q345E 低合金高强度钢（-40°C 冲击）',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6,
    tensileStrength: 470e6,
    designStrength: 305e6,
    thermalConductivity: 42,
    specificHeat: 540,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 230e6,
    composition: { C: { min: 0, max: 0.18 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.60 }, P: { max: 0.025 }, S: { max: 0.025 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q390',
    grade: 'Q390',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q390 低合金高强度钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 390e6,
    tensileStrength: 490e6,
    designStrength: 335e6,
    thermalConductivity: 40,
    specificHeat: 530,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 255e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.60 }, P: { max: 0.035 }, S: { max: 0.035 }, V: { max: 0.20 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q420',
    grade: 'Q420',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q420 低合金高强度钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 420e6,
    tensileStrength: 520e6,
    designStrength: 360e6,
    thermalConductivity: 38,
    specificHeat: 520,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 270e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.50 }, Mn: { min: 1.00, max: 1.70 }, P: { max: 0.035 }, S: { max: 0.035 }, V: { max: 0.20 }, Nb: { max: 0.07 }, Al: { min: 0.015, max: 0.060 } }
  },
  {
    id: 'gb-q460',
    grade: 'Q460',
    standard: 'GB/T 1591-2018',
    category: 'low_alloy',
    displayName: 'Q460 低合金高强度钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 460e6,
    tensileStrength: 550e6,
    designStrength: 390e6,
    thermalConductivity: 36,
    specificHeat: 510,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 295e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.55 }, Mn: { min: 1.00, max: 1.70 }, P: { max: 0.030 }, S: { max: 0.030 }, V: { max: 0.20 }, Nb: { max: 0.07 }, Ti: { max: 0.20 }, Al: { min: 0.015, max: 0.060 } }
  }
]

// ============ 锅炉和压力容器用钢 (GB 713-2014) ============

const pressureVesselSteels: SteelMaterialGB[] = [
  {
    id: 'gb-q245r',
    grade: 'Q245R',
    standard: 'GB 713-2014',
    category: 'pressure_vessel',
    displayName: 'Q245R 锅炉和压力容器用钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 245e6,
    tensileStrength: 400e6,
    designStrength: 148e6,
    thermalConductivity: 48,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 180e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.35 }, Mn: { min: 0.50, max: 1.00 }, P: { max: 0.025 }, S: { max: 0.015 }, Al: { min: 0.020, max: 0.060 } },
    creepData: [
      { temperature: 673, stress: 80e6, ruptureTime: 100000 },
      { temperature: 723, stress: 50e6, ruptureTime: 100000 },
      { temperature: 773, stress: 30e6, ruptureTime: 100000 }
    ]
  },
  {
    id: 'gb-q345r',
    grade: 'Q345R',
    standard: 'GB 713-2014',
    category: 'pressure_vessel',
    displayName: 'Q345R 锅炉和压力容器用钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 345e6,
    tensileStrength: 510e6,
    designStrength: 185e6,
    thermalConductivity: 42,
    specificHeat: 540,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 230e6,
    composition: { C: { min: 0, max: 0.20 }, Si: { min: 0, max: 0.55 }, Mn: { min: 1.20, max: 1.60 }, P: { max: 0.025 }, S: { max: 0.015 }, Al: { min: 0.020, max: 0.060 } },
    creepData: [
      { temperature: 673, stress: 110e6, ruptureTime: 100000 },
      { temperature: 723, stress: 70e6, ruptureTime: 100000 },
      { temperature: 773, stress: 40e6, ruptureTime: 100000 }
    ]
  },
  {
    id: 'gb-q370r',
    grade: 'Q370R',
    standard: 'GB 713-2014',
    category: 'pressure_vessel',
    displayName: 'Q370R 锅炉和压力容器用钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 370e6,
    tensileStrength: 530e6,
    designStrength: 200e6,
    thermalConductivity: 40,
    specificHeat: 530,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 245e6,
    composition: { C: { min: 0, max: 0.18 }, Si: { min: 0, max: 0.55 }, Mn: { min: 1.20, max: 1.60 }, P: { max: 0.025 }, S: { max: 0.015 }, V: { max: 0.05 }, Al: { min: 0.020, max: 0.060 } }
  },
  {
    id: 'gb-q420r',
    grade: 'Q420R',
    standard: 'GB 713-2014',
    category: 'pressure_vessel',
    displayName: 'Q420R 锅炉和压力容器用钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 420e6,
    tensileStrength: 570e6,
    designStrength: 225e6,
    thermalConductivity: 38,
    specificHeat: 520,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 270e6,
    composition: { C: { min: 0, max: 0.18 }, Si: { min: 0, max: 0.55 }, Mn: { min: 1.20, max: 1.60 }, P: { max: 0.020 }, S: { max: 0.010 }, V: { max: 0.08 }, Nb: { max: 0.05 }, Al: { min: 0.020, max: 0.060 } }
  }
]

// ============ 合金结构钢 (GB/T 3077-2015) ============

const alloyStructuralSteels: SteelMaterialGB[] = [
  {
    id: 'gb-20cr',
    grade: '20Cr',
    standard: 'GB/T 3077-2015',
    category: 'alloy_structural',
    displayName: '20Cr 合金结构钢（渗碳钢）',
    elasticModulus: 207e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 540e6,
    tensileStrength: 835e6,
    designStrength: 410e6,
    thermalConductivity: 44,
    specificHeat: 470,
    thermalExpansion: 11.5e-6,
    fatigueLimit: 370e6,
    composition: { C: { min: 0.17, max: 0.24 }, Si: { min: 0.17, max: 0.37 }, Mn: { min: 0.50, max: 0.80 }, P: { max: 0.030 }, S: { max: 0.030 }, Cr: { min: 0.70, max: 1.00 } }
  },
  {
    id: 'gb-40cr',
    grade: '40Cr',
    standard: 'GB/T 3077-2015',
    category: 'alloy_structural',
    displayName: '40Cr 合金结构钢（调质钢）',
    elasticModulus: 207e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 785e6,
    tensileStrength: 980e6,
    designStrength: 590e6,
    thermalConductivity: 40,
    specificHeat: 460,
    thermalExpansion: 11.8e-6,
    fatigueLimit: 430e6,
    composition: { C: { min: 0.37, max: 0.44 }, Si: { min: 0.17, max: 0.37 }, Mn: { min: 0.50, max: 0.80 }, P: { max: 0.030 }, S: { max: 0.030 }, Cr: { min: 0.80, max: 1.10 } }
  },
  {
    id: 'gb-42crmo',
    grade: '42CrMo',
    standard: 'GB/T 3077-2015',
    category: 'alloy_structural',
    displayName: '42CrMo 合金结构钢（高强度调质钢）',
    elasticModulus: 210e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 930e6,
    tensileStrength: 1080e6,
    designStrength: 680e6,
    thermalConductivity: 42,
    specificHeat: 470,
    thermalExpansion: 12.3e-6,
    fatigueLimit: 490e6,
    composition: { C: { min: 0.38, max: 0.45 }, Si: { min: 0.17, max: 0.37 }, Mn: { min: 0.50, max: 0.80 }, P: { max: 0.025 }, S: { max: 0.025 }, Cr: { min: 0.90, max: 1.20 }, Mo: { min: 0.15, max: 0.25 } }
  },
  {
    id: 'gb-20crmnti',
    grade: '20CrMnTi',
    standard: 'GB/T 3077-2015',
    category: 'alloy_structural',
    displayName: '20CrMnTi 合金结构钢（齿轮钢）',
    elasticModulus: 207e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 835e6,
    tensileStrength: 1080e6,
    designStrength: 620e6,
    thermalConductivity: 42,
    specificHeat: 460,
    thermalExpansion: 11.5e-6,
    fatigueLimit: 460e6,
    composition: { C: { min: 0.17, max: 0.23 }, Si: { min: 0.17, max: 0.37 }, Mn: { min: 0.80, max: 1.10 }, P: { max: 0.030 }, S: { max: 0.030 }, Cr: { min: 1.00, max: 1.30 }, Ti: { max: 0.06 } }
  }
]

// ============ 耐候钢 (GB/T 4171-2008) ============

const weatheringSteels: SteelMaterialGB[] = [
  {
    id: 'gb-q235nh',
    grade: 'Q235NH',
    standard: 'GB/T 4171-2008',
    category: 'weathering',
    displayName: 'Q235NH 耐候钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 235e6,
    tensileStrength: 360e6,
    designStrength: 215e6,
    thermalConductivity: 50,
    specificHeat: 490,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 170e6,
    composition: { C: { min: 0, max: 0.15 }, Si: { min: 0.15, max: 0.40 }, Mn: { min: 0.20, max: 0.60 }, P: { max: 0.035 }, S: { max: 0.035 }, Cu: { min: 0.20, max: 0.50 }, Cr: { min: 0.40, max: 0.80 } }
  },
  {
    id: 'gb-q295nh',
    grade: 'Q295NH',
    standard: 'GB/T 4171-2008',
    category: 'weathering',
    displayName: 'Q295NH 耐候钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 295e6,
    tensileStrength: 430e6,
    designStrength: 260e6,
    thermalConductivity: 46,
    specificHeat: 480,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 205e6,
    composition: { C: { min: 0, max: 0.15 }, Si: { min: 0.15, max: 0.50 }, Mn: { min: 0.60, max: 1.00 }, P: { max: 0.035 }, S: { max: 0.035 }, Cu: { min: 0.25, max: 0.55 }, Cr: { min: 0.40, max: 0.80 }, Ni: { min: 0, max: 0.65 } }
  },
  {
    id: 'gb-q355nh',
    grade: 'Q355NH',
    standard: 'GB/T 4171-2008',
    category: 'weathering',
    displayName: 'Q355NH 耐候钢',
    elasticModulus: 206e9,
    poissonsRatio: 0.3,
    density: 7850,
    yieldStrength: 355e6,
    tensileStrength: 490e6,
    designStrength: 305e6,
    thermalConductivity: 42,
    specificHeat: 470,
    thermalExpansion: 12.0e-6,
    fatigueLimit: 235e6,
    composition: { C: { min: 0, max: 0.16 }, Si: { min: 0.20, max: 0.50 }, Mn: { min: 0.50, max: 1.00 }, P: { max: 0.030 }, S: { max: 0.030 }, Cu: { min: 0.25, max: 0.55 }, Cr: { min: 0.40, max: 0.80 }, Ni: { min: 0.12, max: 0.65 } }
  }
]

// ============ 不锈钢 (GB/T 1220-2007) ============

const stainlessSteels: SteelMaterialGB[] = [
  {
    id: 'gb-06cr19ni10',
    grade: '06Cr19Ni10',
    standard: 'GB/T 1220-2007',
    category: 'stainless',
    displayName: '06Cr19Ni10 (304) 奥氏体不锈钢',
    elasticModulus: 193e9,
    poissonsRatio: 0.29,
    density: 7930,
    yieldStrength: 205e6,
    tensileStrength: 520e6,
    designStrength: 137e6,
    thermalConductivity: 16.2,
    specificHeat: 500,
    thermalExpansion: 17.3e-6,
    fatigueLimit: 210e6,
    composition: { C: { min: 0, max: 0.08 }, Si: { min: 0, max: 1.00 }, Mn: { min: 0, max: 2.00 }, P: { max: 0.045 }, S: { max: 0.030 }, Cr: { min: 18.0, max: 20.0 }, Ni: { min: 8.0, max: 11.0 } },
    yieldStrengthHighTemp: [
      { temperature: 573, value: 145e6 },
      { temperature: 673, value: 130e6 },
      { temperature: 773, value: 120e6 },
      { temperature: 873, value: 110e6 }
    ]
  },
  {
    id: 'gb-022cr19ni10',
    grade: '022Cr19Ni10',
    standard: 'GB/T 1220-2007',
    category: 'stainless',
    displayName: '022Cr19Ni10 (304L) 超低碳奥氏体不锈钢',
    elasticModulus: 193e9,
    poissonsRatio: 0.29,
    density: 7930,
    yieldStrength: 175e6,
    tensileStrength: 480e6,
    designStrength: 120e6,
    thermalConductivity: 16.3,
    specificHeat: 500,
    thermalExpansion: 17.3e-6,
    fatigueLimit: 195e6,
    composition: { C: { min: 0, max: 0.03 }, Si: { min: 0, max: 1.00 }, Mn: { min: 0, max: 2.00 }, P: { max: 0.045 }, S: { max: 0.030 }, Cr: { min: 18.0, max: 20.0 }, Ni: { min: 8.0, max: 12.0 } }
  },
  {
    id: 'gb-06cr17ni12mo2',
    grade: '06Cr17Ni12Mo2',
    standard: 'GB/T 1220-2007',
    category: 'stainless',
    displayName: '06Cr17Ni12Mo2 (316) 含钼奥氏体不锈钢',
    elasticModulus: 193e9,
    poissonsRatio: 0.29,
    density: 7980,
    yieldStrength: 205e6,
    tensileStrength: 520e6,
    designStrength: 137e6,
    thermalConductivity: 16.3,
    specificHeat: 500,
    thermalExpansion: 16.0e-6,
    fatigueLimit: 210e6,
    composition: { C: { min: 0, max: 0.08 }, Si: { min: 0, max: 1.00 }, Mn: { min: 0, max: 2.00 }, P: { max: 0.045 }, S: { max: 0.030 }, Cr: { min: 16.0, max: 18.5 }, Ni: { min: 10.0, max: 14.0 }, Mo: { min: 2.0, max: 3.0 } },
    yieldStrengthHighTemp: [
      { temperature: 573, value: 140e6 },
      { temperature: 673, value: 128e6 },
      { temperature: 773, value: 120e6 },
      { temperature: 873, value: 114e6 }
    ]
  }
]

// ============ 全部材料 ============

export const STEEL_MATERIALS_GB: SteelMaterialGB[] = [
  ...carbonSteels,
  ...lowAlloySteels,
  ...pressureVesselSteels,
  ...alloyStructuralSteels,
  ...weatheringSteels,
  ...stainlessSteels
]

// ============ 查询函数 ============

/** 按钢号精确查询 */
export function getSteelByGrade(grade: string): SteelMaterialGB | undefined {
  return STEEL_MATERIALS_GB.find(m => m.grade.toLowerCase() === grade.toLowerCase())
}

/** 按钢号模糊查询 */
export function searchSteels(query: string): SteelMaterialGB[] {
  const q = query.toLowerCase()
  return STEEL_MATERIALS_GB.filter(m =>
    m.grade.toLowerCase().includes(q) ||
    m.displayName.toLowerCase().includes(q) ||
    m.standard.toLowerCase().includes(q) ||
    m.id.toLowerCase().includes(q)
  )
}

/** 按类别查询 */
export function getSteelsByCategory(category: SteelCategory): SteelMaterialGB[] {
  return STEEL_MATERIALS_GB.filter(m => m.category === category)
}

/** 按标准号查询 */
export function getSteelsByStandard(standard: string): SteelMaterialGB[] {
  return STEEL_MATERIALS_GB.filter(m => m.standard === standard)
}

/** 获取所有标准号 */
export function getAllStandards(): string[] {
  return [...new Set(STEEL_MATERIALS_GB.map(m => m.standard))]
}

/** 获取所有类别 */
export function getAllCategories(): SteelCategory[] {
  return [...new Set(STEEL_MATERIALS_GB.map(m => m.category))]
}

/** 获取类别显示名 */
export const CATEGORY_LABELS: Record<SteelCategory, string> = {
  carbon: '碳素结构钢',
  low_alloy: '低合金高强度钢',
  pressure_vessel: '锅炉压力容器钢',
  alloy_structural: '合金结构钢',
  weathering: '耐候钢',
  stainless: '不锈钢'
}

/** 导出为 MaterialPanel 兼容格式 */
export function toMaterialPanelFormat(mat: SteelMaterialGB): {
  name: string
  category: string
  elasticModulus: number
  poissonsRatio: number
  density: number
  yieldStrength: number
  thermalExpansion?: number
  thermalConductivity?: number
  specificHeat?: number
} {
  return {
    name: mat.grade,
    category: CATEGORY_LABELS[mat.category],
    elasticModulus: mat.elasticModulus,
    poissonsRatio: mat.poissonsRatio,
    density: mat.density,
    yieldStrength: mat.yieldStrength,
    thermalExpansion: mat.thermalExpansion,
    thermalConductivity: mat.thermalConductivity,
    specificHeat: mat.specificHeat
  }
}
