<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">原子结构建模器</h2>
        <p class="text-sm" style="color: var(--text-muted)">晶体/非晶/界面/碳纳米管/团簇/位错，参数化生成</p>
      </div>
      <div class="flex items-center gap-2 flex-wrap justify-end">
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
        <button v-if="result" @click="exportAtoms" class="px-3 py-1.5 text-xs border rounded transition" style="border-color: var(--accent-green); color: var(--accent-green)">
          导出坐标
        </button>
      </div>
    </div>

    <!-- Tab Bar -->
    <div class="flex items-center gap-1 px-4 py-2 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        @click="activeTab = tab.key"
        class="px-3 py-1.5 text-xs rounded-full transition"
        :style="activeTab === tab.key
          ? 'background: var(--primary); color: #fff'
          : 'background: var(--bg-elevated); color: var(--text-secondary)'"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- ========== 超胞面板 ========== -->
        <template v-if="activeTab === 'supercell'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">晶体结构</h4>
            <div class="grid grid-cols-3 gap-1">
              <button
                v-for="cs in crystalStructures"
                :key="cs.value"
                @click="supercellConfig.crystal_structure = cs.value"
                :class="['px-2 py-2 rounded text-xs text-center transition border', supercellConfig.crystal_structure === cs.value ? 'text-white' : '']"
                :style="supercellConfig.crystal_structure === cs.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                {{ cs.label }}
              </button>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">元素选择</h4>
            <select v-model="supercellElementSymbol" @change="onSupercellElementChange" class="input w-full text-xs">
              <option v-for="el in elementLibrary" :key="el.symbol" :value="el.symbol">
                {{ el.symbol }} - {{ el.name }} ({{ el.crystal_structure }}, a={{ el.lattice_constant_A }} A)
              </option>
              <option value="custom">自定义</option>
            </select>
            <div v-if="supercellElementSymbol === 'custom'" class="mt-2 space-y-2">
              <div>
                <label class="label">晶格常数 (A)</label>
                <input v-model.number="supercellConfig.lattice_constant_A" type="number" step="0.01" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">超胞尺寸</h4>
            <div class="grid grid-cols-3 gap-2">
              <div>
                <label class="label">nx</label>
                <input v-model.number="supercellConfig.nx" type="number" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">ny</label>
                <input v-model.number="supercellConfig.ny" type="number" min="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">nz</label>
                <input v-model.number="supercellConfig.nz" type="number" min="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">晶向</h4>
            <div class="grid grid-cols-3 gap-1">
              <button
                v-for="ori in orientations"
                :key="ori.label"
                @click="supercellConfig.orientation = ori.value"
                :class="['px-2 py-2 rounded text-xs text-center transition border', isOrientationEqual(supercellConfig.orientation, ori.value) ? 'text-white' : '']"
                :style="isOrientationEqual(supercellConfig.orientation, ori.value)
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                [{{ ori.label }}]
              </button>
            </div>
          </div>

          <div class="panel-section">
            <div>
              <label class="label">真空层 (A)</label>
              <input v-model.number="supercellConfig.vacuum_A" type="number" step="0.5" min="0" class="input w-full text-xs" />
            </div>
          </div>
        </template>

        <!-- ========== 非晶面板 ========== -->
        <template v-if="activeTab === 'amorphous'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">元素选择</h4>
            <select v-model="amorphousElementSymbol" @change="onAmorphousElementChange" class="input w-full text-xs">
              <option v-for="el in elementLibrary" :key="el.symbol" :value="el.symbol">
                {{ el.symbol }} - {{ el.name }}
              </option>
            </select>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">结构参数</h4>
            <div class="space-y-2">
              <div>
                <label class="label">原子数</label>
                <input v-model.number="amorphousConfig.num_atoms" type="number" min="10" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">密度 (g/cm3)</label>
                <input v-model.number="amorphousConfig.density_g_cm3" type="number" step="0.01" min="0.1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">盒子尺寸 (A)</label>
                <input v-model.number="amorphousConfig.box_size_A" type="number" step="0.5" min="5" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">随机种子</label>
                <input v-model.number="amorphousConfig.random_seed" type="number" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">淬火速率 (K/ps)</label>
                <input v-model.number="amorphousConfig.quench_rate_K_ps" type="number" step="0.1" min="0.01" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </template>

        <!-- ========== 界面面板 ========== -->
        <template v-if="activeTab === 'interface'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">下层</h4>
            <div class="space-y-2">
              <div>
                <label class="label">元素</label>
                <select v-model="interfaceElementBottomSymbol" @change="onInterfaceBottomChange" class="input w-full text-xs">
                  <option v-for="el in elementLibrary" :key="el.symbol" :value="el.symbol">{{ el.symbol }} - {{ el.name }}</option>
                </select>
              </div>
              <div>
                <label class="label">晶体结构</label>
                <select v-model="interfaceConfig.crystal_bottom" class="input w-full text-xs">
                  <option v-for="cs in crystalStructures" :key="cs.value" :value="cs.value">{{ cs.label }}</option>
                </select>
              </div>
              <div>
                <label class="label">晶向</label>
                <div class="grid grid-cols-3 gap-1">
                  <button
                    v-for="ori in orientations"
                    :key="'bot-'+ori.label"
                    @click="interfaceConfig.orientation_bottom = { ...ori.value }"
                    :class="['px-2 py-1 rounded text-xs text-center transition border', isOrientationEqual(interfaceConfig.orientation_bottom, ori.value) ? 'text-white' : '']"
                    :style="isOrientationEqual(interfaceConfig.orientation_bottom, ori.value)
                      ? 'background: var(--primary); border-color: var(--primary)'
                      : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                  >
                    [{{ ori.label }}]
                  </button>
                </div>
              </div>
              <div>
                <label class="label">层数</label>
                <input v-model.number="interfaceConfig.num_layers_bottom" type="number" min="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">上层</h4>
            <div class="space-y-2">
              <div>
                <label class="label">元素</label>
                <select v-model="interfaceElementTopSymbol" @change="onInterfaceTopChange" class="input w-full text-xs">
                  <option v-for="el in elementLibrary" :key="el.symbol" :value="el.symbol">{{ el.symbol }} - {{ el.name }}</option>
                </select>
              </div>
              <div>
                <label class="label">晶体结构</label>
                <select v-model="interfaceConfig.crystal_top" class="input w-full text-xs">
                  <option v-for="cs in crystalStructures" :key="cs.value" :value="cs.value">{{ cs.label }}</option>
                </select>
              </div>
              <div>
                <label class="label">晶向</label>
                <div class="grid grid-cols-3 gap-1">
                  <button
                    v-for="ori in orientations"
                    :key="'top-'+ori.label"
                    @click="interfaceConfig.orientation_top = { ...ori.value }"
                    :class="['px-2 py-1 rounded text-xs text-center transition border', isOrientationEqual(interfaceConfig.orientation_top, ori.value) ? 'text-white' : '']"
                    :style="isOrientationEqual(interfaceConfig.orientation_top, ori.value)
                      ? 'background: var(--primary); border-color: var(--primary)'
                      : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
                  >
                    [{{ ori.label }}]
                  </button>
                </div>
              </div>
              <div>
                <label class="label">层数</label>
                <input v-model.number="interfaceConfig.num_layers_top" type="number" min="1" class="input w-full text-xs" />
              </div>
            </div>
          </div>

          <div class="panel-section">
            <div class="space-y-2">
              <div>
                <label class="label">失配容忍度</label>
                <input v-model.number="interfaceConfig.mismatch_tolerance" type="number" step="0.01" min="0" max="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">真空层 (A)</label>
                <input v-model.number="interfaceConfig.vacuum_A" type="number" step="0.5" min="0" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </template>

        <!-- ========== 缺陷面板 ========== -->
        <template v-if="activeTab === 'defect'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">基础超胞</h4>
            <div class="space-y-2">
              <div>
                <label class="label">晶体结构</label>
                <select v-model="defectConfig.base_config.crystal_structure" class="input w-full text-xs">
                  <option v-for="cs in crystalStructures" :key="cs.value" :value="cs.value">{{ cs.label }}</option>
                </select>
              </div>
              <div>
                <label class="label">元素</label>
                <select v-model="defectElementSymbol" @change="onDefectElementChange" class="input w-full text-xs">
                  <option v-for="el in elementLibrary" :key="el.symbol" :value="el.symbol">{{ el.symbol }} - {{ el.name }}</option>
                </select>
              </div>
              <div class="grid grid-cols-3 gap-2">
                <div>
                  <label class="label">nx</label>
                  <input v-model.number="defectConfig.base_config.nx" type="number" min="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">ny</label>
                  <input v-model.number="defectConfig.base_config.ny" type="number" min="1" class="input w-full text-xs" />
                </div>
                <div>
                  <label class="label">nz</label>
                  <input v-model.number="defectConfig.base_config.nz" type="number" min="1" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">缺陷类型</h4>
            <div class="grid grid-cols-2 gap-1">
              <button
                v-for="dt in defectTypes"
                :key="dt.value"
                @click="defectConfig.defect_type = dt.value"
                :class="['px-2 py-2 rounded text-xs text-left transition border', defectConfig.defect_type === dt.value ? 'text-white' : '']"
                :style="defectConfig.defect_type === dt.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                <div class="font-medium">{{ dt.label }}</div>
              </button>
            </div>
          </div>

          <div class="panel-section">
            <div>
              <label class="label">缺陷密度</label>
              <input v-model.number="defectConfig.defect_density" type="number" step="0.001" min="0" max="1" class="input w-full text-xs" />
            </div>
          </div>
        </template>

        <!-- ========== 碳纳米管面板 ========== -->
        <template v-if="activeTab === 'cnt'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">手性指数</h4>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">n</label>
                <input v-model.number="cntConfig.chirality_n" type="number" min="1" max="50" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">m</label>
                <input v-model.number="cntConfig.chirality_m" type="number" min="0" max="50" class="input w-full text-xs" />
              </div>
            </div>
            <div class="mt-2 text-xs" style="color: var(--text-muted)">
              手性角: {{ cntChiralAngle.toFixed(1) }} deg |
              直径: {{ cntDiameter.toFixed(2) }} A |
              {{ cntType }}
            </div>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">管参数</h4>
            <div class="space-y-2">
              <div>
                <label class="label">长度 (A)</label>
                <input v-model.number="cntConfig.length_A" type="number" step="1" min="5" class="input w-full text-xs" />
              </div>
              <div class="flex items-center gap-2">
                <input type="checkbox" v-model="cntConfig.cap_end" id="cnt-cap" class="rounded" />
                <label for="cnt-cap" class="text-xs" style="color: var(--text-secondary)">端帽封闭</label>
              </div>
              <div>
                <label class="label">空位缺陷密度</label>
                <input v-model.number="cntConfig.vacancy_defect_density" type="number" step="0.001" min="0" max="0.5" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </template>

        <!-- ========== 团簇面板 ========== -->
        <template v-if="activeTab === 'cluster'">
          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">元素选择</h4>
            <select v-model="clusterElementSymbol" @change="onClusterElementChange" class="input w-full text-xs">
              <option v-for="el in elementLibrary" :key="el.symbol" :value="el.symbol">{{ el.symbol }} - {{ el.name }}</option>
            </select>
          </div>

          <div class="panel-section">
            <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">团簇类型</h4>
            <div class="grid grid-cols-2 gap-1">
              <button
                v-for="ct in clusterTypes"
                :key="ct.value"
                @click="clusterConfig.cluster_type = ct.value"
                :class="['px-2 py-2 rounded text-xs text-left transition border', clusterConfig.cluster_type === ct.value ? 'text-white' : '']"
                :style="clusterConfig.cluster_type === ct.value
                  ? 'background: var(--primary); border-color: var(--primary)'
                  : 'background: var(--bg-elevated); border-color: var(--border-default); color: var(--text-secondary)'"
              >
                <div class="font-medium">{{ ct.label }}</div>
              </button>
            </div>
          </div>

          <div class="panel-section">
            <div class="space-y-2">
              <div>
                <label class="label">原子数</label>
                <input v-model.number="clusterConfig.num_atoms" type="number" min="1" max="10000" class="input w-full text-xs" />
              </div>
              <div>
                <label class="label">晶格常数 (A)</label>
                <input v-model.number="clusterConfig.lattice_constant_A" type="number" step="0.01" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </template>

        <!-- Generate Button -->
        <div class="space-y-2">
          <button
            @click="generateStructure"
            :disabled="generating"
            class="btn btn-primary text-xs w-full"
          >
            <span v-if="generating" class="mr-1 animate-spin">&#10227;</span>
            {{ generating ? '生成中...' : '生成结构' }}
          </button>
        </div>
      </div>

      <!-- Right Panel: Visualization & Stats -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- No result placeholder -->
        <div v-if="!result" class="flex-1 flex items-center justify-center" style="color: var(--text-muted)">
          <div class="text-center">
            <div class="text-4xl mb-2">&#9883;</div>
            <div class="text-sm">配置参数后生成原子结构</div>
          </div>
        </div>

        <!-- Results -->
        <template v-else>
          <div class="flex-1 flex overflow-hidden">
            <!-- 3D Canvas -->
            <div class="flex-1 relative">
              <canvas
                ref="previewCanvas"
                class="w-full h-full cursor-grab active:cursor-grabbing"
                style="background: var(--bg-base)"
                @mousedown="onCanvasMouseDown"
                @mousemove="onCanvasMouseMove"
                @mouseup="onCanvasMouseUp"
                @mouseleave="onCanvasMouseUp"
                @wheel.prevent="onCanvasWheel"
              ></canvas>
              <!-- Canvas overlay info -->
              <div class="absolute top-3 left-3 text-[10px] px-2 py-1 rounded" style="background: rgba(0,0,0,0.6); color: #ccc">
                拖拽旋转 | 滚轮缩放 | {{ result.atoms.length }} atoms
              </div>
            </div>

            <!-- Right Sidebar: Stats -->
            <div class="w-72 overflow-y-auto border-l p-4 space-y-4" style="background: var(--bg-surface); border-color: var(--border-subtle)">
              <!-- Structure Statistics -->
              <div class="result-card">
                <span class="result-label">结构统计</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">原子数</span>
                    <span class="text-xs font-mono font-semibold" style="color: var(--text-primary)">{{ result.num_atoms }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">元素种类</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.num_types }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">盒子 Lx</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.box.lx.toFixed(3) }} A</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">盒子 Ly</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.box.ly.toFixed(3) }} A</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">盒子 Lz</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.box.lz.toFixed(3) }} A</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">密度</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.density.toFixed(4) }} g/cm3</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">体积</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.volume_A3.toFixed(2) }} A3</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">总质量</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ result.mass_amu.toFixed(2) }} amu</span>
                  </div>
                </div>
              </div>

              <!-- Element Info Card -->
              <div class="result-card">
                <span class="result-label">元素信息</span>
                <div class="space-y-2 mt-2">
                  <div v-for="elInfo in elementInfoList" :key="elInfo.symbol" class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full inline-block" :style="{ background: elInfo.color }"></span>
                    <span class="text-xs font-medium" style="color: var(--text-primary)">{{ elInfo.symbol }}</span>
                    <span class="text-xs" style="color: var(--text-muted)">{{ elInfo.name }}</span>
                    <span class="text-xs ml-auto font-mono" style="color: var(--text-secondary)">{{ elInfo.count }}</span>
                  </div>
                </div>
              </div>

              <!-- CNT specific info -->
              <div v-if="activeTab === 'cnt'" class="result-card">
                <span class="result-label">碳纳米管参数</span>
                <div class="space-y-2 mt-2">
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">手性</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">({{ cntConfig.chirality_n }},{{ cntConfig.chirality_m }})</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">类型</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ cntType }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">直径</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ cntDiameter.toFixed(2) }} A</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-xs" style="color: var(--text-secondary)">手性角</span>
                    <span class="text-xs font-mono" style="color: var(--text-primary)">{{ cntChiralAngle.toFixed(1) }} deg</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted, nextTick } from 'vue'
