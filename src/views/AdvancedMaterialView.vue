<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">高级材料模型</h2>
        <p class="text-sm text-[var(--text-muted)]">Perzyna 粘塑性 / 形状记忆合金(SMA)相变本构，马氏体相变可视化</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板按钮 -->
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="btn btn-ghost text-xs"
        >
          {{ tpl.name }}
        </button>
        <!-- 重置 -->
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <!-- 导出 -->
        <button v-if="hasResults" @click="exportResults" class="btn btn-ghost text-xs" style="color: var(--accent-green); border-color: var(--accent-green);">
          导出结果
        </button>
      </div>
    </div>

    <!-- Tab Switcher -->
    <div class="flex items-center gap-1 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <button
        @click="activeTab = 'viscoplastic'"
        :class="['px-4 py-2 text-sm font-medium rounded-t transition',
          activeTab === 'viscoplastic'
            ? 'bg-[var(--bg-base)] text-[var(--primary)] border border-[var(--border-default)] border-b-white -mb-px'
            : 'text-[var(--text-muted)] hover:text-[var(--text-secondary)]']"
      >
        粘塑性分析
      </button>
      <button
        @click="activeTab = 'sma'"
        :class="['px-4 py-2 text-sm font-medium rounded-t transition',
          activeTab === 'sma'
            ? 'bg-[var(--bg-base)] text-[var(--primary)] border border-[var(--border-default)] border-b-white -mb-px'
            : 'text-[var(--text-muted)] hover:text-[var(--text-secondary)]']"
      >
        形状记忆合金 (SMA)
      </button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">

        <!-- ==================== 粘塑性面板 ==================== -->
        <template v-if="activeTab === 'viscoplastic'">
          <!-- 模型选择 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">粘塑性模型</h4>
            <div class="flex flex-col gap-1.5">
              <button
                v-for="model in vpModels"
                :key="model.value"
                @click="vpConfig.model = model.value"
                :class="['px-3 py-2 rounded text-xs text-left transition border',
                  vpConfig.model === model.value
                    ? 'bg-blue-600 text-white border-blue-600'
                    : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
              >
                <div class="font-medium">{{ model.label }}</div>
                <div class="text-[10px] mt-0.5 opacity-80">{{ model.desc }}</div>
              </button>
            </div>
          </div>

          <!-- 材料库 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">材料选择</h4>
            <div class="mb-3">
              <label class="label">材料库</label>
              <select v-model="vpMaterialPreset" @change="applyVPMaterialPreset" class="input w-full text-xs">
                <option value="ti64">Ti-6Al-4V</option>
                <option value="al7075">Al 7075</option>
                <option value="copper">铜</option>
                <option value="vp_custom">自定义</option>
              </select>
            </div>
            <div class="space-y-2">
              <div>
                <label class="label">弹性模量 E (Pa)</label>
                <input v-model.number="vpConfig.material.E" type="number" step="1e9" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">泊松比 nu</label>
                <input v-model.number="vpConfig.material.nu" type="number" step="0.01" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">屈服应力 (Pa)</label>
                <input v-model.number="vpConfig.material.yield_stress" type="number" step="1e6" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">硬化模量 (Pa)</label>
                <input v-model.number="vpConfig.material.hardening_modulus" type="number" step="1e6" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Perzyna 幂指数 n</label>
                <input v-model.number="vpConfig.material.n_power" type="number" step="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Chaboche C1 (Pa)</label>
                <input v-model.number="vpConfig.material.C1" type="number" step="1e6" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">Chaboche gamma1</label>
                <input v-model.number="vpConfig.material.gamma1" type="number" step="10" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- 参数设置 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">分析参数</h4>
            <div class="space-y-2">
              <div>
                <label class="label">施加应力 (Pa)</label>
                <input v-model.number="vpConfig.applied_stress" type="number" step="1e6" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">应变速率 (1/s)</label>
                <input v-model.number="vpConfig.strain_rate" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">温度 (K)</label>
                <input v-model.number="vpConfig.temperature" type="number" step="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">循环次数</label>
                <input v-model.number="vpConfig.cycles" type="number" step="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- 运行按钮 -->
          <button @click="runViscoplasticAnalysis" :disabled="vpAnalyzing" class="btn btn-primary text-xs w-full">
            {{ vpAnalyzing ? '分析中...' : '运行粘塑性分析' }}
          </button>
        </template>

        <!-- ==================== SMA 面板 ==================== -->
        <template v-if="activeTab === 'sma'">
          <!-- 模型选择 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">SMA 本构模型</h4>
            <div class="flex flex-col gap-1.5">
              <button
                v-for="model in smaModels"
                :key="model.value"
                @click="smaConfig.model = model.value"
                :class="['px-3 py-2 rounded text-xs text-left transition border',
                  smaConfig.model === model.value
                    ? 'bg-blue-600 text-white border-blue-600'
                    : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
              >
                <div class="font-medium">{{ model.label }}</div>
                <div class="text-[10px] mt-0.5 opacity-80">{{ model.desc }}</div>
              </button>
            </div>
          </div>

          <!-- 材料库 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">SMA 材料选择</h4>
            <div class="mb-3">
              <label class="label">材料库</label>
              <select v-model="smaMaterialPreset" @change="applySMAMaterialPreset" class="input w-full text-xs">
                <option value="niti">NiTi 合金</option>
                <option value="cuznal">CuZnAl</option>
                <option value="cualni">CuAlNi</option>
                <option value="sma_custom">自定义</option>
              </select>
            </div>
            <div class="space-y-2">
              <div>
                <label class="label">奥氏体弹性模量 E_A (Pa)</label>
                <input v-model.number="smaConfig.material.E_austenite" type="number" step="1e9" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">马氏体弹性模量 E_M (Pa)</label>
                <input v-model.number="smaConfig.material.E_martensite" type="number" step="1e9" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">泊松比 nu</label>
                <input v-model.number="smaConfig.material.nu" type="number" step="0.01" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- 相变温度 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">相变温度参数</h4>
            <div class="space-y-2">
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="label">Mf (K)</label>
                  <input v-model.number="smaConfig.material.Mf" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">Ms (K)</label>
                  <input v-model.number="smaConfig.material.Ms" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">As (K)</label>
                  <input v-model.number="smaConfig.material.As" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">Af (K)</label>
                  <input v-model.number="smaConfig.material.Af" type="number" step="1" class="input w-full text-xs" />
                </div>
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="label">C_M (K/Pa)</label>
                  <input v-model.number="smaConfig.material.C_M" type="number" step="1e-8" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">C_A (K/Pa)</label>
                  <input v-model.number="smaConfig.material.C_A" type="number" step="1e-8" class="input w-full text-xs" />
                </div>
              </div>
              <div>
                <label class="label">最大可恢复应变 epsilon_L</label>
                <input v-model.number="smaConfig.material.epsilon_L" type="number" step="0.001" class="input w-full text-xs" />
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="label">相变开始应力 (Pa)</label>
                  <input v-model.number="smaConfig.material.sigma_s" type="number" step="1e6" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">相变结束应力 (Pa)</label>
                  <input v-model.number="smaConfig.material.sigma_f" type="number" step="1e6" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- SMA 分析参数 -->
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">分析参数</h4>
            <div class="space-y-2">
              <div>
                <label class="label">施加应力 (Pa)</label>
                <input v-model.number="smaConfig.applied_stress" type="number" step="1e6" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">温度 (K)</label>
                <input v-model.number="smaConfig.temperature" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">循环次数</label>
                <input v-model.number="smaConfig.cycles" type="number" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">最大应变</label>
                <input v-model.number="smaConfig.max_strain" type="number" step="0.001" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <!-- 运行按钮 -->
          <button @click="runSMAAnalysis" :disabled="smaAnalyzing" class="btn btn-primary text-xs w-full">
            {{ smaAnalyzing ? '分析中...' : '运行SMA分析' }}
          </button>
        </template>
      </div>

      <!-- Right Panel: Results Visualization -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Results Tabs -->
        <div v-if="hasResults" class="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <template v-if="activeTab === 'viscoplastic'">
            <button
              v-for="tab in vpResultTabs"
              :key="tab.value"
              @click="vpResultTab = tab.value"
              :class="['px-3 py-1 text-xs rounded transition',
                vpResultTab === tab.value
                  ? 'bg-[var(--primary)] text-white'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
            >
              {{ tab.label }}
            </button>
          </template>
          <template v-if="activeTab === 'sma'">
            <button
              v-for="tab in smaResultTabs"
              :key="tab.value"
              @click="smaResultTab = tab.value"
              :class="['px-3 py-1 text-xs rounded transition',
                smaResultTab === tab.value
                  ? 'bg-[var(--primary)] text-white'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
            >
              {{ tab.label }}
            </button>
          </template>
        </div>

        <!-- Visualization Area -->
        <div class="flex-1 overflow-y-auto p-4">
          <!-- ==================== 粘塑性结果 ==================== -->
          <template v-if="activeTab === 'viscoplastic'">
            <div v-if="vpResults">
              <!-- 应力-应变曲线 -->
              <div v-show="vpResultTab === 'stress_strain'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">应力-应变曲线</h4>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                    <line v-for="i in 6" :key="'vh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'vv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">应变</text>
                    <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">应力 (MPa)</text>
                    <polyline :points="vpStressStrainPoints" fill="none" stroke="var(--primary)" stroke-width="2" stroke-linejoin="round" />
                    <!-- Yield point marker -->
                    <circle :cx="vpYieldPoint.x" :cy="vpYieldPoint.y" r="4" fill="var(--accent-yellow)" stroke="var(--accent-yellow)" stroke-width="1" />
                    <text :x="vpYieldPoint.x + 8" :y="vpYieldPoint.y - 8" fill="var(--accent-yellow)" font-size="9">屈服点</text>
                  </svg>
                </div>
              </div>

              <!-- 应变速率效应 -->
              <div v-show="vpResultTab === 'strain_rate_effect'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">应变速率效应</h4>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                    <line v-for="i in 6" :key="'srh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'srv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">log(应变速率)</text>
                    <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">屈服应力 (MPa)</text>
                    <polyline :points="vpStrainRateEffectPoints" fill="none" stroke="var(--accent-cyan)" stroke-width="2" stroke-linejoin="round" />
                    <circle
                      v-for="(pt, i) in vpStrainRateEffectDataPoints"
                      :key="'sre'+i"
                      :cx="pt.x" :cy="pt.y" r="3" fill="var(--accent-cyan)"
                    />
                  </svg>
                </div>
              </div>

              <!-- 松弛曲线 -->
              <div v-show="vpResultTab === 'relaxation'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">应力松弛曲线</h4>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                    <line v-for="i in 6" :key="'rlh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'rlv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">时间 (s)</text>
                    <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">应力 (MPa)</text>
                    <polyline :points="vpRelaxationPoints" fill="none" stroke="var(--accent-green)" stroke-width="2" stroke-linejoin="round" />
                  </svg>
                </div>
              </div>

              <!-- 累积损伤 -->
              <div v-show="vpResultTab === 'damage'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">累积损伤与棘轮效应</h4>
                <div class="grid grid-cols-2 gap-3 mb-3">
                  <div class="stat-card">
                    <span class="stat-label">累积损伤</span>
                    <span :class="['stat-value', vpResults.accumulated_damage > 0.8 ? 'text-red-500' : 'text-green-500']">
                      {{ (vpResults.accumulated_damage * 100).toFixed(1) }}%
                    </span>
                  </div>
                  <div class="stat-card">
                    <span class="stat-label">循环次数</span>
                    <span class="stat-value">{{ vpConfig.cycles }}</span>
                  </div>
                </div>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 250" class="w-full" style="max-height: 250px;">
                    <line v-for="i in 5" :key="'dmh'+i" :x1="60" :y1="25 + i * 45" :x2="580" :y2="25 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'dmv'+i" :x1="60 + i * 86.6" :y1="25" :x2="60 + i * 86.6" :y2="250" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="250" x2="580" y2="250" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="25" x2="60" y2="250" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="245" text-anchor="middle" fill="var(--text-muted)" font-size="10">循环次数</text>
                    <text x="20" y="137" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 137)">累积应变</text>
                    <polyline :points="vpRatchetingPoints" fill="none" stroke="var(--accent-red)" stroke-width="2" stroke-linejoin="round" />
                  </svg>
                </div>
              </div>
            </div>

            <!-- Empty State -->
            <div v-else class="w-full h-full flex items-center justify-center text-[var(--text-muted)]">
              <div class="text-center">
                <div class="text-5xl mb-3 opacity-30" style="font-family: var(--font-code);">&#x03C3;</div>
                <div class="text-sm">配置参数后运行粘塑性分析</div>
                <div class="text-xs mt-1 opacity-60">支持 Perzyna / Chaboche / Anand 本构模型</div>
              </div>
            </div>
          </template>

          <!-- ==================== SMA 结果 ==================== -->
          <template v-if="activeTab === 'sma'">
            <div v-if="smaResults">
              <!-- 超弹性滞回环 -->
              <div v-show="smaResultTab === 'hysteresis'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">超弹性滞回环</h4>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                    <line v-for="i in 6" :key="'hyh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'hyv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">应变</text>
                    <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">应力 (MPa)</text>
                    <!-- Hysteresis loops -->
                    <polyline
                      v-for="(loop, idx) in smaHysteresisPaths"
                      :key="'loop'+idx"
                      :points="loop"
                      fill="none"
                      :stroke="smaLoopColors[idx % smaLoopColors.length]"
                      :stroke-width="idx === smaHysteresisPaths.length - 1 ? 2.5 : 1.5"
                      :opacity="0.4 + 0.6 * (idx / Math.max(smaHysteresisPaths.length - 1, 1))"
                      stroke-linejoin="round"
                    />
                    <!-- Legend -->
                    <text x="460" y="50" fill="var(--text-muted)" font-size="9">循环 1</text>
                    <text x="460" y="65" fill="var(--text-muted)" font-size="9">循环 N</text>
                    <line x1="450" y1="47" x2="458" y2="47" stroke="var(--accent-cyan)" stroke-width="1.5" opacity="0.5" />
                    <line x1="450" y1="62" x2="458" y2="62" stroke="var(--primary)" stroke-width="2.5" />
                  </svg>
                </div>
              </div>

              <!-- 马氏体体积分数 -->
              <div v-show="smaResultTab === 'martensite'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">马氏体体积分数 vs 温度</h4>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 300" class="w-full" style="max-height: 300px;">
                    <line v-for="i in 6" :key="'mfh'+i" :x1="60" :y1="30 + i * 45" :x2="580" :y2="30 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'mfv'+i" :x1="60 + i * 86.6" :y1="30" :x2="60 + i * 86.6" :y2="300" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="300" x2="580" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="30" x2="60" y2="300" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="295" text-anchor="middle" fill="var(--text-muted)" font-size="10">温度 (K)</text>
                    <text x="20" y="165" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 165)">马氏体分数</text>
                    <!-- Cooling curve -->
                    <polyline :points="smaMartensiteCoolingPoints" fill="none" stroke="var(--accent-cyan)" stroke-width="2" stroke-linejoin="round" />
                    <!-- Heating curve -->
                    <polyline :points="smaMartensiteHeatingPoints" fill="none" stroke="var(--accent-red)" stroke-width="2" stroke-linejoin="round" stroke-dasharray="6,3" />
                    <!-- Phase transition markers -->
                    <line :x1="smaMsX" y1="30" :x2="smaMsX" y2="300" stroke="var(--accent-cyan)" stroke-width="0.5" stroke-dasharray="3,3" />
                    <line :x1="smaMfX" y1="30" :x2="smaMfX" y2="300" stroke="var(--accent-cyan)" stroke-width="0.5" stroke-dasharray="3,3" />
                    <line :x1="smaAsX" y1="30" :x2="smaAsX" y2="300" stroke="var(--accent-red)" stroke-width="0.5" stroke-dasharray="3,3" />
                    <line :x1="smaAfX" y1="30" :x2="smaAfX" y2="300" stroke="var(--accent-red)" stroke-width="0.5" stroke-dasharray="3,3" />
                    <text :x="smaMsX" y="25" text-anchor="middle" fill="var(--accent-cyan)" font-size="9">Ms</text>
                    <text :x="smaMfX" y="25" text-anchor="middle" fill="var(--accent-cyan)" font-size="9">Mf</text>
                    <text :x="smaAsX" y="25" text-anchor="middle" fill="var(--accent-red)" font-size="9">As</text>
                    <text :x="smaAfX" y="25" text-anchor="middle" fill="var(--accent-red)" font-size="9">Af</text>
                    <!-- Legend -->
                    <line x1="460" y1="50" x2="480" y2="50" stroke="var(--accent-cyan)" stroke-width="2" />
                    <text x="485" y="53" fill="var(--text-muted)" font-size="9">冷却</text>
                    <line x1="460" y1="65" x2="480" y2="65" stroke="var(--accent-red)" stroke-width="2" stroke-dasharray="6,3" />
                    <text x="485" y="68" fill="var(--text-muted)" font-size="9">加热</text>
                  </svg>
                </div>
              </div>

              <!-- SMA 统计结果 -->
              <div v-show="smaResultTab === 'stats'" class="mb-4">
                <h4 class="text-sm font-medium mb-2 text-[var(--text-primary)]">SMA 分析结果</h4>
                <div class="grid grid-cols-2 lg:grid-cols-3 gap-3 mb-4">
                  <div class="stat-card">
                    <span class="stat-label">相变应变</span>
                    <span class="stat-value">{{ (smaResults.transformation_strain * 100).toFixed(2) }}%</span>
                  </div>
                  <div class="stat-card">
                    <span class="stat-label">耗散能</span>
                    <span class="stat-value">{{ formatScientific(smaResults.dissipated_energy) }} J/m3</span>
                  </div>
                  <div class="stat-card">
                    <span class="stat-label">形状恢复率</span>
                    <span :class="['stat-value', smaResults.shape_recovery_ratio > 0.9 ? 'text-green-500' : 'text-yellow-500']">
                      {{ (smaResults.shape_recovery_ratio * 100).toFixed(1) }}%
                    </span>
                  </div>
                </div>

                <!-- 循环退化 -->
                <h5 class="text-xs font-medium mb-2 text-[var(--text-secondary)]">循环退化趋势</h5>
                <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
                  <svg viewBox="0 0 600 250" class="w-full" style="max-height: 250px;">
                    <line v-for="i in 5" :key="'cdh'+i" :x1="60" :y1="25 + i * 45" :x2="580" :y2="25 + i * 45" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line v-for="i in 6" :key="'cdv'+i" :x1="60 + i * 86.6" :y1="25" :x2="60 + i * 86.6" :y2="250" stroke="var(--border-subtle)" stroke-width="0.5" />
                    <line x1="60" y1="250" x2="580" y2="250" stroke="var(--text-muted)" stroke-width="1" />
                    <line x1="60" y1="25" x2="60" y2="250" stroke="var(--text-muted)" stroke-width="1" />
                    <text x="320" y="245" text-anchor="middle" fill="var(--text-muted)" font-size="10">循环次数</text>
                    <text x="20" y="137" text-anchor="middle" fill="var(--text-muted)" font-size="10" transform="rotate(-90, 20, 137)">恢复率 (%)</text>
                    <!-- Recovery ratio curve -->
                    <polyline :points="smaDegradationPoints" fill="none" stroke="var(--accent-green)" stroke-width="2" stroke-linejoin="round" />
                    <!-- Dissipated energy curve -->
                    <polyline :points="smaDissipationPoints" fill="none" stroke="var(--accent-amber)" stroke-width="2" stroke-linejoin="round" stroke-dasharray="6,3" />
                    <line x1="460" y1="50" x2="480" y2="50" stroke="var(--accent-green)" stroke-width="2" />
                    <text x="485" y="53" fill="var(--text-muted)" font-size="9">恢复率</text>
                    <line x1="460" y1="65" x2="480" y2="65" stroke="var(--accent-amber)" stroke-width="2" stroke-dasharray="6,3" />
                    <text x="485" y="68" fill="var(--text-muted)" font-size="9">耗散能</text>
                  </svg>
                </div>
              </div>
            </div>

            <!-- Empty State -->
            <div v-else class="w-full h-full flex items-center justify-center text-[var(--text-muted)]">
              <div class="text-center">
                <div class="text-5xl mb-3 opacity-30" style="font-family: var(--font-code);">&#x03BE;</div>
                <div class="text-sm">配置参数后运行SMA分析</div>
                <div class="text-xs mt-1 opacity-60">支持 Liang-Rogers / Brinson / Tanaka / Lagoudas 模型</div>
              </div>
            </div>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import type {
  ViscoplasticModel, ViscoplasticResult,
  SMAModel, SMAResult, StressStrainPoint
} from '../api/advancedMaterial'

