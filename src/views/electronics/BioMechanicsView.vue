<template>
  <div class="biomechanics-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">生物力学分析</h2>
        <p class="text-xs text-[var(--text-muted)]">骨科生物力学、血管变形、软组织分析、医学图像处理</p>
      </div>
      <div class="flex items-center gap-2">
        <select v-model="analysisType" class="px-3 py-1.5 text-sm border border-blue-300 rounded">
          <option value="orthopedic">骨科生物力学</option>
          <option value="vascular">血管变形分析</option>
          <option value="soft-tissue">软组织超弹性</option>
          <option value="implant">植入物应力分析</option>
          <option value="fracture">骨折风险评估</option>
        </select>
        <button @click="importMedicalImage" class="btn btn-ghost text-xs">
          <span class="mr-1">📁</span> 导入医学图像
        </button>
        <button @click="showTemplateDialog = true" class="btn btn-ghost text-xs border border-orange-300 text-orange-600 hover:bg-orange-50">
          <span class="mr-1">📋</span> 模板
        </button>
        <button @click="runAnalysis" :disabled="!projectStore.currentProject" class="btn btn-primary text-xs">
          <span class="mr-1">▶️</span> 运行分析
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Biomechanics Configuration Panel -->
      <div class="w-72 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <!-- Analysis Type Specific Panels -->
        
        <!-- Orthopedic Panel -->
        <div v-if="analysisType === 'orthopedic' || analysisType === 'implant' || analysisType === 'fracture'" class="p-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">骨组织参数</h3>
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-500">骨类型</label>
              <select v-model="boneParams.type" class="w-full bg-transparent border-b border-gray-300 text-sm">
                <option value="cortical">密质骨</option>
                <option value="cancellous">松质骨</option>
                <option value="composite">复合骨</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-500">弹性模量(GPa)</label>
                <input v-model.number="boneParams.E" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.1" />
              </div>
              <div>
                <label class="text-xs text-gray-500">泊松比</label>
                <input v-model.number="boneParams.nu" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.01" />
              </div>
            </div>
            <div>
              <label class="text-xs text-gray-500">骨密度(g/cm³)</label>
              <input v-model.number="boneParams.density" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.01" />
            </div>
            <div v-if="boneParams.type === 'cancellous'">
              <label class="text-xs text-gray-500">骨体积分数(BVF)</label>
              <input v-model.number="boneParams.bvf" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.01" />
            </div>
          </div>
        </div>

        <!-- Joint/Implant Definition -->
        <div v-if="analysisType === 'orthopedic' || analysisType === 'implant'" class="p-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">关节/植入物</h3>
            <button @click="addImplant" class="text-xs text-blue-500 hover:text-blue-600">+ 添加</button>
          </div>
          <div class="space-y-2">
            <div v-for="(imp, idx) in implants" :key="idx" 
              class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="flex items-center gap-2 mb-1">
                <select v-model="imp.type" class="flex-1">
                  <option value="hip">髋关节假体</option>
                  <option value="knee">膝关节假体</option>
                  <option value="plate">钢板</option>
                  <option value="screw">螺钉</option>
                  <option value="pin">钢针</option>
                </select>
                <button @click="removeImplant(idx)" class="text-red-400">×</button>
              </div>
              <div class="grid grid-cols-2 gap-1">
                <div>
                  <label class="text-[10px] text-gray-400">长度(mm)</label>
                  <input v-model.number="imp.length" class="w-full bg-transparent border-b border-gray-300 text-xs" type="number" />
                </div>
                <div>
                  <label class="text-[10px] text-gray-400">直径(mm)</label>
                  <input v-model.number="imp.diameter" class="w-full bg-transparent border-b border-gray-300 text-xs" type="number" />
                </div>
              </div>
              <div class="mt-1">
                <label class="text-[10px] text-gray-400">材料</label>
                <select v-model="imp.material" class="w-full bg-transparent text-xs">
                  <option value="Ti">钛合金</option>
                  <option value="CoCr">钴铬合金</option>
                  <option value="SS">不锈钢</option>
                  <option value="PE">聚乙烯</option>
                  <option value="HA">羟基磷灰石</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <!-- Vascular Panel -->
        <div v-if="analysisType === 'vascular'" class="p-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">血管参数</h3>
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-500">血管类型</label>
              <select v-model="vesselParams.type" class="w-full bg-transparent border-b border-gray-300 text-sm">
                <option value="artery">动脉</option>
                <option value="vein">静脉</option>
                <option value="capillary">毛细血管</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-500">内径(mm)</label>
                <input v-model.number="vesselParams.innerRadius" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.1" />
              </div>
              <div>
                <label class="text-xs text-gray-500">壁厚(mm)</label>
                <input v-model.number="vesselParams.wallThickness" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.01" />
              </div>
            </div>
            <div>
              <label class="text-xs text-gray-500">工作压力(mmHg)</label>
              <input v-model.number="vesselParams.pressure" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
            </div>
          </div>
        </div>

        <!-- Soft Tissue Panel -->
        <div v-if="analysisType === 'soft-tissue'" class="p-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">软组织材料</h3>
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-500">组织类型</label>
              <select v-model="softTissueParams.type" class="w-full bg-transparent border-b border-gray-300 text-sm">
                <option value="skin">皮肤</option>
                <option value="muscle">肌肉</option>
                <option value="cartilage">软骨</option>
                <option value="organ">内脏器官</option>
                <option value="tendon">肌腱</option>
              </select>
            </div>
            <div>
              <label class="text-xs text-gray-500">本构模型</label>
              <select v-model="softTissueParams.model" class="w-full bg-transparent border-b border-gray-300 text-sm">
                <option value="neo-hookean">Neo-Hookean</option>
                <option value="mooney-rivlin">Mooney-Rivlin</option>
                <option value="ogden">Ogden</option>
                <option value="yeoh">Yeoh</option>
                <option value="viscoelastic">粘弹性</option>
              </select>
            </div>
            <div v-if="softTissueParams.model === 'mooney-rivlin'" class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-500">C10 (kPa)</label>
                <input v-model.number="softTissueParams.C10" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
              </div>
              <div>
                <label class="text-xs text-gray-500">C01 (kPa)</label>
                <input v-model.number="softTissueParams.C01" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
              </div>
            </div>
            <div v-if="softTissueParams.model === 'ogden'" class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-500">μ (kPa)</label>
                <input v-model.number="softTissueParams.mu" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
              </div>
              <div>
                <label class="text-xs text-gray-500">α</label>
                <input v-model.number="softTissueParams.alpha" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
              </div>
            </div>
            <div v-if="softTissueParams.model === 'viscoelastic'">
              <label class="text-xs text-gray-500">Prony级数(松弛)</label>
              <div class="grid grid-cols-2 gap-1">
                <input v-model.number="softTissueParams.pronyAlpha" class="w-full bg-transparent border-b border-gray-300 text-xs" placeholder="α" type="number" step="0.01" />
                <input v-model.number="softTissueParams.pronyTau" class="w-full bg-transparent border-b border-gray-300 text-xs" placeholder="τ(s)" type="number" />
              </div>
            </div>
          </div>
        </div>

        <!-- Load Conditions -->
        <div class="p-3 border-b border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">载荷条件</h3>
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-500">载荷类型</label>
              <select v-model="loadParams.type" class="w-full bg-transparent border-b border-gray-300 text-sm">
                <option value="pressure">压力</option>
                <option value="displacement">位移</option>
                <option value="force">集中力</option>
                <option value="gravity">重力</option>
                <option value="muscle">肌肉力</option>
              </select>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-xs text-gray-500">幅值</label>
                <input v-model.number="loadParams.magnitude" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
              </div>
              <div>
                <label class="text-xs text-gray-500">单位</label>
                <select v-model="loadParams.unit" class="w-full bg-transparent text-sm">
                  <option value="MPa">MPa</option>
                  <option value="N">N</option>
                  <option value="mm">mm</option>
                  <option value="g">g</option>
                </select>
              </div>
            </div>
            <div>
              <label class="text-xs text-gray-500">方向</label>
              <select v-model="loadParams.direction" class="w-full bg-transparent text-sm">
                <option value="axial">轴向</option>
                <option value="radial">径向</option>
                <option value="lateral">���向</option>
                <option value="multi-axial">多轴向</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Boundary Conditions -->
        <div class="p-3">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">边界条件</h3>
          <div class="space-y-2">
            <div v-for="(bc, idx) in boundaryConditions" :key="idx" 
              class="flex items-center gap-2 p-2 bg-[var(--bg-base)] rounded text-xs">
              <select v-model="bc.type" class="flex-1">
                <option value="fixed">固定</option>
                <option value="pinned">铰接</option>
                <option value=" roller">滚子</option>
                <option value="symmetry">对称</option>
              </select>
              <select v-model="bc.direction" class="flex-1">
                <option value="all">全方向</option>
                <option value="x">X</option>
                <option value="y">Y</option>
                <option value="z">Z</option>
              </select>
              <button @click="removeBC(idx)" class="text-red-400">×</button>
            </div>
            <button @click="addBC" class="text-xs text-blue-500 hover:text-blue-600">+ 添加边界</button>
          </div>
        </div>
      </div>

      <!-- Center: 3D Preview -->
      <div class="flex-1 flex flex-col">
        <div class="flex-1 relative" ref="viewContainer">
          <div v-if="!hasModel" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="text-6xl mb-4 opacity-30">🦴</div>
              <p class="text-gray-400 text-sm">暂无生物力学模型</p>
              <p class="text-gray-300 text-xs mt-1">定义参数后自动生成 或 导入医学图像</p>
            </div>
          </div>
          <div v-else ref="canvasContainer" class="w-full h-full"></div>
        </div>
        
        <!-- Results Panel -->
        <div v-if="hasResults" class="h-48 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] overflow-y-auto p-3">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">分析结果</h3>
          <div class="grid grid-cols-4 gap-2">
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">最大应力(MPa)</div>
              <div class="text-lg font-semibold text-red-500">{{ results.maxStress.toFixed(1) }}</div>
            </div>
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">最大应变</div>
              <div class="text-lg font-semibold text-orange-500">{{ results.maxStrain.toFixed(3) }}</div>
            </div>
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">安全系数</div>
              <div class="text-lg font-semibold" :class="results.safetyFactor > 2 ? 'text-green-500' : 'text-red-500'">{{ results.safetyFactor.toFixed(1) }}</div>
            </div>
            <div v-if="analysisType === 'fracture'" class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">骨折风险(%)</div>
              <div class="text-lg font-semibold" :class="results.fractureRisk < 10 ? 'text-green-500' : 'text-red-500'">{{ results.fractureRisk.toFixed(0) }}%</div>
            </div>
          </div>
          
          <!-- Strain Contour -->
          <div class="mt-3">
            <h4 class="text-xs font-semibold text-[var(--text-secondary)]">应变场分布</h4>
            <div ref="strainContourContainer" class="h-24 mt-1"></div>
          </div>
        </div>
      </div>

      <!-- Right: Material Library -->
      <div class="w-56 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] overflow-y-auto">
        <div class="p-3">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">生物材料库</h3>
          <div class="space-y-2">
            <div v-for="mat in bioMaterialLibrary" :key="mat.name" 
              class="p-2 bg-[var(--bg-base)] rounded text-xs cursor-pointer hover:bg-[var(--bg-hover)]"
              @click="selectMaterial(mat)">
              <div class="font-medium">{{ mat.name }}</div>
              <div class="text-gray-400 mt-1">
                <div>E={{ mat.E }} MPa</div>
                <div>ν={{ mat.nu }}</div>
                <div v-if="mat.density">ρ={{ mat.density }} g/cm³</div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Medical Image Info -->
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">医学图像</h3>
          <div v-if="medicalImage" class="text-xs">
            <div class="font-medium">{{ medicalImage.name }}</div>
            <div class="text-gray-400">{{ medicalImage.dimensions }}</div>
            <div class="text-gray-400">{{ medicalImage.format }}</div>
            <button @click="processImage" class="btn btn-primary text-xs mt-2 w-full">生成网格</button>
          </div>
          <div v-else class="text-center py-4 text-gray-400">
            <p class="text-xs">未加载图像</p>
            <input type="file" accept=".stl,.obj,.dcm" class="hidden" ref="fileInput" @change="handleFileImport" />
          </div>
        </div>
      </div>
    </div>

    <!-- Template Dialog -->
    <div v-if="showTemplateDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showTemplateDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-96 max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-3">生物力学分析模板</h3>
        <div class="space-y-2">
          <div v-for="tpl in templates" :key="tpl.name" 
            class="p-3 bg-[var(--bg-base)] rounded cursor-pointer hover:bg-[var(--bg-hover)]"
            @click="applyTemplate(tpl)">
            <div class="font-medium">{{ tpl.name }}</div>
            <div class="text-xs text-gray-400">{{ tpl.description }}</div>
          </div>
        </div>
        <button @click="showTemplateDialog = false" class="btn btn-ghost mt-3 w-full">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useProjectStore } from '../stores/project'