import type {
  CrystalStructure,
  Element,
  MdAtom,
  AtomBuilderResult,
  AtomBuilderTemplate,
} from '../api/atomBuilder'

// ============ Constants ============

const elementLibrary: Element[] = [
  { symbol: 'Fe', name: '铁', atomic_number: 26, mass_amu: 55.845, lattice_constant_A: 2.87, crystal_structure: 'BCC', covalent_radius_A: 1.32, color: '#CD7F32' },
  { symbol: 'Al', name: '铝', atomic_number: 13, mass_amu: 26.982, lattice_constant_A: 4.05, crystal_structure: 'FCC', covalent_radius_A: 1.21, color: '#C0C0C0' },
  { symbol: 'Cu', name: '铜', atomic_number: 29, mass_amu: 63.546, lattice_constant_A: 3.615, crystal_structure: 'FCC', covalent_radius_A: 1.32, color: '#B87333' },
  { symbol: 'Ni', name: '镍', atomic_number: 28, mass_amu: 58.693, lattice_constant_A: 3.524, crystal_structure: 'FCC', covalent_radius_A: 1.24, color: '#72787E' },
  { symbol: 'Au', name: '金', atomic_number: 79, mass_amu: 196.967, lattice_constant_A: 4.078, crystal_structure: 'FCC', covalent_radius_A: 1.34, color: '#FFD700' },
  { symbol: 'Si', name: '硅', atomic_number: 14, mass_amu: 28.086, lattice_constant_A: 5.431, crystal_structure: 'diamond', covalent_radius_A: 1.11, color: '#8C7853' },
  { symbol: 'C', name: '碳', atomic_number: 6, mass_amu: 12.011, lattice_constant_A: 3.567, crystal_structure: 'diamond', covalent_radius_A: 0.77, color: '#404040' },
]

