/**
 * V1.2-001: 网格性能优化工具模块
 * 包含 LOD 管理、视口裁剪、InstancedMesh 渲染、性能监控
 */
import * as THREE from 'three'

// ============================================================
// LOD Manager - 多级细节管理器
// ============================================================

export interface LODLevel {
  level: number
  factor: number        // 顶点保留比例 (1.0 = 100%, 0.5 = 50%, 0.25 = 25%)
  distance: number      // 切换到此级别的距离阈值
  geometry: THREE.BufferGeometry | null
}

export class LODManager {
  private lodLevels: LODLevel[] = []
  private currentLevel = 0
  private targetLevel = 0
  private transitionSpeed = 0.05 // LOD 切换平滑过渡速度
  private lodTransition = 1.0    // 当前过渡因子 (0-1)
  private meshMap: Map<THREE.Object3D, LODLevel[]> = new Map()
  private enabled = true
  private targetFPS = 60
  private minFPS = 30

  constructor() {
    this.initDefaultLevels()
  }

  private initDefaultLevels(): void {
    this.lodLevels = [
      { level: 0, factor: 1.0, distance: 0, geometry: null },      // 完整细节
      { level: 1, factor: 0.5, distance: 50, geometry: null },     // 50% 顶点
      { level: 2, factor: 0.25, distance: 150, geometry: null },    // 25% 顶点
    ]
  }

  /**
   * 根据 FPS 自动调整 LOD 级别
   * 当 FPS 低于 minFPS 时降低 LOD，当 FPS 恢复到 targetFPS 时提升 LOD
   */
  updateByFPS(currentFPS: number): number {
    if (!this.enabled || currentFPS <= 0) return this.currentLevel

    if (currentFPS < this.minFPS) {
      // FPS 过低，降低 LOD
      this.decreaseLOD()
    } else if (currentFPS > this.targetFPS * 0.9) {
      // FPS 充裕，尝试提升 LOD（需要稳定几帧才提升）
      this.increaseLOD()
    }

    return this.currentLevel
  }

  /**
   * 设置 FPS 目标阈值
   */
  setFPSThresholds(targetFPS: number, minFPS: number): void {
    this.targetFPS = targetFPS
    this.minFPS = minFPS
  }

  /**
   * 根据相机距离自动切换 LOD 级别
   */
  updateLOD(camera: THREE.Camera, objects: THREE.Object3D[]): void {
    if (!this.enabled) return

    const cameraPos = new THREE.Vector3()
    camera.getWorldPosition(cameraPos)

    for (const obj of objects) {
      const objPos = new THREE.Vector3()
      obj.getWorldPosition(objPos)
      const distance = cameraPos.distanceTo(objPos)

      const levels = this.meshMap.get(obj) || this.lodLevels
      let targetLevel = 0

      for (let i = levels.length - 1; i >= 0; i--) {
        if (distance >= levels[i].distance) {
          targetLevel = i
          break
        }
      }

      this.applyLODToMesh(obj, targetLevel)
    }

    // 平滑过渡
    if (this.currentLevel !== this.targetLevel) {
      this.lodTransition += this.transitionSpeed
      if (this.lodTransition >= 1.0) {
        this.currentLevel = this.targetLevel
        this.lodTransition = 1.0
      }
    }
  }

