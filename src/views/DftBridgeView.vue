<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">DFT 尺度桥接</h2>
        <p class="text-sm" style="color: var(--text-muted)">DFT → MD 势函数训练，DFT → 相场 GL 参数，相图计算</p>
      </div>
      <div class="flex items-center gap-2">
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
        @click="activeTab = 'dft_md'"
        class="px-6 py-2.5 text-sm font-medium transition border-b-2"
        :style="activeTab === 'dft_md'
          ? 'color: var(--primary); border-color: var(--primary)'
          : 'color: var(--text-muted); border-color: transparent'"
      >
        DFT → MD
      </button>
      <button
        @click="activeTab = 'dft_pf'"
        class="px-6 py-2.5 text-sm font-medium transition border-b-2"
        :style="activeTab === 'dft_pf'
          ? 'color: var(--primary); border-color: var(--primary)'
          : 'color: var(--text-muted); border-color: transparent'"
      >
        DFT → 相场
      </button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- ==================== DFT → MD Panel ==================== -->
      <template v-if="activeTab === 'dft_md'">
        <div class="flex-1 flex overflow-hidden">
          <!-- Left: Configuration -->
          <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <!-- Potential Type -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
                势函数类型
              </h4>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="m in potentialTypes"
                  :key="m.value"
                  @click="trainingConfig.potential_type = m.value"
                  :class="['px-2 py-2 rounded text-xs text-left transition border', trainingConfig.potential_type === m.value ? 'text-white' : '']"
                  :style="trainingConfig.potential_type === m.value
                    ? 'background: var(--primary); border-color: var(--primary)'
                    : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                >
                  <div class="font-medium">{{ m.label }}</div>
                  <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
                </button>
              </div>
            </div>

            <!-- Training Data Settings -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">2</span>
                训练数据设置
              </h4>
              <div class="space-y-2">
                <div>
                  <label class="label">DFT 数据目录</label>
                  <input v-model="trainingConfig.dft_data_directory" type="text" placeholder="/path/to/dft/results" class="input w-full text-xs" />
                </div>
                <div class="grid grid-cols-3 gap-2">
                  <div>
                    <label class="label">训练集</label>
                    <input v-model.number="trainingConfig.training_set_ratio" type="number" step="0.05" min="0.1" max="0.9" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">验证集</label>
                    <input v-model.number="trainingConfig.validation_set_ratio" type="number" step="0.05" min="0.05" max="0.5" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">测试集</label>
                    <input v-model.number="trainingConfig.test_set_ratio" type="number" step="0.05" min="0.05" max="0.3" class="input w-full text-xs" />
                  </div>
                </div>
                <div class="grid grid-cols-3 gap-2">
                  <div>
                    <label class="label">能量权重</label>
                    <input v-model.number="trainingConfig.energy_weight" type="number" step="0.1" min="0" max="10" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">力权重</label>
                    <input v-model.number="trainingConfig.force_weight" type="number" step="0.1" min="0" max="100" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">应力权重</label>
                    <input v-model.number="trainingConfig.stress_weight" type="number" step="0.01" min="0" max="10" class="input w-full text-xs" />
                  </div>
                </div>
              </div>
            </div>

            <!-- Network Parameters -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">3</span>
                网络参数
              </h4>
              <div class="space-y-2">
                <div class="grid grid-cols-2 gap-2">
                  <div>
                    <label class="label">最大神经元数</label>
                    <input v-model.number="trainingConfig.max_neuron" type="number" step="10" min="10" max="500" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">截断距离 (Å)</label>
                    <input v-model.number="trainingConfig.cutoff_A" type="number" step="0.5" min="3" max="10" class="input w-full text-xs" />
                  </div>
                </div>
                <div class="grid grid-cols-3 gap-2">
                  <div>
                    <label class="label">训练轮次</label>
                    <input v-model.number="trainingConfig.training_epochs" type="number" step="100" min="100" max="100000" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">批大小</label>
                    <input v-model.number="trainingConfig.batch_size" type="number" step="10" min="1" max="1000" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">学习率</label>
                    <input v-model.number="trainingConfig.learning_rate" type="number" step="0.001" min="0.0001" max="1" class="input w-full text-xs" />
                  </div>
                </div>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="space-y-2">
              <button @click="runPrepareData" :disabled="runningPrepare" class="btn btn-ghost text-xs w-full">
                <span v-if="runningPrepare" class="mr-1 animate-spin">&#10227;</span>
                {{ runningPrepare ? '准备中...' : '准备训练数据' }}
              </button>
              <button @click="runTraining" :disabled="runningTraining" class="btn btn-primary text-xs w-full">
                <span v-if="runningTraining" class="mr-1 animate-spin">&#10227;</span>
                {{ runningTraining ? '训练中...' : '训练势函数' }}
              </button>
              <button @click="runValidation" :disabled="!trainingResult" class="btn btn-ghost text-xs w-full">
                验证势函数
              </button>
              <button @click="runExport" :disabled="!trainingResult" class="btn btn-ghost text-xs w-full">
                导出势函数
              </button>
            </div>
          </div>

          <!-- Right: Results -->
          <div class="flex-1 flex flex-col overflow-hidden">
            <div v-if="!trainingResult && !runningTraining" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
              <div class="text-center">
                <div class="text-4xl mb-2">&#9883;</div>
                <div class="text-sm">配置参数后运行 DFT → MD 势函数训练</div>
              </div>
            </div>
            <template v-else>
              <div class="flex-1 overflow-y-auto p-4 space-y-4">
                <!-- Training Status Panel -->
                <div class="result-card">
                  <span class="result-label">训练状态</span>
                  <div class="mt-3 space-y-3">
                    <div class="flex items-center justify-between">
                      <span class="text-xs" style="color: var(--text-secondary)">运行状态</span>
                      <span
                        class="text-xs font-medium px-2 py-0.5 rounded-full"
                        :style="runningTraining
                          ? 'background: rgba(37,99,235,0.1); color: var(--primary)'
                          : 'background: rgba(34,197,94,0.1); color: var(--accent-green)'"
                      >
                        {{ runningTraining ? '训练中...' : '已完成' }}
                      </span>
                    </div>
                    <!-- Progress Bar -->
                    <div>
                      <div class="flex justify-between text-xs mb-1">
                        <span style="color: var(--text-secondary)">训练进度</span>
                        <span class="font-mono" style="color: var(--text-primary)">{{ trainingProgress.current }} / {{ trainingProgress.total }}</span>
                      </div>
                      <div class="w-full h-3 rounded-full" style="background: var(--bg-elevated)">
                        <div
                          class="h-3 rounded-full transition-all"
                          :style="{ background: 'var(--primary)', width: trainingProgress.percent + '%' }"
                        ></div>
                      </div>
                    </div>
                    <div class="grid grid-cols-2 gap-3">
                      <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                        <div class="text-[10px]" style="color: var(--text-muted)">当前轮次</div>
                        <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ trainingProgress.current }}</div>
                      </div>
                      <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                        <div class="text-[10px]" style="color: var(--text-muted)">最佳轮次</div>
                        <div class="text-sm font-mono font-semibold mt-1" style="color: var(--accent-green)">{{ trainingResult?.best_epoch ?? '-' }}</div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Loss Curves -->
                <div class="grid grid-cols-2 gap-4">
                  <div class="result-card">
                    <span class="result-label">Energy RMSE vs Epoch</span>
                    <div class="mt-2">
                      <svg viewBox="0 0 300 200" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                        <line v-for="i in 5" :key="'eh'+i" x1="40" y1="20 + i * 32" x2="290" y2="20 + i * 32" stroke="rgba(0,0,0,0.05)" stroke-width="0.5" />
                        <line v-for="i in 5" :key="'ev'+i" x1="40 + i * 50" y1="20" x2="40 + i * 50" y2="180" stroke="rgba(0,0,0,0.05)" stroke-width="0.5" />
                        <text x="165" y="12" fill="var(--text-secondary)" font-size="9" text-anchor="middle">Energy RMSE (meV/atom)</text>
                        <text x="35" y="24" fill="var(--text-muted)" font-size="7" text-anchor="end">{{ energyYMaxLabel }}</text>
                        <text x="35" y="184" fill="var(--text-muted)" font-size="7" text-anchor="end">0</text>
                        <text x="40" y="195" fill="var(--text-muted)" font-size="7" text-anchor="middle">0</text>
                        <text x="290" y="195" fill="var(--text-muted)" font-size="7" text-anchor="middle">{{ trainingConfig.training_epochs }}</text>
                        <polyline v-if="energyTrainPoints" :points="energyTrainPoints" fill="none" stroke="var(--primary)" stroke-width="1.5" />
                        <polyline v-if="energyValPoints" :points="energyValPoints" fill="none" stroke="var(--accent-amber)" stroke-width="1.5" stroke-dasharray="4,2" />
                        <line x1="200" y1="20" x2="200" y2="180" stroke="rgba(0,0,0,0.08)" stroke-width="0.5" stroke-dasharray="2,2" />
                        <text x="260" y="16" fill="var(--primary)" font-size="7">训练</text>
                        <text x="260" y="26" fill="var(--accent-amber)" font-size="7">验证</text>
                      </svg>
                    </div>
                  </div>
                  <div class="result-card">
                    <span class="result-label">Force RMSE vs Epoch</span>
                    <div class="mt-2">
                      <svg viewBox="0 0 300 200" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                        <line v-for="i in 5" :key="'fh'+i" x1="40" y1="20 + i * 32" x2="290" y2="20 + i * 32" stroke="rgba(0,0,0,0.05)" stroke-width="0.5" />
                        <line v-for="i in 5" :key="'fv'+i" x1="40 + i * 50" y1="20" x2="40 + i * 50" y2="180" stroke="rgba(0,0,0,0.05)" stroke-width="0.5" />
                        <text x="165" y="12" fill="var(--text-secondary)" font-size="9" text-anchor="middle">Force RMSE (meV/Å)</text>
                        <text x="35" y="24" fill="var(--text-muted)" font-size="7" text-anchor="end">{{ forceYMaxLabel }}</text>
                        <text x="35" y="184" fill="var(--text-muted)" font-size="7" text-anchor="end">0</text>
                        <text x="40" y="195" fill="var(--text-muted)" font-size="7" text-anchor="middle">0</text>
                        <text x="290" y="195" fill="var(--text-muted)" font-size="7" text-anchor="middle">{{ trainingConfig.training_epochs }}</text>
                        <polyline v-if="forceTrainPoints" :points="forceTrainPoints" fill="none" stroke="var(--primary)" stroke-width="1.5" />
                        <polyline v-if="forceValPoints" :points="forceValPoints" fill="none" stroke="var(--accent-amber)" stroke-width="1.5" stroke-dasharray="4,2" />
                        <line x1="200" y1="20" x2="200" y2="180" stroke="rgba(0,0,0,0.08)" stroke-width="0.5" stroke-dasharray="2,2" />
                        <text x="260" y="16" fill="var(--primary)" font-size="7">训练</text>
                        <text x="260" y="26" fill="var(--accent-amber)" font-size="7">验证</text>
                      </svg>
                    </div>
                  </div>
                </div>

                <!-- Test Results Cards -->
                <div v-if="trainingResult && !runningTraining" class="result-card">
                  <span class="result-label">测试结果</span>
                  <div class="grid grid-cols-4 gap-3 mt-3">
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">Energy RMSE</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ trainingResult.test_energy_rmse_meV.toFixed(2) }}</div>
                      <div class="text-[10px]" style="color: var(--text-muted)">meV/atom</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">Force RMSE</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ trainingResult.test_force_rmse_meV_A.toFixed(2) }}</div>
                      <div class="text-[10px]" style="color: var(--text-muted)">meV/Å</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">Stress RMSE</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ trainingResult.test_stress_rmse_GPa.toFixed(4) }}</div>
                      <div class="text-[10px]" style="color: var(--text-muted)">GPa</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">训练时间</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--primary)">{{ formatTime(trainingResult.training_time_seconds) }}</div>
                      <div class="text-[10px]" style="color: var(--text-muted)">{{ trainingResult.training_time_seconds.toFixed(0) }}s</div>
                    </div>
                  </div>
                </div>

                <!-- Potential File Preview -->
                <div v-if="trainingResult && !runningTraining" class="result-card">
                  <span class="result-label">势函数文件</span>
                  <div class="mt-2 p-3 rounded font-mono text-xs" style="background: var(--bg-base); color: var(--text-secondary); word-break: break-all">
                    {{ trainingResult.potential_file_path }}
                  </div>
                </div>
              </div>
            </template>
          </div>
        </div>
      </template>

      <!-- ==================== DFT → Phase Field Panel ==================== -->
      <template v-else>
        <div class="flex-1 flex overflow-hidden">
          <!-- Left: Configuration -->
          <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">
            <!-- Phase Diagram Settings -->
            <div class="panel-section">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">
                <span class="inline-flex items-center justify-center w-5 h-5 rounded-full text-white text-xs mr-1" style="background: var(--primary)">1</span>
                相图计算设置
              </h4>
              <div class="space-y-2">
                <div>
                  <label class="label">DFT 结果目录</label>
                  <input v-model="pfConfig.dft_results_directory" type="text" placeholder="/path/to/dft/results" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">目标相列表</label>
                  <div class="flex flex-wrap gap-1 mt-1">
                    <span
                      v-for="phase in pfConfig.target_phases"
                      :key="phase"
                      class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs"
                      style="background: var(--primary-glow); color: var(--primary)"
                    >
                      {{ phase }}
                      <button @click="removePhase(phase)" class="hover:opacity-70" style="font-size: 10px">&times;</button>
                    </span>
                    <input
                      v-model="newPhaseName"
                      @keydown.enter="addPhase"
                      type="text"
                      placeholder="添加相..."
                      class="input text-xs"
                      style="width: 80px; padding: 2px 6px"
                    />
                  </div>
                </div>
                <div class="grid grid-cols-3 gap-2">
                  <div>
                    <label class="label">T min (K)</label>
                    <input v-model.number="pfConfig.temperature_range.min" type="number" step="100" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">T max (K)</label>
                    <input v-model.number="pfConfig.temperature_range.max" type="number" step="100" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">T step (K)</label>
                    <input v-model.number="pfConfig.temperature_range.step" type="number" step="10" class="input w-full text-xs" />
                  </div>
                </div>
                <div class="grid grid-cols-3 gap-2">
                  <div>
                    <label class="label">x min</label>
                    <input v-model.number="pfConfig.composition_range.min" type="number" step="0.05" min="0" max="1" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">x max</label>
                    <input v-model.number="pfConfig.composition_range.max" type="number" step="0.05" min="0" max="1" class="input w-full text-xs" />
                  </div>
                  <div>
                    <label class="label">x step</label>
                    <input v-model.number="pfConfig.composition_range.step" type="number" step="0.01" min="0.01" max="0.1" class="input w-full text-xs" />
                  </div>
                </div>
                <div>
                  <label class="label">自由能模型</label>
                  <select v-model="pfConfig.free_energy_model" class="input w-full text-xs">
                    <option value="regular_solution">正规溶液模型</option>
                    <option value="subregular">亚正规溶液模型</option>
                    <option value="calphad">CALPHAD</option>
                  </select>
                </div>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="space-y-2">
              <button @click="runCalculatePhaseDiagram" :disabled="runningPhaseDiagram" class="btn btn-primary text-xs w-full">
                <span v-if="runningPhaseDiagram" class="mr-1 animate-spin">&#10227;</span>
                {{ runningPhaseDiagram ? '计算中...' : '计算相图' }}
              </button>
              <button @click="runExtractGlParams" :disabled="runningGlParams" class="btn btn-ghost text-xs w-full">
                <span v-if="runningGlParams" class="mr-1 animate-spin">&#10227;</span>
                {{ runningGlParams ? '提取中...' : '提取 GL 参数' }}
              </button>
              <button @click="runAlignChemicalPotential" :disabled="!pfResult" class="btn btn-ghost text-xs w-full">
                化学势对齐
              </button>
            </div>
          </div>

          <!-- Right: Results -->
          <div class="flex-1 flex flex-col overflow-hidden">
            <div v-if="!pfResult && !glParams && !runningPhaseDiagram && !runningGlParams" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
              <div class="text-center">
                <div class="text-4xl mb-2">&#9883;</div>
                <div class="text-sm">配置参数后运行 DFT → 相场计算</div>
              </div>
            </div>
            <template v-else>
              <div class="flex-1 overflow-y-auto p-4 space-y-4">
                <!-- GL Parameters Display -->
                <div v-if="glParams" class="result-card">
                  <span class="result-label">Ginzburg-Landau 参数</span>
                  <div class="grid grid-cols-4 gap-3 mt-3">
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">A</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ glParams.free_energy_coefficients.A.toFixed(4) }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">B</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ glParams.free_energy_coefficients.B.toFixed(4) }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">C</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ glParams.free_energy_coefficients.C.toFixed(4) }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">&kappa;</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ (glParams.free_energy_coefficients as Record<string, number>)['κ']?.toFixed(4) }}</div>
                    </div>
                  </div>
                  <div class="grid grid-cols-4 gap-3 mt-3">
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">界面宽度</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ glParams.interface_width.toFixed(2) }} nm</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">平衡序参量</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--primary)">{{ glParams.equilibrium_order_parameter.toFixed(4) }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">迁移率</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ glParams.mobility.toExponential(2) }}</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">化学势</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ glParams.chemical_potential.toFixed(3) }} eV</div>
                    </div>
                  </div>
                </div>

                <!-- Phase Diagram Visualization -->
                <div v-if="pfResult" class="result-card">
                  <span class="result-label">T-x 二元相图</span>
                  <div class="mt-2">
                    <canvas ref="phaseDiagramCanvas" width="500" height="360" class="w-full rounded" style="background: var(--bg-base)"></canvas>
                  </div>
                </div>

                <!-- Free Energy Curves -->
                <div v-if="pfResult" class="result-card">
                  <span class="result-label">自由能曲线 (多温度叠加)</span>
                  <div class="mt-2">
                    <svg viewBox="0 0 400 220" class="w-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                      <line v-for="i in 5" :key="'feh'+i" x1="50" y1="20 + i * 36" x2="390" y2="20 + i * 36" stroke="rgba(0,0,0,0.05)" stroke-width="0.5" />
                      <line v-for="i in 5" :key="'fev'+i" x1="50 + i * 68" y1="20" x2="50 + i * 68" y2="200" stroke="rgba(0,0,0,0.05)" stroke-width="0.5" />
                      <text x="220" y="14" fill="var(--text-secondary)" font-size="9" text-anchor="middle">Free Energy vs Composition</text>
                      <text x="220" y="215" fill="var(--text-muted)" font-size="8" text-anchor="middle">Composition (x)</text>
                      <text x="14" y="115" fill="var(--text-muted)" font-size="8" text-anchor="middle" transform="rotate(-90, 14, 115)">F (eV/atom)</text>
                      <polyline
                        v-for="(pts, idx) in freeEnergySvgLines"
                        :key="'fe'+idx"
                        :points="pts.points"
                        fill="none"
                        :stroke="pts.color"
                        stroke-width="1.5"
                      />
                      <!-- Legend -->
                      <text x="300" y="30" fill="var(--primary)" font-size="7">T = 800K</text>
                      <text x="300" y="40" fill="var(--accent-amber)" font-size="7">T = 1000K</text>
                      <text x="300" y="50" fill="var(--accent-red)" font-size="7">T = 1200K</text>
                    </svg>
                  </div>
                </div>

                <!-- Chemical Potential Alignment Result -->
                <div v-if="chemicalPotentials" class="result-card">
                  <span class="result-label">化学势对齐结果</span>
                  <div class="grid grid-cols-3 gap-3 mt-3">
                    <div
                      v-for="(mu, phase) in chemicalPotentials"
                      :key="phase"
                      class="text-center p-2 rounded"
                      style="background: var(--bg-elevated)"
                    >
                      <div class="text-[10px]" style="color: var(--text-muted)">{{ phase }}</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ (mu as number).toFixed(4) }} eV</div>
                    </div>
                  </div>
                </div>

                <!-- Result Statistics -->
                <div v-if="pfResult" class="result-card">
                  <span class="result-label">结果统计</span>
                  <div class="grid grid-cols-4 gap-3 mt-3">
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">混溶隙</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--text-primary)">{{ pfResult.miscibility_gap.min_T.toFixed(0) }}-{{ pfResult.miscibility_gap.max_T.toFixed(0) }} K</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">临界温度</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--accent-red)">{{ pfResult.critical_temperature.toFixed(0) }} K</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">共晶温度</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--accent-amber)">{{ eutecticTemp }} K</div>
                    </div>
                    <div class="text-center p-2 rounded" style="background: var(--bg-elevated)">
                      <div class="text-[10px]" style="color: var(--text-muted)">共晶成分</div>
                      <div class="text-sm font-mono font-semibold mt-1" style="color: var(--accent-amber)">{{ eutecticComp }}</div>
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
  PotentialType,
  TrainingConfig,
  TrainingResult,
  LossRecord,
  GinzburgLandauParams,
  DftToPfConfig,
  DftToPfResult,
  FreeEnergyCurvePoint,
  DftBridgeTemplate,
} from '../api/dftBridge'

