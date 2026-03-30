/**
 * CAE API - Frontend wrapper for backend CAE commands
 * 封装所有CAE仿真相关的后端API
 */

import { invoke } from '@tauri-apps/api/tauri'

// ============ 类型定义 ============

export interface Node {
  id: number
  x: number
  y: number
  z: number
}

export interface Element {
  id: number
  node_ids: number[]
  element_type: ElementType
}

export enum ElementType {
  C3D8 = 'C3D8',
  C3D4 = 'C3D4',
  C2D4 = 'C2D4',
  C2D3 = 'C2D3'
}

export interface Material {
  id: number
  name: string
  elastic_modulus: number
  poisson_ratio: number
  density: number
}

export interface FixedBc {
  name: string
  nodes: number[]
  bc_type: string
  surfaces?: {name: string; node_ids: number[]}[]
}

export interface PointLoad {
  name: string
  node: number
  magnitude: number
  direction: string // 'X' | 'Y' | 'Z' | 'Normal' | 'Custom'
  fx?: number
  fy?: number
  fz?: number
}

export interface UniformLoad {
  name: string
  surface_name: string
  element_faces: Array<[number, number[]]>
  magnitude: number
  load_type: string // 'Pressure' | 'TractionX' | 'TractionY' | 'TractionZ' | 'HeatFlux' | 'Film'
}

export interface BcContainer {
  fixed_bcs: FixedBc[]
  point_loads: PointLoad[]
  uniform_loads: UniformLoad[]
}

export interface StructuredMeshConfig {
  x_min: number
  x_max: number
  x_div: number
  y_min: number
  y_max: number
  y_div: number
  z_min: number
  z_max: number
  z_div: number
  element_type: 'C3D8' | 'C2D4'
}

export interface MeshApiResult {
  nodes: Node[]
  elements: Element[]
}

export interface ResultStats {
  min: number
  max: number
  mean: number
  std_dev: number
}

export interface ResultSet {
  node_values: Array<{node_id: number; value: number}[]>
  stats: ResultStats
  step_name: string
}

// ============ 热传导分析类型 ============

export type ThermalAnalysisType = 'steady_state' | 'transient'

export interface ThermalMaterial {
  id: number
  name: string
  thermal_conductivity: number  // W/(m·K)
  specific_heat: number        // J/(kg·K)
  density: number              // kg/m³
  convection_coefficient?: number // W/(m²·K) 对流系数
}

export interface HeatSource {
  name: string
  type: 'point' | 'surface' | 'volume'
  magnitude: number  // W (点热源) 或 W/m² (面热源) 或 W/m³ (体热源)
  node_ids?: number[]  // 点热源
  surface_name?: string  // 面热源
}

export type ThermalBcType = 'fixed_temperature' | 'heat_convection' | 'heat_radiation' | 'insulation'

export interface ThermalBoundaryCondition {
  name: string
  bc_type: ThermalBcType
  nodes?: number[]  // 固定温度
  surface_name?: string  // 对流/辐射
  temperature?: number  // 固定温度值 (K)
  ambient_temperature?: number  // 环境温度 (K)
  film_coefficient?: number  // 对流系数 W/(m²·K)
  emissivity?: number  // 辐射率
  radiation_temperature?: number  // 辐射环境温度 (K)
}

export interface ThermalSimulationConfig {
  analysis_type: ThermalAnalysisType
  initial_temperature?: number  // 初始温度 (K)
  time_period?: number  // 瞬态分析时间周期 (s)
  time_increment?: number  // 时间步长 (s)
  max_iterations?: number  // 最大迭代数
  tolerance?: number  // 收敛容差
}

// ============ CFD分析类型 ============

export type CfdAnalysisType = 'incompressible' | 'compressible'

export interface FluidProperties {
  density: number        // kg/m³
  viscosity: number      // Pa·s
  specific_heat?: number // J/(kg·K)
  thermal_conductivity?: number // W/(m·K)
  turbulent_model?: 'laminar' | 'k_epsilon' | 'k_omega'
  turbulence_intensity?: number
}

export interface InletBoundary {
  name: string
  type: 'velocity' | 'mass_flow' | 'pressure'
  velocity_x?: number  // m/s
  velocity_y?: number
  velocity_z?: number
  mass_flow_rate?: number  // kg/s
  pressure?: number  // Pa
  turbulent_intensity?: number
}

export interface OutletBoundary {
  name: string
  type: 'pressure' | 'outflow'
  pressure?: number  // Pa
}

