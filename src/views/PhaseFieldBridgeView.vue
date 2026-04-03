<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">相场尺度桥接</h2>
        <p class="text-sm" style="color: var(--text-muted)">MD → 相场 → FE，多尺度数据流，粗粒化/均匀化</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Template Buttons -->
        <button
          v-for="tpl in templates"
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
      </div>
    </div>

    <!-- Tab Switch -->
    <div class="flex border-b" style="border-color: var(--border-subtle); background: var(--bg-surface)">
      <button
        @click="activeTab = 'md_to_pf'"
        class="px-6 py-2.5 text-sm font-medium transition border-b-2"
        :style="activeTab === 'md_to_pf'
          ? 'color: var(--primary); border-color: var(--primary)'
          : 'color: var(--text-muted); border-color: transparent'"
      >
        MD → 相场
      </button>
      <button
        @click="activeTab = 'pf_to_fe'"
        class="px-6 py-2.5 text-sm font-medium transition border-b-2"
        :style="activeTab === 'pf_to_fe'
          ? 'color: var(--primary); border-color: var(--primary)'
          : 'color: var(--text-muted); border-color: transparent'"
      >
        相场 → FE
      </button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- ==================== MD → Phase Field Panel ==================== -->
      <template v-if="activeTab === 'md_to_pf'">
        <div class="flex-1 flex overflow-hidden">
          <!-- Left: Configuration -->
          <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <!-- Coarse Graining Method -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
                粗粒化方法
              </h4>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="m in cgMethods"
                  :key="m.value"
                  @click="mdPfConfig.coarse_graining_method = m.value"
                  :class="['px-2 py-2 rounded text-xs text-left transition border', mdPfConfig.coarse_graining_method === m.value ? 'text-white' : '']"
                  :style="mdPfConfig.coarse_graining_method === m.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  <div class="font-medium">{{ m.label }}</div>
                  <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
                </button>
              </div>
            </div>

            <!-- Target Grid -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
                目标网格
              </h4>
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <label class="label">nx</label>
                  <input v-model.number="mdPfConfig.target_grid.nx" type="number" step="10" min="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">ny</label>
                  <input v-model.number="mdPfConfig.target_grid.ny" type="number" step="10" min="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">nz</label>
                  <input v-model.number="mdPfConfig.target_grid.nz" type="number" step="10" min="10" class="input w-full text-xs" />
                </div>
              </div>
            </div>

            <!-- Smoothing & Mapping -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
                映射参数
              </h4>
              <div class="space-y-2">
                <div>
                  <label class="label">平滑长度 (Å)</label>
                  <input v-model.number="mdPfConfig.smoothing_length_A" type="number" step="0.5" min="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">序参量映射</label>
                  <select v-model="mdPfConfig.order_parameter_mapping" class="input w-full text-xs">
                    <option value="local_density">局部密度</option>
                    <option value="crystal_structure">晶体结构</option>
                    <option value="custom">自定义</option>
                  </select>
                </div>
                <div>
                  <label class="label">温度映射</label>
                  <select v-model="mdPfConfig.temperature_mapping" class="input w-full text-xs">
                    <option value="kinetic_energy">动能</option>
                    <option value="local_pressure">局部压力</option>
                  </select>
                </div>
                <div>
                  <label class="label">界面检测阈值</label>
                  <input v-model.number="mdPfConfig.interface_detection_threshold" type="number" step="0.05" min="0" max="1" class="input w-full text-xs" />
                </div>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="space-y-2">
              <button @click="runMdToPf" :disabled="runningMdPf" class="btn btn-primary text-xs w-full">
                <span v-if="runningMdPf" class="mr-1 animate-spin">&#10227;</span>
                {{ runningMdPf ? '粗粒化中...' : '运行粗粒化' }}
              </button>
              <button @click="validateMdPf" :disabled="!mdPfResult" class="btn btn-ghost text-xs w-full">
                验证质量
              </button>
              <button @click="exportMdPf" :disabled="!mdPfResult" class="btn btn-ghost text-xs w-full">
                导出数据
              </button>
            </div>
          </div>

          <!-- Right: Results -->
          <div class="flex-1 flex flex-col overflow-hidden">
            <div v-if="!mdPfResult" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
              <div class="text-center">
                <div class="text-4xl mb-2">&#9670;</div>
                <div class="text-sm">配置参数后运行 MD → 相场粗粒化</div>
              </div>
            </div>
            <template v-else>
              <div class="flex-1 overflow-y-auto p-4 space-y-4">
                <!-- Field Previews -->
                <div class="grid grid-cols-3 gap-4">
                  <div class="result-card">
                    <span class="result-label">序参量场</span>
                    <canvas ref="orderParamCanvas" width="200" height="200" class="w-full mt-2 rounded" style="background: var(--bg-base)"></canvas>
                  </div>
                  <div class="result-card">
                    <span class="result-label">温度场</span>
                    <canvas ref="tempFieldCanvas" width="200" height="200" class="w-full mt-2 rounded" style="background: var(--bg-base)"></canvas>
                  </div>
                  <div class="result-card">
                    <span class="result-label">晶粒结构</span>
                    <canvas ref="grainCanvas" width="200" height="200" class="w-full mt-2 rounded" style="background: var(--bg-base)"></canvas>
                  </div>
                </div>

                <!-- Mapping Quality -->
                <div class="result-card">
                  <span class="result-label">映射质量指标</span>
                  <div class="space-y-3 mt-3">
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span style="color: var(--text-secondary)">空间相关性</span>
                        <span class="font-mono" style="color: var(--text-primary)">{{ (mdPfResult.mapping_quality.spatial_correlation * 100).toFixed(1) }}%</span>
                      </div>
                      <div class="w-full h-2.5 rounded-full" style="background: var(--bg-elevated)">
                        <div class="h-2.5 rounded-full transition-all" :style="{ background: qualityColor(mdPfResult.mapping_quality.spatial_correlation), width: (mdPfResult.mapping_quality.spatial_correlation * 100) + '%' }"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span style="color: var(--text-secondary)">能量守恒</span>
                        <span class="font-mono" style="color: var(--text-primary)">{{ (mdPfResult.mapping_quality.energy_conservation * 100).toFixed(1) }}%</span>
                      </div>
                      <div class="w-full h-2.5 rounded-full" style="background: var(--bg-elevated)">
                        <div class="h-2.5 rounded-full transition-all" :style="{ background: qualityColor(mdPfResult.mapping_quality.energy_conservation), width: (mdPfResult.mapping_quality.energy_conservation * 100) + '%' }"></div>
                      </div>
                    </div>
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span style="color: var(--text-secondary)">界面锐度</span>
                        <span class="font-mono" style="color: var(--text-primary)">{{ (mdPfResult.mapping_quality.interface_sharpness * 100).toFixed(1) }}%</span>
                      </div>
                      <div class="w-full h-2.5 rounded-full" style="background: var(--bg-elevated)">
                        <div class="h-2.5 rounded-full transition-all" :style="{ background: qualityColor(mdPfResult.mapping_quality.interface_sharpness), width: (mdPfResult.mapping_quality.interface_sharpness * 100) + '%' }"></div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Coarse Graining Stats -->
                <div class="result-card">
                  <span class="result-label">粗粒化统计</span>
                  <div class="grid grid-cols-3 gap-3 mt-3">
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">源原子数</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ mdPfResult.coarse_graining_stats.source_atoms.toLocaleString() }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">目标网格点</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ mdPfResult.coarse_graining_stats.target_grid_points.toLocaleString() }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">降维比</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--primary)">{{ mdPfResult.coarse_graining_stats.reduction_ratio.toFixed(1) }}x</div>
                    </div>
                  </div>
                </div>

                <!-- Quality Validation Result -->
                <div v-if="qualityResult" class="result-card">
                  <span class="result-label">质量验证结果</span>
                  <div class="mt-2 space-y-2">
                    <div class="flex items-center justify-between">
                      <span class="text-xs" style="color: var(--text-secondary)">综合质量评分</span>
                      <span class="text-sm font-mono font-semibold" :style="{ color: qualityColor(qualityResult.quality_score) }">{{ (qualityResult.quality_score * 100).toFixed(1) }}%</span>
                    </div>
                    <div v-if="qualityResult.issues.length > 0" class="space-y-1">
                      <div v-for="(issue, idx) in qualityResult.issues" :key="idx" class="text-xs px-2 py-1 rounded" style="background: var(--bg-elevated); color: var(--accent-yellow)">
                        {{ issue }}
                      </div>
                    </div>
                    <div v-else class="text-xs" style="color: var(--accent-green)">所有质量检查通过</div>
                  </div>
                </div>
              </div>
            </template>
          </div>
        </div>
      </template>

      <!-- ==================== Phase Field → FE Panel ==================== -->
      <template v-else>
        <div class="flex-1 flex overflow-hidden">
          <!-- Left: Configuration -->
          <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <!-- Homogenization Method -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
                均匀化方法
              </h4>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="m in homMethods"
                  :key="m.value"
                  @click="pfFeConfig.homogenization_method = m.value"
                  :class="['px-2 py-2 rounded text-xs text-left transition border', pfFeConfig.homogenization_method === m.value ? 'text-white' : '']"
                  :style="pfFeConfig.homogenization_method === m.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  <div class="font-medium">{{ m.label }}</div>
                  <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
                </button>
              </div>
            </div>

            <!-- Strain Range -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
                应变范围
              </h4>
              <div class="space-y-2">
                <div class="grid grid-cols-2 gap-2">
                  <div>
                    <label class="label">最小应变</label>
                    <input v-model.number="pfFeConfig.strain_range.min" type="number" step="0.001" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">最大应变</label>
                    <input v-model.number="pfFeConfig.strain_range.max" type="number" step="0.01" class="input w-full text-xs" />
                  </div>
                </div>
                <div>
                  <label class="label">步数</label>
                  <input v-model.number="pfFeConfig.strain_range.steps" type="number" step="5" min="5" max="200" class="input w-full text-xs" />
                </div>
              </div>
            </div>

            <!-- Temperature & Output -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
                输出设置
              </h4>
              <div class="space-y-2">
                <div>
                  <label class="label">温度 (K)</label>
                  <input v-model.number="pfFeConfig.temperature" type="number" step="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">输出类型</label>
                  <select v-model="pfFeConfig.output_type" class="input w-full text-xs">
                    <option value="stress_strain_curve">应力-应变曲线</option>
                    <option value="elastic_tensor">弹性张量</option>
                    <option value="yield_surface">屈服面</option>
                  </select>
                </div>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="space-y-2">
              <button @click="runPfToFe" :disabled="runningPfFe" class="btn btn-primary text-xs w-full">
                <span v-if="runningPfFe" class="mr-1 animate-spin">&#10227;</span>
                {{ runningPfFe ? '均匀化中...' : '运行均匀化' }}
              </button>
              <button @click="validatePfFe" :disabled="!pfFeResult" class="btn btn-ghost text-xs w-full">
                验证质量
              </button>
              <button @click="exportPfFe" :disabled="!pfFeResult" class="btn btn-ghost text-xs w-full">
                导出数据
              </button>
            </div>
          </div>

          <!-- Right: Results -->
          <div class="flex-1 flex flex-col overflow-hidden">
            <div v-if="!pfFeResult" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
              <div class="text-center">
                <div class="text-4xl mb-2">&#9670;</div>
                <div class="text-sm">配置参数后运行 相场 → FE 均匀化</div>
              </div>
            </div>
            <template v-else>
              <div class="flex-1 overflow-y-auto p-4 space-y-4">
                <!-- Equivalent Properties Cards -->
                <div class="result-card">
                  <span class="result-label">等效力学性能</span>
                  <div class="grid grid-cols-4 gap-3 mt-3">
                    <div v-for="prop in equivProps" :key="prop.key" class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">{{ prop.label }}</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ prop.value }}</div>
                      <div class="text-[10px]" style="color: var(--text-muted)">{{ prop.unit }}</div>
                    </div>
                  </div>
                </div>

                <!-- 6x6 Elastic Tensor -->
                <div class="result-card">
                  <span class="result-label">6×6 弹性张量 (GPa)</span>
                  <div class="mt-2 overflow-x-auto">
                    <table class="matrix-table">
                      <thead>
                        <tr>
                          <th class="matrix-header"></th>
                          <th v-for="j in 6" :key="j" class="matrix-header">{{ strainLabels[j - 1] }}</th>
                        </tr>
                      </thead>
                      <tbody>
                        <tr v-for="i in 6" :key="i">
                          <td class="matrix-header">{{ strainLabels[i - 1] }}</td>
                          <td
                            v-for="j in 6"
                            :key="j"
                            class="matrix-cell"
                            :class="{ 'matrix-diagonal': i === j }"
                          >
                            {{ formatTensorCell(pfFeResult.effective_elastic_tensor[i - 1][j - 1]) }}
                          </td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
                </div>

                <!-- Stress-Strain Curve & Convergence -->
                <div class="grid grid-cols-2 gap-4">
                  <div class="result-card">
                    <span class="result-label">应力-应变曲线</span>
                    <div class="mt-2">
                      <svg viewBox="0 0 300 200" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                        <!-- Grid -->
                        <line v-for="i in 5" :key="'h'+i" x1="40" y1="20 + i * 32" x2="290" y2="20 + i * 32" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />
                        <line v-for="i in 5" :key="'v'+i" x1="40 + i * 50" y1="20" x2="40 + i * 50" y2="180" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />
                        <!-- Y axis -->
                        <text x="35" y="24" fill="var(--text-muted)" font-size="8" text-anchor="end">{{ maxStressLabel }}</text>
                        <text x="35" y="104" fill="var(--text-muted)" font-size="8" text-anchor="end">{{ (maxStress / 2).toFixed(0) }}</text>
                        <text x="35" y="184" fill="var(--text-muted)" font-size="8" text-anchor="end">0</text>
                        <!-- X axis -->
                        <text x="40" y="195" fill="var(--text-muted)" font-size="8" text-anchor="middle">0</text>
                        <text x="165" y="195" fill="var(--text-muted)" font-size="8" text-anchor="middle">{{ (pfFeConfig.strain_range.max / 2).toFixed(3) }}</text>
                        <text x="290" y="195" fill="var(--text-muted)" font-size="8" text-anchor="middle">{{ pfFeConfig.strain_range.max.toFixed(3) }}</text>
                        <!-- Axis labels -->
                        <text x="165" y="12" fill="var(--text-secondary)" font-size="9" text-anchor="middle">Stress-Strain Curve</text>
                        <text x="8" y="104" fill="var(--text-muted)" font-size="8" text-anchor="middle" transform="rotate(-90, 8, 104)">Stress (MPa)</text>
                        <text x="165" y="208" fill="var(--text-muted)" font-size="8" text-anchor="middle">Strain</text>
                        <!-- Curve -->
                        <polyline :points="ssCurvePoints" fill="none" stroke="var(--primary)" stroke-width="1.5" />
                        <!-- Yield point -->
                        <circle v-if="yieldPoint" :cx="yieldPoint.x" :cy="yieldPoint.y" r="3" fill="var(--accent-yellow)" />
                        <text v-if="yieldPoint" :x="yieldPoint.x + 6" :y="yieldPoint.y - 6" fill="var(--accent-yellow)" font-size="7">yield</text>
                      </svg>
                    </div>
                  </div>

                  <div class="result-card">
                    <span class="result-label">收敛曲线</span>
                    <div class="mt-2">
                      <svg viewBox="0 0 300 200" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                        <line v-for="i in 5" :key="'h'+i" x1="40" y1="20 + i * 32" x2="290" y2="20 + i * 32" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />
                        <line v-for="i in 5" :key="'v'+i" x1="40 + i * 50" y1="20" x2="40 + i * 50" y2="180" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />
                        <text x="35" y="24" fill="var(--text-muted)" font-size="8" text-anchor="end">1e0</text>
                        <text x="35" y="104" fill="var(--text-muted)" font-size="8" text-anchor="end">1e-4</text>
                        <text x="35" y="184" fill="var(--text-muted)" font-size="8" text-anchor="end">1e-8</text>
                        <text x="165" y="12" fill="var(--text-secondary)" font-size="9" text-anchor="middle">Convergence History</text>
                        <polyline :points="convPoints" fill="none" stroke="var(--primary)" stroke-width="1.5" />
                        <line x1="40" y1="160" x2="290" y2="160" stroke="var(--accent-red)" stroke-width="0.8" stroke-dasharray="4,3" />
                        <text x="292" y="163" fill="var(--accent-red)" font-size="7">tol</text>
                      </svg>
                    </div>
                  </div>
                </div>

                <!-- Yield Surface & Phase Fraction -->
                <div class="grid grid-cols-2 gap-4">
                  <div class="result-card">
                    <span class="result-label">屈服面 (pi 平面投影)</span>
                    <canvas ref="yieldSurfaceCanvas" width="280" height="240" class="w-full mt-2 rounded" style="background: var(--bg-base)"></canvas>
                  </div>
                  <div class="result-card">
                    <span class="result-label">相分数</span>
                    <div class="mt-3 space-y-3">
                      <div>
                        <div class="flex justify-between text-xs mb-1">
                          <span style="color: var(--text-secondary)">相 1 (基体)</span>
                          <span class="font-mono" style="color: var(--text-primary)">{{ ((1 - pfFeResult.phase_fraction) * 100).toFixed(1) }}%</span>
                        </div>
                        <div class="w-full h-3 rounded-full" style="background: var(--bg-elevated)">
                          <div class="h-3 rounded-full" style="background: var(--primary)" :style="{ width: ((1 - pfFeResult.phase_fraction) * 100) + '%' }"></div>
                        </div>
                      </div>
                      <div>
                        <div class="flex justify-between text-xs mb-1">
                          <span style="color: var(--text-secondary)">相 2 (增强相)</span>
                          <span class="font-mono" style="color: var(--text-primary)">{{ (pfFeResult.phase_fraction * 100).toFixed(1) }}%</span>
                        </div>
                        <div class="w-full h-3 rounded-full" style="background: var(--bg-elevated)">
                          <div class="h-3 rounded-full" style="background: var(--accent-green)" :style="{ width: (pfFeResult.phase_fraction * 100) + '%' }"></div>
                        </div>
                      </div>
                    </div>
                    <div class="mt-4">
                      <span class="result-label">硬化曲线</span>
                      <div class="mt-2">
                        <svg viewBox="0 0 260 120" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                          <line v-for="i in 4" :key="'h'+i" x1="30" y1="10 + i * 25" x2="250" y2="10 + i * 25" stroke="rgba(255,255,255,0.06)" stroke-width="0.5" />
                          <polyline :points="hardeningPoints" fill="none" stroke="var(--accent-green)" stroke-width="1.5" />
                          <text x="140" y="8" fill="var(--text-secondary)" font-size="8" text-anchor="middle">Hardening Curve</text>
                        </svg>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </template>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick, watch } from 'vue'