// ============ Constants ============

const potentialTypes = [
  { value: 'nep' as PotentialType, label: 'NEP', desc: '神经网势' },
  { value: 'mtp' as PotentialType, label: 'MTP', desc: '力矩势' },
  { value: 'eam' as PotentialType, label: 'EAM', desc: '嵌入原子' },
  { value: 'meam' as PotentialType, label: 'MEAM', desc: '改进EAM' },
  { value: 'lj' as PotentialType, label: 'LJ', desc: 'Lennard-Jones' },
  { value: 'tersoff' as PotentialType, label: 'Tersoff', desc: '三体势' },
]

const templates: DftBridgeTemplate[] = [
  {
    id: 'fe_ni_nep',
    name: 'Fe-Ni NEP势',
    description: 'Fe-Ni 二元合金 NEP 势函数训练',
    config: {
      potential_type: 'nep',
      dft_data_directory: '/data/dft/fe_ni',
      training_set_ratio: 0.8,
      validation_set_ratio: 0.1,
      test_set_ratio: 0.1,
      energy_weight: 1.0,
      force_weight: 50.0,
      stress_weight: 0.1,
      max_neuron: 120,
      cutoff_A: 6.0,
      training_epochs: 5000,
      batch_size: 32,
      learning_rate: 0.005,
    },
  },
  {
    id: 'cu_zr_mtp',
    name: 'Cu-Zr MTP势',
    description: 'Cu-Zr 非晶合金 MTP 势函数训练',
    config: {
      potential_type: 'mtp',
      dft_data_directory: '/data/dft/cu_zr',
      training_set_ratio: 0.75,
      validation_set_ratio: 0.15,
      test_set_ratio: 0.1,
      energy_weight: 2.0,
      force_weight: 80.0,
      stress_weight: 0.05,
      max_neuron: 200,
      cutoff_A: 5.5,
      training_epochs: 8000,
      batch_size: 64,
      learning_rate: 0.002,
    },
  },
  {
    id: 'al_si_phase',
    name: 'Al-Si 相图',
    description: 'Al-Si 二元共晶相图计算',
    config: {
      dft_results_directory: '/data/dft/al_si',
      target_phases: ['FCC_Al', 'Diamond_Si', 'Liquid'],
      temperature_range: { min: 300, max: 2000, step: 20 },
      composition_range: { min: 0, max: 1, step: 0.01 },
      free_energy_model: 'regular_solution',
    },
  },
  {
    id: 'fe_c_phase',
    name: 'Fe-C 相图',
    description: 'Fe-C 二元相图计算',
    config: {
      dft_results_directory: '/data/dft/fe_c',
      target_phases: ['BCC_Fe', 'FCC_Austenite', 'Cementite'],
      temperature_range: { min: 300, max: 1800, step: 15 },
      composition_range: { min: 0, max: 0.08, step: 0.001 },
      free_energy_model: 'calphad',
    },
  },
]