export interface WallBoundary {
  name: string
  type: 'no_slip' | 'free_slip'
  nodes: number[]
  temperature?: number  // 壁面温度 (K) - 用于热耦合
  heat_flux?: number  // W/m²
}

export interface CfdSimulationConfig {
  analysis_type: CfdAnalysisType
  gravity?: { x: number; y: number; z: number }
  reference_pressure?: number  // 参考压力 Pa
  relaxation_factor?: number
  max_iterations?: number
  convergence_criteria?: number
}

// ============ 网格生成 API ============

/** 生成2D结构化网格 */
export async function generate2dMesh(
  x_min: number, x_max: number, x_div: number,
  y_min: number, y_max: number, y_div: number,
  element_type: 'C2D4' | 'C2D3'
): Promise<MeshApiResult> {
  return await invoke<MeshApiResult>('generate_2d_mesh', {
    x_min, x_max, x_div,
    y_min, y_max, y_div,
    element_type
  })
}

/** 生成3D结构化网格 */
export async function generate3dMesh(
  x_min: number, x_max: number, x_div: number,
  y_min: number, y_max: number, y_div: number,
  z_min: number, z_max: number, z_div: number,
  element_type: 'C3D8' | 'C3D4'
): Promise<MeshApiResult> {
  return await invoke<MeshApiResult>('generate_3d_mesh', {
    x_min, x_max, x_div,
    y_min, y_max, y_div,
    z_min, z_max, z_div,
    element_type
  })
}

/** 生成结构化网格（统一接口） */
export async function generateStructuredMesh(config: StructuredMeshConfig): Promise<MeshApiResult> {
  return await invoke<MeshApiResult>('generate_structured_mesh', { config })
}

/** 导出网格到INP文件 */
export async function exportMeshToInp(nodes: Node[], elements: Element[], path: string): Promise<string> {
  return await invoke<string>('export_mesh_to_inp', { nodes, elements, output_path: path })
}

// ============ 边界条件 & 荷载 API ============

/** 创建固定边界条件 */
export async function createFixedBc(name: string, nodes: number[], bcType: string): Promise<FixedBc> {
  return await invoke<FixedBc>('create_fixed_bc', { name, nodes, bc_type: bcType })
}

/** 创建自定义固定边界条件（指定自由度） */
export async function createCustomFixedBc(name: string, nodes: number[], dofs: number[]): Promise<FixedBc> {
  return await invoke<FixedBc>('create_custom_fixed_bc', { name, nodes, dofs })
}

/** 创建点荷载 */
export async function createPointLoad(name: string, node: number, magnitude: number, direction: string): Promise<PointLoad> {
  return await invoke<PointLoad>('create_point_load', { name, node, magnitude, direction })
}

/** 创建向量方向点荷载 */
export async function createVectorPointLoad(name: string, node: number, magnitude: number, fx: number, fy: number, fz: number): Promise<PointLoad> {
  return await invoke<PointLoad>('create_vector_point_load', { name, node, magnitude, fx, fy, fz })
}

/** 创建压力荷载 */
export async function createPressureLoad(name: string, surfaceName: string, magnitude: number): Promise<UniformLoad> {
  return await invoke<UniformLoad>('create_pressure_load', { name, surface_name: surfaceName, magnitude })
}

/** 创建牵引力荷载 */
export async function createTractionLoad(name: string, surfaceName: string, direction: string, magnitude: number): Promise<UniformLoad> {
  return await invoke<UniformLoad>('create_traction_load', { name, surface_name: surfaceName, direction, magnitude })
}

/** 创建热流荷载 */
export async function createHeatFluxLoad(name: string, surfaceName: string, magnitude: number): Promise<UniformLoad> {
  return await invoke<UniformLoad>('create_heat_flux_load', { name, surface_name: surfaceName, magnitude })
}

/** 创建BC容器 */
export async function createBcContainer(): Promise<BcContainer> {
  return await invoke<BcContainer>('create_bc_container')
}

/** 生成BC的INP片段 */
export async function generateBcInp(fixedBcs: FixedBc[], pointLoads: PointLoad[], uniformLoads: UniformLoad[]): Promise<string> {
  return await invoke<string>('generate_bc_inp', { fixed_bcs: fixedBcs, point_loads: pointLoads, uniform_loads: uniformLoads })
}

/** 获取所有支持的BC类型 */
export async function getBcTypes(): Promise<string[]> {
  return await invoke<string[]>('get_bc_types')
}

