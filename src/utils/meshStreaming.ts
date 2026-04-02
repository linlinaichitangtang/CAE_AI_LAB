/**
 * V1.2-002: 网格流式加载工具模块
 * 包含分块加载器、AABB 包围盒、网格块定义
 */
import * as THREE from 'three'

// ============================================================
// 接口定义
// ============================================================

export interface AABB {
  min: [number, number, number]
  max: [number, number, number]
}

export interface MeshChunk {
  index: number
  nodeCount: number
  elementCount: number
  bounds: AABB
  loaded: boolean
  loading: boolean
  geometry: THREE.BufferGeometry | null
  nodeIds: number[]
  elementIndices: number[]  // 原始 elements 数组中的索引
}

export interface MeshData {
  nodes: Array<{ id: number; x: number; y: number; z: number }>
  elements: Array<{ id: number; type: string; nodeIds: number[] }>
}

export interface LoadProgress {
  loaded: number
  total: number
  percentage: number
  loadedNodes: number
  loadedElements: number
  estimatedTimeRemaining: number  // 秒
}

export type ChunkLoadCallback = (chunk: MeshChunk, index: number) => void
export type ProgressCallback = (progress: LoadProgress) => void

// ============================================================
// AABB 工具函数
// ============================================================

export function createAABB(
  min: [number, number, number],
  max: [number, number, number]
): AABB {
  return { min: [...min], max: [...max] }
}

export function mergeAABB(a: AABB, b: AABB): AABB {
  return {
    min: [
      Math.min(a.min[0], b.min[0]),
      Math.min(a.min[1], b.min[1]),
      Math.min(a.min[2], b.min[2]),
    ],
    max: [
      Math.max(a.max[0], b.max[0]),
      Math.max(a.max[1], b.max[1]),
      Math.max(a.max[2], b.max[2]),
    ],
  }
}

export function aabbContainsPoint(aabb: AABB, point: [number, number, number]): boolean {
  return (
    point[0] >= aabb.min[0] && point[0] <= aabb.max[0] &&
    point[1] >= aabb.min[1] && point[1] <= aabb.max[1] &&
    point[2] >= aabb.min[2] && point[2] <= aabb.max[2]
  )
}

export function aabbCenter(aabb: AABB): [number, number, number] {
  return [
    (aabb.min[0] + aabb.max[0]) / 2,
    (aabb.min[1] + aabb.max[1]) / 2,
    (aabb.min[2] + aabb.max[2]) / 2,
  ]
}

export function aabbSize(aabb: AABB): [number, number, number] {
  return [
    aabb.max[0] - aabb.min[0],
    aabb.max[1] - aabb.min[1],
    aabb.max[2] - aabb.min[2],
  ]
}

export function aabbDistanceToPoint(aabb: AABB, point: [number, number, number]): number {
  const center = aabbCenter(aabb)
  const dx = center[0] - point[0]
  const dy = center[1] - point[1]
  const dz = center[2] - point[2]
  return Math.sqrt(dx * dx + dy * dy + dz * dz)
}

// ============================================================
// ChunkedMeshLoader - 分块网格加载器
// ============================================================

export class ChunkedMeshLoader {
  private chunks: MeshChunk[] = []
  private loadedChunks: Set<number> = new Set()
  private visibleChunks: Set<number> = new Set()
  private meshData: MeshData | null = null
  private chunkSize: number = 10000  // 每块最大单元数

  // 加载控制
  private loading = false
  private cancelled = false
  private loadQueue: number[] = []
  private currentLoadPromise: Promise<void> | null = null

  // 进度追踪
  private loadStartTime = 0
  private loadedNodeCount = 0
  private loadedElementCount = 0

  // 回调
  private onChunkLoaded: ChunkLoadCallback | null = null
  private onProgress: ProgressCallback | null = null

  // 空间索引：用于快速查找节点所属的块
  private nodeToChunkMap: Map<number, number> = new Map()

  constructor(chunkSize?: number) {
    if (chunkSize) this.chunkSize = chunkSize
  }