// ============ State ============

const activeTab = ref<'dft_md' | 'dft_pf'>('dft_md')
const runningPrepare = ref(false)
const runningTraining = ref(false)
const runningPhaseDiagram = ref(false)
const runningGlParams = ref(false)
const newPhaseName = ref('')

const trainingResult = ref<TrainingResult | null>(null)
const pfResult = ref<DftToPfResult | null>(null)
const glParams = ref<GinzburgLandauParams | null>(null)
const chemicalPotentials = ref<Record<string, number> | null>(null)

const phaseDiagramCanvas = ref<HTMLCanvasElement | null>(null)

const trainingConfig = reactive<TrainingConfig>({
  potential_type: 'nep',
  dft_data_directory: '/data/dft/results',
  training_set_ratio: 0.8,
  validation_set_ratio: 0.1,
  test_set_ratio: 0.1,
  energy_weight: 1.0,
  force_weight: 50.0,
  stress_weight: 0.1,
  max_neuron: 100,
  cutoff_A: 6.0,
  training_epochs: 2000,
  batch_size: 32,
  learning_rate: 0.005,
})

const pfConfig = reactive<DftToPfConfig>({
  dft_results_directory: '/data/dft/results',
  target_phases: ['Liquid', 'FCC', 'BCC'],
  temperature_range: { min: 300, max: 1800, step: 20 },
  composition_range: { min: 0, max: 1, step: 0.01 },
  free_energy_model: 'regular_solution',
})