const tabs = [
  { key: 'supercell', label: '超胞' },
  { key: 'amorphous', label: '非晶' },
  { key: 'interface', label: '界面' },
  { key: 'defect', label: '缺陷' },
  { key: 'cnt', label: '碳纳米管' },
  { key: 'cluster', label: '团簇' },
]

const crystalStructures = [
  { value: 'FCC' as CrystalStructure, label: 'FCC' },
  { value: 'BCC' as CrystalStructure, label: 'BCC' },
  { value: 'HCP' as CrystalStructure, label: 'HCP' },
  { value: 'diamond' as CrystalStructure, label: '金刚石' },
  { value: 'SC' as CrystalStructure, label: 'SC' },
]

const orientations = [
  { label: '100', value: { x: 1, y: 0, z: 0 } },
  { label: '110', value: { x: 1, y: 1, z: 0 } },
  { label: '111', value: { x: 1, y: 1, z: 1 } },
]

const defectTypes = [
  { value: 'vacancy' as const, label: '空位' },
  { value: 'interstitial' as const, label: '间隙' },
  { value: 'substitution' as const, label: '替换' },
  { value: 'dislocation_edge' as const, label: '刃位错' },
  { value: 'dislocation_screw' as const, label: '螺位错' },
  { value: 'grain_boundary' as const, label: '晶界' },
]

const clusterTypes = [
  { value: 'icosahedron' as const, label: '二十面体' },
  { value: 'decahedron' as const, label: '十面体' },
  { value: 'fcc_cube' as const, label: 'FCC立方' },
  { value: 'bcc_cube' as const, label: 'BCC立方' },
  { value: 'sphere' as const, label: '球形' },
]

const quickTemplates: AtomBuilderTemplate[] = [
  { id: 'fe_bcc', name: 'Fe BCC超胞', name_en: 'Fe BCC Supercell', category: 'supercell', description: '铁BCC超胞', config_type: 'supercell', typical_application: '钢铁材料模拟' },
  { id: 'al_fcc', name: 'Al FCC超胞', name_en: 'Al FCC Supercell', category: 'supercell', description: '铝FCC超胞', config_type: 'supercell', typical_application: '铝合金模拟' },
  { id: 'cu_sigma5', name: 'Cu Sigma5晶界', name_en: 'Cu Sigma5 GB', category: 'defect', description: '铜Sigma5晶界', config_type: 'defect', typical_application: '晶界研究' },
  { id: 'cnt_10_10', name: '(10,10)碳纳米管', name_en: '(10,10) CNT', category: 'cnt', description: '扶手椅型碳纳米管', config_type: 'cnt', typical_application: '纳米材料' },
  { id: 'al_ico', name: 'Al 二十面体团簇', name_en: 'Al Icosahedron', category: 'cluster', description: '铝二十面体团簇', config_type: 'cluster', typical_application: '纳米颗粒' },
  { id: 'si_amorphous', name: 'Si 非晶', name_en: 'Amorphous Si', category: 'amorphous', description: '非晶硅', config_type: 'amorphous', typical_application: '非晶半导体' },
]

// ============ State ============

const activeTab = ref<string>('supercell')
const generating = ref(false)
const result = ref<AtomBuilderResult | null>(null)
const previewCanvas = ref<HTMLCanvasElement | null>(null)

// Canvas rotation state
const rotX = ref(-0.4)
const rotY = ref(0.6)
const zoom = ref(1.0)
let isDragging = false
let lastMouseX = 0
let lastMouseY = 0
let animFrameId = 0

// Supercell config
const supercellElementSymbol = ref('Fe')
const supercellConfig = reactive({
  nx: 4,
  ny: 4,
  nz: 4,
  crystal_structure: 'BCC' as CrystalStructure,
  element: { ...elementLibrary[0] },
  lattice_constant_A: 2.87,
  orientation: { x: 1, y: 0, z: 0 },
  vacuum_A: 0,
})

// Amorphous config
const amorphousElementSymbol = ref('Si')
const amorphousConfig = reactive({
  element: { ...elementLibrary[5] },
  num_atoms: 500,
  density_g_cm3: 2.33,
  box_size_A: 20,
  random_seed: 42,
  quench_rate_K_ps: 10,
})

// Interface config
const interfaceElementBottomSymbol = ref('Cu')
const interfaceElementTopSymbol = ref('Ni')
const interfaceConfig = reactive({
  element_bottom: { ...elementLibrary[2] },
  element_top: { ...elementLibrary[3] },
  crystal_bottom: 'FCC' as CrystalStructure,
  crystal_top: 'FCC' as CrystalStructure,
  orientation_bottom: { x: 1, y: 0, z: 0 },
  orientation_top: { x: 1, y: 0, z: 0 },
  mismatch_tolerance: 0.05,
  vacuum_A: 15,
  num_layers_bottom: 10,
  num_layers_top: 10,
})

// Defect config
const defectElementSymbol = ref('Cu')
const defectConfig = reactive({
  base_config: {
    nx: 10,
    ny: 10,
    nz: 10,
    crystal_structure: 'FCC' as CrystalStructure,
    element: { ...elementLibrary[2] },
    lattice_constant_A: 3.615,
    orientation: { x: 1, y: 0, z: 0 },
    vacuum_A: 0,
  },
  defect_type: 'vacancy' as 'vacancy' | 'interstitial' | 'substitution' | 'dislocation_edge' | 'dislocation_screw' | 'grain_boundary',
  defect_density: 0.02,
  defect_positions: [] as { x: number; y: number; z: number }[],
})

// CNT config
const cntConfig = reactive({
  chirality_n: 10,
  chirality_m: 10,
  length_A: 30,
  element: { ...elementLibrary[6] },
  cap_end: true,
  vacancy_defect_density: 0,
})

// Cluster config
const clusterElementSymbol = ref('Al')
const clusterConfig = reactive({
  element: { ...elementLibrary[1] },
  cluster_type: 'icosahedron' as 'icosahedron' | 'decahedron' | 'fcc_cube' | 'bcc_cube' | 'sphere',
  num_atoms: 55,
  lattice_constant_A: 4.05,
})

// ============ Computed ============

const cntChiralAngle = computed(() => {
  const n = cntConfig.chirality_n
  const m = cntConfig.chirality_m
  if (n === 0 && m === 0) return 0
  return (Math.atan2(Math.sqrt(3) * m, 2 * n + m)) * 180 / Math.PI
})

const cntDiameter = computed(() => {
  const n = cntConfig.chirality_n
  const m = cntConfig.chirality_m
  const aCC = 1.421 // C-C bond length in graphene (A)
  return (aCC / Math.PI) * Math.sqrt(3 * (n * n + m * m + n * m))
})

const cntType = computed(() => {
  const n = cntConfig.chirality_n
  const m = cntConfig.chirality_m
  if (m === 0) return '锯齿型 (Zigzag)'
  if (n === m) return '扶手椅型 (Armchair)'
  return '手性型 (Chiral)'
})

const elementInfoList = computed(() => {
  if (!result.value) return []
  const counts: Record<string, { count: number; color: string; name: string }> = {}
  for (const atom of result.value.atoms) {
    if (!counts[atom.symbol]) {
      const el = elementLibrary.find(e => e.symbol === atom.symbol)
      counts[atom.symbol] = { count: 0, color: el?.color ?? '#888', name: el?.name ?? atom.symbol }
    }
    counts[atom.symbol].count++
  }
  return Object.entries(counts).map(([symbol, info]) => ({
    symbol,
    count: info.count,
    color: info.color,
    name: info.name,
  }))
})