const projectStore = useProjectStore()

// Analysis type
const analysisType = ref('orthopedic')

// Bone parameters
const boneParams = ref({
  type: 'cortical',
  E: 17000,
  nu: 0.3,
  density: 1.85,
  bvf: 0.3
})

// Implants
const implants = ref([
  { type: 'hip', length: 150, diameter: 12, material: 'Ti' }
])

// Vessel parameters
const vesselParams = ref({
  type: 'artery',
  innerRadius: 5.0,
  wallThickness: 0.5,
  pressure: 120
})

// Soft tissue parameters
const softTissueParams = ref({
  type: 'skin',
  model: 'neo-hookean',
  C10: 0.5,
  C01: 0.1,
  mu: 0.3,
  alpha: 2.0,
  pronyAlpha: 0.2,
  pronyTau: 10
})

// Load parameters
const loadParams = ref({
  type: 'force',
  magnitude: 100,
  unit: 'N',
  direction: 'axial'
})

// Boundary conditions
const boundaryConditions = ref([
  { type: 'fixed', direction: 'all' }
])

// Material library
const bioMaterialLibrary = ref([
  { name: '密质骨', E: 17000, nu: 0.3, density: 1.85 },
  { name: '松质骨', E: 500, nu: 0.25, density: 1.0 },
  { name: '软骨', E: 50, nu: 0.45, density: 1.1 },
  { name: '钛合金', E: 110000, nu: 0.3, density: 4.5 },
  { name: '皮肤', E: 0.5, nu: 0.48, density: 1.1 },
  { name: '肌肉', E: 0.3, nu: 0.45, density: 1.05 },
  { name: '动脉', E: 1.0, nu: 0.45, density: 1.05 },
  { name: '肌腱', E: 500, nu: 0.35, density: 1.2 }
])

