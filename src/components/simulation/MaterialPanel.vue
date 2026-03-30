<template>
  <div class="space-y-4">
    <!-- Material Type Selection -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">材料类型</h4>
      <div class="grid grid-cols-2 gap-2">
        <button
          v-for="type in materialTypes"
          :key="type.value"
          @click="selectMaterialType(type.value)"
          :class="[
            'px-3 py-2 rounded text-xs font-medium transition',
            currentType === type.value
              ? 'bg-blue-600 text-white'
              : 'bg-[var(--bg-secondary)] hover:bg-[var(--bg-tertiary)]'
          ]"
        >
          {{ type.label }}
        </button>
      </div>
    </div>

    <!-- Base Properties (Always Show) -->
    <div class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">基础属性</h4>
      
      <!-- Material Name -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">材料名称</label>
        <input 
          v-model="material.name" 
          type="text" 
          placeholder="Steel-Aluminum-..."
          class="input w-full text-xs" 
        />
      </div>

      <!-- Elastic Modulus -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">
          弹性模量 E (MPa)
        </label>
        <input 
          v-model.number="material.youngs_modulus" 
          type="number" 
          step="100"
          min="0"
          class="input w-full text-xs" 
        />
      </div>

      <!-- Poisson's Ratio -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">泊松比 ν</label>
        <input 
          v-model.number="material.poissons_ratio" 
          type="number" 
          step="0.01"
          min="0" 
          max="0.5"
          class="input w-full text-xs" 
        />
      </div>

      <!-- Density -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">密度 (ton/mm³)</label>
        <input 
          v-model.number="material.density" 
          type="number" 
          step="0.000000001"
          min="0"
          class="input w-full text-xs" 
        />
      </div>
    </div>

    <!-- Elastic (Linear) Properties -->
    <div v-if="currentType === 'elastic'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">线弹性</h4>
      <p class="text-[10px] text-[var(--text-muted)]">
        使用 *ELASTIC 卡片定义线性弹性材料
      </p>
    </div>

    <!-- Plastic (Elasto-Plastic) Properties -->
    <div v-if="currentType === 'plastic'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">弹塑性</h4>
      
      <!-- Yield Criterion -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">屈服准则</label>
        <select v-model="material.plastic.yield_criterion" class="input w-full text-xs">
          <option value="von_mises">Von Mises</option>
          <option value="tresca">Tresca</option>
          <option value="drucker_prager">Drucker-Prager</option>
        </select>
      </div>

      <!-- Hardening Type -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">强化类型</label>
        <select v-model="material.plastic.hardening_type" class="input w-full text-xs">
          <option value="isotropic">等向强化 (Isotropic)</option>
          <option value="kinematic">随动强化 (Kinematic)</option>
          <option value="combined">混合强化 (Combined)</option>
        </select>
      </div>

      <!-- Plastic Model -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">塑性模型</label>
        <select v-model="material.plastic.model" class="input w-full text-xs">
          <option value="bilinear">双线性 (Bilinear)</option>
          <option value="multilinear">多线性 (Multilinear)</option>
          <option value="exponential">指数强化</option>
        </select>
      </div>

      <!-- Yield Strength -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">屈服强度 σ_y (MPa)</label>
        <input 
          v-model.number="material.plastic.yield_strength" 
          type="number" 
          step="1"
          min="0"
          class="input w-full text-xs" 
        />
      </div>

      <!-- Tangent Modulus (for Bilinear) -->
      <div v-if="material.plastic.model === 'bilinear'" class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">切线模量 E_t (MPa)</label>
        <input 
          v-model.number="material.plastic.tangent_modulus" 
          type="number" 
          step="100"
          min="0"
          class="input w-full text-xs" 
        />
      </div>

      <!-- Multilinear Plastic Table -->
      <div v-if="material.plastic.model === 'multilinear'" class="mb-3">
        <div class="flex justify-between items-center mb-2">
          <label class="text-[10px] text-[var(--text-muted)] uppercase">应力-应变数据点</label>
          <button @click="addPlasticPoint" class="text-[10px] text-blue-500 hover:text-blue-600">
            + 添加点
          </button>
        </div>
        <div class="space-y-2 max-h-40 overflow-y-auto">
          <div v-for="(point, idx) in material.plastic.plastic_table" :key="idx" class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-[9px] text-[var(--text-muted)]">ε (应变)</label>
              <input 
                v-model.number="point.strain" 
                type="number" 
                step="0.001"
                class="input w-full text-xs" 
              />
            </div>
            <div>
              <label class="text-[9px] text-[var(--text-muted)]">σ (MPa)</label>
              <input 
                v-model.number="point.stress" 
                type="number" 
                step="1"
                class="input w-full text-xs" 
              />
            </div>
            <button 
              @click="removePlasticPoint(idx)" 
              class="col-span-2 text-[9px] text-red-500 hover:text-red-600"
            >
              删除
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Viscoelastic Properties -->
    <div v-if="currentType === 'viscoelastic'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">粘弹性</h4>
      
      <!-- Viscoelastic Model -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">粘弹性模型</label>
        <select v-model="material.viscoelastic.model" class="input w-full text-xs">
          <option value="maxwell">Maxwell 模型</option>
          <option value="kelvin">Kelvin/Voigt 模型</option>
          <option value="standard_linear">标准线性固体</option>
          <option value="generalized_maxwell">广义 Maxwell (Prony级数)</option>
        </select>
      </div>

      <!-- Prony Series (for Generalized Maxwell) -->
      <div v-if="material.viscoelastic.model === 'generalized_maxwell'" class="mb-3">
        <div class="flex justify-between items-center mb-2">
          <label class="text-[10px] text-[var(--text-muted)] uppercase">Prony 级数参数</label>
          <button @click="addPronyTerm" class="text-[10px] text-blue-500 hover:text-blue-600">
            + 添加项
          </button>
        </div>
        <div class="space-y-2 max-h-40 overflow-y-auto">
          <div v-for="(term, idx) in material.viscoelastic.prony_series" :key="idx" class="bg-[var(--bg-secondary)] p-2 rounded">
            <div class="text-[10px] text-[var(--text-muted)] mb-2">项 {{ idx + 1 }}</div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[9px] text-[var(--text-muted)]">E_i / E_0 (相对模量)</label>
                <input 
                  v-model.number="term.relative_modulus" 
                  type="number" 
                  step="0.01"
                  min="0"
                  max="1"
                  class="input w-full text-xs" 
                />
              </div>
              <div>
                <label class="text-[9px] text-[var(--text-muted)]">τ_i (松弛时间, s)</label>
                <input 
                  v-model.number="term.relaxation_time" 
                  type="number" 
                  step="0.1"
                  min="0"
                  class="input w-full text-xs" 
                />
              </div>
            </div>
            <button 
              @click="removePronyTerm(idx)" 
              class="mt-1 text-[9px] text-red-500 hover:text-red-600"
            >
              删除此项
            </button>
          </div>
        </div>
      </div>

      <!-- Maxwell/Kelvin Parameters -->
      <div v-if="material.viscoelastic.model === 'maxwell' || material.viscoelastic.model === 'kelvin'" class="mb-3">
        <div class="grid grid-cols-2 gap-2">
          <div>
            <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">E (MPa)</label>
            <input 
              v-model.number="material.viscoelastic.elastic_modulus" 
              type="number" 
              step="100"
              min="0"
              class="input w-full text-xs" 
            />
          </div>
          <div>
            <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">η (粘性系数)</label>
            <input 
              v-model.number="material.viscoelastic.viscosity" 
              type="number" 
              step="1"
              min="0"
              class="input w-full text-xs" 
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Hyperelastic Properties -->
    <div v-if="currentType === 'hyperelastic'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">超弹性</h4>
      
      <!-- Hyperelastic Model -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">超弹性模型</label>
        <select v-model="material.hyperelastic.model" class="input w-full text-xs">
          <option value="neo_hookean">Neo-Hookean</option>
          <option value="mooney_rivlin">Mooney-Rivlin (2-参数)</option>
          <option value="mooney_rivlin_3">Mooney-Rivlin (3-参数)</option>
          <option value="ogden">Ogden 模型</option>
          <option value="yeoh">Yeoh 模型</option>
        </select>
      </div>

      <!-- Neo-Hookean Parameters -->
      <div v-if="material.hyperelastic.model === 'neo_hookean'" class="mb-3">
        <div>
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">C10 (MPa)</label>
          <input 
            v-model.number="material.hyperelastic.c10" 
            type="number" 
            step="0.1"
            min="0"
            class="input w-full text-xs" 
          />
          <p class="text-[9px] text-[var(--text-muted)] mt-1">通常 C10 = E/(4(1+ν))</p>
        </div>
      </div>

      <!-- Mooney-Rivlin Parameters -->
      <div v-if="material.hyperelastic.model === 'mooney_rivlin' || material.hyperelastic.model === 'mooney_rivlin_3'" class="mb-3">
        <div class="grid grid-cols-2 gap-2">
          <div>
            <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">C10 (MPa)</label>
            <input 
              v-model.number="material.hyperelastic.c10" 
              type="number" 
              step="0.1"
              min="0"
              class="input w-full text-xs" 
            />
          </div>
          <div>
            <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">C01 (MPa)</label>
            <input 
              v-model.number="material.hyperelastic.c01" 
              type="number" 
              step="0.1"
              class="input w-full text-xs" 
            />
          </div>
        </div>
        <div v-if="material.hyperelastic.model === 'mooney_rivlin_3'" class="mt-2">
          <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">C20 (MPa)</label>
          <input 
            v-model.number="material.hyperelastic.c20" 
            type="number" 
            step="0.1"
            class="input w-full text-xs" 
          />
        </div>
      </div>

      <!-- Ogden Parameters -->
      <div v-if="material.hyperelastic.model === 'ogden'" class="mb-3">
        <div class="flex justify-between items-center mb-2">
          <label class="text-[10px] text-[var(--text-muted)] uppercase">Ogden 项</label>
          <button @click="addOgdenTerm" class="text-[10px] text-blue-500 hover:text-blue-600">
            + 添加项
          </button>
        </div>
        <div class="space-y-2 max-h-40 overflow-y-auto">
          <div v-for="(term, idx) in material.hyperelastic.ogden_terms" :key="idx" class="bg-[var(--bg-secondary)] p-2 rounded">
            <div class="text-[10px] text-[var(--text-muted)] mb-2">项 {{ idx + 1 }}</div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[9px] text-[var(--text-muted)]">μ_i (MPa)</label>
                <input 
                  v-model.number="term.mu" 
                  type="number" 
                  step="0.1"
                  class="input w-full text-xs" 
                />
              </div>
              <div>
                <label class="text-[9px] text-[var(--text-muted)]">α_i</label>
                <input 
                  v-model.number="term.alpha" 
                  type="number" 
                  step="0.1"
                  class="input w-full text-xs" 
                />
              </div>
            </div>
            <button 
              @click="removeOgdenTerm(idx)" 
              class="mt-1 text-[9px] text-red-500 hover:text-red-600"
            >
              删除此项
            </button>
          </div>
        </div>
      </div>

      <!-- D Value (for incompressibility) -->
      <div class="mb-3">
        <label class="text-[10px] text-[var(--text-muted)] uppercase mb-1 block">D (压缩性参数)</label>
        <input 
          v-model.number="material.hyperelastic.d" 
          type="number" 
          step="0.0001"
          min="0"
          class="input w-full text-xs" 
        />
        <p class="text-[9px] text-[var(--text-muted)] mt-1">D = 2 / K, K 为体积模量</p>
      </div>
    </div>

    <!-- Stress-Strain Curve Preview -->
    <div v-if="currentType !== 'elastic'" class="panel-section">
      <h4 class="text-xs font-medium text-[var(--text-secondary)] mb-3">材料曲线预览</h4>
      <div class="bg-[var(--bg-secondary)] rounded p-2">
        <canvas ref="curveCanvas" width="280" height="160" class="w-full"></canvas>
      </div>
    </div>

    <!-- Actions -->
    <div class="panel-section flex gap-2">
      <button 
        @click="saveMaterial" 
        :disabled="!canSave"
        class="btn btn-primary flex-1 text-xs"
      >
        保存材料
      </button>
      <button @click="resetMaterial" class="btn btn-ghost flex-1 text-xs">
        重置
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted, nextTick } from 'vue'