// ============ Methods ============

function isOrientationEqual(a: { x: number; y: number; z: number }, b: { x: number; y: number; z: number }): boolean {
  return a.x === b.x && a.y === b.y && a.z === b.z
}

function onSupercellElementChange() {
  const el = elementLibrary.find(e => e.symbol === supercellElementSymbol.value)
  if (el) {
    supercellConfig.element = { ...el }
    supercellConfig.lattice_constant_A = el.lattice_constant_A
    supercellConfig.crystal_structure = el.crystal_structure
  }
}

function onAmorphousElementChange() {
  const el = elementLibrary.find(e => e.symbol === amorphousElementSymbol.value)
  if (el) {
    amorphousConfig.element = { ...el }
  }
}

function onInterfaceBottomChange() {
  const el = elementLibrary.find(e => e.symbol === interfaceElementBottomSymbol.value)
  if (el) {
    interfaceConfig.element_bottom = { ...el }
    interfaceConfig.crystal_bottom = el.crystal_structure
  }
}

function onInterfaceTopChange() {
  const el = elementLibrary.find(e => e.symbol === interfaceElementTopSymbol.value)
  if (el) {
    interfaceConfig.element_top = { ...el }
    interfaceConfig.crystal_top = el.crystal_structure
  }
}

function onDefectElementChange() {
  const el = elementLibrary.find(e => e.symbol === defectElementSymbol.value)
  if (el) {
    defectConfig.base_config.element = { ...el }
    defectConfig.base_config.lattice_constant_A = el.lattice_constant_A
    defectConfig.base_config.crystal_structure = el.crystal_structure
  }
}

function onClusterElementChange() {
  const el = elementLibrary.find(e => e.symbol === clusterElementSymbol.value)
  if (el) {
    clusterConfig.element = { ...el }
    clusterConfig.lattice_constant_A = el.lattice_constant_A
  }
}

function applyTemplate(tpl: AtomBuilderTemplate) {
  switch (tpl.id) {
    case 'fe_bcc':
      activeTab.value = 'supercell'
      supercellElementSymbol.value = 'Fe'
      onSupercellElementChange()
      supercellConfig.nx = 5
      supercellConfig.ny = 5
      supercellConfig.nz = 5
      supercellConfig.orientation = { x: 1, y: 0, z: 0 }
      supercellConfig.vacuum_A = 0
      break
    case 'al_fcc':
      activeTab.value = 'supercell'
      supercellElementSymbol.value = 'Al'
      onSupercellElementChange()
      supercellConfig.nx = 4
      supercellConfig.ny = 4
      supercellConfig.nz = 4
      supercellConfig.orientation = { x: 1, y: 1, z: 1 }
      supercellConfig.vacuum_A = 0
      break
    case 'cu_sigma5':
      activeTab.value = 'defect'
      defectElementSymbol.value = 'Cu'
      onDefectElementChange()
      defectConfig.base_config.nx = 10
      defectConfig.base_config.ny = 10
      defectConfig.base_config.nz = 10
      defectConfig.defect_type = 'grain_boundary'
      defectConfig.defect_density = 0.01
      break
    case 'cnt_10_10':
      activeTab.value = 'cnt'
      cntConfig.chirality_n = 10
      cntConfig.chirality_m = 10
      cntConfig.length_A = 30
      cntConfig.cap_end = true
      cntConfig.vacancy_defect_density = 0
      break
    case 'al_ico':
      activeTab.value = 'cluster'
      clusterElementSymbol.value = 'Al'
      onClusterElementChange()
      clusterConfig.cluster_type = 'icosahedron'
      clusterConfig.num_atoms = 55
      break
    case 'si_amorphous':
      activeTab.value = 'amorphous'
      amorphousElementSymbol.value = 'Si'
      onAmorphousElementChange()
      amorphousConfig.num_atoms = 500
      amorphousConfig.density_g_cm3 = 2.33
      amorphousConfig.box_size_A = 20
      amorphousConfig.random_seed = 42
      amorphousConfig.quench_rate_K_ps = 10
      break
  }
}

function resetAll() {
  result.value = null
  activeTab.value = 'supercell'
  supercellElementSymbol.value = 'Fe'
  onSupercellElementChange()
  supercellConfig.nx = 4
  supercellConfig.ny = 4
  supercellConfig.nz = 4
  supercellConfig.orientation = { x: 1, y: 0, z: 0 }
  supercellConfig.vacuum_A = 0
  rotX.value = -0.4
  rotY.value = 0.6
  zoom.value = 1.0
}

// ============ Crystallographic Atom Generation ============

/**
 * Generate FCC unit cell basis atoms (4 atoms per cell)
 */
function generateFCCAtoms(a: number, nx: number, ny: number, nz: number): MdAtom[] {
  const atoms: MdAtom[] = []
  const basis = [
    [0, 0, 0],
    [0.5, 0.5, 0],
    [0.5, 0, 0.5],
    [0, 0.5, 0.5],
  ]
  let id = 0
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        for (const [bx, by, bz] of basis) {
          atoms.push({
            id,
            type: 1,
            symbol: '',
            x: (ix + bx) * a,
            y: (iy + by) * a,
            z: (iz + bz) * a,
            fx: 0, fy: 0, fz: 0,
          })
          id++
        }
      }
    }
  }
  return atoms
}

/**
 * Generate BCC unit cell basis atoms (2 atoms per cell)
 */
function generateBCCAtoms(a: number, nx: number, ny: number, nz: number): MdAtom[] {
  const atoms: MdAtom[] = []
  const basis = [
    [0, 0, 0],
    [0.5, 0.5, 0.5],
  ]
  let id = 0
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        for (const [bx, by, bz] of basis) {
          atoms.push({
            id,
            type: 1,
            symbol: '',
            x: (ix + bx) * a,
            y: (iy + by) * a,
            z: (iz + bz) * a,
            fx: 0, fy: 0, fz: 0,
          })
          id++
        }
      }
    }
  }
  return atoms
}

/**
 * Generate HCP unit cell basis atoms (2 atoms per cell)
 * Uses ideal c/a ratio = sqrt(8/3)
 */
function generateHCPAtoms(a: number, nx: number, ny: number, nz: number): MdAtom[] {
  const atoms: MdAtom[] = []
  const c = a * Math.sqrt(8 / 3)
  const basis = [
    [0, 0, 0],
    [1 / 3, 2 / 3, 0.5],
  ]
  let id = 0
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        for (const [bx, by, bz] of basis) {
          const px = (ix + bx) * a + (iy + by) * a * 0.5
          const py = (iy + by) * a * Math.sqrt(3) / 2
          const pz = (iz + bz) * c
          atoms.push({
            id,
            type: 1,
            symbol: '',
            x: px,
            y: py,
            z: pz,
            fx: 0, fy: 0, fz: 0,
          })
          id++
        }
      }
    }
  }
  return atoms
}

/**
 * Generate diamond cubic unit cell basis atoms (8 atoms per cell)
 */
function generateDiamondAtoms(a: number, nx: number, ny: number, nz: number): MdAtom[] {
  const atoms: MdAtom[] = []
  const basis = [
    [0, 0, 0],
    [0.5, 0.5, 0],
    [0.5, 0, 0.5],
    [0, 0.5, 0.5],
    [0.25, 0.25, 0.25],
    [0.75, 0.75, 0.25],
    [0.75, 0.25, 0.75],
    [0.25, 0.75, 0.75],
  ]
  let id = 0
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        for (const [bx, by, bz] of basis) {
          atoms.push({
            id,
            type: 1,
            symbol: '',
            x: (ix + bx) * a,
            y: (iy + by) * a,
            z: (iz + bz) * a,
            fx: 0, fy: 0, fz: 0,
          })
          id++
        }
      }
    }
  }
  return atoms
}

/**
 * Generate simple cubic unit cell atoms (1 atom per cell)
 */
function generateSCAtoms(a: number, nx: number, ny: number, nz: number): MdAtom[] {
  const atoms: MdAtom[] = []
  let id = 0
  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        atoms.push({
          id,
          type: 1,
          symbol: '',
          x: ix * a,
          y: iy * a,
          z: iz * a,
          fx: 0, fy: 0, fz: 0,
        })
        id++
      }
    }
  }
  return atoms
}

/**
 * Generate carbon nanotube atoms using chiral vector formula
 * Ch = n*a1 + m*a2, where a1, a2 are graphene lattice vectors
 * |a1| = |a2| = a = sqrt(3) * aCC
 */