  /**
   * 将大网格分割为块
   * 基于空间八叉树分割，确保每块单元数不超过 chunkSize
   */
  splitIntoChunks(meshData: MeshData, chunkSize?: number): MeshChunk[] {
    if (chunkSize) this.chunkSize = chunkSize
    this.meshData = meshData
    this.chunks = []
    this.loadedChunks.clear()
    this.visibleChunks.clear()
    this.nodeToChunkMap.clear()

    // 计算全局包围盒
    const globalBounds = this.computeGlobalBounds(meshData.nodes)

    // 基于空间分割将单元分配到块中
    const elementGroups = this.spatialPartition(meshData, globalBounds)

    // 创建 MeshChunk 对象
    elementGroups.forEach((elements, chunkIdx) => {
      const nodeIds = new Set<number>()
      elements.forEach(el => el.nodeIds.forEach(nid => nodeIds.add(nid)))

      // 计算块的包围盒
      let chunkBounds: AABB = {
        min: [Infinity, Infinity, Infinity],
        max: [-Infinity, -Infinity, -Infinity],
      }
      nodeIds.forEach(nid => {
        const node = meshData.nodes.find(n => n.id === nid)
        if (node) {
          chunkBounds.min[0] = Math.min(chunkBounds.min[0], node.x)
          chunkBounds.min[1] = Math.min(chunkBounds.min[1], node.y)
          chunkBounds.min[2] = Math.min(chunkBounds.min[2], node.z)
          chunkBounds.max[0] = Math.max(chunkBounds.max[0], node.x)
          chunkBounds.max[1] = Math.max(chunkBounds.max[1], node.y)
          chunkBounds.max[2] = Math.max(chunkBounds.max[2], node.z)
        }
      })

      // 记录节点到块的映射
      nodeIds.forEach(nid => this.nodeToChunkMap.set(nid, chunkIdx))

      const elementIndices = elements.map(el => meshData.elements.indexOf(el))

      this.chunks.push({
        index: chunkIdx,
        nodeCount: nodeIds.size,
        elementCount: elements.length,
        bounds: chunkBounds,
        loaded: false,
        loading: false,
        geometry: null,
        nodeIds: Array.from(nodeIds),
        elementIndices,
      })
    })

    return this.chunks
  }

  /**
   * 异步加载单个块
   */
  async loadChunk(chunkIndex: number): Promise<THREE.BufferGeometry | null> {
    const chunk = this.chunks[chunkIndex]
    if (!chunk || chunk.loaded || chunk.loading || this.cancelled) return null

    chunk.loading = true

    try {
      // 使用 setTimeout 让出主线程，避免阻塞 UI
      const geometry = await new Promise<THREE.BufferGeometry>((resolve) => {
        setTimeout(() => {
          const geo = this.buildChunkGeometry(chunk)
          resolve(geo)
        }, 0)
      })

      chunk.geometry = geometry
      chunk.loaded = true
      chunk.loading = false
      this.loadedChunks.add(chunkIndex)

      // 更新统计
      this.loadedNodeCount += chunk.nodeCount
      this.loadedElementCount += chunk.elementCount

      // 回调
      if (this.onChunkLoaded) {
        this.onChunkLoaded(chunk, chunkIndex)
      }

      this.reportProgress()

      return geometry
    } catch (error) {
      chunk.loading = false
      console.error(`Failed to load chunk ${chunkIndex}:`, error)
      return null
    }
  }

  /**
   * 异步按优先级批量加载块
   */
  async loadChunksByPriority(
    priorityOrder: number[],
    onChunkLoaded?: ChunkLoadCallback
  ): Promise<void> {
    this.loading = true
    this.cancelled = false
    this.loadStartTime = performance.now()
    this.onChunkLoaded = onChunkLoaded || null

    for (const chunkIdx of priorityOrder) {
      if (this.cancelled) break
      if (this.chunks[chunkIdx]?.loaded) continue

      await this.loadChunk(chunkIdx)
    }

    this.loading = false
  }