// Material types
const materialTypes = [
  { label: '线弹性', value: 'elastic' },
  { label: '弹塑性', value: 'plastic' },
  { label: '粘弹性', value: 'viscoelastic' },
  { label: '超弹性', value: 'hyperelastic' }
]

const currentType = ref('elastic')

// Material data
const material = reactive({
  name: 'Steel',
  youngs_modulus: 200000,
  poissons_ratio: 0.3,
  density: 7.85e-9,
  type: 'elastic',
  
  // Plastic parameters
  plastic: {
    model: 'bilinear',
    yield_criterion: 'von_mises',
    hardening_type: 'isotropic',
    yield_strength: 250,
    tangent_modulus: 20000,
    plastic_table: [
      { strain: 0.00125, stress: 250 },
      { strain: 0.01, stress: 450 },
      { strain: 0.02, stress: 600 },
      { strain: 0.05, stress: 800 }
    ]
  },
  
  // Viscoelastic parameters
  viscoelastic: {
    model: 'generalized_maxwell',
    elastic_modulus: 200000,
    viscosity: 1000,
    prony_series: [
      { relative_modulus: 0.2, relaxation_time: 1.0 },
      { relative_modulus: 0.3, relaxation_time: 10.0 },
      { relative_modulus: 0.1, relaxation_time: 100.0 }
    ]
  },
  
  // Hyperelastic parameters
  hyperelastic: {
    model: 'neo_hookean',
    c10: 0.5,
    c01: 0.0,
    c20: 0.0,
    d: 0.001,
    ogden_terms: [
      { mu: 1.0, alpha: 2.0 }
    ]
  }
})

