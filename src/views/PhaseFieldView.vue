<!--
  PhaseFieldView.vue - 相场法模拟视图
  ==========================================
  V1.6-001: 相场方程求解器 (Cahn-Hilliard / Allen-Cahn / 相场晶体 / Karma模型)
  V1.6-002: 组织初始化器 (随机/形核/层状/圆形/导入)
  V1.6-003: 相场-温度耦合 (单向/双向热耦合)
-->

<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">相场法模拟 (Phase Field)</h2>
        <p class="text-sm" style="color: var(--text-muted)">微观组织演变：Cahn-Hilliard / Allen-Cahn，晶粒长大/相分离/析出</p>
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
        <button @click="resetAll" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--border-default); color: var(--text-secondary)">
          重置
        </button>
        <button @click="stopSimulation" v-if="simulating" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--accent-red); color: var(--accent-red)">
          停止
        </button>
        <button @click="runSimulation" :disabled="simulating" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--primary); color: #fff; background: var(--primary)">
          <span v-if="simulating" class="mr-1 animate-spin">&#10227;</span>
          {{ simulating ? '模拟中...' : '运行模拟' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Step 1: Equation Type -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
            方程类型
          </h4>
          <div class="grid grid-cols-2 gap-1">
            <button
              v-for="eq in equationTypes"
              :key="eq.value"
              @click="config.equation = eq.value"
              :class="['px-2 py-2 rounded text-xs text-left transition border', config.equation === eq.value ? 'text-white' : '']"
              :style="config.equation === eq.value
                ? 'background: var(--primary); border-color: var(--primary)'
                : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
            >
              <div class="font-medium">{{ eq.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ eq.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 2: Free Energy -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
            自由能
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">自由能类型</label>
              <select v-model="config.material.free_energy_type" class="input w-full text-xs">
                <option value="double_well">双势阱 (Double Well)</option>
                <option value="polynomial">多项式 (Polynomial)</option>
                <option value="landau">Landau</option>
                <option value="regular_solution">正规溶液 (Regular Solution)</option>
              </select>
            </div>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">A</label>
                <input v-model.number="config.material.free_energy_coefficients.A" type="number" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">B</label>
                <input v-model.number="config.material.free_energy_coefficients.B" type="number" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">C</label>
                <input v-model.number="config.material.free_energy_coefficients.C" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">kappa (k)</label>
                <input v-model.number="config.material.gradient_energy_coeff" type="number" step="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">迁移率 M</label>
                <input v-model.number="config.material.mobility" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">界面宽度</label>
                <input v-model.number="config.material.interface_width" type="number" step="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">平衡序参量</label>
                <input v-model.number="config.material.equilibrium_order_parameter" type="number" step="0.01" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Grid Settings -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
            网格设置 (2D)
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">nx</label>
                <input v-model.number="config.grid.nx" type="number" step="10" min="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">ny</label>
                <input v-model.number="config.grid.ny" type="number" step="10" min="10" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">dx = dy</label>
              <input v-model.number="config.grid.dx" type="number" step="0.01" min="0.001" class="input w-full text-xs" />
            </div>
          </div>
        </div>

        <!-- Step 4: Initial Condition -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">4</span>
            初始条件
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-3 gap-1">
              <button
                v-for="ic in initialConditionTypes"
                :key="ic.value"
                @click="config.initial_condition.type = ic.value"
                :class="['px-2 py-1.5 rounded text-xs transition border', config.initial_condition.type === ic.value ? 'text-white' : '']"
                :style="config.initial_condition.type === ic.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ ic.label }}
              </button>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">晶粒数</label>
                <input v-model.number="config.initial_condition.grain_count" type="number" step="1" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">噪声振幅</label>
                <input v-model.number="config.initial_condition.noise_amplitude" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" />
              </div>
            </div>
            <div v-if="config.initial_condition.type === 'nucleation'">
              <label class="label">形核位点 (x, y, r)</label>
              <div class="space-y-1">
                <div v-for="(site, idx) in config.initial_condition.nucleation_sites" :key="idx" class="flex gap-1 items-center">
                  <input v-model.number="site.x" type="number" step="0.1" class="input w-full text-xs" placeholder="x" />
                  <input v-model.number="site.y" type="number" step="0.1" class="input w-full text-xs" placeholder="y" />
                  <input v-model.number="site.r" type="number" step="0.1" class="input w-full text-xs" placeholder="r" />
                  <button @click="removeNucleationSite(idx)" class="text-xs px-1" style="color: var(--accent-red)">&times;</button>
                </div>
                <button @click="addNucleationSite" class="text-xs px-2 py-1 border rounded" style="border-color: var(--border-default); color: var(--text-secondary)">
                  + 添加位点
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 5: Thermal Coupling -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">5</span>
            热耦合
          </h4>
          <div class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">初始温度 (K)</label>
                <input v-model.number="config.thermal.initial_temperature_K" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">边界温度 (K)</label>
                <input v-model.number="config.thermal.boundary_temperature_K" type="number" step="10" class="input w-full text-xs" />
              </div>
            </div>
            <div>
              <label class="label">耦合方式</label>
              <div class="grid grid-cols-3 gap-1">
                <button
                  v-for="tc in thermalCouplingTypes"
                  :key="tc.value"
                  @click="config.thermal.thermal_coupling = tc.value"
                  :class="['px-2 py-1.5 rounded text-xs transition border', config.thermal.thermal_coupling === tc.value ? 'text-white' : '']"
                  :style="config.thermal.thermal_coupling === tc.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  {{ tc.label }}
                </button>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <input type="checkbox" v-model="config.thermal.latent_heat_release" id="latent_heat" class="rounded" />
              <label for="latent_heat" class="text-xs" style="color: var(--text-secondary)">潜热释放</label>
            </div>
          </div>
        </div>

        <!-- Step 6: Solver Control -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">6</span>
            求解控制
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">时间积分</label>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="ti in timeIntegrationTypes"
                  :key="ti.value"
                  @click="config.time_integration = ti.value"
                  :class="['px-2 py-1.5 rounded text-xs transition border', config.time_integration === ti.value ? 'text-white' : '']"
                  :style="config.time_integration === ti.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  {{ ti.label }}
                </button>
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">dt</label>
                <input v-model.number="config.dt" type="number" step="0.001" min="1e-6" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">总步数</label>
                <input v-model.number="config.num_steps" type="number" step="100" min="1" class="input w-full text-xs" />
              </div>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">输出间隔</label>
                <input v-model.number="config.output_interval" type="number" step="10" min="1" class="input w-full text-xs" />
              </div>
              <div class="flex items-end pb-1">
                <label class="flex items-center gap-2 cursor-pointer">
                  <input type="checkbox" v-model="config.adaptive_timestep" class="rounded" />
                  <span class="text-xs" style="color: var(--text-secondary)">自适应时间步</span>
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No results placeholder -->
        <div v-if="!hasResult" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#9881;</div>
            <div class="text-sm">配置参数后运行相场模拟</div>
          </div>
        </div>

        <!-- Results Content -->
        <template v-else>
          <div class="flex-1 overflow-y-auto p-4">
            <!-- Canvas Row: Order Parameter + Temperature + Free Energy -->
            <div class="grid grid-cols-3 gap-3 mb-4">
              <!-- Order Parameter Field -->
              <div class="result-card">
                <span class="result-label">序参量场 (eta)</span>
                <div class="mt-2 relative" style="background: #0A0B0E; border-radius: var(--radius-md); overflow: hidden;">
                  <canvas ref="orderParamCanvas" class="w-full" style="aspect-ratio: 1;"></canvas>
                  <div class="absolute bottom-1 left-1 right-1 flex justify-between items-center">
                    <span class="text-[9px] px-1 rounded" style="background: rgba(0,0,0,0.6); color: #fff;">
                      min: {{ stats.minOrderParam.toFixed(3) }}
                    </span>
                    <span class="text-[9px] px-1 rounded" style="background: rgba(0,0,0,0.6); color: #fff;">
                      max: {{ stats.maxOrderParam.toFixed(3) }}
                    </span>
                  </div>
                </div>
                <!-- Color Legend -->
                <div class="mt-1 flex items-center gap-1">
                  <span class="text-[9px]" style="color: var(--text-muted)">-1</span>
                  <div class="flex-1 h-2 rounded-full" style="background: linear-gradient(to right, #2563EB, #7C3AED, #EC4899, #EF4444);"></div>
                  <span class="text-[9px]" style="color: var(--text-muted)">+1</span>
                </div>
              </div>

              <!-- Temperature Field -->
              <div class="result-card">
                <span class="result-label">温度场 (K)</span>
                <div class="mt-2 relative" style="background: #0A0B0E; border-radius: var(--radius-md); overflow: hidden;">
                  <canvas ref="temperatureCanvas" class="w-full" style="aspect-ratio: 1;"></canvas>
                  <div class="absolute bottom-1 left-1 right-1 flex justify-between items-center">
                    <span class="text-[9px] px-1 rounded" style="background: rgba(0,0,0,0.6); color: #fff;">
                      min: {{ stats.minTemp.toFixed(1) }}K
                    </span>
                    <span class="text-[9px] px-1 rounded" style="background: rgba(0,0,0,0.6); color: #fff;">
                      max: {{ stats.maxTemp.toFixed(1) }}K
                    </span>
                  </div>
                </div>
                <div class="mt-1 flex items-center gap-1">
                  <span class="text-[9px]" style="color: var(--text-muted)">冷</span>
                  <div class="flex-1 h-2 rounded-full" style="background: linear-gradient(to right, #3B82F6, #06B6D4, #22C55E, #EAB308, #EF4444);"></div>
                  <span class="text-[9px]" style="color: var(--text-muted)">热</span>
                </div>
              </div>

              <!-- Free Energy Density -->
              <div class="result-card">
                <span class="result-label">自由能密度 f(eta)</span>
                <div class="mt-2 relative" style="background: #0A0B0E; border-radius: var(--radius-md); overflow: hidden;">
                  <canvas ref="freeEnergyCanvas" class="w-full" style="aspect-ratio: 1;"></canvas>
                  <div class="absolute bottom-1 left-1 right-1 flex justify-between items-center">
                    <span class="text-[9px] px-1 rounded" style="background: rgba(0,0,0,0.6); color: #fff;">
                      min: {{ stats.minFE.toFixed(4) }}
                    </span>
                    <span class="text-[9px] px-1 rounded" style="background: rgba(0,0,0,0.6); color: #fff;">
                      max: {{ stats.maxFE.toFixed(4) }}
                    </span>
                  </div>
                </div>
                <div class="mt-1 flex items-center gap-1">
                  <span class="text-[9px]" style="color: var(--text-muted)">低</span>
                  <div class="flex-1 h-2 rounded-full" style="background: linear-gradient(to right, #1E293B, #334155, #64748B, #94A3B8, #E2E8F0);"></div>
                  <span class="text-[9px]" style="color: var(--text-muted)">高</span>
                </div>
              </div>
            </div>

            <!-- Monitoring Cards -->
            <div class="grid grid-cols-6 gap-3 mb-4">
              <div class="result-card text-center">
                <span class="result-label">总自由能</span>
                <div class="result-value text-sm mt-1" style="color: var(--primary)">{{ stats.totalFE.toFixed(2) }}</div>
              </div>
              <div class="result-card text-center">
                <span class="result-label">界面面积</span>
                <div class="result-value text-sm mt-1" style="color: var(--accent-yellow)">{{ stats.interfaceArea.toFixed(1) }}</div>
              </div>
              <div class="result-card text-center">
                <span class="result-label">晶粒数</span>
                <div class="result-value text-sm mt-1" style="color: var(--accent-green)">{{ stats.grainCount }}</div>
              </div>
              <div class="result-card text-center">
                <span class="result-label">平均晶粒尺寸</span>
                <div class="result-value text-sm mt-1" style="color: var(--text-primary)">{{ stats.avgGrainSize.toFixed(2) }}</div>
              </div>
              <div class="result-card text-center">
                <span class="result-label">max(eta)</span>
                <div class="result-value text-sm mt-1" style="color: var(--accent-red)">{{ stats.maxOrderParam.toFixed(3) }}</div>
              </div>
              <div class="result-card text-center">
                <span class="result-label">min(eta)</span>
                <div class="result-value text-sm mt-1" style="color: #2563EB">{{ stats.minOrderParam.toFixed(3) }}</div>
              </div>
            </div>

            <!-- Charts Row: Free Energy vs Time + Grain Count vs Time -->
            <div class="grid grid-cols-2 gap-3 mb-4">
              <!-- Free Energy - Time Curve -->
              <div class="result-card">
                <span class="result-label">自由能 - 时间曲线</span>
                <div class="mt-2" style="background: #0A0B0E; border-radius: var(--radius-md); padding: 8px;">
                  <svg ref="feChartSvg" viewBox="0 0 400 200" class="w-full" style="height: 180px;">
                    <!-- Grid lines -->
                    <line v-for="i in 5" :key="'h'+i" :x1="40" :y1="10 + (i-1)*45" :x2="390" :y2="10 + (i-1)*45" stroke="#1E293B" stroke-width="0.5"/>
                    <line v-for="i in 5" :key="'v'+i" :x1="40 + (i-1)*87.5" :y1="10" :x2="40 + (i-1)*87.5" :y2="190" stroke="#1E293B" stroke-width="0.5"/>
                    <!-- Axes -->
                    <line x1="40" y1="10" x2="40" y2="190" stroke="#475569" stroke-width="1"/>
                    <line x1="40" y1="190" x2="390" y2="190" stroke="#475569" stroke-width="1"/>
                    <!-- Data path -->
                    <polyline
                      v-if="feChartData.length > 1"
                      :points="feChartData.map((p, i) => `${40 + (i / (feChartData.length - 1)) * 350},${190 - ((p - feChartMin) / (feChartMax - feChartMin + 1e-10)) * 170}`).join(' ')"
                      fill="none"
                      stroke="#4F8CFF"
                      stroke-width="1.5"
                    />
                    <!-- Labels -->
                    <text x="20" y="15" fill="#94A3B8" font-size="8" text-anchor="end">{{ feChartMax.toFixed(1) }}</text>
                    <text x="20" y="105" fill="#94A3B8" font-size="8" text-anchor="end">{{ ((feChartMax + feChartMin) / 2).toFixed(1) }}</text>
                    <text x="20" y="193" fill="#94A3B8" font-size="8" text-anchor="end">{{ feChartMin.toFixed(1) }}</text>
                    <text x="215" y="208" fill="#94A3B8" font-size="8" text-anchor="middle">time</text>
                  </svg>
                </div>
              </div>

              <!-- Grain Count - Time Curve -->
              <div class="result-card">
                <span class="result-label">晶粒数 - 时间曲线</span>
                <div class="mt-2" style="background: #0A0B0E; border-radius: var(--radius-md); padding: 8px;">
                  <svg ref="gcChartSvg" viewBox="0 0 400 200" class="w-full" style="height: 180px;">
                    <!-- Grid lines -->
                    <line v-for="i in 5" :key="'h'+i" :x1="40" :y1="10 + (i-1)*45" :x2="390" :y2="10 + (i-1)*45" stroke="#1E293B" stroke-width="0.5"/>
                    <line v-for="i in 5" :key="'v'+i" :x1="40 + (i-1)*87.5" :y1="10" :x2="40 + (i-1)*87.5" :y2="190" stroke="#1E293B" stroke-width="0.5"/>
                    <!-- Axes -->
                    <line x1="40" y1="10" x2="40" y2="190" stroke="#475569" stroke-width="1"/>
                    <line x1="40" y1="190" x2="390" y2="190" stroke="#475569" stroke-width="1"/>
                    <!-- Data path -->
                    <polyline
                      v-if="gcChartData.length > 1"
                      :points="gcChartData.map((p, i) => `${40 + (i / (gcChartData.length - 1)) * 350},${190 - ((p - gcChartMin) / (gcChartMax - gcChartMin + 1e-10)) * 170}`).join(' ')"
                      fill="none"
                      stroke="#22C55E"
                      stroke-width="1.5"
                    />
                    <!-- Labels -->
                    <text x="20" y="15" fill="#94A3B8" font-size="8" text-anchor="end">{{ gcChartMax.toFixed(0) }}</text>
                    <text x="20" y="105" fill="#94A3B8" font-size="8" text-anchor="end">{{ ((gcChartMax + gcChartMin) / 2).toFixed(0) }}</text>
                    <text x="20" y="193" fill="#94A3B8" font-size="8" text-anchor="end">{{ gcChartMin.toFixed(0) }}</text>
                    <text x="215" y="208" fill="#94A3B8" font-size="8" text-anchor="middle">time</text>
                  </svg>
                </div>
              </div>
            </div>

            <!-- Progress Info -->
            <div v-if="simulating" class="result-card">
              <div class="flex items-center justify-between">
                <span class="result-label">模拟进度</span>
                <span class="text-xs font-mono" style="color: var(--text-secondary)">
                  步 {{ currentStep }} / {{ config.num_steps }}
                </span>
              </div>
              <div class="w-full h-2 rounded-full mt-2" style="background: var(--bg-elevated)">
                <div
                  class="h-2 rounded-full transition-all"
                  :style="{ width: progressPercent + '%', background: 'var(--primary)' }"
                ></div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick, watch } from 'vue'
import type {
  PfEquation,
  FreeEnergyType,
  TimeIntegration,
  PfGrid,
  PfMaterial,
  PfInitialCondition,
  PfThermalConfig,
  PfSolverConfig,
  PfFrame,
} from '../api/phaseField'

// ============ Constants ============

const equationTypes: Array<{ value: PfEquation; label: string; desc: string }> = [
  { value: 'cahn_hilliard', label: 'Cahn-Hilliard', desc: '相分离/旋节分解' },
  { value: 'allen_cahn', label: 'Allen-Cahn', desc: '晶粒长大/有序化' },
  { value: 'phase_field_crystal', label: '相场晶体', desc: '原子尺度晶体' },
  { value: 'karma_model', label: 'Karma模型', desc: '枝晶凝固' },
]

const initialConditionTypes = [
  { value: 'random' as const, label: '随机' },
  { value: 'nucleation' as const, label: '形核' },
  { value: 'layered' as const, label: '层状' },
  { value: 'circular' as const, label: '圆形' },
  { value: 'import' as const, label: '导入' },
]

const thermalCouplingTypes = [
  { value: 'none' as const, label: '无' },
  { value: 'one_way' as const, label: '单向' },
  { value: 'two_way' as const, label: '双向' },
]

const timeIntegrationTypes: Array<{ value: TimeIntegration; label: string }> = [
  { value: 'explicit_euler', label: '显式 Euler' },
  { value: 'implicit_euler', label: '隐式 Euler' },
  { value: 'semi_implicit', label: '半隐式' },
  { value: 'runge_kutta4', label: 'RK4' },
]

const quickTemplates = [
  { id: 'grain_growth', name: '晶粒长大(CH)', equation: 'cahn_hilliard' as PfEquation },
  { id: 'phase_separation', name: '相分离(CH)', equation: 'cahn_hilliard' as PfEquation },
  { id: 'dendrite', name: '枝晶凝固(Karma)', equation: 'karma_model' as PfEquation },
  { id: 'precipitation', name: '沉淀析出(AC)', equation: 'allen_cahn' as PfEquation },
]

// ============ State ============

const simulating = ref(false)
const currentStep = ref(0)
const stopRequested = ref(false)

// Field data
let orderParamField: number[][] = []
let temperatureField: number[][] = []
let freeEnergyField: number[][] = []
let grainBoundaryField: number[][] = []

// History for charts
const feHistory: number[] = []
const gcHistory: number[] = []
const timeHistory: number[] = []

// Canvas refs
const orderParamCanvas = ref<HTMLCanvasElement | null>(null)
const temperatureCanvas = ref<HTMLCanvasElement | null>(null)
const freeEnergyCanvas = ref<HTMLCanvasElement | null>(null)
const feChartSvg = ref<SVGSVGElement | null>(null)
const gcChartSvg = ref<SVGSVGElement | null>(null)

const config = reactive<PfSolverConfig>({
  project_id: '',
  equation: 'cahn_hilliard',
  grid: { nx: 128, ny: 128, nz: 1, dx: 0.5, dy: 0.5, dz: 1.0 },
  material: {
    name: 'Default',
    free_energy_type: 'double_well',
    free_energy_coefficients: { A: 1.0, B: 1.0, C: 0.0, m: 0.0 },
    mobility: 1.0,
    gradient_energy_coeff: 0.5,
    latent_heat: 0.0,
    heat_capacity: 1.0,
    thermal_conductivity: 1.0,
    interface_width: 1.0,
    equilibrium_order_parameter: 1.0,
  },
  initial_condition: {
    type: 'random',
    params: {},
    grain_count: 20,
    nucleation_sites: [
      { x: 0.25, y: 0.25, r: 0.08 },
      { x: 0.75, y: 0.75, r: 0.08 },
      { x: 0.25, y: 0.75, r: 0.08 },
      { x: 0.75, y: 0.25, r: 0.08 },
    ],
    noise_amplitude: 0.05,
  },
  thermal: {
    initial_temperature_K: 800,
    boundary_temperature_K: 300,
    thermal_coupling: 'none',
    latent_heat_release: false,
    heat_source: [],
  },
  time_integration: 'explicit_euler',
  dt: 0.01,
  num_steps: 500,
  output_interval: 10,
  adaptive_timestep: false,
  convergence_tolerance: 1e-6,
})

// ============ Computed ============

const hasResult = computed(() => orderParamField.length > 0)

const progressPercent = computed(() => {
  if (config.num_steps <= 0) return 0
  return Math.min((currentStep.value / config.num_steps) * 100, 100)
})

const stats = computed(() => {
  if (orderParamField.length === 0) {
    return {
      totalFE: 0, interfaceArea: 0, grainCount: 0, avgGrainSize: 0,
      maxOrderParam: 0, minOrderParam: 0,
      maxTemp: 0, minTemp: 0, maxFE: 0, minFE: 0,
    }
  }

  let totalFE = 0
  let maxOP = -Infinity
  let minOP = Infinity
  let maxT = -Infinity
  let minT = Infinity
  let maxFE = -Infinity
  let minFE = Infinity
  let interfaceArea = 0

  const ny = orderParamField.length
  const nx = orderParamField[0]?.length ?? 0

  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      const eta = orderParamField[j][i]
      const T = temperatureField[j]?.[i] ?? 0
      const fe = freeEnergyField[j]?.[i] ?? 0

      totalFE += fe
      if (eta > maxOP) maxOP = eta
      if (eta < minOP) minOP = eta
      if (T > maxT) maxT = T
      if (T < minT) minT = T
      if (fe > maxFE) maxFE = fe
      if (fe < minFE) minFE = fe

      // Estimate interface area from gradient magnitude
      if (j > 0 && j < ny - 1 && i > 0 && i < nx - 1) {
        const deta_dx = (orderParamField[j][i + 1] - orderParamField[j][i - 1]) / (2 * config.grid.dx)
        const deta_dy = (orderParamField[j + 1][i] - orderParamField[j - 1][i]) / (2 * config.grid.dy)
        const gradMag = Math.sqrt(deta_dx * deta_dx + deta_dy * deta_dy)
        interfaceArea += gradMag * config.grid.dx * config.grid.dy
      }
    }
  }

  // Estimate grain count using connected component-like heuristic
  const grainCount = estimateGrainCount(orderParamField)

  // Average grain size = total area / grain count
  const totalArea = nx * config.grid.dx * ny * config.grid.dy
  const avgGrainSize = grainCount > 0 ? Math.sqrt(totalArea / grainCount) : 0

  return {
    totalFE,
    interfaceArea,
    grainCount,
    avgGrainSize,
    maxOrderParam: maxOP,
    minOrderParam: minOP,
    maxTemp: maxT,
    minTemp: minT,
    maxFE,
    minFE,
  }
})

const feChartData = computed(() => feHistory)
const gcChartData = computed(() => gcHistory)

const feChartMax = computed(() => {
  if (feHistory.length === 0) return 1
  return Math.max(...feHistory) * 1.05
})

const feChartMin = computed(() => {
  if (feHistory.length === 0) return 0
  return Math.min(...feHistory) * 0.95
})

const gcChartMax = computed(() => {
  if (gcHistory.length === 0) return 1
  return Math.max(...gcHistory) * 1.1
})

const gcChartMin = computed(() => {
  if (gcHistory.length === 0) return 0
  return Math.max(0, Math.min(...gcHistory) * 0.9)
})

// ============ Methods ============

function addNucleationSite() {
  config.initial_condition.nucleation_sites.push({ x: 0.5, y: 0.5, r: 0.05 })
}

function removeNucleationSite(idx: number) {
  config.initial_condition.nucleation_sites.splice(idx, 1)
}

function applyTemplate(tpl: typeof quickTemplates[0]) {
  switch (tpl.id) {
    case 'grain_growth':
      config.equation = 'cahn_hilliard'
      config.material.free_energy_type = 'double_well'
      config.material.free_energy_coefficients = { A: 1.0, B: 1.0, C: 0.0, m: 0.0 }
      config.material.mobility = 1.0
      config.material.gradient_energy_coeff = 0.5
      config.initial_condition.type = 'random'
      config.initial_condition.grain_count = 30
      config.initial_condition.noise_amplitude = 0.05
      config.thermal.thermal_coupling = 'none'
      config.dt = 0.01
      config.num_steps = 500
      config.grid.nx = 128
      config.grid.ny = 128
      config.grid.dx = 0.5
      config.grid.dy = 0.5
      break
    case 'phase_separation':
      config.equation = 'cahn_hilliard'
      config.material.free_energy_type = 'regular_solution'
      config.material.free_energy_coefficients = { A: 0.5, B: 1.0, C: 0.0, m: 0.0 }
      config.material.mobility = 1.0
      config.material.gradient_energy_coeff = 0.5
      config.initial_condition.type = 'random'
      config.initial_condition.grain_count = 10
      config.initial_condition.noise_amplitude = 0.1
      config.thermal.thermal_coupling = 'none'
      config.dt = 0.005
      config.num_steps = 1000
      config.grid.nx = 128
      config.grid.ny = 128
      config.grid.dx = 1.0
      config.grid.dy = 1.0
      break
    case 'dendrite':
      config.equation = 'karma_model'
      config.material.free_energy_type = 'double_well'
      config.material.free_energy_coefficients = { A: 1.0, B: 1.0, C: 0.0, m: 0.0 }
      config.material.mobility = 1.0
      config.material.gradient_energy_coeff = 0.25
      config.material.latent_heat = 2.0
      config.initial_condition.type = 'nucleation'
      config.initial_condition.grain_count = 1
      config.initial_condition.nucleation_sites = [{ x: 0.5, y: 0.5, r: 0.05 }]
      config.initial_condition.noise_amplitude = 0.02
      config.thermal.initial_temperature_K = 1000
      config.thermal.boundary_temperature_K = 800
      config.thermal.thermal_coupling = 'two_way'
      config.thermal.latent_heat_release = true
      config.dt = 0.005
      config.num_steps = 800
      config.grid.nx = 128
      config.grid.ny = 128
      config.grid.dx = 0.5
      config.grid.dy = 0.5
      break
    case 'precipitation':
      config.equation = 'allen_cahn'
      config.material.free_energy_type = 'landau'
      config.material.free_energy_coefficients = { A: -0.5, B: 0.5, C: 0.0, m: 0.0 }
      config.material.mobility = 1.0
      config.material.gradient_energy_coeff = 0.5
      config.initial_condition.type = 'nucleation'
      config.initial_condition.grain_count = 5
      config.initial_condition.nucleation_sites = [
        { x: 0.2, y: 0.3, r: 0.06 },
        { x: 0.7, y: 0.2, r: 0.05 },
        { x: 0.5, y: 0.7, r: 0.07 },
        { x: 0.3, y: 0.8, r: 0.04 },
        { x: 0.8, y: 0.6, r: 0.05 },
      ]
      config.initial_condition.noise_amplitude = 0.02
      config.thermal.thermal_coupling = 'one_way'
      config.dt = 0.01
      config.num_steps = 600
      config.grid.nx = 128
      config.grid.ny = 128
      config.grid.dx = 0.5
      config.grid.dy = 0.5
      break
  }
}

function resetAll() {
  simulating.value = false
  stopRequested.value = false
  currentStep.value = 0
  orderParamField = []
  temperatureField = []
  freeEnergyField = []
  grainBoundaryField = []
  feHistory.length = 0
  gcHistory.length = 0
  timeHistory.length = 0

  config.equation = 'cahn_hilliard'
  config.grid = { nx: 128, ny: 128, nz: 1, dx: 0.5, dy: 0.5, dz: 1.0 }
  config.material = {
    name: 'Default',
    free_energy_type: 'double_well',
    free_energy_coefficients: { A: 1.0, B: 1.0, C: 0.0, m: 0.0 },
    mobility: 1.0,
    gradient_energy_coeff: 0.5,
    latent_heat: 0.0,
    heat_capacity: 1.0,
    thermal_conductivity: 1.0,
    interface_width: 1.0,
    equilibrium_order_parameter: 1.0,
  }
  config.initial_condition = {
    type: 'random',
    params: {},
    grain_count: 20,
    nucleation_sites: [
      { x: 0.25, y: 0.25, r: 0.08 },
      { x: 0.75, y: 0.75, r: 0.08 },
      { x: 0.25, y: 0.75, r: 0.08 },
      { x: 0.75, y: 0.25, r: 0.08 },
    ],
    noise_amplitude: 0.05,
  }
  config.thermal = {
    initial_temperature_K: 800,
    boundary_temperature_K: 300,
    thermal_coupling: 'none',
    latent_heat_release: false,
    heat_source: [],
  }
  config.time_integration = 'explicit_euler'
  config.dt = 0.01
  config.num_steps = 500
  config.output_interval = 10
  config.adaptive_timestep = false
  config.convergence_tolerance = 1e-6
}

function stopSimulation() {
  stopRequested.value = true
}

// ============ Simulation Engine ============

/**
 * Initialize the order parameter field based on initial condition type.
 * Uses a simplified Allen-Cahn model: eta_t = eps^2 * laplacian(eta) + eta * (1 - eta^2)
 */
function initializeField(): number[][] {
  const { nx, ny } = config.grid
  const field: number[][] = []
  const ic = config.initial_condition

  for (let j = 0; j < ny; j++) {
    field[j] = new Array(nx)
    for (let i = 0; i < nx; i++) {
      const x = i / nx
      const y = j / ny

      switch (ic.type) {
        case 'random': {
          // Random noise with slight bias toward equilibrium values
          const noise = (Math.random() - 0.5) * 2 * ic.noise_amplitude
          const base = Math.random() > 0.5 ? 0.8 : -0.8
          field[j][i] = base + noise
          break
        }
        case 'nucleation': {
          // Nucleation sites: seed regions with eta=1, background eta=-1
          let eta = -0.8 + (Math.random() - 0.5) * ic.noise_amplitude
          for (const site of ic.nucleation_sites) {
            const dx = x - site.x
            const dy = y - site.y
            const dist = Math.sqrt(dx * dx + dy * dy)
            if (dist < site.r) {
              eta = 0.8 + (Math.random() - 0.5) * ic.noise_amplitude * 0.5
            } else if (dist < site.r * 1.5) {
              // Smooth transition at interface
              const t = (dist - site.r) / (site.r * 0.5)
              eta = 0.8 * (1 - t) + (-0.8) * t + (Math.random() - 0.5) * ic.noise_amplitude
            }
          }
          field[j][i] = eta
          break
        }
        case 'layered': {
          // Alternating horizontal layers
          const layerWidth = 1.0 / ic.grain_count
          const layerIdx = Math.floor(y / layerWidth)
          const sign = layerIdx % 2 === 0 ? 1 : -1
          field[j][i] = sign * 0.8 + (Math.random() - 0.5) * ic.noise_amplitude
          break
        }
        case 'circular': {
          // Circular domains
          const cx = 0.5
          const cy = 0.5
          const dist = Math.sqrt((x - cx) ** 2 + (y - cy) ** 2)
          const numRings = ic.grain_count
          const ringWidth = 0.5 / numRings
          const ringIdx = Math.floor(dist / ringWidth)
          const sign = ringIdx % 2 === 0 ? 1 : -1
          field[j][i] = sign * 0.8 + (Math.random() - 0.5) * ic.noise_amplitude
          break
        }
        case 'import':
        default: {
          // Default: random initialization
          field[j][i] = (Math.random() - 0.5) * 2 * 0.5
          break
        }
      }
    }
  }

  return field
}

/**
 * Initialize temperature field.
 * Simple uniform temperature with optional boundary cooling.
 */
function initializeTemperature(): number[][] {
  const { nx, ny } = config.grid
  const field: number[][] = []
  const T0 = config.thermal.initial_temperature_K
  const Tb = config.thermal.boundary_temperature_K

  for (let j = 0; j < ny; j++) {
    field[j] = new Array(nx)
    for (let i = 0; i < nx; i++) {
      // Linear gradient from center (T0) to boundary (Tb)
      const x = i / nx
      const y = j / ny
      const edgeDist = Math.min(x, 1 - x, y, 1 - y)
      const blend = Math.min(edgeDist * 4, 1.0) // Transition zone
      field[j][i] = Tb + (T0 - Tb) * blend
    }
  }

  return field
}

/**
 * Compute Laplacian with periodic boundary conditions.
 */
function laplacian(field: number[][], j: number, i: number, dx: number, dy: number): number {
  const ny = field.length
  const nx = field[0].length

  const jp = (j + 1) % ny
  const jm = (j - 1 + ny) % ny
  const ip = (i + 1) % nx
  const im = (i - 1 + nx) % nx

  const d2eta_dx2 = (field[j][ip] - 2 * field[j][i] + field[j][im]) / (dx * dx)
  const d2eta_dy2 = (field[jp][i] - 2 * field[j][i] + field[jm][i]) / (dy * dy)

  return d2eta_dx2 + d2eta_dy2
}

/**
 * Compute Laplacian with Neumann (zero-flux) boundary conditions.
 */
function laplacianNeumann(field: number[][], j: number, i: number, dx: number, dy: number): number {
  const ny = field.length
  const nx = field[0].length

  const jp = Math.min(j + 1, ny - 1)
  const jm = Math.max(j - 1, 0)
  const ip = Math.min(i + 1, nx - 1)
  const im = Math.max(i - 1, 0)

  const d2eta_dx2 = (field[j][ip] - 2 * field[j][i] + field[j][im]) / (dx * dx)
  const d2eta_dy2 = (field[jp][i] - 2 * field[j][i] + field[jm][i]) / (dy * dy)

  return d2eta_dx2 + d2eta_dy2
}

/**
 * Compute free energy density: f(eta) = -0.5*eta^2 + 0.25*eta^4
 */
function computeFreeEnergyDensity(eta: number): number {
  return -0.5 * eta * eta + 0.25 * eta * eta * eta * eta
}

/**
 * Estimate grain count using a simple thresholding + connected component approach.
 * Simplified: count regions where |eta| > threshold using flood fill.
 */
function estimateGrainCount(field: number[][]): number {
  if (field.length === 0) return 0

  const ny = field.length
  const nx = field[0].length
  const visited: boolean[][] = Array.from({ length: ny }, () => new Array(nx).fill(false))
  let count = 0

  const threshold = 0.3

  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      if (visited[j][i]) continue
      visited[j][i] = true

      // Only count positive-phase grains
      if (field[j][i] > threshold) {
        count++
        // Flood fill
        const stack: Array<[number, number]> = [[j, i]]
        while (stack.length > 0) {
          const [cj, ci] = stack.pop()!
          const neighbors: Array<[number, number]> = [
            [(cj - 1 + ny) % ny, ci],
            [(cj + 1) % ny, ci],
            [cj, (ci - 1 + nx) % nx],
            [cj, (ci + 1) % nx],
          ]
          for (const [nj, ni] of neighbors) {
            if (!visited[nj][ni] && field[nj][ni] > threshold) {
              visited[nj][ni] = true
              stack.push([nj, ni])
            }
          }
        }
      }
    }
  }

  return count
}