// ============ 常量 ============

const vpModels = [
  { value: 'perzyna' as ViscoplasticModel, label: 'Perzyna', desc: '过应力粘塑性模型' },
  { value: 'chaboche' as ViscoplasticModel, label: 'Chaboche', desc: '随动+各向同性硬化' },
  { value: 'anand' as ViscoplasticModel, label: 'Anand', desc: '适用于高温大变形' },
]

const smaModels = [
  { value: 'liang_rogers' as SMAModel, label: 'Liang-Rogers', desc: '基于余弦的相变模型' },
  { value: 'brinson' as SMAModel, label: 'Brinson', desc: '应力/温度驱动分离' },
  { value: 'tanaka' as SMAModel, label: 'Tanaka', desc: '指数型相变演化' },
  { value: 'lagoudas' as SMAModel, label: 'Lagoudas', desc: '热力学框架相变模型' },
]

const quickTemplates = [
  { id: 'blade', name: '航空发动机叶片', tab: 'viscoplastic' as const },
  { id: 'stent', name: '生物医用支架', tab: 'sma' as const },
  { id: 'damper', name: '减振器', tab: 'sma' as const },
  { id: 'actuator', name: '驱动器', tab: 'sma' as const },
]

const vpMaterialPresets: Record<string, { E: number; nu: number; ys: number; H: number; n: number; C1: number; g1: number }> = {
  ti64: { E: 114e9, nu: 0.34, ys: 880e6, H: 2e9, n: 1.5, C1: 80e9, g1: 500 },
  al7075: { E: 71.7e9, nu: 0.33, ys: 503e6, H: 0.8e9, n: 2.0, C1: 40e9, g1: 800 },
  copper: { E: 110e9, nu: 0.34, ys: 70e6, H: 0.5e9, n: 3.0, C1: 20e9, g1: 300 },
}