// ============ Training Progress ============

const trainingProgress = reactive({
  current: 0,
  total: 0,
  percent: 0,
})

// ============ Mock Data Generation ============

function generateLossHistory(epochs: number, initialEnergy: number, initialForce: number): { training: LossRecord[]; validation: LossRecord[] } {
  const training: LossRecord[] = []
  const validation: LossRecord[] = []
  const bestEpoch = Math.floor(epochs * (0.6 + Math.random() * 0.3))

  for (let e = 1; e <= epochs; e++) {
    const t = e / epochs
    const decay = Math.exp(-3.5 * t)
    const noise = () => (Math.random() - 0.5) * 0.02

    const eTrain = initialEnergy * decay * (1 + noise()) + 0.5
    const fTrain = initialForce * decay * (1 + noise()) + 2.0
    training.push({ epoch: e, energy_rmse: Math.max(0.1, eTrain), force_rmse: Math.max(0.5, fTrain) })

    const valDecay = Math.exp(-3.0 * t)
    const valNoise = () => (Math.random() - 0.5) * 0.04
    const eVal = initialEnergy * valDecay * (1 + valNoise()) + 0.8 + (e > bestEpoch ? (e - bestEpoch) * 0.005 : 0)
    const fVal = initialForce * valDecay * (1 + valNoise()) + 3.0 + (e > bestEpoch ? (e - bestEpoch) * 0.02 : 0)
    validation.push({ epoch: e, energy_rmse: Math.max(0.1, eVal), force_rmse: Math.max(0.5, fVal) })
  }

  return { training, validation }
}

