import type { SimulationResult } from '../../types'

/**
 * Parse simulation results from backend data
 * The backend sends results in a structured format that needs to be
 * transformed into the internal SimulationResult format
 */

interface BackendResult {
  mesh: {
    nodes: Array<{ id: number; coordinates: [number, number, number] }>
    elements: Array<{ id: number; type: string; nodeIds: number[] }>
  }
  results: {
    displacement?: {
      step: number
      components: {
        ux?: number[]
        uy?: number[]
        uz?: number[]
      }
    }
    stress?: {
      step: number
      components: {
        sx?: number[]
        sy?: number[]
        sz?: number[]
        sxy?: number[]
        syz?: number[]
        sxz?: number[]
      }
    }
    vonMises?: number[]
  }
}

export function parseBackendResult(data: BackendResult): SimulationResult {
  const nodes = data.mesh.nodes.map(n => ({
    id: n.id,
    x: n.coordinates[0],
    y: n.coordinates[1],
    z: n.coordinates[2]
  }))

  const elements = data.mesh.elements.map(e => ({
    id: e.id,
    type: e.type,
    nodeIds: e.nodeIds
  }))

  const displacement = data.results.displacement ? {
    step: data.results.displacement.step,
    data: {
      ux: Object.fromEntries((data.results.displacement.components.ux || []).map((v, i) => [String(i + 1), v])),
      uy: Object.fromEntries((data.results.displacement.components.uy || []).map((v, i) => [String(i + 1), v])),
      uz: Object.fromEntries((data.results.displacement.components.uz || []).map((v, i) => [String(i + 1), v]))
    }
  } : undefined

  const stress = data.results.stress ? {
    step: data.results.stress.step,
    data: {
      sx: Object.fromEntries((data.results.stress.components.sx || []).map((v, i) => [String(i + 1), v])),
      sy: Object.fromEntries((data.results.stress.components.sy || []).map((v, i) => [String(i + 1), v])),
      sz: Object.fromEntries((data.results.stress.components.sz || []).map((v, i) => [String(i + 1), v])),
      sxy: Object.fromEntries((data.results.stress.components.sxy || []).map((v, i) => [String(i + 1), v])),
      syz: Object.fromEntries((data.results.stress.components.syz || []).map((v, i) => [String(i + 1), v])),
      sxz: Object.fromEntries((data.results.stress.components.sxz || []).map((v, i) => [String(i + 1), v]))
    }
  } : undefined

  const vonMises: Record<string, number> = {}
  if (data.results.vonMises) {
    data.results.vonMises.forEach((v, i) => {
      vonMises[String(i + 1)] = v
    })
  }

  return {
    nodes,
    elements,
    displacement,
    stress,
    vonMises: data.results.vonMises ? { step: 0, data: vonMises } : undefined
  }
}

/**
 * Generate sample result data for testing
 */
export function generateSampleResult(): SimulationResult {
  // Create a simple box mesh
  const nodes: SimulationResult['nodes'] = []
  const elements: SimulationResult['elements'] = []

  // Box dimensions
  const w = 2, h = 2, d = 2
  const nodeIds = [
    [-w/2, -h/2, -d/2], [w/2, -h/2, -d/2], [w/2, h/2, -d/2], [-w/2, h/2, -d/2],
    [-w/2, -h/2, d/2], [w/2, -h/2, d/2], [w/2, h/2, d/2], [-w/2, h/2, d/2]
  ]

  nodeIds.forEach((coords, i) => {
    nodes.push({
      id: i + 1,
      x: coords[0],
      y: coords[1],
      z: coords[2]
    })
  })

  // Create hex elements (divide box into smaller elements)
  // Simple approach: create 8 smaller cubes
  const subdivisions = 2
  const step = 1 / subdivisions

  let elemId = 1
  for (let ix = 0; ix < subdivisions; ix++) {
    for (let iy = 0; iy < subdivisions; iy++) {
      for (let iz = 0; iz < subdivisions; iz++) {
        const ox = -0.5 + ix * step
        const oy = -0.5 + iy * step
        const oz = -0.5 + iz * step

        // Find or create corner nodes
        const corners = [
          [ox, oy, oz], [ox + step, oy, oz], [ox + step, oy + step, oz], [ox, oy + step, oz],
          [ox, oy, oz + step], [ox + step, oy, oz + step], [ox + step, oy + step, oz + step], [ox, oy + step, oz + step]
        ]

        const cornerIds: number[] = []
        corners.forEach((c: number[]) => {
          const existingNode = nodes.find((n: { x: number; y: number; z: number }) => 
            Math.abs(n.x - c[0]) < 0.001 && 
            Math.abs(n.y - c[1]) < 0.001 && 
            Math.abs(n.z - c[2]) < 0.001
          )
          if (existingNode) {
            cornerIds.push(existingNode.id)
          } else {
            const newId = nodes.length + 1
            nodes.push({ id: newId, x: c[0], y: c[1], z: c[2] })
            cornerIds.push(newId)
          }
        })

        elements.push({
          id: elemId++,
          type: 'HEX8',
          nodeIds: cornerIds
        })
      }
    }
  }

  // Generate displacement data (simple bending)
  const displacement = {
    step: 1,
    data: {
      ux: {} as Record<string, number>,
      uy: {} as Record<string, number>,
      uz: {} as Record<string, number>
    }
  }

  nodes.forEach((n: { id: number; x: number; y: number; z: number }) => {
    const z = n.z
    const y = n.y
    // Simple cantilever bending
    displacement.data.ux[String(n.id)] = z * 0.5
    displacement.data.uy[String(n.id)] = z * y * 0.3
    displacement.data.uz[String(n.id)] = z * z * 0.2
  })

  // Generate Von Mises stress (higher at corners)
  const vonMises: Record<string, number> = {}
  nodes.forEach((n: { id: number; x: number; y: number; z: number }) => {
    const dist = Math.sqrt(n.x * n.x + n.y * n.y + n.z * n.z)
    vonMises[String(n.id)] = 50 + dist * 100 + Math.random() * 20
  })

  return {
    nodes,
    elements,
    displacement,
    vonMises: { step: 1, data: vonMises },
    deformationScale: 1.0
  }
}