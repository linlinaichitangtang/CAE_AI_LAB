<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">DFT 后处理与可视化</h2>
        <p class="text-sm" style="color: var(--text-muted)">能带/DOS/电荷密度，能量/力验证，标准算例对比</p>
      </div>
      <div class="flex items-center gap-2">
        <button
          v-for="tpl in quickTemplates"
          :key="tpl.id"
          @click="applyTemplate(tpl)"
          class="px-3 py-1.5 text-xs border rounded transition"
          style="border-color: var(--border-default); background: var(--bg-elevated); color: var(--text-secondary)"
        >
          {{ tpl.name }}
        </button>
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <button v-if="hasResults" @click="exportResults" class="btn btn-ghost text-xs" style="color: var(--accent-green); border-color: var(--accent-green);">
          导出结果
        </button>
      </div>
    </div>

    <!-- Tab Bar -->
    <div class="flex items-center gap-1 px-4 py-2 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        @click="activeTab = tab.value"
        class="px-4 py-1.5 text-xs rounded-full transition font-medium"
        :style="activeTab === tab.value
          ? 'background: var(--primary); color: white; box-shadow: 0 0 12px var(--primary-glow)'
          : 'background: var(--bg-elevated); color: var(--text-secondary)'"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">

      <!-- Left Panel: Parameters -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Band Structure Panel -->
        <template v-if="activeTab === 'band'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">文件加载</h4>
            <div class="space-y-2">
              <div>
                <label class="label">能带文件</label>
                <select v-model="bandParams.fileType" class="input w-full text-xs">
                  <option value="EIGENVAL">EIGENVAL</option>
                  <option value="PROCAR">PROCAR</option>
                </select>
              </div>
              <div>
                <label class="label">文件路径</label>
                <input v-model="bandParams.filepath" type="text" class="input w-full text-xs" placeholder="/path/to/EIGENVAL" />
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">显示设置</h4>
            <div class="space-y-2">
              <div>
                <label class="label">能量范围 (eV)</label>
                <div class="grid grid-cols-2 gap-2">
                  <input v-model.number="bandParams.eMin" type="number" step="0.5" class="input w-full text-xs" placeholder="最小" />
                  <input v-model.number="bandParams.eMax" type="number" step="0.5" class="input w-full text-xs" placeholder="最大" />
                </div>
              </div>
              <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="bandParams.showFermi" class="rounded" />
                显示费米能级
              </label>
              <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="bandParams.showGap" class="rounded" />
                标注带隙
              </label>
            </div>
          </div>
          <button @click="loadBandStructure" class="btn btn-primary text-xs w-full">加载能带结构</button>
        </template>

        <!-- DOS Panel -->
        <template v-if="activeTab === 'dos'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">文件加载</h4>
            <div class="space-y-2">
              <div>
                <label class="label">DOS 文件</label>
                <select v-model="dosParams.fileType" class="input w-full text-xs">
                  <option value="DOSCAR">DOSCAR (VASP)</option>
                  <option value="pdos">Projector DOS</option>
                </select>
              </div>
              <div>
                <label class="label">文件路径</label>
                <input v-model="dosParams.filepath" type="text" class="input w-full text-xs" placeholder="/path/to/DOSCAR" />
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">显示设置</h4>
            <div class="space-y-2">
              <div>
                <label class="label">能量范围 (eV)</label>
                <div class="grid grid-cols-2 gap-2">
                  <input v-model.number="dosParams.eMin" type="number" step="0.5" class="input w-full text-xs" placeholder="最小" />
                  <input v-model.number="dosParams.eMax" type="number" step="0.5" class="input w-full text-xs" placeholder="最大" />
                </div>
              </div>
              <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="dosParams.showFermi" class="rounded" />
                显示费米能级
              </label>
              <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="dosParams.showIntegrated" class="rounded" />
                显示积分态密度
              </label>
            </div>
          </div>
          <button @click="loadDos" class="btn btn-primary text-xs w-full">加载态密度</button>
        </template>

        <!-- Energy/Force Panel -->
        <template v-if="activeTab === 'energy'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">文件加载</h4>
            <div class="space-y-2">
              <div>
                <label class="label">输出文件</label>
                <select v-model="energyParams.fileType" class="input w-full text-xs">
                  <option value="OUTCAR">OUTCAR (VASP)</option>
                  <option value="qe_out">QE Output</option>
                </select>
              </div>
              <div>
                <label class="label">文件路径</label>
                <input v-model="energyParams.filepath" type="text" class="input w-full text-xs" placeholder="/path/to/OUTCAR" />
              </div>
            </div>
          </div>
          <button @click="loadEnergyForce" class="btn btn-primary text-xs w-full">加载能量/力</button>
        </template>

        <!-- Validation Panel -->
        <template v-if="activeTab === 'validation'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">标准算例列表</h4>
            <div class="space-y-1">
              <div v-for="tc in validationTestCases" :key="tc.id"
                class="flex items-center gap-2 px-3 py-2 rounded text-xs border cursor-pointer transition"
                :style="selectedTestCase === tc.id
                  ? 'background: var(--primary); border-color: var(--primary); color: white'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                @click="selectedTestCase = tc.id"
              >
                <div>
                  <div class="font-medium">{{ tc.name }}</div>
                  <div class="text-[10px] opacity-70">{{ tc.description }}</div>
                </div>
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">容差设置</h4>
            <div>
              <label class="label">容差 (meV/atom)</label>
              <input v-model.number="validationTolerance" type="number" step="1" min="0.1" max="100" class="input w-full text-xs" />
            </div>
          </div>
          <button @click="runValidationSuite" class="btn btn-primary text-xs w-full" :disabled="validationRunning">
            {{ validationRunning ? '验证中...' : '运行验证' }}
          </button>
        </template>

        <!-- Charge Density Panel -->
        <template v-if="activeTab === 'chargedensity'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">文件加载</h4>
            <div class="space-y-2">
              <div>
                <label class="label">电荷密度文件</label>
                <select v-model="chargeParams.fileType" class="input w-full text-xs">
                  <option value="CHGCAR">CHGCAR</option>
                  <option value="PARCHG">PARCHG</option>
                </select>
              </div>
              <div>
                <label class="label">文件路径</label>
                <input v-model="chargeParams.filepath" type="text" class="input w-full text-xs" placeholder="/path/to/CHGCAR" />
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">切片设置</h4>
            <div class="space-y-2">
              <div>
                <label class="label">切片方向</label>
                <select v-model="chargeParams.sliceAxis" class="input w-full text-xs">
                  <option value="z">Z 轴</option>
                  <option value="y">Y 轴</option>
                  <option value="x">X 轴</option>
                </select>
              </div>
              <div>
                <label class="label">切片位置: {{ chargeParams.sliceIndex }}</label>
                <input v-model.number="chargeParams.sliceIndex" type="range" min="0" max="49" step="1" class="w-full" />
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">等值面设置</h4>
            <div class="space-y-2">
              <div>
                <label class="label">等值面值: {{ chargeParams.isosurfaceValue.toFixed(3) }}</label>
                <input v-model.number="chargeParams.isosurfaceValue" type="range" min="0" max="1" step="0.01" class="w-full" />
              </div>
              <label class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="chargeParams.showContour" class="rounded" />
                显示等值线
              </label>
            </div>
          </div>
          <button @click="loadChargeDensity" class="btn btn-primary text-xs w-full">加载电荷密度</button>
        </template>

      </div>

      <!-- Right Panel: Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" style="background: var(--bg-base)">

        <!-- Band Structure Results -->
        <template v-if="activeTab === 'band'">
          <div v-if="bandData" class="space-y-4">
            <!-- Band Structure SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">能带结构 E(k)</h4>
              <svg :viewBox="`0 0 ${bandSvgW} ${bandSvgH}`" class="w-full" style="max-height: 420px">
                <!-- Grid lines -->
                <line v-for="i in 6" :key="'bh'+i"
                  :x1="bandPadL" :y1="bandPadT + (bandPlotH / 6) * (i - 1)"
                  :x2="bandPadL + bandPlotW" :y2="bandPadT + (bandPlotH / 6) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="bandPadL" :y1="bandPadT" :x2="bandPadL" :y2="bandPadT + bandPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="bandPadL" :y1="bandPadT + bandPlotH" :x2="bandPadL + bandPlotW" :y2="bandPadT + bandPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- Fermi level -->
                <line v-if="bandParams.showFermi"
                  :x1="bandPadL" :y1="bandFermiY"
                  :x2="bandPadL + bandPlotW" :y2="bandFermiY"
                  stroke="var(--accent-red)" stroke-width="1.5" stroke-dasharray="6,3" />
                <text v-if="bandParams.showFermi"
                  :x="bandPadL + bandPlotW - 4" :y="bandFermiY - 4"
                  text-anchor="end" fill="var(--accent-red)" font-size="9">E_F = {{ bandData.fermi_energy_eV.toFixed(2) }} eV</text>
                <!-- High symmetry vertical lines -->
                <line v-for="(hs, idx) in bandHighSymX" :key="'hs'+idx"
                  :x1="bandPadL + hs.x" :y1="bandPadT"
                  :x2="bandPadL + hs.x" :y2="bandPadT + bandPlotH"
                  stroke="var(--text-muted)" stroke-width="0.8" stroke-dasharray="2,2" />
                <text v-for="(hs, idx) in bandHighSymX" :key="'hsl'+idx"
                  :x="bandPadL + hs.x" :y="bandPadT + bandPlotH + 16"
                  text-anchor="middle" fill="var(--text-muted)" font-size="10">{{ hs.label }}</text>
                <!-- Band lines -->
                <polyline
                  v-for="(band, bIdx) in bandData.bands"
                  :key="'b'+bIdx"
                  :points="bandCurvePoints(bIdx)"
                  fill="none"
                  :stroke="bIdx < 3 ? 'var(--primary)' : 'var(--accent-green)'"
                  stroke-width="1.8"
                  stroke-linejoin="round"
                  opacity="0.85"
                />
                <!-- Band gap annotation -->
                <template v-if="bandParams.showGap && bandData.band_gap_eV > 0">
                  <line
                    :x1="bandPadL + bandPlotW * 0.3" :y1="bandGapTopY"
                    :x2="bandPadL + bandPlotW * 0.3" :y2="bandGapBotY"
                    stroke="var(--accent-yellow)" stroke-width="2" />
                  <line
                    :x1="bandPadL + bandPlotW * 0.3 - 5" :y1="bandGapTopY"
                    :x2="bandPadL + bandPlotW * 0.3 + 5" :y2="bandGapTopY"
                    stroke="var(--accent-yellow)" stroke-width="2" />
                  <line
                    :x1="bandPadL + bandPlotW * 0.3 - 5" :y1="bandGapBotY"
                    :x2="bandPadL + bandPlotW * 0.3 + 5" :y2="bandGapBotY"
                    stroke="var(--accent-yellow)" stroke-width="2" />
                  <text
                    :x="bandPadL + bandPlotW * 0.3 + 10" :y="(bandGapTopY + bandGapBotY) / 2 + 3"
                    fill="var(--accent-yellow)" font-size="10" font-weight="600">
                    {{ bandData.band_gap_eV.toFixed(2) }} eV
                  </text>
                </template>
                <!-- Axis labels -->
                <text :x="bandPadL + bandPlotW / 2" :y="bandSvgH - 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">k-path</text>
                <text :x="10" :y="bandPadT + bandPlotH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 10, 60)">E (eV)</text>
                <!-- Y ticks -->
                <text v-for="i in 7" :key="'byt'+i"
                  :x="bandPadL - 4" :y="bandPadT + (bandPlotH / 6) * (6 - (i - 1)) + 3"
                  text-anchor="end" fill="var(--text-muted)" font-size="8">
                  {{ (bandEMin + (bandERange / 6) * (i - 1)).toFixed(1) }}
                </text>
                <!-- Legend -->
                <line x1="bandPadL + 10" :y1="bandPadT + 12" :x2="bandPadL + 30" :y2="bandPadT + 12"
                  stroke="var(--primary)" stroke-width="2" />
                <text :x="bandPadL + 34" :y="bandPadT + 15" fill="var(--text-secondary)" font-size="9">价带</text>
                <line x1="bandPadL + 10" :y1="bandPadT + 26" :x2="bandPadL + 30" :y2="bandPadT + 26"
                  stroke="var(--accent-green)" stroke-width="2" />
                <text :x="bandPadL + 34" :y="bandPadT + 29" fill="var(--text-secondary)" font-size="9">导带</text>
              </svg>
            </div>
            <!-- Result Cards -->
            <div class="grid grid-cols-5 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">带隙</div>
                <div class="text-lg font-semibold" style="color: var(--accent-yellow)">{{ bandData.band_gap_eV.toFixed(3) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">直接带隙</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ bandData.direct_gap_eV.toFixed(3) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">间接带隙</div>
                <div class="text-lg font-semibold" style="color: var(--accent-green)">{{ bandData.indirect_gap_eV.toFixed(3) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">费米能级</div>
                <div class="text-lg font-semibold" style="color: var(--accent-red)">{{ bandData.fermi_energy_eV.toFixed(3) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">能带数</div>
                <div class="text-lg font-semibold" style="color: var(--text-primary)">{{ bandData.num_bands }}</div>
              </div>
            </div>
            <!-- High Symmetry Points -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">高对称点</h4>
              <div class="space-y-1">
                <div v-for="(hs, idx) in bandData.high_symmetry_points" :key="'hsp'+idx"
                  class="flex items-center justify-between text-xs px-2 py-1 rounded" style="background: var(--bg-elevated)">
                  <span style="color: var(--text-secondary)">{{ hs.label }}</span>
                  <span style="color: var(--text-primary)">{{ hs.energy.toFixed(4) }} eV</span>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">选择文件后点击"加载能带结构"查看结果</p>
          </div>
        </template>

        <!-- DOS Results -->
        <template v-if="activeTab === 'dos'">
          <div v-if="dosData" class="space-y-4">
            <!-- Total DOS SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">总态密度 (TDOS)</h4>
              <svg :viewBox="`0 0 ${dosSvgW} ${dosSvgH}`" class="w-full" style="max-height: 360px">
                <!-- Grid -->
                <line v-for="i in 6" :key="'dh'+i"
                  :x1="dosPadL" :y1="dosPadT + (dosPlotH / 6) * (i - 1)"
                  :x2="dosPadL + dosPlotW" :y2="dosPadT + (dosPlotH / 6) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="dosPadL" :y1="dosPadT" :x2="dosPadL" :y2="dosPadT + dosPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="dosPadL" :y1="dosPadT + dosPlotH" :x2="dosPadL + dosPlotW" :y2="dosPadT + dosPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- Fermi level -->
                <line v-if="dosParams.showFermi"
                  :x1="dosPadL" :y1="dosFermiY"
                  :x2="dosPadL + dosPlotW" :y2="dosFermiY"
                  stroke="var(--accent-red)" stroke-width="1.5" stroke-dasharray="6,3" />
                <text v-if="dosParams.showFermi"
                  :x="dosPadL + dosPlotW - 4" :y="dosFermiY - 4"
                  text-anchor="end" fill="var(--accent-red)" font-size="9">E_F</text>
                <!-- TDOS curve -->
                <polyline
                  :points="dosTotalCurvePoints"
                  fill="none"
                  stroke="var(--primary)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Axis labels -->
                <text :x="dosPadL + dosPlotW / 2" :y="dosSvgH - 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">E (eV)</text>
                <text :x="10" :y="dosPadT + dosPlotH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 10, 60)">DOS (states/eV)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'dxt'+i"
                  :x="dosPadL + (dosPlotW / 5) * (i - 1)" :y="dosPadT + dosPlotH + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ (dosEMin + (dosERange / 5) * (i - 1)).toFixed(1) }}
                </text>
                <!-- Y ticks -->
                <text v-for="i in 6" :key="'dyt'+i"
                  :x="dosPadL - 4" :y="dosPadT + (dosPlotH / 5) * (5 - (i - 1)) + 3"
                  text-anchor="end" fill="var(--text-muted)" font-size="8">
                  {{ ((dosYMax / 5) * (i - 1)).toFixed(1) }}
                </text>
              </svg>
            </div>
            <!-- PDOS SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">分波态密度 (PDOS)</h4>
              <svg :viewBox="`0 0 ${dosSvgW} ${dosSvgH}`" class="w-full" style="max-height: 360px">
                <!-- Grid -->
                <line v-for="i in 6" :key="'ph'+i"
                  :x1="dosPadL" :y1="dosPadT + (dosPlotH / 6) * (i - 1)"
                  :x2="dosPadL + dosPlotW" :y2="dosPadT + (dosPlotH / 6) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="dosPadL" :y1="dosPadT" :x2="dosPadL" :y2="dosPadT + dosPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="dosPadL" :y1="dosPadT + dosPlotH" :x2="dosPadL + dosPlotW" :y2="dosPadT + dosPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- Fermi level -->
                <line v-if="dosParams.showFermi"
                  :x1="dosPadL" :y1="dosFermiY"
                  :x2="dosPadL + dosPlotW" :y2="dosFermiY"
                  stroke="var(--accent-red)" stroke-width="1.5" stroke-dasharray="6,3" />
                <!-- PDOS curves -->
                <polyline
                  v-for="(pdos, pIdx) in dosData.partial_dos"
                  :key="'pd'+pIdx"
                  :points="pdosCurvePoints(pIdx)"
                  fill="none"
                  :stroke="pdosColors[pIdx % pdosColors.length]"
                  stroke-width="1.8"
                  stroke-linejoin="round"
                  opacity="0.85"
                />
                <!-- Legend -->
                <rect v-for="(pdos, pIdx) in dosData.partial_dos" :key="'pl'+pIdx"
                  :x="dosPadL + 10 + pIdx * 100" :y="dosPadT + 6"
                  width="12" height="12" rx="2"
                  :fill="pdosColors[pIdx % pdosColors.length]" opacity="0.85" />
                <text v-for="(pdos, pIdx) in dosData.partial_dos" :key="'plt'+pIdx"
                  :x="dosPadL + 26 + pIdx * 100" :y="dosPadT + 16"
                  fill="var(--text-secondary)" font-size="9">{{ pdos.element }}-{{ pdos.orbital }}</text>
                <!-- Axis labels -->
                <text :x="dosPadL + dosPlotW / 2" :y="dosSvgH - 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">E (eV)</text>
                <text :x="10" :y="dosPadT + dosPlotH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 10, 60)">PDOS (states/eV)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'pxt'+i"
                  :x="dosPadL + (dosPlotW / 5) * (i - 1)" :y="dosPadT + dosPlotH + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ (dosEMin + (dosERange / 5) * (i - 1)).toFixed(1) }}
                </text>
              </svg>
            </div>
            <!-- Integrated DOS SVG -->
            <div v-if="dosParams.showIntegrated" class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">积分态密度</h4>
              <svg :viewBox="`0 0 ${dosSvgW} ${dosSvgH}`" class="w-full" style="max-height: 360px">
                <!-- Grid -->
                <line v-for="i in 6" :key="'ih'+i"
                  :x1="dosPadL" :y1="dosPadT + (dosPlotH / 6) * (i - 1)"
                  :x2="dosPadL + dosPlotW" :y2="dosPadT + (dosPlotH / 6) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="dosPadL" :y1="dosPadT" :x2="dosPadL" :y2="dosPadT + dosPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="dosPadL" :y1="dosPadT + dosPlotH" :x2="dosPadL + dosPlotW" :y2="dosPadT + dosPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- Fermi level -->
                <line v-if="dosParams.showFermi"
                  :x1="dosPadL" :y1="dosFermiY"
                  :x2="dosPadL + dosPlotW" :y2="dosFermiY"
                  stroke="var(--accent-red)" stroke-width="1.5" stroke-dasharray="6,3" />
                <!-- Integrated DOS curve -->
                <polyline
                  :points="dosIntegratedCurvePoints"
                  fill="none"
                  stroke="var(--accent-yellow)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Axis labels -->
                <text :x="dosPadL + dosPlotW / 2" :y="dosSvgH - 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">E (eV)</text>
                <text :x="10" :y="dosPadT + dosPlotH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 10, 60)">N(E)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'ixt'+i"
                  :x="dosPadL + (dosPlotW / 5) * (i - 1)" :y="dosPadT + dosPlotH + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ (dosEMin + (dosERange / 5) * (i - 1)).toFixed(1) }}
                </text>
              </svg>
            </div>
            <!-- Result Cards -->
            <div class="grid grid-cols-2 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">费米能级</div>
                <div class="text-lg font-semibold" style="color: var(--accent-red)">{{ dosData.fermi_energy_eV.toFixed(3) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">DOS(E_F)</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ dosAtFermi.toFixed(3) }} states/eV</div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">选择文件后点击"加载态密度"查看结果</p>
          </div>
        </template>

        <!-- Energy/Force Results -->
        <template v-if="activeTab === 'energy'">
          <div v-if="energyData" class="space-y-4">
            <!-- Energy Convergence SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">能量收敛曲线</h4>
              <svg :viewBox="`0 0 ${energySvgW} ${energySvgH}`" class="w-full" style="max-height: 360px">
                <!-- Grid -->
                <line v-for="i in 5" :key="'eh'+i"
                  :x1="energyPadL" :y1="energyPadT + (energyPlotH / 5) * (i - 1)"
                  :x2="energyPadL + energyPlotW" :y2="energyPadT + (energyPlotH / 5) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="energyPadL" :y1="energyPadT" :x2="energyPadL" :y2="energyPadT + energyPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="energyPadL" :y1="energyPadT + energyPlotH" :x2="energyPadL + energyPlotW" :y2="energyPadT + energyPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- Energy curve -->
                <polyline
                  :points="energyConvergencePoints"
                  fill="none"
                  stroke="var(--primary)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Drift curve -->
                <polyline
                  :points="energyDriftPoints"
                  fill="none"
                  stroke="var(--accent-red)"
                  stroke-width="1.5"
                  stroke-dasharray="4,3"
                />
                <!-- Legend -->
                <line x1="energyPadL + 10" :y1="energyPadT + 12" :x2="energyPadL + 30" :y2="energyPadT + 12"
                  stroke="var(--primary)" stroke-width="2" />
                <text :x="energyPadL + 34" :y="energyPadT + 15" fill="var(--text-secondary)" font-size="9">总能量</text>
                <line x1="energyPadL + 10" :y1="energyPadT + 26" :x2="energyPadL + 30" :y2="energyPadT + 26"
                  stroke="var(--accent-red)" stroke-width="1.5" stroke-dasharray="4,3" />
                <text :x="energyPadL + 34" :y="energyPadT + 29" fill="var(--text-secondary)" font-size="9">能量漂移</text>
                <!-- Axis labels -->
                <text :x="energyPadL + energyPlotW / 2" :y="energySvgH - 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">Ionic Step</text>
                <text :x="10" :y="energyPadT + energyPlotH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 10, 60)">E (eV)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'ext'+i"
                  :x="energyPadL + (energyPlotW / 5) * (i - 1)" :y="energyPadT + energyPlotH + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ Math.round((energyData.ionic_steps.length / 5) * (i - 1)) }}
                </text>
              </svg>
            </div>
            <!-- Force Convergence SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">力收敛曲线</h4>
              <svg :viewBox="`0 0 ${energySvgW} ${energySvgH}`" class="w-full" style="max-height: 360px">
                <!-- Grid -->
                <line v-for="i in 5" :key="'fh'+i"
                  :x1="energyPadL" :y1="energyPadT + (energyPlotH / 5) * (i - 1)"
                  :x2="energyPadL + energyPlotW" :y2="energyPadT + (energyPlotH / 5) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="energyPadL" :y1="energyPadT" :x2="energyPadL" :y2="energyPadT + energyPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="energyPadL" :y1="energyPadT + energyPlotH" :x2="energyPadL + energyPlotW" :y2="energyPadT + energyPlotH"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- Max force curve -->
                <polyline
                  :points="forceConvergencePoints"
                  fill="none"
                  stroke="var(--accent-green)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Axis labels -->
                <text :x="energyPadL + energyPlotW / 2" :y="energySvgH - 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">Ionic Step</text>
                <text :x="10" :y="energyPadT + energyPlotH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 10, 60)">Max Force (eV/A)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'fxt'+i"
                  :x="energyPadL + (energyPlotW / 5) * (i - 1)" :y="energyPadT + energyPlotH + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ Math.round((energyData.ionic_steps.length / 5) * (i - 1)) }}
                </text>
              </svg>
            </div>
            <!-- Result Cards -->
            <div class="grid grid-cols-3 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">总能量</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ energyData.total_energy_eV.toFixed(4) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">每原子能量</div>
                <div class="text-lg font-semibold" style="color: var(--accent-yellow)">{{ energyData.energy_per_atom_eV.toFixed(4) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">自由能</div>
                <div class="text-lg font-semibold" style="color: var(--accent-green)">{{ energyData.free_energy_eV.toFixed(4) }} eV</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">最大力</div>
                <div class="text-lg font-semibold" style="color: var(--accent-red)">{{ forcesData.max_force.toFixed(4) }} eV/A</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">RMS 力</div>
                <div class="text-lg font-semibold" style="color: var(--text-primary)">{{ forcesData.rms_force.toFixed(4) }} eV/A</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">离子步数</div>
                <div class="text-lg font-semibold" style="color: var(--text-primary)">{{ energyData.ionic_steps.length }}</div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">选择文件后点击"加载能量/力"查看结果</p>
          </div>
        </template>

        <!-- Validation Results -->
        <template v-if="activeTab === 'validation'">
          <div v-if="validationResults.length > 0" class="space-y-4">
            <!-- Summary Cards -->
            <div class="grid grid-cols-2 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">通过率</div>
                <div class="text-lg font-semibold" :style="{ color: validationPassRate >= 80 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                  {{ validationPassRate.toFixed(0) }}%
                </div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">平均误差</div>
                <div class="text-lg font-semibold" :style="{ color: validationAvgError < 5 ? 'var(--accent-green)' : 'var(--accent-yellow)' }">
                  {{ validationAvgError.toFixed(2) }} meV/atom
                </div>
              </div>
            </div>
            <!-- Results Table -->
            <div class="rounded-lg border overflow-hidden" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <table class="w-full text-xs">
                <thead>
                  <tr style="background: var(--bg-elevated)">
                    <th class="px-4 py-2 text-left font-medium" style="color: var(--text-secondary)">算例</th>
                    <th class="px-4 py-2 text-right font-medium" style="color: var(--text-secondary)">参考能量 (eV)</th>
                    <th class="px-4 py-2 text-right font-medium" style="color: var(--text-secondary)">计算能量 (eV)</th>
                    <th class="px-4 py-2 text-right font-medium" style="color: var(--text-secondary)">误差 (meV/atom)</th>
                    <th class="px-4 py-2 text-center font-medium" style="color: var(--text-secondary)">状态</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(vr, idx) in validationResults" :key="'vr'+idx"
                    class="border-t" style="border-color: var(--border-subtle)">
                    <td class="px-4 py-2" style="color: var(--text-primary)">{{ vr.test_name }}</td>
                    <td class="px-4 py-2 text-right" style="color: var(--text-secondary)">{{ vr.reference_energy_eV.toFixed(6) }}</td>
                    <td class="px-4 py-2 text-right" style="color: var(--text-secondary)">{{ vr.calculated_energy_eV.toFixed(6) }}</td>
                    <td class="px-4 py-2 text-right" :style="{ color: Math.abs(vr.error_meV_per_atom) < vr.tolerance_meV ? 'var(--accent-green)' : 'var(--accent-red)' }">
                      {{ vr.error_meV_per_atom.toFixed(2) }}
                    </td>
                    <td class="px-4 py-2 text-center">
                      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-medium"
                        :style="{
                          background: vr.passed ? 'rgba(34,197,94,0.15)' : 'rgba(239,68,68,0.15)',
                          color: vr.passed ? 'var(--accent-green)' : 'var(--accent-red)'
                        }">
                        {{ vr.passed ? 'PASS' : 'FAIL' }}
                      </span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">选择标准算例后点击"运行验证"查看结果</p>
          </div>
        </template>

        <!-- Charge Density Results -->
        <template v-if="activeTab === 'chargedensity'">
          <div v-if="chargeDensityData" class="space-y-4">
            <!-- 2D Slice Heatmap -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-medium" style="color: var(--text-primary)">
                  电荷密度切片 ({{ chargeParams.sliceAxis.toUpperCase() }} = {{ chargeParams.sliceIndex }})
                </h4>
              </div>
              <canvas ref="chargeCanvas" width="480" height="480" class="w-full rounded border" style="border-color: var(--border-subtle); max-width: 480px; image-rendering: pixelated;"></canvas>
              <!-- Color Legend -->
              <div class="flex items-center gap-2 mt-2">
                <span class="text-[10px]" style="color: var(--text-muted)">{{ chargeMinVal.toFixed(4) }}</span>
                <div class="flex-1 h-3 rounded" :style="{ background: 'linear-gradient(to right, #1e3a5f, #3b82f6, #22c55e, #eab308, #ef4444)' }"></div>
                <span class="text-[10px]" style="color: var(--text-muted)">{{ chargeMaxVal.toFixed(4) }}</span>
              </div>
            </div>
            <!-- Isosurface Values -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">等值面值</h4>
              <div class="flex flex-wrap gap-2">
                <span v-for="(val, idx) in chargeDensityData.isosurface_values" :key="'iso'+idx"
                  class="px-2 py-1 rounded text-xs cursor-pointer transition"
                  :style="Math.abs(chargeParams.isosurfaceValue - val) < 0.005
                    ? 'background: var(--primary); color: white'
                    : 'background: var(--bg-elevated); color: var(--text-secondary)'"
                  @click="chargeParams.isosurfaceValue = val">
                  {{ val.toFixed(3) }}
                </span>
              </div>
            </div>
            <!-- Grid Info -->
            <div class="grid grid-cols-3 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">网格 Nx</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ chargeDensityData.grid.nx }}</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">网格 Ny</div>
                <div class="text-lg font-semibold" style="color: var(--accent-green)">{{ chargeDensityData.grid.ny }}</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">网格 Nz</div>
                <div class="text-lg font-semibold" style="color: var(--accent-yellow)">{{ chargeDensityData.grid.nz }}</div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">选择文件后点击"加载电荷密度"查看结果</p>
          </div>
        </template>

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, nextTick } from 'vue'
import type {
  BandStructureData,
  DosData,
  DftEnergyData,
  DftForcesData,
  ChargeDensityData,
  ValidationResult,
} from '../api/dftPostProcess'

// ============ Tab State ============

type TabValue = 'band' | 'dos' | 'energy' | 'validation' | 'chargedensity'

const tabs = [
  { value: 'band' as TabValue, label: '能带结构' },
  { value: 'dos' as TabValue, label: '态密度(DOS)' },
  { value: 'energy' as TabValue, label: '能量/力' },
  { value: 'validation' as TabValue, label: '验证' },
  { value: 'chargedensity' as TabValue, label: '电荷密度' },
]

const activeTab = ref<TabValue>('band')

// ============ Band Structure State ============

const bandParams = reactive({
  fileType: 'EIGENVAL',
  filepath: '',
  eMin: -6,
  eMax: 6,
  showFermi: true,
  showGap: true,
})

const bandData = ref<BandStructureData | null>(null)

const bandSvgW = 640
const bandSvgH = 420
const bandPadL = 50
const bandPadT = 20
const bandPlotW = bandSvgW - bandPadL - 20
const bandPlotH = bandSvgH - bandPadT - 35

const bandEMin = computed(() => bandParams.eMin)
const bandEMax = computed(() => bandParams.eMax)
const bandERange = computed(() => bandEMax.value - bandEMin.value)

const bandFermiY = computed(() => {
  if (!bandData.value) return bandPadT + bandPlotH / 2
  const ef = bandData.value.fermi_energy_eV
  return bandPadT + bandPlotH - ((ef - bandEMin.value) / bandERange.value) * bandPlotH
})

const bandHighSymX = computed(() => {
  if (!bandData.value || bandData.value.kpoints.length === 0) return []
  const kpts = bandData.value.kpoints
  const totalK = kpts.length
  const result: Array<{ label: string; x: number }> = []
  for (let i = 0; i < totalK; i++) {
    if (kpts[i].label && kpts[i].label.trim() !== '') {
      result.push({ label: kpts[i].label, x: (i / (totalK - 1)) * bandPlotW })
    }
  }
  return result
})

const bandGapTopY = computed(() => {
  if (!bandData.value) return 0
  const ef = bandData.value.fermi_energy_eV
  return bandPadT + bandPlotH - ((ef + bandData.value.band_gap_eV / 2 - bandEMin.value) / bandERange.value) * bandPlotH
})

const bandGapBotY = computed(() => {
  if (!bandData.value) return 0
  const ef = bandData.value.fermi_energy_eV
  return bandPadT + bandPlotH - ((ef - bandData.value.band_gap_eV / 2 - bandEMin.value) / bandERange.value) * bandPlotH
})

function bandCurvePoints(bandIdx: number): string {
  if (!bandData.value) return ''
  const band = bandData.value.bands[bandIdx]
  if (!band) return ''
  const totalK = band.length
  return band.map((pt, i) => {
    const x = bandPadL + (i / (totalK - 1)) * bandPlotW
    const y = bandPadT + bandPlotH - ((pt.eV - bandEMin.value) / bandERange.value) * bandPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
}

// ============ DOS State ============

const dosParams = reactive({
  fileType: 'DOSCAR',
  filepath: '',
  eMin: -8,
  eMax: 4,
  showFermi: true,
  showIntegrated: true,
})

const dosData = ref<DosData | null>(null)

const dosSvgW = 640
const dosSvgH = 360
const dosPadL = 50
const dosPadT = 20
const dosPlotW = dosSvgW - dosPadL - 20
const dosPlotH = dosSvgH - dosPadT - 35

const dosEMin = computed(() => dosParams.eMin)
const dosEMax = computed(() => dosParams.eMax)
const dosERange = computed(() => dosEMax.value - dosEMin.value)

const dosYMax = computed(() => {
  if (!dosData.value) return 5
  const maxDos = Math.max(...dosData.value.total_dos)
  return Math.ceil(maxDos * 1.15 * 10) / 10
})

const dosFermiY = computed(() => {
  if (!dosData.value) return dosPadT + dosPlotH / 2
  const ef = dosData.value.fermi_energy_eV
  return dosPadT + dosPlotH - ((ef - dosEMin.value) / dosERange.value) * dosPlotH
})

const dosAtFermi = computed(() => {
  if (!dosData.value) return 0
  const ef = dosData.value.fermi_energy_eV
  const idx = dosData.value.energies.findIndex(e => e >= ef)
  if (idx < 0) return 0
  return dosData.value.total_dos[idx]
})

const pdosColors = ['#3b82f6', '#ef4444', '#22c55e', '#eab308', '#a855f7', '#f97316']

const dosTotalCurvePoints = computed(() => {
  if (!dosData.value) return ''
  return dosData.value.energies.map((e, i) => {
    if (e < dosEMin.value || e > dosEMax.value) return null
    const x = dosPadL + ((e - dosEMin.value) / dosERange.value) * dosPlotW
    const y = dosPadT + dosPlotH - (dosData.value!.total_dos[i] / dosYMax.value) * dosPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).filter(Boolean).join(' ')
})

function pdosCurvePoints(pdosIdx: number): string {
  if (!dosData.value) return ''
  const pdos = dosData.value.partial_dos[pdosIdx]
  if (!pdos) return ''
  return dosData.value.energies.map((e, i) => {
    if (e < dosEMin.value || e > dosEMax.value) return null
    const x = dosPadL + ((e - dosEMin.value) / dosERange.value) * dosPlotW
    const y = dosPadT + dosPlotH - (pdos.dos[i] / dosYMax.value) * dosPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).filter(Boolean).join(' ')
}

const dosIntegratedCurvePoints = computed(() => {
  if (!dosData.value) return ''
  const maxInt = Math.max(...dosData.value.integrated_dos)
  return dosData.value.energies.map((e, i) => {
    if (e < dosEMin.value || e > dosEMax.value) return null
    const x = dosPadL + ((e - dosEMin.value) / dosERange.value) * dosPlotW
    const y = dosPadT + dosPlotH - (dosData.value!.integrated_dos[i] / maxInt) * dosPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).filter(Boolean).join(' ')
})

// ============ Energy/Force State ============

const energyParams = reactive({
  fileType: 'OUTCAR',
  filepath: '',
})

const energyData = ref<DftEnergyData | null>(null)
const forcesData = ref<DftForcesData>({ atoms: [], forces: [], max_force: 0, rms_force: 0 })

const energySvgW = 640
const energySvgH = 360
const energyPadL = 60
const energyPadT = 20
const energyPlotW = energySvgW - energyPadL - 20
const energyPlotH = energySvgH - energyPadT - 35

const energyYMin = computed(() => {
  if (!energyData.value || energyData.value.ionic_steps.length === 0) return -10
  return Math.min(...energyData.value.ionic_steps.map(s => s.energy)) - 0.5
})

const energyYMax = computed(() => {
  if (!energyData.value || energyData.value.ionic_steps.length === 0) return 0
  return Math.max(...energyData.value.ionic_steps.map(s => s.energy)) + 0.5
})

const energyYRange = computed(() => energyYMax.value - energyYMin.value)

const energyConvergencePoints = computed(() => {
  if (!energyData.value) return ''
  const steps = energyData.value.ionic_steps
  const n = steps.length
  return steps.map((s, i) => {
    const x = energyPadL + (i / Math.max(1, n - 1)) * energyPlotW
    const y = energyPadT + energyPlotH - ((s.energy - energyYMin.value) / energyYRange.value) * energyPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

const energyDriftPoints = computed(() => {
  if (!energyData.value) return ''
  const steps = energyData.value.ionic_steps
  const n = steps.length
  return steps.map((s, i) => {
    const x = energyPadL + (i / Math.max(1, n - 1)) * energyPlotW
    const driftE = s.energy + s.drift
    const y = energyPadT + energyPlotH - ((driftE - energyYMin.value) / energyYRange.value) * energyPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

const forceYMax = computed(() => {
  if (!energyData.value || energyData.value.ionic_steps.length === 0) return 1
  return Math.max(...energyData.value.ionic_steps.map(s => {
    const maxF = 0.5 * Math.exp(-s.step * 0.15) + 0.01
    return maxF
  })) * 1.15
})

const forceConvergencePoints = computed(() => {
  if (!energyData.value) return ''
  const steps = energyData.value.ionic_steps
  const n = steps.length
  return steps.map((s, i) => {
    const x = energyPadL + (i / Math.max(1, n - 1)) * energyPlotW
    const maxF = 0.5 * Math.exp(-s.step * 0.15) + 0.01
    const y = energyPadT + energyPlotH - (maxF / forceYMax.value) * energyPlotH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// ============ Validation State ============

const validationTestCases = [
  { id: 'si_fcc', name: 'Si FCC', description: '硅面心立方 SCF 自洽计算' },
  { id: 'al_fcc', name: 'Al FCC', description: '铝面心立方结构优化' },
  { id: 'fe_bcc', name: 'Fe BCC', description: '铁体心立方磁性计算' },
  { id: 'cu_fcc', name: 'Cu FCC', description: '铜面心立方 SCF 自洽计算' },
  { id: 'diamond', name: 'Diamond', description: '金刚石结构 SCF 自洽计算' },
]

const selectedTestCase = ref('si_fcc')
const validationTolerance = ref(5.0)
const validationRunning = ref(false)
const validationResults = ref<ValidationResult[]>([])

const validationPassRate = computed(() => {
  if (validationResults.value.length === 0) return 0
  const passed = validationResults.value.filter(r => r.passed).length
  return (passed / validationResults.value.length) * 100
})

const validationAvgError = computed(() => {
  if (validationResults.value.length === 0) return 0
  const sum = validationResults.value.reduce((s, r) => s + Math.abs(r.error_meV_per_atom), 0)
  return sum / validationResults.value.length
})

// ============ Charge Density State ============

const chargeParams = reactive({
  fileType: 'CHGCAR',
  filepath: '',
  sliceAxis: 'z',
  sliceIndex: 25,
  isosurfaceValue: 0.3,
  showContour: true,
})

const chargeDensityData = ref<ChargeDensityData | null>(null)
const chargeCanvas = ref<HTMLCanvasElement | null>(null)

const chargeMinVal = computed(() => {
  if (!chargeDensityData.value) return 0
  return 0
})

const chargeMaxVal = computed(() => {
  if (!chargeDensityData.value) return 1
  return 1
})

// ============ Has Results ============

const hasResults = computed(() => {
  return bandData.value || dosData.value || energyData.value || validationResults.value.length > 0 || chargeDensityData.value
})

// ============ Quick Templates ============

const quickTemplates = [
  { id: 'si-scf', name: 'Si-SCF', tab: 'band' as TabValue },
  { id: 'al-relax', name: 'Al-结构优化', tab: 'energy' as TabValue },
  { id: 'fe-mag', name: 'Fe-磁性计算', tab: 'validation' as TabValue },
  { id: 'semi-band', name: '能带-半导体', tab: 'band' as TabValue },
]

function applyTemplate(tpl: { id: string; name: string; tab: TabValue }) {
  activeTab.value = tpl.tab
  switch (tpl.id) {
    case 'si-scf':
      loadBandStructure()
      break
    case 'al-relax':
      loadEnergyForce()
      break
    case 'fe-mag':
      runValidationSuite()
      break
    case 'semi-band':
      bandParams.eMin = -6
      bandParams.eMax = 6
      loadBandStructure()
      break
  }
}

// ============ Simulated Data Generators ============

function generateBandStructureData(): BandStructureData {
  const numBands = 6
  const numKpoints = 100
  const fermiEnergy = 0.0
  const bandGap = 1.12

  // High symmetry path: Gamma -> X -> M -> Gamma -> R -> X
  const highSymLabels = ['G', '', '', '', 'X', '', '', 'M', '', '', '', 'G', '', '', '', 'R', '', '', 'X']
  const kpoints = highSymLabels.map((label, i) => ({
    label,
    x: 0,
    y: 0,
    z: 0,
  }))

  const bands: BandStructureData['bands'] = []

  for (let b = 0; b < numBands; b++) {
    const bandPoints: Array<{ k_index: number; energy: number; eV: number }> = []
    for (let k = 0; k < numKpoints; k++) {
      const t = k / (numKpoints - 1)
      let energy: number

      if (b < 3) {
        // Valence bands: below Fermi level
        const baseE = -bandGap / 2 - (2 - b) * 1.5
        const dispersion = -2.0 * Math.cos(2 * Math.PI * t) * (1 - b * 0.2)
        energy = baseE + dispersion + 0.1 * Math.sin(4 * Math.PI * t + b)
      } else {
        // Conduction bands: above Fermi level
        const baseE = bandGap / 2 + (b - 3) * 1.5
        const dispersion = 2.0 * Math.cos(2 * Math.PI * t) * (1 - (b - 3) * 0.2)
        energy = baseE + dispersion + 0.1 * Math.sin(4 * Math.PI * t + b)
      }

      // Add small noise
      energy += (Math.random() - 0.5) * 0.02

      bandPoints.push({ k_index: k, energy, eV: energy })
    }
    bands.push(bandPoints)
  }

  return {
    kpoints,
    bands,
    num_bands: numBands,
    fermi_energy_eV: fermiEnergy,
    band_gap_eV: bandGap,
    direct_gap_eV: bandGap + 0.05,
    indirect_gap_eV: bandGap,
    high_symmetry_points: [
      { label: 'G', energy: 0 },
      { label: 'X', energy: -0.8 },
      { label: 'M', energy: -1.2 },
      { label: 'R', energy: 0.6 },
    ],
  }
}

function generateDosData(): DosData {
  const numPoints = 500
  const eMin = -10
  const eMax = 6
  const de = (eMax - eMin) / numPoints
  const fermiEnergy = 0.0

  const energies: number[] = []
  const totalDos: number[] = []
  const sOrbitalDos: number[] = []
  const pOrbitalDos: number[] = []
  const integratedDos: number[] = []

  let cumInt = 0

  for (let i = 0; i <= numPoints; i++) {
    const e = eMin + i * de
    energies.push(e)

    // Total DOS: Gaussian peaks for valence and conduction bands
    let tdos = 0
    // Valence band: centered around -3 eV, width ~2 eV
    tdos += 3.0 * Math.exp(-Math.pow((e + 3) / 1.5, 2))
    // Lower valence band
    tdos += 1.5 * Math.exp(-Math.pow((e + 6) / 1.0, 2))
    // Conduction band: centered around 2 eV
    tdos += 2.0 * Math.exp(-Math.pow((e - 2) / 1.5, 2))
    // Higher conduction band
    tdos += 1.0 * Math.exp(-Math.pow((e - 4.5) / 1.2, 2))
    // Small noise
    tdos += Math.abs((Math.random() - 0.5) * 0.1)
    tdos = Math.max(0, tdos)

    totalDos.push(tdos)

    // s orbital: mainly lower valence band
    const sDos = 1.2 * Math.exp(-Math.pow((e + 6) / 1.0, 2)) + 0.3 * Math.exp(-Math.pow((e + 3) / 1.5, 2))
    sOrbitalDos.push(Math.max(0, sDos + Math.abs((Math.random() - 0.5) * 0.05)))

    // p orbital: mainly upper valence band and conduction
    const pDos = 2.5 * Math.exp(-Math.pow((e + 3) / 1.5, 2)) + 1.5 * Math.exp(-Math.pow((e - 2) / 1.5, 2))
    pOrbitalDos.push(Math.max(0, pDos + Math.abs((Math.random() - 0.5) * 0.05)))

    cumInt += tdos * de
    integratedDos.push(cumInt)
  }

  return {
    energies,
    total_dos: totalDos,
    partial_dos: [
      { element: 'Si', orbital: 's', dos: sOrbitalDos },
      { element: 'Si', orbital: 'p', dos: pOrbitalDos },
    ],
    fermi_energy_eV: fermiEnergy,
    integrated_dos: integratedDos,
  }
}

function generateEnergyForceData(): { energy: DftEnergyData; forces: DftForcesData } {
  const numIonicSteps = 25
  const ionicSteps: Array<{ step: number; energy: number; drift: number }> = []
  const electronicSteps: Array<{ step: number; energy: number }> = []

  const finalEnergy = -10.8462
  const initialEnergy = finalEnergy + 2.5

  for (let i = 0; i < numIonicSteps; i++) {
    const t = i / (numIonicSteps - 1)
    const energy = initialEnergy + (finalEnergy - initialEnergy) * (1 - Math.exp(-3 * t)) + (Math.random() - 0.5) * 0.01
    const drift = 0.001 * Math.exp(-2 * t) + Math.abs((Math.random() - 0.5) * 0.0005)
    ionicSteps.push({ step: i, energy, drift })
  }

  for (let i = 0; i < 15; i++) {
    const t = i / 14
    const energy = finalEnergy - 0.01 * (1 - Math.exp(-5 * t)) + (Math.random() - 0.5) * 0.001
    electronicSteps.push({ step: i, energy })
  }

  const numAtoms = 8
  const atoms: DftForcesData['atoms'] = []
  const forces: DftForcesData['forces'] = []

  const elements = ['Si', 'Si', 'Si', 'Si', 'Si', 'Si', 'Si', 'Si']
  for (let i = 0; i < numAtoms; i++) {
    atoms.push({
      element: elements[i],
      index: i,
      x: (i % 2) * 2.715 + (Math.random() - 0.5) * 0.01,
      y: Math.floor(i / 2) % 2 * 2.715 + (Math.random() - 0.5) * 0.01,
      z: Math.floor(i / 4) * 5.43 + (Math.random() - 0.5) * 0.01,
    })
    const fx = (Math.random() - 0.5) * 0.02
    const fy = (Math.random() - 0.5) * 0.02
    const fz = (Math.random() - 0.5) * 0.02
    forces.push({ fx, fy, fz, magnitude: Math.sqrt(fx * fx + fy * fy + fz * fz) })
  }

  const maxForce = Math.max(...forces.map(f => f.magnitude))
  const rmsForce = Math.sqrt(forces.reduce((s, f) => s + f.magnitude * f.magnitude, 0) / forces.length)

  return {
    energy: {
      total_energy_eV: finalEnergy,
      energy_per_atom_eV: finalEnergy / numAtoms,
      free_energy_eV: finalEnergy - 0.0005,
      entropy: 0.0005,
      zero_point_energy: 0.162,
      ionic_steps: ionicSteps,
      electronic_steps: electronicSteps,
    },
    forces: {
      atoms,
      forces,
      max_force: maxForce,
      rms_force: rmsForce,
    },
  }
}

function generateValidationResults(): ValidationResult[] {
  const referenceEnergies: Record<string, number> = {
    si_fcc: -10.8462,
    al_fcc: -8.3265,
    fe_bcc: -16.2847,
    cu_fcc: -9.1523,
    diamond: -22.9756,
  }

  const names: Record<string, string> = {
    si_fcc: 'Si FCC',
    al_fcc: 'Al FCC',
    fe_bcc: 'Fe BCC',
    cu_fcc: 'Cu FCC',
    diamond: 'Diamond',
  }

  return Object.entries(referenceEnergies).map(([id, refE]) => {
    const error = (Math.random() - 0.3) * 8
    const calcE = refE + error * 0.001
    return {
      test_name: names[id],
      reference_energy_eV: refE,
      calculated_energy_eV: calcE,
      error_meV_per_atom: error,
      passed: Math.abs(error) < validationTolerance.value,
      tolerance_meV: validationTolerance.value,
    }
  })
}

function generateChargeDensityData(): ChargeDensityData {
  const nx = 50
  const ny = 50
  const nz = 50

  const data: number[][][] = []
  for (let iz = 0; iz < nz; iz++) {
    const slice: number[][] = []
    for (let iy = 0; iy < ny; iy++) {
      const row: number[] = []
      for (let ix = 0; ix < nx; ix++) {
        // Simulate periodic charge density with Gaussian peaks at atom positions
        let val = 0.1
        // FCC-like atom positions
        const atomPositions = [
          [0, 0, 0], [0.5, 0.5, 0], [0.5, 0, 0.5], [0, 0.5, 0.5],
          [0.25, 0.25, 0.25], [0.75, 0.75, 0.25], [0.75, 0.25, 0.75], [0.25, 0.75, 0.75],
        ]
        for (const [ax, ay, az] of atomPositions) {
          const dx = ix / nx - ax
          const dy = iy / ny - ay
          const dz = iz / nz - az
          // Periodic distance
          const pdx = dx - Math.round(dx)
          const pdy = dy - Math.round(dy)
          const pdz = dz - Math.round(dz)
          const r2 = pdx * pdx + pdy * pdy + pdz * pdz
          val += 0.8 * Math.exp(-r2 * 50)
        }
        row.push(val)
      }
      slice.push(row)
    }
    data.push(slice)
  }

  return {
    grid: { nx, ny, nz },
    data,
    isosurface_values: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
  }
}

// ============ Load / Run Functions ============

function loadBandStructure() {
  bandData.value = generateBandStructureData()
}

function loadDos() {
  dosData.value = generateDosData()
}

function loadEnergyForce() {
  const result = generateEnergyForceData()
  energyData.value = result.energy
  forcesData.value = result.forces
}

function runValidationSuite() {
  validationRunning.value = true
  setTimeout(() => {
    validationResults.value = generateValidationResults()
    validationRunning.value = false
  }, 800)
}

function loadChargeDensity() {
  chargeDensityData.value = generateChargeDensityData()
  nextTick(() => drawChargeDensityCanvas())
}

// ============ Canvas Drawing ============

function heatColor(t: number): string {
  t = Math.max(0, Math.min(1, t))
  let r: number, g: number, b: number
  if (t < 0.25) {
    const s = t / 0.25
    r = Math.round(30 * (1 - s) + 59 * s)
    g = Math.round(58 * (1 - s) + 130 * s)
    b = Math.round(95 * (1 - s) + 246 * s)
  } else if (t < 0.5) {
    const s = (t - 0.25) / 0.25
    r = Math.round(59 * (1 - s) + 34 * s)
    g = Math.round(130 * (1 - s) + 197 * s)
    b = Math.round(246 * (1 - s) + 94 * s)
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
  return `rgb(${r},${g},${b})`
}

function drawChargeDensityCanvas() {
  if (!chargeCanvas.value || !chargeDensityData.value) return
  const canvas = chargeCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const { nx, ny, nz } = chargeDensityData.value.grid
  const sliceIdx = Math.min(chargeParams.sliceIndex, nz - 1)
  const axis = chargeParams.sliceAxis

  const sliceSize1 = axis === 'z' ? ny : axis === 'y' ? nx : nx
  const sliceSize2 = axis === 'z' ? nx : axis === 'y' ? nz : ny

  ctx.clearRect(0, 0, canvas.width, canvas.height)

  const cellW = canvas.width / sliceSize1
  const cellH = canvas.height / sliceSize2

  let globalMin = Infinity
  let globalMax = -Infinity

  for (let i2 = 0; i2 < sliceSize2; i2++) {
    for (let i1 = 0; i1 < sliceSize1; i1++) {
      let val: number
      if (axis === 'z') {
        val = chargeDensityData.value.data[sliceIdx][i2][i1]
      } else if (axis === 'y') {
        val = chargeDensityData.value.data[i2][sliceIdx][i1]
      } else {
        val = chargeDensityData.value.data[i2][i1][sliceIdx]
      }
      if (val < globalMin) globalMin = val
      if (val > globalMax) globalMax = val
    }
  }

  const range = globalMax - globalMin || 1

  for (let i2 = 0; i2 < sliceSize2; i2++) {
    for (let i1 = 0; i1 < sliceSize1; i1++) {
      let val: number
      if (axis === 'z') {
        val = chargeDensityData.value.data[sliceIdx][i2][i1]
      } else if (axis === 'y') {
        val = chargeDensityData.value.data[i2][sliceIdx][i1]
      } else {
        val = chargeDensityData.value.data[i2][i1][sliceIdx]
      }
      const t = (val - globalMin) / range
      ctx.fillStyle = heatColor(t)
      ctx.fillRect(i1 * cellW, i2 * cellH, Math.ceil(cellW), Math.ceil(cellH))
    }
  }

  // Draw contour lines if enabled
  if (chargeParams.showContour) {
    const isoVal = chargeParams.isosurfaceValue
    const isoT = (isoVal - globalMin) / range
    ctx.strokeStyle = 'rgba(255,255,255,0.5)'
    ctx.lineWidth = 0.5

    for (let i2 = 0; i2 < sliceSize2 - 1; i2++) {
      for (let i1 = 0; i1 < sliceSize1 - 1; i1++) {
        let v00: number, v10: number, v01: number, v11: number
        if (axis === 'z') {
          v00 = chargeDensityData.value.data[sliceIdx][i2][i1]
          v10 = chargeDensityData.value.data[sliceIdx][i2][i1 + 1]
          v01 = chargeDensityData.value.data[sliceIdx][i2 + 1][i1]
          v11 = chargeDensityData.value.data[sliceIdx][i2 + 1][i1 + 1]
        } else if (axis === 'y') {
          v00 = chargeDensityData.value.data[i2][sliceIdx][i1]
          v10 = chargeDensityData.value.data[i2][sliceIdx][i1 + 1]
          v01 = chargeDensityData.value.data[i2 + 1][sliceIdx][i1]
          v11 = chargeDensityData.value.data[i2 + 1][sliceIdx][i1 + 1]
        } else {
          v00 = chargeDensityData.value.data[i2][i1][sliceIdx]
          v10 = chargeDensityData.value.data[i2][i1 + 1][sliceIdx]
          v01 = chargeDensityData.value.data[i2 + 1][i1][sliceIdx]
          v11 = chargeDensityData.value.data[i2 + 1][i1 + 1][sliceIdx]
        }

        const t00 = (v00 - globalMin) / range
        const t10 = (v10 - globalMin) / range
        const t01 = (v01 - globalMin) / range
        const t11 = (v11 - globalMin) / range

        // Simple marching squares for contour
        const x0 = i1 * cellW
        const y0 = i2 * cellH
        const x1 = (i1 + 1) * cellW
        const y1 = (i2 + 1) * cellH

        const edges: Array<{ x1: number; y1: number; x2: number; y2: number }> = []

        if ((t00 < isoT) !== (t10 < isoT)) {
          const frac = (isoT - t00) / (t10 - t00)
          edges.push({ x1: x0 + frac * (x1 - x0), y1: y0, x2: x0 + frac * (x1 - x0), y2: y0 })
        }
        if ((t10 < isoT) !== (t11 < isoT)) {
          const frac = (isoT - t10) / (t11 - t10)
          edges.push({ x1: x1, y1: y0 + frac * (y1 - y0), x2: x1, y2: y0 + frac * (y1 - y0) })
        }
        if ((t01 < isoT) !== (t11 < isoT)) {
          const frac = (isoT - t01) / (t11 - t01)
          edges.push({ x1: x0 + frac * (x1 - x0), y1: y1, x2: x0 + frac * (x1 - x0), y2: y1 })
        }
        if ((t00 < isoT) !== (t01 < isoT)) {
          const frac = (isoT - t00) / (t01 - t00)
          edges.push({ x1: x0, y1: y0 + frac * (y1 - y0), x2: x0, y2: y0 + frac * (y1 - y0) })
        }

        if (edges.length >= 2) {
          ctx.beginPath()
          ctx.moveTo(edges[0].x1, edges[0].y1)
          ctx.lineTo(edges[1].x1, edges[1].y1)
          ctx.stroke()
        }
      }
    }
  }
}

// Watch for charge density parameter changes
watch([() => chargeParams.sliceIndex, () => chargeParams.isosurfaceValue, () => chargeParams.showContour], () => {
  if (chargeDensityData.value) {
    nextTick(() => drawChargeDensityCanvas())
  }
})

// ============ Reset ============

function resetAll() {
  bandData.value = null
  dosData.value = null
  energyData.value = null
  forcesData.value = { atoms: [], forces: [], max_force: 0, rms_force: 0 }
  validationResults.value = []
  chargeDensityData.value = null
  bandParams.fileType = 'EIGENVAL'
  bandParams.filepath = ''
  bandParams.eMin = -6
  bandParams.eMax = 6
  bandParams.showFermi = true
  bandParams.showGap = true
  dosParams.fileType = 'DOSCAR'
  dosParams.filepath = ''
  dosParams.eMin = -8
  dosParams.eMax = 4
  dosParams.showFermi = true
  dosParams.showIntegrated = true
  energyParams.fileType = 'OUTCAR'
  energyParams.filepath = ''
  selectedTestCase.value = 'si_fcc'
  validationTolerance.value = 5.0
  chargeParams.fileType = 'CHGCAR'
  chargeParams.filepath = ''
  chargeParams.sliceAxis = 'z'
  chargeParams.sliceIndex = 25
  chargeParams.isosurfaceValue = 0.3
  chargeParams.showContour = true
}

// ============ Export ============

function exportResults() {
  const data: Record<string, unknown> = {}
  if (bandData.value) data.band_structure = bandData.value
  if (dosData.value) data.dos = dosData.value
  if (energyData.value) data.energy = energyData.value
  if (forcesData.value) data.forces = forcesData.value
  if (validationResults.value.length > 0) data.validation = validationResults.value

  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'dft_postprocess_results.json'
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<style scoped>
.label {
  display: block;
  font-size: 11px;
  font-weight: 500;
  margin-bottom: 4px;
  color: var(--text-secondary);
}

.panel-section {
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.panel-section:last-child {
  border-bottom: none;
}

.input {
  display: block;
  width: 100%;
  padding: 6px 10px;
  font-size: 12px;
  line-height: 1.4;
  color: var(--text-primary);
  background: var(--bg-elevated);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md, 6px);
  outline: none;
  transition: border-color var(--duration-fast, 150ms) var(--ease-out, ease-out);
}

.input:focus {
  border-color: var(--primary);
  box-shadow: 0 0 0 2px var(--primary-glow, rgba(59, 130, 246, 0.15));
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 6px 14px;
  font-size: 12px;
  font-weight: 500;
  line-height: 1.4;
  border-radius: var(--radius-md, 6px);
  border: 1px solid transparent;
  cursor: pointer;
  transition: all var(--duration-fast, 150ms) var(--ease-out, ease-out);
  white-space: nowrap;
}

.btn-primary {
  background: var(--primary);
  color: white;
  border-color: var(--primary);
}

.btn-primary:hover {
  opacity: 0.9;
  box-shadow: 0 0 12px var(--primary-glow, rgba(59, 130, 246, 0.3));
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
}
</style>