/**
 * Detect grain boundaries: where |grad(eta)| is large.
 */
function computeGrainBoundaries(field: number[][], dx: number, dy: number): number[][] {
  const ny = field.length
  const nx = field[0].length
  const boundaries: number[][] = Array.from({ length: ny }, () => new Array(nx).fill(0))

  for (let j = 1; j < ny - 1; j++) {
    for (let i = 1; i < nx - 1; i++) {
      const deta_dx = (field[j][i + 1] - field[j][i - 1]) / (2 * dx)
      const deta_dy = (field[j + 1][i] - field[j - 1][i]) / (2 * dy)
      const gradMag = Math.sqrt(deta_dx * deta_dx + deta_dy * deta_dy)
      boundaries[j][i] = Math.min(gradMag / 2.0, 1.0)
    }
  }

  return boundaries
}

/**
 * Perform one simulation step.
 * Allen-Cahn simplified: eta_t = eps^2 * laplacian(eta) + eta * (1 - eta^2)
 */
function simulationStep(eta: number[][], temp: number[][], dt: number): { eta: number[][]; temp: number[][] } {
  const { nx, ny, dx, dy } = config.grid
  const eps2 = config.material.gradient_energy_coeff
  const kappa = config.material.thermal_conductivity
  const coupling = config.thermal.thermal_coupling

  const newEta: number[][] = Array.from({ length: ny }, () => new Array(nx))
  const newTemp: number[][] = Array.from({ length: ny }, () => new Array(nx))

  // Temperature-dependent driving force for coupled simulations
  const T0 = config.thermal.initial_temperature_K
  const Tm = (T0 + config.thermal.boundary_temperature_K) / 2

  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      // Allen-Cahn: d_eta/dt = eps^2 * lap(eta) + eta*(1-eta^2)
      const lapEta = laplacian(eta, j, i, dx, dy)

      // Driving force
      let drivingForce = eta[j][i] * (1 - eta[j][i] * eta[j][i])

      // Temperature coupling: undercooling effect
      if (coupling !== 'none') {
        const undercooling = (Tm - temp[j][i]) / Tm
        drivingForce += undercooling * 0.5
      }

      // Karma model: anisotropic growth with preferential directions
      if (config.equation === 'karma_model') {
        const angle = Math.atan2(j - ny / 2, i - nx / 2)
        const anisotropy = 1.0 + 0.04 * Math.cos(4 * angle)
        newEta[j][i] = eta[j][i] + dt * (eps2 * anisotropy * lapEta + drivingForce)
      } else {
        newEta[j][i] = eta[j][i] + dt * (eps2 * lapEta + drivingForce)
      }

      // Clamp eta to prevent divergence
      newEta[j][i] = Math.max(-1.5, Math.min(1.5, newEta[j][i]))

      // Temperature evolution: simple diffusion
      if (coupling !== 'none') {
        const lapT = laplacianNeumann(temp, j, i, dx, dy)
        let dTdt = kappa * lapT

        // Latent heat release when eta changes
        if (config.thermal.latent_heat_release) {
          const dEta = newEta[j][i] - eta[j][i]
          dTdt += config.material.latent_heat * dEta / (config.material.heat_capacity * dt + 1e-10) * dt
        }

        newTemp[j][i] = temp[j][i] + dt * dTdt
      } else {
        newTemp[j][i] = temp[j][i]
      }
    }
  }

  return { eta: newEta, temp: newTemp }
}