function generateMockTrainingResult(): TrainingResult {
  const epochs = trainingConfig.training_epochs
  const { training, validation } = generateLossHistory(epochs, 50, 300)

  const bestEpoch = training.reduce((best, r, i) => r.energy_rmse < training[best].energy_rmse ? i : best, 0)

  return {
    success: true,
    potential_type: trainingConfig.potential_type,
    training_loss_history: training,
    validation_loss_history: validation,
    test_energy_rmse_meV: 1.2 + Math.random() * 2.5,
    test_force_rmse_meV_A: 25 + Math.random() * 40,
    test_stress_rmse_GPa: 0.05 + Math.random() * 0.15,
    potential_file_path: `/output/potentials/${trainingConfig.potential_type}_trained_${Date.now()}.${trainingConfig.potential_type === 'nep' ? 'nep' : trainingConfig.potential_type === 'mtp' ? 'mtp' : 'txt'}`,
    training_time_seconds: 120 + Math.random() * 600,
    best_epoch: bestEpoch + 1,
  }
}

function generateMockPhaseDiagram(): DftToPfResult {
  const T_min = pfConfig.temperature_range.min
  const T_max = pfConfig.temperature_range.max
  const x_min = pfConfig.composition_range.min
  const x_max = pfConfig.composition_range.max

  // Eutectic point
  const eutecticT = T_min + (T_max - T_min) * (0.35 + Math.random() * 0.15)
  const eutecticX = x_min + (x_max - x_min) * (0.3 + Math.random() * 0.2)

  // Liquidus curves
  const liquidusLeft: Array<{ T: number; x: number }> = []
  const liquidusRight: Array<{ T: number; x: number }> = []
  const solidusLeft: Array<{ T: number; x: number }> = []
  const solidusRight: Array<{ T: number; x: number }> = []

  const numPoints = 50
  for (let i = 0; i <= numPoints; i++) {
    const T = eutecticT + (T_max - eutecticT) * (i / numPoints)
    const xL = eutecticX * Math.pow((T - eutecticT) / (T_max - eutecticT), 0.6)
    const xR = 1 - (1 - eutecticX) * Math.pow((T - eutecticT) / (T_max - eutecticT), 0.6)
    liquidusLeft.push({ T, x: x_min + (x_max - x_min) * xL })
    liquidusRight.push({ T, x: x_min + (x_max - x_min) * xR })
  }

  // Solidus (horizontal at eutectic)
  for (let i = 0; i <= 20; i++) {
    const x = x_min + (x_max - x_min) * (i / 20)
    solidusLeft.push({ T: eutecticT, x })
  }

  // Tie lines
  const tieLines: Array<{ T: number; x1: number; x2: number }> = []
  for (let i = 0; i < numPoints; i += 5) {
    tieLines.push({
      T: liquidusLeft[i].T,
      x1: liquidusLeft[i].x,
      x2: liquidusRight[i].x,
    })
  }

  // Free energy curves at multiple temperatures
  const freeEnergyCurves: FreeEnergyCurvePoint[] = []
  const temps = [800, 1000, 1200]
  for (const T of temps) {
    for (let i = 0; i <= 40; i++) {
      const x = i / 40
      const omega = 0.15 * (1 - T / 2000)
      const G = x * Math.log(Math.max(x, 1e-10)) + (1 - x) * Math.log(Math.max(1 - x, 1e-10)) + omega * x * (1 - x)
      freeEnergyCurves.push({ T, composition: x, free_energy: G })
    }
  }

  // Miscibility gap
  const miscGapTemps: number[] = []
  const xLeft: number[] = []
  const xRight: number[] = []
  for (let i = 0; i <= 30; i++) {
    const T = T_min + (T_max - T_min) * 0.3 * (i / 30)
    miscGapTemps.push(T)
    const width = 0.3 * Math.sqrt(1 - (T / (T_max * 0.3)) ** 2)
    xLeft.push(0.5 - width / 2)
    xRight.push(0.5 + width / 2)
  }

  return {
    success: true,
    gl_params: {
      free_energy_coefficients: {
        A: 0.5 + Math.random() * 0.5,
        B: 1.0 + Math.random() * 1.0,
        C: 0.3 + Math.random() * 0.3,
        'κ': 0.1 + Math.random() * 0.2,
      },
      interface_width: 2.5 + Math.random() * 3.5,
      equilibrium_order_parameter: 0.6 + Math.random() * 0.3,
      mobility: 1e-10 + Math.random() * 9e-10,
      chemical_potential: -0.5 + Math.random() * 1.0,
      temperature_range: { min: T_min, max: T_max },
    },
    phase_diagram: {
      phases: [
        { name: 'Liquid', composition_range: { min: 0, max: 1 }, temperature_range: { min: eutecticT, max: T_max }, stability: 'stable' },
        { name: 'Alpha (FCC)', composition_range: { min: 0, max: eutecticX }, temperature_range: { min: T_min, max: eutecticT }, stability: 'stable' },
        { name: 'Beta (BCC)', composition_range: { min: eutecticX, max: 1 }, temperature_range: { min: T_min, max: eutecticT }, stability: 'stable' },
      ],
      tie_lines: tieLines,
      critical_points: [],
      eutectic_points: [{ T: eutecticT, x: eutecticX }],
    },
    free_energy_curves: freeEnergyCurves,
    miscibility_gap: {
      min_T: miscGapTemps[0],
      max_T: miscGapTemps[miscGapTemps.length - 1],
      x_left: xLeft,
      x_right: xRight,
    },
    critical_temperature: T_max * 0.3 * (0.9 + Math.random() * 0.1),
  }
}