import type {
  CoarseGrainingMethod,
  OrderParameterMapping,
  TemperatureMapping,
  HomogenizationMethod,
  OutputType,
  MdToPfConfig,
  MdToPfResult,
  PfToFeConfig,
  PfToFeResult,
  BridgeTemplate,
} from '../api/phaseFieldBridge'

// ============ Constants ============

const strainLabels = ['11', '22', '33', '12', '23', '13']

const cgMethods = [
  { value: 'atomic_density' as CoarseGrainingMethod, label: '原子密度', desc: '局部原子密度' },
  { value: 'bond_order' as CoarseGrainingMethod, label: '键序参量', desc: 'Steinhardt OP' },
  { value: 'centro_symmetry' as CoarseGrainingMethod, label: '中心对称性', desc: 'CSP 参数' },
  { value: 'voronoi' as CoarseGrainingMethod, label: 'Voronoi', desc: 'Voronoi 分析' },
  { value: 'ml_interpolation' as CoarseGrainingMethod, label: 'ML 插值', desc: '机器学习' },
]

const homMethods = [
  { value: 'voigt' as HomogenizationMethod, label: 'Voigt', desc: '等应变假设' },
  { value: 'reuss' as HomogenizationMethod, label: 'Reuss', desc: '等应力假设' },
  { value: 'mori_tanaka' as HomogenizationMethod, label: 'Mori-Tanaka', desc: '稀疏近似' },
  { value: 'self_consistent' as HomogenizationMethod, label: '自洽', desc: 'Self-Consistent' },
  { value: 'numerical' as HomogenizationMethod, label: '数值', desc: 'FE² 数值' },
]