/**
 * Run the full simulation with animation.
 */
async function runSimulation() {
  simulating.value = true
  stopRequested.value = false
  currentStep.value = 0

  // Clear history
  feHistory.length = 0
  gcHistory.length = 0
  timeHistory.length = 0

  // Initialize fields
  orderParamField = initializeField()
  temperatureField = initializeTemperature()

  const { nx, ny } = config.grid
  const dt = config.dt
  const totalSteps = config.num_steps
  const outputInterval = config.output_interval

  // Compute initial free energy
  freeEnergyField = Array.from({ length: ny }, (_, j) =>
    Array.from({ length: nx }, (_, i) => computeFreeEnergyDensity(orderParamField[j][i]))
  )

  // Render initial state
  await nextTick()
  renderAllCanvases()

  // Record initial history
  let totalFE = 0
  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      totalFE += freeEnergyField[j][i]
    }
  }
  feHistory.push(totalFE)
  gcHistory.push(estimateGrainCount(orderParamField))
  timeHistory.push(0)

  // Simulation loop with yielding to UI
  const batchSize = Math.max(1, Math.floor(outputInterval / 2))

  for (let step = 1; step <= totalSteps; step++) {
    if (stopRequested.value) break

    const result = simulationStep(orderParamField, temperatureField, dt)
    orderParamField = result.eta
    temperatureField = result.temp

    currentStep.value = step

    // Update free energy field
    if (step % Math.max(1, Math.floor(outputInterval / 5)) === 0) {
      for (let j = 0; j < ny; j++) {
        for (let i = 0; i < nx; i++) {
          freeEnergyField[j][i] = computeFreeEnergyDensity(orderParamField[j][i])
        }
      }
    }

    // Record history at output intervals
    if (step % outputInterval === 0) {
      let tfe = 0
      for (let j = 0; j < ny; j++) {
        for (let i = 0; i < nx; i++) {
          tfe += freeEnergyField[j][i]
        }
      }
      feHistory.push(tfe)
      gcHistory.push(estimateGrainCount(orderParamField))
      timeHistory.push(step * dt)

      // Render canvases
      grainBoundaryField = computeGrainBoundaries(orderParamField, config.grid.dx, config.grid.dy)
      await nextTick()
      renderAllCanvases()

      // Yield to UI thread
      await new Promise(resolve => setTimeout(resolve, 0))
    }
  }

  // Final render
  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      freeEnergyField[j][i] = computeFreeEnergyDensity(orderParamField[j][i])
    }
  }
  grainBoundaryField = computeGrainBoundaries(orderParamField, config.grid.dx, config.grid.dy)

  await nextTick()
  renderAllCanvases()

  simulating.value = false
}