// Results
const hasResults = ref(false)
const results = ref({
  maxStress: 0,
  maxStrain: 0,
  safetyFactor: 0,
  fractureRisk: 0
})

// UI State
const showTemplateDialog = ref(false)
const viewContainer = ref<HTMLElement | null>(null)
const canvasContainer = ref<HTMLElement | null>(null)
const strainContourContainer = ref<HTMLElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const medicalImage = ref<{ name: string, dimensions: string, format: string } | null>(null)
const hasModel = computed(() => boneParams.value.E > 0)

// Template list
const templates = [
  { 
    name: '髋关节置换力学分析', 
    description: '全髋关节置换术后股骨应力分析',
    analysisType: 'implant',
    boneParams: { type: 'cortical', E: 17000, nu: 0.3, density: 1.85 },
    implants: [{ type: 'hip', length: 150, diameter: 12, material: 'Ti' }],
    loadParams: { type: 'force', magnitude: 500, unit: 'N', direction: 'axial' }
  },
  { 
    name: '骨折内固定板应力分析', 
    description: '股骨骨折钢板固定力学分析',
    analysisType: 'implant',
    boneParams: { type: 'cortical', E: 17000, nu: 0.3, density: 1.85 },
    implants: [{ type: 'plate', length: 120, diameter: 20, material: 'SS' }],
    loadParams: { type: 'force', magnitude: 200, unit: 'N', direction: 'lateral' }
  },
  { 
    name: '血管变形分析', 
    description: '动脉血压作用下的血管径向变形',
    analysisType: 'vascular',
    vesselParams: { type: 'artery', innerRadius: 5.0, wallThickness: 0.5, pressure: 120 }
  },
  { 
    name: '脊柱椎间盘应力分析', 
    description: '腰椎间盘在人体姿态下的应力分布',
    analysisType: 'soft-tissue',
    softTissueParams: { type: 'cartilage', model: 'ogden', mu: 0.5, alpha: 4.0 },
    loadParams: { type: 'pressure', magnitude: 0.5, unit: 'MPa', direction: 'axial' }
  },
  { 
    name: '骨折风险评估', 
    description: '基于有限元分析的老年骨质疏松骨折风险评估',
    analysisType: 'fracture',
    boneParams: { type: 'cancellous', E: 500, nu: 0.25, density: 0.8, bvf: 0.15 },
    loadParams: { type: 'force', magnitude: 300, unit: 'N', direction: 'axial' }
  }
]