  /**
   * 根据视口确定需要加载的块（按距离排序）
   */
  getVisibleChunks(camera: THREE.Camera, chunkBounds?: AABB[]): number[] {
    if (!this.meshData || this.chunks.length === 0) return []

    const cameraPos = new THREE.Vector3()
    camera.getWorldPosition(cameraPos)
    const camPoint: [number, number, number] = [cameraPos.x, cameraPos.y, cameraPos.z]

    // 使用 Three.js Frustum 进行视锥体裁剪
    const frustum = new THREE.Frustum()
    const projScreenMatrix = new THREE.Matrix4()
    projScreenMatrix.multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse)
    frustum.setFromProjectionMatrix(projScreenMatrix)

    // 筛选视口内的块并按距离排序
    const visibleWithDistance: Array<{ index: number; distance: number }> = []

    for (let i = 0; i < this.chunks.length; i++) {
      const chunk = this.chunks[i]

      // 检查是否在视锥体内
      const center = aabbCenter(chunk.bounds)
      const size = aabbSize(chunk.bounds)
      const box = new THREE.Box3(
        new THREE.Vector3(chunk.bounds.min[0], chunk.bounds.min[1], chunk.bounds.min[2]),
        new THREE.Vector3(chunk.bounds.max[0], chunk.bounds.max[1], chunk.bounds.max[2])
      )

      if (frustum.intersectsBox(box)) {
        const distance = aabbDistanceToPoint(chunk.bounds, camPoint)
        visibleWithDistance.push({ index: i, distance })
      }
    }

    // 按距离排序（近的优先）
    visibleWithDistance.sort((a, b) => a.distance - b.distance)

    // 更新可见块集合
    this.visibleChunks = new Set(visibleWithDistance.map(v => v.index))