// ============ Canvas Rendering ============

/**
 * Color map: blue -> purple -> red for order parameter field.
 */
function orderParamColor(value: number): [number, number, number] {
  // Normalize from [-1, 1] to [0, 1]
  const t = Math.max(0, Math.min(1, (value + 1) / 2))

  // Blue -> Purple -> Pink -> Red
  let r: number, g: number, b: number

  if (t < 0.25) {
    const s = t / 0.25
    r = Math.round(37 * (1 - s) + 124 * s)
    g = Math.round(99 * (1 - s) + 58 * s)
    b = Math.round(235 * (1 - s) + 237 * s)
  } else if (t < 0.5) {
    const s = (t - 0.25) / 0.25
    r = Math.round(124 * (1 - s) + 168 * s)
    g = Math.round(58 * (1 - s) + 85 * s)
    b = Math.round(237 * (1 - s) + 247 * s)
  } else if (t < 0.75) {
    const s = (t - 0.5) / 0.25
    r = Math.round(168 * (1 - s) + 236 * s)
    g = Math.round(85 * (1 - s) + 72 * s)
    b = Math.round(247 * (1 - s) + 153 * s)
  } else {
    const s = (t - 0.75) / 0.25
    r = Math.round(236 * (1 - s) + 239 * s)
    g = Math.round(72 * (1 - s) + 68 * s)
    b = Math.round(153 * (1 - s) + 68 * s)
  }

  return [r, g, b]
}