// ============ Computed ============

const energyYMaxLabel = computed(() => {
  if (!trainingResult.value) return '50'
  const maxE = Math.max(...trainingResult.value.training_loss_history.map(r => r.energy_rmse))
  return maxE.toFixed(0)
})

const forceYMaxLabel = computed(() => {
  if (!trainingResult.value) return '300'
  const maxF = Math.max(...trainingResult.value.training_loss_history.map(r => r.force_rmse))
  return maxF.toFixed(0)
})

function lossToSvgPoints(history: LossRecord[], key: 'energy_rmse' | 'force_rmse', plotL: number, plotR: number, plotT: number, plotB: number): string {
  if (history.length === 0) return ''
  const maxVal = Math.max(...history.map(r => r[key]))
  const maxEpoch = Math.max(...history.map(r => r.epoch))
  const plotW = plotR - plotL
  const plotH = plotB - plotT

  return history.map(r => {
    const x = plotL + (r.epoch / Math.max(maxEpoch, 1)) * plotW
    const y = plotB - (r[key] / Math.max(maxVal, 1e-10)) * plotH
    return `${x},${y}`
  }).join(' ')
}

const energyTrainPoints = computed(() => {
  if (!trainingResult.value) return ''
  return lossToSvgPoints(trainingResult.value.training_loss_history, 'energy_rmse', 40, 290, 20, 180)
})

const energyValPoints = computed(() => {
  if (!trainingResult.value) return ''
  return lossToSvgPoints(trainingResult.value.validation_loss_history, 'energy_rmse', 40, 290, 20, 180)
})

const forceTrainPoints = computed(() => {
  if (!trainingResult.value) return ''
  return lossToSvgPoints(trainingResult.value.training_loss_history, 'force_rmse', 40, 290, 20, 180)
})

const forceValPoints = computed(() => {
  if (!trainingResult.value) return ''
  return lossToSvgPoints(trainingResult.value.validation_loss_history, 'force_rmse', 40, 290, 20, 180)
})

const eutecticTemp = computed(() => {
  if (!pfResult.value || pfResult.value.phase_diagram.eutectic_points.length === 0) return '-'
  return pfResult.value.phase_diagram.eutectic_points[0].T.toFixed(0)
})

const eutecticComp = computed(() => {
  if (!pfResult.value || pfResult.value.phase_diagram.eutectic_points.length === 0) return '-'
  return pfResult.value.phase_diagram.eutectic_points[0].x.toFixed(3)
})

const freeEnergySvgLines = computed(() => {
  if (!pfResult.value) return []
  const curves = pfResult.value.free_energy_curves
  const temps = [800, 1000, 1200]
  const colors = ['var(--primary)', 'var(--accent-amber)', 'var(--accent-red)']
  const result: Array<{ points: string; color: string }> = []

  for (let ti = 0; ti < temps.length; ti++) {
    const tempCurves = curves.filter(c => Math.abs(c.T - temps[ti]) < 1)
    if (tempCurves.length === 0) continue

    let minF = Infinity
    let maxF = -Infinity
    for (const c of tempCurves) {
      if (c.free_energy < minF) minF = c.free_energy
      if (c.free_energy > maxF) maxF = c.free_energy
    }
    const fRange = maxF - minF || 1

    const plotL = 50
    const plotR = 390
    const plotT = 20
    const plotB = 200
    const plotW = plotR - plotL
    const plotH = plotB - plotT

    const points = tempCurves.map(c => {
      const x = plotL + c.composition * plotW
      const y = plotB - ((c.free_energy - minF) / fRange) * plotH * 0.9 - plotH * 0.05
      return `${x},${y}`
    }).join(' ')

    result.push({ points, color: colors[ti] })
  }

  return result
})

// ============ Methods ============

function formatTime(seconds: number): string {
  if (seconds < 60) return `${seconds.toFixed(0)}s`
  const m = Math.floor(seconds / 60)
  const s = Math.floor(seconds % 60)
  return `${m}m ${s}s`
}

function addPhase() {
  const name = newPhaseName.value.trim()
  if (name && !pfConfig.target_phases.includes(name)) {
    pfConfig.target_phases.push(name)
    newPhaseName.value = ''
  }
}

function removePhase(phase: string) {
  const idx = pfConfig.target_phases.indexOf(phase)
  if (idx >= 0) pfConfig.target_phases.splice(idx, 1)
}

function applyTemplate(tpl: DftBridgeTemplate) {
  const cfg = tpl.config
  if (tpl.id === 'fe_ni_nep' || tpl.id === 'cu_zr_mtp') {
    activeTab.value = 'dft_md'
    if (cfg.potential_type) trainingConfig.potential_type = cfg.potential_type as PotentialType
    if (cfg.dft_data_directory) trainingConfig.dft_data_directory = cfg.dft_data_directory as string
    if (cfg.training_set_ratio) trainingConfig.training_set_ratio = cfg.training_set_ratio as number
    if (cfg.validation_set_ratio) trainingConfig.validation_set_ratio = cfg.validation_set_ratio as number
    if (cfg.test_set_ratio) trainingConfig.test_set_ratio = cfg.test_set_ratio as number
    if (cfg.energy_weight) trainingConfig.energy_weight = cfg.energy_weight as number
    if (cfg.force_weight) trainingConfig.force_weight = cfg.force_weight as number
    if (cfg.stress_weight) trainingConfig.stress_weight = cfg.stress_weight as number
    if (cfg.max_neuron) trainingConfig.max_neuron = cfg.max_neuron as number
    if (cfg.cutoff_A) trainingConfig.cutoff_A = cfg.cutoff_A as number
    if (cfg.training_epochs) trainingConfig.training_epochs = cfg.training_epochs as number
    if (cfg.batch_size) trainingConfig.batch_size = cfg.batch_size as number
    if (cfg.learning_rate) trainingConfig.learning_rate = cfg.learning_rate as number
  } else {
    activeTab.value = 'dft_pf'
    if (cfg.dft_results_directory) pfConfig.dft_results_directory = cfg.dft_results_directory as string
    if (cfg.target_phases) pfConfig.target_phases = [...(cfg.target_phases as string[])]
    if (cfg.temperature_range) {
      const tr = cfg.temperature_range as { min: number; max: number; step: number }
      pfConfig.temperature_range.min = tr.min
      pfConfig.temperature_range.max = tr.max
      pfConfig.temperature_range.step = tr.step
    }
    if (cfg.composition_range) {
      const cr = cfg.composition_range as { min: number; max: number; step: number }
      pfConfig.composition_range.min = cr.min
      pfConfig.composition_range.max = cr.max
      pfConfig.composition_range.step = cr.step
    }
    if (cfg.free_energy_model) pfConfig.free_energy_model = cfg.free_energy_model as 'regular_solution' | 'subregular' | 'calphad'
  }
}