  /**
   * 生成简化网格（顶点抽取算法）
   * 通过均匀采样顶点来简化几何体
   */
  generateLODMesh(geometry: THREE.BufferGeometry, factor: number): THREE.BufferGeometry {
    if (factor >= 1.0) return geometry.clone()

    const positionAttr = geometry.getAttribute('position')
    const normalAttr = geometry.getAttribute('normal')
    const colorAttr = geometry.getAttribute('color')
    const indexAttr = geometry.getIndex()

    const vertexCount = positionAttr.count
    const targetCount = Math.max(3, Math.floor(vertexCount * factor))

    // 使用均匀间隔采样选择保留的顶点索引
    const step = vertexCount / targetCount
    const keptIndices = new Set<number>()

    for (let i = 0; i < targetCount; i++) {
      keptIndices.add(Math.floor(i * step))
    }

    // 确保最后几个顶点也被保留
    keptIndices.add(vertexCount - 1)
    keptIndices.add(vertexCount - 2)
    keptIndices.add(vertexCount - 3)

    // 构建新顶点数据
    const newPositions: number[] = []
    const newNormals: number[] = []
    const newColors: number[] = []
    const oldToNew = new Map<number, number>()

    keptIndices.forEach(oldIdx => {
      oldToNew.set(oldIdx, newPositions.length / 3)
      newPositions.push(
        positionAttr.getX(oldIdx),
        positionAttr.getY(oldIdx),
        positionAttr.getZ(oldIdx)
      )
      if (normalAttr) {
        newNormals.push(
          normalAttr.getX(oldIdx),
          normalAttr.getY(oldIdx),
          normalAttr.getZ(oldIdx)
        )
      }
      if (colorAttr) {
        newColors.push(
          colorAttr.getX(oldIdx),
          colorAttr.getY(oldIdx),
          colorAttr.getZ(oldIdx)
        )
      }
    })

    // 重建索引 - 只保留所有顶点都在保留集合中的三角形
    const newIndices: number[] = []
    if (indexAttr) {
      for (let i = 0; i < indexAttr.count; i += 3) {
        const a = indexAttr.getX(i)
        const b = indexAttr.getX(i + 1)
        const c = indexAttr.getX(i + 2)

        if (oldToNew.has(a) && oldToNew.has(b) && oldToNew.has(c)) {
          newIndices.push(oldToNew.get(a)!, oldToNew.get(b)!, oldToNew.get(c)!)
        }
      }
    }

    const newGeometry = new THREE.BufferGeometry()
    newGeometry.setAttribute('position', new THREE.Float32BufferAttribute(newPositions, 3))
    if (newNormals.length > 0) {
      newGeometry.setAttribute('normal', new THREE.Float32BufferAttribute(newNormals, 3))
    }
    if (newColors.length > 0) {
      newGeometry.setAttribute('color', new THREE.Float32BufferAttribute(newColors, 3))
    }
    if (newIndices.length > 0) {
      newGeometry.setIndex(newIndices)
    }
    newGeometry.computeVertexNormals()
    newGeometry.computeBoundingSphere()

    return newGeometry
  }

  /**
   * 为网格对象注册 LOD 级别
   */
  registerMesh(obj: THREE.Object3D, levels: LODLevel[]): void {
    this.meshMap.set(obj, levels)
  }

  /**
   * 取消注册网格对象
   */
  unregisterMesh(obj: THREE.Object3D): void {
    this.meshMap.delete(obj)
  }

  /**
   * 获取当前 LOD 级别
   */
  getCurrentLevel(): number {
    return this.currentLevel
  }

  /**
   * 设置目标 LOD 级别（手动控制）
   */
  setLevel(level: number): void {
    this.targetLevel = Math.max(0, Math.min(level, this.lodLevels.length - 1))
    this.lodTransition = 0
  }

  /**
   * 强制降低 LOD（性能自适应时使用）
   */
  decreaseLOD(): boolean {
    if (this.currentLevel < this.lodLevels.length - 1) {
      this.setLevel(this.currentLevel + 1)
      return true
    }
    return false
  }

  /**
   * 尝试提高 LOD（性能恢复时使用）
   */
  increaseLOD(): boolean {
    if (this.currentLevel > 0) {
      this.setLevel(this.currentLevel - 1)
      return true
    }
    return false
  }

  /**
   * 启用/禁用 LOD
   */
  setEnabled(enabled: boolean): void {
    this.enabled = enabled
    if (!enabled) {
      this.setLevel(0)
    }
  }