/**
 * Color map: cold -> warm for temperature field.
 */
function temperatureColor(value: number, minT: number, maxT: number): [number, number, number] {
  const range = maxT - minT
  const t = range > 0 ? Math.max(0, Math.min(1, (value - minT) / range)) : 0.5

  let r: number, g: number, b: number

  if (t < 0.25) {
    const s = t / 0.25
    r = Math.round(59 * (1 - s) + 6 * s)
    g = Math.round(130 * (1 - s) + 182 * s)
    b = Math.round(246 * (1 - s) + 212 * s)
  } else if (t < 0.5) {
    const s = (t - 0.25) / 0.25
    r = Math.round(6 * (1 - s) + 34 * s)
    g = Math.round(182 * (1 - s) + 197 * s)
    b = Math.round(212 * (1 - s) + 94 * s)
  } else if (t < 0.75) {
    const s = (t - 0.5) / 0.25
    r = Math.round(34 * (1 - s) + 234 * s)
    g = Math.round(197 * (1 - s) + 179 * s)
    b = Math.round(94 * (1 - s) + 8 * s)
  } else {
    const s = (t - 0.75) / 0.25
    r = Math.round(234 * (1 - s) + 239 * s)
    g = Math.round(179 * (1 - s) + 68 * s)
    b = Math.round(8 * (1 - s) + 68 * s)
  }

  return [r, g, b]
}