function generateCNTAtoms(n: number, m: number, length: number): MdAtom[] {
  const aCC = 1.421 // C-C bond length in A
  const a = Math.sqrt(3) * aCC // graphene lattice constant

  // Chiral vector
  const Ch_x = a * (n + m * 0.5)
  const Ch_y = a * (m * Math.sqrt(3) / 2)
  const Ch = Math.sqrt(Ch_x * Ch_x + Ch_y * Ch_y) // circumference

  // Translational vector magnitude
  const T_mag = a * Math.sqrt(3) * Math.sqrt(n * n + m * m + n * m) / gcd(n, m)

  // Number of atoms per unit cell
  const N = 2 * (n * n + m * m + n * m) / gcd(n, m)

  // Generate atoms along the tube
  const atoms: MdAtom[] = []
  const R = Ch / (2 * Math.PI) // tube radius
  const numRepeats = Math.max(1, Math.round(length / T_mag))
  let id = 0

  // Use parametric approach: generate atoms on a cylinder
  const numAtomsCircum = Math.max(1, Math.round(Ch / aCC))
  const numAtomsLength = Math.max(1, Math.round(length / (aCC * Math.sqrt(3) / 2)))

  for (let j = 0; j < numAtomsLength; j++) {
    for (let i = 0; i < numAtomsCircum; i++) {
      const theta = (2 * Math.PI * i) / numAtomsCircum
      const z = j * (aCC * Math.sqrt(3) / 2)

      // Zigzag/armchair pattern offset
      const offset = (j % 2 === 0) ? 0 : Math.PI / numAtomsCircum
      const thetaOffset = theta + offset

      const x = R * Math.cos(thetaOffset)
      const y = R * Math.sin(thetaOffset)

      atoms.push({
        id,
        type: 1,
        symbol: 'C',
        x,
        y,
        z,
        fx: 0, fy: 0, fz: 0,
      })
      id++
    }
  }

  // Add cap atoms (hemisphere) if enabled
  if (cntConfig.cap_end) {
    const numCapRings = Math.max(2, Math.round(numAtomsCircum / 4))
    for (let ring = 1; ring <= numCapRings; ring++) {
      const ringAtoms = Math.max(1, Math.round(numAtomsCircum * (1 - ring / (numCapRings + 1))))
      const ringR = R * Math.cos(Math.PI / 2 * ring / (numCapRings + 1))
      const ringZ = R * Math.sin(Math.PI / 2 * ring / (numCapRings + 1))
      for (let i = 0; i < ringAtoms; i++) {
        const theta = (2 * Math.PI * i) / ringAtoms + (ring % 2 === 0 ? Math.PI / ringAtoms : 0)
        atoms.push({
          id,
          type: 1,
          symbol: 'C',
          x: ringR * Math.cos(theta),
          y: ringR * Math.sin(theta),
          z: ringZ,
          fx: 0, fy: 0, fz: 0,
        })
        id++
      }
    }
    // Top cap
    for (let ring = 1; ring <= numCapRings; ring++) {
      const ringAtoms = Math.max(1, Math.round(numAtomsCircum * (1 - ring / (numCapRings + 1))))
      const ringR = R * Math.cos(Math.PI / 2 * ring / (numCapRings + 1))
      const topZ = (numAtomsLength - 1) * (aCC * Math.sqrt(3) / 2) + R * Math.sin(Math.PI / 2 * ring / (numCapRings + 1))
      for (let i = 0; i < ringAtoms; i++) {
        const theta = (2 * Math.PI * i) / ringAtoms + (ring % 2 === 0 ? Math.PI / ringAtoms : 0)
        atoms.push({
          id,
          type: 1,
          symbol: 'C',
          x: ringR * Math.cos(theta),
          y: ringR * Math.sin(theta),
          z: topZ,
          fx: 0, fy: 0, fz: 0,
        })
        id++
      }
    }
  }

  // Apply vacancy defects
  if (cntConfig.vacancy_defect_density > 0) {
    const numVacancies = Math.floor(atoms.length * cntConfig.vacancy_defect_density)
    const rng = seededRandom(42)
    for (let v = 0; v < numVacancies && atoms.length > 10; v++) {
      const idx = Math.floor(rng() * atoms.length)
      atoms.splice(idx, 1)
    }
  }

  return atoms
}

/**
 * Simple seeded pseudo-random number generator
 */
function seededRandom(seed: number): () => number {
  let s = seed
  return () => {
    s = (s * 1103515245 + 12345) & 0x7fffffff
    return s / 0x7fffffff
  }
}

/**
 * Greatest common divisor
 */
function gcd(a: number, b: number): number {
  a = Math.abs(Math.round(a))
  b = Math.abs(Math.round(b))
  while (b) {
    const t = b
    b = a % b
    a = t
  }
  return a || 1
}

/**
 * Generate amorphous structure (random placement with minimum distance)
 */
function generateAmorphousAtoms(config: typeof amorphousConfig): MdAtom[] {
  const rng = seededRandom(config.random_seed)
  const atoms: MdAtom[] = []
  const minDist = 1.5 // minimum interatomic distance in A
  const boxSize = config.box_size_A
  const numAtoms = config.num_atoms

  let id = 0
  let attempts = 0
  const maxAttempts = numAtoms * 100

  while (atoms.length < numAtoms && attempts < maxAttempts) {
    const x = rng() * boxSize
    const y = rng() * boxSize
    const z = rng() * boxSize

    // Check minimum distance
    let tooClose = false
    for (const atom of atoms) {
      const dx = x - atom.x
      const dy = y - atom.y
      const dz = z - atom.z
      if (dx * dx + dy * dy + dz * dz < minDist * minDist) {
        tooClose = true
        break
      }
    }

    if (!tooClose) {
      atoms.push({
        id,
        type: 1,
        symbol: config.element.symbol,
        x, y, z,
        fx: 0, fy: 0, fz: 0,
      })
      id++
    }
    attempts++
  }

  return atoms
}

/**
 * Generate interface structure (two layers of different crystals)
 */
function generateInterfaceAtoms(config: typeof interfaceConfig): MdAtom[] {
  const aBot = config.element_bottom.lattice_constant_A
  const aTop = config.element_top.lattice_constant_A
  const atoms: MdAtom[] = []
  let id = 0

  // Generate bottom layer (FCC for simplicity)
  const basis = [[0, 0, 0], [0.5, 0.5, 0], [0.5, 0, 0.5], [0, 0.5, 0.5]]
  const nx = 6
  const ny = 6

  for (let iz = 0; iz < config.num_layers_bottom; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        for (const [bx, by, bz] of basis) {
          atoms.push({
            id,
            type: 1,
            symbol: config.element_bottom.symbol,
            x: (ix + bx) * aBot,
            y: (iy + by) * aBot,
            z: (iz + bz) * aBot,
            fx: 0, fy: 0, fz: 0,
          })
          id++
        }
      }
    }
  }

  // Interface offset
  const interfaceZ = config.num_layers_bottom * aBot + config.vacuum_A

  // Generate top layer
  for (let iz = 0; iz < config.num_layers_top; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        for (const [bx, by, bz] of basis) {
          atoms.push({
            id,
            type: 2,
            symbol: config.element_top.symbol,
            x: (ix + bx) * aTop,
            y: (iy + by) * aTop,
            z: interfaceZ + (iz + bz) * aTop,
            fx: 0, fy: 0, fz: 0,
          })
          id++
        }
      }
    }
  }

  return atoms
}

/**
 * Generate defect structure (base supercell with defects)
 */