/** 获取所有支持的荷载方向 */
export async function getLoadDirections(): Promise<string[]> {
  return await invoke<string[]>('get_load_directions')
}

/** 获取所有支持的均布荷载类型 */
export async function getUniformLoadTypes(): Promise<string[]> {
  return await invoke<string[]>('get_uniform_load_types')
}

/** 在指定面上找节点 */
export async function findNodesAtFace(nodes: Node[], axis: 'x' | 'y' | 'z', value: number, tolerance: number): Promise<number[]> {
  return await invoke<number[]>('find_nodes_at_face', { nodes, axis, value, tolerance })
}

/** 为悬臂梁创建固定边界条件（左端面x=0） */
export async function createCantileverFixedBc(nodes: Node[]): Promise<FixedBc> {
  return await invoke<FixedBc>('create_cantilever_fixed_bc', { nodes })
}

/** 为悬臂梁创建点荷载（右端面顶部） */
export async function createCantileverPointLoad(nodes: Node[], maxX: number, magnitude: number): Promise<PointLoad> {
  return await invoke<PointLoad>('create_cantilever_point_load', { nodes, max_x: maxX, magnitude })
}

/** 生成完整的INP输入文件 */
export async function generateCompleteInp(
  nodes: Node[],
  elements: Element[],
  material: Material,
  fixedBc: FixedBc,
  pointLoad: PointLoad | null,
  uniformLoads: UniformLoad[],
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_complete_inp', {
    nodes,
    elements,
    material,
    fixed_bc: fixedBc,
    point_load: pointLoad,
    uniform_loads: uniformLoads,
    output_path: outputPath
  })
}

// ============ 结果解析 API ============

/** 解析FRD结果文件 */
export async function parseFrdFile(path: string): Promise<ResultSet> {
  return await invoke<ResultSet>('parse_frd_file', { path })
}

/** 运行求解器 */
export async function runSolver(inputPath: string, workingDir: string, config?: {executable_path?: string, num_threads?: number, memory_limit_mb?: number}): Promise<any> {
  return await invoke<any>('run_solver', { input_path: inputPath, working_dir: workingDir, config })
}

// ============ 热传导分析 API ============

/** 生成热传导INP输入文件 */
export async function generateThermalInp(
  nodes: Node[],
  elements: Element[],
  material: ThermalMaterial,
  heatSources: HeatSource[],
  boundaryConditions: ThermalBoundaryCondition[],
  config: ThermalSimulationConfig,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_thermal_inp', {
    nodes,
    elements,
    material,
    heat_sources: heatSources,
    boundary_conditions: boundaryConditions,
    config,
    output_path: outputPath
  })
}

/** 运行热传导求解器 */
export async function runThermalSolver(inputPath: string, workingDir: string): Promise<any> {
  return await invoke<any>('run_thermal_solver', { input_path: inputPath, working_dir: workingDir })
}

/** 解析热传导结果文件 */
export async function parseThermalResult(path: string): Promise<ResultSet> {
  return await invoke<ResultSet>('parse_thermal_result', { path })
}

// ============ 屈曲分析类型 ============

export type BucklingAnalysisType = 'linear' | 'nonlinear'

export interface BucklingConfig {
  analysis_type: BucklingAnalysisType
  num_modes: number          // 要计算的屈曲模态数量
  min_load_factor?: number    // 最小载荷因子阈值
  max_load_factor?: number    // 最大载荷因子阈值
  arc_length_options?: {
    max_iterations: number    // 最大迭代次数
    tolerance: number         // 收敛容差
    initial_increment: number // 初始增量步
    min_increment: number     // 最小增量
    max_increment: number     // 最大增量
  }
}

export interface BucklingResult {
  job_id: string
  analysis_type: BucklingAnalysisType
  critical_load_factor: number    // 临界载荷因子
  load_multipliers: number[]     // 各阶模态的载荷因子
  mode_shapes: BucklingModeShape[]
  warnings: string[]
  errors: string[]
}

export interface BucklingModeShape {
  mode_number: number
  load_multiplier: number       // 该模态的临界载荷因子
  max_displacement: number      // 最大位移幅值
  max_von_mises?: number        // 最大等效应力（可选）
  participation_factor?: number // 参与因子
  description: string           // 描述，如 "一阶屈曲", "二阶屈曲"
  nodal_displacements: Array<{node_id: number; dx: number; dy: number; dz: number}>
}

