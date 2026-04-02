<template>
  <div class="advanced-mesh-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">高级网格工具</h2>
        <p class="text-xs text-[var(--text-muted)]">梁/壳/实体混合网格：欧拉-伯努利/铁木辛柯梁，4节点/8节点壳</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- 模板选择 -->
        <select v-model="selectedTemplate" @change="loadTemplate" class="input w-auto text-xs">
          <option value="">选择模板</option>
          <option v-for="t in templates" :key="t.id" :value="t.id">{{ t.name }}</option>
        </select>
        <!-- 重置 -->
        <button @click="resetAll" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x27F3;</span> 重置
        </button>
        <!-- 导出 -->
        <button v-if="meshResult" @click="exportMesh" class="btn btn-ghost text-xs">
          <span class="mr-1">&#x2B07;</span> 导出
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col overflow-y-auto">
        <!-- Step 1: 梁单元设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">1. 梁单元设置</h3>
          </div>

          <!-- 截面类型 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">截面类型</label>
            <select v-model="beamSection.type" @change="onBeamTypeChange" class="input w-full text-xs">
              <option value="circular">圆形</option>
              <option value="rectangular">矩形</option>
              <option value="i_section">工字钢</option>
              <option value="t_section">T型</option>
              <option value="l_section">L型</option>
              <option value="hollow_circular">空心圆</option>
              <option value="hollow_rectangular">空心矩形</option>
            </select>
          </div>

          <!-- 截面尺寸参数 (动态) -->
          <div class="mb-2 space-y-1.5">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">截面尺寸</label>
            <div v-for="(dim, key) in beamDimensionFields" :key="key">
              <div class="flex items-center gap-2">
                <span class="text-[10px] text-[var(--text-muted)] w-20 shrink-0">{{ dim.label }}</span>
                <input
                  v-model.number="beamSection.dimensions[key]"
                  type="number"
                  step="0.1"
                  min="0"
                  class="input w-full text-xs"
                  :placeholder="dim.unit"
                />
                <span class="text-[10px] text-[var(--text-muted)] w-6 shrink-0">{{ dim.unit }}</span>
              </div>
            </div>
          </div>

          <!-- 梁单元类型 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">单元类型</label>
            <select v-model="config.beam_element_type" class="input w-full text-xs">
              <option value="B31">B31 (2节点线性梁)</option>
              <option value="B32">B32 (3节点二次梁)</option>
              <option value="PIPE31">PIPE31 (2节点管单元)</option>
            </select>
          </div>
        </div>

        <!-- Step 2: 壳单元设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">2. 壳单元设置</h3>
          </div>

          <!-- 厚度 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">厚度 (mm)</label>
            <input v-model.number="config.shell_properties.thickness" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
          </div>

          <!-- 壳单元类型 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">单元类型</label>
            <select v-model="config.shell_properties.element_type" class="input w-full text-xs">
              <option value="S4R">S4R (4节点减缩积分)</option>
              <option value="S8R">S8R (8节点减缩积分)</option>
              <option value="S3">S3 (3节点三角形)</option>
              <option value="STRI65">STRI65 (6节点三角形)</option>
            </select>
          </div>

          <!-- 积分点数 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">积分点数</label>
            <select v-model.number="config.shell_properties.integration_points" class="input w-full text-xs">
              <option :value="1">1 (Simpson)</option>
              <option :value="2">2 (Simpson)</option>
              <option :value="3">3 (Simpson)</option>
              <option :value="5">5 (Gauss)</option>
            </select>
          </div>

          <!-- 偏置类型 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">偏置</label>
            <select v-model="config.shell_properties.offset_type" class="input w-full text-xs">
              <option value="top">顶面 (TOP)</option>
              <option value="middle">中面 (MIDDLE)</option>
              <option value="bottom">底面 (BOTTOM)</option>
            </select>
          </div>
        </div>

        <!-- Step 3: 实体单元设置 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">3. 实体单元设置</h3>
          </div>

          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">单元类型</label>
            <select v-model="config.solid_element_type" class="input w-full text-xs">
              <option value="C3D8R">C3D8R (8节点六面体减缩积分)</option>
              <option value="C3D4">C3D4 (4节点四面体)</option>
              <option value="C3D10">C3D10 (10节点二次四面体)</option>
            </select>
          </div>
        </div>

        <!-- Step 4: 网格控制 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">4. 网格控制</h3>
          </div>

          <!-- 全局网格尺寸 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">全局网格尺寸 (mm)</label>
            <input v-model.number="config.mesh_size" type="number" step="0.5" min="0.1" class="input w-full text-xs" />
          </div>

          <!-- 过渡单元开关 -->
          <div class="mb-2 flex items-center justify-between">
            <label class="text-xs text-[var(--text-secondary)]">过渡单元</label>
            <button
              @click="config.transition_elements = !config.transition_elements"
              :class="['w-10 h-5 rounded-full transition-colors relative', config.transition_elements ? 'bg-blue-600' : 'bg-gray-600']"
            >
              <span
                :class="['absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform', config.transition_elements ? 'translate-x-5' : 'translate-x-0.5']"
              ></span>
            </button>
          </div>

          <!-- 质量目标 -->
          <div class="mb-2">
            <label class="text-xs text-[var(--text-secondary)] block mb-1">质量目标</label>
            <div class="flex gap-2">
              <button
                v-for="q in qualityTargets"
                :key="q.value"
                @click="config.mesh_quality_target = q.value"
                :class="['px-3 py-1.5 rounded text-xs transition', config.mesh_quality_target === q.value ? 'bg-[var(--primary)] text-white' : 'bg-[var(--bg-base)] text-[var(--text-secondary)]']"
              >
                {{ q.label }}
              </button>
            </div>
          </div>
        </div>

        <!-- Step 5: 截面属性计算 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <div class="flex items-center gap-2 mb-3">
            <span class="step-dot active"></span>
            <h3 class="text-sm font-semibold text-[var(--text-primary)]">5. 截面属性计算</h3>
          </div>

          <button @click="calcSectionProps" class="btn btn-outline text-xs w-full mb-3">
            计算截面属性
          </button>

          <!-- 计算结果 -->
          <div v-if="calculatedSection" class="space-y-1.5">
            <div class="flex justify-between text-xs">
              <span class="text-[var(--text-muted)]">面积 A</span>
              <span class="text-[var(--text-primary)] font-mono">{{ calculatedSection.area.toFixed(2) }} mm&sup2;</span>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-[var(--text-muted)]">惯性矩 Iy</span>
              <span class="text-[var(--text-primary)] font-mono">{{ calculatedSection.Iy.toFixed(2) }} mm&sup4;</span>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-[var(--text-muted)]">惯性矩 Iz</span>
              <span class="text-[var(--text-primary)] font-mono">{{ calculatedSection.Iz.toFixed(2) }} mm&sup4;</span>
            </div>
            <div class="flex justify-between text-xs">
              <span class="text-[var(--text-muted)]">扭转常数 J</span>
              <span class="text-[var(--text-primary)] font-mono">{{ calculatedSection.J.toFixed(2) }} mm&sup4;</span>
            </div>
          </div>
        </div>

        <!-- 快捷模板按钮 -->
        <div class="px-4 py-3 border-b border-[var(--border-subtle)]">
          <label class="text-xs text-[var(--text-secondary)] block mb-2">快捷模板</label>
          <div class="flex gap-2 flex-wrap">
            <button @click="applyQuickTemplate('frame')" class="btn btn-outline text-xs px-2 py-1">框架结构</button>
            <button @click="applyQuickTemplate('vessel')" class="btn btn-outline text-xs px-2 py-1">薄壁容器</button>
            <button @click="applyQuickTemplate('stiffened')" class="btn btn-outline text-xs px-2 py-1">加筋板</button>
            <button @click="applyQuickTemplate('bridge')" class="btn btn-outline text-xs px-2 py-1">桥梁结构</button>
          </div>
        </div>

        <!-- 生成网格按钮 -->
        <div class="px-4 py-3 mt-auto border-t border-[var(--border-subtle)]">
          <button
            @click="generateMesh"
            :disabled="generating"
            :class="['btn w-full text-sm', generating ? 'btn-loading' : 'btn-primary']"
          >
            <span v-if="generating" class="mr-2 animate-spin">&#x27F3;</span>
            {{ generating ? '生成中...' : '&#x25B6; 生成混合网格' }}
          </button>
        </div>
      </div>

      <!-- Right: Visualization & Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Results Tabs -->
        <div v-if="meshResult" class="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
          <button
            v-for="tab in resultTabs"
            :key="tab.value"
            @click="resultViewTab = tab.value"
            :class="['px-3 py-1 text-xs rounded transition',
              resultViewTab === tab.value
                ? 'bg-[var(--primary)] text-white'
                : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)]']"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- Visualization Area -->
        <div class="flex-1 flex">
          <div class="flex-1 relative">
            <!-- 3D 混合网格预览 Canvas -->
            <div v-if="meshResult && resultViewTab === 'preview'" class="w-full h-full">
              <canvas ref="meshCanvas" class="w-full h-full"></canvas>
            </div>
            <!-- 网格质量报告 -->
            <div v-if="meshResult && resultViewTab === 'quality'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-3">网格质量报告</h4>
              <div class="space-y-3">
                <!-- 长宽比 -->
                <div class="quality-card">
                  <div class="flex justify-between items-center mb-2">
                    <span class="text-xs text-[var(--text-secondary)]">长宽比 (Aspect Ratio)</span>
                    <span class="text-xs font-mono text-[var(--text-primary)]">{{ meshResult.quality_metrics.aspect_ratio.min.toFixed(2) }} ~ {{ meshResult.quality_metrics.aspect_ratio.max.toFixed(2) }}</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full transition-all" :style="{ width: qualityBarWidth(meshResult.quality_metrics.aspect_ratio.avg, 10) + '%', background: qualityBarColor(meshResult.quality_metrics.aspect_ratio.avg, 10) }"></div>
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">平均: {{ meshResult.quality_metrics.aspect_ratio.avg.toFixed(2) }}</div>
                </div>
                <!-- 雅可比 -->
                <div class="quality-card">
                  <div class="flex justify-between items-center mb-2">
                    <span class="text-xs text-[var(--text-secondary)]">雅可比 (Jacobian)</span>
                    <span class="text-xs font-mono text-[var(--text-primary)]">{{ meshResult.quality_metrics.jacobian.min.toFixed(3) }} ~ {{ meshResult.quality_metrics.jacobian.max.toFixed(3) }}</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full transition-all" :style="{ width: qualityBarWidth(meshResult.quality_metrics.jacobian.avg, 1) + '%', background: qualityBarColor(meshResult.quality_metrics.jacobian.avg, 1) }"></div>
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">平均: {{ meshResult.quality_metrics.jacobian.avg.toFixed(3) }}</div>
                </div>
                <!-- 翘曲 -->
                <div class="quality-card">
                  <div class="flex justify-between items-center mb-2">
                    <span class="text-xs text-[var(--text-secondary)]">翘曲 (Warpage)</span>
                    <span class="text-xs font-mono text-[var(--text-primary)]">{{ meshResult.quality_metrics.warpage.min.toFixed(2) }} ~ {{ meshResult.quality_metrics.warpage.max.toFixed(2) }}</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full transition-all" :style="{ width: qualityBarWidth(meshResult.quality_metrics.warpage.avg, 45) + '%', background: qualityBarColor(meshResult.quality_metrics.warpage.avg, 45) }"></div>
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">平均: {{ meshResult.quality_metrics.warpage.avg.toFixed(2) }}</div>
                </div>
                <!-- 偏斜 -->
                <div class="quality-card">
                  <div class="flex justify-between items-center mb-2">
                    <span class="text-xs text-[var(--text-secondary)]">偏斜 (Skewness)</span>
                    <span class="text-xs font-mono text-[var(--text-primary)]">{{ meshResult.quality_metrics.skewness.min.toFixed(2) }} ~ {{ meshResult.quality_metrics.skewness.max.toFixed(2) }}</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full transition-all" :style="{ width: qualityBarWidth(meshResult.quality_metrics.skewness.avg, 90) + '%', background: qualityBarColor(meshResult.quality_metrics.skewness.avg, 90) }"></div>
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">平均: {{ meshResult.quality_metrics.skewness.avg.toFixed(2) }}</div>
                </div>
                <!-- 锥度 -->
                <div class="quality-card">
                  <div class="flex justify-between items-center mb-2">
                    <span class="text-xs text-[var(--text-secondary)]">锥度 (Taper)</span>
                    <span class="text-xs font-mono text-[var(--text-primary)]">{{ meshResult.quality_metrics.taper.min.toFixed(3) }} ~ {{ meshResult.quality_metrics.taper.max.toFixed(3) }}</span>
                  </div>
                  <div class="w-full h-2 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                    <div class="h-full rounded-full transition-all" :style="{ width: qualityBarWidth(meshResult.quality_metrics.taper.avg, 0.5) + '%', background: qualityBarColor(meshResult.quality_metrics.taper.avg, 0.5) }"></div>
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">平均: {{ meshResult.quality_metrics.taper.avg.toFixed(3) }}</div>
                </div>
              </div>
            </div>
            <!-- 网格统计 -->
            <div v-if="meshResult && resultViewTab === 'stats'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-3">网格统计</h4>
              <div class="grid grid-cols-2 gap-4">
                <!-- 梁单元 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">梁单元</div>
                  <div class="text-xl font-bold text-blue-400">
                    {{ meshResult.beam_elements.count }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    节点: {{ meshResult.beam_elements.nodes }}
                  </div>
                </div>
                <!-- 壳单元 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">壳单元</div>
                  <div class="text-xl font-bold text-green-400">
                    {{ meshResult.shell_elements.count }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    节点: {{ meshResult.shell_elements.nodes }}
                  </div>
                </div>
                <!-- 实体单元 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">实体单元</div>
                  <div class="text-xl font-bold text-amber-400">
                    {{ meshResult.solid_elements.count }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    节点: {{ meshResult.solid_elements.nodes }}
                  </div>
                </div>
                <!-- 过渡单元 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">过渡单元</div>
                  <div class="text-xl font-bold text-purple-400">
                    {{ meshResult.transition_elements.count }}
                  </div>
                  <div class="text-[10px] text-[var(--text-muted)] mt-1">
                    节点: {{ meshResult.transition_elements.nodes }}
                  </div>
                </div>
                <!-- 总节点数 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">总节点数</div>
                  <div class="text-xl font-bold text-[var(--text-primary)]">
                    {{ meshResult.total_nodes }}
                  </div>
                </div>
                <!-- 总单元数 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">总单元数</div>
                  <div class="text-xl font-bold text-[var(--text-primary)]">
                    {{ meshResult.total_elements }}
                  </div>
                </div>
                <!-- 失败单元 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">失败单元</div>
                  <div class="text-xl font-bold" :style="{ color: meshResult.quality_metrics.failed_elements > 0 ? 'var(--accent-red)' : 'var(--accent-green)' }">
                    {{ meshResult.quality_metrics.failed_elements }}
                  </div>
                </div>
                <!-- 警告单元 -->
                <div class="stat-card">
                  <div class="text-[10px] text-[var(--text-muted)] uppercase">警告单元</div>
                  <div class="text-xl font-bold" :style="{ color: meshResult.quality_metrics.warning_elements > 0 ? 'var(--accent-amber)' : 'var(--accent-green)' }">
                    {{ meshResult.quality_metrics.warning_elements }}
                  </div>
                </div>
              </div>
            </div>
            <!-- 质量评分仪表盘 -->
            <div v-if="meshResult && resultViewTab === 'dashboard'" class="w-full h-full overflow-y-auto p-4">
              <h4 class="text-sm font-semibold text-[var(--text-primary)] mb-3">质量评分仪表盘</h4>
              <div class="flex flex-col items-center gap-6">
                <!-- 综合评分仪表 -->
                <div class="relative">
                  <canvas ref="gaugeCanvas" width="200" height="200"></canvas>
                  <div class="absolute inset-0 flex flex-col items-center justify-center">
                    <span class="text-3xl font-bold" :style="{ color: gaugeColor }">{{ (meshResult.quality_metrics.element_quality_score * 100).toFixed(1) }}</span>
                    <span class="text-xs text-[var(--text-muted)]">综合评分</span>
                  </div>
                </div>
                <!-- 评分等级 -->
                <div class="w-full max-w-md">
                  <div class="flex justify-between text-xs mb-1">
                    <span class="text-[var(--text-muted)]">质量等级</span>
                    <span class="font-semibold" :style="{ color: gaugeColor }">{{ qualityGrade }}</span>
                  </div>
                  <div class="w-full h-3 bg-[var(--bg-elevated)] rounded-full overflow-hidden">
                    <div
                      class="h-full rounded-full transition-all"
                      :style="{ width: (meshResult.quality_metrics.element_quality_score * 100) + '%', background: gaugeGradient }"
                    ></div>
                  </div>
                  <div class="flex justify-between text-[10px] text-[var(--text-muted)] mt-1">
                    <span>差 (0)</span>
                    <span>中 (0.5)</span>
                    <span>优 (1.0)</span>
                  </div>
                </div>
                <!-- 各项指标评分 -->
                <div class="w-full max-w-md grid grid-cols-2 gap-3">
                  <div class="quality-card text-center">
                    <div class="text-[10px] text-[var(--text-muted)] mb-1">长宽比</div>
                    <div class="text-lg font-bold" :style="{ color: metricScoreColor(1 - meshResult.quality_metrics.aspect_ratio.avg / 10) }">
                      {{ (Math.max(0, 1 - meshResult.quality_metrics.aspect_ratio.avg / 10) * 100).toFixed(0) }}
                    </div>
                  </div>
                  <div class="quality-card text-center">
                    <div class="text-[10px] text-[var(--text-muted)] mb-1">雅可比</div>
                    <div class="text-lg font-bold" :style="{ color: metricScoreColor(meshResult.quality_metrics.jacobian.avg) }">
                      {{ (meshResult.quality_metrics.jacobian.avg * 100).toFixed(0) }}
                    </div>
                  </div>
                  <div class="quality-card text-center">
                    <div class="text-[10px] text-[var(--text-muted)] mb-1">翘曲</div>
                    <div class="text-lg font-bold" :style="{ color: metricScoreColor(1 - meshResult.quality_metrics.warpage.avg / 45) }">
                      {{ (Math.max(0, 1 - meshResult.quality_metrics.warpage.avg / 45) * 100).toFixed(0) }}
                    </div>
                  </div>
                  <div class="quality-card text-center">
                    <div class="text-[10px] text-[var(--text-muted)] mb-1">偏斜</div>
                    <div class="text-lg font-bold" :style="{ color: metricScoreColor(1 - meshResult.quality_metrics.skewness.avg / 90) }">
                      {{ (Math.max(0, 1 - meshResult.quality_metrics.skewness.avg / 90) * 100).toFixed(0) }}
                    </div>
                  </div>
                  <div class="quality-card text-center">
                    <div class="text-[10px] text-[var(--text-muted)] mb-1">锥度</div>
                    <div class="text-lg font-bold" :style="{ color: metricScoreColor(1 - meshResult.quality_metrics.taper.avg / 0.5) }">
                      {{ (Math.max(0, 1 - meshResult.quality_metrics.taper.avg / 0.5) * 100).toFixed(0) }}
                    </div>
                  </div>
                  <div class="quality-card text-center">
                    <div class="text-[10px] text-[var(--text-muted)] mb-1">失败/警告</div>
                    <div class="text-lg font-bold" :style="{ color: meshResult.quality_metrics.failed_elements === 0 ? 'var(--accent-green)' : 'var(--accent-red)' }">
                      {{ meshResult.quality_metrics.failed_elements }}/{{ meshResult.quality_metrics.warning_elements }}
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <!-- 空状态 -->
            <div v-if="!meshResult" class="absolute inset-0 flex items-center justify-center">
              <div class="text-center text-[var(--text-muted)]">
                <div class="text-4xl mb-2">&#x1F4D0;</div>
                <p class="text-sm">配置参数后生成混合网格</p>
                <p class="text-xs mt-1">支持梁/壳/实体单元混合建模</p>
              </div>
            </div>
          </div>

          <!-- Color Legend (预览模式) -->
          <div v-if="meshResult && resultViewTab === 'preview'" class="w-16 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] p-2">
            <div class="text-[10px] text-[var(--text-muted)] mb-2">图例</div>
            <div class="space-y-2">
              <div class="flex items-center gap-1">
                <div class="w-3 h-3 rounded-sm" style="background: #3B82F6;"></div>
                <span class="text-[10px] text-[var(--text-secondary)]">梁</span>
              </div>
              <div class="flex items-center gap-1">
                <div class="w-3 h-3 rounded-sm" style="background: #22C55E;"></div>
                <span class="text-[10px] text-[var(--text-secondary)]">壳</span>
              </div>
              <div class="flex items-center gap-1">
                <div class="w-3 h-3 rounded-sm" style="background: #F59E0B;"></div>
                <span class="text-[10px] text-[var(--text-secondary)]">实体</span>
              </div>
              <div class="flex items-center gap-1">
                <div class="w-3 h-3 rounded-sm" style="background: #A855F7;"></div>
                <span class="text-[10px] text-[var(--text-secondary)]">过渡</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick, watch } from 'vue'
import type {
  BeamSection,
  BeamSectionType,
  ShellProperties,
  SolidElementType,
  BeamElementType,
  MeshQualityTarget,
  MixedMeshConfig,
  MixedMeshResult,
  AdvancedMeshTemplate,
} from '@/api/advancedMesh'

// ============ 常量 ============

/** 截面尺寸字段定义 */
interface DimensionField {
  label: string
  unit: string
  default: number
}

const beamDimensionMap: Record<BeamSectionType, Record<string, DimensionField>> = {
  circular: {
    radius: { label: '半径 R', unit: 'mm', default: 10 },
  },
  rectangular: {
    width: { label: '宽度 B', unit: 'mm', default: 20 },
    height: { label: '高度 H', unit: 'mm', default: 30 },
  },
  i_section: {
    H: { label: '总高 H', unit: 'mm', default: 200 },
    B: { label: '翼缘宽 B', unit: 'mm', default: 100 },
    tw: { label: '腹板厚 tw', unit: 'mm', default: 6 },
    tf: { label: '翼缘厚 tf', unit: 'mm', default: 10 },
  },
  t_section: {
    H: { label: '总高 H', unit: 'mm', default: 150 },
    B: { label: '翼缘宽 B', unit: 'mm', default: 80 },
    tw: { label: '腹板厚 tw', unit: 'mm', default: 6 },
    tf: { label: '翼缘厚 tf', unit: 'mm', default: 8 },
  },
  l_section: {
    H: { label: '总高 H', unit: 'mm', default: 100 },
    B: { label: '翼缘宽 B', unit: 'mm', default: 60 },
    t: { label: '厚度 t', unit: 'mm', default: 6 },
  },
  hollow_circular: {
    R: { label: '外径 R', unit: 'mm', default: 20 },
    r: { label: '内径 r', unit: 'mm', default: 15 },
  },
  hollow_rectangular: {
    B: { label: '外宽 B', unit: 'mm', default: 40 },
    H: { label: '外高 H', unit: 'mm', default: 60 },
    b: { label: '内宽 b', unit: 'mm', default: 35 },
    h: { label: '内高 h', unit: 'mm', default: 55 },
  },
}

const qualityTargets = [
  { value: 'high' as MeshQualityTarget, label: '高' },
  { value: 'medium' as MeshQualityTarget, label: '中' },
  { value: 'low' as MeshQualityTarget, label: '低' },
]

const resultTabs = [
  { value: 'preview', label: '网格预览' },
  { value: 'quality', label: '质量报告' },
  { value: 'stats', label: '网格统计' },
  { value: 'dashboard', label: '质量仪表盘' },
]

// ============ 模板数据 ============
const templateData: AdvancedMeshTemplate[] = [
  {
    id: 'frame',
    name: '框架结构',
    name_en: 'Frame Structure',
    category: 'building',
    description: '多层建筑钢框架，H型钢梁柱，楼板壳单元',
    typical_application: '多层钢结构建筑',
    default_sections: [
      {
        type: 'i_section',
        name: 'HN200x100',
        dimensions: { H: 200, B: 100, tw: 5.5, tf: 8 },
        area: 2667,
        Iy: 22100000,
        Iz: 1340000,
        J: 632000,
      },
    ],
  },
  {
    id: 'vessel',
    name: '薄壁容器',
    name_en: 'Thin-walled Vessel',
    category: 'pressure',
    description: '压力容器薄壁结构，壳单元为主，接管处实体单元',
    typical_application: '压力容器、储罐',
    default_sections: [
      {
        type: 'hollow_circular',
        name: 'Pipe DN200',
        dimensions: { R: 110, r: 100 },
        area: 6409,
        Iy: 14600000,
        Iz: 14600000,
        J: 29200000,
      },
    ],
  },
  {
    id: 'stiffened',
    name: '加筋板',
    name_en: 'Stiffened Plate',
    category: 'marine',
    description: '船体加筋板结构，T型加筋骨，板壳单元',
    typical_application: '船体结构、甲板',
    default_sections: [
      {
        type: 't_section',
        name: 'T100x60',
        dimensions: { H: 100, B: 60, tw: 5, tf: 6 },
        area: 860,
        Iy: 1130000,
        Iz: 108000,
        J: 41000,
      },
    ],
  },
  {
    id: 'bridge',
    name: '桥梁结构',
    name_en: 'Bridge Structure',
    category: 'infra',
    description: '桥梁主梁与桥面板，工字钢主梁+壳单元桥面',
    typical_application: '公路桥梁、铁路桥梁',
    default_sections: [
      {
        type: 'i_section',
        name: 'HN400x200',
        dimensions: { H: 400, B: 200, tw: 8, tf: 13 },
        area: 8412,
        Iy: 237000000,
        Iz: 17400000,
        J: 1120000,
      },
    ],
  },
]

// ============ 状态 ============
const generating = ref(false)
const selectedTemplate = ref('')
const templates = ref<AdvancedMeshTemplate[]>([])
const resultViewTab = ref('preview')
const meshResult = ref<MixedMeshResult | null>(null)
const calculatedSection = ref<BeamSection | null>(null)

// Canvas refs
const meshCanvas = ref<HTMLCanvasElement | null>(null)
const gaugeCanvas = ref<HTMLCanvasElement | null>(null)

// ============ 梁截面配置 ============
const beamSection = reactive<BeamSection>({
  type: 'i_section',
  name: '自定义截面',
  dimensions: { H: 200, B: 100, tw: 6, tf: 10 },
  area: 0,
  Iy: 0,
  Iz: 0,
  J: 0,
})

// ============ 混合网格配置 ============
const config = reactive<MixedMeshConfig>({
  project_id: 'default',
  beam_sections: [beamSection],
  shell_properties: {
    thickness: 5,
    element_type: 'S4R',
    integration_points: 5,
    offset_type: 'middle',
  },
  solid_element_type: 'C3D8R',
  beam_element_type: 'B31',
  mesh_size: 10,
  transition_elements: true,
  mesh_quality_target: 'medium',
})

// ============ 计算属性 ============

/** 当前截面类型的尺寸字段 */
const beamDimensionFields = computed(() => {
  return beamDimensionMap[beamSection.type] || {}
})

/** 质量评分颜色 */
const gaugeColor = computed(() => {
  if (!meshResult.value) return 'var(--text-primary)'
  const score = meshResult.value.quality_metrics.element_quality_score
  if (score >= 0.8) return 'var(--accent-green)'
  if (score >= 0.6) return 'var(--accent-amber)'
  return 'var(--accent-red)'
})

const gaugeGradient = computed(() => {
  if (!meshResult.value) return 'var(--text-muted)'
  const score = meshResult.value.quality_metrics.element_quality_score
  if (score >= 0.8) return 'linear-gradient(to right, #22C55E, #16A34A)'
  if (score >= 0.6) return 'linear-gradient(to right, #F59E0B, #D97706)'
  return 'linear-gradient(to right, #EF4444, #DC2626)'
})

/** 质量等级 */
const qualityGrade = computed(() => {
  if (!meshResult.value) return '-'
  const score = meshResult.value.quality_metrics.element_quality_score
  if (score >= 0.9) return '优秀 (A)'
  if (score >= 0.8) return '良好 (B)'
  if (score >= 0.6) return '合格 (C)'
  if (score >= 0.4) return '较差 (D)'
  return '不合格 (F)'
})

// ============ 方法 ============

/** 截面类型变更时重置尺寸 */
function onBeamTypeChange() {
  const fields = beamDimensionMap[beamSection.type]
  if (!fields) return
  const newDims: Record<string, number> = {}
  for (const [key, field] of Object.entries(fields)) {
    newDims[key] = field.default
  }
  beamSection.dimensions = newDims
  calculatedSection.value = null
}

/** 计算截面属性 (模拟) */
function calcSectionProps() {
  const d = beamSection.dimensions
  let area = 0, Iy = 0, Iz = 0, J = 0

  switch (beamSection.type) {
    case 'circular': {
      const r = d.radius || 10
      area = Math.PI * r * r
      Iy = Math.PI * Math.pow(r, 4) / 4
      Iz = Iy
      J = Math.PI * Math.pow(r, 4) / 2
      break
    }
    case 'rectangular': {
      const b = d.width || 20
      const h = d.height || 30
      area = b * h
      Iy = b * Math.pow(h, 3) / 12
      Iz = h * Math.pow(b, 3) / 12
      J = b * h * Math.min(b, h) * Math.pow(Math.max(b, h), 2) / (3 * (Math.pow(b, 2) + Math.pow(h, 2)))
      break
    }
    case 'i_section': {
      const H = d.H || 200
      const B = d.B || 100
      const tw = d.tw || 6
      const tf = d.tf || 10
      area = 2 * B * tf + (H - 2 * tf) * tw
      Iy = (B * Math.pow(H, 3) - (B - tw) * Math.pow(H - 2 * tf, 3)) / 12
      Iz = (2 * tf * Math.pow(B, 3) + (H - 2 * tf) * Math.pow(tw, 3)) / 12
      J = (2 * B * Math.pow(tf, 3) + (H - 2 * tf) * Math.pow(tw, 3)) / 3
      break
    }
    case 't_section': {
      const H = d.H || 150
      const B = d.B || 80
      const tw = d.tw || 6
      const tf = d.tf || 8
      area = B * tf + (H - tf) * tw
      // 近似计算 (形心偏移)
      const yBar = (B * tf * (H - tf / 2) + tw * (H - tf) * (H - tf) / 2) / area
      Iy = B * Math.pow(tf, 3) / 12 + B * tf * Math.pow(H - tf / 2 - yBar, 2)
        + tw * Math.pow(H - tf, 3) / 12 + tw * (H - tf) * Math.pow((H - tf) / 2 - yBar, 2)
      Iz = tf * Math.pow(B, 3) / 12 + (H - tf) * Math.pow(tw, 3) / 12
      J = (B * Math.pow(tf, 3) + (H - tf) * Math.pow(tw, 3)) / 3
      break
    }
    case 'l_section': {
      const H = d.H || 100
      const B = d.B || 60
      const t = d.t || 6
      area = (H + B - t) * t
      const yBar = (B * t * t / 2 + (H - t) * t * (t + (H - t) / 2)) / area
      Iy = B * Math.pow(t, 3) / 12 + B * t * Math.pow(yBar - t / 2, 2)
        + t * Math.pow(H - t, 3) / 12 + t * (H - t) * Math.pow(t + (H - t) / 2 - yBar, 2)
      Iz = t * Math.pow(B, 3) / 12 + B * t * Math.pow(B / 2 - B / 2, 2)
        + (H - t) * Math.pow(t, 3) / 12 + (H - t) * t * Math.pow(t / 2 - B / 2, 2)
      J = ((H + B) * Math.pow(t, 3)) / 3
      break
    }
    case 'hollow_circular': {
      const R = d.R || 20
      const r = d.r || 15
      area = Math.PI * (R * R - r * r)
      Iy = Math.PI * (Math.pow(R, 4) - Math.pow(r, 4)) / 4
      Iz = Iy
      J = Math.PI * (Math.pow(R, 4) - Math.pow(r, 4)) / 2
      break
    }
    case 'hollow_rectangular': {
      const B = d.B || 40
      const H = d.H || 60
      const b = d.b || 35
      const h = d.h || 55
      area = B * H - b * h
      Iy = (B * Math.pow(H, 3) - b * Math.pow(h, 3)) / 12
      Iz = (H * Math.pow(B, 3) - h * Math.pow(b, 3)) / 12
      J = 2 * Math.pow(B - b, 2) * Math.pow(H - h, 2) / (B * H - b * h + Math.sqrt(B * H) - Math.sqrt(b * h))
      break
    }
  }

  calculatedSection.value = {
    ...beamSection,
    area: parseFloat(area.toFixed(2)),
    Iy: parseFloat(Iy.toFixed(2)),
    Iz: parseFloat(Iz.toFixed(2)),
    J: parseFloat(J.toFixed(2)),
  }
}

/** 质量条宽度 (反向指标: avg越低越好) */
function qualityBarWidth(avg: number, maxThreshold: number): number {
  const ratio = Math.min(avg / maxThreshold, 1)
  return Math.max(0, (1 - ratio) * 100)
}

/** 质量条颜色 */
function qualityBarColor(avg: number, maxThreshold: number): string {
  const ratio = avg / maxThreshold
  if (ratio < 0.5) return '#22C55E'
  if (ratio < 0.8) return '#F59E0B'
  return '#EF4444'
}

/** 单项指标评分颜色 */
function metricScoreColor(score: number): string {
  const s = Math.max(0, Math.min(1, score))
  if (s >= 0.8) return 'var(--accent-green)'
  if (s >= 0.6) return 'var(--accent-amber)'
  return 'var(--accent-red)'
}

/** 加载模板 */
function loadTemplate() {
  if (!selectedTemplate.value) return
  const tmpl = templateData.find(t => t.id === selectedTemplate.value)
  if (!tmpl) return

  if (tmpl.default_sections && tmpl.default_sections.length > 0) {
    const sec = tmpl.default_sections[0]
    beamSection.type = sec.type
    beamSection.name = sec.name
    beamSection.dimensions = { ...sec.dimensions }
    beamSection.area = sec.area
    beamSection.Iy = sec.Iy
    beamSection.Iz = sec.Iz
    beamSection.J = sec.J
    calculatedSection.value = { ...sec }
  }

  switch (tmpl.id) {
    case 'frame':
      config.beam_element_type = 'B31'
      config.shell_properties.element_type = 'S4R'
      config.shell_properties.thickness = 8
      config.solid_element_type = 'C3D8R'
      config.mesh_size = 15
      config.transition_elements = true
      config.mesh_quality_target = 'high'
      break
    case 'vessel':
      config.beam_element_type = 'PIPE31'
      config.shell_properties.element_type = 'S8R'
      config.shell_properties.thickness = 12
      config.solid_element_type = 'C3D10'
      config.mesh_size = 8
      config.transition_elements = true
      config.mesh_quality_target = 'high'
      break
    case 'stiffened':
      config.beam_element_type = 'B31'
      config.shell_properties.element_type = 'S4R'
      config.shell_properties.thickness = 4
      config.solid_element_type = 'C3D8R'
      config.mesh_size = 10
      config.transition_elements = true
      config.mesh_quality_target = 'medium'
      break
    case 'bridge':
      config.beam_element_type = 'B32'
      config.shell_properties.element_type = 'S4R'
      config.shell_properties.thickness = 10
      config.solid_element_type = 'C3D8R'
      config.mesh_size = 20
      config.transition_elements = true
      config.mesh_quality_target = 'medium'
      break
  }
}

/** 快捷模板 */
function applyQuickTemplate(type: string) {
  selectedTemplate.value = type
  loadTemplate()
}

/** 重置 */
function resetAll() {
  meshResult.value = null
  selectedTemplate.value = ''
  calculatedSection.value = null
  beamSection.type = 'i_section'
  beamSection.name = '自定义截面'
  beamSection.dimensions = { H: 200, B: 100, tw: 6, tf: 10 }
  beamSection.area = 0
  beamSection.Iy = 0
  beamSection.Iz = 0
  beamSection.J = 0
  config.beam_element_type = 'B31'
  config.shell_properties = {
    thickness: 5,
    element_type: 'S4R',
    integration_points: 5,
    offset_type: 'middle',
  }
  config.solid_element_type = 'C3D8R'
  config.mesh_size = 10
  config.transition_elements = true
  config.mesh_quality_target = 'medium'
  resultViewTab.value = 'preview'
}

/** 生成模拟网格数据 */
function generateMockMeshResult(): MixedMeshResult {
  const qualityMultiplier = config.mesh_quality_target === 'high' ? 0.7 : config.mesh_quality_target === 'medium' ? 1.0 : 1.5
  const meshSizeFactor = 10 / config.mesh_size

  const beamCount = Math.round(120 * meshSizeFactor * (0.8 + Math.random() * 0.4))
  const shellCount = Math.round(350 * meshSizeFactor * (0.8 + Math.random() * 0.4))
  const solidCount = Math.round(800 * meshSizeFactor * (0.8 + Math.random() * 0.4))
  const transCount = config.transition_elements ? Math.round(45 * meshSizeFactor * (0.8 + Math.random() * 0.4)) : 0

  const beamNodes = beamCount * (config.beam_element_type === 'B32' ? 3 : 2)
  const shellNodes = Math.round(shellCount * 0.7)
  const solidNodes = Math.round(solidCount * 0.6)
  const transNodes = Math.round(transCount * 0.8)

  const totalNodes = beamNodes + shellNodes + solidNodes + transNodes
  const totalElements = beamCount + shellCount + solidCount + transCount

  const failedElements = config.mesh_quality_target === 'high' ? 0 : Math.round(Math.random() * 3)
  const warningElements = config.mesh_quality_target === 'high' ? Math.round(Math.random() * 5) : Math.round(Math.random() * 15 + 3)

  const aspectRatio = {
    min: parseFloat((1.0 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
    max: parseFloat((4.5 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
    avg: parseFloat((2.2 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
  }
  const jacobian = {
    min: parseFloat((0.65 / qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(3)),
    max: parseFloat((1.0).toFixed(3)),
    avg: parseFloat((0.88 / qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(3)),
  }
  const warpage = {
    min: parseFloat((0.5 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
    max: parseFloat((18.0 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
    avg: parseFloat((6.5 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
  }
  const skewness = {
    min: parseFloat((1.0 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
    max: parseFloat((35.0 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
    avg: parseFloat((12.0 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(2)),
  }
  const taper = {
    min: parseFloat((0.001 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(3)),
    max: parseFloat((0.15 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(3)),
    avg: parseFloat((0.04 * qualityMultiplier * (0.8 + Math.random() * 0.4)).toFixed(3)),
  }

  const qualityScore = config.mesh_quality_target === 'high'
    ? 0.85 + Math.random() * 0.12
    : config.mesh_quality_target === 'medium'
      ? 0.65 + Math.random() * 0.15
      : 0.40 + Math.random() * 0.20

  return {
    success: true,
    beam_elements: { count: beamCount, nodes: beamNodes },
    shell_elements: { count: shellCount, nodes: shellNodes },
    solid_elements: { count: solidCount, nodes: solidNodes },
    transition_elements: { count: transCount, nodes: transNodes },
    total_nodes: totalNodes,
    total_elements: totalElements,
    quality_metrics: {
      aspect_ratio: aspectRatio,
      jacobian: jacobian,
      warpage: warpage,
      skewness: skewness,
      taper: taper,
      element_quality_score: parseFloat(qualityScore.toFixed(3)),
      total_elements: totalElements,
      failed_elements: failedElements,
      warning_elements: warningElements,
    },
    mesh_stats: {
      beam_ratio: parseFloat(((beamCount / totalElements) * 100).toFixed(1)),
      shell_ratio: parseFloat(((shellCount / totalElements) * 100).toFixed(1)),
      solid_ratio: parseFloat(((solidCount / totalElements) * 100).toFixed(1)),
      trans_ratio: transCount > 0 ? parseFloat(((transCount / totalElements) * 100).toFixed(1)) : 0,
      avg_node_density: parseFloat((totalNodes / (totalElements * config.mesh_size * config.mesh_size) * 100).toFixed(2)),
    },
  }
}

/** 生成网格 */
async function generateMesh() {
  generating.value = true
  try {
    // 尝试调用后端 API
    // const result = await generateMixedMesh(config)
    // meshResult.value = result

    // 使用模拟数据
    await new Promise(resolve => setTimeout(resolve, 2000))
    meshResult.value = generateMockMeshResult()

    await nextTick()
    resultViewTab.value = 'preview'
    drawMeshPreview()
    drawGauge()
  } catch (e) {
    console.error('Mesh generation failed:', e)
    meshResult.value = generateMockMeshResult()
    await nextTick()
    drawMeshPreview()
    drawGauge()
  } finally {
    generating.value = false
  }
}

/** 绘制3D混合网格预览 */
function drawMeshPreview() {
  if (!meshCanvas.value || !meshResult.value) return

  const container = meshCanvas.value.parentElement
  if (!container) return

  meshCanvas.value.width = container.clientWidth
  meshCanvas.value.height = container.clientHeight

  const ctx = meshCanvas.value.getContext('2d')
  if (!ctx) return

  const w = meshCanvas.value.width
  const h = meshCanvas.value.height
  const padding = 60

  ctx.clearRect(0, 0, w, h)

  // 背景
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  // 网格
  ctx.strokeStyle = '#1C1D24'
  ctx.lineWidth = 0.5
  for (let x = padding; x < w - padding; x += 30) {
    ctx.beginPath()
    ctx.moveTo(x, padding)
    ctx.lineTo(x, h - padding)
    ctx.stroke()
  }
  for (let y = padding; y < h - padding; y += 30) {
    ctx.beginPath()
    ctx.moveTo(padding, y)
    ctx.lineTo(w - padding, y)
    ctx.stroke()
  }

  const result = meshResult.value
  const cx = w / 2
  const cy = h / 2
  const scale = Math.min(w, h) * 0.35

  // 简单等轴测投影
  const isoProject = (x: number, y: number, z: number) => ({
    sx: cx + (x - z) * 0.866 * scale / 100,
    sy: cy - y * scale / 100 + (x + z) * 0.5 * scale / 100,
  })

  // 绘制实体单元 (六面体) - 琥珀色
  const solidCount = Math.min(result.solid_elements.count, 60)
  ctx.strokeStyle = 'rgba(245, 158, 11, 0.4)'
  ctx.lineWidth = 0.5
  for (let i = 0; i < solidCount; i++) {
    const gx = (i % 8) * 12 - 48
    const gy = Math.floor(i / 8) * 12 - 36
    const gz = 0
    const s = 10
    const corners = [
      isoProject(gx, gy, gz),
      isoProject(gx + s, gy, gz),
      isoProject(gx + s, gy + s, gz),
      isoProject(gx, gy + s, gz),
      isoProject(gx, gy, gz + s),
      isoProject(gx + s, gy, gz + s),
      isoProject(gx + s, gy + s, gz + s),
      isoProject(gx, gy + s, gz + s),
    ]
    // 前面
    ctx.beginPath()
    ctx.moveTo(corners[0].sx, corners[0].sy)
    for (let j = 1; j < 4; j++) ctx.lineTo(corners[j].sx, corners[j].sy)
    ctx.closePath()
    ctx.stroke()
    // 后面
    ctx.beginPath()
    ctx.moveTo(corners[4].sx, corners[4].sy)
    for (let j = 5; j < 8; j++) ctx.lineTo(corners[j].sx, corners[j].sy)
    ctx.closePath()
    ctx.stroke()
    // 连接线
    for (let j = 0; j < 4; j++) {
      ctx.beginPath()
      ctx.moveTo(corners[j].sx, corners[j].sy)
      ctx.lineTo(corners[j + 4].sx, corners[j + 4].sy)
      ctx.stroke()
    }
  }

  // 绘制壳单元 (四边形) - 绿色
  const shellCount = Math.min(result.shell_elements.count, 40)
  ctx.strokeStyle = 'rgba(34, 197, 94, 0.6)'
  ctx.lineWidth = 0.8
  for (let i = 0; i < shellCount; i++) {
    const gx = (i % 10) * 10 - 50
    const gy = Math.floor(i / 10) * 10 - 20
    const gz = 50
    const s = 8
    const p1 = isoProject(gx, gy, gz)
    const p2 = isoProject(gx + s, gy, gz)
    const p3 = isoProject(gx + s, gy + s, gz)
    const p4 = isoProject(gx, gy + s, gz)
    ctx.beginPath()
    ctx.moveTo(p1.sx, p1.sy)
    ctx.lineTo(p2.sx, p2.sy)
    ctx.lineTo(p3.sx, p3.sy)
    ctx.lineTo(p4.sx, p4.sy)
    ctx.closePath()
    ctx.stroke()
    // 填充半透明
    ctx.fillStyle = 'rgba(34, 197, 94, 0.08)'
    ctx.fill()
  }

  // 绘制梁单元 (线段) - 蓝色
  const beamCount = Math.min(result.beam_elements.count, 20)
  ctx.strokeStyle = '#3B82F6'
  ctx.lineWidth = 2.5
  for (let i = 0; i < beamCount; i++) {
    const x1 = -50 + i * 5
    const y1 = 50
    const z1 = -30 + Math.floor(i / 5) * 20
    const x2 = x1 + 5
    const y2 = y1
    const z2 = z1
    const p1 = isoProject(x1, y1, z1)
    const p2 = isoProject(x2, y2, z2)
    ctx.beginPath()
    ctx.moveTo(p1.sx, p1.sy)
    ctx.lineTo(p2.sx, p2.sy)
    ctx.stroke()
    // 节点
    ctx.beginPath()
    ctx.arc(p1.sx, p1.sy, 2.5, 0, Math.PI * 2)
    ctx.fillStyle = '#3B82F6'
    ctx.fill()
  }
  // 最后一个节点
  {
    const p = isoProject(-50 + beamCount * 5, 50, -30 + Math.floor((beamCount - 1) / 5) * 20)
    ctx.beginPath()
    ctx.arc(p.sx, p.sy, 2.5, 0, Math.PI * 2)
    ctx.fillStyle = '#3B82F6'
    ctx.fill()
  }

  // 绘制过渡单元 - 紫色
  if (result.transition_elements.count > 0) {
    const transCount = Math.min(result.transition_elements.count, 10)
    ctx.strokeStyle = 'rgba(168, 85, 247, 0.7)'
    ctx.lineWidth = 1.5
    for (let i = 0; i < transCount; i++) {
      const gx = -20 + i * 8
      const gy = -30
      const gz = 25
      const s = 6
      const p1 = isoProject(gx, gy, gz)
      const p2 = isoProject(gx + s, gy, gz)
      const p3 = isoProject(gx + s * 0.5, gy + s, gz)
      ctx.beginPath()
      ctx.moveTo(p1.sx, p1.sy)
      ctx.lineTo(p2.sx, p2.sy)
      ctx.lineTo(p3.sx, p3.sy)
      ctx.closePath()
      ctx.stroke()
      ctx.fillStyle = 'rgba(168, 85, 247, 0.1)'
      ctx.fill()
    }
  }

  // 坐标轴
  const origin = isoProject(-60, -50, -40)
  const axisLen = 30
  const axX = isoProject(-60 + axisLen, -50, -40)
  const axY = isoProject(-60, -50 + axisLen, -40)
  const axZ = isoProject(-60, -50, -40 + axisLen)

  ctx.lineWidth = 1.5
  // X轴
  ctx.strokeStyle = '#EF4444'
  ctx.beginPath()
  ctx.moveTo(origin.sx, origin.sy)
  ctx.lineTo(axX.sx, axX.sy)
  ctx.stroke()
  ctx.fillStyle = '#EF4444'
  ctx.font = '11px sans-serif'
  ctx.fillText('X', axX.sx + 5, axX.sy)
  // Y轴
  ctx.strokeStyle = '#22C55E'
  ctx.beginPath()
  ctx.moveTo(origin.sx, origin.sy)
  ctx.lineTo(axY.sx, axY.sy)
  ctx.stroke()
  ctx.fillStyle = '#22C55E'
  ctx.fillText('Y', axY.sx + 5, axY.sy)
  // Z轴
  ctx.strokeStyle = '#3B82F6'
  ctx.beginPath()
  ctx.moveTo(origin.sx, origin.sy)
  ctx.lineTo(axZ.sx, axZ.sy)
  ctx.stroke()
  ctx.fillStyle = '#3B82F6'
  ctx.fillText('Z', axZ.sx + 5, axZ.sy)

  // 统计信息
  ctx.fillStyle = '#9CA3AF'
  ctx.font = '11px sans-serif'
  ctx.fillText(`梁: ${result.beam_elements.count}  壳: ${result.shell_elements.count}  实体: ${result.solid_elements.count}  过渡: ${result.transition_elements.count}`, padding, h - 20)
  ctx.fillText(`总节点: ${result.total_nodes}  总单元: ${result.total_elements}`, padding, h - 6)
}

/** 绘制质量评分仪表盘 */
function drawGauge() {
  if (!gaugeCanvas.value || !meshResult.value) return

  const ctx = gaugeCanvas.value.getContext('2d')
  if (!ctx) return

  const w = gaugeCanvas.value.width
  const h = gaugeCanvas.value.height
  const cx = w / 2
  const cy = h / 2 + 10
  const radius = 80

  ctx.clearRect(0, 0, w, h)

  const score = meshResult.value.quality_metrics.element_quality_score
  const startAngle = Math.PI * 0.75
  const endAngle = Math.PI * 2.25
  const scoreAngle = startAngle + (endAngle - startAngle) * score

  // 背景弧
  ctx.beginPath()
  ctx.arc(cx, cy, radius, startAngle, endAngle)
  ctx.strokeStyle = '#2D2E38'
  ctx.lineWidth = 12
  ctx.lineCap = 'round'
  ctx.stroke()

  // 评分弧
  const gradient = ctx.createLinearGradient(0, h, w, 0)
  gradient.addColorStop(0, '#EF4444')
  gradient.addColorStop(0.5, '#F59E0B')
  gradient.addColorStop(1, '#22C55E')

  ctx.beginPath()
  ctx.arc(cx, cy, radius, startAngle, scoreAngle)
  ctx.strokeStyle = gradient
  ctx.lineWidth = 12
  ctx.lineCap = 'round'
  ctx.stroke()

  // 刻度标记
  for (let i = 0; i <= 10; i++) {
    const angle = startAngle + (endAngle - startAngle) * (i / 10)
    const innerR = radius - 20
    const outerR = radius - 16
    ctx.beginPath()
    ctx.moveTo(cx + innerR * Math.cos(angle), cy + innerR * Math.sin(angle))
    ctx.lineTo(cx + outerR * Math.cos(angle), cy + outerR * Math.sin(angle))
    ctx.strokeStyle = '#4B5563'
    ctx.lineWidth = 1
    ctx.stroke()
  }
}

/** 导出网格 */
function exportMesh() {
  if (!meshResult.value) return

  const data = JSON.stringify(meshResult.value, null, 2)
  const blob = new Blob([data], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = 'mixed_mesh_result.json'
  a.click()
  URL.revokeObjectURL(url)
}

/** 监听结果标签切换，重绘 Canvas */
watch(resultViewTab, async (tab) => {
  if (!meshResult.value) return
  await nextTick()
  if (tab === 'preview') drawMeshPreview()
  if (tab === 'dashboard') drawGauge()
})

// ============ 生命周期 ============
onMounted(() => {
  templates.value = templateData
})
</script>

<style scoped>
.advanced-mesh-view {
  --bg-base: #0A0B0E;
  --bg-surface: #14151A;
  --bg-elevated: #1C1D24;
  --bg-hover: #25262E;
  --text-primary: #E8E9EB;
  --text-secondary: #9CA3AF;
  --text-muted: #6B7280;
  --primary: #4F8CFF;
  --accent-red: #EF4444;
  --accent-green: #22C55E;
  --accent-amber: #F59E0B;
  --border-subtle: #2D2E38;
}

.btn {
  @apply px-3 py-1.5 rounded text-xs font-medium transition;
}

.btn-primary {
  @apply bg-blue-600 text-white hover:bg-blue-700;
}

.btn-primary:disabled {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.btn-ghost {
  @apply text-gray-400 hover:text-white hover:bg-gray-700;
}

.btn-outline {
  @apply border border-gray-600 text-gray-300 hover:bg-gray-700;
}

.btn-loading {
  @apply bg-blue-800 text-white cursor-not-allowed;
}

.input {
  @apply px-2 py-1.5 bg-[var(--bg-base)] border border-[var(--border-subtle)] rounded text-xs text-[var(--text-primary)] focus:outline-none focus:border-[var(--primary)];
}

.stat-card {
  @apply p-3 rounded-lg bg-[var(--bg-base)];
}

.quality-card {
  @apply p-3 rounded-lg bg-[var(--bg-base)];
}

.step-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-subtle);
}

.step-dot.active {
  background: var(--primary);
  box-shadow: 0 0 0 3px rgba(79, 140, 255, 0.2);
}

.step-dot.completed {
  background: var(--accent-green);
}
</style>