const templates: BridgeTemplate[] = [
  { id: 'nano_polycrystal', name: '纳米多晶→相场', name_en: 'Nano Polycrystal → Phase Field', category: 'md_to_pf', description: '纳米多晶体 MD 轨迹到相场序参量', direction: 'md_to_pf', typical_application: '纳米晶体材料' },
  { id: 'dual_phase_steel', name: '双相钢→FE', name_en: 'Dual Phase Steel → FE', category: 'pf_to_fe', description: '双相钢相场到有限元均匀化', direction: 'pf_to_fe', typical_application: '汽车用钢' },
  { id: 'composite_homo', name: '复合材料均匀化', name_en: 'Composite Homogenization', category: 'pf_to_fe', description: '复合材料相场均匀化到 FE', direction: 'pf_to_fe', typical_application: '航空复合材料' },
  { id: 'crack_tip_fe', name: '裂纹尖端→FE', name_en: 'Crack Tip → FE', category: 'md_to_pf', description: '裂纹尖端 MD 到相场再到 FE', direction: 'md_to_pf', typical_application: '断裂力学' },
]

// ============ State ============

const activeTab = ref<'md_to_pf' | 'pf_to_fe'>('md_to_pf')
const runningMdPf = ref(false)
const runningPfFe = ref(false)
const mdPfResult = ref<MdToPfResult | null>(null)
const pfFeResult = ref<PfToFeResult | null>(null)
const qualityResult = ref<{ quality_score: number; issues: string[] } | null>(null)