const curveCanvas = ref<HTMLCanvasElement | null>(null)

// Select material type
function selectMaterialType(type: string) {
  currentType.value = type
  material.type = type
  nextTick(() => drawCurve())
}

// Add plastic data point
function addPlasticPoint() {
  const last = material.plastic.plastic_table[material.plastic.plastic_table.length - 1]
  material.plastic.plastic_table.push({
    strain: (last?.strain || 0) + 0.01,
    stress: (last?.stress || 0) + 50
  })
}

// Remove plastic data point
function removePlasticPoint(idx: number) {
  if (material.plastic.plastic_table.length > 2) {
    material.plastic.plastic_table.splice(idx, 1)
  }
}

// Add Prony term
function addPronyTerm() {
  material.viscoelastic.prony_series.push({
    relative_modulus: 0.1,
    relaxation_time: 10
  })
}

// Remove Prony term
function removePronyTerm(idx: number) {
  if (material.viscoelastic.prony_series.length > 1) {
    material.viscoelastic.prony_series.splice(idx, 1)
  }
}

// Add Ogden term
function addOgdenTerm() {
  material.hyperelastic.ogden_terms.push({
    mu: 1.0,
    alpha: 2.0
  })
}

// Remove Ogden term
function removeOgdenTerm(idx: number) {
  if (material.hyperelastic.ogden_terms.length > 1) {
    material.hyperelastic.ogden_terms.splice(idx, 1)
  }
}