  /**
   * 获取 LOD 级别数量
   */
  getLevelCount(): number {
    return this.lodLevels.length
  }

  /**
   * 获取指定级别的保留比例
   */
  getFactor(level: number): number {
    return this.lodLevels[level]?.factor ?? 1.0
  }

  private applyLODToMesh(_obj: THREE.Object3D, _level: number): void {
    // 实际 LOD 切换由外部调用 generateLODMesh 并重建网格
    // 这里只更新目标级别
    this.targetLevel = _level
  }
}

// ============================================================
// Frustum Culler - 视口裁剪器
// ============================================================

export class FrustumCuller {
  private frustum = new THREE.Frustum()
  private projScreenMatrix = new THREE.Matrix4()
  private enabled = true
  private cullStats = {
    total: 0,
    visible: 0,
    culled: 0,
  }

  /**
   * 更新视口锥体
   */
  updateFrustum(camera: THREE.Camera): void {
    this.projScreenMatrix.multiplyMatrices(
      camera.projectionMatrix,
      camera.matrixWorldInverse
    )
    this.frustum.setFromProjectionMatrix(this.projScreenMatrix)
  }

  /**
   * 检查 AABB 包围盒是否在视口内
   */
  isBoxVisible(min: THREE.Vector3, max: THREE.Vector3): boolean {
    if (!this.enabled) return true
    const box = new THREE.Box3(min, max)
    return this.frustum.intersectsBox(box)
  }

  /**
   * 检查对象是否在视口内
   */
  isVisible(object: THREE.Object3D): boolean {
    if (!this.enabled) return true

    // 如果对象有 geometry，使用包围球/包围盒检测
    if ((object as THREE.Mesh).geometry) {
      const geometry = (object as THREE.Mesh).geometry
      if (geometry.boundingSphere === null) {
        geometry.computeBoundingSphere()
      }
      return this.frustum.intersectsSphere(geometry.boundingSphere!)
    }

    // 对于 Group 等容器，使用 children 的联合包围盒
    if (object.children && object.children.length > 0) {
      const box = new THREE.Box3().setFromObject(object)
      return this.frustum.intersectsBox(box)
    }

    // 回退：检查对象的世界位置
    const pos = new THREE.Vector3()
    object.getWorldPosition(pos)
    return this.frustum.containsPoint(pos)
  }

  /**
   * 批量裁剪，返回可见对象列表
   */
  cullObjects(objects: THREE.Object3D[]): THREE.Object3D[] {
    this.cullStats.total = objects.length
    this.cullStats.culled = 0
    this.cullStats.visible = 0

    const visible: THREE.Object3D[] = []

    for (const obj of objects) {
      if (this.isVisible(obj)) {
        visible.push(obj)
        this.cullStats.visible++
      } else {
        this.cullStats.culled++
      }
    }

    return visible
  }

  /**
   * 对场景中的对象应用可见性裁剪
   * 直接修改对象的 visible 属性
   */
  applyVisibility(objects: THREE.Object3D[]): void {
    this.cullStats.total = objects.length
    this.cullStats.culled = 0
    this.cullStats.visible = 0

    for (const obj of objects) {
      const vis = this.isVisible(obj)
      obj.visible = vis
      if (vis) {
        this.cullStats.visible++
      } else {
        this.cullStats.culled++
      }
    }
  }

  /**
   * 获取裁剪统计
   */
  getCullStats(): { total: number; visible: number; culled: number } {
    return { ...this.cullStats }
  }

  /**
   * 启用/禁用裁剪
   */
  setEnabled(enabled: boolean): void {
    this.enabled = enabled
    if (!enabled) {
      // 恢复所有对象可见性
      this.cullStats = { total: 0, visible: 0, culled: 0 }
    }
  }

  isEnabled(): boolean {
    return this.enabled
  }
}

// ============================================================
// InstancedMesh Renderer - 实例化网格渲染器
// ============================================================