// ============ 屈曲分析 API ============

/** 生成屈曲分析INP输入文件 */
export async function generateBucklingInp(
  nodes: Node[],
  elements: Element[],
  material: Material,
  fixedBc: FixedBc,
  pointLoad: PointLoad | null,
  uniformLoads: UniformLoad[],
  config: BucklingConfig,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_buckling_inp', {
    nodes,
    elements,
    material,
    fixed_bc: fixedBc,
    point_load: pointLoad,
    uniform_loads: uniformLoads,
    config,
    output_path: outputPath
  })
}

/** 运行屈曲分析求解器 */
export async function runBucklingSolver(inputPath: string, workingDir: string): Promise<any> {
  return await invoke<any>('run_buckling_solver', { input_path: inputPath, working_dir: workingDir })
}

/** 解析屈曲结果文件（.dat/.frd） */
export async function parseBucklingResult(path: string): Promise<BucklingResult> {
  return await invoke<BucklingResult>('parse_buckling_result', { path })
}

/** 屈曲安全系数计算 */
export async function calculateBucklingSafetyFactor(
  loadFactor: number,
  appliedLoad: number,
  yieldStrength: number,
  materialFactor: number = 1.5  // 材料安全系数
): Promise<{ safetyFactor: number; isSafe: boolean; description: string }> {
  return await invoke<{ safetyFactor: number; isSafe: boolean; description: string }>('calculate_buckling_safety', {
    load_factor: loadFactor,
    applied_load: appliedLoad,
    yield_strength: yieldStrength,
    material_factor: materialFactor
  })
}

/** 生成非线性屈曲INP文件（弧长法） */
export async function generateNonlinearBucklingInp(
  nodes: Node[],
  elements: Element[],
  material: Material,
  fixedBc: FixedBc,
  pointLoad: PointLoad | null,
  uniformLoads: UniformLoad[],
  config: BucklingConfig,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_nonlinear_buckling_inp', {
    nodes,
    elements,
    material,
    fixed_bc: fixedBc,
    point_load: pointLoad,
    uniform_loads: uniformLoads,
    config,
    output_path: outputPath
  })
}

// ============ CFD分析 API ============

/** 生成CFD INP输入文件 (简化版，生成CalculiX流体段) */
export async function generateCfdInp(
  nodes: Node[],
  elements: Element[],
  fluidProperties: FluidProperties,
  inletBoundaries: InletBoundary[],
  outletBoundaries: OutletBoundary[],
  wallBoundaries: WallBoundary[],
  config: CfdSimulationConfig,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_cfd_inp', {
    nodes,
    elements,
    fluid_properties: fluidProperties,
    inlet_boundaries: inletBoundaries,
    outlet_boundaries: outletBoundaries,
    wall_boundaries: wallBoundaries,
    config,
    output_path: outputPath
  })
}

/** 运行CFD求解器 */
export async function runCfdSolver(inputPath: string, workingDir: string): Promise<any> {
  return await invoke<any>('run_cfd_solver', { input_path: inputPath, working_dir: workingDir })
}

/** 解析CFD结果文件 */
export async function parseCfdResult(path: string): Promise<any> {
  return await invoke<any>('parse_cfd_result', { path })
}

// ============ 模态分析 API ============

/** 模态分析配置 */
export interface ModalConfig {
  num_modes: number
  min_frequency?: number
  max_frequency?: number
  lumped_mass?: boolean
}

/** 模态分析单阶振型信息 */
export interface ModalModeInfo {
  mode_number: number
  frequency_hz: number
  eigenvalue: number
}

/** 完整模态分析结果 */
export interface ModalResults {
  job_id: string
  num_nodes: number
  num_elements: number
  modes: ModalModeInfo[]
  warnings: string[]
  errors: string[]
}

/** 节点的振型位移数据 */
export interface ModeShapeNode {
  node_id: number
  dx: number
  dy: number
  dz: number
  displacement_magnitude: number
}

/** 单阶模态完整数据 */
export interface ModalMode {
  mode_number: number
  frequency_hz: number
  eigenvalue: number
  generalized_mass: number
  node_displacements: ModeShapeNode[]
}

/** 动画帧数据 */
export interface AnimationFrame {
  frame_index: number
  phase: number
  displacements: NodeDisplacement[]
}

/** 带坐标的节点位移 */
export interface NodeDisplacement {
  node_id: number
  x: number
  y: number
  z: number
  ux: number
  uy: number
  uz: number
  magnitude: number
}

