<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">MD 后处理分析</h2>
        <p class="text-sm" style="color: var(--text-muted)">RDF / MSD / 扩散系数 / VACF，MD→相场 / MD→FE 尺度桥接</p>
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

        <!-- RDF Panel -->
        <template v-if="activeTab === 'rdf'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">元素对选择</h4>
            <div class="space-y-2">
              <div>
                <label class="label">预设元素对</label>
                <select v-model="rdfParams.elementPair" class="input w-full text-xs">
                  <option value="Fe-Fe">Fe-Fe</option>
                  <option value="Al-Al">Al-Al</option>
                  <option value="Cu-Cu">Cu-Cu</option>
                  <option value="Si-Si">Si-Si</option>
                  <option value="Fe-C">Fe-C</option>
                  <option value="custom">自定义</option>
                </select>
              </div>
              <div v-if="rdfParams.elementPair === 'custom'" class="grid grid-cols-2 gap-2">
                <div>
                  <label class="label">元素 A</label>
                  <input v-model="rdfParams.customElementA" type="text" class="input w-full text-xs" placeholder="Fe" />
                </div>
                <div>
                  <label class="label">元素 B</label>
                  <input v-model="rdfParams.customElementB" type="text" class="input w-full text-xs" placeholder="C" />
                </div>
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">参数设置</h4>
            <div class="space-y-2">
              <div>
                <label class="label">r_max (Å)</label>
                <input v-model.number="rdfParams.rMax" type="number" step="0.5" min="1" max="20" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">bin_width (Å)</label>
                <input v-model.number="rdfParams.binWidth" type="number" step="0.01" min="0.01" max="0.5" class="input w-full text-xs" />
              </div>
            </div>
          </div>
          <button @click="runRdf" class="btn btn-primary text-xs w-full">计算 RDF</button>
        </template>

        <!-- MSD Panel -->
        <template v-if="activeTab === 'msd'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">元素选择</h4>
            <div class="space-y-2">
              <div>
                <label class="label">元素</label>
                <select v-model="msdParams.element" class="input w-full text-xs">
                  <option value="Al">Al</option>
                  <option value="Cu">Cu</option>
                  <option value="Fe">Fe</option>
                  <option value="Si">Si</option>
                  <option value="C">C</option>
                  <option value="Ni">Ni</option>
                </select>
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">时间范围</h4>
            <div class="space-y-2">
              <div>
                <label class="label">起始时间 (fs)</label>
                <input v-model.number="msdParams.startFs" type="number" step="100" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">结束时间 (fs)</label>
                <input v-model.number="msdParams.endFs" type="number" step="1000" class="input w-full text-xs" />
              </div>
            </div>
          </div>
          <button @click="runMsd" class="btn btn-primary text-xs w-full">计算 MSD</button>
        </template>

        <!-- VACF Panel -->
        <template v-if="activeTab === 'vacf'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">元素选择</h4>
            <div class="space-y-2">
              <div>
                <label class="label">元素</label>
                <select v-model="vacfParams.element" class="input w-full text-xs">
                  <option value="Al">Al</option>
                  <option value="Cu">Cu</option>
                  <option value="Fe">Fe</option>
                  <option value="Si">Si</option>
                  <option value="C">C</option>
                  <option value="Ni">Ni</option>
                </select>
              </div>
            </div>
          </div>
          <button @click="runVacf" class="btn btn-primary text-xs w-full">计算 VACF</button>
        </template>

        <!-- MD->PhaseField Panel -->
        <template v-if="activeTab === 'phasefield'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">粗粒化方法</h4>
            <div class="flex flex-col gap-1">
              <button
                v-for="m in cgMethods"
                :key="m.value"
                @click="pfConfig.coarseGrainingMethod = m.value"
                :class="['px-3 py-2 rounded text-xs text-left transition border', pfConfig.coarseGrainingMethod === m.value ? 'text-white' : '']"
                :style="pfConfig.coarseGrainingMethod === m.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                <div class="font-medium">{{ m.label }}</div>
                <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
              </button>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">网格参数</h4>
            <div class="space-y-2">
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <label class="label">Nx</label>
                  <input v-model.number="pfConfig.gridNx" type="number" step="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">Ny</label>
                  <input v-model.number="pfConfig.gridNy" type="number" step="10" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">Nz</label>
                  <input v-model.number="pfConfig.gridNz" type="number" step="10" class="input w-full text-xs" />
                </div>
              </div>
              <div>
                <label class="label">平滑长度 (Å)</label>
                <input v-model.number="pfConfig.smoothingLength" type="number" step="0.1" class="input w-full text-xs" />
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">输出场</h4>
            <div class="flex flex-col gap-1">
              <label v-for="field in pfOutputFields" :key="field.value" class="flex items-center gap-2 text-xs" style="color: var(--text-secondary)">
                <input type="checkbox" v-model="pfConfig.outputFields" :value="field.value" class="rounded" />
                {{ field.label }}
              </label>
            </div>
          </div>
          <button @click="runPhaseField" class="btn btn-primary text-xs w-full">粗粒化到相场</button>
        </template>

        <!-- MD->FE Panel -->
        <template v-if="activeTab === 'fe'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">FE 网格参数</h4>
            <div class="space-y-2">
              <div>
                <label class="label">网格尺寸 (Å)</label>
                <input v-model.number="feConfig.meshSize" type="number" step="0.5" class="input w-full text-xs" />
              </div>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">应力平均方法</h4>
            <div class="flex flex-col gap-1">
              <button
                v-for="m in stressMethods"
                :key="m.value"
                @click="feConfig.stressMethod = m.value"
                :class="['px-3 py-2 rounded text-xs text-left transition border', feConfig.stressMethod === m.value ? 'text-white' : '']"
                :style="feConfig.stressMethod === m.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                <div class="font-medium">{{ m.label }}</div>
                <div class="text-[10px] mt-0.5 opacity-80">{{ m.desc }}</div>
              </button>
            </div>
          </div>
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">输出类型</h4>
            <div class="flex flex-col gap-1">
              <button
                v-for="t in feOutputTypes"
                :key="t.value"
                @click="feConfig.outputType = t.value"
                :class="['px-3 py-2 rounded text-xs text-left transition border', feConfig.outputType === t.value ? 'text-white' : '']"
                :style="feConfig.outputType === t.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                <div class="font-medium">{{ t.label }}</div>
                <div class="text-[10px] mt-0.5 opacity-80">{{ t.desc }}</div>
              </button>
            </div>
          </div>
          <button @click="runFeMapping" class="btn btn-primary text-xs w-full">映射到 FE</button>
        </template>
      </div>

      <!-- Right Panel: Results -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4" style="background: var(--bg-base)">

        <!-- RDF Results -->
        <template v-if="activeTab === 'rdf'">
          <div v-if="rdfResult" class="space-y-4">
            <!-- RDF Curve SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">径向分布函数 g(r)</h4>
              <svg :viewBox="`0 0 ${rdfSvgWidth} ${rdfSvgHeight}`" class="w-full" style="max-height: 360px">
                <!-- Grid lines -->
                <line v-for="i in 5" :key="'h'+i"
                  :x1="rdfSvgPadL" :y1="rdfSvgPadT + (rdfSvgH / 5) * (i - 1)"
                  :x2="rdfSvgW + rdfSvgPadL" :y2="rdfSvgPadT + (rdfSvgH / 5) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <line v-for="i in 6" :key="'v'+i"
                  :x1="rdfSvgPadL + (rdfSvgW / 6) * (i - 1)" :y1="rdfSvgPadT"
                  :x2="rdfSvgPadL + (rdfSvgW / 6) * (i - 1)" :y2="rdfSvgH + rdfSvgPadT"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="rdfSvgPadL" :y1="rdfSvgPadT" :x2="rdfSvgPadL" :y2="rdfSvgH + rdfSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="rdfSvgPadL" :y1="rdfSvgH + rdfSvgPadT" :x2="rdfSvgW + rdfSvgPadL" :y2="rdfSvgH + rdfSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- RDF Curve -->
                <polyline
                  :points="rdfCurvePoints"
                  fill="none"
                  stroke="var(--primary)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Peak markers -->
                <circle
                  v-for="(peak, idx) in rdfResult.peak_positions"
                  :key="'pk'+idx"
                  :cx="rdfSvgPadL + (peak / rdfParams.rMax) * rdfSvgW"
                  :cy="rdfSvgPadT + rdfSvgH - (getRdfYAtR(peak) / rdfYMax) * rdfSvgH"
                  r="4"
                  fill="var(--accent-red)"
                  stroke="white"
                  stroke-width="1.5"
                />
                <!-- First peak annotation -->
                <line
                  :x1="rdfSvgPadL + (rdfResult.first_peak_position / rdfParams.rMax) * rdfSvgW"
                  :y1="rdfSvgPadT + rdfSvgH - (rdfResult.first_peak_height / rdfYMax) * rdfSvgH"
                  :x2="rdfSvgPadL + (rdfResult.first_peak_position / rdfParams.rMax) * rdfSvgW + 30"
                  :y2="rdfSvgPadT + 15"
                  stroke="var(--accent-red)"
                  stroke-width="0.8"
                  stroke-dasharray="3,2"
                />
                <text
                  :x="rdfSvgPadL + (rdfResult.first_peak_position / rdfParams.rMax) * rdfSvgW + 32"
                  :y="rdfSvgPadT + 18"
                  fill="var(--accent-red)"
                  font-size="9"
                >
                  第一峰: {{ rdfResult.first_peak_position.toFixed(2) }} Å, g(r)={{ rdfResult.first_peak_height.toFixed(2) }}
                </text>
                <!-- Axis labels -->
                <text :x="rdfSvgPadL + rdfSvgW / 2" :y="rdfSvgHeight - 5" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">r (Å)</text>
                <text :x="12" :y="rdfSvgPadT + rdfSvgH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 12, 60)">g(r)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'xt'+i"
                  :x="rdfSvgPadL + (rdfSvgW / 6) * (i - 1)" :y="rdfSvgH + rdfSvgPadT + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ ((rdfParams.rMax / 6) * (i - 1)).toFixed(1) }}
                </text>
                <!-- Y ticks -->
                <text v-for="i in 5" :key="'yt'+i"
                  :x="rdfSvgPadL - 4" :y="rdfSvgPadT + (rdfSvgH / 5) * (5 - i) + 3"
                  text-anchor="end" fill="var(--text-muted)" font-size="8">
                  {{ ((rdfYMax / 5) * i).toFixed(1) }}
                </text>
              </svg>
            </div>
            <!-- Statistics Cards -->
            <div class="grid grid-cols-3 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">第一峰位置</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ rdfResult.first_peak_position.toFixed(3) }} Å</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">第一峰高度</div>
                <div class="text-lg font-semibold" style="color: var(--accent-red)">{{ rdfResult.first_peak_height.toFixed(3) }}</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">配位数</div>
                <div class="text-lg font-semibold" style="color: var(--accent-green)">{{ rdfResult.coordination_number.toFixed(2) }}</div>
              </div>
            </div>
            <!-- Peak list -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">峰位列表</h4>
              <div class="space-y-1">
                <div v-for="(peak, idx) in rdfResult.peak_positions" :key="'pl'+idx"
                  class="flex items-center justify-between text-xs px-2 py-1 rounded" style="background: var(--bg-elevated)">
                  <span style="color: var(--text-secondary)">第 {{ idx + 1 }} 峰</span>
                  <span style="color: var(--text-primary)">r = {{ peak.toFixed(3) }} Å</span>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">配置参数后点击"计算 RDF"查看结果</p>
          </div>
        </template>

        <!-- MSD Results -->
        <template v-if="activeTab === 'msd'">
          <div v-if="msdResult" class="space-y-4">
            <!-- MSD Curve SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">均方位移 MSD(t)</h4>
              <svg :viewBox="`0 0 ${msdSvgWidth} ${msdSvgHeight}`" class="w-full" style="max-height: 360px">
                <!-- Grid -->
                <line v-for="i in 5" :key="'mh'+i"
                  :x1="msdSvgPadL" :y1="msdSvgPadT + (msdSvgH / 5) * (i - 1)"
                  :x2="msdSvgW + msdSvgPadL" :y2="msdSvgPadT + (msdSvgH / 5) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <line v-for="i in 6" :key="'mv'+i"
                  :x1="msdSvgPadL + (msdSvgW / 6) * (i - 1)" :y1="msdSvgPadT"
                  :x2="msdSvgPadL + (msdSvgW / 6) * (i - 1)" :y2="msdSvgH + msdSvgPadT"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="msdSvgPadL" :y1="msdSvgPadT" :x2="msdSvgPadL" :y2="msdSvgH + msdSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="msdSvgPadL" :y1="msdSvgH + msdSvgPadT" :x2="msdSvgW + msdSvgPadL" :y2="msdSvgH + msdSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- MSD Data Points -->
                <polyline
                  :points="msdCurvePoints"
                  fill="none"
                  stroke="var(--primary)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Linear Fit -->
                <line
                  :x1="msdSvgPadL"
                  :y1="msdSvgPadT + msdSvgH - (msdResult.fit_intercept / msdYMax) * msdSvgH"
                  :x2="msdSvgPadL + msdSvgW"
                  :y2="msdSvgPadT + msdSvgH - ((msdResult.fit_slope * msdResult.time_points[msdResult.time_points.length - 1].time_fs + msdResult.fit_intercept) / msdYMax) * msdSvgH"
                  stroke="var(--accent-red)"
                  stroke-width="1.5"
                  stroke-dasharray="6,3"
                />
                <!-- Legend -->
                <line x1="msdSvgPadL + 10" :y1="msdSvgPadT + 12" :x2="msdSvgPadL + 30" :y2="msdSvgPadT + 12"
                  stroke="var(--primary)" stroke-width="2" />
                <text :x="msdSvgPadL + 34" :y="msdSvgPadT + 15" fill="var(--text-secondary)" font-size="9">MSD 数据</text>
                <line x1="msdSvgPadL + 10" :y1="msdSvgPadT + 26" :x2="msdSvgPadL + 30" :y2="msdSvgPadT + 26"
                  stroke="var(--accent-red)" stroke-width="1.5" stroke-dasharray="6,3" />
                <text :x="msdSvgPadL + 34" :y="msdSvgPadT + 29" fill="var(--text-secondary)" font-size="9">线性拟合 (R²={{ msdResult.r_squared.toFixed(4) }})</text>
                <!-- Axis labels -->
                <text :x="msdSvgPadL + msdSvgW / 2" :y="msdSvgHeight - 5" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">time (fs)</text>
                <text :x="12" :y="msdSvgPadT + msdSvgH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 12, 60)">MSD (Å²)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'mxt'+i"
                  :x="msdSvgPadL + (msdSvgW / 6) * (i - 1)" :y="msdSvgH + msdSvgPadT + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ ((msdParams.endFs / 6) * (i - 1)).toFixed(0) }}
                </text>
                <!-- Y ticks -->
                <text v-for="i in 5" :key="'myt'+i"
                  :x="msdSvgPadL - 4" :y="msdSvgPadT + (msdSvgH / 5) * (5 - i) + 3"
                  text-anchor="end" fill="var(--text-muted)" font-size="8">
                  {{ ((msdYMax / 5) * i).toFixed(1) }}
                </text>
              </svg>
            </div>
            <!-- Diffusion Coefficient Cards -->
            <div class="grid grid-cols-3 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">扩散系数 D</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ msdResult.diffusion_coefficient_cm2_s.toExponential(3) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">cm²/s</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">扩散系数 D</div>
                <div class="text-lg font-semibold" style="color: var(--accent-yellow)">{{ msdResult.diffusion_coefficient_A2_fs.toExponential(3) }}</div>
                <div class="text-[10px]" style="color: var(--text-muted)">Å²/fs</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">拟合 R²</div>
                <div class="text-lg font-semibold" :style="{ color: msdResult.r_squared > 0.99 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                  {{ msdResult.r_squared.toFixed(6) }}
                </div>
                <div class="text-[10px]" style="color: var(--text-muted)">线性度</div>
              </div>
            </div>
            <!-- Einstein Relation Verification -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">Einstein 关系验证</h4>
              <div class="text-xs space-y-1" style="color: var(--text-secondary)">
                <p>MSD = 6Dt (三维扩散)</p>
                <p>拟合斜率: {{ msdResult.fit_slope.toExponential(4) }} Å²/fs</p>
                <p>拟合截距: {{ msdResult.fit_intercept.toExponential(4) }} Å²</p>
                <p>D = slope / 6 = {{ (msdResult.fit_slope / 6).toExponential(4) }} Å²/fs</p>
                <p class="font-medium" :style="{ color: msdResult.r_squared > 0.99 ? 'var(--accent-green)' : 'var(--accent-yellow)' }">
                  {{ msdResult.r_squared > 0.99 ? '线性关系良好，扩散行为正常' : '线性度偏低，可能存在非扩散行为' }}
                </p>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">配置参数后点击"计算 MSD"查看结果</p>
          </div>
        </template>

        <!-- VACF Results -->
        <template v-if="activeTab === 'vacf'">
          <div v-if="vacfResult" class="space-y-4">
            <!-- VACF Curve SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">速度自相关函数 C(t)</h4>
              <svg :viewBox="`0 0 ${vacfSvgWidth} ${vacfSvgHeight}`" class="w-full" style="max-height: 320px">
                <!-- Grid -->
                <line v-for="i in 5" :key="'vh'+i"
                  :x1="vacfSvgPadL" :y1="vacfSvgPadT + (vacfSvgH / 5) * (i - 1)"
                  :x2="vacfSvgW + vacfSvgPadL" :y2="vacfSvgPadT + (vacfSvgH / 5) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <line v-for="i in 6" :key="'vv'+i"
                  :x1="vacfSvgPadL + (vacfSvgW / 6) * (i - 1)" :y1="vacfSvgPadT"
                  :x2="vacfSvgPadL + (vacfSvgW / 6) * (i - 1)" :y2="vacfSvgH + vacfSvgPadT"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Zero line -->
                <line :x1="vacfSvgPadL" :y1="vacfSvgPadT + vacfSvgH / 2"
                  :x2="vacfSvgW + vacfSvgPadL" :y2="vacfSvgPadT + vacfSvgH / 2"
                  stroke="var(--text-muted)" stroke-width="0.8" />
                <!-- Axes -->
                <line :x1="vacfSvgPadL" :y1="vacfSvgPadT" :x2="vacfSvgPadL" :y2="vacfSvgH + vacfSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="vacfSvgPadL" :y1="vacfSvgH + vacfSvgPadT" :x2="vacfSvgW + vacfSvgPadL" :y2="vacfSvgH + vacfSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- VACF Curve -->
                <polyline
                  :points="vacfCurvePoints"
                  fill="none"
                  stroke="var(--primary)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Axis labels -->
                <text :x="vacfSvgPadL + vacfSvgW / 2" :y="vacfSvgHeight - 5" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">t (fs)</text>
                <text :x="12" :y="vacfSvgPadT + vacfSvgH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 12, 60)">C(t)</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'vxt'+i"
                  :x="vacfSvgPadL + (vacfSvgW / 6) * (i - 1)" :y="vacfSvgH + vacfSvgPadT + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ ((vacfMaxTime / 6) * (i - 1)).toFixed(0) }}
                </text>
                <!-- Y ticks -->
                <text v-for="i in 5" :key="'vyt'+i"
                  :x="vacfSvgPadL - 4" :y="vacfSvgPadT + (vacfSvgH / 5) * (5 - i) + 3"
                  text-anchor="end" fill="var(--text-muted)" font-size="8">
                  {{ ((1.0 / 5) * (2 * i - 5)).toFixed(2) }}
                </text>
              </svg>
            </div>
            <!-- VDOS SVG -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">振动态密度 (VDOS)</h4>
              <svg :viewBox="`0 0 ${vdosSvgWidth} ${vdosSvgHeight}`" class="w-full" style="max-height: 320px">
                <!-- Grid -->
                <line v-for="i in 5" :key="'dh'+i"
                  :x1="vdosSvgPadL" :y1="vdosSvgPadT + (vdosSvgH / 5) * (i - 1)"
                  :x2="vdosSvgW + vdosSvgPadL" :y2="vdosSvgPadT + (vdosSvgH / 5) * (i - 1)"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <line v-for="i in 6" :key="'dv'+i"
                  :x1="vdosSvgPadL + (vdosSvgW / 6) * (i - 1)" :y1="vdosSvgPadT"
                  :x2="vdosSvgPadL + (vdosSvgW / 6) * (i - 1)" :y2="vdosSvgH + vdosSvgPadT"
                  stroke="var(--border-subtle)" stroke-width="0.5" stroke-dasharray="4,4" />
                <!-- Axes -->
                <line :x1="vdosSvgPadL" :y1="vdosSvgPadT" :x2="vdosSvgPadL" :y2="vdosSvgH + vdosSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <line :x1="vdosSvgPadL" :y1="vdosSvgH + vdosSvgPadT" :x2="vdosSvgW + vdosSvgPadL" :y2="vdosSvgH + vdosSvgPadT"
                  stroke="var(--text-muted)" stroke-width="1" />
                <!-- VDOS Curve -->
                <polyline
                  :points="vdosCurvePoints"
                  fill="none"
                  stroke="var(--accent-green)"
                  stroke-width="2"
                  stroke-linejoin="round"
                />
                <!-- Axis labels -->
                <text :x="vdosSvgPadL + vdosSvgW / 2" :y="vdosSvgHeight - 5" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10">Frequency (THz)</text>
                <text :x="12" :y="vdosSvgPadT + vdosSvgH / 2" text-anchor="middle"
                  fill="var(--text-muted)" font-size="10" transform="rotate(-90, 12, 60)">VDOS</text>
                <!-- X ticks -->
                <text v-for="i in 6" :key="'dxt'+i"
                  :x="vdosSvgPadL + (vdosSvgW / 6) * (i - 1)" :y="vdosSvgH + vdosSvgPadT + 14"
                  text-anchor="middle" fill="var(--text-muted)" font-size="8">
                  {{ ((vdosMaxFreq / 6) * (i - 1)).toFixed(1) }}
                </text>
                <!-- Y ticks -->
                <text v-for="i in 5" :key="'dyt'+i"
                  :x="vdosSvgPadL - 4" :y="vdosSvgPadT + (vdosSvgH / 5) * (5 - i) + 3"
                  text-anchor="end" fill="var(--text-muted)" font-size="8">
                  {{ ((vdosYMax / 5) * i).toFixed(2) }}
                </text>
              </svg>
            </div>
            <!-- VACF Stats -->
            <div class="grid grid-cols-2 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">C(0) 初始值</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ vacfResult.velocity_autocorrelation.toFixed(4) }}</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">VDOS 数据点</div>
                <div class="text-lg font-semibold" style="color: var(--accent-green)">{{ vacfResult.vibrational_density_of_states.length }}</div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">配置参数后点击"计算 VACF"查看结果</p>
          </div>
        </template>

        <!-- PhaseField Results -->
        <template v-if="activeTab === 'phasefield'">
          <div v-if="pfResult" class="space-y-4">
            <!-- 2D Slice Heatmap Canvas -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <div class="flex items-center justify-between mb-3">
                <h4 class="text-sm font-medium" style="color: var(--text-primary)">粗粒化结果预览 (Z={{ pfSliceZ }} 切片)</h4>
                <div class="flex items-center gap-2">
                  <label class="text-[10px]" style="color: var(--text-muted)">切片 Z:</label>
                  <input v-model.number="pfSliceZ" type="range" min="0" :max="pfConfig.gridNz - 1" step="1" class="w-24" />
                  <span class="text-xs" style="color: var(--text-secondary)">{{ pfSliceZ }}</span>
                </div>
              </div>
              <canvas ref="pfCanvas" width="480" height="480" class="w-full rounded border" style="border-color: var(--border-subtle); max-width: 480px; image-rendering: pixelated;"></canvas>
              <!-- Color Legend -->
              <div class="flex items-center gap-2 mt-2">
                <span class="text-[10px]" style="color: var(--text-muted)">{{ pfFieldMin.toFixed(3) }}</span>
                <div class="flex-1 h-3 rounded" :style="{ background: 'linear-gradient(to right, #3b82f6, #22c55e, #eab308, #ef4444)' }"></div>
                <span class="text-[10px]" style="color: var(--text-muted)">{{ pfFieldMax.toFixed(3) }}</span>
              </div>
            </div>
            <!-- Stats -->
            <div class="grid grid-cols-2 gap-3">
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">源原子数</div>
                <div class="text-lg font-semibold" style="color: var(--primary)">{{ pfResult.coarse_graining_stats.num_atoms_source.toLocaleString() }}</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">网格点数</div>
                <div class="text-lg font-semibold" style="color: var(--accent-green)">{{ pfResult.coarse_graining_stats.num_grid_points.toLocaleString() }}</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">降维比</div>
                <div class="text-lg font-semibold" style="color: var(--accent-yellow)">{{ pfResult.coarse_graining_stats.reduction_ratio.toFixed(1) }}x</div>
              </div>
              <div class="rounded-lg border p-3" style="background: var(--bg-surface); border-color: var(--border-subtle)">
                <div class="text-[10px] mb-1" style="color: var(--text-muted)">质量守恒误差</div>
                <div class="text-lg font-semibold" :style="{ color: pfResult.coarse_graining_stats.mass_conservation_error < 0.01 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                  {{ (pfResult.coarse_graining_stats.mass_conservation_error * 100).toFixed(3) }}%
                </div>
              </div>
            </div>
            <!-- Field names -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">输出场变量</h4>
              <div class="flex flex-wrap gap-2">
                <span v-for="fn in pfResult.field_names" :key="fn"
                  class="px-2 py-1 rounded text-xs" style="background: var(--bg-elevated); color: var(--text-secondary)">
                  {{ fn }}
                </span>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">配置参数后点击"粗粒化到相场"查看结果</p>
          </div>
        </template>

        <!-- FE Results -->
        <template v-if="activeTab === 'fe'">
          <div v-if="feResult" class="space-y-4">
            <!-- Averaged Stress Tensor -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">平均应力张量 (GPa)</h4>
              <div class="grid grid-cols-3 gap-2 max-w-md">
                <div v-for="(val, idx) in feResult.averaged_stress_tensor" :key="'st'+idx"
                  class="rounded p-2 text-center" style="background: var(--bg-elevated)">
                  <div class="text-[10px] mb-1" style="color: var(--text-muted)">{{ stressLabels[idx] }}</div>
                  <div class="text-sm font-semibold" style="color: var(--text-primary)">{{ val.toFixed(4) }}</div>
                </div>
              </div>
            </div>
            <!-- Temperature Field Preview -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">温度场预览</h4>
              <canvas ref="feTempCanvas" width="400" height="300" class="w-full rounded border" style="border-color: var(--border-subtle); max-width: 400px; image-rendering: pixelated;"></canvas>
              <div class="flex items-center gap-2 mt-2">
                <span class="text-[10px]" style="color: var(--text-muted)">{{ feTempMin.toFixed(1) }} K</span>
                <div class="flex-1 h-3 rounded" :style="{ background: 'linear-gradient(to right, #3b82f6, #22c55e, #eab308, #ef4444)' }"></div>
                <span class="text-[10px]" style="color: var(--text-muted)">{{ feTempMax.toFixed(1) }} K</span>
              </div>
            </div>
            <!-- Mapping Quality -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">映射质量指标</h4>
              <div class="grid grid-cols-2 gap-3">
                <div v-for="(item, idx) in feQualityItems" :key="'q'+idx"
                  class="rounded-lg border p-3" style="background: var(--bg-elevated); border-color: var(--border-subtle)">
                  <div class="text-[10px] mb-1" style="color: var(--text-muted)">{{ item.label }}</div>
                  <div class="flex items-center gap-2">
                    <div class="flex-1 h-2 rounded-full overflow-hidden" style="background: var(--bg-base)">
                      <div class="h-full rounded-full transition-all" :style="{ width: (item.value * 100) + '%', background: item.color }"></div>
                    </div>
                    <span class="text-xs font-semibold" :style="{ color: item.color }">{{ (item.value * 100).toFixed(1) }}%</span>
                  </div>
                </div>
              </div>
            </div>
            <!-- Boundary Conditions -->
            <div class="rounded-lg border p-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <h4 class="text-sm font-medium mb-2" style="color: var(--text-primary)">边界条件 (前10条)</h4>
              <div class="space-y-1 max-h-40 overflow-y-auto">
                <div v-for="(bc, idx) in feResult.fe_boundary_conditions.slice(0, 10)" :key="'bc'+idx"
                  class="flex items-center justify-between text-xs px-2 py-1 rounded" style="background: var(--bg-elevated)">
                  <span style="color: var(--text-secondary)">节点 {{ bc.node_id }}</span>
                  <span style="color: var(--text-primary)">{{ bc.dof }} = {{ bc.value.toFixed(6) }}</span>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="flex items-center justify-center h-64" style="color: var(--text-muted)">
            <p class="text-sm">配置参数后点击"映射到 FE"查看结果</p>
          </div>
        </template>

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch, nextTick } from 'vue'
import type {
  RdfResult,
  MsdResult,
  VacfResult,
  MdToPhaseFieldResult,
  MdToFeResult
} from '../api/mdPostProcess'