function resetAll() {
  activeTab.value = 'dft_md'
  trainingResult.value = null
  pfResult.value = null
  glParams.value = null
  chemicalPotentials.value = null
  runningPrepare.value = false
  runningTraining.value = false
  runningPhaseDiagram.value = false
  runningGlParams.value = false
  trainingProgress.current = 0
  trainingProgress.total = 0
  trainingProgress.percent = 0

  trainingConfig.potential_type = 'nep'
  trainingConfig.dft_data_directory = '/data/dft/results'
  trainingConfig.training_set_ratio = 0.8
  trainingConfig.validation_set_ratio = 0.1
  trainingConfig.test_set_ratio = 0.1
  trainingConfig.energy_weight = 1.0
  trainingConfig.force_weight = 50.0
  trainingConfig.stress_weight = 0.1
  trainingConfig.max_neuron = 100
  trainingConfig.cutoff_A = 6.0
  trainingConfig.training_epochs = 2000
  trainingConfig.batch_size = 32
  trainingConfig.learning_rate = 0.005

  pfConfig.dft_results_directory = '/data/dft/results'
  pfConfig.target_phases = ['Liquid', 'FCC', 'BCC']
  pfConfig.temperature_range = { min: 300, max: 1800, step: 20 }
  pfConfig.composition_range = { min: 0, max: 1, step: 0.01 }
  pfConfig.free_energy_model = 'regular_solution'
}

async function runPrepareData() {
  runningPrepare.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
  } catch (e) {
    console.error('Prepare data failed:', e)
  } finally {
    runningPrepare.value = false
  }
}

async function runTraining() {
  runningTraining.value = true
  trainingResult.value = null
  trainingProgress.total = trainingConfig.training_epochs
  trainingProgress.current = 0
  trainingProgress.percent = 0

  try {
    // Simulate progressive training
    const totalSteps = 20
    for (let i = 1; i <= totalSteps; i++) {
      await new Promise(resolve => setTimeout(resolve, 200))
      trainingProgress.current = Math.floor(trainingConfig.training_epochs * (i / totalSteps))
      trainingProgress.percent = (i / totalSteps) * 100
    }

    trainingResult.value = generateMockTrainingResult()
    trainingProgress.current = trainingConfig.training_epochs
    trainingProgress.percent = 100
  } catch (e) {
    console.error('Training failed:', e)
  } finally {
    runningTraining.value = false
  }
}

async function runValidation() {
  if (!trainingResult.value) return
  try {
    await new Promise(resolve => setTimeout(resolve, 800))
  } catch (e) {
    console.error('Validation failed:', e)
  }
}

async function runExport() {
  if (!trainingResult.value) return
  try {
    await new Promise(resolve => setTimeout(resolve, 500))
    const data = JSON.stringify(trainingResult.value, null, 2)
    const blob = new Blob([data], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `potential_${trainingResult.value.potential_type}_result.json`
    a.click()
    URL.revokeObjectURL(url)
  } catch (e) {
    console.error('Export failed:', e)
  }
}

async function runCalculatePhaseDiagram() {
  runningPhaseDiagram.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 2500))
    pfResult.value = generateMockPhaseDiagram()
    await nextTick()
    drawPhaseDiagram()
  } catch (e) {
    console.error('Phase diagram calculation failed:', e)
  } finally {
    runningPhaseDiagram.value = false
  }
}

async function runExtractGlParams() {
  runningGlParams.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    if (pfResult.value) {
      glParams.value = pfResult.value.gl_params
    } else {
      glParams.value = generateMockPhaseDiagram().gl_params
    }
  } catch (e) {
    console.error('GL params extraction failed:', e)
  } finally {
    runningGlParams.value = false
  }
}

async function runAlignChemicalPotential() {
  if (!pfResult.value) return
  try {
    await new Promise(resolve => setTimeout(resolve, 1000))
    chemicalPotentials.value = {
      Liquid: -0.35 + Math.random() * 0.1,
      FCC: -0.42 + Math.random() * 0.1,
      BCC: -0.38 + Math.random() * 0.1,
    }
  } catch (e) {
    console.error('Chemical potential alignment failed:', e)
  }
}

// ============ Phase Diagram Canvas Drawing ============