const orderParamCanvas = ref<HTMLCanvasElement | null>(null)
const tempFieldCanvas = ref<HTMLCanvasElement | null>(null)
const grainCanvas = ref<HTMLCanvasElement | null>(null)
const yieldSurfaceCanvas = ref<HTMLCanvasElement | null>(null)

const mdPfConfig = reactive<MdToPfConfig>({
  trajectory_data: {},
  coarse_graining_method: 'voronoi',
  target_grid: { nx: 64, ny: 64, nz: 1 },
  smoothing_length_A: 3.0,
  order_parameter_mapping: 'crystal_structure',
  temperature_mapping: 'kinetic_energy',
  interface_detection_threshold: 0.5,
})

const pfFeConfig = reactive<PfToFeConfig>({
  phase_field_data: {},
  grid_size: { nx: 64, ny: 64, nz: 1 },
  homogenization_method: 'mori_tanaka',
  strain_range: { min: 0, max: 0.15, steps: 50 },
  temperature: 300,
  output_type: 'stress_strain_curve',
})

// ============ Voronoi Seed Generation ============

function generateVoronoiSeeds(count: number, size: number): Array<{ x: number; y: number }> {
  const seeds: Array<{ x: number; y: number }> = []
  for (let i = 0; i < count; i++) {
    seeds.push({ x: Math.random() * size, y: Math.random() * size })
  }
  return seeds
}