/**
 * Color map: dark -> light for free energy density.
 */
function freeEnergyColor(value: number, minFE: number, maxFE: number): [number, number, number] {
  const range = maxFE - minFE
  const t = range > 0 ? Math.max(0, Math.min(1, (value - minFE) / range)) : 0.5

  // Dark slate -> light gray
  const r = Math.round(30 + t * 196)
  const g = Math.round(41 + t * 187)
  const b = Math.round(59 + t * 173)

  return [r, g, b]
}

/**
 * Render a 2D field to a canvas using the given color map function.
 */
function renderFieldToCanvas(
  canvas: HTMLCanvasElement,
  field: number[][],
  colorFn: (val: number, min: number, max: number) => [number, number, number],
  minVal: number,
  maxVal: number
) {
  const ny = field.length
  const nx = field[0]?.length ?? 0
  if (nx === 0 || ny === 0) return

  const container = canvas.parentElement
  if (!container) return

  const displaySize = Math.min(container.clientWidth, 400)
  const dpr = window.devicePixelRatio || 1

  canvas.width = displaySize * dpr
  canvas.height = displaySize * dpr
  canvas.style.width = displaySize + 'px'
  canvas.style.height = displaySize + 'px'

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const imageData = ctx.createImageData(canvas.width, canvas.height)
  const data = imageData.data

  const scaleX = canvas.width / nx
  const scaleY = canvas.height / ny

  for (let py = 0; py < canvas.height; py++) {
    const fj = Math.min(Math.floor(py / scaleY), ny - 1)
    for (let px = 0; px < canvas.width; px++) {
      const fi = Math.min(Math.floor(px / scaleX), nx - 1)
      const val = field[fj][fi]
      const [r, g, b] = colorFn(val, minVal, maxVal)

      const idx = (py * canvas.width + px) * 4
      data[idx] = r
      data[idx + 1] = g
      data[idx + 2] = b
      data[idx + 3] = 255
    }
  }

  ctx.putImageData(imageData, 0, 0)
}