// Draw stress-strain curve
function drawCurve() {
  const canvas = curveCanvas.value
  if (!canvas) return
  
  const ctx = canvas.getContext('2d')
  if (!ctx) return
  
  const w = canvas.width
  const h = canvas.height
  const padding = 30
  
  // Clear canvas
  ctx.fillStyle = '#1e1e1e'
  ctx.fillRect(0, 0, w, h)
  
  // Draw axes
  ctx.strokeStyle = '#666'
  ctx.lineWidth = 1
  
  // Y axis
  ctx.beginPath()
  ctx.moveTo(padding, padding)
  ctx.lineTo(padding, h - padding)
  ctx.stroke()
  
  // X axis
  ctx.beginPath()
  ctx.moveTo(padding, h - padding)
  ctx.lineTo(w - padding, h - padding)
  ctx.stroke()
  
  // Labels
  ctx.fillStyle = '#999'
  ctx.font = '10px sans-serif'
  ctx.fillText('σ', 10, padding + 10)
  ctx.fillText('ε', w - 20, h - 10)
  
  let points: {x: number, y: number}[] = []
  
  if (currentType.value === 'plastic') {
    // Draw plastic curve
    const E = material.youngs_modulus
    const sigmaY = material.plastic.yield_strength
    
    // Generate points
    for (let eps = 0; eps <= 0.1; eps += 0.001) {
      let sigma: number
      if (eps <= sigmaY / E) {
        // Elastic region
        sigma = E * eps
      } else {
        // Plastic region
        if (material.plastic.model === 'bilinear') {
          const Et = material.plastic.tangent_modulus
          sigma = sigmaY + Et * (eps - sigmaY / E)
        } else if (material.plastic.model === 'multilinear') {
          // Interpolate from table
          const table = material.plastic.plastic_table
          sigma = interpolatePlasticTable(eps, table, sigmaY, E)
        } else {
          // Exponential
          sigma = sigmaY + (E - sigmaY) * (1 - Math.exp(-5 * (eps - sigmaY / E)))
        }
      }
      points.push({ x: eps, y: sigma })
    }
  } else if (currentType.value === 'viscoelastic') {
    // Draw relaxation curve (stress vs time at constant strain)
    const E0 = material.viscoelastic.elastic_modulus
    for (let t = 0; t <= 100; t += 1) {
      let E = E0
      for (const term of material.viscoelastic.prony_series) {
        E -= term.relative_modulus * E0 * Math.exp(-t / term.relaxation_time)
      }
      // Normalize for display
      const normalizedE = (E / E0) * 0.08 // Scale for visibility
      points.push({ x: t / 100, y: normalizedE })
    }
  } else if (currentType.value === 'hyperelastic') {
    // Draw hyperelastic stress-stretch curve
    const c10 = material.hyperelastic.c10
    const lambdaMax = 2.5
    
    for (let lam = 1; lam <= lambdaMax; lam += 0.05) {
      let sigma = 0
      const lamSq = lam * lam
      const lamInv = 1 / lam
      
      if (material.hyperelastic.model === 'neo_hookean') {
        sigma = 2 * c10 * (lam - lamInv * lamInv)
      } else if (material.hyperelastic.model === 'mooney_rivlin') {
        const c01 = material.hyperelastic.c01
        sigma = 2 * (c10 + c01 / lam) * (lam - 1 / lam / lam)
      }
      
      points.push({ x: lam - 1, y: sigma * 0.0003 }) // Scale for visibility
    }
  }
  
  if (points.length === 0) return
  
  // Find bounds
  const maxX = Math.max(...points.map(p => p.x))
  const maxY = Math.max(...points.map(p => p.y))
  
  // Draw curve
  ctx.strokeStyle = '#4F8CFF'
  ctx.lineWidth = 2
  ctx.beginPath()
  
  for (let i = 0; i < points.length; i++) {
    const x = padding + (points[i].x / maxX) * (w - 2 * padding)
    const y = (h - padding) - (points[i].y / maxY) * (h - 2 * padding)
    
    if (i === 0) {
      ctx.moveTo(x, y)
    } else {
      ctx.lineTo(x, y)
    }
  }
  
  ctx.stroke()
}