function closestSeed(px: number, py: number, seeds: Array<{ x: number; y: number }>): number {
  let minDist = Infinity
  let closest = 0
  for (let i = 0; i < seeds.length; i++) {
    const dx = px - seeds[i].x
    const dy = py - seeds[i].y
    const d = dx * dx + dy * dy
    if (d < minDist) { minDist = d; closest = i }
  }
  return closest
}

// ============ Mock Data Generation ============

function generateMockMdPfResult(): MdToPfResult {
  const nx = mdPfConfig.target_grid.nx
  const ny = mdPfConfig.target_grid.ny
  const nz = mdPfConfig.target_grid.nz
  const totalPoints = nx * ny * nz
  const sourceAtoms = Math.round(totalPoints * 120)

  const numGrains = 8 + Math.floor(Math.random() * 8)
  const seeds = generateVoronoiSeeds(numGrains, nx)

  // Order parameter field (2D slice for visualization)
  const orderField: number[][] = []
  const grainField: number[][] = []
  const tempField: number[][] = []

  for (let iy = 0; iy < ny; iy++) {
    const orderRow: number[] = []
    const grainRow: number[] = []
    const tempRow: number[] = []
    for (let ix = 0; ix < nx; ix++) {
      const grainId = closestSeed(ix, iy, seeds)

      // Distance to nearest grain boundary
      let minBoundaryDist = Infinity
      for (let s = 0; s < seeds.length; s++) {
        if (s === grainId) continue
        const dx = seeds[s].x - seeds[grainId].x
        const dy = seeds[s].y - seeds[grainId].y
        const mx = (seeds[s].x + seeds[grainId].x) / 2
        const my = (seeds[s].y + seeds[grainId].y) / 2
        const distToBoundary = Math.sqrt((ix - mx) ** 2 + (iy - my) ** 2)
        if (distToBoundary < minBoundaryDist) minBoundaryDist = distToBoundary
      }

      // Order parameter: 1 inside grain, 0 at boundary
      const op = 1.0 / (1.0 + Math.exp(-(minBoundaryDist - mdPfConfig.interface_detection_threshold * 5) / 2))
      orderRow.push(op)
      grainRow.push(grainId)

      // Temperature field: base 300K with some variation
      const temp = 300 + 20 * Math.sin(ix * 0.1) * Math.cos(iy * 0.1) + (Math.random() - 0.5) * 10
      tempRow.push(temp)
    }
    orderField.push(orderRow)
    grainField.push(grainRow)
    tempField.push(tempRow)
  }

  return {
    success: true,
    order_parameter_field: orderField,
    temperature_field: tempField,
    grain_structure: grainField,
    mapping_quality: {
      spatial_correlation: 0.85 + Math.random() * 0.12,
      energy_conservation: 0.90 + Math.random() * 0.08,
      interface_sharpness: 0.78 + Math.random() * 0.15,
    },
    coarse_graining_stats: {
      source_atoms: sourceAtoms,
      target_grid_points: totalPoints,
      reduction_ratio: sourceAtoms / totalPoints,
    },
  }
}

function generateMockPfFeResult(): PfToFeResult {
  const phaseFraction = 0.25 + Math.random() * 0.2
  const E1 = 210e3 // MPa (steel matrix)
  const E2 = 70e3  // MPa (softer phase)
  const nu1 = 0.3
  const nu2 = 0.33
  const vf = phaseFraction
  const vm = 1 - vf

  // Rule of mixtures for equivalent properties
  const E_eq = E1 * vm + E2 * vf
  const nu_eq = nu1 * vm + nu2 * vf
  const sigma_y = 250 * vm + 120 * vf + Math.random() * 30
  const H = E_eq * 0.01 * (0.5 + Math.random() * 0.5)

  // Build 6x6 elastic tensor (isotropic, in GPa)
  const E_GPa = E_eq / 1000
  const lambda = E_GPa * nu_eq / ((1 + nu_eq) * (1 - 2 * nu_eq))
  const mu = E_GPa / (2 * (1 + nu_eq))
  const tensor: number[][] = Array.from({ length: 6 }, () => Array(6).fill(0))
  tensor[0][0] = lambda + 2 * mu; tensor[0][1] = lambda; tensor[0][2] = lambda
  tensor[1][0] = lambda; tensor[1][1] = lambda + 2 * mu; tensor[1][2] = lambda
  tensor[2][0] = lambda; tensor[2][1] = lambda; tensor[2][2] = lambda + 2 * mu
  tensor[3][3] = mu; tensor[4][4] = mu; tensor[5][5] = mu

  // Stress-strain curve (linear hardening)
  const { min: strainMin, max: strainMax, steps } = pfFeConfig.strain_range
  const ssCurve: Array<{ strain: number; stress: number }> = []
  const hardeningCurve: Array<{ strain: number; stress: number }> = []
  for (let i = 0; i <= steps; i++) {
    const strain = strainMin + (strainMax - strainMin) * (i / steps)
    const plasticStrain = Math.max(0, strain - sigma_y / E_eq)
    const stress = strain < sigma_y / E_eq
      ? E_eq * strain
      : sigma_y + H * plasticStrain
    ssCurve.push({ strain, stress })
    if (strain >= sigma_y / E_eq) {
      hardeningCurve.push({ strain: plasticStrain, stress })
    }
  }

  // Convergence history
  const convergence: Array<{ iteration: number; error: number }> = []
  let err = 1.0
  for (let i = 1; i <= 30; i++) {
    err = 1.0 * Math.pow(0.45, i - 1)
    convergence.push({ iteration: i, error: err })
    if (err < 1e-7) break
  }

  return {
    success: true,
    effective_elastic_tensor: tensor,
    yield_stress: sigma_y,
    hardening_curve: hardeningCurve,
    stress_strain_curve: ssCurve,
    phase_fraction: phaseFraction,
    equivalent_properties: {
      E: E_eq,
      nu: nu_eq,
      sigma_y,
      hardening_modulus: H,
    },
    homogenization_convergence: convergence,
  }
}