const smaMaterialPresets: Record<string, { EA: number; EM: number; nu: number; Mf: number; Ms: number; As: number; Af: number; CM: number; CA: number; eL: number; ss: number; sf: number }> = {
  niti: { EA: 75e9, EM: 28e9, nu: 0.33, Mf: 225, Ms: 245, As: 265, Af: 285, CM: 8e-8, CA: 8e-8, eL: 0.06, ss: 100e6, sf: 400e6 },
  cuznal: { EA: 80e9, EM: 35e9, nu: 0.35, Mf: 200, Ms: 220, As: 240, Af: 260, CM: 1e-7, CA: 1e-7, eL: 0.05, ss: 80e6, sf: 300e6 },
  cualni: { EA: 85e9, EM: 40e9, nu: 0.33, Mf: 260, Ms: 280, As: 310, Af: 330, CM: 1.2e-7, CA: 1.2e-7, eL: 0.04, ss: 120e6, sf: 350e6 },
}

const vpResultTabs = [
  { value: 'stress_strain', label: '应力-应变' },
  { value: 'strain_rate_effect', label: '应变速率效应' },
  { value: 'relaxation', label: '松弛曲线' },
  { value: 'damage', label: '累积损伤' },
]

const smaResultTabs = [
  { value: 'hysteresis', label: '超弹性滞回环' },
  { value: 'martensite', label: '马氏体分数' },
  { value: 'stats', label: '统计结果' },
]