/** 生成模态分析INP输入文件 */
export async function generateModalInp(
  nodes: Node[],
  elements: Element[],
  material: Material,
  numModes: number,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_modal_analysis_inp', {
    nodes,
    elements,
    material,
    num_modes: numModes,
    output_path: outputPath
  })
}

/** 运行模态分析求解器 */
export async function runModalSolver(
  inputPath: string,
  workingDir: string,
  numThreads?: number
): Promise<any> {
  return await invoke<any>('run_modal_solver', {
    input_file: inputPath,
    working_dir: workingDir,
    num_threads: numThreads
  })
}

/** 解析模态分析结果文件 */
export async function parseModalResults(frdFile: string): Promise<ModalResults> {
  return await invoke<ModalResults>('parse_modal_results', { frd_file: frdFile })
}

/** 获取模态列表（频率摘要） */
export async function getModalModeList(frdFile: string): Promise<ModalModeInfo[]> {
  return await invoke<ModalModeInfo[]>('get_modal_mode_list', { frd_file: frdFile })
}

/** 获取特定阶的振型数据 */
export async function getModeShape(frdFile: string, modeNumber: number): Promise<ModalMode> {
  return await invoke<ModalMode>('get_mode_shape', { frd_file: frdFile, mode_number: modeNumber })
}

/** 导出振型为位移结果格式 */
export async function exportModeShapeAsDisplacement(
  nodes: Node[],
  frdFile: string,
  modeNumber: number,
  scale: number
): Promise<NodeDisplacement[]> {
  return await invoke<NodeDisplacement[]>('export_mode_shape_as_displacement', {
    nodes,
    frd_file: frdFile,
    mode_number: modeNumber,
    scale
  })
}

/** 获取振型动画帧（用于播放动画） */
export async function getModeAnimationFrames(
  nodes: Node[],
  frdFile: string,
  modeNumber: number,
  numFrames: number,
  amplitude: number
): Promise<AnimationFrame[]> {
  return await invoke<AnimationFrame[]>('get_mode_animation_frames', {
    nodes,
    frd_file: frdFile,
    mode_number: modeNumber,
    num_frames: numFrames,
    amplitude
  })
}

// ============ Frequency Response Analysis API ============

/** Frequency response analysis configuration */
export interface FrequencyResponseConfig {
  start_frequency: number
  end_frequency: number
  num_steps: number
  damping_type: 'rayleigh' | 'modal'
  damping: FrequencyDamping
  solution_method: 'direct' | 'modal'
  num_modes?: number
  load_type: 'harmonic' | 'base_accel'
  load_amplitude?: number
  load_phase?: number
  base_acceleration?: BaseAcceleration
}

export interface FrequencyDamping {
  alpha?: number
  beta?: number
  modal_damping?: number
}

export interface BaseAcceleration {
  x: number
  y: number
  z: number
}

export interface FrequencyPoint {
  frequency: number
  displacement: number
  phase: number
}

export interface FrequencyResponseResult {
  job_id: string
  analysis_type: string
  start_frequency: number
  end_frequency: number
  num_steps: number
  frequency_points: FrequencyPoint[]
  max_displacement: number
  resonance_frequency?: number
  warnings: string[]
  errors: string[]
}

/** Generate frequency response INP file */
export async function generateFrequencyResponseInp(
  nodes: Node[],
  elements: Element[],
  material: Material,
  fixedBc: FixedBc,
  pointLoad: PointLoad | null,
  uniformLoads: UniformLoad[],
  config: FrequencyResponseConfig,
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_frequency_response_inp', {
    nodes,
    elements,
    material,
    fixed_bc: fixedBc,
    point_load: pointLoad,
    uniform_loads: uniformLoads,
    config,
    output_path: outputPath
  })
}

/** Run frequency response solver */
export async function runFrequencyResponseSolver(
  inputPath: string,
  workingDir: string
): Promise<any> {
  return await invoke('run_frequency_response_solver', {
    input_file: inputPath,
    working_dir: workingDir
  })
}

/** Parse frequency response results */
export async function parseFrequencyResponseResult(
  datFile: string,
  numPoints: number,
  startFrequency: number,
  endFrequency: number
): Promise<FrequencyResponseResult> {
  return await invoke<FrequencyResponseResult>('parse_frequency_response_result', {
    dat_file: datFile,
    num_points: numPoints,
    start_frequency: startFrequency,
    end_frequency: endFrequency
  })
}