// ============ Canvas Drawing ============

function drawFieldOnCanvas(canvas: HTMLCanvasElement, data: number[][], colorMap: (v: number) => string) {
  const ctx = canvas.getContext('2d')
  if (!ctx || data.length === 0) return

  const rows = data.length
  const cols = data[0].length
  canvas.width = 200
  canvas.height = 200

  const cellW = 200 / cols
  const cellH = 200 / rows

  for (let iy = 0; iy < rows; iy++) {
    for (let ix = 0; ix < cols; ix++) {
      ctx.fillStyle = colorMap(data[iy][ix])
      ctx.fillRect(ix * cellW, iy * cellH, cellW + 0.5, cellH + 0.5)
    }
  }
}

function drawGrainStructure(canvas: HTMLCanvasElement, data: number[][]) {
  const ctx = canvas.getContext('2d')
  if (!ctx || data.length === 0) return

  const rows = data.length
  const cols = data[0].length
  canvas.width = 200
  canvas.height = 200

  // Generate distinct colors for grains
  const uniqueGrains = new Set<number>()
  for (const row of data) for (const v of row) uniqueGrains.add(v)
  const grainColors = new Map<number, string>()
  const palette = [
    '#4F8CFF', '#22c55e', '#ef4444', '#a855f7', '#f59e0b',
    '#06b6d4', '#ec4899', '#84cc16', '#f97316', '#6366f1',
    '#14b8a6', '#e11d48', '#8b5cf6', '#0ea5e9', '#d946ef',
    '#65a30d',
  ]
  let colorIdx = 0
  for (const g of uniqueGrains) {
    grainColors.set(g, palette[colorIdx % palette.length])
    colorIdx++
  }

  const cellW = 200 / cols
  const cellH = 200 / rows

  for (let iy = 0; iy < rows; iy++) {
    for (let ix = 0; ix < cols; ix++) {
      ctx.fillStyle = grainColors.get(data[iy][ix]) ?? '#444'
      ctx.fillRect(ix * cellW, iy * cellH, cellW + 0.5, cellH + 0.5)
    }
  }

  // Draw grain boundaries
  ctx.strokeStyle = 'rgba(0,0,0,0.6)'
  ctx.lineWidth = 0.5
  for (let iy = 0; iy < rows - 1; iy++) {
    for (let ix = 0; ix < cols - 1; ix++) {
      if (data[iy][ix] !== data[iy][ix + 1]) {
        ctx.beginPath()
        ctx.moveTo((ix + 1) * cellW, iy * cellH)
        ctx.lineTo((ix + 1) * cellW, (iy + 1) * cellH)
        ctx.stroke()
      }
      if (data[iy][ix] !== data[iy + 1][ix]) {
        ctx.beginPath()
        ctx.moveTo(ix * cellW, (iy + 1) * cellH)
        ctx.lineTo((ix + 1) * cellW, (iy + 1) * cellH)
        ctx.stroke()
      }
    }
  }
}

function drawYieldSurface(canvas: HTMLCanvasElement, sigma_y: number) {
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  canvas.width = 280
  canvas.height = 240
  const cx = 140
  const cy = 120
  const R = 90

  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, 280, 240)

  // Grid
  ctx.strokeStyle = 'rgba(255,255,255,0.06)'
  ctx.lineWidth = 0.5
  for (let i = -3; i <= 3; i++) {
    ctx.beginPath()
    ctx.moveTo(cx + i * 25, 10)
    ctx.lineTo(cx + i * 25, 230)
    ctx.stroke()
    ctx.beginPath()
    ctx.moveTo(10, cy + i * 25)
    ctx.lineTo(270, cy + i * 25)
    ctx.stroke()
  }

  // Axes
  ctx.strokeStyle = 'rgba(255,255,255,0.2)'
  ctx.lineWidth = 1
  ctx.beginPath(); ctx.moveTo(cx, 10); ctx.lineTo(cx, 230); ctx.stroke()
  ctx.beginPath(); ctx.moveTo(10, cy); ctx.lineTo(270, cy); ctx.stroke()

  // Pi-plane axes (120 deg apart)
  const axes = [
    { angle: -Math.PI / 2, label: 's1' },
    { angle: -Math.PI / 2 + 2 * Math.PI / 3, label: 's2' },
    { angle: -Math.PI / 2 + 4 * Math.PI / 3, label: 's3' },
  ]
  for (const ax of axes) {
    ctx.strokeStyle = 'rgba(255,255,255,0.1)'
    ctx.setLineDash([4, 4])
    ctx.beginPath()
    ctx.moveTo(cx, cy)
    ctx.lineTo(cx + Math.cos(ax.angle) * R * 1.2, cy + Math.sin(ax.angle) * R * 1.2)
    ctx.stroke()
    ctx.setLineDash([])
    ctx.fillStyle = 'rgba(255,255,255,0.4)'
    ctx.font = '9px sans-serif'
    ctx.textAlign = 'center'
    ctx.fillText(ax.label, cx + Math.cos(ax.angle) * (R + 15), cy + Math.sin(ax.angle) * (R + 15))
  }

  // Von Mises circle (Tresca hexagon approx)
  const r_vm = R * 0.75
  ctx.strokeStyle = 'var(--primary)'
  ctx.lineWidth = 2
  ctx.beginPath()
  ctx.arc(cx, cy, r_vm, 0, 2 * Math.PI)
  ctx.stroke()

  // Tresca hexagon
  ctx.strokeStyle = 'var(--accent-yellow)'
  ctx.lineWidth = 1.5
  ctx.beginPath()
  for (let i = 0; i < 6; i++) {
    const angle = -Math.PI / 2 + i * Math.PI / 3
    const x = cx + r_vm * Math.cos(angle)
    const y = cy + r_vm * Math.sin(angle)
    if (i === 0) ctx.moveTo(x, y)
    else ctx.lineTo(x, y)
  }
  ctx.closePath()
  ctx.stroke()

  // Labels
  ctx.fillStyle = 'var(--primary)'
  ctx.font = '9px sans-serif'
  ctx.textAlign = 'left'
  ctx.fillText('Von Mises', 10, 20)
  ctx.fillStyle = 'var(--accent-yellow)'
  ctx.fillText('Tresca', 10, 32)

  // Sigma_y annotation
  ctx.fillStyle = 'rgba(255,255,255,0.5)'
  ctx.font = '9px monospace'
  ctx.textAlign = 'right'
  ctx.fillText(`sy = ${sigma_y.toFixed(0)} MPa`, 270, 20)
}

