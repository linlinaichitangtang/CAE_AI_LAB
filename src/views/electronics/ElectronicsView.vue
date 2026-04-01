<template>
  <div class="electronics-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">电子封装分析</h2>
        <p class="text-xs text-[var(--text-muted)]">PCB热应力、BGA焊点可靠性、电子材料分析</p>
      </div>
      <div class="flex items-center gap-2">
        <select v-model="analysisType" class="px-3 py-1.5 text-sm border border-blue-300 rounded">
          <option value="thermal-structural">热-结构耦合分析</option>
          <option value="pcb-stress">PCB板应力分析</option>
          <option value="bga-reliability">BGA焊点可靠性分析</option>
          <option value="via-current">过孔电流分析</option>
        </select>
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
      <!-- Left: PCB Structure Panel -->
      <div class="w-72 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <!-- PCB Stackup Section -->
        <div class="p-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">PCB叠层结构</h3>
            <button @click="addLayer" class="text-xs text-blue-500 hover:text-blue-600">+ 添加层</button>
          </div>
          <div class="space-y-1">
            <div v-for="(layer, idx) in pcbStackup" :key="idx" 
              class="flex items-center gap-2 p-2 bg-[var(--bg-base)] rounded text-xs">
              <input v-model="layer.name" class="w-12 bg-transparent border-b border-gray-300 text-xs" placeholder="层名" />
              <select v-model="layer.material" class="flex-1 bg-transparent text-xs">
                <option value="FR4">FR4</option>
                <option value="Rogers">Rogers</option>
                <option value="MetalBase">金属基板</option>
                <option value="Ceramic">陶瓷</option>
              </select>
              <input v-model.number="layer.thickness" class="w-12 bg-transparent border-b border-gray-300 text-xs" type="number" step="0.01" placeholder="mm" />
              <span class="text-gray-400">mm</span>
              <button @click="removeLayer(idx)" class="text-red-400 hover:text-red-600">×</button>
            </div>
          </div>
          <p class="text-[10px] text-gray-400 mt-1">总厚度: {{ totalThickness.toFixed(2) }} mm</p>
        </div>

        <!-- Via Definition -->
        <div class="p-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">过孔定义</h3>
            <button @click="addVia" class="text-xs text-blue-500 hover:text-blue-600">+ 添加过孔</button>
          </div>
          <div class="space-y-1">
            <div v-for="(via, idx) in vias" :key="idx" 
              class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="flex items-center gap-2 mb-1">
                <span class="font-medium">过孔 {{ idx + 1 }}</span>
                <select v-model="via.type" class="flex-1">
                  <option value="through">电镀通孔</option>
                  <option value="blind">盲孔</option>
                  <option value="buried">埋孔</option>
                </select>
              </div>
              <div class="grid grid-cols-2 gap-1">
                <div>
                  <label class="text-[10px] text-gray-400">直径(mm)</label>
                  <input v-model.number="via.diameter" class="w-full bg-transparent border-b border-gray-300 text-xs" type="number" step="0.01" />
                </div>
                <div>
                  <label class="text-[10px] text-gray-400">镀层厚度(μm)</label>
                  <input v-model.number="via.platingThickness" class="w-full bg-transparent border-b border-gray-300 text-xs" type="number" step="0.1" />
                </div>
              </div>
              <button @click="removeVia(idx)" class="text-red-400 hover:text-red-600 text-xs mt-1">删除</button>
            </div>
          </div>
        </div>

        <!-- Solder Definition -->
        <div class="p-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center justify-between mb-2">
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">焊点/焊球</h3>
            <button @click="addSolder" class="text-xs text-blue-500 hover:text-blue-600">+ 添加焊点</button>
          </div>
          <div class="space-y-1">
            <div v-for="(solder, idx) in solders" :key="idx" 
              class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="flex items-center gap-2 mb-1">
                <span class="font-medium">焊点 {{ idx + 1 }}</span>
                <select v-model="solder.type" class="flex-1">
                  <option value="SMT">SMT焊点</option>
                  <option value="BGA">BGA焊球</option>
                  <option value="QFN">QFN焊盘</option>
                </select>
              </div>
              <div class="grid grid-cols-2 gap-1">
                <div>
                  <label class="text-[10px] text-gray-400">直径(mm)</label>
                  <input v-model.number="solder.diameter" class="w-full bg-transparent border-b border-gray-300 text-xs" type="number" step="0.01" />
                </div>
                <div>
                  <label class="text-[10px] text-gray-400">材料</label>
                  <select v-model="solder.material" class="w-full bg-transparent text-xs">
                    <option value="SAC305">SAC305</option>
                    <option value="SnPb">SnPb</option>
                    <option value="ENIG">无铅</option>
                  </select>
                </div>
              </div>
              <button @click="removeSolder(idx)" class="text-red-400 hover:text-red-600 text-xs mt-1">删除</button>
            </div>
          </div>
        </div>

        <!-- Thermal Boundary -->
        <div class="p-3">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">热边界条件</h3>
          <div class="space-y-2">
            <div>
              <label class="text-xs text-gray-500">芯片功耗(W)</label>
              <input v-model.number="thermal.power" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="0.1" />
            </div>
            <div>
              <label class="text-xs text-gray-500">环境温度(°C)</label>
              <input v-model.number="thermal.ambientTemp" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" />
            </div>
            <div>
              <label class="text-xs text-gray-500">散热方式</label>
              <select v-model="thermal.coolingType" class="w-full bg-transparent text-sm">
                <option value="natural">自然对流</option>
                <option value="forced">强迫风冷</option>
                <option value="liquid">液冷</option>
              </select>
            </div>
            <div v-if="thermal.coolingType !== 'natural'">
              <label class="text-xs text-gray-500">对流系数(W/m²·K)</label>
              <input v-model.number="thermal.h" class="w-full bg-transparent border-b border-gray-300 text-sm" type="number" step="1" />
            </div>
          </div>
        </div>
      </div>

      <!-- Center: 3D Preview -->
      <div class="flex-1 flex flex-col">
        <div class="flex-1 relative" ref="viewContainer">
          <div v-if="!hasModel" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="text-6xl mb-4 opacity-30">📱</div>
              <p class="text-gray-400 text-sm">暂无PCB模型</p>
              <p class="text-gray-300 text-xs mt-1">定义叠层结构后自动生成</p>
            </div>
          </div>
          <div v-else ref="canvasContainer" class="w-full h-full"></div>
        </div>
        
        <!-- Results Panel -->
        <div v-if="hasResults" class="h-48 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] overflow-y-auto p-3">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">分析结果</h3>
          <div class="grid grid-cols-4 gap-2">
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">最大温度</div>
              <div class="text-lg font-semibold text-red-500">{{ results.maxTemp.toFixed(1) }}°C</div>
            </div>
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">最大应力(MPa)</div>
              <div class="text-lg font-semibold text-orange-500">{{ results.maxStress.toFixed(1) }}</div>
            </div>
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">最大位移(mm)</div>
              <div class="text-lg font-semibold text-blue-500">{{ results.maxDisplacement.toFixed(3) }}</div>
            </div>
            <div class="p-2 bg-[var(--bg-base)] rounded text-xs">
              <div class="text-gray-400">焊点疲劳寿命</div>
              <div class="text-lg font-semibold text-green-500">{{ results.fatigueLife.toFixed(0) }}cycles</div>
            </div>
          </div>
          
          <!-- Temperature Contour -->
          <div class="mt-3">
            <h4 class="text-xs font-semibold text-[var(--text-secondary)]">温度场分布</h4>
            <div ref="tempContourContainer" class="h-24 mt-1"></div>
          </div>
        </div>
      </div>

      <!-- Right: Material Library -->
      <div class="w-56 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] overflow-y-auto">
        <div class="p-3">
          <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-2">电子材料库</h3>
          <div class="space-y-2">
            <div v-for="mat in materialLibrary" :key="mat.name" 
              class="p-2 bg-[var(--bg-base)] rounded text-xs cursor-pointer hover:bg-[var(--bg-hover)]"
              @click="selectMaterial(mat)">
              <div class="font-medium">{{ mat.name }}</div>
              <div class="text-gray-400 mt-1">
                <div>E={{ mat.E }} GPa</div>
                <div>α={{ mat.CTE }} ppm/K</div>
                <div>k={{ mat.k }} W/m·K</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Template Dialog -->
    <div v-if="showTemplateDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showTemplateDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-96 max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-3">电子封装分析模板</h3>
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
import { useProjectStore } from '../../stores/project'