function generateDefectAtoms(config: typeof defectConfig): MdAtom[] {
  const bc = config.base_config
  let atoms: MdAtom[] = []

  // Generate base supercell
  switch (bc.crystal_structure) {
    case 'FCC':
      atoms = generateFCCAtoms(bc.lattice_constant_A, bc.nx, bc.ny, bc.nz)
      break
    case 'BCC':
      atoms = generateBCCAtoms(bc.lattice_constant_A, bc.nx, bc.ny, bc.nz)
      break
    case 'HCP':
      atoms = generateHCPAtoms(bc.lattice_constant_A, bc.nx, bc.ny, bc.nz)
      break
    case 'diamond':
      atoms = generateDiamondAtoms(bc.lattice_constant_A, bc.nx, bc.ny, bc.nz)
      break
    case 'SC':
      atoms = generateSCAtoms(bc.lattice_constant_A, bc.nx, bc.ny, bc.nz)
      break
    default:
      atoms = generateFCCAtoms(bc.lattice_constant_A, bc.nx, bc.ny, bc.nz)
  }

  const sym = bc.element.symbol
  for (const atom of atoms) {
    atom.symbol = sym
  }

  // Apply defects
  const rng = seededRandom(12345)
  const numDefects = Math.floor(atoms.length * config.defect_density)

  if (config.defect_type === 'vacancy') {
    for (let d = 0; d < numDefects && atoms.length > 10; d++) {
      const idx = Math.floor(rng() * atoms.length)
      atoms.splice(idx, 1)
    }
  } else if (config.defect_type === 'interstitial') {
    const lx = bc.nx * bc.lattice_constant_A
    const ly = bc.ny * bc.lattice_constant_A
    const lz = bc.nz * bc.lattice_constant_A
    for (let d = 0; d < numDefects; d++) {
      atoms.push({
        id: atoms.length,
        type: 2,
        symbol: sym,
        x: rng() * lx,
        y: rng() * ly,
        z: rng() * lz,
        fx: 0, fy: 0, fz: 0,
      })
    }
  } else if (config.defect_type === 'substitution') {
    for (let d = 0; d < numDefects; d++) {
      const idx = Math.floor(rng() * atoms.length)
      atoms[idx].type = 2
      atoms[idx].symbol = 'X'
    }
  } else if (config.defect_type === 'dislocation_edge') {
    // Insert extra half-plane along y-axis at x = lx/2
    const lx = bc.nx * bc.lattice_constant_A
    const ly = bc.ny * bc.lattice_constant_A
    const lz = bc.nz * bc.lattice_constant_A
    const numExtra = Math.floor(ly / bc.lattice_constant_A) * Math.floor(lz / bc.lattice_constant_A)
    for (let d = 0; d < numExtra; d++) {
      const iy = Math.floor(d / Math.floor(lz / bc.lattice_constant_A))
      const iz = d % Math.floor(lz / bc.lattice_constant_A)
      atoms.push({
        id: atoms.length,
        type: 2,
        symbol: sym,
        x: lx / 2 + bc.lattice_constant_A * 0.3,
        y: iy * bc.lattice_constant_A,
        z: iz * bc.lattice_constant_A,
        fx: 0, fy: 0, fz: 0,
      })
    }
  } else if (config.defect_type === 'dislocation_screw') {
    // Twist atoms around z-axis
    const cx = bc.nx * bc.lattice_constant_A / 2
    const cy = bc.ny * bc.lattice_constant_A / 2
    const Burgers = bc.lattice_constant_A
    for (const atom of atoms) {
      const dx = atom.x - cx
      const dy = atom.y - cy
      const r = Math.sqrt(dx * dx + dy * dy)
      if (r > 0.1) {
        const theta = Math.atan2(dy, dx)
        const twist = Burgers / (2 * Math.PI * r) * 0.3
        atom.x = cx + r * Math.cos(theta + twist)
        atom.y = cy + r * Math.sin(theta + twist)
      }
    }
  } else if (config.defect_type === 'grain_boundary') {
    // Tilt boundary: rotate upper half around y-axis
    const lz = bc.nz * bc.lattice_constant_A
    const tiltAngle = 5 * Math.PI / 180 // 5 degree tilt
    const cy = bc.ny * bc.lattice_constant_A / 2
    for (const atom of atoms) {
      if (atom.z > lz / 2) {
        const dy = atom.y - cy
        atom.y = cy + dy * Math.cos(tiltAngle) - (atom.z - lz / 2) * Math.sin(tiltAngle)
        atom.z = lz / 2 + dy * Math.sin(tiltAngle) + (atom.z - lz / 2) * Math.cos(tiltAngle)
      }
    }
  }

  // Re-index
  for (let i = 0; i < atoms.length; i++) {
    atoms[i].id = i
  }

  return atoms
}

/**
 * Generate cluster atoms
 */
function generateClusterAtoms(config: typeof clusterConfig): MdAtom[] {
  const a = config.lattice_constant_A
  const atoms: MdAtom[] = []
  const numAtoms = config.num_atoms
  let id = 0

  if (config.cluster_type === 'icosahedron') {
    // Generate icosahedral shells
    // An icosahedron has 12 vertices, 20 faces, 30 edges
    // Mackay icosahedra: 1, 13, 55, 147, 309, 561, ...
    const magicNumbers = [1, 13, 55, 147, 309, 561, 923, 1415]
    let shell = 0
    for (let i = 0; i < magicNumbers.length; i++) {
      if (magicNumbers[i] <= numAtoms) shell = i
    }

    // Golden ratio
    const phi = (1 + Math.sqrt(5)) / 2
    const vertices12: number[][] = [
      [0, 1, phi], [0, -1, phi], [0, 1, -phi], [0, -1, -phi],
      [1, phi, 0], [-1, phi, 0], [1, -phi, 0], [-1, -phi, 0],
      [phi, 0, 1], [-phi, 0, 1], [phi, 0, -1], [-phi, 0, -1],
    ]
    // Normalize
    const norm = Math.sqrt(1 + phi * phi)
    for (const v of vertices12) {
      v[0] /= norm
      v[1] /= norm
      v[2] /= norm
    }

    // Center atom
    atoms.push({ id: id++, type: 1, symbol: config.element.symbol, x: 0, y: 0, z: 0, fx: 0, fy: 0, fz: 0 })

    // Generate shells
    for (let s = 1; s <= shell; s++) {
      const R = s * a * 0.5
      // Place atoms on concentric icosahedral shells
      const numPerShell = Math.min(magicNumbers[s] - magicNumbers[s - 1], numAtoms - atoms.length)
      for (let i = 0; i < numPerShell; i++) {
        // Distribute on sphere using golden spiral
        const theta = Math.acos(1 - 2 * (i + 0.5) / numPerShell)
        const phiAngle = 2 * Math.PI * i / ((1 + Math.sqrt(5)) / 2)
        atoms.push({
          id: id++,
          type: 1,
          symbol: config.element.symbol,
          x: R * Math.sin(theta) * Math.cos(phiAngle),
          y: R * Math.sin(theta) * Math.sin(phiAngle),
          z: R * Math.cos(theta),
          fx: 0, fy: 0, fz: 0,
        })
      }
    }
  } else if (config.cluster_type === 'decahedron') {
    // Decahedron: 5-fold symmetry
    const R = a * Math.pow(numAtoms / 10, 1 / 3) * 0.8
    const numRings = Math.max(1, Math.round(R / (a * 0.5)))
    atoms.push({ id: id++, type: 1, symbol: config.element.symbol, x: 0, y: 0, z: 0, fx: 0, fy: 0, fz: 0 })
    for (let ring = 1; ring <= numRings; ring++) {
      const ringR = (ring / numRings) * R
      const atomsPerRing = Math.max(5, Math.round(5 * ring))
      for (let i = 0; i < atomsPerRing && atoms.length < numAtoms; i++) {
        const theta = (2 * Math.PI * i) / atomsPerRing
        atoms.push({
          id: id++,
          type: 1,
          symbol: config.element.symbol,
          x: ringR * Math.cos(theta),
          y: ringR * Math.sin(theta),
          z: ringR * 0.3 * Math.sin(5 * theta),
          fx: 0, fy: 0, fz: 0,
        })
      }
    }
  } else if (config.cluster_type === 'fcc_cube') {
    // FCC cube cluster
    const n = Math.max(1, Math.round(Math.pow(numAtoms / 4, 1 / 3)))
    const basis = [[0, 0, 0], [0.5, 0.5, 0], [0.5, 0, 0.5], [0, 0.5, 0.5]]
    const center = n * a / 2
    for (let iz = 0; iz < n; iz++) {
      for (let iy = 0; iy < n; iy++) {
        for (let ix = 0; ix < n; ix++) {
          for (const [bx, by, bz] of basis) {
            if (atoms.length >= numAtoms) break
            atoms.push({
              id: id++,
              type: 1,
              symbol: config.element.symbol,
              x: (ix + bx) * a - center,
              y: (iy + by) * a - center,
              z: (iz + bz) * a - center,
              fx: 0, fy: 0, fz: 0,
            })
          }
        }
      }
    }
  } else if (config.cluster_type === 'bcc_cube') {
    // BCC cube cluster
    const n = Math.max(1, Math.round(Math.pow(numAtoms / 2, 1 / 3)))
    const basis = [[0, 0, 0], [0.5, 0.5, 0.5]]
    const center = n * a / 2
    for (let iz = 0; iz < n; iz++) {
      for (let iy = 0; iy < n; iy++) {
        for (let ix = 0; ix < n; ix++) {
          for (const [bx, by, bz] of basis) {
            if (atoms.length >= numAtoms) break
            atoms.push({
              id: id++,
              type: 1,
              symbol: config.element.symbol,
              x: (ix + bx) * a - center,
              y: (iy + by) * a - center,
              z: (iz + bz) * a - center,
              fx: 0, fy: 0, fz: 0,
            })
          }
        }
      }
    }
  } else {
    // Sphere cluster
    const R = a * Math.pow(3 * numAtoms / (4 * Math.PI), 1 / 3) * 0.6
    const rng = seededRandom(777)
    let attempts = 0
    while (atoms.length < numAtoms && attempts < numAtoms * 50) {
      const x = (rng() - 0.5) * 2 * R
      const y = (rng() - 0.5) * 2 * R
      const z = (rng() - 0.5) * 2 * R
      if (x * x + y * y + z * z <= R * R) {
        atoms.push({
          id: id++,
          type: 1,
          symbol: config.element.symbol,
          x, y, z,
          fx: 0, fy: 0, fz: 0,
        })
      }
      attempts++
    }
  }

  return atoms
}