// ============ Tab State ============

type TabValue = 'rdf' | 'msd' | 'vacf' | 'phasefield' | 'fe'

const tabs = [
  { value: 'rdf' as TabValue, label: 'RDF' },
  { value: 'msd' as TabValue, label: 'MSD' },
  { value: 'vacf' as TabValue, label: 'VACF' },
  { value: 'phasefield' as TabValue, label: 'MD→相场' },
  { value: 'fe' as TabValue, label: 'MD→FE' }
]

const activeTab = ref<TabValue>('rdf')

// ============ RDF State ============

const rdfParams = reactive({
  elementPair: 'Al-Al',
  customElementA: '',
  customElementB: '',
  rMax: 10.0,
  binWidth: 0.05
})

const rdfResult = ref<RdfResult | null>(null)

// SVG dimensions
const rdfSvgWidth = 600
const rdfSvgHeight = 300
const rdfSvgPadL = 45
const rdfSvgPadT = 20
const rdfSvgW = rdfSvgWidth - rdfSvgPadL - 20
const rdfSvgH = rdfSvgHeight - rdfSvgPadT - 30

const rdfYMax = computed(() => {
  if (!rdfResult.value) return 5
  const maxGr = Math.max(...rdfResult.value.bins.map(b => b.g_r))
  return Math.ceil(maxGr * 1.15 * 10) / 10
})