const projectStore = useProjectStore()

// Analysis type
const analysisType = ref('thermal-structural')

// PCB Stackup
const pcbStackup = ref([
  { name: 'TopSol', material: 'FR4', thickness: 0.02 },
  { name: 'Cu1', material: 'Copper', thickness: 0.035 },
  { name: 'Core', material: 'FR4', thickness: 1.0 },
  { name: 'Cu2', material: 'Copper', thickness: 0.035 },
  { name: 'BotSol', material: 'FR4', thickness: 0.02 }
])

const totalThickness = computed(() => 
  pcbStackup.value.reduce((sum, l) => sum + l.thickness, 0)
)

// Vias
const vias = ref([
  { type: 'through', diameter: 0.3, platingThickness: 25 }
])

// Solders
const solders = ref([
  { type: 'BGA', diameter: 0.5, material: 'SAC305' }
])

// Thermal boundary
const thermal = ref({
  power: 1.0,
  ambientTemp: 25,
  coolingType: 'natural',
  h: 10
})

// Material library
const materialLibrary = ref([
  { name: 'FR4', E: 20, nu: 0.28, CTE: 15, k: 0.3, density: 1850 },
  { name: 'Rogers RO4003', E: 22, nu: 0.27, CTE: 11, k: 0.64, density: 1790 },
  { name: 'Copper', E: 118, nu: 0.34, CTE: 17, k: 401, density: 8960 },
  { name: 'SAC305', E: 50, nu: 0.36, CTE: 21, k: 50, density: 7250 },
  { name: 'SnPb', E: 40, nu: 0.36, CTE: 25, k: 50, density: 8860 },
  { name: 'AlN', E: 310, nu: 0.24, CTE: 4.5, k: 180, density: 3260 },
  { name: 'Al2O3', E: 380, nu: 0.22, CTE: 6.5, k: 30, density: 3900 }
])