function drawPhaseDiagram() {
  const canvas = phaseDiagramCanvas.value
  if (!canvas || !pfResult.value) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const W = canvas.width
  const H = canvas.height
  const margin = { top: 30, right: 30, bottom: 50, left: 60 }
  const plotW = W - margin.left - margin.right
  const plotH = H - margin.top - margin.bottom

  const T_min = pfConfig.temperature_range.min
  const T_max = pfConfig.temperature_range.max
  const x_min = pfConfig.composition_range.min
  const x_max = pfConfig.composition_range.max

  const toX = (x: number) => margin.left + ((x - x_min) / (x_max - x_min)) * plotW
  const toY = (T: number) => margin.top + plotH - ((T - T_min) / (T_max - T_min)) * plotH

  // Clear
  ctx.fillStyle = '#FAFBFC'
  ctx.fillRect(0, 0, W, H)

  // Grid
  ctx.strokeStyle = 'rgba(0,0,0,0.06)'
  ctx.lineWidth = 0.5
  for (let i = 0; i <= 8; i++) {
    const T = T_min + (T_max - T_min) * (i / 8)
    const y = toY(T)
    ctx.beginPath(); ctx.moveTo(margin.left, y); ctx.lineTo(W - margin.right, y); ctx.stroke()
  }
  for (let i = 0; i <= 10; i++) {
    const x = x_min + (x_max - x_min) * (i / 10)
    const px = toX(x)
    ctx.beginPath(); ctx.moveTo(px, margin.top); ctx.lineTo(px, H - margin.bottom); ctx.stroke()
  }

  // Phase regions coloring
  const eutectic = pfResult.value.phase_diagram.eutectic_points[0]
  if (eutectic) {
    const eT = eutectic.T
    const eX = eutectic.x
    const ePx = toX(eX)
    const ePy = toY(eT)

    // Liquid region (above liquidus)
    ctx.fillStyle = 'rgba(37, 99, 235, 0.06)'
    ctx.beginPath()
    ctx.moveTo(toX(x_min), toY(T_max))
    ctx.lineTo(toX(x_max), toY(T_max))
    ctx.lineTo(toX(x_max), ePy)
    ctx.lineTo(ePx, ePy)
    ctx.lineTo(toX(x_min), ePy)
    ctx.closePath()
    ctx.fill()

    // Alpha + Liquid region
    ctx.fillStyle = 'rgba(34, 197, 94, 0.06)'
    ctx.beginPath()
    ctx.moveTo(toX(x_min), toY(T_min))
    ctx.lineTo(toX(x_min), ePy)
    ctx.lineTo(ePx, ePy)
    ctx.lineTo(toX(x_min), ePy)
    ctx.closePath()
    ctx.fill()

    // Beta + Liquid region
    ctx.fillStyle = 'rgba(245, 158, 11, 0.06)'
    ctx.beginPath()
    ctx.moveTo(toX(x_max), toY(T_min))
    ctx.lineTo(toX(x_max), ePy)
    ctx.lineTo(ePx, ePy)
    ctx.lineTo(toX(x_max), ePy)
    ctx.closePath()
    ctx.fill()

    // Eutectic region (below eutectic)
    ctx.fillStyle = 'rgba(168, 85, 247, 0.06)'
    ctx.beginPath()
    ctx.moveTo(toX(x_min), toY(T_min))
    ctx.lineTo(toX(x_max), toY(T_min))
    ctx.lineTo(toX(x_max), ePy)
    ctx.lineTo(ePx, ePy)
    ctx.lineTo(toX(x_min), ePy)
    ctx.closePath()
    ctx.fill()
  }

  // Liquidus curves
  const tieLines = pfResult.value.phase_diagram.tie_lines
  if (tieLines.length > 0) {
    ctx.strokeStyle = '#2563EB'
    ctx.lineWidth = 2
    ctx.beginPath()
    for (let i = 0; i < tieLines.length; i++) {
      const px = toX(tieLines[i].x1)
      const py = toY(tieLines[i].T)
      if (i === 0) ctx.moveTo(px, py)
      else ctx.lineTo(px, py)
    }
    ctx.stroke()

    ctx.beginPath()
    for (let i = 0; i < tieLines.length; i++) {
      const px = toX(tieLines[i].x2)
      const py = toY(tieLines[i].T)
      if (i === 0) ctx.moveTo(px, py)
      else ctx.lineTo(px, py)
    }
    ctx.stroke()

    // Solidus (horizontal eutectic line)
    ctx.strokeStyle = '#EF4444'
    ctx.lineWidth = 1.5
    ctx.setLineDash([6, 3])
    ctx.beginPath()
    ctx.moveTo(toX(x_min), toY(tieLines[0].T))
    ctx.lineTo(toX(x_max), toY(tieLines[0].T))
    ctx.stroke()
    ctx.setLineDash([])

    // Tie lines
    ctx.strokeStyle = 'rgba(0,0,0,0.1)'
    ctx.lineWidth = 0.5
    for (const tl of tieLines) {
      ctx.beginPath()
      ctx.moveTo(toX(tl.x1), toY(tl.T))
      ctx.lineTo(toX(tl.x2), toY(tl.T))
      ctx.stroke()
    }
  }

  // Eutectic point marker
  if (eutectic) {
    const ePx = toX(eutectic.x)
    const ePy = toY(eutectic.T)

    ctx.fillStyle = '#EF4444'
    ctx.beginPath()
    ctx.arc(ePx, ePy, 5, 0, 2 * Math.PI)
    ctx.fill()

    ctx.strokeStyle = '#EF4444'
    ctx.lineWidth = 1
    ctx.beginPath()
    ctx.arc(ePx, ePy, 8, 0, 2 * Math.PI)
    ctx.stroke()

    ctx.fillStyle = '#EF4444'
    ctx.font = '11px sans-serif'
    ctx.textAlign = 'left'
    ctx.fillText(`E (${eutectic.T.toFixed(0)}K, ${eutectic.x.toFixed(2)})`, ePx + 12, ePy - 8)
  }

  // Phase labels
  ctx.font = '12px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillStyle = '#2563EB'
  ctx.fillText('Liquid', W / 2, margin.top + 20)
  ctx.fillStyle = '#22C55E'
  ctx.fillText('Alpha + L', margin.left + plotW * 0.15, margin.top + plotH * 0.6)
  ctx.fillStyle = '#F59E0B'
  ctx.fillText('Beta + L', margin.left + plotW * 0.85, margin.top + plotH * 0.6)
  ctx.fillStyle = '#A855F7'
  ctx.fillText('Alpha + Beta', W / 2, margin.top + plotH * 0.85)

  // Axes
  ctx.strokeStyle = '#4B5563'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(margin.left, margin.top)
  ctx.lineTo(margin.left, H - margin.bottom)
  ctx.lineTo(W - margin.right, H - margin.bottom)
  ctx.stroke()

  // Axis labels
  ctx.fillStyle = '#4B5563'
  ctx.font = '11px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText('Composition (x)', W / 2, H - 8)

  ctx.save()
  ctx.translate(14, H / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Temperature (K)', 0, 0)
  ctx.restore()

  // Tick labels
  ctx.font = '9px sans-serif'
  ctx.fillStyle = '#9CA3AF'
  ctx.textAlign = 'center'
  for (let i = 0; i <= 10; i++) {
    const x = x_min + (x_max - x_min) * (i / 10)
    ctx.fillText(x.toFixed(1), toX(x), H - margin.bottom + 14)
  }
  ctx.textAlign = 'right'
  for (let i = 0; i <= 8; i++) {
    const T = T_min + (T_max - T_min) * (i / 8)
    ctx.fillText(T.toFixed(0), margin.left - 6, toY(T) + 3)
  }
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
</style>
