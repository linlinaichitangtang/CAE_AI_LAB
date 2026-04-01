<!--
  ThermalCouplingView.vue - 热-结构耦合分析视图
  ==========================================
  
  核心功能:
  - 提供热-结构耦合分析的全流程配置与执行界面
  - 支持两种耦合方式: 顺序耦合(先热分析→后结构分析) 和 完全耦合
  - 集成稳态/瞬态热分析 + 热膨胀应力计算
  - 3D可视化显示温度云图、热应力分布、位移变形
  - 支持嵌入到笔记功能
  - 提供多种预设模板(电子封装、PCB、芯片等)
  
  分析流程:
  1. 网格设置 → 2. 材料属性 → 3. 热分析设置 → 4. 热边界条件 → 5. 热源 → 6. 结构设置 → 7. 结构边界条件
  2. 调用后端API生成INP文件 → 调用CalculiX求解器 → 解析结果 → 可视化显示
-->

<template>
  <div class="thermal-coupling-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">热-结构耦合分析</h2>
        <p class="text-xs text-[var(--text-muted)]">多物理场耦合：稳态/瞬态热分析 → 热膨胀应力计算</p>
      </div>
      <div class="flex items-center gap-2">
        <select v-model="couplingType" class="px-3 py-1.5 text-sm border border-blue-300 rounded">
          <option value="sequential">顺序耦合(热→结构)</option>
          <option value="fully_coupled">完全耦合</option>
        </select>
        <button @click="showTemplateDialog = true" class="btn btn-ghost text-xs border border-orange-300 text-orange-600 hover:bg-orange-50">
          <span class="mr-1">📋</span> 模板
        </button>
        <button @click="showEmbedDialog = true" :disabled="!projectStore.currentNoteId" class="btn btn-ghost text-xs border border-purple-300 text-purple-600 hover:bg-purple-50 disabled:opacity-50">
          <span class="mr-1">🔗</span> 嵌入笔记
        </button>
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">⟳</span> 重置
        </button>
        <button @click="runCouplingAnalysis" :disabled="!canRun || running" class="btn btn-primary text-xs">
          <span v-if="running" class="mr-1 animate-spin">⟳</span>
          <span v-else class="mr-1">▶</span>
          {{ running ? '分析中...' : '运行分析' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Configuration Panel -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          耦合分析设置
        </div>
        <div class="flex-1 p-3 space-y-4">
          <!-- Step 1: Mesh Configuration -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-[10px] flex items-center justify-center">1</span>
              网格设置
            </h3>
            <div class="grid grid-cols-2 gap-2 pl-7">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">X范围 (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="meshConfig.xMin" type="number" step="1" class="input w-full text-xs" placeholder="min" />
                  <input v-model.number="meshConfig.xMax" type="number" step="1" class="input w-full text-xs" placeholder="max" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y范围 (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="meshConfig.yMin" type="number" step="1" class="input w-full text-xs" placeholder="min" />
                  <input v-model.number="meshConfig.yMax" type="number" step="1" class="input w-full text-xs" placeholder="max" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z范围 (mm)</label>
                <div class="flex gap-1">
                  <input v-model.number="meshConfig.zMin" type="number" step="0.1" class="input w-full text-xs" placeholder="min" />
                  <input v-model.number="meshConfig.zMax" type="number" step="0.1" class="input w-full text-xs" placeholder="max" />
                </div>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">单元类型</label>
                <select v-model="meshConfig.elementType" class="input w-full text-xs">
                  <option value="C3D8">C3D8 (8节点砖)</option>
                  <option value="C3D8R">C3D8R (减缩积分)</option>
                  <option value="C3D4">C3D4 (4节点四面体)</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">X 分格</label>
                <input v-model.number="meshConfig.xDiv" type="number" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Y 分格</label>
                <input v-model.number="meshConfig.yDiv" type="number" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">Z 分格</label>
                <input v-model.number="meshConfig.zDiv" type="number" min="1" class="input w-full text-xs" />
              </div>
            </div>
            <p class="text-[10px] text-[var(--text-muted)] pl-7">
              预计: {{ estimatedNodes }} 节点, {{ estimatedElements }} 单元
            </p>
          </div>

          <!-- Step 2: Material Properties -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-blue-600 text-white text-[10px] flex items-center justify-center">2</span>
              材料热-力学属性
            </h3>
            <div class="space-y-2 pl-7">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">材料名称</label>
                <select v-model="material.name" class="input w-full text-xs" @change="applyMaterialPreset">
                  <option value="custom">自定义</option>
                  <option value="Steel">钢材</option>
                  <option value="Aluminum">铝合金</option>
                  <option value="Copper">铜</option>
                  <option value="FR4">FR4 (PCB)</option>
                  <option value="Solder">焊料</option>
                  <option value="Silicon">硅</option>
                </select>
              </div>
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">密度 (kg/m³)</label>
                  <input v-model.number="material.density" type="number" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">弹性模量 (MPa)</label>
                  <input v-model.number="material.youngsModulus" type="number" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">泊松比</label>
                  <input v-model.number="material.poissonRatio" type="number" step="0.01" min="0" max="0.5" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">导热系数 W/(m·K)</label>
                  <input v-model.number="material.thermalConductivity" type="number" step="0.1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">比热 J/(kg·K)</label>
                  <input v-model.number="material.specificHeat" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">热膨胀系数 1/K</label>
                  <input v-model.number="material.expansionCoefficient" type="number" step="1e-7" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 3: Thermal Analysis Settings -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-orange-500 text-white text-[10px] flex items-center justify-center">3</span>
              热分析设置
            </h3>
            <div class="space-y-2 pl-7">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">分析类型</label>
                <select v-model="thermalConfig.analysisType" class="input w-full text-xs">
                  <option value="steady_state">稳态热分析</option>
                  <option value="transient">瞬态热分析</option>
                </select>
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">初始温度 (K)</label>
                <input v-model.number="thermalConfig.initialTemperature" type="number" class="input w-full text-xs" />
              </div>
              <div v-if="thermalConfig.analysisType === 'transient'" class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">时间周期 (s)</label>
                  <input v-model.number="thermalConfig.timePeriod" type="number" step="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">时间步长 (s)</label>
                  <input v-model.number="thermalConfig.timeIncrement" type="number" step="0.1" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 4: Thermal Boundary Conditions -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
                <span class="w-5 h-5 rounded-full bg-orange-500 text-white text-[10px] flex items-center justify-center">4</span>
                热边界条件
              </h3>
              <button @click="addThermalBC" class="text-[10px] text-blue-500 hover:text-blue-600">+ 添加</button>
            </div>
            <div class="space-y-2 pl-7">
              <div v-for="(bc, idx) in thermalBCs" :key="idx" class="p-2 bg-[var(--bg-base)] rounded text-xs">
                <div class="flex items-center justify-between mb-1">
                  <select v-model="bc.bcType" class="bg-transparent text-xs font-medium">
                    <option value="fixed_temperature">固定温度</option>
                    <option value="heat_flux">热流密度</option>
                    <option value="convection">对流换热</option>
                    <option value="radiation">热辐射</option>
                    <option value="insulation">绝热</option>
                  </select>
                  <button @click="removeThermalBC(idx)" class="text-red-400 hover:text-red-600">×</button>
                </div>
                <div class="grid grid-cols-2 gap-1">
                  <div v-if="['fixed_temperature'].includes(bc.bcType)">
                    <label class="text-[9px] text-gray-400">温度(K)</label>
                    <input v-model.number="bc.temperature" type="number" class="input w-full text-xs" />
                  </div>
                  <div v-if="['heat_flux'].includes(bc.bcType)">
                    <label class="text-[9px] text-gray-400">热流(W/m²)</label>
                    <input v-model.number="bc.heatFlux" type="number" class="input w-full text-xs" />
                  </div>
                  <div v-if="['convection', 'radiation'].includes(bc.bcType)">
                    <label class="text-[9px] text-gray-400">对流系数</label>
                    <input v-model.number="bc.filmCoefficient" type="number" step="0.1" class="input w-full text-xs" />
                  </div>
                  <div v-if="['convection', 'radiation'].includes(bc.bcType)">
                    <label class="text-[9px] text-gray-400">环境温度(K)</label>
                    <input v-model.number="bc.ambientTemperature" type="number" class="input w-full text-xs" />
                  </div>
                </div>
                <div class="mt-1">
                  <label class="text-[9px] text-gray-400">作用面</label>
                  <select v-model="bc.face" class="input w-full text-xs">
                    <option value="all">全部表面</option>
                    <option value="x_min">X最小面</option>
                    <option value="x_max">X最大面</option>
                    <option value="y_min">Y最小面</option>
                    <option value="y_max">Y最大面</option>
                    <option value="z_min">Z最小面</option>
                    <option value="z_max">Z最大面</option>
                  </select>
                </div>
              </div>
              <p v-if="thermalBCs.length === 0" class="text-[10px] text-gray-400">暂无热边界条件</p>
            </div>
          </div>

          <!-- Step 5: Heat Sources -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
                <span class="w-5 h-5 rounded-full bg-orange-500 text-white text-[10px] flex items-center justify-center">5</span>
                热源
              </h3>
              <button @click="addHeatSource" class="text-[10px] text-blue-500 hover:text-blue-600">+ 添加</button>
            </div>
            <div class="space-y-2 pl-7">
              <div v-for="(hs, idx) in heatSources" :key="idx" class="p-2 bg-[var(--bg-base)] rounded text-xs">
                <div class="flex items-center justify-between mb-1">
                  <select v-model="hs.sourceType" class="bg-transparent text-xs font-medium">
                    <option value="volume">体热源 (W/m³)</option>
                    <option value="surface">面热源 (W/m²)</option>
                    <option value="point">点热源 (W)</option>
                  </select>
                  <button @click="removeHeatSource(idx)" class="text-red-400 hover:text-red-600">×</button>
                </div>
                <div>
                  <label class="text-[9px] text-gray-400">热功率</label>
                  <input v-model.number="hs.magnitude" type="number" step="1" class="input w-full text-xs" />
                </div>
              </div>
              <p v-if="heatSources.length === 0" class="text-[10px] text-gray-400">暂无热源</p>
            </div>
          </div>

          <!-- Step 6: Structural Settings -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
              <span class="w-5 h-5 rounded-full bg-green-600 text-white text-[10px] flex items-center justify-center">6</span>
              结构分析设置
            </h3>
            <div class="space-y-2 pl-7">
              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">参考温度 (K)</label>
                  <input v-model.number="structuralConfig.referenceTemperature" type="number" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">无应力温度 (K)</label>
                  <input v-model.number="structuralConfig.stressFreeTemperature" type="number" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 7: Structural Boundary Conditions -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <h3 class="text-xs font-semibold text-[var(--text-primary)] flex items-center gap-2">
                <span class="w-5 h-5 rounded-full bg-green-600 text-white text-[10px] flex items-center justify-center">7</span>
                结构边界条件
              </h3>
              <button @click="addStructuralBC" class="text-[10px] text-blue-500 hover:text-blue-600">+ 添加</button>
            </div>
            <div class="space-y-2 pl-7">
              <div v-for="(bc, idx) in structuralBCs" :key="idx" class="p-2 bg-[var(--bg-base)] rounded text-xs">
                <div class="flex items-center justify-between mb-1">
                  <span class="font-medium">边界 {{ idx + 1 }}</span>
                  <button @click="removeStructuralBC(idx)" class="text-red-400 hover:text-red-600">×</button>
                </div>
                <div class="grid grid-cols-3 gap-1 mb-1">
                  <label class="flex items-center gap-1">
                    <input type="checkbox" v-model="bc.fixX" class="rounded text-[var(--accent)]" />
                    <span class="text-[10px]">X固定</span>
                  </label>
                  <label class="flex items-center gap-1">
                    <input type="checkbox" v-model="bc.fixY" class="rounded text-[var(--accent)]" />
                    <span class="text-[10px]">Y固定</span>
                  </label>
                  <label class="flex items-center gap-1">
                    <input type="checkbox" v-model="bc.fixZ" class="rounded text-[var(--accent)]" />
                    <span class="text-[10px]">Z固定</span>
                  </label>
                </div>
                <div>
                  <label class="text-[9px] text-gray-400">作用面</label>
                  <select v-model="bc.face" class="input w-full text-xs">
                    <option value="x_min">X最小面</option>
                    <option value="x_max">X最大面</option>
                    <option value="y_min">Y最小面</option>
                    <option value="y_max">Y最大面</option>
                    <option value="z_min">Z最小面</option>
                    <option value="z_max">Z最大面</option>
                    <option value="all">全部</option>
                  </select>
                </div>
              </div>
              <p v-if="structuralBCs.length === 0" class="text-[10px] text-gray-400">暂无结构边界条件</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Center: 3D Visualization -->
      <div class="flex-1 flex flex-col relative">
        <!-- Canvas -->
        <div class="flex-1 bg-[var(--bg-base)] relative">
          <div v-if="!hasResults" class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="text-6xl mb-4 opacity-30">🌡️</div>
              <p class="text-gray-400 text-sm">配置参数后运行分析</p>
              <p class="text-gray-300 text-xs mt-1">显示温度云图和热应力分布</p>
            </div>
          </div>
          <div v-else ref="canvasContainer" class="w-full h-full"></div>
        </div>
        
        <!-- Results Summary -->
        <div v-if="hasResults" class="h-40 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex">
          <!-- Thermal Results -->
          <div class="flex-1 p-3 border-r border-[var(--border-subtle)]">
            <h4 class="text-xs font-semibold text-[var(--text-secondary)] mb-2">🌡️ 热分析结果</h4>
            <div class="grid grid-cols-3 gap-2">
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最高温度</div>
                <div class="text-lg font-bold text-red-500">{{ thermalResults.maxTemp.toFixed(1) }}K</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最低温度</div>
                <div class="text-lg font-bold text-blue-500">{{ thermalResults.minTemp.toFixed(1) }}K</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">平均温度</div>
                <div class="text-lg font-bold text-orange-500">{{ thermalResults.avgTemp.toFixed(1) }}K</div>
              </div>
            </div>
          </div>
          <!-- Structural Results -->
          <div class="flex-1 p-3">
            <h4 class="text-xs font-semibold text-[var(--text-secondary)] mb-2">⚙️ 结构分析结果</h4>
            <div class="grid grid-cols-3 gap-2">
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最大位移</div>
                <div class="text-lg font-bold text-blue-500">{{ structuralResults.maxDisplacement.toFixed(4) }}mm</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">最大应力</div>
                <div class="text-lg font-bold text-orange-500">{{ structuralResults.maxStress.toFixed(1) }}MPa</div>
              </div>
              <div class="bg-[var(--bg-base)] rounded p-2 text-center">
                <div class="text-[10px] text-gray-400">等效Mises</div>
                <div class="text-lg font-bold text-red-500">{{ structuralResults.maxVonMises.toFixed(1) }}MPa</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Toolbar -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase tracking-wider">显示</span>
          <div class="flex items-center gap-1">
            <button
              @click="displayMode = 'temperature'"
              :class="['icon-btn w-8 h-8', displayMode === 'temperature' ? 'active' : '']"
              title="温度云图"
            >
              <span class="text-sm">🌡️</span>
            </button>
            <button
              @click="displayMode = 'stress'"
              :class="['icon-btn w-8 h-8', displayMode === 'stress' ? 'active' : '']"
              title="热应力"
            >
              <span class="text-sm">⚡</span>
            </button>
            <button
              @click="displayMode = 'displacement'"
              :class="['icon-btn w-8 h-8', displayMode === 'displacement' ? 'active' : '']"
              title="位移"
            >
              <span class="text-sm">↔</span>
            </button>
          </div>
          <div class="flex-1"></div>
          <div class="flex items-center gap-2">
            <span class="text-[10px] text-[var(--text-muted)]">变形系数</span>
            <input v-model.number="deformationScale" type="range" min="1" max="20" step="1" class="w-20" />
            <span class="text-xs">{{ deformationScale }}x</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Template Dialog -->
    <div v-if="showTemplateDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showTemplateDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-[480px] max-h-[80vh] overflow-y-auto">
        <h3 class="text-lg font-semibold mb-3">热-结构耦合分析模板</h3>
        <div class="space-y-3">
          <div v-for="tpl in templates" :key="tpl.id" 
            class="p-3 bg-[var(--bg-base)] rounded cursor-pointer hover:bg-[var(--bg-hover)] border border-transparent hover:border-[var(--accent)]"
            @click="applyTemplate(tpl)">
            <div class="font-medium mb-1">{{ tpl.name }}</div>
            <div class="text-xs text-gray-400 mb-2">{{ tpl.description }}</div>
            <div class="flex gap-2">
              <span class="text-[10px] px-2 py-0.5 bg-blue-100 text-blue-600 rounded">{{ tpl.category }}</span>
            </div>
          </div>
        </div>
        <button @click="showTemplateDialog = false" class="btn btn-ghost mt-3 w-full">关闭</button>
      </div>
    </div>

    <!-- Embed to Note Dialog -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="showEmbedDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-80">
        <h3 class="text-sm font-semibold mb-3">
          <span class="mr-1">🔗</span> 嵌入到笔记
        </h3>
        <div class="space-y-3">
          <div>
            <label class="text-xs text-[var(--text-muted)] mb-2 block">选择要嵌入的笔记</label>
            <select v-model="embedTargetNoteId" class="input w-full text-sm">
              <option value="">选择笔记...</option>
              <option v-for="note in availableNotes" :key="note.id" :value="note.id">
                {{ note.name }}
              </option>
            </select>
          </div>
          <div v-if="embedTargetNoteId" class="bg-[var(--bg-base)] rounded p-2">
            <p class="text-xs text-[var(--text-muted)] mb-1">嵌入预览</p>
            <p class="text-sm text-[var(--text-primary)]">🌡️ 热-结构耦合分析</p>
            <p class="text-xs text-gray-400">{{ material.name }} | {{ couplingType }}</p>
          </div>
          <div class="flex gap-2 pt-2">
            <button @click="showEmbedDialog = false" class="btn btn-ghost flex-1 text-xs">取消</button>
            <button @click="confirmEmbed" :disabled="!embedTargetNoteId" class="btn btn-primary flex-1 text-xs">确认嵌入</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * ThermalCouplingView - 热-结构耦合分析视图脚本
 * 
 * 本模块负责:
 * - 配置热-结构耦合分析的参数(网格、材料、热边界、热源、结构边界)
 * - 调用后端API生成INP文件并执行CalculiX求解
 * - 3D可视化温度/应力/位移结果
 * - 与笔记模块的双向联动
 */

// ===== 导入部分 =====
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '../stores/project'
import { getThermalCouplingTemplates, generateThermalCouplingInp, type TemplateConfig, type ThermalCouplingJob } from '../api/cae'
import * as THREE from 'three'

// ===== 全局状态 =====
const projectStore = useProjectStore()

// ===== 耦合类型配置 =====
/**
 * 耦合类型: 
 * - sequential: 顺序耦合 (先进行热分析，将温度场作为结构分析的荷载)
 * - fully_coupled: 完全耦合 (热场和结构场同时求解，互为依赖)
 */
const couplingType = ref<'sequential' | 'fully_coupled'>('sequential')

// ===== 耦合参数配置 =====
const couplingParams = ref({
  analysisType: 'sequential' as 'sequential' | 'fully_coupled',
  numIterations: 3,
  convergenceTol: 1e-4,
  couplingType: 'stiffness' as 'stiffness' | 'lumped' | 'full'
})

// ===== 显示控制 =====
const showResults = ref(false)
const results = ref<any>(null)

// ===== 网格配置 =====
/**
 * 网格参数配置
 * 定义几何体的尺寸范围和分格数量，用于生成有限元网格
 */
const meshConfig = ref({
  xMin: 0, xMax: 50, xDiv: 10,
  yMin: 0, yMax: 20, yDiv: 4,
  zMin: 0, zMax: 5, zDiv: 2,
  elementType: 'C3D8'
})

// ===== 材料热-力学属性 =====
/**
 * 材料属性
 * 包含力学参数(密度、弹性模量、泊松比)和热学参数(导热系数、比热、热膨胀系数)
 */
const material = ref({
  name: 'Steel',
  density: 7850,
  youngsModulus: 200000,
  poissonRatio: 0.3,
  thermalConductivity: 50,
  specificHeat: 450,
  expansionCoefficient: 1.2e-5
})

// ===== 热分析配置 =====
/**
 * 热分析参数
 * - analysisType: 稳态热分析(仅计算最终温度场) 或 瞬态热分析(计算温度随时间变化)
 * - initialTemperature: 初始温度
 * - timePeriod/timeIncrement: 瞬态分析的时长和时间步长
 */
const thermalConfig = ref({
  analysisType: 'steady_state' as 'steady_state' | 'transient',
  initialTemperature: 293.15,
  timePeriod: 10,
  timeIncrement: 0.5,
  maxIterations: 100,
  tolerance: 1e-6
})

// ===== 热边界条件 =====
/**
 * 热边界条件列表
 * 支持: 固定温度、热流密度、对流换热、热辐射、绝热
 * 每个BC包含: 类型、数值、作用面
 */
const thermalBCs = ref<Array<{
  bcType: string
  temperature?: number
  heatFlux?: number
  filmCoefficient?: number
  ambientTemperature?: number
  face: string
}>>([])

// ===== 热源 =====
/**
 * 热源列表
 * 支持体热源、面热源、点热源，用于模拟加热元件
 */
const heatSources = ref<Array<{
  sourceType: 'volume' | 'surface' | 'point'
  magnitude: number
}>>([])

// ===== 结构分析配置 =====
/**
 * 结构分析参数
 * - referenceTemperature: 参考温度(用于定义零应力状态)
 * - stressFreeTemperature: 无应力温度(热膨胀计算的基准温度)
 */
const structuralConfig = ref({
  referenceTemperature: 293.15,
  stressFreeTemperature: 298.15,
  stepTime: 1.0
})

// ===== 结构边界条件 =====
/**
 * 结构边界条件列表
 * 定义几何体的固定约束(X/Y/Z方向)和作用面
 */
const structuralBCs = ref<Array<{
  fixX: boolean
  fixY: boolean
  fixZ: boolean
  face: string
}>>([])

// ===== 显示状态 =====
/**
 * 可视化显示模式
 * - temperature: 温度云图
 * - stress: 热应力云图
 * - displacement: 位移变形显示
 */
const displayMode = ref<'temperature' | 'stress' | 'displacement'>('temperature')

/**
 * 变形缩放系数
 * 用于放大显示热膨胀导致的位移变形(1x-20x)
 */
const deformationScale = ref(5)

// ===== 分析结果 =====
/**
 * 分析结果标志和数值
 * hasResults: 是否已有分析结果
 * thermalResults: 热分析结果(最高/最低/平均温度)
 * structuralResults: 结构分析结果(最大位移/最大应力/Mises等效应力)
 */
const hasResults = ref(false)
const thermalResults = ref({ maxTemp: 0, minTemp: 0, avgTemp: 0 })
const structuralResults = ref({ maxDisplacement: 0, maxStress: 0, maxVonMises: 0 })

// ===== UI状态 =====
/**
 * UI交互状态
 * - running: 是否正在运行分析
 * - showTemplateDialog: 模板选择对话框显示
 * - showEmbedDialog: 嵌入笔记对话框显示
 * - embedTargetNoteId: 目标笔记ID
 */
const running = ref(false)
const showTemplateDialog = ref(false)
const showEmbedDialog = ref(false)
const embedTargetNoteId = ref('')

// ===== Three.js场景对象 =====
/**
 * Three.js渲染所需的核心对象
 * - scene: 场景容器
 * - camera: 透视相机
 * - renderer: WebGL渲染器
 */
let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null

// ===== 模板数据 =====
/**
 * 可用的分析模板列表
 * 从后端API获取预配置的模板
 */
const templates = ref<TemplateConfig[]>([])

// ===== 计算属性 =====

/**
 * 估计网格节点数
 * 节点数 = (xDiv+1) * (yDiv+1) * (zDiv+1)
 */
const estimatedNodes = computed(() => 
  (meshConfig.value.xDiv + 1) * (meshConfig.value.yDiv + 1) * (meshConfig.value.zDiv + 1)
)

/**
 * 估计网格单元数
 * 单元数 = xDiv * yDiv * zDiv
 */
const estimatedElements = computed(() => 
  meshConfig.value.xDiv * meshConfig.value.yDiv * meshConfig.value.zDiv
)

/**
 * 检查是否可运行分析
 * 条件: 正分格数 > 0, 弹性模量 > 0, 导热系数 > 0
 */
const canRun = computed(() => 
  meshConfig.value.xDiv > 0 && meshConfig.value.yDiv > 0 &&
  material.value.youngsModulus > 0 && material.value.thermalConductivity > 0
)

/**
 * 可嵌入的笔记列表
 * 过滤当前项目的文件类型为'note'的文件
 */
const availableNotes = ref<any[]>([])

// 加载笔记列表
async function loadNotesList() {
  try {
    if (projectStore.currentProject) {
      const files = await invoke('list_files', { projectId: projectStore.currentProject.id })
      availableNotes.value = (files as any[]).filter(f => f.file_type === 'note')
    }
  } catch (e) {
    console.warn('加载笔记列表失败:', e)
    availableNotes.value = []
  }
}

onMounted(() => {
  loadNotesList()
})

// ===== 重置/运行/确认 =====
function resetAll() {
  meshConfig.value = { xMin: 0, xMax: 10, yMin: 0, yMax: 10, zMin: 0, zMax: 10, xDiv: 20, yDiv: 20, zDiv: 20, elementType: 'C3D8' }
  material.value = { name: 'Steel', density: 7850, youngsModulus: 200000, poissonRatio: 0.3, thermalConductivity: 50, specificHeat: 450, expansionCoefficient: 1.2e-5 }
  thermalBCs.value = []
  structuralBCs.value = []
  heatSources.value = []
  couplingParams.value = { analysisType: 'sequential', numIterations: 3, convergenceTol: 1e-4, couplingType: 'stiffness' }
  showResults.value = false
}

async function runCouplingAnalysis() {
  if (!canRun.value) return
  running.value = true
  try {
    const job = {
      analysis_type: couplingParams.value.analysisType,
      mesh: meshConfig.value,
      material: material.value,
      thermal_bcs: thermalBCs.value,
      structural_bcs: structuralBCs.value,
      heat_sources: heatSources.value,
      params: couplingParams.value
    }
    const result = await invoke('run_coupling_analysis', { job })
    results.value = result
    showResults.value = true
    console.log('热-结构耦合分析完成:', results.value)
  } catch (e) {
    console.error('分析失败:', e)
    alert('分析失败: ' + e)
  } finally {
    running.value = false
  }
}

async function confirmEmbed() {
  if (!embedTargetNoteId.value) return
  try {
    await invoke('embed_simulation_result', {
      noteId: embedTargetNoteId.value,
      resultType: 'thermal_coupling',
      resultData: JSON.stringify(results.value)
    })
    alert('已嵌入笔记')
  } catch (e) {
    console.error('嵌入失败:', e)
    alert('嵌入失败: ' + e)
  }
}

// ===== 材料预设 =====
/**
 * 内置材料预设
 * 包含常用工程材料的热-力学参数
 */
const materialPresets: Record<string, typeof material.value> = {
  Steel: { name: 'Steel', density: 7850, youngsModulus: 200000, poissonRatio: 0.3, thermalConductivity: 50, specificHeat: 450, expansionCoefficient: 1.2e-5 },
  Aluminum: { name: 'Aluminum', density: 2700, youngsModulus: 70000, poissonRatio: 0.33, thermalConductivity: 237, specificHeat: 900, expansionCoefficient: 2.3e-5 },
  Copper: { name: 'Copper', density: 8940, youngsModulus: 110000, poissonRatio: 0.34, thermalConductivity: 401, specificHeat: 385, expansionCoefficient: 1.7e-5 },
  FR4: { name: 'FR4', density: 1900, youngsModulus: 22000, poissonRatio: 0.28, thermalConductivity: 0.3, specificHeat: 1150, expansionCoefficient: 1.5e-5 },
  Solder: { name: 'Solder', density: 7380, youngsModulus: 30000, poissonRatio: 0.35, thermalConductivity: 50, specificHeat: 230, expansionCoefficient: 2.5e-5 },
  Silicon: { name: 'Silicon', density: 2330, youngsModulus: 169000, poissonRatio: 0.28, thermalConductivity: 148, specificHeat: 712, expansionCoefficient: 2.6e-6 }
}

/**
 * 应用材料预设
 * 当用户从下拉菜单选择材料时，自动填充对应参数
 */
function applyMaterialPreset() {
  const preset = materialPresets[material.value.name]
  if (preset) {
    material.value = { ...preset }
  }
}

// ===== 热边界条件管理 =====

/**
 * 添加热边界条件
 * 默认添加一个固定温度边界条件
 */
function addThermalBC() {
  thermalBCs.value.push({
    bcType: 'fixed_temperature',
    temperature: 293.15,
    face: 'all'
  })
}

/**
 * 移除热边界条件
 */
function removeThermalBC(idx: number) {
  thermalBCs.value.splice(idx, 1)
}

// ===== 热源管理 =====

/**
 * 添加热源
 * 默认添加一个体热源
 */
function addHeatSource() {
  heatSources.value.push({
    sourceType: 'volume',
    magnitude: 1000
  })
}

/**
 * 移除热源
 */
function removeHeatSource(idx: number) {
  heatSources.value.splice(idx, 1)
}

// ===== 结构边界条件管理 =====

/**
 * 添加结构边界条件
 * 默认添加一个Z方向固定的约束
 */
function addStructuralBC() {
  structuralBCs.value.push({
    fixX: false,
    fixY: false,
    fixZ: true,
    face: 'z_min'
  })
}

/**
 * 移除结构边界条件
 */
function removeStructuralBC(idx: number) {
  structuralBCs.value.splice(idx, 1)
}

/**
 * 应用分析模板
 * 加载预设模板的配置参数，包括网格、材料、边界条件等
 */
function applyTemplate(tpl: TemplateConfig) {
  // 网格配置
  if (tpl.mesh_config) {
    const mc = tpl.mesh_config
    meshConfig.value = {
      xMin: mc.x_min, xMax: mc.x_max, xDiv: mc.x_div,
      yMin: mc.y_min, yMax: mc.y_max, yDiv: mc.y_div,
      zMin: mc.z_min, zMax: mc.z_max, zDiv: mc.z_div,
      elementType: mc.element_type
    }
  }
  
  // 材料配置 - snake_case后端 -> camelCase前端
  if (tpl.material) {
    material.value = {
      name: tpl.material.name,
      density: tpl.material.density,
      youngsModulus: tpl.material.youngs_modulus,
      poissonRatio: tpl.material.poisson_ratio,
      thermalConductivity: tpl.material.thermal_conductivity,
      specificHeat: tpl.material.specific_heat,
      expansionCoefficient: tpl.material.expansion_coefficient
    }
  }
  
  // 热边界条件 - thermal_config.boundary_conditions -> thermalBCs
  if (tpl.thermal_config?.boundary_conditions) {
    thermalBCs.value = tpl.thermal_config.boundary_conditions.map((bc: any) => ({
      bcType: bc.bc_type,
      temperature: bc.temperature || 0,
      heatFlux: bc.heat_flux || 0,
      filmCoefficient: bc.film_coefficient || 0,
      ambientTemperature: bc.ambient_temperature || 25,
      face: bc.face || 'all'
    }))
  }
  
  // 热源
  if (tpl.thermal_config?.heat_sources) {
    heatSources.value = tpl.thermal_config.heat_sources.map((hs: any) => ({
      sourceType: hs.source_type || 'volume',
      magnitude: hs.magnitude
    }))
  }
  
  // 结构边界条件 - structural_config.boundary_conditions -> structuralBCs
  if (tpl.structural_config?.boundary_conditions) {
    structuralBCs.value = tpl.structural_config.boundary_conditions.map((bc: any) => ({
      fixX: bc.fix_x || false,
      fixY: bc.fix_y || false,
      fixZ: bc.fix_z || false,
      face: bc.face || 'all'
    }))
  }
}

/**
 * 自动保存配置
 * 当参数变化时自动保存到项目存储
 */
// 自动保存（防抖）
let saveTimer: number | null = null
watch([meshConfig, material, thermalBCs, structuralBCs, heatSources], () => {
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = window.setTimeout(async () => {
    try {
      const data = {
        meshConfig: meshConfig.value,
        material: material.value,
        thermalBCs: thermalBCs.value,
        structuralBCs: structuralBCs.value,
        heatSources: heatSources.value,
        couplingParams: couplingParams.value,
        results: results.value
      }
      if (projectStore.currentProject) {
        await invoke('create_file', {
          projectId: projectStore.currentProject.id,
          fileType: 'modeling_data',
          fileName: 'thermal_coupling_config.json',
          content: JSON.stringify(data)
        })
      }
    } catch (e) {
      console.warn('保存失败:', e)
    }
  }, 2000)
}, { deep: true })
</script>