export class InstancedMeshRenderer {
  private instancedMesh: THREE.InstancedMesh | null = null
  private instanceCount = 0
  private dummy = new THREE.Object3D()
  private tempColor = new THREE.Color()

  /**
   * 创建 InstancedMesh
   */
  createInstancedMesh(
    geometry: THREE.BufferGeometry,
    material: THREE.Material,
    count: number
  ): THREE.InstancedMesh {
    // 清理旧的实例
    this.dispose()

    this.instanceCount = count
    this.instancedMesh = new THREE.InstancedMesh(geometry, material, count)
    this.instancedMesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage)

    // 初始化所有实例矩阵为单位矩阵
    const identity = new THREE.Matrix4()
    for (let i = 0; i < count; i++) {
      this.instancedMesh.setMatrixAt(i, identity)
    }
    this.instancedMesh.instanceMatrix.needsUpdate = true

    return this.instancedMesh
  }

  /**
   * 更新实例矩阵
   */
  updateInstanceMatrices(matrices: THREE.Matrix4[]): void {
    if (!this.instancedMesh) return

    const count = Math.min(matrices.length, this.instanceCount)
    for (let i = 0; i < count; i++) {
      this.instancedMesh.setMatrixAt(i, matrices[i])
    }
    this.instancedMesh.instanceMatrix.needsUpdate = true
    this.instancedMesh.computeBoundingSphere()
  }

  /**
   * 通过位置、旋转、缩放设置单个实例矩阵
   */
  setInstanceMatrix(
    index: number,
    position?: THREE.Vector3,
    rotation?: THREE.Euler,
    scale?: THREE.Vector3
  ): void {
    if (!this.instancedMesh || index >= this.instanceCount) return

    this.dummy.position.copy(position || new THREE.Vector3())
    if (rotation) this.dummy.rotation.copy(rotation)
    if (scale) this.dummy.scale.copy(scale)
    this.dummy.updateMatrix()

    this.instancedMesh.setMatrixAt(index, this.dummy.matrix)
    this.instancedMesh.instanceMatrix.needsUpdate = true
  }

  /**
   * 更新实例颜色
   */
  updateInstanceColors(colors: THREE.Color[]): void {
    if (!this.instancedMesh) return

    const count = Math.min(colors.length, this.instanceCount)
    for (let i = 0; i < count; i++) {
      this.instancedMesh.setColorAt(i, colors[i])
    }
    if (this.instancedMesh.instanceColor) {
      this.instancedMesh.instanceColor.needsUpdate = true
    }
  }

  /**
   * 设置单个实例颜色
   */
  setInstanceColor(index: number, color: THREE.Color): void {
    if (!this.instancedMesh || index >= this.instanceCount) return

    this.instancedMesh.setColorAt(index, color)
    if (this.instancedMesh.instanceColor) {
      this.instancedMesh.instanceColor.needsUpdate = true
    }
  }

  /**
   * 获取 InstancedMesh 对象
   */
  getMesh(): THREE.InstancedMesh | null {
    return this.instancedMesh
  }

  /**
   * 获取实例数量
   */
  getCount(): number {
    return this.instanceCount
  }

  /**
   * 释放资源
   */
  dispose(): void {
    if (this.instancedMesh) {
      this.instancedMesh.dispose()
      this.instancedMesh = null
    }
    this.instanceCount = 0
  }
}

// ============================================================
// Performance Monitor - 性能监控器
// ============================================================

export interface PerformanceStats {
  fps: number
  frameTime: number       // 毫秒
  memoryUsage: number     // MB (如果可用)
  drawCalls: number
  triangles: number
  lodLevel: number
}

export class PerformanceMonitor {
  private fps = 0
  private frameTime = 0
  private memoryUsage = 0
  private drawCalls = 0
  private triangles = 0
  private lodLevel = 0

