/**
 * CSG (Constructive Solid Geometry) utilities - Simplified version
 * 布尔运算：并集、差集、交集
 * 使用简单包围盒方法实现
 */

import * as THREE from 'three'

export interface CSGResult {
  group: THREE.Group
  dimensions: string
  params: any
}

/**
 * 从GeometryItem提取Mesh
 */
function extractMesh(item: any): THREE.Mesh | null {
  if (!item.group) return null
  
  for (const child of item.group.children) {
    if (child instanceof THREE.Mesh) {
      return child.clone()
    }
  }
  
  return null
}

/**
 * 计算包围盒尺寸
 */
function computeDimensions(geometry: THREE.BufferGeometry): string {
  geometry.computeBoundingBox()
  const box = geometry.boundingBox!
  const w = box.max.x - box.min.x
  const h = box.max.y - box.min.y
  const d = box.max.z - box.min.z
  
  return `${w.toFixed(1)}×${h.toFixed(1)}×${d.toFixed(1)}`
}

/**
 * 计算包围盒中心
 */
function computeCenter(geometry: THREE.BufferGeometry): THREE.Vector3 {
  geometry.computeBoundingBox()
  const box = geometry.boundingBox!
  return new THREE.Vector3(
    (box.min.x + box.max.x) / 2,
    (box.min.y + box.max.y) / 2,
    (box.min.z + box.max.z) / 2
  )
}

/**
 * 创建布尔结果Group
 */
function createResultGroup(geometry: THREE.BufferGeometry, position: THREE.Vector3, color: number = 0x00aaff): THREE.Group {
  const edges = new THREE.EdgesGeometry(geometry)
  const lineMaterial = new THREE.LineBasicMaterial({ color: 0x00ffff, linewidth: 1 })
  const wireframe = new THREE.LineSegments(edges, lineMaterial)
  
  const material = new THREE.MeshPhongMaterial({
    color: color,
    transparent: true,
    opacity: 0.4,
    side: THREE.DoubleSide
  })
  const mesh = new THREE.Mesh(geometry.clone(), material)
  
  const group = new THREE.Group()
  group.add(mesh)
  group.add(wireframe)
  group.position.copy(position)
  
  return group
}

/**
 * 执行布尔运算
 * @param items 参与运算的几何体数组
 * @param operation 'union' | 'subtract' | 'intersect'
 */
export async function performCSG(
  items: any[],
  operation: 'union' | 'subtract' | 'intersect'
): Promise<CSGResult | null> {
  if (items.length < 2) {
    throw new Error('至少需要两个几何体进行布尔运算')
  }
  
  const meshes = items.map(extractMesh).filter(Boolean) as THREE.Mesh[]
  
  if (meshes.length < 2) {
    throw new Error('没有足够的有效几何体进行布尔运算')
  }
  
  // 计算所有几何体的总包围盒
  let minX = Infinity, minY = Infinity, minZ = Infinity
  let maxX = -Infinity, maxY = -Infinity, maxZ = -Infinity
  
  for (const mesh of meshes) {
    const box = new THREE.Box3().setFromObject(mesh)
    minX = Math.min(minX, box.min.x)
    minY = Math.min(minY, box.min.y)
    minZ = Math.min(minZ, box.min.z)
    maxX = Math.max(maxX, box.max.x)
    maxY = Math.max(maxY, box.max.y)
    maxZ = Math.max(maxZ, box.max.z)
  }
  
  const centerX = (minX + maxX) / 2
  const centerY = (minY + maxY) / 2
  const centerZ = (minZ + maxZ) / 2
  const center = new THREE.Vector3(centerX, centerY, centerZ)
  
  let geometry: THREE.BufferGeometry
  let dimensions: string
  
  switch (operation) {
    case 'union':
      // 并集：使用完整包围盒
      geometry = new THREE.BoxGeometry(maxX - minX, maxY - minY, maxZ - minZ)
      dimensions = `${(maxX - minX).toFixed(1)}×${(maxY - minY).toFixed(1)}×${(maxZ - minZ).toFixed(1)}`
      break
      
    case 'subtract':
      // 差集：使用第一个几何体
      geometry = meshes[0].geometry.clone()
      geometry.applyMatrix4(meshes[0].matrixWorld)
      dimensions = computeDimensions(geometry)
      break
      
    case 'intersect':
      // 交集：使用第一个几何体
      geometry = meshes[0].geometry.clone()
      geometry.applyMatrix4(meshes[0].matrixWorld)
      dimensions = computeDimensions(geometry)
      break
      
    default:
      throw new Error(`Unknown operation: ${operation}`)
  }
  
  // 居中几何体
  const geoCenter = computeCenter(geometry)
  geometry.translate(-geoCenter.x, -geoCenter.y, -geoCenter.z)
  
  // 创建Group
  const group = createResultGroup(geometry, center)
  
  return {
    group,
    dimensions,
    params: { ...items[0].params }
  }
}