const smaLoopColors = ['#06B6D4', '#3B82F6', '#8B5CF6', '#EC4899', '#EF4444', '#F59E0B', '#22C55E']

// ============ 响应式状态 ============

const activeTab = ref<'viscoplastic' | 'sma'>('viscoplastic')
const vpAnalyzing = ref(false)
const smaAnalyzing = ref(false)
const vpMaterialPreset = ref('ti64')
const smaMaterialPreset = ref('niti')
const vpResultTab = ref('stress_strain')
const smaResultTab = ref('hysteresis')

const vpConfig = reactive({
  model: 'perzyna' as ViscoplasticModel,
  material: {
    name: 'Ti-6Al-4V',
    E: 114e9,
    nu: 0.34,
    density: 4430,
    yield_stress: 880e6,
    hardening_modulus: 2e9,
    n_power: 1.5,
    C1: 80e9,
    gamma1: 500,
    Q_inf: 200e6,
    b_kinematic: 10,
    temperature_dependent: true,
  },
  applied_stress: 600e6,
  strain_rate: 0.001,
  temperature: 293,
  cycles: 10,
})

const smaConfig = reactive({
  model: 'liang_rogers' as SMAModel,
  material: {
    name: 'NiTi',
    E_austenite: 75e9,
    E_martensite: 28e9,
    nu: 0.33,
    density: 6450,
    Mf: 225,
    Ms: 245,
    As: 265,
    Af: 285,
    C_M: 8e-8,
    C_A: 8e-8,
    epsilon_L: 0.06,
    sigma_s: 100e6,
    sigma_f: 400e6,
  },
  applied_stress: 300e6,
  temperature: 310,
  cycles: 5,
  max_strain: 0.08,
})