/**
 * Build the full result object from generated atoms
 */
function buildResult(atoms: MdAtom[], element: Element, box: { lx: number; ly: number; lz: number }): AtomBuilderResult {
  // Set symbols if empty
  for (const atom of atoms) {
    if (!atom.symbol) atom.symbol = element.symbol
  }

  // Count unique types
  const typeSet = new Set(atoms.map(a => a.type))
  const numTypes = typeSet.size

  // Calculate volume and density
  const volume = box.lx * box.ly * box.lz
  const totalMass = atoms.reduce((sum, a) => {
    const el = elementLibrary.find(e => e.symbol === a.symbol)
    return sum + (el?.mass_amu ?? element.mass_amu)
  }, 0)

  // density in g/cm3: (mass_amu * 1.66054e-24 g) / (volume_A3 * 1e-24 cm3)
  const density = (totalMass * 1.66054) / volume

  // Generate bonds (nearest neighbors within cutoff)
  const cutoff = element.covalent_radius_A * 2.5
  const bonds: Array<{ i: number; j: number }> = []
  const colors: Record<string, string> = {}
  for (const atom of atoms) {
    if (!colors[atom.symbol]) {
      const el = elementLibrary.find(e => e.symbol === atom.symbol)
      colors[atom.symbol] = el?.color ?? '#888888'
    }
  }

  // Only compute bonds for small systems to avoid O(n^2) slowdown
  if (atoms.length <= 5000) {
    for (let i = 0; i < atoms.length; i++) {
      for (let j = i + 1; j < atoms.length; j++) {
        const dx = atoms[i].x - atoms[j].x
        const dy = atoms[i].y - atoms[j].y
        const dz = atoms[i].z - atoms[j].z
        const dist = Math.sqrt(dx * dx + dy * dy + dz * dz)
        if (dist < cutoff && dist > 0.5) {
          bonds.push({ i, j })
        }
      }
    }
  }

  return {
    success: true,
    atoms,
    box,
    num_atoms: atoms.length,
    num_types: numTypes,
    density,
    volume_A3: volume,
    mass_amu: totalMass,
    visualization_data: { bonds, colors },
  }
}

// ============ Main Generate ============

async function generateStructure() {
  generating.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 800))

    let atoms: MdAtom[] = []
    let element: Element
    let box: { lx: number; ly: number; lz: number }

    switch (activeTab.value) {
      case 'supercell': {
        const cfg = supercellConfig
        element = cfg.element
        const a = cfg.lattice_constant_A
        switch (cfg.crystal_structure) {
          case 'FCC':
            atoms = generateFCCAtoms(a, cfg.nx, cfg.ny, cfg.nz)
            break
          case 'BCC':
            atoms = generateBCCAtoms(a, cfg.nx, cfg.ny, cfg.nz)
            break
          case 'HCP':
            atoms = generateHCPAtoms(a, cfg.nx, cfg.ny, cfg.nz)
            break
          case 'diamond':
            atoms = generateDiamondAtoms(a, cfg.nx, cfg.ny, cfg.nz)
            break
          case 'SC':
            atoms = generateSCAtoms(a, cfg.nx, cfg.ny, cfg.nz)
            break
          default:
            atoms = generateFCCAtoms(a, cfg.nx, cfg.ny, cfg.nz)
        }
        const lx = cfg.nx * a + cfg.vacuum_A
        const ly = cfg.ny * a
        const lz = cfg.nz * a
        if (cfg.crystal_structure === 'HCP') {
          const c = a * Math.sqrt(8 / 3)
          box = { lx, ly: cfg.ny * a * Math.sqrt(3) / 2, lz: cfg.nz * c + cfg.vacuum_A }
        } else {
          box = { lx, ly, lz: cfg.nz * a + cfg.vacuum_A }
        }
        break
      }
      case 'amorphous': {
        element = amorphousConfig.element
        atoms = generateAmorphousAtoms(amorphousConfig)
        const bs = amorphousConfig.box_size_A
        box = { lx: bs, ly: bs, lz: bs }
        break
      }
      case 'interface': {
        element = interfaceConfig.element_bottom
        atoms = generateInterfaceAtoms(interfaceConfig)
        const aBot = interfaceConfig.element_bottom.lattice_constant_A
        const aTop = interfaceConfig.element_top.lattice_constant_A
        const maxA = Math.max(aBot, aTop)
        box = {
          lx: 6 * maxA,
          ly: 6 * maxA,
          lz: (interfaceConfig.num_layers_bottom * aBot) + interfaceConfig.vacuum_A + (interfaceConfig.num_layers_top * aTop),
        }
        break
      }
      case 'defect': {
        element = defectConfig.base_config.element
        atoms = generateDefectAtoms(defectConfig)
        const bc = defectConfig.base_config
        box = {
          lx: bc.nx * bc.lattice_constant_A,
          ly: bc.ny * bc.lattice_constant_A,
          lz: bc.nz * bc.lattice_constant_A,
        }
        break
      }
      case 'cnt': {
        element = cntConfig.element
        atoms = generateCNTAtoms(cntConfig.chirality_n, cntConfig.chirality_m, cntConfig.length_A)
        const R = cntDiameter.value / 2
        box = {
          lx: cntDiameter.value + 4,
          ly: cntDiameter.value + 4,
          lz: cntConfig.length_A + (cntConfig.cap_end ? cntDiameter.value : 0) + 4,
        }
        break
      }
      case 'cluster': {
        element = clusterConfig.element
        atoms = generateClusterAtoms(clusterConfig)
        const maxR = atoms.reduce((max, a) => {
          return Math.max(max, Math.sqrt(a.x * a.x + a.y * a.y + a.z * a.z))
        }, 0)
        const d = maxR * 2 + 4
        box = { lx: d, ly: d, lz: d }
        break
      }
      default:
        return
    }

    result.value = buildResult(atoms, element, box)
    await nextTick()
    drawPreview()
  } catch (e) {
    console.error('Structure generation failed:', e)
  } finally {
    generating.value = false
  }
}

// ============ 3D Canvas Rendering ============