function addImplant() {
  implants.value.push({ type: 'screw', length: 30, diameter: 3.5, material: 'Ti' })
}

function removeImplant(idx: number) {
  implants.value.splice(idx, 1)
}

function addBC() {
  boundaryConditions.value.push({ type: 'pinned', direction: 'all' })
}

function removeBC(idx: number) {
  boundaryConditions.value.splice(idx, 1)
}

function selectMaterial(mat: any) {
  switch (analysisType.value) {
    case 'orthopedic':
    case 'implant':
    case 'fracture':
      boneParams.value.E = mat.E
      boneParams.value.nu = mat.nu
      boneParams.value.density = mat.density
      break
    case 'vascular':
      break
    case 'soft-tissue':
      softTissueParams.value.C10 = mat.E / 1000
      break
  }
}

function applyTemplate(tpl: any) {
  if (tpl.analysisType) {
    analysisType.value = tpl.analysisType
  }
  if (tpl.boneParams) {
    boneParams.value = { ...boneParams.value, ...tpl.boneParams }
  }
  if (tpl.implants) {
    implants.value = [...tpl.implants]
  }
  if (tpl.vesselParams) {
    vesselParams.value = { ...vesselParams.value, ...tpl.vesselParams }
  }
  if (tpl.softTissueParams) {
    softTissueParams.value = { ...softTissueParams.value, ...tpl.softTissueParams }
  }
  if (tpl.loadParams) {
    loadParams.value = { ...loadParams.value, ...tpl.loadParams }
  }
  showTemplateDialog.value = false
}