// ============ 参数化扫描分析 API ============

/** 参数类型 */
export interface Parameter {
  name: string
  parameter_type: ParameterType
}

/** 参数类型 - 离散值列表 */
export interface DiscreteParameter {
  type: 'Discrete'
  values: number[]
}

/** 参数类型 - 范围+步长 */
export interface RangeParameter {
  type: 'Range'
  start: number
  end: number
  step: number
}

/** 参数类型 - 范围+等分数 */
export interface LinspaceParameter {
  type: 'Linspace'
  start: number
  end: number
  num_points: number
}

export type ParameterType = DiscreteParameter | RangeParameter | LinspaceParameter

/** 网格配置（参数化） */
export interface ParametricMeshConfig {
  dimension: '2d' | '3d'
  x_min: number
  x_max: number
  x_div: number
  y_min: number
  y_max: number
  y_div: number
  z_min?: number
  z_max?: number
  z_div?: number
  element_type: 'C3D8' | 'C3D4' | 'C2D4' | 'C2D3'
}

/** 材料配置（参数化） */
export interface ParametricMaterial {
  name: string
  elastic_modulus?: number  // 可参数化
  poisson_ratio: number
  density: number
}

/** 参数化扫描配置 */
export interface ParametricConfig {
  parameters: Parameter[]
  mesh_config: ParametricMeshConfig
  material: ParametricMaterial
  boundary_conditions: any[]  // BC类型
  loads: any[]  // 荷载类型
  output_dir: string
  max_parallel: number
  result_variable: 'max_stress' | 'max_displacement' | 'max_von_mises'
}

/** 单个扫描案例结果 */
export interface ScanCaseResult {
  case_id: number
  parameter_values: Record<string, number>
  input_file: string
  output_file?: string
  result_file?: string
  success: boolean
  max_stress?: number
  max_displacement?: number
  max_von_mises?: number
  elapsed_time_seconds?: number
  error_message?: string
}

/** 结果汇总 */
export interface ParametricSummary {
  result_variable: string
  min_value?: number
  max_value?: number
  min_case?: number
  max_case?: number
  parameter_influence: Record<string, number>
}

/** 扫描结果汇总 */
export interface ParametricScanResult {
  total_cases: number
  successful_cases: number
  failed_cases: number
  results: ScanCaseResult[]
  summary: ParametricSummary
}

/** 运行参数化扫描分析 */
export async function runParametricScan(config: ParametricConfig): Promise<ParametricScanResult> {
  return await invoke<ParametricScanResult>('run_parametric_scan_async', { config })
}

// ========== 优化设计 API ==========

/** 优化类型 */
export type OptimizationType = 'topology' | 'shape' | 'size'

/** 目标函数类型 */
export type ObjectiveType = 'min_compliance' | 'min_mass' | 'max_stiffness'

/** 约束类型 */
export type ConstraintType = 'VolumeFraction' | 'MaxStress'

/** 优化配置 */
export interface OptimizationConfig {
  optimization_type: OptimizationType
  objective: ObjectiveType
  constraints: Array<{
    constraint_type: ConstraintType
    upper_bound: number
  }>
  max_iterations: number
  convergence_tolerance: number
  design_domain: {
    x_min: number
    x_max: number
    y_min: number
    y_max: number
    z_min: number
    z_max: number
  }
  penalization_factor?: number
  min_density?: number
  size_parameters?: Array<{
    name: string
    lower_bound: number
    upper_bound: number
  }>
}

/** 优化迭代结果 */
export interface OptimizationIteration {
  iteration: number
  objective_value: number
  volume_fraction: number
  max_stress?: number
  density_field?: number[]
  converged: boolean
}

/** 优化结果 */
export interface OptimizationResult {
  success: boolean
  iterations: OptimizationIteration[]
  final_objective: number
  final_volume: number
  density_field?: number[]
  elapsed_time_seconds: number
  error_message?: string
}

/** 运行拓扑优化 */
export async function runTopologyOptimization(request: {
  config: OptimizationConfig
  nodes: Node[]
  elements: Element[]
  boundary_conditions: BcContainer
  material: {
    elastic_modulus: number
    poisson_ratio: number
    density: number
  }
}): Promise<OptimizationResult> {
  return await invoke<OptimizationResult>('run_topology_optimization', { request })
}