// Interpolate plastic table
function interpolatePlasticTable(strain: number, table: {strain: number, stress: number}[], sigmaY: number, E: number): number {
  // Linear interpolation
  for (let i = 0; i < table.length - 1; i++) {
    if (strain <= table[i + 1].strain) {
      const t = (strain - table[i].strain) / (table[i + 1].strain - table[i].strain)
      return table[i].stress + t * (table[i + 1].stress - table[i].stress)
    }
  }
  return table[table.length - 1].stress
}

// Can save
const canSave = computed(() => {
  return material.name.trim().length > 0 && material.youngs_modulus > 0
})

// Save material
function saveMaterial() {
  // Emit event to parent
  emit('save', {
    ...material,
    type: currentType.value
  })
  
  console.log('Material saved:', material)
}

// Reset material
function resetMaterial() {
  material.name = 'Steel'
  material.youngs_modulus = 200000
  material.poissons_ratio = 0.3
  material.density = 7.85e-9
  currentType.value = 'elastic'
  material.type = 'elastic'
}

// Draw curve when type changes or data changes
watch([currentType, () => material], () => {
  nextTick(() => drawCurve())
}, { deep: true })

// Initial draw
onMounted(() => {
  nextTick(() => drawCurve())
})

const emit = defineEmits<{
  save: [material: any]
}>()
</script>

<style scoped>
.panel-section {
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.panel-section:last-child {
  border-bottom: none;
  padding-bottom: 0;
}
</style>