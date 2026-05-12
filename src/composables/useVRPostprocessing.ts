/**
 * CAELab V3.5-004: VR Immersive Postprocessing
 * VR 环境查看应力/位移分布 (Meta Quest)
 */
import { ref, computed, watch } from 'vue'

export type VRController = 'left' | 'right' | 'both'
export type VRVisualizationMode = 'surface' | 'wireframe' | 'combined'
export type VRColorScheme = 'rainbow' | 'thermal' | 'grayscale' | 'scientific'

export interface VRFieldData {
  fieldName: string
  min: number
  max: number
  unit: string
  data: Float32Array
}

export interface VRAnnotation {
  id: string
  position: [number, number, number]
  text: string
  color: string
  timestamp: Date
}

export interface VRSnapshot {
  id: string
  timestamp: Date
  viewpoint: { position: [number, number, number], rotation: [number, number, number, number] }
  fieldData: string
  thumbnail?: string
}

const isVRSupported = ref(false)
const isSessionActive = ref(false)
const connectedControllers = ref<VRController[]>([])
const currentField = ref<string | null>(null)
const visualizationMode = ref<VRVisualizationMode>('surface')
const colorScheme = ref<VRColorScheme>('rainbow')
const fieldData = ref<Map<string, VRFieldData>>(new Map())
const annotations = ref<VRAnnotation[]>([])
const snapshots = ref<VRSnapshot[]>([])
const showAxes = ref(true)
const showColorBar = ref(true)
const showLabels = ref(true)
const lastError = ref<string | null>(null)

export function useVRPostprocessing() {
  const checkVRSupport = async () => {
    try {
      if ('xr' in navigator) {
        const supported = await (navigator as any).xr.isSessionSupported('immersive-vr')
        isVRSupported.value = supported
      } else {
        isVRSupported.value = false
      }
    } catch (e: any) {
      lastError.value = e.message || 'Failed to check VR support'
      isVRSupported.value = false
    }
  }

  const startVRSession = async (initialField?: string) => {
    try {
      lastError.value = null

      if (!isVRSupported.value) {
        throw new Error('VR not supported on this device')
      }

      isSessionActive.value = true
      connectedControllers.value = ['left', 'right', 'both']

      if (initialField) {
        currentField.value = initialField
      }

      return true
    } catch (e: any) {
      lastError.value = e.message
      throw e
    }
  }

  const stopVRSession = () => {
    isSessionActive.value = false
    connectedControllers.value = []
    currentField.value = null
  }

  const loadFieldData = (fieldName: string, data: VRFieldData) => {
    fieldData.value.set(fieldName, data)
  }

  const setCurrentField = (fieldName: string) => {
    if (fieldData.value.has(fieldName)) {
      currentField.value = fieldName
    } else {
      lastError.value = `Field '${fieldName}' not loaded`
    }
  }

  const getFieldValueAt = (position: [number, number, number], fieldName?: string): number | null => {
    const targetField = fieldName || currentField.value
    if (!targetField) return null

    const data = fieldData.value.get(targetField)
    if (!data || !data.data.length) return null

    // Simplified: return interpolated value
    const idx = Math.floor(
      (position[0] + 1) * 0.5 * data.data.length
    )
    return data.data[Math.min(Math.max(idx, 0), data.data.length - 1)] || 0
  }

  const setVisualizationMode = (mode: VRVisualizationMode) => {
    visualizationMode.value = mode
  }

  const setColorScheme = (scheme: VRColorScheme) => {
    colorScheme.value = scheme
  }

  const addAnnotation = (position: [number, number, number], text: string, color?: string) => {
    const annotation: VRAnnotation = {
      id: `ann_${Date.now()}`,
      position,
      text,
      color: color || '#FFFFFF',
      timestamp: new Date()
    }
    annotations.value.push(annotation)
    return annotation
  }

  const removeAnnotation = (id: string) => {
    annotations.value = annotations.value.filter(a => a.id !== id)
  }

  const clearAnnotations = () => {
    annotations.value = []
  }

  const takeSnapshot = (viewpoint: {
    position: [number, number, number]
    rotation: [number, number, number, number]
  }) => {
    const snapshot: VRSnapshot = {
      id: `snap_${Date.now()}`,
      timestamp: new Date(),
      viewpoint,
      fieldData: currentField.value || 'none'
    }
    snapshots.value.push(snapshot)
    return snapshot
  }

  const exportSnapshot = (id: string): VRSnapshot | null => {
    return snapshots.value.find(s => s.id === id) || null
  }

  const clearSnapshots = () => {
    snapshots.value = []
  }

  const toggleAxes = () => {
    showAxes.value = !showAxes.value
  }

  const toggleColorBar = () => {
    showColorBar.value = !showColorBar.value
  }

  const toggleLabels = () => {
    showLabels.value = !showLabels.value
  }

  const getFieldStats = computed(() => {
    if (!currentField.value) return null
    const data = fieldData.value.get(currentField.value)
    if (!data) return null

    return {
      name: data.fieldName,
      min: data.min,
      max: data.max,
      range: data.max - data.min,
      unit: data.unit
    }
  })

  const normalizedFieldData = computed(() => {
    if (!currentField.value) return null
    const data = fieldData.value.get(currentField.value)
    if (!data) return null

    const range = data.max - data.min || 1
    return {
      ...data,
      normalized: Array.from(data.data).map(v => (v - data.min) / range)
    }
  })

  return {
    // State
    isVRSupported: computed(() => isVRSupported.value),
    isSessionActive: computed(() => isSessionActive.value),
    connectedControllers: computed(() => connectedControllers.value),
    currentField: computed(() => currentField.value),
    visualizationMode: computed(() => visualizationMode.value),
    colorScheme: computed(() => colorScheme.value),
    fieldData: computed(() => fieldData.value),
    annotations: computed(() => annotations.value),
    snapshots: computed(() => snapshots.value),
    showAxes: computed(() => showAxes.value),
    showColorBar: computed(() => showColorBar.value),
    showLabels: computed(() => showLabels.value),
    lastError: computed(() => lastError.value),
    fieldStats: getFieldStats,
    normalizedFieldData,

    // Methods
    checkVRSupport,
    startVRSession,
    stopVRSession,
    loadFieldData,
    setCurrentField,
    getFieldValueAt,
    setVisualizationMode,
    setColorScheme,
    addAnnotation,
    removeAnnotation,
    clearAnnotations,
    takeSnapshot,
    exportSnapshot,
    clearSnapshots,
    toggleAxes,
    toggleColorBar,
    toggleLabels
  }
}