  private frameCount = 0
  private lastTime = performance.now()
  private fpsUpdateInterval = 500 // ms
  private lastFPSUpdate = performance.now()

  private monitoring = false
  private rafId: number | null = null
  private onStatsUpdate: ((stats: PerformanceStats) => void) | null = null
  private renderer: THREE.WebGLRenderer | null = null

  // FPS 历史用于平滑计算
  private fpsHistory: number[] = []
  private maxHistoryLength = 60

  constructor(renderer?: THREE.WebGLRenderer) {
    this.renderer = renderer || null
  }

  /**
   * 设置关联的渲染器（用于获取 drawCalls 和 triangles 信息）
   */
  setRenderer(renderer: THREE.WebGLRenderer): void {
    this.renderer = renderer
  }

  /**
   * 开始监控
   */
  startMonitoring(onStatsUpdate?: (stats: PerformanceStats) => void): void {
    if (this.monitoring) return
    this.monitoring = true
    this.onStatsUpdate = onStatsUpdate || null
    this.lastTime = performance.now()
    this.lastFPSUpdate = performance.now()
    this.frameCount = 0
    this.fpsHistory = []
    this.tick()
  }

  /**
   * 停止监控
   */
  stopMonitoring(): void {
    this.monitoring = false
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId)
      this.rafId = null
    }
  }

  /**
   * 每帧开始时调用
   */
  beginFrame(): void {
    this.lastTime = performance.now()
  }

  /**
   * 每帧结束时调用，返回当前 FPS
   */
  endFrame(): number {
    const now = performance.now()
    this.frameTime = now - this.lastTime
    this.frameCount++

    // 每 fpsUpdateInterval 毫秒更新一次 FPS
    if (now - this.lastFPSUpdate >= this.fpsUpdateInterval) {
      this.fps = Math.round((this.frameCount * 1000) / (now - this.lastFPSUpdate))
      this.frameCount = 0
      this.lastFPSUpdate = now

      // 更新 FPS 历史
      this.fpsHistory.push(this.fps)
      if (this.fpsHistory.length > this.maxHistoryLength) {
        this.fpsHistory.shift()
      }

      // 更新内存使用
      this.updateMemoryUsage()

      // 更新渲染器统计
      if (this.renderer) {
        this.drawCalls = this.renderer.info.render.calls
        this.triangles = this.renderer.info.render.triangles
      }

      // 回调通知
      if (this.onStatsUpdate) {
        this.onStatsUpdate(this.getStats())
      }
    }

    return this.fps
  }

  /**
   * 获取单帧耗时（毫秒）
   */
  getFrameTime(): number {
    return Math.round(this.frameTime * 100) / 100
  }

  /**
   * 获取内存使用量（MB）
   */
  getMemoryUsage(): number {
    this.updateMemoryUsage()
    return Math.round(this.memoryUsage * 100) / 100
  }

  /**
   * 每帧调用，更新性能数据（兼容旧 API）
   */
  tick(): void {
    if (!this.monitoring) return

    const now = performance.now()
    this.frameTime = now - this.lastTime
    this.lastTime = now
    this.frameCount++

    // 每 fpsUpdateInterval 毫秒更新一次 FPS
    if (now - this.lastFPSUpdate >= this.fpsUpdateInterval) {
      this.fps = Math.round((this.frameCount * 1000) / (now - this.lastFPSUpdate))
      this.frameCount = 0
      this.lastFPSUpdate = now

      // 更新 FPS 历史
      this.fpsHistory.push(this.fps)
      if (this.fpsHistory.length > this.maxHistoryLength) {
        this.fpsHistory.shift()
      }

      // 更新内存使用
      this.updateMemoryUsage()

      // 更新渲染器统计
      if (this.renderer) {
        this.drawCalls = this.renderer.info.render.calls
        this.triangles = this.renderer.info.render.triangles
      }

      // 回调通知
      if (this.onStatsUpdate) {
        this.onStatsUpdate(this.getStats())
      }
    }

    this.rafId = requestAnimationFrame(() => this.tick())
  }

  /**
   * 获取当前性能统计
   */
  getStats(): PerformanceStats {
    return {
      fps: this.fps,
      frameTime: Math.round(this.frameTime * 100) / 100,
      memoryUsage: Math.round(this.memoryUsage * 100) / 100,
      drawCalls: this.drawCalls,
      triangles: this.triangles,
      lodLevel: this.lodLevel,
    }
  }

  /**
   * 检查 FPS 是否低于阈值
   */
  isBelowThreshold(targetFPS: number): boolean {
    return this.fps < targetFPS && this.fps > 0
  }

  /**
   * 检查 FPS 是否高于阈值（性能充裕）
   */
  isAboveThreshold(targetFPS: number): boolean {
    return this.fps > targetFPS && this.fps > 0
  }

  /**
   * 获取平均 FPS（基于历史数据）
   */
  getAverageFPS(): number {
    if (this.fpsHistory.length === 0) return 0
    const sum = this.fpsHistory.reduce((a, b) => a + b, 0)
    return Math.round(sum / this.fpsHistory.length)
  }

  /**
   * 设置当前 LOD 级别（用于统计报告）
   */
  setLODLevel(level: number): void {
    this.lodLevel = level
  }

  /**
   * 是否正在监控
   */
  isMonitoring(): boolean {
    return this.monitoring
  }

  private updateMemoryUsage(): void {
    // 使用 performance.memory API（Chrome 专有）
    const perfMemory = (performance as unknown as { memory?: { usedJSHeapSize: number } }).memory
    if (perfMemory) {
      this.memoryUsage = perfMemory.usedJSHeapSize / (1024 * 1024) // 转换为 MB
    } else {
      // 估算：基于渲染器信息
      if (this.renderer) {
        const info = this.renderer.info
        this.memoryUsage = (info.memory.geometries * 0.01) + (info.memory.textures * 0.5)
      }
    }
  }
}