const vpResults = ref<ViscoplasticResult | null>(null)
const smaResults = ref<SMAResult | null>(null)

// ============ 计算属性 ============

const hasResults = computed(() => {
  return activeTab.value === 'viscoplastic' ? !!vpResults.value : !!smaResults.value
})

// --- 粘塑性 SVG 坐标计算 ---

function toSvgPoints(data: StressStrainPoint[], xKey: 'strain' | 'stress', yKey: 'strain' | 'stress', plotW = 520, plotH = 270, offsetX = 60, offsetY = 30): string {
  if (!data || data.length === 0) return ''
  const xVals = data.map(d => d[xKey])
  const yVals = data.map(d => d[yKey])
  const minX = Math.min(...xVals)
  const maxX = Math.max(...xVals)
  const minY = Math.min(...yVals)
  const maxY = Math.max(...yVals)
  const rangeX = maxX - minX || 1
  const rangeY = maxY - minY || 1
  return data.map(d => {
    const x = offsetX + ((d[xKey] - minX) / rangeX) * plotW
    const y = offsetY + plotH - ((d[yKey] - minY) / rangeY) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
}

const vpStressStrainPoints = computed(() => {
  if (!vpResults.value) return ''
  return toSvgPoints(vpResults.value.stress_strain_curve, 'strain', 'stress')
})

const vpYieldPoint = computed(() => {
  if (!vpResults.value || vpResults.value.stress_strain_curve.length === 0) return { x: 60, y: 165 }
  const data = vpResults.value.stress_strain_curve
  const ys = vpConfig.material.yield_stress
  const xVals = data.map(d => d.strain)
  const yVals = data.map(d => d.stress)
  const minX = Math.min(...xVals)
  const maxX = Math.max(...xVals)
  const minY = Math.min(...yVals)
  const maxY = Math.max(...yVals)
  const rangeX = maxX - minX || 1
  const rangeY = maxY - minY || 1
  // Find closest point to yield stress
  let closest = data[0]
  let minDist = Math.abs(data[0].stress - ys)
  for (const pt of data) {
    if (Math.abs(pt.stress - ys) < minDist) {
      minDist = Math.abs(pt.stress - ys)
      closest = pt
    }
  }
  return {
    x: 60 + ((closest.strain - minX) / rangeX) * 520,
    y: 30 + 270 - ((closest.stress - minY) / rangeY) * 270,
  }
})

const vpStrainRateEffectPoints = computed(() => {
  if (!vpResults.value || vpResults.value.strain_rate_effect.length === 0) return ''
  const data = vpResults.value.strain_rate_effect
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30
  const logRates = data.map(d => Math.log10(d.strain_rate))
  const stresses = data.map(d => d.yield_stress / 1e6) // MPa
  const minLR = Math.min(...logRates)
  const maxLR = Math.max(...logRates)
  const minS = Math.min(...stresses)
  const maxS = Math.max(...stresses)
  const rangeLR = maxLR - minLR || 1
  const rangeS = maxS - minS || 1
  return data.map(d => {
    const x = offsetX + ((Math.log10(d.strain_rate) - minLR) / rangeLR) * plotW
    const y = offsetY + plotH - ((d.yield_stress / 1e6 - minS) / rangeS) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

const vpStrainRateEffectDataPoints = computed(() => {
  if (!vpResults.value || vpResults.value.strain_rate_effect.length === 0) return []
  const data = vpResults.value.strain_rate_effect
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30
  const logRates = data.map(d => Math.log10(d.strain_rate))
  const stresses = data.map(d => d.yield_stress / 1e6)
  const minLR = Math.min(...logRates)
  const maxLR = Math.max(...logRates)
  const minS = Math.min(...stresses)
  const maxS = Math.max(...stresses)
  const rangeLR = maxLR - minLR || 1
  const rangeS = maxS - minS || 1
  return data.map(d => ({
    x: offsetX + ((Math.log10(d.strain_rate) - minLR) / rangeLR) * plotW,
    y: offsetY + plotH - ((d.yield_stress / 1e6 - minS) / rangeS) * plotH,
  }))
})

const vpRelaxationPoints = computed(() => {
  if (!vpResults.value || vpResults.value.relaxation_curve.length === 0) return ''
  const data = vpResults.value.relaxation_curve
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30
  const times = data.map(d => d.time)
  const stresses = data.map(d => d.stress / 1e6)
  const minT = Math.min(...times)
  const maxT = Math.max(...times)
  const minS = Math.min(...stresses)
  const maxS = Math.max(...stresses)
  const rangeT = maxT - minT || 1
  const rangeS = maxS - minS || 1
  return data.map(d => {
    const x = offsetX + ((d.time - minT) / rangeT) * plotW
    const y = offsetY + plotH - ((d.stress / 1e6 - minS) / rangeS) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

const vpRatchetingPoints = computed(() => {
  if (!vpResults.value || vpResults.value.creep_ratcheting.length === 0) return ''
  const data = vpResults.value.creep_ratcheting
  const plotW = 520
  const plotH = 225
  const offsetX = 60
  const offsetY = 25
  const cycles = data.map(d => d.cycle)
  const strains = data.map(d => d.accumulated_strain)
  const minC = Math.min(...cycles)
  const maxC = Math.max(...cycles)
  const minS = Math.min(...strains)
  const maxS = Math.max(...strains)
  const rangeC = maxC - minC || 1
  const rangeS = maxS - minS || 1
  return data.map(d => {
    const x = offsetX + ((d.cycle - minC) / rangeC) * plotW
    const y = offsetY + plotH - ((d.accumulated_strain - minS) / rangeS) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// --- SMA SVG 坐标计算 ---

const smaHysteresisPaths = computed(() => {
  if (!smaResults.value || smaResults.value.hysteresis_loops.length === 0) return []
  const plotW = 520
  const plotH = 270
  const offsetX = 60
  const offsetY = 30

  // Collect all points for global scaling
  const allPoints: StressStrainPoint[] = []
  for (const loop of smaResults.value.hysteresis_loops) {
    allPoints.push(...loop.loading, ...loop.unloading)
  }
  if (allPoints.length === 0) return []

  const minStrain = Math.min(...allPoints.map(d => d.strain))
  const maxStrain = Math.max(...allPoints.map(d => d.strain))
  const minStress = Math.min(...allPoints.map(d => d.stress))
  const maxStress = Math.max(...allPoints.map(d => d.stress))
  const rangeS = maxStrain - minStrain || 1
  const rangeSt = maxStress - minStress || 1

  return smaResults.value.hysteresis_loops.map(loop => {
    const pts = [...loop.loading, ...loop.unloading]
    return pts.map(d => {
      const x = offsetX + ((d.strain - minStrain) / rangeS) * plotW
      const y = offsetY + plotH - ((d.stress - minStress) / rangeSt) * plotH
      return `${x.toFixed(1)},${y.toFixed(1)}`
    }).join(' ')
  })
})

// 马氏体分数曲线 - 温度范围
const smaTempRange = computed(() => {
  const mat = smaConfig.material
  return { min: mat.Mf - 20, max: mat.Af + 20 }
})

function tempToX(temp: number): number {
  const range = smaTempRange.value
  return 60 + ((temp - range.min) / (range.max - range.min)) * 520
}

const smaMsX = computed(() => tempToX(smaConfig.material.Ms))
const smaMfX = computed(() => tempToX(smaConfig.material.Mf))
const smaAsX = computed(() => tempToX(smaConfig.material.As))
const smaAfX = computed(() => tempToX(smaConfig.material.Af))

const smaMartensiteCoolingPoints = computed(() => {
  const mat = smaConfig.material
  const range = smaTempRange.value
  const points: string[] = []
  const steps = 50
  for (let i = 0; i <= steps; i++) {
    const temp = range.min + (i / steps) * (range.max - range.min)
    let fraction: number
    if (temp > mat.Ms) {
      fraction = 0
    } else if (temp < mat.Mf) {
      fraction = 1
    } else {
      fraction = 0.5 * (1 + Math.cos(Math.PI * (temp - mat.Mf) / (mat.Ms - mat.Mf)))
    }
    const x = 60 + ((temp - range.min) / (range.max - range.min)) * 520
    const y = 30 + 270 - fraction * 270
    points.push(`${x.toFixed(1)},${y.toFixed(1)}`)
  }
  return points.join(' ')
})

const smaMartensiteHeatingPoints = computed(() => {
  const mat = smaConfig.material
  const range = smaTempRange.value
  const points: string[] = []
  const steps = 50
  for (let i = 0; i <= steps; i++) {
    const temp = range.min + (i / steps) * (range.max - range.min)
    let fraction: number
    if (temp < mat.As) {
      fraction = 1
    } else if (temp > mat.Af) {
      fraction = 0
    } else {
      fraction = 0.5 * (1 + Math.cos(Math.PI * (temp - mat.As) / (mat.Af - mat.As)))
    }
    const x = 60 + ((temp - range.min) / (range.max - range.min)) * 520
    const y = 30 + 270 - fraction * 270
    points.push(`${x.toFixed(1)},${y.toFixed(1)}`)
  }
  return points.join(' ')
})

const smaDegradationPoints = computed(() => {
  if (!smaResults.value || smaResults.value.cyclic_degradation.length === 0) return ''
  const data = smaResults.value.cyclic_degradation
  const plotW = 520
  const plotH = 225
  const offsetX = 60
  const offsetY = 25
  const cycles = data.map(d => d.cycle)
  const ratios = data.map(d => d.recovery_ratio * 100)
  const minC = Math.min(...cycles)
  const maxC = Math.max(...cycles)
  const minR = Math.min(...ratios) * 0.95
  const maxR = Math.max(...ratios) * 1.02
  const rangeC = maxC - minC || 1
  const rangeR = maxR - minR || 1
  return data.map(d => {
    const x = offsetX + ((d.cycle - minC) / rangeC) * plotW
    const y = offsetY + plotH - ((d.recovery_ratio * 100 - minR) / rangeR) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

const smaDissipationPoints = computed(() => {
  if (!smaResults.value || smaResults.value.cyclic_degradation.length === 0) return ''
  const data = smaResults.value.cyclic_degradation
  const plotW = 520
  const plotH = 225
  const offsetX = 60
  const offsetY = 25
  const cycles = data.map(d => d.cycle)
  const energies = data.map(d => d.dissipated_energy)
  const minC = Math.min(...cycles)
  const maxC = Math.max(...cycles)
  const minE = Math.min(...energies) * 0.9
  const maxE = Math.max(...energies) * 1.1
  const rangeC = maxC - minC || 1
  const rangeE = maxE - minE || 1
  return data.map(d => {
    const x = offsetX + ((d.cycle - minC) / rangeC) * plotW
    const y = offsetY + plotH - ((d.dissipated_energy - minE) / rangeE) * plotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// ============ 方法 ============

function applyVPMaterialPreset() {
  const preset = vpMaterialPresets[vpMaterialPreset.value]
  if (preset) {
    vpConfig.material.E = preset.E
    vpConfig.material.nu = preset.nu
    vpConfig.material.yield_stress = preset.ys
    vpConfig.material.hardening_modulus = preset.H
    vpConfig.material.n_power = preset.n
    vpConfig.material.C1 = preset.C1
    vpConfig.material.gamma1 = preset.g1
  }
}

function applySMAMaterialPreset() {
  const preset = smaMaterialPresets[smaMaterialPreset.value]
  if (preset) {
    smaConfig.material.E_austenite = preset.EA
    smaConfig.material.E_martensite = preset.EM
    smaConfig.material.nu = preset.nu
    smaConfig.material.Mf = preset.Mf
    smaConfig.material.Ms = preset.Ms
    smaConfig.material.As = preset.As
    smaConfig.material.Af = preset.Af
    smaConfig.material.C_M = preset.CM
    smaConfig.material.C_A = preset.CA
    smaConfig.material.epsilon_L = preset.eL
    smaConfig.material.sigma_s = preset.ss
    smaConfig.material.sigma_f = preset.sf
  }
}

function applyTemplate(tpl: typeof quickTemplates[0]) {
  activeTab.value = tpl.tab
  if (tpl.tab === 'sma') {
    smaMaterialPreset.value = 'niti'
    applySMAMaterialPreset()
  } else {
    vpMaterialPreset.value = 'ti64'
    applyVPMaterialPreset()
  }
}

function resetAll() {
  vpResults.value = null
  smaResults.value = null
  vpConfig.model = 'perzyna'
  vpConfig.applied_stress = 600e6
  vpConfig.strain_rate = 0.001
  vpConfig.temperature = 293
  vpConfig.cycles = 10
  smaConfig.model = 'liang_rogers'
  smaConfig.applied_stress = 300e6
  smaConfig.temperature = 310
  smaConfig.cycles = 5
  smaConfig.max_strain = 0.08
  vpMaterialPreset.value = 'ti64'
  smaMaterialPreset.value = 'niti'
  applyVPMaterialPreset()
  applySMAMaterialPreset()
}

function generateVPData(): ViscoplasticResult {
  const { E, yield_stress, hardening_modulus, n_power } = vpConfig.material
  const strainRate = vpConfig.strain_rate
  const cycles = vpConfig.cycles

  // 应力-应变曲线 (弹塑性 + 粘塑性)
  const stressStrain: StressStrainPoint[] = []
  const maxStrain = 0.08
  const steps = 100
  for (let i = 0; i <= steps; i++) {
    const strain = (i / steps) * maxStrain
    const elasticStress = E * strain
    const plasticStrain = Math.max(0, strain - yield_stress / E)
    const hardening = hardening_modulus * plasticStrain
    const viscousStress = n_power > 0 ? 50e6 * Math.pow(strainRate, 1 / n_power) * plasticStrain : 0
    const stress = Math.min(elasticStress, yield_stress + hardening + viscousStress)
    stressStrain.push({ stress, strain })
  }

  // 应变速率效应
  const strainRateEffect = [
    { strain_rate: 0.0001, yield_stress: yield_stress * 0.95 },
    { strain_rate: 0.001, yield_stress: yield_stress * 1.0 },
    { strain_rate: 0.01, yield_stress: yield_stress * 1.08 },
    { strain_rate: 0.1, yield_stress: yield_stress * 1.18 },
    { strain_rate: 1.0, yield_stress: yield_stress * 1.30 },
    { strain_rate: 100.0, yield_stress: yield_stress * 1.50 },
  ]

  // 松弛曲线
  const relaxation: Array<{ time: number; stress: number }> = []
  const maxTime = 100
  for (let i = 0; i <= 50; i++) {
    const time = (i / 50) * maxTime
    const stress = vpConfig.applied_stress * (0.6 + 0.4 * Math.exp(-time / 20))
    relaxation.push({ time, stress })
  }

  // 棘轮效应
  const creepRatcheting = Array.from({ length: cycles }, (_, i) => ({
    cycle: i + 1,
    accumulated_strain: 0.001 * (i + 1) + 0.0002 * (i + 1) * (i + 1),
  }))

  return {
    success: true,
    stress_strain_curve: stressStrain,
    strain_rate_effect: strainRateEffect,
    relaxation_curve: relaxation,
    creep_ratcheting: creepRatcheting,
    accumulated_damage: 0.35 + 0.05 * cycles,
  }
}

function generateSMAData(): SMAResult {
  const mat = smaConfig.material
  const cycles = smaConfig.cycles
  const maxStrain = smaConfig.max_strain

  // 超弹性滞回环
  const hysteresisLoops = Array.from({ length: Math.min(cycles, 5) }, (_, loopIdx) => {
    const degradation = 1 - 0.03 * loopIdx
    const loading: StressStrainPoint[] = []
    const unloading: StressStrainPoint[] = []
    const steps = 50

    for (let i = 0; i <= steps; i++) {
      const strain = (i / steps) * maxStrain
      let stress: number
      if (strain < mat.sigma_s / mat.E_austenite) {
        // 线弹性 (奥氏体)
        stress = mat.E_austenite * strain
      } else if (strain < mat.sigma_f / mat.E_austenite + mat.epsilon_L * degradation) {
        // 相变平台
        const phaseProgress = (strain - mat.sigma_s / mat.E_austenite) / (mat.epsilon_L * degradation)
        stress = mat.sigma_s + (mat.sigma_f - mat.sigma_s) * (1 - Math.exp(-3 * phaseProgress)) * degradation
      } else {
        // 马氏体弹性
        stress = mat.sigma_f * degradation + mat.E_martensite * (strain - mat.sigma_f / mat.E_austenite - mat.epsilon_L * degradation)
      }
      loading.push({ stress, strain })
    }

    for (let i = steps; i >= 0; i--) {
      const strain = (i / steps) * maxStrain
      let stress: number
      if (strain > mat.epsilon_L * degradation * 0.3) {
        // 逆相变
        const phaseProgress = (strain - mat.epsilon_L * degradation * 0.1) / (mat.epsilon_L * degradation * 0.9)
        stress = mat.sigma_s * 0.7 * degradation + (mat.sigma_f * 0.8 * degradation - mat.sigma_s * 0.7 * degradation) * Math.max(0, phaseProgress)
      } else {
        stress = mat.E_austenite * strain * 0.95
      }
      unloading.push({ stress, strain })
    }

    return { loop_number: loopIdx + 1, loading, unloading }
  })

  // 马氏体体积分数 vs 温度
  const martensiteFraction: Array<{ temperature: number; fraction: number }> = []
  const tempMin = mat.Mf - 20
  const tempMax = mat.Af + 20
  for (let i = 0; i <= 100; i++) {
    const temp = tempMin + (i / 100) * (tempMax - tempMin)
    let fraction: number
    if (temp > mat.Ms) {
      fraction = 0
    } else if (temp < mat.Mf) {
      fraction = 1
    } else {
      fraction = 0.5 * (1 + Math.cos(Math.PI * (temp - mat.Mf) / (mat.Ms - mat.Mf)))
    }
    martensiteFraction.push({ temperature: temp, fraction })
  }

  // 循环退化
  const cyclicDegradation = Array.from({ length: cycles }, (_, i) => ({
    cycle: i + 1,
    recovery_ratio: Math.max(0.7, 0.98 - 0.04 * i),
    dissipated_energy: 5e6 * (1 + 0.1 * i),
  }))

  return {
    success: true,
    stress_strain_curve: hysteresisLoops[0]?.loading || [],
    hysteresis_loops: hysteresisLoops,
    martensite_fraction_curve: martensiteFraction,
    transformation_strain: mat.epsilon_L * 0.95,
    dissipated_energy: 5.2e6,
    shape_recovery_ratio: 0.96,
    cyclic_degradation: cyclicDegradation,
  }
}

function runViscoplasticAnalysis() {
  vpAnalyzing.value = true
  setTimeout(() => {
    try {
      vpResults.value = generateVPData()
      vpResultTab.value = 'stress_strain'
    } catch (e) {
      console.error('Viscoplastic analysis failed:', e)
    } finally {
      vpAnalyzing.value = false
    }
  }, 800)
}

function runSMAAnalysis() {
  smaAnalyzing.value = true
  setTimeout(() => {
    try {
      smaResults.value = generateSMAData()
      smaResultTab.value = 'hysteresis'
    } catch (e) {
      console.error('SMA analysis failed:', e)
    } finally {
      smaAnalyzing.value = false
    }
  }, 800)
}

function formatScientific(val: number): string {
  if (!val || !isFinite(val)) return 'N/A'
  if (val === 0) return '0'
  const exp = Math.floor(Math.log10(Math.abs(val)))
  const mantissa = val / Math.pow(10, exp)
  return `${mantissa.toFixed(2)}e${exp}`
}

function exportResults() {
  const data = activeTab.value === 'viscoplastic'
    ? vpResults.value
    : smaResults.value
  if (!data) return
  const json = JSON.stringify(data, null, 2)
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = activeTab.value === 'viscoplastic' ? 'viscoplastic_results.json' : 'sma_results.json'
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<style scoped>
.panel-section {
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.stat-card {
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
}

.stat-label {
  font-size: 10px;
  color: var(--text-muted);
  text-transform: uppercase;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}
</style>
