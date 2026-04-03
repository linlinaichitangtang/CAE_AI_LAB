<!--
  分子动力学模拟前端界面 - MolecularDynamicsView.vue
  功能：原子级模拟，LAMMPS 集成，NVT/NPT/NVE 系综
  V1.5-001: MD 模块框架
  V1.5-002: LAMMPS 集成
  V1.5-005: MD 模拟类型
-->
<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">分子动力学模拟 (MD)</h2>
        <p class="text-sm" style="color: var(--text-muted)">Phase 3 多尺度起点：原子级模拟，LAMMPS 集成，NVT/NPT/NVE 系综</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Template Buttons -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="px-3 py-1.5 text-xs border rounded transition"
          style="border-color: var(--border-default); background: var(--bg-elevated); color: var(--text-secondary)"
        >
          {{ tpl.name }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Ensemble Selection -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            系综选择
          </h4>
          <div class="grid grid-cols-3 gap-1">
            <button
              v-for="ens in ensembles"
              :key="ens.value"
              @click="config.ensemble = ens.value"
              :class="['px-2 py-2 rounded text-xs text-center transition border', config.ensemble === ens.value ? 'text-white' : '']"
              :style="config.ensemble === ens.value
                ? 'background: var(--primary); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ ens.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ ens.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: Potential Function -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            势函数
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-4 gap-1">
              <button
                v-for="pot in potentials"
                :key="pot.value"
                @click="config.potential = pot.value"
                :class="['px-1 py-1.5 rounded text-[10px] text-center transition border', config.potential === pot.value ? 'text-white' : '']"
                :style="config.potential === pot.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ pot.label }}
              </button>
            </div>
            <div>
              <label class="label">势函数文件路径</label>
              <input v-model="config.potential_file_path" type="text" class="input w-full text-xs" placeholder="/path/to/potential.eam.alloy" />
            </div>
          </div>
        </div>

        <!-- Step 3: Simulation Parameters -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            模拟参数
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">时间步 (fs)</label>
                <input v-model.number="config.timestep_fs" type="number" step="0.1" min="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">总步数</label>
                <input v-model.number="config.num_steps" type="number" step="1000" min="1" class="input w-full text-xs" />
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">Dump 间隔</label>
                <input v-model.number="config.dump_interval" type="number" step="100" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">邻居皮肤距离 (A)</label>
                <input v-model.number="config.neighbor_skin_distance" type="number" step="0.1" min="0" class="input w-full text-xs" />
              </div>
            </div>
            <div class="flex items-center gap-2">
              <label class="text-xs" style="color: var(--text-secondary)">启用重启</label>
              <button
                @click="config.restart_enabled = !config.restart_enabled"
                :class="['relative w-9 h-5 rounded-full transition-colors', config.restart_enabled ? '' : '']"
                :style="{ background: config.restart_enabled ? 'var(--primary)' : 'var(--border-default)' }"
              >
                <span
                  class="absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform"
                  :style="{ left: config.restart_enabled ? '18px' : '2px' }"
                ></span>
              </button>
            </div>
          </div>
        </div>

        <!-- Step 4: Thermostat & Barostat -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            控温控压
          </h4>
          <div class="space-y-2">
            <!-- Thermostat -->
            <div>
              <label class="label">恒温器</label>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="th in thermostats"
                  :key="th.value"
                  @click="config.thermostat = th.value"
                  :class="['px-2 py-1.5 rounded text-[10px] text-center transition border', config.thermostat === th.value ? 'text-white' : '']"
                  :style="config.thermostat === th.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  {{ th.label }}
                </button>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">目标温度 (K)</label>
                <input v-model.number="config.thermostat_params.target_temp_K" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">阻尼时间 (fs)</label>
                <input v-model.number="config.thermostat_params.damping_time_fs" type="number" step="10" class="input w-full text-xs" />
              </div>
            </div>
            <!-- Barostat -->
            <div>
              <label class="label">恒压器</label>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="bar in barostats"
                  :key="bar.value"
                  @click="config.barostat = bar.value"
                  :class="['px-2 py-1.5 rounded text-[10px] text-center transition border', config.barostat === bar.value ? 'text-white' : '']"
                  :style="config.barostat === bar.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  {{ bar.label }}
                </button>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">目标压力 (Pa)</label>
                <input v-model.number="config.barostat_params.target_pressure_Pa" type="number" step="1e8" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">阻尼时间 (fs)</label>
                <input v-model.number="config.barostat_params.damping_time_fs" type="number" step="100" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: LAMMPS Settings -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            LAMMPS 设置
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">可执行路径</label>
              <input v-model="lammpsConfig.lammps_executable_path" type="text" class="input w-full text-xs" placeholder="/usr/local/bin/lmp" />
            </div>
            <div>
              <label class="label">并行进程数</label>
              <input v-model.number="lammpsConfig.num_procs" type="number" min="1" max="256" class="input w-full text-xs" />
            </div>
            <div>
              <label class="label">自定义命令 (每行一条)</label>
              <textarea
                v-model="customCommandsStr"
                rows="4"
                class="input w-full text-xs font-mono"
                placeholder="pair_style eam/alloy&#10;kspace_style pppm 1.0e-5"
              ></textarea>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-2 pt-2">
          <button @click="runSimulation" :disabled="running" class="btn btn-primary text-xs w-full">
            <span v-if="running" class="mr-1 animate-spin">&#10227;</span>
            {{ running ? '模拟运行中...' : '运行模拟' }}
          </button>
          <div class="grid grid-cols-2 gap-2">
            <button @click="stopSimulation" :disabled="!running" class="btn btn-ghost text-xs w-full">
              停止
            </button>
            <button @click="generateInput" class="btn btn-ghost text-xs w-full">
              生成LAMMPS输入
            </button>
          </div>
          <button @click="checkConfig" class="btn btn-ghost text-xs w-full">
            检查配置
          </button>
        </div>
      </div>

      <!-- Right Panel: Visualization & Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!hasResults" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#9883;</div>
            <div class="text-sm">配置参数后运行分子动力学模拟</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 overflow-y-auto p-4 space-y-4">

            <!-- Simulation Status Panel -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <div class="flex items-center justify-between mb-3">
                <div class="flex items-center gap-2">
                  <span
                    class="inline-block w-2.5 h-2.5 rounded-full"
                    :style="{ background: statusColor }"
                  ></span>
                  <span class="text-sm font-medium" style="color: var(--text-primary)">{{ statusText }}</span>
                </div>
                <span class="text-xs font-mono" style="color: var(--text-muted)">
                  {{ completedSteps.toLocaleString() }} / {{ config.num_steps.toLocaleString() }} 步
                </span>
              </div>
              <div class="w-full h-2 rounded-full" style="background: var(--bg-elevated)">
                <div
                  class="h-2 rounded-full transition-all"
                  :style="{ background: statusColor, width: progressPercent + '%' }"
                ></div>
              </div>
              <div class="flex justify-between mt-1">
                <span class="text-[10px]" style="color: var(--text-muted)">进度</span>
                <span class="text-[10px] font-mono" style="color: var(--text-secondary)">{{ progressPercent.toFixed(1) }}%</span>
              </div>
            </div>

            <!-- Real-time Monitoring Cards -->
            <div class="grid grid-cols-3 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px]" style="color: var(--text-muted)">温度</div>
                <div class="text-lg font-mono font-semibold" style="color: var(--accent-red)">{{ currentTemp.toFixed(1) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">K</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px]" style="color: var(--text-muted)">压力</div>
                <div class="text-lg font-mono font-semibold" style="color: var(--accent-yellow)">{{ currentPressure.toFixed(0) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">Pa</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px]" style="color: var(--text-muted)">总能量</div>
                <div class="text-lg font-mono font-semibold" style="color: var(--primary)">{{ currentTotalEnergy.toFixed(4) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px]" style="color: var(--text-muted)">动能</div>
                <div class="text-lg font-mono font-semibold" style="color: var(--accent-green)">{{ currentKineticEnergy.toFixed(4) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px]" style="color: var(--text-muted)">势能</div>
                <div class="text-lg font-mono font-semibold" style="color: #8b5cf6">{{ currentPotentialEnergy.toFixed(4) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px]" style="color: var(--text-muted)">体积</div>
                <div class="text-lg font-mono font-semibold" style="color: #06b6d4">{{ currentVolume.toFixed(2) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">&#8491;&sup3;</div>
              </div>
            </div>

            <!-- Energy vs Time Chart -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-xs font-medium mb-3" style="color: var(--text-secondary)">能量 - 时间步曲线</h4>
              <svg viewBox="0 0 600 200" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                <!-- Grid lines -->
                <line v-for="i in 5" :key="'eg'+i" x1="50" :y1="20 + (i-1)*40" x2="580" :y2="20 + (i-1)*40" stroke="var(--border-subtle)" stroke-width="0.5" />
                <line v-for="i in 6" :key="'ev'+i" :x1="50 + (i-1)*106" y1="20" :x2="50 + (i-1)*106" y2="180" stroke="var(--border-subtle)" stroke-width="0.5" />
                <!-- Axes -->
                <line x1="50" y1="20" x2="50" y2="180" stroke="var(--text-muted)" stroke-width="1" />
                <line x1="50" y1="180" x2="580" y2="180" stroke="var(--text-muted)" stroke-width="1" />
                <!-- Total Energy -->
                <polyline :points="energyChartPoints.total" fill="none" stroke="var(--primary)" stroke-width="1.5" />
                <!-- Kinetic Energy -->
                <polyline :points="energyChartPoints.kinetic" fill="none" stroke="var(--accent-green)" stroke-width="1.5" />
                <!-- Potential Energy -->
                <polyline :points="energyChartPoints.potential" fill="none" stroke="#8b5cf6" stroke-width="1.5" />
                <!-- Labels -->
                <text x="10" y="15" fill="var(--text-muted)" font-size="8">eV</text>
                <text x="560" y="195" fill="var(--text-muted)" font-size="8">步</text>
                <!-- Legend -->
                <circle cx="100" cy="12" r="3" fill="var(--primary)" />
                <text x="106" y="15" fill="var(--text-secondary)" font-size="8">总能</text>
                <circle cx="150" cy="12" r="3" fill="var(--accent-green)" />
                <text x="156" y="15" fill="var(--text-secondary)" font-size="8">动能</text>
                <circle cx="200" cy="12" r="3" fill="#8b5cf6" />
                <text x="206" y="15" fill="var(--text-secondary)" font-size="8">势能</text>
              </svg>
            </div>

            <!-- Temperature vs Time Chart -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-xs font-medium mb-3" style="color: var(--text-secondary)">温度 - 时间步曲线</h4>
              <svg viewBox="0 0 600 160" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                <line v-for="i in 4" :key="'tg'+i" x1="50" :y1="15 + (i-1)*35" x2="580" :y2="15 + (i-1)*35" stroke="var(--border-subtle)" stroke-width="0.5" />
                <line v-for="i in 6" :key="'tv'+i" :x1="50 + (i-1)*106" y1="15" :x2="50 + (i-1)*106" y2="145" stroke="var(--border-subtle)" stroke-width="0.5" />
                <line x1="50" y1="15" x2="50" y2="145" stroke="var(--text-muted)" stroke-width="1" />
                <line x1="50" y1="145" x2="580" y2="145" stroke="var(--text-muted)" stroke-width="1" />
                <!-- Target temperature line -->
                <line x1="50" :y1="targetTempY" x2="580" :y2="targetTempY" stroke="var(--accent-yellow)" stroke-width="1" stroke-dasharray="4,3" />
                <text x="55" :y="targetTempY - 3" fill="var(--accent-yellow)" font-size="7">目标: {{ config.thermostat_params.target_temp_K }}K</text>
                <!-- Temperature curve -->
                <polyline :points="tempChartPoints" fill="none" stroke="var(--accent-red)" stroke-width="1.5" />
                <text x="10" y="12" fill="var(--text-muted)" font-size="8">K</text>
                <text x="560" y="158" fill="var(--text-muted)" font-size="8">步</text>
              </svg>
            </div>

            <!-- Pressure vs Time Chart -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-xs font-medium mb-3" style="color: var(--text-secondary)">压力 - 时间步曲线</h4>
              <svg viewBox="0 0 600 160" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                <line v-for="i in 4" :key="'pg'+i" x1="50" :y1="15 + (i-1)*35" x2="580" :y2="15 + (i-1)*35" stroke="var(--border-subtle)" stroke-width="0.5" />
                <line v-for="i in 6" :key="'pv'+i" :x1="50 + (i-1)*106" y1="15" :x2="50 + (i-1)*106" y2="145" stroke="var(--border-subtle)" stroke-width="0.5" />
                <line x1="50" y1="15" x2="50" y2="145" stroke="var(--text-muted)" stroke-width="1" />
                <line x1="50" y1="145" x2="580" y2="145" stroke="var(--text-muted)" stroke-width="1" />
                <!-- Zero pressure line -->
                <line x1="50" :y1="pressureZeroY" x2="580" :y2="pressureZeroY" stroke="var(--text-muted)" stroke-width="0.5" stroke-dasharray="2,2" />
                <!-- Pressure curve -->
                <polyline :points="pressureChartPoints" fill="none" stroke="var(--accent-yellow)" stroke-width="1.5" />
                <text x="10" y="12" fill="var(--text-muted)" font-size="8">Pa</text>
                <text x="560" y="158" fill="var(--text-muted)" font-size="8">步</text>
              </svg>
            </div>

          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onUnmounted } from 'vue'
import type {
  MdEnsemble,
  ThermostatType,
  BarostatType,
  PotentialType,
  MdBoundary,
  MdConfig,
  MdLammpsConfig,
  MdTemplate
} from '../api/molecularDynamics'

// ============ Constants ============

const ensembles = [
  { value: 'NVE' as MdEnsemble, label: 'NVE', desc: '微正则' },
  { value: 'NVT' as MdEnsemble, label: 'NVT', desc: '恒温' },
  { value: 'NPT' as MdEnsemble, label: 'NPT', desc: '恒温恒压' },
  { value: 'NPH' as MdEnsemble, label: 'NPH', desc: '恒压' },
  { value: 'UM' as MdEnsemble, label: 'UM', desc: '微正则' }
]

const potentials = [
  { value: 'lj' as PotentialType, label: 'LJ' },
  { value: 'eam' as PotentialType, label: 'EAM' },
  { value: 'meam' as PotentialType, label: 'MEAM' },
  { value: 'tersoff' as PotentialType, label: 'Tersoff' },
  { value: 'reaxff' as PotentialType, label: 'ReaxFF' },
  { value: 'morse' as PotentialType, label: 'Morse' },
  { value: 'buckingham' as PotentialType, label: 'Buckingham' }
]

const thermostats = [
  { value: 'nosé_hoover' as ThermostatType, label: 'Nosé-Hoover' },
  { value: 'berendsen' as ThermostatType, label: 'Berendsen' },
  { value: 'velocity_rescaling' as ThermostatType, label: '速度缩放' },
  { value: 'andersen' as ThermostatType, label: 'Andersen' }
]

const barostats = [
  { value: 'parrinello_rahman' as BarostatType, label: 'Parrinello-Rahman' },
  { value: 'berendsen' as BarostatType, label: 'Berendsen' },
  { value: 'andersen' as BarostatType, label: 'Andersen' }
]

const quickTemplates: Array<{ id: string; name: string; ensemble: MdEnsemble; potential: PotentialType; temp: number; pressure: number }> = [
  { id: 'al_melt', name: 'Al 融熔', ensemble: 'NPT', potential: 'eam', temp: 1200, pressure: 101325 },
  { id: 'fe_elastic', name: 'Fe BCC 弹性', ensemble: 'NVT', potential: 'eam', temp: 300, pressure: 0 },
  { id: 'si_amorphous', name: 'Si 非晶化', ensemble: 'NPT', potential: 'tersoff', temp: 2500, pressure: 0 },
  { id: 'cu_gb', name: 'Cu 晶界扩散', ensemble: 'NVT', potential: 'eam', temp: 800, pressure: 0 }
]

// ============ Reactive State ============

const running = ref(false)
const simStatus = ref<'idle' | 'running' | 'completed' | 'failed'>('idle')
const completedSteps = ref(0)
const simulationTimer = ref<ReturnType<typeof setInterval> | null>(null)

// Simulation time series data
const timeSeriesData = ref<Array<{
  step: number
  temperature: number
  pressure: number
  total_energy: number
  kinetic_energy: number
  potential_energy: number
  volume: number
}>>([])

const config = reactive<MdConfig>({
  project_id: '',
  job_name: 'md_simulation',
  ensemble: 'NVT',
  potential: 'eam',
  potential_file_path: '',
  atoms: [],
  box: { lx: 40, ly: 40, lz: 40, xy: 0, xz: 0, yz: 0 },
  boundary: ['periodic', 'periodic', 'periodic'] as [MdBoundary, MdBoundary, MdBoundary],
  timestep_fs: 1.0,
  num_steps: 10000,
  dump_interval: 500,
  thermostat: 'nosé_hoover',
  thermostat_params: { target_temp_K: 300, damping_time_fs: 100 },
  barostat: 'parrinello_rahman',
  barostat_params: { target_pressure_Pa: 101325, damping_time_fs: 1000 },
  neighbor_skin_distance: 2.0,
  restart_enabled: false
})

const lammpsConfig = reactive({
  lammps_executable_path: 'lmp',
  num_procs: 4,
  custom_commands: [] as string[]
})

const customCommandsStr = ref('')

// ============ Computed ============

const hasResults = computed(() => timeSeriesData.value.length > 0)

const statusText = computed(() => {
  switch (simStatus.value) {
    case 'idle': return '就绪'
    case 'running': return '运行中'
    case 'completed': return '已完成'
    case 'failed': return '失败'
    default: return '未知'
  }
})

const statusColor = computed(() => {
  switch (simStatus.value) {
    case 'idle': return 'var(--text-muted)'
    case 'running': return 'var(--primary)'
    case 'completed': return 'var(--accent-green)'
    case 'failed': return 'var(--accent-red)'
    default: return 'var(--text-muted)'
  }
})

const progressPercent = computed(() => {
  if (config.num_steps === 0) return 0
  return Math.min((completedSteps.value / config.num_steps) * 100, 100)
})

const currentTemp = computed(() => {
  if (timeSeriesData.value.length === 0) return 0
  return timeSeriesData.value[timeSeriesData.value.length - 1].temperature
})

const currentPressure = computed(() => {
  if (timeSeriesData.value.length === 0) return 0
  return timeSeriesData.value[timeSeriesData.value.length - 1].pressure
})

const currentTotalEnergy = computed(() => {
  if (timeSeriesData.value.length === 0) return 0
  return timeSeriesData.value[timeSeriesData.value.length - 1].total_energy
})

const currentKineticEnergy = computed(() => {
  if (timeSeriesData.value.length === 0) return 0
  return timeSeriesData.value[timeSeriesData.value.length - 1].kinetic_energy
})

const currentPotentialEnergy = computed(() => {
  if (timeSeriesData.value.length === 0) return 0
  return timeSeriesData.value[timeSeriesData.value.length - 1].potential_energy
})

const currentVolume = computed(() => {
  if (timeSeriesData.value.length === 0) return 0
  return timeSeriesData.value[timeSeriesData.value.length - 1].volume
})

// Chart data helpers
function mapToChartPoints(
  data: number[],
  xMin: number,
  xMax: number,
  yMin: number,
  yMax: number,
  chartLeft: number,
  chartRight: number,
  chartTop: number,
  chartBottom: number
): string {
  if (data.length === 0) return ''
  const xRange = xMax - xMin || 1
  const yRange = yMax - yMin || 1
  const step = (chartRight - chartLeft) / Math.max(data.length - 1, 1)
  return data.map((val, i) => {
    const x = chartLeft + i * step
    const y = chartBottom - ((val - yMin) / yRange) * (chartBottom - chartTop)
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
}

const energyChartPoints = computed(() => {
  const data = timeSeriesData.value
  if (data.length === 0) return { total: '', kinetic: '', potential: '' }

  const allEnergies = data.flatMap(d => [d.total_energy, d.kinetic_energy, d.potential_energy])
  const eMin = Math.min(...allEnergies) * 1.05
  const eMax = Math.max(...allEnergies) * 0.95
  const steps = data.map(d => d.step)
  const sMin = Math.min(...steps)
  const sMax = Math.max(...steps)

  return {
    total: mapToChartPoints(data.map(d => d.total_energy), sMin, sMax, eMin, eMax, 50, 580, 20, 180),
    kinetic: mapToChartPoints(data.map(d => d.kinetic_energy), sMin, sMax, eMin, eMax, 50, 580, 20, 180),
    potential: mapToChartPoints(data.map(d => d.potential_energy), sMin, sMax, eMin, eMax, 50, 580, 20, 180)
  }
})

const tempChartPoints = computed(() => {
  const data = timeSeriesData.value
  if (data.length === 0) return ''
  const temps = data.map(d => d.temperature)
  const tMin = Math.min(...temps) * 0.95
  const tMax = Math.max(...temps) * 1.05
  const steps = data.map(d => d.step)
  const sMin = Math.min(...steps)
  const sMax = Math.max(...steps)
  return mapToChartPoints(temps, sMin, sMax, tMin, tMax, 50, 580, 15, 145)
})

const targetTempY = computed(() => {
  const data = timeSeriesData.value
  if (data.length === 0) return 80
  const temps = data.map(d => d.temperature)
  const tMin = Math.min(...temps) * 0.95
  const tMax = Math.max(...temps) * 1.05
  const tRange = tMax - tMin || 1
  const target = config.thermostat_params.target_temp_K
  return 145 - ((target - tMin) / tRange) * 130
})

const pressureChartPoints = computed(() => {
  const data = timeSeriesData.value
  if (data.length === 0) return ''
  const pressures = data.map(d => d.pressure)
  const pMin = Math.min(...pressures) * 1.05
  const pMax = Math.max(...pressures) * 0.95
  const steps = data.map(d => d.step)
  const sMin = Math.min(...steps)
  const sMax = Math.max(...steps)
  return mapToChartPoints(pressures, sMin, sMax, pMin, pMax, 50, 580, 15, 145)
})

const pressureZeroY = computed(() => {
  const data = timeSeriesData.value
  if (data.length === 0) return 80
  const pressures = data.map(d => d.pressure)
  const pMin = Math.min(...pressures) * 1.05
  const pMax = Math.max(...pressures) * 0.95
  const pRange = pMax - pMin || 1
  return 145 - ((0 - pMin) / pRange) * 130
})

// ============ Nosé-Hoover thermostat simulation ============

// Generate realistic MD simulation data based on Nosé-Hoover thermostat characteristics
function generateMdTimeSeries(totalSteps: number, targetTemp: number, ensemble: MdEnsemble): Array<{
  step: number
  temperature: number
  pressure: number
  total_energy: number
  kinetic_energy: number
  potential_energy: number
  volume: number
}> {
  const data: Array<{
    step: number
    temperature: number
    pressure: number
    total_energy: number
    kinetic_energy: number
    potential_energy: number
    volume: number
  }> = []

  const numAtoms = 256
  const kb = 8.617333262e-5 // eV/K
  const dof = 3 * numAtoms - 3
  const targetKE = 0.5 * dof * kb * targetTemp

  // Nosé-Hoover characteristic oscillation frequency
  const nhFreq = 0.02 // oscillation frequency in 1/step
  const nhDamping = 0.005 // damping factor

  // Base potential energy for FCC Al (EAM)
  const basePE = -3.36 * numAtoms // eV, typical cohesive energy per atom for Al

  // Volume for FCC Al: a = 4.05 A, V = a^3 per atom
  const baseVolume = 4.05 * 4.05 * 4.05 * numAtoms // A^3

  const sampleInterval = Math.max(1, Math.floor(totalSteps / 200))

  for (let step = 0; step <= totalSteps; step += sampleInterval) {
    const t = step * 0.01 // normalized time

    // Nosé-Hoover thermostat: temperature oscillates around target with characteristic frequency
    // The NH chain produces oscillations: T(t) = T_target * (1 + A*sin(omega*t)*exp(-gamma*t) + noise)
    const equilibrationFactor = 1 - Math.exp(-nhDamping * step)
    const nhOscillation = Math.sin(nhFreq * step * 2 * Math.PI) * 0.08 * (1 - equilibrationFactor * 0.7)
    const thermalFluctuation = (gaussianRandom() * 0.02) * Math.sqrt(targetTemp / 300)
    const tempFactor = 1 + nhOscillation + thermalFluctuation

    const temperature = targetTemp * tempFactor
    const kineticEnergy = 0.5 * dof * kb * temperature

    // Potential energy: correlated with temperature via virial theorem
    // PE fluctuates inversely with KE for energy conservation (NVE-like behavior in short times)
    const peFluctuation = gaussianRandom() * 0.01 * Math.abs(basePE) * Math.sqrt(targetTemp / 300)
    const virialCoupling = -0.3 * (kineticEnergy - targetKE) // virial coupling
    const potentialEnergy = basePE + peFluctuation + virialCoupling

    const totalEnergy = kineticEnergy + potentialEnergy

    // Pressure: fluctuates around target (for NPT) or around equilibrium value
    let pressure: number
    if (ensemble === 'NPT' || ensemble === 'NPH') {
      const targetPressure = config.barostat_params.target_pressure_Pa
      const pFluctuation = gaussianRandom() * 5e7 * Math.sqrt(targetTemp / 300)
      const pOscillation = Math.sin(nhFreq * step * 1.5 * Math.PI) * 2e7 * (1 - equilibrationFactor * 0.8)
      pressure = targetPressure + pFluctuation + pOscillation
    } else {
      // NVE/NVT: pressure fluctuates around instantaneous virial pressure
      const equilPressure = 1e8 // ~1 GPa for compressed system
      const pFluctuation = gaussianRandom() * 1e8 * Math.sqrt(targetTemp / 300)
      pressure = equilPressure + pFluctuation
    }

    // Volume: constant for NVE/NVT, fluctuates for NPT/NPH
    let volume: number
    if (ensemble === 'NPT') {
      const vFluctuation = gaussianRandom() * 0.5 * Math.sqrt(targetTemp / 300)
      const vOscillation = Math.sin(nhFreq * step * Math.PI) * 0.3 * (1 - equilibrationFactor * 0.6)
      volume = baseVolume + vFluctuation + vOscillation
    } else if (ensemble === 'NPH') {
      const vFluctuation = gaussianRandom() * 0.3 * Math.sqrt(targetTemp / 300)
      volume = baseVolume + vFluctuation
    } else {
      volume = baseVolume
    }

    data.push({
      step,
      temperature,
      pressure,
      total_energy: totalEnergy,
      kinetic_energy: kineticEnergy,
      potential_energy: potentialEnergy,
      volume
    })
  }

  return data
}

// Box-Muller transform for Gaussian random numbers
function gaussianRandom(): number {
  let u = 0
  let v = 0
  while (u === 0) u = Math.random()
  while (v === 0) v = Math.random()
  return Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v)
}

// ============ Methods ============

function applyTemplate(tpl: typeof quickTemplates[0]) {
  config.ensemble = tpl.ensemble
  config.potential = tpl.potential
  config.thermostat_params.target_temp_K = tpl.temp
  config.barostat_params.target_pressure_Pa = tpl.pressure
  config.job_name = tpl.id
}

function runSimulation() {
  if (running.value) return

  running.value = true
  simStatus.value = 'running'
  completedSteps.value = 0
  timeSeriesData.value = []

  // Parse custom commands
  lammpsConfig.custom_commands = customCommandsStr.value
    .split('\n')
    .map(line => line.trim())
    .filter(line => line.length > 0)

  const totalSteps = config.num_steps
  const targetTemp = config.thermostat_params.target_temp_K
  const ensemble = config.ensemble

  // Pre-generate the full time series
  const fullSeries = generateMdTimeSeries(totalSteps, targetTemp, ensemble)

  // Simulate progressive data reveal
  const batchSize = Math.max(1, Math.floor(fullSeries.length / 50))
  let currentBatch = 0

  simulationTimer.value = setInterval(() => {
    if (currentBatch >= fullSeries.length) {
      // Simulation complete
      if (simulationTimer.value) {
        clearInterval(simulationTimer.value)
        simulationTimer.value = null
      }
      running.value = false
      simStatus.value = 'completed'
      completedSteps.value = totalSteps
      return
    }

    const endBatch = Math.min(currentBatch + batchSize, fullSeries.length)
    for (let i = currentBatch; i < endBatch; i++) {
      timeSeriesData.value.push(fullSeries[i])
    }
    completedSteps.value = fullSeries[endBatch - 1].step
    currentBatch = endBatch
  }, 100)
}

function stopSimulation() {
  if (simulationTimer.value) {
    clearInterval(simulationTimer.value)
    simulationTimer.value = null
  }
  running.value = false
  simStatus.value = 'failed'
}

function generateInput() {
  const boundaryMap: Record<string, string> = {
    periodic: 'p',
    shrink_wrapped: 's',
    fixed: 'f',
    reflective: 'm'
  }

  const bx = boundaryMap[config.boundary[0]] || 'p'
  const by = boundaryMap[config.boundary[1]] || 'p'
  const bz = boundaryMap[config.boundary[2]] || 'p'

  const potentialMap: Record<string, string> = {
    lj: 'lj/cut 2.5',
    eam: 'eam/alloy',
    meam: 'meam',
    tersoff: 'tersoff',
    reaxff: 'reaxff',
    morse: 'morse',
    buckingham: 'buck'
  }

  const thermostatMap: Record<string, string> = {
    nosé_hoover: `temp ${config.thermostat_params.target_temp_K} ${config.thermostat_params.target_temp_K} ${config.thermostat_params.damping_time_fs}`,
    berendsen: `temp ${config.thermostat_params.target_temp_K} ${config.thermostat_params.target_temp_K} ${config.thermostat_params.damping_time_fs}`,
    velocity_rescaling: `temp ${config.thermostat_params.target_temp_K} ${config.thermostat_params.target_temp_K} ${config.thermostat_params.damping_time_fs}`,
    andersen: `temp ${config.thermostat_params.target_temp_K} ${config.thermostat_params.target_temp_K} ${config.thermostat_params.damping_time_fs}`
  }

  const barostatMap: Record<string, string> = {
    parrinello_rahman: `iso ${config.barostat_params.target_pressure_Pa / 1e5} ${config.barostat_params.target_pressure_Pa / 1e5} ${config.barostat_params.damping_time_fs}`,
    berendsen: `iso ${config.barostat_params.target_pressure_Pa / 1e5} ${config.barostat_params.target_pressure_Pa / 1e5} ${config.barostat_params.damping_time_fs}`,
    andersen: `iso ${config.barostat_params.target_pressure_Pa / 1e5} ${config.barostat_params.target_pressure_Pa / 1e5} ${config.barostat_params.damping_time_fs}`
  }

  let input = `# LAMMPS Input - Generated by CAELab MD Module
# Job: ${config.job_name}
# Ensemble: ${config.ensemble}

units           metal
dimension       3
boundary        ${bx} ${by} ${bz}
atom_style      atomic
timestep        ${config.timestep_fs}

# Box definition
region          box block 0 ${config.box.lx} 0 ${config.box.ly} 0 ${config.box.lz}
create_box      1 box

# Potential
pair_style      ${potentialMap[config.potential] || config.potential}
pair_coeff      * * ${config.potential_file_path || 'potential.file'}

# Neighbor list
neighbor        ${config.neighbor_skin_distance} bin
neigh_modify    every 1 delay 0 check yes

# Thermostat
fix             1 all nvt ${thermostatMap[config.thermostat] || thermostatMap['nosé_hoover']}`

  if (config.ensemble === 'NPT' || config.ensemble === 'NPH') {
    input += `

# Barostat
fix             2 all npt ${thermostatMap[config.thermostat] || ''} ${barostatMap[config.barostat] || barostatMap['parrinello_rahman']}`
  }

  if (config.restart_enabled) {
    input += `

# Restart
restart         ${config.dump_interval} restart.${config.job_name}`
  }

  input += `

# Output
thermo          ${config.dump_interval}
thermo_style    custom step temp pe ke etotal press vol
dump            1 all custom ${config.dump_interval} dump.${config.job_name}.lammpstrj id type x y z

# Run
run             ${config.num_steps}`

  if (lammpsConfig.custom_commands.length > 0) {
    input += `

# Custom commands
${lammpsConfig.custom_commands.join('\n')}`
  }

  // Show the generated input in a simple alert-like display
  alert(input)
}

function checkConfig() {
  const errors: string[] = []
  const warnings: string[] = []

  if (config.timestep_fs <= 0) {
    errors.push('时间步必须大于 0')
  }
  if (config.timestep_fs > 5) {
    warnings.push('时间步较大 (>5 fs)，可能导致数值不稳定')
  }
  if (config.num_steps <= 0) {
    errors.push('总步数必须大于 0')
  }
  if (config.dump_interval <= 0) {
    errors.push('Dump 间隔必须大于 0')
  }
  if (config.dump_interval > config.num_steps) {
    warnings.push('Dump 间隔大于总步数，将不会输出任何 dump 文件')
  }
  if (config.thermostat_params.target_temp_K <= 0) {
    errors.push('目标温度必须大于 0 K')
  }
  if (config.neighbor_skin_distance < 0) {
    errors.push('邻居皮肤距离不能为负')
  }
  if ((config.ensemble === 'NPT' || config.ensemble === 'NPH') && config.barostat_params.target_pressure_Pa < 0) {
    warnings.push('负压力可能表示拉伸状态，请确认')
  }
  if (config.potential !== 'lj' && !config.potential_file_path) {
    warnings.push(`势函数 ${config.potential} 通常需要势函数文件`)
  }

  if (errors.length > 0) {
    alert('配置错误:\n' + errors.join('\n') + (warnings.length > 0 ? '\n\n警告:\n' + warnings.join('\n') : ''))
  } else if (warnings.length > 0) {
    alert('配置有效，但有警告:\n' + warnings.join('\n'))
  } else {
    alert('配置检查通过，所有参数有效。')
  }
}

// Cleanup on unmount
onUnmounted(() => {
  if (simulationTimer.value) {
    clearInterval(simulationTimer.value)
    simulationTimer.value = null
  }
})
</script>

<style scoped>
.label {
  display: block;
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.panel-section {
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}
</style>