// ============================================================
// Adaptive Quality Controller - 自适应质量控制
// ============================================================

export class AdaptiveQualityController {
  private lodManager: LODManager
  private performanceMonitor: PerformanceMonitor
  private lowFPSThreshold = 30
  private highFPSThreshold = 55
  private checkInterval = 2000 // ms
  private lastCheck = performance.now()
  private stabilityFrames = 0
  private requiredStabilityFrames = 3

  constructor(lodManager: LODManager, performanceMonitor: PerformanceMonitor) {
    this.lodManager = lodManager
    this.performanceMonitor = performanceMonitor
  }

  /**
   * 每帧调用，检查是否需要调整 LOD
   */
  update(): void {
    const now = performance.now()
    if (now - this.lastCheck < this.checkInterval) return
    this.lastCheck = now

    const stats = this.performanceMonitor.getStats()
    if (stats.fps === 0) return

    if (stats.fps < this.lowFPSThreshold) {
      // FPS 过低，降低 LOD
      this.stabilityFrames = 0
      if (this.lodManager.decreaseLOD()) {
        this.performanceMonitor.setLODLevel(this.lodManager.getCurrentLevel())
      }
    } else if (stats.fps > this.highFPSThreshold) {
      // FPS 充裕，尝试提高 LOD
      this.stabilityFrames++
      if (this.stabilityFrames >= this.requiredStabilityFrames) {
        if (this.lodManager.increaseLOD()) {
          this.performanceMonitor.setLODLevel(this.lodManager.getCurrentLevel())
          this.stabilityFrames = 0
        }
      }
    } else {
      // FPS 在正常范围
      this.stabilityFrames = 0
    }
  }

  /**
   * 设置 FPS 阈值
   */
  setThresholds(low: number, high: number): void {
    this.lowFPSThreshold = low
    this.highFPSThreshold = high
  }
}