/** 运行形状优化 */
export async function runShapeOptimization(request: {
  config: OptimizationConfig
  nodes: Node[]
  elements: Element[]
  boundary_conditions: BcContainer
  material: {
    elastic_modulus: number
    poisson_ratio: number
    density: number
  }
}): Promise<OptimizationResult> {
  return await invoke<OptimizationResult>('run_shape_optimization', { request })
}

/** 运行尺寸优化 */
export async function runSizeOptimization(request: {
  config: OptimizationConfig
  nodes: Node[]
  elements: Element[]
  boundary_conditions: BcContainer
  material: {
    elastic_modulus: number
    poisson_ratio: number
    density: number
  }
}): Promise<OptimizationResult> {
  return await invoke<OptimizationResult>('run_size_optimization', { request })
}

/** 获取当前密度场 */
export async function getDensityField(iteration: number): Promise<number[]> {
  return await invoke<number[]>('get_density_field', { iteration })
}

/** 生成优化迭代INP文件 */
export async function generateOptimizationInp(
  nodes: Node[],
  elements: Element[],
  densityField: number[],
  outputPath: string
): Promise<string> {
  return await invoke<string>('generate_optimization_inp', {
    nodes,
    elements,
    densityField,
    outputPath
  })
}

// ============ 接触分析 API ============

/** Contact pair configuration */
export interface ContactConfig {
  master_surface: string
  slave_surface: string
  contact_type: string  // 'bonded' | 'frictionless' | 'frictional'
  friction_coefficient: number
  normal_stiffness?: number
  tangential_stiffness?: number
  algorithm: string  // 'penalty' | 'lagrange' | 'augmented_lagrange'
}

/** Contact diagnostic result */
export interface ContactDiagnostic {
  contact_pair: string
  level: 'none' | 'info' | 'warning' | 'error'
  issues: Array<{type: string; message: string}>
  recommendations: string[]
}

/** Create contact configuration */
export async function createContactConfig(
  masterSurface: string,
  slaveSurface: string,
  contactType: string,
  frictionCoefficient: number,
  normalStiffness?: number,
  algorithm?: string
): Promise<ContactConfig> {
  return await invoke<ContactConfig>('create_contact_config', {
    master_surface: masterSurface,
    slave_surface: slaveSurface,
    contact_type: contactType,
    friction_coefficient: frictionCoefficient,
    normal_stiffness: normalStiffness,
    algorithm: algorithm || 'penalty'
  })
}

/** Generate contact INP section */
export async function generateContactInp(configs: ContactConfig[]): Promise<string> {
  return await invoke<string>('generate_contact_inp', { configs })
}

/** Run contact diagnostics */
export async function diagnoseContactPairs(
  configs: ContactConfig[],
  numNodes: number,
  numElements: number
): Promise<ContactDiagnostic[]> {
  return await invoke<ContactDiagnostic[]>('diagnose_contact_pairs', {
    configs,
    num_nodes: numNodes,
    num_elements: numElements
  })
}

/** Get contact template INP */
export async function getContactTemplateInp(templateName: string): Promise<string> {
  return await invoke<string>('get_contact_template_inp', { template_name: templateName })
}

/** Generate surface definition for contact */
export async function generateSurfaceDef(
  surfaceName: string,
  surfaceType: 'element' | 'node',
  elementIds: number[],
  faces: string[]
): Promise<string> {
  return await invoke<string>('generate_surface_def', {
    surface_name: surfaceName,
    surface_type: surfaceType,
    element_ids: elementIds,
    faces
  })
}

/** Get contact algorithm recommendations */
export async function getContactAlgorithmRecommendations(): Promise<string[]> {
  return await invoke<string[]>('get_contact_algorithm_recommendations')
}

/** Get convergence improvement suggestions */
export async function getConvergenceSuggestions(
  iterations: number,
  maxPenetration: number,
  contactForceError: number
): Promise<string[]> {
  return await invoke<string[]>('get_convergence_suggestions', {
    iterations,
    max_penetration: maxPenetration,
    contact_force_error: contactForceError
  })
}

// ============ 多物理场耦合 (热-结构) API ============

export type ThermalCouplingAnalysisType = 'steady_state' | 'transient'

export interface ThermalCouplingMaterial {
  name: string
  density: number
  youngs_modulus: number
  poisson_ratio: number
  thermal_conductivity: number
  expansion_coefficient: number
  specific_heat: number
}