function getRdfYAtR(r: number): number {
  if (!rdfResult.value) return 0
  const bin = rdfResult.value.bins.find(b => Math.abs(b.r_A - r) < rdfParams.binWidth)
  return bin ? bin.g_r : 0
}

const rdfCurvePoints = computed(() => {
  if (!rdfResult.value) return ''
  return rdfResult.value.bins.map(b => {
    const x = rdfSvgPadL + (b.r_A / rdfParams.rMax) * rdfSvgW
    const y = rdfSvgPadT + rdfSvgH - (b.g_r / rdfYMax.value) * rdfSvgH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// ============ MSD State ============

const msdParams = reactive({
  element: 'Al',
  startFs: 0,
  endFs: 50000
})

const msdResult = ref<MsdResult | null>(null)

const msdSvgWidth = 600
const msdSvgHeight = 300
const msdSvgPadL = 50
const msdSvgPadT = 20
const msdSvgW = msdSvgWidth - msdSvgPadL - 20
const msdSvgH = msdSvgHeight - msdSvgPadT - 30

const msdYMax = computed(() => {
  if (!msdResult.value) return 10
  const maxMsd = Math.max(...msdResult.value.time_points.map(p => p.msd_A2))
  return Math.ceil(maxMsd * 1.15)
})

const msdCurvePoints = computed(() => {
  if (!msdResult.value) return ''
  const maxTime = msdParams.endFs
  return msdResult.value.time_points.map(p => {
    const x = msdSvgPadL + (p.time_fs / maxTime) * msdSvgW
    const y = msdSvgPadT + msdSvgH - (p.msd_A2 / msdYMax.value) * msdSvgH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// ============ VACF State ============

const vacfParams = reactive({
  element: 'Al'
})

const vacfResult = ref<VacfResult | null>(null)

const vacfSvgWidth = 600
const vacfSvgHeight = 280
const vacfSvgPadL = 45
const vacfSvgPadT = 20
const vacfSvgW = vacfSvgWidth - vacfSvgPadL - 20
const vacfSvgH = vacfSvgHeight - vacfSvgPadT - 30

const vacfMaxTime = 2000

const vacfCurvePoints = computed(() => {
  if (!vacfResult.value) return ''
  return vacfResult.value.time_points.map(p => {
    const x = vacfSvgPadL + (p.time_fs / vacfMaxTime) * vacfSvgW
    const y = vacfSvgPadT + vacfSvgH / 2 - (p.vacf / 1.0) * (vacfSvgH / 2)
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

const vdosSvgWidth = 600
const vdosSvgHeight = 280
const vdosSvgPadL = 45
const vdosSvgPadT = 20
const vdosSvgW = vdosSvgWidth - vdosSvgPadL - 20
const vdosSvgH = vdosSvgHeight - vdosSvgPadT - 30

const vdosMaxFreq = 30

const vdosYMax = computed(() => {
  if (!vacfResult.value) return 1
  const maxVdos = Math.max(...vacfResult.value.vibrational_density_of_states.map(p => p.vdos))
  return Math.ceil(maxVdos * 1.15 * 100) / 100
})

const vdosCurvePoints = computed(() => {
  if (!vacfResult.value) return ''
  return vacfResult.value.vibrational_density_of_states.map(p => {
    const x = vdosSvgPadL + (p.frequency_THz / vdosMaxFreq) * vdosSvgW
    const y = vdosSvgPadT + vdosSvgH - (p.vdos / vdosYMax.value) * vdosSvgH
    return `${x.toFixed(1)},${y.toFixed(1)}`
  }).join(' ')
})

// ============ PhaseField State ============

const pfConfig = reactive({
  coarseGrainingMethod: 'density_field' as 'qc' | 'mqc' | 'density_field' | 'order_parameter',
  gridNx: 50,
  gridNy: 50,
  gridNz: 50,
  smoothingLength: 2.0,
  outputFields: ['density'] as string[]
})

const pfResult = ref<MdToPhaseFieldResult | null>(null)
const pfSliceZ = ref(25)
const pfCanvas = ref<HTMLCanvasElement | null>(null)

const pfOutputFields = [
  { value: 'density', label: '密度场' },
  { value: 'order_parameter', label: '序参量' },
  { value: 'stress', label: '应力场' },
  { value: 'temperature', label: '温度场' }
]

const cgMethods = [
  { value: 'qc' as const, label: '准连续介质法 (QC)', desc: '耦合原子区与连续介质区' },
  { value: 'mqc' as const, label: '多尺度 QC (MQC)', desc: '改进的准连续介质方法' },
  { value: 'density_field' as const, label: '密度场方法', desc: '基于原子密度的粗粒化' },
  { value: 'order_parameter' as const, label: '序参量方法', desc: '基于序参量的相场描述' }
]

const pfFieldMin = computed(() => {
  if (!pfResult.value || pfResult.value.grid_data.length === 0) return 0
  const sliceData = pfResult.value.grid_data.filter(d => d.iz === pfSliceZ.value)
  if (sliceData.length === 0) return 0
  const vals = sliceData.map(d => d.fields['density'] ?? 0)
  return Math.min(...vals)
})

const pfFieldMax = computed(() => {
  if (!pfResult.value || pfResult.value.grid_data.length === 0) return 1
  const sliceData = pfResult.value.grid_data.filter(d => d.iz === pfSliceZ.value)
  if (sliceData.length === 0) return 1
  const vals = sliceData.map(d => d.fields['density'] ?? 0)
  return Math.max(...vals)
})

// ============ FE State ============

const feConfig = reactive({
  meshSize: 5.0,
  stressMethod: 'virial' as 'virial' | 'hardy' | 'irving_kirkwood',
  outputType: 'boundary_condition' as 'boundary_condition' | 'initial_condition' | 'material_property'
})

const feResult = ref<MdToFeResult | null>(null)
const feTempCanvas = ref<HTMLCanvasElement | null>(null)

const stressMethods = [
  { value: 'virial' as const, label: 'Virial 应力', desc: '基于 Virial 定理的应力张量' },
  { value: 'hardy' as const, label: 'Hardy 应力', desc: '基于 Hardy 公式的局部应力' },
  { value: 'irving_kirkwood' as const, label: 'Irving-Kirkwood', desc: '基于 IK 公式的连续介质应力' }
]

const feOutputTypes = [
  { value: 'boundary_condition' as const, label: '边界条件', desc: '生成 FE 边界条件' },
  { value: 'initial_condition' as const, label: '初始条件', desc: '生成 FE 初始条件' },
  { value: 'material_property' as const, label: '材料属性', desc: '提取等效材料属性' }
]

const stressLabels = ['σ_xx', 'σ_yy', 'σ_zz', 'τ_xy', 'τ_yz', 'τ_xz']

const feTempMin = computed(() => {
  if (!feResult.value || feResult.value.fe_temperature_field.length === 0) return 0
  return Math.min(...feResult.value.fe_temperature_field.map(p => p.temperature))
})

const feTempMax = computed(() => {
  if (!feResult.value || feResult.value.fe_temperature_field.length === 0) return 1
  return Math.max(...feResult.value.fe_temperature_field.map(p => p.temperature))
})

const feQualityItems = computed(() => {
  if (!feResult.value) return []
  const mq = feResult.value.mapping_quality
  return [
    { label: '空间相关性', value: mq.spatial_correlation, color: mq.spatial_correlation > 0.9 ? 'var(--accent-green)' : 'var(--accent-yellow)' },
    { label: '能量守恒', value: mq.energy_conservation, color: mq.energy_conservation > 0.95 ? 'var(--accent-green)' : 'var(--accent-yellow)' },
    { label: '网格收敛性', value: mq.mesh_convergence, color: mq.mesh_convergence > 0.9 ? 'var(--accent-green)' : 'var(--accent-yellow)' },
    { label: '综合评分', value: mq.overall_score, color: mq.overall_score > 0.9 ? 'var(--accent-green)' : 'var(--accent-red)' }
  ]
})

// ============ Has Results ============

const hasResults = computed(() => {
  return rdfResult.value || msdResult.value || vacfResult.value || pfResult.value || feResult.value
})

// ============ Quick Templates ============

const quickTemplates = [
  { id: 'liq-al-rdf', name: '液态Al RDF', tab: 'rdf' as TabValue },
  { id: 'cu-diffusion', name: 'Cu 扩散', tab: 'msd' as TabValue },
  { id: 'fe-gb-pf', name: 'Fe 晶界偏析→相场', tab: 'phasefield' as TabValue },
  { id: 'nano-indent-fe', name: '纳米压痕→FE边界', tab: 'fe' as TabValue }
]

function applyTemplate(tpl: { id: string; name: string; tab: TabValue }) {
  activeTab.value = tpl.tab
  switch (tpl.id) {
    case 'liq-al-rdf':
      rdfParams.elementPair = 'Al-Al'
      rdfParams.rMax = 10.0
      rdfParams.binWidth = 0.05
      runRdf()
      break
    case 'cu-diffusion':
      msdParams.element = 'Cu'
      msdParams.startFs = 0
      msdParams.endFs = 80000
      runMsd()
      break
    case 'fe-gb-pf':
      pfConfig.coarseGrainingMethod = 'order_parameter'
      pfConfig.gridNx = 60
      pfConfig.gridNy = 60
      pfConfig.gridNz = 60
      pfConfig.smoothingLength = 2.5
      pfConfig.outputFields = ['density', 'order_parameter']
      runPhaseField()
      break
    case 'nano-indent-fe':
      feConfig.meshSize = 3.0
      feConfig.stressMethod = 'virial'
      feConfig.outputType = 'boundary_condition'
      runFeMapping()
      break
  }
}

// ============ Simulated Data Generators ============

function generateRdfData(pair: string, rMax: number, binWidth: number): RdfResult {
  const numBins = Math.floor(rMax / binWidth)
  const bins: Array<{ r_A: number; g_r: number; coordination_number: number }> = []
  const peakPositions: number[] = []
  let cumCoord = 0

  // Hard-sphere model parameters based on element pair
  const sigma = pair.includes('C') ? 1.54 : pair.includes('Fe') ? 2.48 : pair.includes('Cu') ? 2.56 : pair.includes('Si') ? 2.35 : 2.86
  const peakHeight1 = pair.includes('C') ? 3.5 : 2.8
  const peakHeight2 = peakHeight1 * 0.55
  const peakHeight3 = peakHeight1 * 0.35
  const peak1R = sigma
  const peak2R = sigma * 1.73
  const peak3R = sigma * 2.0
  const peakWidth = sigma * 0.12

  for (let i = 0; i < numBins; i++) {
    const r = (i + 0.5) * binWidth
    // Hard-sphere: g(r)=0 for r < sigma, then peaks
    let gr = 0
    if (r >= sigma * 0.85) {
      gr = 1.0
        + (peakHeight1 - 1) * Math.exp(-Math.pow((r - peak1R) / peakWidth, 2))
        + (peakHeight2 - 1) * Math.exp(-Math.pow((r - peak2R) / (peakWidth * 1.3), 2))
        + (peakHeight3 - 1) * Math.exp(-Math.pow((r - peak3R) / (peakWidth * 1.5), 2))
      // Add small noise
      gr += (Math.random() - 0.5) * 0.05
      gr = Math.max(0, gr)
    }
    const rho = 0.08
    cumCoord += rho * 4 * Math.PI * r * r * gr * binWidth
    bins.push({ r_A: r, g_r: gr, coordination_number: cumCoord })
  }

  // Find peaks
  for (let i = 2; i < bins.length - 2; i++) {
    if (bins[i].g_r > bins[i - 1].g_r && bins[i].g_r > bins[i + 1].g_r && bins[i].g_r > 1.5) {
      peakPositions.push(bins[i].r_A)
    }
  }

  const firstPeakBin = bins.reduce((best, b) => b.g_r > best.g_r ? b : best, bins[0])

  return {
    success: true,
    element_pair: pair,
    bins,
    peak_positions: peakPositions.slice(0, 6),
    first_peak_height: firstPeakBin.g_r,
    first_peak_position: firstPeakBin.r_A,
    coordination_number: bins[Math.floor(bins.length * 0.3)].coordination_number
  }
}

function generateMsdData(element: string, startFs: number, endFs: number): MsdResult {
  // D values in A^2/fs for different elements (liquid state)
  const dValues: Record<string, number> = {
    Al: 0.0085,
    Cu: 0.0052,
    Fe: 0.0041,
    Si: 0.0028,
    C: 0.0019,
    Ni: 0.0048
  }
  const D = dValues[element] || 0.005
  const numPoints = 100
  const dt = (endFs - startFs) / numPoints
  const timePoints: Array<{ time_fs: number; msd_A2: number }> = []

  for (let i = 0; i <= numPoints; i++) {
    const t = startFs + i * dt
    const msd = 6 * D * t + (Math.random() - 0.5) * 0.02 * t
    timePoints.push({ time_fs: t, msd_A2: Math.max(0, msd) })
  }

  // Linear fit (skip first 10% as equilibration)
  const fitStart = Math.floor(numPoints * 0.1)
  const fitPoints = timePoints.slice(fitStart)
  const n = fitPoints.length
  let sumX = 0, sumY = 0, sumXY = 0, sumX2 = 0, sumY2 = 0
  for (const p of fitPoints) {
    sumX += p.time_fs
    sumY += p.msd_A2
    sumXY += p.time_fs * p.msd_A2
    sumX2 += p.time_fs * p.time_fs
    sumY2 += p.msd_A2 * p.msd_A2
  }
  const slope = (n * sumXY - sumX * sumY) / (n * sumX2 - sumX * sumX)
  const intercept = (sumY - slope * sumX) / n
  const rSquared = Math.pow((n * sumXY - sumX * sumY) / Math.sqrt((n * sumX2 - sumX * sumX) * (n * sumY2 - sumY * sumY)), 2)

  // Convert D from A^2/fs to cm^2/s: 1 A^2/fs = 1e-16 / 1e-15 = 1e-1 cm^2/s
  const D_cm2_s = (slope / 6) * 1e-1

  return {
    success: true,
    element,
    time_points: timePoints,
    diffusion_coefficient_cm2_s: D_cm2_s,
    diffusion_coefficient_A2_fs: slope / 6,
    fit_slope: slope,
    fit_intercept: intercept,
    r_squared: Math.min(1, rSquared)
  }
}

function generateVacfData(element: string): VacfResult {
  // Relaxation time depends on element mass
  const tauValues: Record<string, number> = {
    Al: 120, Cu: 180, Fe: 200, Si: 250, C: 150, Ni: 190
  }
  const tau = tauValues[element] || 150
  const numPoints = 200
  const maxTime = vacfMaxTime
  const dt = maxTime / numPoints
  const timePoints: Array<{ time_fs: number; vacf: number }> = []

  for (let i = 0; i <= numPoints; i++) {
    const t = i * dt
    const vacf = Math.exp(-t / tau) * (1 + 0.1 * Math.sin(2 * Math.PI * t / (tau * 0.3)) * Math.exp(-t / (tau * 2)))
    timePoints.push({ time_fs: t, vacf })
  }

  // Generate VDOS via Fourier transform of VACF
  const numFreq = 150
  const maxFreq = vdosMaxFreq
  const df = maxFreq / numFreq
  const vdos: Array<{ frequency_THz: number; vdos: number }> = []

  for (let j = 0; j <= numFreq; j++) {
    const freq = j * df
    let realPart = 0
    let imagPart = 0
    for (let i = 0; i < timePoints.length; i++) {
      const t = timePoints[i].time_fs * 1e-3 // convert fs to ps
      const omega = 2 * Math.PI * freq
      realPart += timePoints[i].vacf * Math.cos(omega * t) * dt * 1e-3
      imagPart += timePoints[i].vacf * Math.sin(omega * t) * dt * 1e-3
    }
    const vdosVal = Math.sqrt(realPart * realPart + imagPart * imagPart)
    // Add Debye-like behavior: VDOS ~ omega^2 at low freq
    const debyeFactor = Math.pow(freq / 5, 2) / (1 + Math.pow(freq / 5, 2))
    vdos.push({ frequency_THz: freq, vdos: vdosVal * debyeFactor * 50 })
  }

  return {
    success: true,
    element,
    time_points: timePoints,
    velocity_autocorrelation: timePoints[0].vacf,
    vibrational_density_of_states: vdos
  }
}

function generatePhaseFieldData(config: typeof pfConfig): MdToPhaseFieldResult {
  const { gridNx, gridNy, gridNz, smoothingLength } = config
  const gridData: MdToPhaseFieldResult['grid_data'] = []
  const fieldNames = [...config.outputFields]

  // Generate Gaussian-smoothed density field with some structure
  // Create random "atom" positions for the density field
  const numAtoms = 200
  const atoms: Array<{ x: number; y: number; z: number }> = []
  for (let i = 0; i < numAtoms; i++) {
    atoms.push({
      x: Math.random() * gridNx,
      y: Math.random() * gridNy,
      z: Math.random() * gridNz
    })
  }

  // Add some clustering
  for (let c = 0; c < 3; c++) {
    const cx = Math.random() * gridNx
    const cy = Math.random() * gridNy
    const cz = Math.random() * gridNz
    for (let i = 0; i < 30; i++) {
      atoms.push({
        x: cx + (Math.random() - 0.5) * gridNx * 0.2,
        y: cy + (Math.random() - 0.5) * gridNy * 0.2,
        z: cz + (Math.random() - 0.5) * gridNz * 0.2
      })
    }
  }

  const sigma = smoothingLength
  const sigma2 = 2 * sigma * sigma

  for (let iz = 0; iz < gridNz; iz++) {
    for (let iy = 0; iy < gridNy; iy++) {
      for (let ix = 0; ix < gridNx; ix++) {
        const fields: Record<string, number> = {}
        // Density field: sum of Gaussians
        let density = 0
        for (const atom of atoms) {
          const dx = ix - atom.x
          const dy = iy - atom.y
          const dz = iz - atom.z
          const r2 = dx * dx + dy * dy + dz * dz
          density += Math.exp(-r2 / sigma2)
        }
        density /= (Math.pow(2 * Math.PI, 1.5) * sigma * sigma * sigma)
        density *= gridNx * gridNy * gridNz * 0.05
        fields['density'] = density

        // Order parameter: based on local density variation
        fields['order_parameter'] = Math.tanh((density - 0.5) * 4) * (0.8 + Math.random() * 0.4)

        // Stress field
        fields['stress'] = density * 2.5 + (Math.random() - 0.5) * 0.1

        // Temperature field
        fields['temperature'] = 300 + density * 200 + (Math.random() - 0.5) * 10

        // Only include requested fields
        const filteredFields: Record<string, number> = {}
        for (const fn of fieldNames) {
          if (fields[fn] !== undefined) {
            filteredFields[fn] = fields[fn]
          }
        }

        gridData.push({ ix, iy, iz, fields: filteredFields })
      }
    }
  }

  const numAtomsSource = gridNx * gridNy * gridNz * 4
  const numGridPoints = gridNx * gridNy * gridNz

  return {
    success: true,
    grid_data: gridData,
    grid_size: { nx: gridNx, ny: gridNy, nz: gridNz },
    field_names: fieldNames,
    smoothing_info: {
      method: config.coarseGrainingMethod,
      kernel_width: smoothingLength,
      effective_resolution: smoothingLength * 2
    },
    coarse_graining_stats: {
      num_atoms_source: numAtomsSource,
      num_grid_points: numGridPoints,
      reduction_ratio: numAtomsSource / numGridPoints,
      mass_conservation_error: 0.002 + Math.random() * 0.005
    }
  }
}

function generateFeData(config: typeof feConfig): MdToFeResult {
  const meshSize = config.meshSize
  const boxSize = 50
  const numNodes = Math.floor(boxSize / meshSize)
  const stressField: MdToFeResult['fe_stress_field'] = []
  const tempField: MdToFeResult['fe_temperature_field'] = []
  const bcs: MdToFeResult['fe_boundary_conditions'] = []

  const baseStress = [1.5, 1.5, 1.5, 0.1, 0.05, 0.08] // GPa

  for (let iz = 0; iz <= numNodes; iz++) {
    for (let iy = 0; iy <= numNodes; iy++) {
      for (let ix = 0; ix <= numNodes; ix++) {
        const x = ix * meshSize
        const y = iy * meshSize
        const z = iz * meshSize
        const noise = () => (Math.random() - 0.5) * 0.15

        stressField.push({
          x, y, z,
          sigma_xx: baseStress[0] + noise(),
          sigma_yy: baseStress[1] + noise(),
          sigma_zz: baseStress[2] + noise(),
          tau_xy: baseStress[3] + noise() * 0.5,
          tau_yz: baseStress[4] + noise() * 0.5,
          tau_xz: baseStress[5] + noise() * 0.5
        })

        tempField.push({
          x, y, z,
          temperature: 300 + 50 * Math.sin(x / boxSize * Math.PI) * Math.cos(y / boxSize * Math.PI) + (Math.random() - 0.5) * 5
        })
      }
    }
  }

  // Generate boundary conditions for surface nodes
  let bcId = 0
  for (let iy = 0; iy <= numNodes; iy++) {
    for (let iz = 0; iz <= numNodes; iz++) {
      // x=0 face: fix x displacement
      bcs.push({ node_id: bcId++, dof: 'ux', value: 0 })
      // x=boxSize face: apply displacement
      const disp = 0.5 + (Math.random() - 0.5) * 0.02
      bcs.push({ node_id: bcId++, dof: 'ux', value: disp })
    }
  }

  // Average stress tensor
  const avgStress: [number, number, number, number, number, number] = [
    baseStress[0] + (Math.random() - 0.5) * 0.05,
    baseStress[1] + (Math.random() - 0.5) * 0.05,
    baseStress[2] + (Math.random() - 0.5) * 0.05,
    baseStress[3] + (Math.random() - 0.5) * 0.02,
    baseStress[4] + (Math.random() - 0.5) * 0.02,
    baseStress[5] + (Math.random() - 0.5) * 0.02
  ]

  return {
    success: true,
    fe_stress_field: stressField,
    fe_temperature_field: tempField,
    fe_boundary_conditions: bcs,
    averaged_stress_tensor: avgStress,
    mapping_quality: {
      spatial_correlation: 0.92 + Math.random() * 0.06,
      energy_conservation: 0.96 + Math.random() * 0.03,
      mesh_convergence: 0.91 + Math.random() * 0.07,
      overall_score: 0.93 + Math.random() * 0.05
    }
  }
}

// ============ Run Functions ============

function runRdf() {
  const pair = rdfParams.elementPair === 'custom'
    ? `${rdfParams.customElementA}-${rdfParams.customElementB}`
    : rdfParams.elementPair
  rdfResult.value = generateRdfData(pair, rdfParams.rMax, rdfParams.binWidth)
}

function runMsd() {
  msdResult.value = generateMsdData(msdParams.element, msdParams.startFs, msdParams.endFs)
}

function runVacf() {
  vacfResult.value = generateVacfData(vacfParams.element)
}

function runPhaseField() {
  pfResult.value = generatePhaseFieldData(pfConfig)
  pfSliceZ.value = Math.floor(pfConfig.gridNz / 2)
  nextTick(() => drawPhaseFieldCanvas())
}

function runFeMapping() {
  feResult.value = generateFeData(feConfig)
  nextTick(() => drawFeTempCanvas())
}

// ============ Canvas Drawing ============

function heatColor(t: number): string {
  // t in [0, 1]: blue -> green -> yellow -> red
  t = Math.max(0, Math.min(1, t))
  let r: number, g: number, b: number
  if (t < 0.33) {
    const s = t / 0.33
    r = Math.round(59 * (1 - s) + 34 * s)
    g = Math.round(130 * (1 - s) + 197 * s)
    b = Math.round(246 * (1 - s) + 94 * s)
  } else if (t < 0.66) {
    const s = (t - 0.33) / 0.33
    r = Math.round(34 * (1 - s) + 234 * s)
    g = Math.round(197 * (1 - s) + 179 * s)
    b = Math.round(94 * (1 - s) + 8 * s)
  } else {
    const s = (t - 0.66) / 0.34
    r = Math.round(234 * (1 - s) + 239 * s)
    g = Math.round(179 * (1 - s) + 68 * s)
    b = Math.round(8 * (1 - s) + 68 * s)
  }
  return `rgb(${r},${g},${b})`
}

function drawPhaseFieldCanvas() {
  if (!pfCanvas.value || !pfResult.value) return
  const canvas = pfCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const { nx: gridNx, ny: gridNy } = pfResult.value.grid_size
  const sliceData = pfResult.value.grid_data.filter(d => d.iz === pfSliceZ.value)

  if (sliceData.length === 0) return

  const fieldName = pfResult.value.field_names[0] || 'density'
  const vals = sliceData.map(d => d.fields[fieldName] ?? 0)
  const minVal = Math.min(...vals)
  const maxVal = Math.max(...vals)
  const range = maxVal - minVal || 1

  const cellW = canvas.width / gridNx
  const cellH = canvas.height / gridNy

  ctx.clearRect(0, 0, canvas.width, canvas.height)

  for (const point of sliceData) {
    const val = point.fields[fieldName] ?? 0
    const t = (val - minVal) / range
    ctx.fillStyle = heatColor(t)
    ctx.fillRect(point.ix * cellW, point.iy * cellH, Math.ceil(cellW), Math.ceil(cellH))
  }
}

function drawFeTempCanvas() {
  if (!feTempCanvas.value || !feResult.value) return
  const canvas = feTempCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const tempField = feResult.value.fe_temperature_field
  if (tempField.length === 0) return

  const temps = tempField.map(p => p.temperature)
  const minT = Math.min(...temps)
  const maxT = Math.max(...temps)
  const range = maxT - minT || 1

  // Get unique x and y positions
  const xPositions = [...new Set(tempField.map(p => p.x))].sort((a, b) => a - b)
  const yPositions = [...new Set(tempField.map(p => p.y))].sort((a, b) => a - b)
  const nx = xPositions.length
  const ny = yPositions.length

  const cellW = canvas.width / nx
  const cellH = canvas.height / ny

  ctx.clearRect(0, 0, canvas.width, canvas.height)

  for (const point of tempField) {
    const ix = xPositions.indexOf(point.x)
    const iy = yPositions.indexOf(point.y)
    if (ix < 0 || iy < 0) continue
    const t = (point.temperature - minT) / range
    ctx.fillStyle = heatColor(t)
    ctx.fillRect(ix * cellW, iy * cellH, Math.ceil(cellW), Math.ceil(cellH))
  }
}

// Watch for slice changes
watch(pfSliceZ, () => {
  nextTick(() => drawPhaseFieldCanvas())
})

// ============ Reset ============

function resetAll() {
  rdfResult.value = null
  msdResult.value = null
  vacfResult.value = null
  pfResult.value = null
  feResult.value = null
  rdfParams.elementPair = 'Al-Al'
  rdfParams.rMax = 10.0
  rdfParams.binWidth = 0.05
  msdParams.element = 'Al'
  msdParams.startFs = 0
  msdParams.endFs = 50000
  vacfParams.element = 'Al'
  pfConfig.coarseGrainingMethod = 'density_field'
  pfConfig.gridNx = 50
  pfConfig.gridNy = 50
  pfConfig.gridNz = 50
  pfConfig.smoothingLength = 2.0
  pfConfig.outputFields = ['density']
  feConfig.meshSize = 5.0
  feConfig.stressMethod = 'virial'
  feConfig.outputType = 'boundary_condition'
}

// ============ Export ============

function exportResults() {
  const data: Record<string, unknown> = {}
  if (rdfResult.value) data.rdf = rdfResult.value
  if (msdResult.value) data.msd = msdResult.value
  if (vacfResult.value) data.vacf = vacfResult.value
  if (pfResult.value) data.phaseField = pfResult.value
  if (feResult.value) data.fe = feResult.value

  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'md_postprocess_results.json'
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

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
}
</style>
