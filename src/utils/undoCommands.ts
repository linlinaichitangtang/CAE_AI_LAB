/**
 * Undo/Redo Command Implementations
 * 各模块操作的 UndoCommand 具体实现
 */

import type { UndoCommand } from '@/stores/undo'

// ============================================================
// 建模操作命令
// ============================================================

/**
 * 创建几何体命令
 */
export class CreateGeometryCommand implements UndoCommand {
  id: string
  description: string
  private geometryItem: any
  private scene: any
  private geometryItems: any[]

  constructor(options: {
    geometryItem: any
    scene: any
    geometryItems: any[]
  }) {
    this.id = `create-geometry-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    this.description = `创建几何体「${options.geometryItem.name}」`
    this.geometryItem = options.geometryItem
    this.scene = options.scene
    this.geometryItems = options.geometryItems
  }

  execute(): void {
    if (this.scene) {
      this.scene.add(this.geometryItem.group)
    }
    // Avoid duplicate push
    if (!this.geometryItems.includes(this.geometryItem)) {
      this.geometryItems.push(this.geometryItem)
    }
  }

  undo(): void {
    if (this.scene && this.geometryItem.group) {
      this.scene.remove(this.geometryItem.group)
    }
    const idx = this.geometryItems.indexOf(this.geometryItem)
    if (idx >= 0) {
      this.geometryItems.splice(idx, 1)
    }
  }
}

/**
 * 删除几何体命令
 */
export class DeleteGeometryCommand implements UndoCommand {
  id: string
  description: string
  private geometryItem: any
  private index: number
  private scene: any
  private geometryItems: any[]

  constructor(options: {
    geometryItem: any
    index: number
    scene: any
    geometryItems: any[]
  }) {
    this.id = `delete-geometry-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    this.description = `删除几何体「${options.geometryItem.name}」`
    this.geometryItem = options.geometryItem
    this.index = options.index
    this.scene = options.scene
    this.geometryItems = options.geometryItems
  }

  execute(): void {
    if (this.scene && this.geometryItem.group) {
      this.scene.remove(this.geometryItem.group)
    }
    const idx = this.geometryItems.indexOf(this.geometryItem)
    if (idx >= 0) {
      this.geometryItems.splice(idx, 1)
    }
  }

  undo(): void {
    if (this.scene) {
      this.scene.add(this.geometryItem.group)
    }
    // Restore at original index or end
    const insertIdx = Math.min(this.index, this.geometryItems.length)
    this.geometryItems.splice(insertIdx, 0, this.geometryItem)
  }
}

/**
 * 变换几何体命令（移动/旋转/缩放）
 */
export class TransformGeometryCommand implements UndoCommand {
  id: string
  description: string
  private geometryItem: any
  private oldTransform: { position: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number }; scale: { x: number; y: number; z: number } }
  private newTransform: { position: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number }; scale: { x: number; y: number; z: number } }

  constructor(options: {
    geometryItem: any
    oldTransform: { position: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number }; scale: { x: number; y: number; z: number } }
    newTransform: { position: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number }; scale: { x: number; y: number; z: number } }
  }) {
    this.id = `transform-geometry-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    this.description = `变换几何体「${options.geometryItem.name}」`
    this.geometryItem = options.geometryItem
    this.oldTransform = JSON.parse(JSON.stringify(options.oldTransform))
    this.newTransform = JSON.parse(JSON.stringify(options.newTransform))
  }

  execute(): void {
    if (this.geometryItem.group) {
      this.geometryItem.group.position.set(
        this.newTransform.position.x,
        this.newTransform.position.y,
        this.newTransform.position.z
      )
      this.geometryItem.group.rotation.set(
        this.newTransform.rotation.x,
        this.newTransform.rotation.y,
        this.newTransform.rotation.z
      )
      this.geometryItem.group.scale.set(
        this.newTransform.scale.x,
        this.newTransform.scale.y,
        this.newTransform.scale.z
      )
    }
  }

  undo(): void {
    if (this.geometryItem.group) {
      this.geometryItem.group.position.set(
        this.oldTransform.position.x,
        this.oldTransform.position.y,
        this.oldTransform.position.z
      )
      this.geometryItem.group.rotation.set(
        this.oldTransform.rotation.x,
        this.oldTransform.rotation.y,
        this.oldTransform.rotation.z
      )
      this.geometryItem.group.scale.set(
        this.oldTransform.scale.x,
        this.oldTransform.scale.y,
        this.oldTransform.scale.z
      )
    }
  }
}

// ============================================================
// 仿真操作命令
// ============================================================

/**
 * 设置材料命令
 */
export class SetMaterialCommand implements UndoCommand {
  id: string
  description: string
  private oldMaterial: any
  private newMaterial: any
  private materialRefs: { materialE: any; materialNu: any; materialDensity: any; materialType: any }

  constructor(options: {
    oldMaterial: any
    newMaterial: any
    materialRefs: { materialE: any; materialNu: any; materialDensity: any; materialType: any }
  }) {
    this.id = `set-material-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    this.description = `设置材料参数`
    this.oldMaterial = JSON.parse(JSON.stringify(options.oldMaterial))
    this.newMaterial = JSON.parse(JSON.stringify(options.newMaterial))
    this.materialRefs = options.materialRefs
  }

  execute(): void {
    this.materialRefs.materialE.value = this.newMaterial.elastic_modulus
    this.materialRefs.materialNu.value = this.newMaterial.poisson_ratio
    this.materialRefs.materialDensity.value = this.newMaterial.density
    if (this.newMaterial.material_type) {
      this.materialRefs.materialType.value = this.newMaterial.material_type
    }
  }

  undo(): void {
    this.materialRefs.materialE.value = this.oldMaterial.elastic_modulus
    this.materialRefs.materialNu.value = this.oldMaterial.poisson_ratio
    this.materialRefs.materialDensity.value = this.oldMaterial.density
    if (this.oldMaterial.material_type) {
      this.materialRefs.materialType.value = this.oldMaterial.material_type
    }
  }
}