    return visibleWithDistance.map(v => v.index)
  }

  /**
   * 卸载不可见的块释放内存
   * @param visibleChunkIds 可见的块 ID 列表（可选，如果不传则使用内部 visibleChunks）
   */
  unloadInvisibleChunks(visibleChunkIds?: number[]): number {
    if (visibleChunkIds) {
      this.visibleChunks = new Set(visibleChunkIds)
    }

    let unloadedCount = 0

    for (let i = 0; i < this.chunks.length; i++) {
      const chunk = this.chunks[i]
      if (chunk.loaded && !this.visibleChunks.has(i)) {
        this.disposeChunk(i)
        unloadedCount++
      }
    }

    return unloadedCount
  }

  /**
   * 取消加载
   */
  cancelLoading(): void {
    this.cancelled = true
    this.loading = false
  }

  /**
   * 取消加载（别名，兼容不同调用方式）
   */
  cancel(): void {
    this.cancelLoading()
  }

  /**
   * 获取加载进度
   */
  getLoadProgress(): LoadProgress {
    const total = this.chunks.length
    const loaded = this.loadedChunks.size
    const percentage = total > 0 ? (loaded / total) * 100 : 0

    // 估算剩余时间
    let estimatedTimeRemaining = 0
    if (loaded > 0 && this.loadStartTime > 0) {
      const elapsed = (performance.now() - this.loadStartTime) / 1000
      const avgTimePerChunk = elapsed / loaded
      estimatedTimeRemaining = avgTimePerChunk * (total - loaded)
    }

    return {
      loaded,
      total,
      percentage: Math.round(percentage * 10) / 10,
      loadedNodes: this.loadedNodeCount,
      loadedElements: this.loadedElementCount,
      estimatedTimeRemaining: Math.round(estimatedTimeRemaining * 10) / 10,
    }
  }

  /**
   * 获取所有块
   */
  getChunks(): MeshChunk[] {
    return this.chunks
  }

  /**
   * 获取已加载的块
   */
  getLoadedChunks(): MeshChunk[] {
    return this.chunks.filter(c => c.loaded)
  }

  /**
   * 是否正在加载
   */
  isLoading(): boolean {
    return this.loading
  }

  /**
   * 设置进度回调
   */
  setProgressCallback(callback: ProgressCallback): void {
    this.onProgress = callback
  }

  /**
   * 释放所有资源
   */
  dispose(): void {
    this.cancelLoading()
    for (let i = 0; i < this.chunks.length; i++) {
      this.disposeChunk(i)
    }
    this.chunks = []
    this.loadedChunks.clear()
    this.visibleChunks.clear()
    this.nodeToChunkMap.clear()
    this.meshData = null
  }

  // ---- 私有方法 ----

  private computeGlobalBounds(
    nodes: Array<{ id: number; x: number; y: number; z: number }>
  ): AABB {
    const bounds: AABB = {
      min: [Infinity, Infinity, Infinity],
      max: [-Infinity, -Infinity, -Infinity],
    }
    nodes.forEach(n => {
      bounds.min[0] = Math.min(bounds.min[0], n.x)
      bounds.min[1] = Math.min(bounds.min[1], n.y)
      bounds.min[2] = Math.min(bounds.min[2], n.z)
      bounds.max[0] = Math.max(bounds.max[0], n.x)
      bounds.max[1] = Math.max(bounds.max[1], n.y)
      bounds.max[2] = Math.max(bounds.max[2], n.z)
    })
    return bounds
  }

  /**
   * 空间分割：将单元按空间位置分组
   * 使用简单的网格分割策略
   */
  private spatialPartition(
    meshData: MeshData,
    globalBounds: AABB
  ): Array<Array<{ id: number; type: string; nodeIds: number[] }>> {
    const size = aabbSize(globalBounds)
    const maxDim = Math.max(size[0], size[1], size[2])

    // 计算分割的网格维度
    const totalElements = meshData.elements.length
    const targetChunks = Math.max(1, Math.ceil(totalElements / this.chunkSize))
    const gridDim = Math.max(1, Math.ceil(Math.cbrt(targetChunks)))

    const cellSize = maxDim / gridDim
    const origin = globalBounds.min

    // 将单元分配到网格单元中
    const cellMap = new Map<string, Array<{ id: number; type: string; nodeIds: number[] }>>()

    meshData.elements.forEach(el => {
      // 计算单元中心
      let cx = 0, cy = 0, cz = 0
      el.nodeIds.forEach(nid => {
        const node = meshData.nodes.find(n => n.id === nid)
        if (node) {
          cx += node.x
          cy += node.y
          cz += node.z
        }
      })
      cx /= el.nodeIds.length
      cy /= el.nodeIds.length
      cz /= el.nodeIds.length

      // 确定网格单元坐标
      const gx = Math.min(gridDim - 1, Math.max(0, Math.floor((cx - origin[0]) / cellSize)))
      const gy = Math.min(gridDim - 1, Math.max(0, Math.floor((cy - origin[1]) / cellSize)))
      const gz = Math.min(gridDim - 1, Math.max(0, Math.floor((cz - origin[2]) / cellSize)))

      const key = `${gx}_${gy}_${gz}`
      if (!cellMap.has(key)) {
        cellMap.set(key, [])
      }
      cellMap.get(key)!.push(el)
    })

    // 合并过小的块（单元数少于 chunkSize/4 的相邻块）
    const groups = Array.from(cellMap.values())
    const mergedGroups: Array<Array<{ id: number; type: string; nodeIds: number[] }>> = []

    for (const group of groups) {
      if (group.length < this.chunkSize / 4 && mergedGroups.length > 0) {
        // 合并到上一个组
        mergedGroups[mergedGroups.length - 1].push(...group)
      } else {
        mergedGroups.push(group)
      }
    }

    // 如果某个组仍然超过 chunkSize，进一步分割
    const finalGroups: Array<Array<{ id: number; type: string; nodeIds: number[] }>> = []
    for (const group of mergedGroups) {
      if (group.length <= this.chunkSize) {
        finalGroups.push(group)
      } else {
        // 简单地按 chunkSize 分割
        for (let i = 0; i < group.length; i += this.chunkSize) {
          finalGroups.push(group.slice(i, i + this.chunkSize))
        }
      }
    }

    return finalGroups
  }

  /**
   * 为单个块构建 BufferGeometry
   */
  private buildChunkGeometry(chunk: MeshChunk): THREE.BufferGeometry {
    if (!this.meshData) {
      return new THREE.BufferGeometry()
    }

    const positions: number[] = []
    const normals: number[] = []
    const indices: number[] = []
    let idx = 0
    const nodeIndexMap = new Map<number, number>()

    // 构建节点映射
    chunk.nodeIds.forEach(nid => {
      if (!nodeIndexMap.has(nid)) {
        nodeIndexMap.set(nid, idx++)
      }
    })

    // 获取节点位置
    const nodePositions = new Map<number, THREE.Vector3>()
    this.meshData.nodes.forEach(n => {
      if (nodeIndexMap.has(n.id)) {
        nodePositions.set(n.id, new THREE.Vector3(n.x, n.y, n.z))
      }
    })

    // 为每个单元生成三角形
    chunk.elementIndices.forEach(elIdx => {
      const el = this.meshData!.elements[elIdx]
      if (!el) return

      const positionsLocal: THREE.Vector3[] = []
      el.nodeIds.forEach(nid => {
        positionsLocal.push(nodePositions.get(nid) || new THREE.Vector3())
      })

      // 三角化（与 ResultViewer.buildGeometry 相同的逻辑）
      if (el.type === 'TET4' || el.type === 'TETRA') {
        const faces = [
          [0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]
        ]
        faces.forEach(f => {
          const a = positionsLocal[f[0]]
          const b = positionsLocal[f[1]]
          const c = positionsLocal[f[2]]

          const ab = new THREE.Vector3().subVectors(b, a)
          const ac = new THREE.Vector3().subVectors(c, a)
          const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

          ;[a, b, c].forEach(v => {
            positions.push(v.x, v.y, v.z)
            normals.push(normal.x, normal.y, normal.z)
          })
          indices.push(idx - 3, idx - 2, idx - 1)
          idx += 3
        })
      } else if (el.type === 'HEX8' || el.type === 'BRICK') {
        const v = positionsLocal
        const faces = [
          [0, 1, 2], [0, 2, 3],
          [4, 6, 5], [4, 7, 6],
          [0, 3, 7], [0, 7, 4],
          [1, 5, 6], [1, 6, 2],
          [0, 4, 5], [0, 5, 1],
          [3, 2, 6], [3, 6, 7]
        ]
        faces.forEach(fi => {
          const a = v[fi[0]]
          const b = v[fi[1]]
          const c = v[fi[2]]

          const ab = new THREE.Vector3().subVectors(b, a)
          const ac = new THREE.Vector3().subVectors(c, a)
          const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

          ;[a, b, c].forEach(v => {
            positions.push(v.x, v.y, v.z)
            normals.push(normal.x, normal.y, normal.z)
          })
          indices.push(idx, idx + 1, idx + 2)
          idx += 3
        })
      } else {
        for (let i = 1; i < el.nodeIds.length - 1; i++) {
          const a = positionsLocal[0]
          const b = positionsLocal[i]
          const c = positionsLocal[i + 1]

          const ab = new THREE.Vector3().subVectors(b, a)
          const ac = new THREE.Vector3().subVectors(c, a)
          const normal = new THREE.Vector3().crossVectors(ab, ac).normalize()

          ;[a, b, c].forEach(v => {
            positions.push(v.x, v.y, v.z)
            normals.push(normal.x, normal.y, normal.z)
          })
          indices.push(idx, idx + 1, idx + 2)
          idx += 3
        }
      }
    })

    const geometry = new THREE.BufferGeometry()
    geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
    geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))
    geometry.setIndex(indices)
    geometry.computeVertexNormals()
    geometry.computeBoundingSphere()

    return geometry
  }

  private disposeChunk(index: number): void {
    const chunk = this.chunks[index]
    if (chunk && chunk.geometry) {
      chunk.geometry.dispose()
      chunk.geometry = null
      chunk.loaded = false
      this.loadedChunks.delete(index)
    }
  }

  private reportProgress(): void {
    if (this.onProgress) {
      this.onProgress(this.getLoadProgress())
    }
  }
}