function drawPreview() {
  if (!previewCanvas.value || !result.value) return

  const canvas = previewCanvas.value
  const rect = canvas.parentElement?.getBoundingClientRect()
  if (!rect) return

  canvas.width = rect.width * window.devicePixelRatio
  canvas.height = rect.height * window.devicePixelRatio
  canvas.style.width = rect.width + 'px'
  canvas.style.height = rect.height + 'px'

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const scale = window.devicePixelRatio
  ctx.scale(scale, scale)

  const w = rect.width
  const h = rect.height

  ctx.clearRect(0, 0, w, h)
  ctx.fillStyle = '#0A0B0E'
  ctx.fillRect(0, 0, w, h)

  const atoms = result.value.atoms
  if (atoms.length === 0) return

  // Compute center
  let cx = 0, cy = 0, cz = 0
  for (const atom of atoms) {
    cx += atom.x
    cy += atom.y
    cz += atom.z
  }
  cx /= atoms.length
  cy /= atoms.length
  cz /= atoms.length

  // Compute max extent for scaling
  let maxExtent = 0
  for (const atom of atoms) {
    const dx = atom.x - cx
    const dy = atom.y - cy
    const dz = atom.z - cz
    maxExtent = Math.max(maxExtent, Math.sqrt(dx * dx + dy * dy + dz * dz))
  }
  maxExtent = Math.max(maxExtent, 1)

  const viewScale = Math.min(w, h) * 0.35 / maxExtent * zoom.value

  // Rotation matrix
  const cosX = Math.cos(rotX.value)
  const sinX = Math.sin(rotX.value)
  const cosY = Math.cos(rotY.value)
  const sinY = Math.sin(rotY.value)

  function project(x: number, y: number, z: number): { px: number; py: number; depth: number } {
    const dx = x - cx
    const dy = y - cy
    const dz = z - cz

    // Rotate around Y
    const rx = dx * cosY + dz * sinY
    const rz = -dx * sinY + dz * cosY
    const ry = dy

    // Rotate around X
    const ry2 = ry * cosX - rz * sinX
    const rz2 = ry * sinX + rz * cosX

    return {
      px: w / 2 + rx * viewScale,
      py: h / 2 - ry2 * viewScale,
      depth: rz2,
    }
  }

  // Project all atoms
  const projected = atoms.map(atom => {
    const p = project(atom.x, atom.y, atom.z)
    return { ...atom, ...p }
  })

  // Sort by depth (painter's algorithm)
  projected.sort((a, b) => a.depth - b.depth)

  const colors = result.value.visualization_data.colors
  const atomRadius = Math.max(2, Math.min(8, 200 / Math.sqrt(atoms.length)))

  // Draw bonds first (behind atoms)
  if (result.value.visualization_data.bonds.length > 0 && atoms.length <= 3000) {
    ctx.globalAlpha = 0.3
    for (const bond of result.value.visualization_data.bonds) {
      if (bond.i >= atoms.length || bond.j >= atoms.length) continue
      const a1 = projected.find(p => p.id === bond.i)
      const a2 = projected.find(p => p.id === bond.j)
      if (!a1 || !a2) continue
      const avgDepth = (a1.depth + a2.depth) / 2
      const brightness = 0.2 + 0.5 * (1 - (avgDepth + maxExtent) / (2 * maxExtent))
      ctx.strokeStyle = `rgba(150, 180, 220, ${brightness})`
      ctx.lineWidth = 0.5
      ctx.beginPath()
      ctx.moveTo(a1.px, a1.py)
      ctx.lineTo(a2.px, a2.py)
      ctx.stroke()
    }
    ctx.globalAlpha = 1.0
  }

  // Draw atoms
  for (const atom of projected) {
    const depthNorm = (atom.depth + maxExtent) / (2 * maxExtent)
    const brightness = 0.4 + 0.6 * (1 - depthNorm)
    const color = colors[atom.symbol] ?? '#888888'

    // Parse color and apply brightness
    const r = parseInt(color.slice(1, 3), 16)
    const g = parseInt(color.slice(3, 5), 16)
    const b = parseInt(color.slice(5, 7), 16)

    const br = Math.round(r * brightness)
    const bg = Math.round(g * brightness)
    const bb = Math.round(b * brightness)

    // Gradient for 3D sphere effect
    const gradient = ctx.createRadialGradient(
      atom.px - atomRadius * 0.3,
      atom.py - atomRadius * 0.3,
      atomRadius * 0.1,
      atom.px,
      atom.py,
      atomRadius
    )
    gradient.addColorStop(0, `rgba(${Math.min(255, br + 80)},${Math.min(255, bg + 80)},${Math.min(255, bb + 80)},1)`)
    gradient.addColorStop(0.7, `rgba(${br},${bg},${bb},1)`)
    gradient.addColorStop(1, `rgba(${Math.round(br * 0.4)},${Math.round(bg * 0.4)},${Math.round(bb * 0.4)},1)`)

    ctx.fillStyle = gradient
    ctx.beginPath()
    ctx.arc(atom.px, atom.py, atomRadius, 0, Math.PI * 2)
    ctx.fill()
  }

  // Draw box wireframe
  const box = result.value.box
  const boxCenter = { x: cx, y: cy, z: cz }
  const boxCorners = [
    { x: 0, y: 0, z: 0 },
    { x: box.lx, y: 0, z: 0 },
    { x: box.lx, y: box.ly, z: 0 },
    { x: 0, y: box.ly, z: 0 },
    { x: 0, y: 0, z: box.lz },
    { x: box.lx, y: 0, z: box.lz },
    { x: box.lx, y: box.ly, z: box.lz },
    { x: 0, y: box.ly, z: box.lz },
  ]

  const boxEdges = [
    [0, 1], [1, 2], [2, 3], [3, 0],
    [4, 5], [5, 6], [6, 7], [7, 4],
    [0, 4], [1, 5], [2, 6], [3, 7],
  ]

  ctx.strokeStyle = 'rgba(100, 140, 200, 0.2)'
  ctx.lineWidth = 0.5
  for (const [i, j] of boxEdges) {
    const c1 = boxCorners[i]
    const c2 = boxCorners[j]
    const p1 = project(c1.x, c1.y, c1.z)
    const p2 = project(c2.x, c2.y, c2.z)
    ctx.beginPath()
    ctx.moveTo(p1.px, p1.py)
    ctx.lineTo(p2.px, p2.py)
    ctx.stroke()
  }

  // Title
  ctx.fillStyle = '#E8E9EB'
  ctx.font = '12px sans-serif'
  ctx.textAlign = 'center'
  const tabLabel = tabs.find(t => t.key === activeTab.value)?.label ?? ''
  ctx.fillText(`${tabLabel} 结构 - ${atoms.length} 原子`, w / 2, 20)

  // Axis indicator
  const axLen = 40
  const axOrigin = { x: 50, y: h - 50 }
  const axEndX = project(cx + axLen, cy, cz)
  const axEndY = project(cx, cy + axLen, cz)
  const axEndZ = project(cx, cy, cz + axLen)

  function normArrow(target: { px: number; py: number }, origin: { x: number; y: number }, len: number): { ex: number; ey: number } {
    const dx = target.px - origin.x
    const dy = target.py - origin.y
    const mag = Math.sqrt(dx * dx + dy * dy)
    if (mag < 1e-10) return { ex: origin.x, ey: origin.y }
    return { ex: origin.x + dx / mag * len, ey: origin.y + dy / mag * len }
  }

  const axes = [
    { end: axEndX, color: '#ef4444', label: 'X' },
    { end: axEndY, color: '#22c55e', label: 'Y' },
    { end: axEndZ, color: '#4F8CFF', label: 'Z' },
  ]

  for (const axis of axes) {
    const { ex, ey } = normArrow(axis.end, axOrigin, axLen)
    ctx.strokeStyle = axis.color
    ctx.lineWidth = 2
    ctx.beginPath()
    ctx.moveTo(axOrigin.x, axOrigin.y)
    ctx.lineTo(ex, ey)
    ctx.stroke()

    ctx.fillStyle = axis.color
    ctx.font = '10px sans-serif'
    ctx.textAlign = 'center'
    ctx.fillText(axis.label, ex + (ex - axOrigin.x) * 0.3, ey + (ey - axOrigin.y) * 0.3)
  }
}

// ============ Canvas Interaction ============

function onCanvasMouseDown(e: MouseEvent) {
  isDragging = true
  lastMouseX = e.clientX
  lastMouseY = e.clientY
}

function onCanvasMouseMove(e: MouseEvent) {
  if (!isDragging) return
  const dx = e.clientX - lastMouseX
  const dy = e.clientY - lastMouseY
  rotY.value += dx * 0.005
  rotX.value += dy * 0.005
  lastMouseX = e.clientX
  lastMouseY = e.clientY
  drawPreview()
}

function onCanvasMouseUp() {
  isDragging = false
}

function onCanvasWheel(e: WheelEvent) {
  const delta = e.deltaY > 0 ? 0.9 : 1.1
  zoom.value = Math.max(0.1, Math.min(10, zoom.value * delta))
  drawPreview()
}

function exportAtoms() {
  if (!result.value) return
  const data = result.value.atoms.map(a => `${a.id} ${a.type} ${a.symbol} ${a.x.toFixed(6)} ${a.y.toFixed(6)} ${a.z.toFixed(6)}`).join('\n')
  const blob = new Blob([data], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `atoms_${activeTab.value}.xyz`
  a.click()
  URL.revokeObjectURL(url)
}

// ============ Lifecycle ============

onMounted(() => {
  onSupercellElementChange()
  onAmorphousElementChange()
  onInterfaceBottomChange()
  onInterfaceTopChange()
  onDefectElementChange()
  onClusterElementChange()
})

onUnmounted(() => {
  if (animFrameId) cancelAnimationFrame(animFrameId)
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