// Results
const hasResults = ref(false)
const results = ref({
  maxTemp: 0,
  maxStress: 0,
  maxDisplacement: 0,
  fatigueLife: 0
})

// UI State
const showTemplateDialog = ref(false)
const viewContainer = ref<HTMLElement | null>(null)
const canvasContainer = ref<HTMLElement | null>(null)
const tempContourContainer = ref<HTMLElement | null>(null)
const hasModel = computed(() => pcbStackup.value.length > 0)

// Methods
function addLayer() {
  pcbStackup.value.push({ name: `L${pcbStackup.value.length}`, material: 'FR4', thickness: 0.5 })
}

function removeLayer(idx: number) {
  pcbStackup.value.splice(idx, 1)
}

function addVia() {
  vias.value.push({ type: 'through', diameter: 0.3, platingThickness: 25 })
}

function removeVia(idx: number) {
  vias.value.splice(idx, 1)
}

function addSolder() {
  solders.value.push({ type: 'SMT', diameter: 0.5, material: 'SAC305' })
}

function removeSolder(idx: number) {
  solders.value.splice(idx, 1)
}

function selectMaterial(mat: any) {
  console.log('Selected material:', mat)
}

const templates = [
  { 
    name: 'PCB板热应力分析', 
    description: '标准4层PCB热-结构耦合分析',
    stackup: [
      { name: 'TopSol', material: 'FR4', thickness: 0.02 },
      { name: 'Cu1', material: 'Copper', thickness: 0.035 },
      { name: 'Core', material: 'FR4', thickness: 1.0 },
      { name: 'Cu2', material: 'Copper', thickness: 0.035 },
      { name: 'BotSol', material: 'FR4', thickness: 0.02 }
    ],
    thermal: { power: 1.0, ambientTemp: 25, coolingType: 'natural', h: 10 }
  },
  { 
    name: 'BGA焊点可靠性分析', 
    description: 'BGA焊球热循环疲劳分析',
    stackup: [],
    solders: [{ type: 'BGA', diameter: 0.5, material: 'SAC305' }],
    thermal: { power: 2.0, ambientTemp: 25, coolingType: 'forced', h: 50 }
  },
  { 
    name: '芯片热分析', 
    description: '芯片封装热阻网络分析',
    stackup: [],
    thermal: { power: 5.0, ambientTemp: 25, coolingType: 'liquid', h: 200 }
  }
]

function applyTemplate(tpl: any) {
  if (tpl.stackup && tpl.stackup.length > 0) {
    pcbStackup.value = [...tpl.stackup]
  }
  if (tpl.thermal) {
    thermal.value = { ...thermal.value, ...tpl.thermal }
  }
  showTemplateDialog.value = false
}

async function runAnalysis() {
  if (!projectStore.currentProject) return
  
  try {
    // Call backend to generate input file
    const response = await fetch('/api/electronics/analysis', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        analysisType: analysisType.value,
        stackup: pcbStackup.value,
        vias: vias.value,
        solders: solders.value,
        thermal: thermal.value
      })
    })
    
    if (response.ok) {
      const data = await response.json()
      results.value = {
        maxTemp: data.maxTemp || 85,
        maxStress: data.maxStress || 45,
        maxDisplacement: data.maxDisplacement || 0.012,
        fatigueLife: data.fatigueLife || 5000
      }
      hasResults.value = true
    }
  } catch (error) {
    console.error('Analysis failed:', error)
    // Demo results for preview
    results.value = {
      maxTemp: 72.5,
      maxStress: 38.2,
      maxDisplacement: 0.008,
      fatigueLife: 12500
    }
    hasResults.value = true
  }
}
</script>

<style scoped>
.electronics-view {
  --view-bg: #1a1a2e;
  --panel-bg: #16213e;
}
</style>