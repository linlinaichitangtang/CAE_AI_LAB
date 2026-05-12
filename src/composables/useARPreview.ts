/**
 * CAELab V3.5-003: 3D AR Preview
 * 手机/平板 AR 查看 3D 模型叠加
 */
import { ref, computed } from 'vue'

export type ARQuality = 'low' | 'medium' | 'high'
export type ARPlatform = 'ios' | 'android' | 'webxr'

export interface ARMarker {
  id: string
  name: string
  position: [number, number, number]
  size: number
  imageData?: string
}

export interface ARSession {
  id: string
  modelId: string
  startedAt: Date
  platform: ARPlatform
  quality: ARQuality
  markerCount: number
}

export interface ARMeasurement {
  id: string
  from: [number, number, number]
  to: [number, number, number]
  distance: number
  label: string
}

const isARSupported = ref(false)
const currentSession = ref<ARSession | null>(null)
const availableCameras = ref<MediaDeviceInfo[]>([])
const arQuality = ref<ARQuality>('medium')
const measurements = ref<ARMeasurement[]>([])
const showGrid = ref(true)
const showAnnotations = ref(true)
const brightness = ref(1.0)
const lastError = ref<string | null>(null)

export function useARPreview() {
  const checkARSupport = async () => {
    try {
      const hasCamera = 'mediaDevices' in navigator && 'getUserMedia' in navigator.mediaDevices
      const isMobile = /Android|iPhone|iPad|iPod/i.test(navigator.userAgent)

      // WebXR AR support check
      let webXRSupported = false
      if ('xr' in navigator) {
        try {
          const supported = await (navigator as any).xr.isSessionSupported('immersive-ar')
          webXRSupported = supported
        } catch {
          webXRSupported = false
        }
      }

      isARSupported.value = hasCamera && isMobile && webXRSupported

      if (!isMobile) {
        lastError.value = 'AR preview is optimized for mobile devices'
      } else if (!webXRSupported) {
        lastError.value = 'WebXR AR not supported on this device'
      }
    } catch (e: any) {
      lastError.value = e.message || 'Failed to check AR support'
      isARSupported.value = false
    }
  }

  const getAvailableCameras = async () => {
    try {
      const devices = await navigator.mediaDevices.enumerateDevices()
      availableCameras.value = devices.filter(d => d.kind === 'videoinput')
    } catch (e: any) {
      lastError.value = 'Failed to enumerate cameras'
    }
  }

  const startARSession = async (modelId: string, options?: {
    quality?: ARQuality
    useMarker?: boolean
    markerImage?: string
  }) => {
    try {
      lastError.value = null

      if (!isARSupported.value) {
        throw new Error('AR not supported on this device')
      }

      const sessionId = `ar_${Date.now()}`
      const platform = /iPhone|iPad|iPod/i.test(navigator.userAgent) ? 'ios' : 'android'

      currentSession.value = {
        id: sessionId,
        modelId,
        startedAt: new Date(),
        platform,
        quality: options?.quality || arQuality.value,
        markerCount: 0
      }

      // Initialize AR features based on quality setting
      if (options?.quality === 'high') {
        brightness.value = 1.2
      }

      return sessionId
    } catch (e: any) {
      lastError.value = e.message
      throw e
    }
  }

  const stopARSession = () => {
    if (currentSession.value) {
      currentSession.value = null
      measurements.value = []
    }
  }

  const placeModel = (position: [number, number, number], scale?: number) => {
    if (!currentSession.value) return

    // Simulate model placement in AR space
    return {
      id: `model_${Date.now()}`,
      position,
      scale: scale || 1.0,
      rotation: [0, 0, 0] as [number, number, number]
    }
  }

  const rotateModel = (modelId: string, delta: [number, number, number]) => {
    // Apply rotation to placed model
    return delta
  }

  const scaleModel = (modelId: string, factor: number) => {
    // Apply scale factor
    return factor
  }

  const addMeasurement = (from: [number, number, number], to: [number, number, number], label?: string) => {
    const distance = Math.sqrt(
      Math.pow(to[0] - from[0], 2) +
      Math.pow(to[1] - from[1], 2) +
      Math.pow(to[2] - from[2], 2)
    )

    const measurement: ARMeasurement = {
      id: `meas_${Date.now()}`,
      from,
      to,
      distance: Math.round(distance * 1000) / 1000,
      label: label || `Distance: ${distance.toFixed(3)}m`
    }

    measurements.value.push(measurement)
    return measurement
  }

  const removeMeasurement = (id: string) => {
    measurements.value = measurements.value.filter(m => m.id !== id)
  }

  const clearMeasurements = () => {
    measurements.value = []
  }

  const addMarker = async (marker: Omit<ARMarker, 'id'>) => {
    if (!currentSession.value) return null

    const newMarker: ARMarker = {
      id: `marker_${Date.now()}`,
      ...marker
    }

    if (currentSession.value) {
      currentSession.value.markerCount++
    }

    return newMarker
  }

  const removeMarker = (id: string) => {
    if (currentSession.value) {
      currentSession.value.markerCount--
    }
  }

  const setQuality = (quality: ARQuality) => {
    arQuality.value = quality
    if (currentSession.value) {
      currentSession.value.quality = quality
    }

    switch (quality) {
      case 'low':
        brightness.value = 0.8
        break
      case 'medium':
        brightness.value = 1.0
        break
      case 'high':
        brightness.value = 1.2
        break
    }
  }

  const toggleGrid = () => {
    showGrid.value = !showGrid.value
  }

  const toggleAnnotations = () => {
    showAnnotations.value = !showAnnotations.value
  }

  const captureScreenshot = (): string | null => {
    // Capture current AR view as base64 image
    const canvas = document.querySelector('canvas')
    if (!canvas) return null

    try {
      return canvas.toDataURL('image/png')
    } catch {
      return null
    }
  }

  const exportSession = (): ARSession | null => {
    return currentSession.value
  }

  const isSessionActive = computed(() => currentSession.value !== null)

  const sessionDuration = computed(() => {
    if (!currentSession.value) return 0
    return Date.now() - currentSession.value.startedAt.getTime()
  })

  return {
    // State
    isARSupported: computed(() => isARSupported.value),
    currentSession: computed(() => currentSession.value),
    availableCameras: computed(() => availableCameras.value),
    arQuality: computed(() => arQuality.value),
    measurements: computed(() => measurements.value),
    showGrid: computed(() => showGrid.value),
    showAnnotations: computed(() => showAnnotations.value),
    brightness: computed(() => brightness.value),
    lastError: computed(() => lastError.value),
    isSessionActive,
    sessionDuration,

    // Methods
    checkARSupport,
    getAvailableCameras,
    startARSession,
    stopARSession,
    placeModel,
    rotateModel,
    scaleModel,
    addMeasurement,
    removeMeasurement,
    clearMeasurements,
    addMarker,
    removeMarker,
    setQuality,
    toggleGrid,
    toggleAnnotations,
    captureScreenshot,
    exportSession
  }
}