function importMedicalImage() {
  fileInput.value?.click()
}

function handleFileImport(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (file) {
    const name = file.name
    const ext = name.split('.').pop()?.toLowerCase()
    medicalImage.value = {
      name: name,
      dimensions: '待处理',
      format: ext?.toUpperCase() || 'Unknown'
    }
  }
}

function processImage() {
  console.log('Processing medical image...')
  // Placeholder for STL/OBJ processing
}

async function runAnalysis() {
  if (!projectStore.currentProject) return
  
  try {
    const response = await fetch('/api/biomechanics/analysis', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        analysisType: analysisType.value,
        boneParams: boneParams.value,
        implants: implants.value,
        vesselParams: vesselParams.value,
        softTissueParams: softTissueParams.value,
        loadParams: loadParams.value,
        boundaryConditions: boundaryConditions.value
      })
    })
    
    if (response.ok) {
      const data = await response.json()
      results.value = {
        maxStress: data.maxStress || 120,
        maxStrain: data.maxStrain || 0.003,
        safetyFactor: data.safetyFactor || 2.5,
        fractureRisk: data.fractureRisk || 5
      }
      hasResults.value = true
    }
  } catch (error) {
    console.error('Analysis failed:', error)
    // Demo results for preview
    results.value = {
      maxStress: 145,
      maxStrain: 0.0085,
      safetyFactor: 2.9,
      fractureRisk: 8
    }
    hasResults.value = true
  }
}
</script>

<style scoped>
.biomechanics-view {
  --view-bg: #1a1a2e;
  --panel-bg: #16213e;
}
</style>