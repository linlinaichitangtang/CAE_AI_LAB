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