// ============ Computed ============

const equivProps = computed(() => {
  if (!pfFeResult.value) return []
  const ep = pfFeResult.value.equivalent_properties
  return [
    { key: 'E', label: 'E (等效模量)', value: (ep.E / 1000).toFixed(1), unit: 'GPa' },
    { key: 'nu', label: 'nu (泊松比)', value: ep.nu.toFixed(3), unit: '' },
    { key: 'sigma_y', label: 'sy (屈服应力)', value: ep.sigma_y.toFixed(1), unit: 'MPa' },
    { key: 'H', label: 'H (硬化模量)', value: ep.hardening_modulus.toFixed(1), unit: 'MPa' },
  ]
})

const maxStress = computed(() => {
  if (!pfFeResult.value) return 0
  let max = 0
  for (const p of pfFeResult.value.stress_strain_curve) {
    if (p.stress > max) max = p.stress
  }
  return max
})

const maxStressLabel = computed(() => {
  return maxStress.value.toFixed(0)
})

const ssCurvePoints = computed(() => {
  if (!pfFeResult.value) return ''
  const curve = pfFeResult.value.stress_strain_curve
  const { max: strainMax } = pfFeConfig.strain_range
  const plotL = 40
  const plotR = 290
  const plotT = 20
  const plotB = 180
  const plotW = plotR - plotL
  const plotH = plotB - plotT

  return curve.map(p => {
    const x = plotL + (p.strain / strainMax) * plotW
    const y = plotB - (p.stress / maxStress.value) * plotH
    return `${x},${y}`
  }).join(' ')
})

const yieldPoint = computed(() => {
  if (!pfFeResult.value) return null
  const { max: strainMax } = pfFeConfig.strain_range
  const plotL = 40
  const plotR = 290
  const plotT = 20
  const plotB = 180
  const plotW = plotR - plotL
  const plotH = plotB - plotT

  const sy = pfFeResult.value.equivalent_properties.sigma_y
  const E = pfFeResult.value.equivalent_properties.E
  const yieldStrain = sy / E

  return {
    x: plotL + (yieldStrain / strainMax) * plotW,
    y: plotB - (sy / maxStress.value) * plotH,
  }
})

const convPoints = computed(() => {
  if (!pfFeResult.value) return ''
  const hist = pfFeResult.value.homogenization_convergence
  if (hist.length === 0) return ''
  const maxIter = hist.length
  const plotL = 40
  const plotR = 290
  const plotT = 20
  const plotB = 180
  const plotW = plotR - plotL
  const plotH = plotB - plotT

  const maxErr = Math.log10(Math.max(hist[0].error, 1e-15))
  const minErr = -8

  return hist.map(r => {
    const x = plotL + (r.iteration / Math.max(maxIter, 1)) * plotW
    const logErr = Math.log10(Math.max(r.error, 1e-15))
    const norm = (logErr - minErr) / (maxErr - minErr + 1e-10)
    const y = plotB - Math.max(0, Math.min(1, norm)) * plotH
    return `${x},${y}`
  }).join(' ')
})

const hardeningPoints = computed(() => {
  if (!pfFeResult.value || pfFeResult.value.hardening_curve.length === 0) return ''
  const curve = pfFeResult.value.hardening_curve
  const plotL = 30
  const plotR = 250
  const plotT = 10
  const plotB = 110
  const plotW = plotR - plotL
  const plotH = plotB - plotT

  let maxPStrain = 0
  let maxPStress = 0
  for (const p of curve) {
    if (p.strain > maxPStrain) maxPStrain = p.strain
    if (p.stress > maxPStress) maxPStress = p.stress
  }
  if (maxPStrain === 0 || maxPStress === 0) return ''

  return curve.map(p => {
    const x = plotL + (p.strain / maxPStrain) * plotW
    const y = plotB - (p.stress / maxPStress) * plotH
    return `${x},${y}`
  }).join(' ')
})

// ============ Methods ============

function qualityColor(v: number): string {
  if (v >= 0.9) return 'var(--accent-green)'
  if (v >= 0.7) return 'var(--accent-yellow)'
  return 'var(--accent-red)'
}

function formatTensorCell(val: number): string {
  if (Math.abs(val) < 1e-10) return '0.00'
  return val.toFixed(2)
}

function applyTemplate(tpl: BridgeTemplate) {
  if (tpl.direction === 'md_to_pf') {
    activeTab.value = 'md_to_pf'
    switch (tpl.id) {
      case 'nano_polycrystal':
        mdPfConfig.coarse_graining_method = 'voronoi'
        mdPfConfig.target_grid = { nx: 128, ny: 128, nz: 1 }
        mdPfConfig.smoothing_length_A = 2.5
        mdPfConfig.order_parameter_mapping = 'crystal_structure'
        mdPfConfig.temperature_mapping = 'kinetic_energy'
        mdPfConfig.interface_detection_threshold = 0.45
        break
      case 'crack_tip_fe':
        mdPfConfig.coarse_graining_method = 'centro_symmetry'
        mdPfConfig.target_grid = { nx: 256, ny: 256, nz: 1 }
        mdPfConfig.smoothing_length_A = 1.5
        mdPfConfig.order_parameter_mapping = 'local_density'
        mdPfConfig.temperature_mapping = 'local_pressure'
        mdPfConfig.interface_detection_threshold = 0.3
        break
    }
  } else {
    activeTab.value = 'pf_to_fe'
    switch (tpl.id) {
      case 'dual_phase_steel':
        pfFeConfig.homogenization_method = 'mori_tanaka'
        pfFeConfig.strain_range = { min: 0, max: 0.2, steps: 60 }
        pfFeConfig.temperature = 293
        pfFeConfig.output_type = 'stress_strain_curve'
        break
      case 'composite_homo':
        pfFeConfig.homogenization_method = 'self_consistent'
        pfFeConfig.strain_range = { min: 0, max: 0.1, steps: 40 }
        pfFeConfig.temperature = 300
        pfFeConfig.output_type = 'elastic_tensor'
        break
    }
  }
}