export interface ThermalCouplingMeshConfig {
  x_min: number; x_max: number; x_div: number
  y_min: number; y_max: number; y_div: number
  z_min: number; z_max: number; z_div: number
  element_type: 'C3D8' | 'C3D8R' | 'C3D4'
}

export type ThermalCouplingBcType = 'fixed_temperature' | 'heat_flux' | 'convection' | 'radiation' | 'insulation'

export interface ThermalBcConfig {
  name: string
  bc_type: ThermalCouplingBcType
  nodes: number[]
  surface_name?: string
  temperature?: number
  ambient_temperature?: number
  film_coefficient?: number
  heat_flux?: number
  emissivity?: number
}

export type HeatSourceType = 'point' | 'surface' | 'volume'

export interface HeatSourceConfig {
  name: string
  source_type: HeatSourceType
  magnitude: number
  node_ids?: number[]
  surface_name?: string
}

export interface ThermalAnalysisConfig {
  analysis_type: ThermalCouplingAnalysisType
  initial_temperature: number
  time_period: number
  time_increment: number
  max_iterations: number
  tolerance: number
  boundary_conditions: ThermalBcConfig[]
  heat_sources: HeatSourceConfig[]
}

export interface StructuralBcConfig {
  name: string
  bc_type: 'fixed' | 'symmetry'
  nodes: number[]
  fix_x: boolean
  fix_y: boolean
  fix_z: boolean
}

export interface StructuralAnalysisConfig {
  reference_temperature: number
  stress_free_temperature: number
  step_time: number
  boundary_conditions: StructuralBcConfig[]
}

export interface ThermalCouplingJob {
  id: string
  name: string
  coupling_type: 'sequential' | 'fully_coupled'
  mesh_config: ThermalCouplingMeshConfig
  thermal_config: ThermalAnalysisConfig
  structural_config: StructuralAnalysisConfig
  material: ThermalCouplingMaterial
  temperature_field?: Array<[number, number]>
  thermal_result_path?: string
  structural_result_path?: string
  status: string
}

export interface TemplateConfig {
  id: string
  name: string
  category: string
  description: string
  mesh_config: ThermalCouplingMeshConfig
  thermal_config: ThermalAnalysisConfig
  structural_config: StructuralAnalysisConfig
  material: ThermalCouplingMaterial
}

export interface ThermalResultData {
  node_temperatures: Array<[number, number]>
  max_temperature: number
  min_temperature: number
  avg_temperature: number
  result_path: string
}

export interface StructuralResultData {
  max_displacement: number
  max_stress: number
  max_von_mises: number
  result_path: string
}

export interface ThermalCouplingResult {
  job_id: string
  success: boolean
  thermal_results?: ThermalResultData
  structural_results?: StructuralResultData
  warnings: string[]
  errors: string[]
}

/** 生成热-结构耦合INP文件 */
export async function generateThermalCouplingInp(
  job: ThermalCouplingJob,
  workingDir: string
): Promise<string> {
  return await invoke<string>('generate_thermal_coupling_inp', {
    job,
    working_dir: workingDir
  })
}

/** 获取热-结构耦合模板列表 */
export async function getThermalCouplingTemplates(): Promise<TemplateConfig[]> {
  return await invoke<TemplateConfig[]>('get_thermal_coupling_templates')
}

/** 生成顺序耦合INP文件（热分析+结构分析） */
export async function generateSequentialCouplingInpFiles(
  thermalJob: ThermalCouplingJob,
  structuralJob: ThermalCouplingJob,
  workingDir: string
): Promise<[string, string]> {
  return await invoke<[string, string]>('generate_sequential_coupling_inp_files', {
    thermal_job: thermalJob,
    structural_job: structuralJob,
    working_dir: workingDir
  })
}

/** 解析热分析结果文件，提取温度场 */
export async function parseThermalResultFile(resultPath: string): Promise<ThermalResultData> {
  return await invoke<ThermalResultData>('parse_thermal_result_file', {
    result_path: resultPath
  })
}

/** 获取指定面上的节点ID列表 */
export async function getFaceNodes(
  x_min: number, x_max: number,
  y_min: number, y_max: number,
  z_min: number, z_max: number,
  x_div: number, y_div: number, z_div: number,
  face: 'x_min' | 'x_max' | 'y_min' | 'y_max' | 'z_min' | 'z_max' | 'all'
): Promise<number[]> {
  return await invoke<number[]>('get_face_nodes', {
    x_min, x_max, y_min, y_max, z_min, z_max,
    x_div, y_div, z_div, face
  })
}