/**
 * 添加边界条件命令
 */
export class AddBoundaryConditionCommand implements UndoCommand {
  id: string
  description: string
  private bc: any
  private bcType: 'fixed' | 'pointLoad' | 'uniformLoad'
  private projectStore: any
  private bcIndex: number = -1

  constructor(options: {
    bc: any
    bcType: 'fixed' | 'pointLoad' | 'uniformLoad'
    projectStore: any
  }) {
    this.id = `add-bc-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    const typeLabels: Record<string, string> = {
      fixed: '固定约束',
      pointLoad: '点荷载',
      uniformLoad: '均布荷载'
    }
    this.description = `添加${typeLabels[options.bcType]}`
    this.bc = JSON.parse(JSON.stringify(options.bc))
    this.bcType = options.bcType
    this.projectStore = options.projectStore
  }

  execute(): void {
    switch (this.bcType) {
      case 'fixed':
        this.bcIndex = this.projectStore.boundaryConditions.fixedBcs.length
        this.projectStore.addFixedBC(this.bc)
        break
      case 'pointLoad':
        this.bcIndex = this.projectStore.boundaryConditions.pointLoads.length
        this.projectStore.addPointLoad(this.bc)
        break
      case 'uniformLoad':
        this.bcIndex = this.projectStore.boundaryConditions.uniformLoads.length
        this.projectStore.addUniformLoad(this.bc)
        break
    }
  }

  undo(): void {
    switch (this.bcType) {
      case 'fixed':
        this.projectStore.removeFixedBC(this.bcIndex)
        break
      case 'pointLoad':
        this.projectStore.removePointLoad(this.bcIndex)
        break
      case 'uniformLoad':
        this.projectStore.removeUniformLoad(this.bcIndex)
        break
    }
  }
}

// ============================================================
// 笔记操作命令
// ============================================================

/**
 * 编辑笔记命令
 */
export class EditNoteCommand implements UndoCommand {
  id: string
  description: string
  private noteId: string
  private oldContent: string
  private newContent: string
  private contentRef: any
  private updateFileFn: (params: { id: string; file_name: string; content: string }) => Promise<any>

  constructor(options: {
    noteId: string
    oldContent: string
    newContent: string
    contentRef: any
    updateFileFn: (params: { id: string; file_name: string; content: string }) => Promise<any>
  }) {
    this.id = `edit-note-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    this.description = `编辑笔记`
    this.noteId = options.noteId
    this.oldContent = options.oldContent
    this.newContent = options.newContent
    this.contentRef = options.contentRef
    this.updateFileFn = options.updateFileFn
  }

  execute(): void {
    this.contentRef.value = this.newContent
  }

  undo(): void {
    this.contentRef.value = this.oldContent
  }
}

/**
 * 删除笔记命令
 */
export class DeleteNoteCommand implements UndoCommand {
  id: string
  description: string
  private note: any
  private noteId: string
  private noteTitle: string
  private noteContent: string
  private currentFileIdRef: any
  private noteTitleRef: any
  private noteContentRef: any
  private filesRef: any[]
  private deleteFileFn: (id: string) => Promise<any>
  private createFileFn: (params: any) => Promise<any>
  private loadFilesFn: () => Promise<any>
  private projectStore: any

  constructor(options: {
    note: any
    noteId: string
    noteTitle: string
    noteContent: string
    currentFileIdRef: any
    noteTitleRef: any
    noteContentRef: any
    filesRef: any[]
    deleteFileFn: (id: string) => Promise<any>
    createFileFn: (params: any) => Promise<any>
    loadFilesFn: () => Promise<any>
    projectStore: any
  }) {
    this.id = `delete-note-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`
    this.description = `删除笔记「${options.noteTitle}」`
    this.note = options.note
    this.noteId = options.noteId
    this.noteTitle = options.noteTitle
    this.noteContent = options.noteContent
    this.currentFileIdRef = options.currentFileIdRef
    this.noteTitleRef = options.noteTitleRef
    this.noteContentRef = options.noteContentRef
    this.filesRef = options.filesRef
    this.deleteFileFn = options.deleteFileFn
    this.createFileFn = options.createFileFn
    this.loadFilesFn = options.loadFilesFn
    this.projectStore = options.projectStore
  }

  async execute(): Promise<void> {
    await this.deleteFileFn(this.noteId)
    if (this.currentFileIdRef.value === this.noteId) {
      this.currentFileIdRef.value = null
      this.noteTitleRef.value = ''
      this.noteContentRef.value = ''
    }
    await this.loadFilesFn()
  }

  async undo(): Promise<void> {
    try {
      const restored = await this.createFileFn({
        project_id: this.projectStore.currentProjectId || this.note.project_id,
        file_type: 'note',
        file_name: this.noteTitle,
        content: this.noteContent,
        file_path: ''
      })
      this.currentFileIdRef.value = restored.id
      this.noteTitleRef.value = restored.file_name
      this.noteContentRef.value = restored.content || ''
      await this.loadFilesFn()
    } catch (e) {
      console.error('Failed to undo delete note:', e)
    }
  }
}