function resetAll() {
  activeTab.value = 'md_to_pf'
  mdPfResult.value = null
  pfFeResult.value = null
  qualityResult.value = null
  runningMdPf.value = false
  runningPfFe.value = false
  mdPfConfig.coarse_graining_method = 'voronoi'
  mdPfConfig.target_grid = { nx: 64, ny: 64, nz: 1 }
  mdPfConfig.smoothing_length_A = 3.0
  mdPfConfig.order_parameter_mapping = 'crystal_structure'
  mdPfConfig.temperature_mapping = 'kinetic_energy'
  mdPfConfig.interface_detection_threshold = 0.5
  pfFeConfig.homogenization_method = 'mori_tanaka'
  pfFeConfig.strain_range = { min: 0, max: 0.15, steps: 50 }
  pfFeConfig.temperature = 300
  pfFeConfig.output_type = 'stress_strain_curve'
}

function colorMapBlueRed(v: number): string {
  const t = Math.max(0, Math.min(1, v))
  const r = Math.round(t * 255)
  const g = Math.round((1 - Math.abs(t - 0.5) * 2) * 100)
  const b = Math.round((1 - t) * 255)
  return `rgb(${r},${g},${b})`
}

function colorMapTemp(v: number): string {
  const t = Math.max(0, Math.min(1, (v - 280) / 60))
  const r = Math.round(t * 255)
  const g = Math.round(t * 180)
  const b = Math.round((1 - t) * 200)
  return `rgb(${r},${g},${b})`
}

async function runMdToPf() {
  runningMdPf.value = true
  qualityResult.value = null
  try {
    await new Promise(resolve => setTimeout(resolve, 2000))
    mdPfResult.value = generateMockMdPfResult()
    await nextTick()
    drawMdPfCanvases()
  } catch (e) {
    console.error('MD to PF failed:', e)
  } finally {
    runningMdPf.value = false
  }
}

async function runPfToFe() {
  runningPfFe.value = true
  qualityResult.value = null
  try {
    await new Promise(resolve => setTimeout(resolve, 2500))
    pfFeResult.value = generateMockPfFeResult()
    await nextTick()
    drawPfFeCanvases()
  } catch (e) {
    console.error('PF to FE failed:', e)
  } finally {
    runningPfFe.value = false
  }
}

function drawMdPfCanvases() {
  if (!mdPfResult.value) return
  if (orderParamCanvas.value) {
    drawFieldOnCanvas(orderParamCanvas.value, mdPfResult.value.order_parameter_field, colorMapBlueRed)
  }
  if (tempFieldCanvas.value) {
    drawFieldOnCanvas(tempFieldCanvas.value, mdPfResult.value.temperature_field, colorMapTemp)
  }
  if (grainCanvas.value) {
    drawGrainStructure(grainCanvas.value, mdPfResult.value.grain_structure)
  }
}

function drawPfFeCanvases() {
  if (!pfFeResult.value) return
  if (yieldSurfaceCanvas.value) {
    drawYieldSurface(yieldSurfaceCanvas.value, pfFeResult.value.yield_stress)
  }
}

async function validateMdPf() {
  if (!mdPfResult.value) return
  try {
    await new Promise(resolve => setTimeout(resolve, 800))
    const q = mdPfResult.value.mapping_quality
    const score = (q.spatial_correlation + q.energy_conservation + q.interface_sharpness) / 3
    const issues: string[] = []
    if (q.spatial_correlation < 0.85) issues.push('空间相关性低于推荐阈值 (0.85)')
    if (q.energy_conservation < 0.90) issues.push('能量守恒偏差较大')
    if (q.interface_sharpness < 0.75) issues.push('界面锐度不足，建议减小平滑长度')
    qualityResult.value = { quality_score: score, issues }
  } catch (e) {
    console.error('Validation failed:', e)
  }
}

async function validatePfFe() {
  if (!pfFeResult.value) return
  try {
    await new Promise(resolve => setTimeout(resolve, 800))
    const conv = pfFeResult.value.homogenization_convergence
    const converged = conv.length > 0 && conv[conv.length - 1].error < 1e-6
    const score = converged ? 0.92 + Math.random() * 0.07 : 0.6 + Math.random() * 0.2
    const issues: string[] = []
    if (!converged) issues.push('均匀化迭代未收敛')
    if (pfFeResult.value.phase_fraction > 0.5) issues.push('增强相体积分数较高，建议使用数值方法')
    qualityResult.value = { quality_score: score, issues }
  } catch (e) {
    console.error('Validation failed:', e)
  }
}

function exportMdPf() {
  if (!mdPfResult.value) return
  const data = JSON.stringify(mdPfResult.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'md_to_phasefield_result.json'
  a.click()
  URL.revokeObjectURL(url)
}

function exportPfFe() {
  if (!pfFeResult.value) return
  const data = JSON.stringify(pfFeResult.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'phasefield_to_fe_result.json'
  a.click()
  URL.revokeObjectURL(url)
}
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

.matrix-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 10px;
}

.matrix-header {
  padding: 3px 5px;
  color: var(--text-muted);
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
}

.matrix-cell {
  padding: 3px 5px;
  color: var(--text-secondary);
  font-family: monospace;
  font-size: 9px;
  text-align: right;
  white-space: nowrap;
}

.matrix-diagonal {
  color: var(--primary);
  font-weight: 600;
}
</style>