function renderAllCanvases() {
  if (orderParamField.length === 0) return

  const ny = orderParamField.length
  const nx = orderParamField[0]?.length ?? 0

  // Find ranges
  let minOP = Infinity, maxOP = -Infinity
  let minT = Infinity, maxT = -Infinity
  let minFE = Infinity, maxFE = -Infinity

  for (let j = 0; j < ny; j++) {
    for (let i = 0; i < nx; i++) {
      const eta = orderParamField[j][i]
      const T = temperatureField[j]?.[i] ?? 0
      const fe = freeEnergyField[j]?.[i] ?? 0

      if (eta < minOP) minOP = eta
      if (eta > maxOP) maxOP = eta
      if (T < minT) minT = T
      if (T > maxT) maxT = T
      if (fe < minFE) minFE = fe
      if (fe > maxFE) maxFE = fe
    }
  }

  if (orderParamCanvas.value) {
    renderFieldToCanvas(orderParamCanvas.value, orderParamField, orderParamColor, -1, 1)
  }
  if (temperatureCanvas.value) {
    renderFieldToCanvas(temperatureCanvas.value, temperatureField, temperatureColor, minT, maxT)
  }
  if (freeEnergyCanvas.value) {
    renderFieldToCanvas(freeEnergyCanvas.value, freeEnergyField, freeEnergyColor, minFE, maxFE)
  }
}

// ============ Lifecycle ============

onMounted(() => {
  // Nothing to do on mount
})
</script>

<style scoped>
.panel-section {
  margin-bottom: 4px;
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
}

.input {
  width: 100%;
  padding: 8px;
  background: var(--bg-base);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 12px;
  transition: border-color var(--duration-fast) var(--ease-out);
}

.input:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px var(--primary-glow);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-weight: 500;
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--primary);
  color: #fff;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
}

.btn-ghost:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.result-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
}

.result-label {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 4px;
}

.result-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}
</style>
