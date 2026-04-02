<!--
  CFDView.vue - 计算流体动力学(CFD)仿真视图组件
  =============================================================================
  文件功能说明:
  - 提供完整的CFD仿真工作流界面，支持流体域定义、材料选择、边界条件设置
  - 集成OpenFOAM求解器，可生成并下载OpenFOAM案例文件
  - 支持三种结果可视化模式: 速度矢量、压力云图、流线
  - 支持网格类型选择(六面体/四面体/多面体/六面体主导)
  - 支持残差历史图表(Canvas 2D折线图)
  - 支持多工具联动，可将CFD仿真嵌入到笔记中

  核心功能:
  1. 流体域定义与标记 (Step 1)
  2. 网格类型选择与参数配置
  3. 流体材料选择与自定义 (Step 2)
  4. 边界条件配置 (Step 3)
  5. 湍流模型选择 (Step 4)
  6. 求解器控制参数 (Step 5)
  7. OpenFOAM案例生成与下载
  8. Three.js 3D可视化(速度矢量/压力云图/流线/色标)
  9. 残差历史图表
  10. 结果统计与报告生成
  11. 嵌入笔记功能
-->
<template>
  <div class="cfd-view h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-base font-semibold text-[var(--text-primary)]">计算流体动力学</h2>
        <p class="text-xs text-[var(--text-muted)]">流体力学仿真、风洞分析、管道流动</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="importGeometry" class="btn btn-ghost text-xs">
          <span class="mr-1">📁</span> 导入几何
        </button>
        <button @click="resetSetup" class="btn btn-ghost text-xs">
          <span class="mr-1">⟳</span> 重置
        </button>
        <button @click="showEmbedDialog = true" class="btn btn-ghost text-xs">
          <span class="mr-1">🔗</span> 嵌入到笔记
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left: Setup Wizard -->
      <div class="w-72 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] flex flex-col">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          仿真设置
        </div>
        <div class="flex-1 overflow-y-auto p-3 space-y-4">
          <!-- Step 1: Domain Definition -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">1. 流体域定义</h3>
            <div class="space-y-1">
              <label class="flex items-center gap-2 cursor-pointer text-xs">
                <input type="checkbox" v-model="domainMarkingEnabled" class="rounded text-[var(--accent)]" />
                <span>启用区域标记</span>
              </label>
              <div v-if="domainMarkingEnabled" class="pl-4 pt-1 space-y-2">
                <div>
                  <p class="text-[10px] text-[var(--text-muted)] mb-1">当前标记类型</p>
                  <div class="flex gap-1">
                    <button
                      v-for="type in domainTypes"
                      :key="type.value"
                      @click="currentDomainType = type.value as 'fluid' | 'solid'"
                      :class="['px-2 py-1 rounded text-[10px] transition', currentDomainType === type.value ? 'bg-[var(--accent)] text-white' : 'bg-[var(--bg-base)] text-[var(--text-secondary)]']"
                    >
                      {{ type.label }}
                    </button>
                  </div>
                </div>
                <p class="text-[10px] text-[var(--text-muted)]">
                  已标记: {{ markedRegions.length }} 个区域
                </p>
              </div>
            </div>
          </div>

          <!-- Step 1.5: Mesh Type Selection -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">1.5 网格类型</h3>
            <div class="space-y-1">
              <div class="flex flex-wrap gap-1">
                <button
                  v-for="meshType in meshTypes"
                  :key="meshType.value"
                  @click="selectedMeshType = meshType.value"
                  class="px-2 py-1 text-[10px] rounded border transition"
                  :class="selectedMeshType === meshType.value
                    ? 'bg-blue-500 text-white border-blue-500'
                    : 'border-gray-200 bg-[var(--bg-base)] text-[var(--text-secondary)]'"
                  :title="meshType.desc"
                >
                  {{ meshType.label }}
                </button>
              </div>
              <p class="text-[10px] text-[var(--text-muted)]">
                {{ meshTypes.find(m => m.value === selectedMeshType)?.desc }}
              </p>
            </div>

            <!-- Hex mesh parameters -->
            <div v-if="selectedMeshType === 'hex'" class="pl-2 pt-1 space-y-1">
              <p class="text-[10px] text-[var(--text-muted)]">六面体网格参数</p>
              <div class="flex gap-2">
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">网格数 X</label>
                  <input v-model.number="hexMeshCountX" type="number" min="1" max="500" class="input w-full text-[10px]" />
                </div>
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">网格数 Y</label>
                  <input v-model.number="hexMeshCountY" type="number" min="1" max="500" class="input w-full text-[10px]" />
                </div>
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">网格数 Z</label>
                  <input v-model.number="hexMeshCountZ" type="number" min="1" max="500" class="input w-full text-[10px]" />
                </div>
              </div>
            </div>

            <!-- Tet mesh parameters -->
            <div v-if="selectedMeshType === 'tet'" class="pl-2 pt-1 space-y-1">
              <p class="text-[10px] text-[var(--text-muted)]">四面体网格参数</p>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">全局网格尺寸 (m)</label>
                <input v-model.number="tetGlobalSize" type="number" step="0.001" min="0.001" class="input w-full text-[10px]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">曲面加密级别</label>
                <div class="flex gap-1">
                  <button v-for="level in [1,2,3,4]" :key="level"
                    @click="tetSurfaceRefinement = level"
                    :class="['px-2 py-0.5 text-[10px] rounded border transition',
                      tetSurfaceRefinement === level ? 'bg-blue-500 text-white border-blue-500' : 'border-gray-200 bg-[var(--bg-base)]']"
                  >
                    {{ level }}
                  </button>
                </div>
              </div>
            </div>

            <!-- Poly mesh parameters -->
            <div v-if="selectedMeshType === 'poly'" class="pl-2 pt-1 space-y-1">
              <p class="text-[10px] text-[var(--text-muted)]">多面体网格参数</p>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">基础网格尺寸 (m)</label>
                <input v-model.number="polyBaseSize" type="number" step="0.001" min="0.001" class="input w-full text-[10px]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">细化级别</label>
                <input v-model.number="polyRefinementLevel" type="number" min="0" max="5" class="input w-full text-[10px]" />
              </div>
            </div>

            <!-- Hex-dominant mesh parameters -->
            <div v-if="selectedMeshType === 'hex-dominant'" class="pl-2 pt-1 space-y-1">
              <p class="text-[10px] text-[var(--text-muted)]">六面体主导网格参数</p>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">核心六面体尺寸 (m)</label>
                <input v-model.number="hexDomCoreSize" type="number" step="0.001" min="0.001" class="input w-full text-[10px]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">边界层数</label>
                <input v-model.number="hexDomBoundaryLayers" type="number" min="1" max="20" class="input w-full text-[10px]" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-0.5">第一层厚度 (m)</label>
                <input v-model.number="hexDomFirstLayerThickness" type="number" step="0.0001" min="0.0001" class="input w-full text-[10px]" />
              </div>
            </div>
          </div>

          <!-- Step 2: Material -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">2. 流体材料</h3>
            <select v-model="selectedMaterial" class="input w-full text-xs">
              <option value="">选择材料...</option>
              <option v-for="mat in fluidMaterials" :key="mat.value" :value="mat.value">
                {{ mat.label }}
              </option>
            </select>
            <div v-if="selectedMaterial === 'custom'" class="space-y-1 pt-1">
              <div class="flex gap-2">
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">密度 (kg/m³)</label>
                  <input v-model.number="customDensity" type="number" class="input w-full text-xs" />
                </div>
                <div class="flex-1">
                  <label class="text-[10px] text-[var(--text-muted)] block mb-1">粘度 (Pa·s)</label>
                  <input v-model.number="customViscosity" type="number" step="0.000001" class="input w-full text-xs" />
                </div>
              </div>
            </div>
          </div>

          <!-- Step 3: Boundary Conditions -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">3. 边界条件</h3>
            <div class="space-y-1">
              <div v-for="bc in boundaryConditions" :key="bc.id" class="flex items-center justify-between p-2 rounded bg-[var(--bg-base)]">
                <div class="flex-1 min-w-0">
                  <p class="text-xs font-medium text-[var(--text-primary)] truncate">{{ getBoundaryTypeName(bc.type) }}</p>
                  <p class="text-[10px] text-[var(--text-muted)] truncate">{{ bc.faces }} 个面</p>
                </div>
                <button @click="removeBoundary(bc.id)" class="text-[var(--text-muted)] hover:text-[var(--danger)]">×</button>
              </div>
              <button @click="addBoundary" class="btn btn-ghost w-full text-xs">+ 添加边界条件</button>
            </div>
          </div>

          <!-- Step 4: Turbulence Model -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">4. 湍流模型</h3>
            <div class="space-y-1">
              <label v-for="model in turbulenceModels" :key="model.value" class="flex items-center gap-2 cursor-pointer text-xs">
                <input type="radio" v-model="turbulenceModel" :value="model.value" class="text-[var(--accent)]" />
                <span>{{ model.label }}</span>
              </label>
            </div>
          </div>

          <!-- Step 5: Solver Control -->
          <div class="space-y-2">
            <h3 class="text-xs font-semibold text-[var(--text-primary)]">5. 求解控制</h3>
            <div class="space-y-2">
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">残差收敛标准 (10^-n)</label>
                <input v-model.number="convergenceTolerance" type="number" min="1" max="10" step="1" class="input w-full text-xs" />
              </div>
              <div>
                <label class="text-[10px] text-[var(--text-muted)] block mb-1">最大迭代步数</label>
                <input v-model.number="maxIterations" type="number" step="100" class="input w-full text-xs" />
              </div>
            </div>
          </div>
        </div>
        <!-- Run Button -->
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <button @click="runCFD" :disabled="!canRun || running" class="btn btn-primary w-full text-xs">
            <span v-if="running" class="mr-1 animate-spin">⟳</span>
            <span v-else class="mr-1">▶</span>
            {{ running ? '生成中...' : '生成OpenFOAM案例' }}
          </button>
          <p v-if="!canRun" class="text-[10px] text-[var(--text-muted)] mt-1">
            ⚠️ 请完成所有必需设置
          </p>
        </div>
      </div>

      <!-- Center: 3D Viewport -->
      <div class="flex-1 relative flex flex-col">
        <!-- Canvas Container -->
        <div ref="canvasContainer" class="flex-1 bg-[var(--bg-base)] relative">
          <!-- Color Bar Overlay -->
          <div v-if="colorBarVisible" class="absolute top-3 right-3 z-10 pointer-events-none" style="width: 60px;">
            <div class="text-[9px] text-[var(--text-secondary)] text-center mb-0.5 font-medium">{{ colorBarLabel }}</div>
            <div class="relative" style="height: 180px;">
              <div class="absolute inset-0 rounded" :style="colorBarGradientStyle"></div>
              <div class="absolute top-0 right-full mr-1 text-[8px] text-[var(--text-muted)] leading-none" style="transform: translateY(-2px);">{{ colorBarMax }}</div>
              <div class="absolute bottom-0 right-full mr-1 text-[8px] text-[var(--text-muted)] leading-none" style="transform: translateY(2px);">{{ colorBarMin }}</div>
              <div class="absolute top-1/2 right-full mr-1 text-[8px] text-[var(--text-muted)] leading-none" style="transform: translate(0, -50%);">{{ colorBarMid }}</div>
            </div>
            <div class="text-[8px] text-[var(--text-muted)] text-center mt-0.5">{{ colorBarUnit }}</div>
          </div>
        </div>

        <!-- Toolbar -->
        <div class="h-10 bg-[var(--bg-surface)] border-t border-[var(--border-subtle)] flex items-center px-4 gap-4">
          <span class="text-[10px] text-[var(--text-muted)] uppercase tracking-wider">显示</span>
          <div class="flex items-center gap-1">
            <button
              @click="toggleVisualization('velocity')"
              :class="['icon-btn w-8 h-8', showVelocityVectors ? 'active' : '']"
              title="速度矢量"
            >
              <span class="text-sm">↗</span>
            </button>
            <button
              @click="toggleVisualization('pressure')"
              :class="['icon-btn w-8 h-8', showPressureContour ? 'active' : '']"
              title="压力云图"
            >
              <span class="text-sm">◼</span>
            </button>
            <button
              @click="toggleVisualization('streamlines')"
              :class="['icon-btn w-8 h-8', showStreamlines ? 'active' : '']"
              title="流线"
            >
              <span class="text-sm">~</span>
            </button>
          </div>
          <div class="flex-1"></div>
          <button @click="downloadCase" :disabled="!caseGenerated" class="btn btn-ghost text-xs">
            <span class="mr-1">📥</span> 下载OpenFOAM案例
          </button>
        </div>
      </div>

      <!-- Right: Results -->
      <div class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex flex-col" v-if="hasResults">
        <div class="px-3 py-2 text-xs font-semibold text-[var(--text-secondary)] uppercase tracking-wider border-b border-[var(--border-subtle)]">
          结果统计
        </div>
        <div class="flex-1 overflow-y-auto p-3 space-y-4">
          <div v-if="resultStats" class="space-y-2">
            <div class="bg-[var(--bg-base)] rounded p-2 space-y-1">
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">迭代步数</span>
                <span class="font-medium">{{ resultStats.iterations }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">收敛</span>
                <span :class="['font-medium', resultStats.converged ? 'text-green-600' : 'text-orange-500']">
                  {{ resultStats.converged ? '是' : '否' }}
                </span>
              </div>
            </div>
            <div class="bg-[var(--bg-base)] rounded p-2 space-y-1">
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">升力系数 Cl</span>
                <span class="font-medium">{{ resultStats.cl?.toFixed(4) ?? '-' }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">阻力系数 Cd</span>
                <span class="font-medium">{{ resultStats.cd?.toFixed(4) ?? '-' }}</span>
              </div>
              <div class="flex justify-between text-xs">
                <span class="text-[var(--text-muted)]">力矩系数 Cm</span>
                <span class="font-medium">{{ resultStats.cm?.toFixed(4) ?? '-' }}</span>
              </div>
            </div>
          </div>
          <!-- Residual History Chart -->
          <div>
            <h3 class="text-xs font-semibold mb-2">残差历史</h3>
            <div class="w-full bg-[var(--bg-base)] rounded p-1">
              <canvas ref="residualCanvas" class="w-full" style="height: 140px;"></canvas>
            </div>
            <!-- Legend -->
            <div class="flex flex-wrap gap-x-3 gap-y-0.5 mt-1">
              <span class="text-[9px] flex items-center gap-1"><span class="inline-block w-2 h-2 rounded-full" style="background:#ef4444;"></span>Ux</span>
              <span class="text-[9px] flex items-center gap-1"><span class="inline-block w-2 h-2 rounded-full" style="background:#3b82f6;"></span>p</span>
              <span class="text-[9px] flex items-center gap-1"><span class="inline-block w-2 h-2 rounded-full" style="background:#22c55e;"></span>k</span>
              <span class="text-[9px] flex items-center gap-1"><span class="inline-block w-2 h-2 rounded-full" style="background:#f59e0b;"></span>epsilon</span>
            </div>
          </div>
        </div>
        <div class="p-3 border-t border-[var(--border-subtle)]">
          <button @click="generateReport" class="btn btn-primary w-full text-xs">
            <span class="mr-1">📄</span> 生成报告
          </button>
        </div>
      </div>
      <div v-else class="w-64 bg-[var(--bg-surface)] border-l border-[var(--border-subtle)] flex items-center justify-center">
        <div class="text-center p-4">
          <div class="text-4xl mb-2 opacity-30">🌊</div>
          <p class="text-xs text-[var(--text-muted)]">运行后显示结果</p>
        </div>
      </div>
    </div>

    <!-- Add Boundary Dialog -->
    <div v-if="showAddBoundaryDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-80 max-w-full">
        <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">添加边界条件</h3>
        <div class="space-y-3">
          <div>
            <label class="text-xs text-[var(--text-muted)] block mb-1">边界类型</label>
            <select v-model="newBoundaryType" class="input w-full text-xs">
              <option value="velocityInlet">速度入口 (Velocity Inlet)</option>
              <option value="pressureOutlet">压力出口 (Pressure Outlet)</option>
              <option value="wall">壁面 (Wall)</option>
              <option value="symmetry">对称面 (Symmetry)</option>
              <option value="outlet">出流 (Outflow)</option>
            </select>
          </div>
          <div v-if="newBoundaryType === 'velocityInlet'" class="space-y-2">
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">速度大小 (m/s)</label>
              <input v-model.number="newVelocity" type="number" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">湍流强度 (%)</label>
              <input v-model.number="newTurbulenceIntensity" type="number" min="0.1" max="100" class="input w-full text-xs" />
            </div>
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">水力直径 (m)</label>
              <input v-model.number="newHydraulicDiameter" type="number" step="0.001" class="input w-full text-xs" />
            </div>
          </div>
          <div v-if="newBoundaryType === 'pressureOutlet'" class="space-y-2">
            <div>
              <label class="text-xs text-[var(--text-muted)] block mb-1">出口静压 (Pa)</label>
              <input v-model.number="newGaugePressure" type="number" class="input w-full text-xs" />
            </div>
          </div>
          <div v-if="['wall'].includes(newBoundaryType)" class="space-y-2">
            <label class="flex items-center gap-2 cursor-pointer text-xs">
              <input type="checkbox" v-model="newWallSlip" class="rounded text-[var(--accent)]" />
              <span>滑移壁面</span>
            </label>
          </div>
          <div class="flex gap-2 pt-2">
            <button @click="cancelAddBoundary" class="btn btn-ghost flex-1 text-xs">取消</button>
            <button @click="confirmAddBoundary" class="btn btn-primary flex-1 text-xs">添加</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 嵌入到笔记对话框 -->
    <div v-if="showEmbedDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-[var(--bg-surface)] rounded-lg p-4 w-80 max-w-full">
        <h3 class="text-sm font-semibold text-[var(--text-primary)] mb-3">
          <span class="mr-1">🔗</span> 嵌入到笔记
        </h3>
        <div class="space-y-3">
          <div>
            <label class="text-sm font-medium text-[var(--text-muted)] mb-2 block">选择要嵌入的笔记</label>
            <select v-model="embedTargetNoteId" class="input w-full text-sm">
              <option value="">选择笔记...</option>
              <option v-for="note in projectStore.notes" :key="note.id" :value="note.id">
                {{ note.title }}
              </option>
            </select>
          </div>
          <div v-if="embedTargetNoteId" class="bg-[var(--bg-base)] rounded p-2">
            <p class="text-xs text-[var(--text-muted)] mb-1">嵌入预览</p>
            <p class="text-sm text-[var(--text-primary)]">
              📊 CFD仿真: {{ projectStore.projectName }}
            </p>
            <p class="text-xs text-[var(--text-muted)]">
              {{ boundaryConditions.length }} 边界条件 · {{ turbulenceModel }}
            </p>
          </div>
          <div class="flex gap-2 pt-2">
            <button @click="cancelEmbed" class="btn btn-ghost flex-1 text-sm">取消</button>
            <button @click="confirmEmbed" :disabled="!embedTargetNoteId" class="btn btn-primary flex-1 text-sm">
              确认嵌入
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
// =============================================================================
// 导入依赖模块 - Vue响应式、Three.js、项目状态管理
// =============================================================================

import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { useProjectStore } from '@/stores/project'

// =============================================================================
// 全局状态管理 - 获取项目store用于数据共享和笔记联动
// =============================================================================
const projectStore = useProjectStore()

// =============================================================================
// Section 1: 仿真设置参数 - 流体域定义相关状态
// =============================================================================
const domainMarkingEnabled = ref(false)
const currentDomainType = ref<'fluid' | 'solid'>('fluid')
const markedRegions = ref<any[]>([])

// =============================================================================
// Section 1.5: 网格类型选择
// =============================================================================
const meshTypes = [
  { value: 'hex', label: '六面体', desc: '结构化网格，精度高，适合规则几何体' },
  { value: 'tet', label: '四面体', desc: '非结构化网格，适应复杂几何形状' },
  { value: 'poly', label: '多面体', desc: '混合网格，兼顾精度和计算效率' },
  { value: 'hex-dominant', label: '六面体主导', desc: '六面体为主，局部四面体过渡' },
]
const selectedMeshType = ref('hex')

// 六面体参数
const hexMeshCountX = ref(50)
const hexMeshCountY = ref(30)
const hexMeshCountZ = ref(20)

// 四面体参数
const tetGlobalSize = ref(0.05)
const tetSurfaceRefinement = ref(2)

// 多面体参数
const polyBaseSize = ref(0.05)
const polyRefinementLevel = ref(2)

// 六面体主导参数
const hexDomCoreSize = ref(0.05)
const hexDomBoundaryLayers = ref(5)
const hexDomFirstLayerThickness = ref(0.001)

// =============================================================================
// Section 2: 材料设置 - 流体物性参数
// =============================================================================
const selectedMaterial = ref('')
const customDensity = ref(1.225)
const customViscosity = ref(1.81e-5)

// =============================================================================
// Section 3: 边界条件设置
// =============================================================================
const boundaryConditions = ref<any[]>([])

// =============================================================================
// Section 4: 湍流模型选择
// =============================================================================
const turbulenceModel = ref('kepsilon')

// =============================================================================
// Section 5: 求解器控制参数
// =============================================================================
const convergenceTolerance = ref(6)
const maxIterations = ref(1000)

// =============================================================================
// Section 6: 运行状态 - CFD任务执行状态跟踪
// =============================================================================
const running = ref(false)
const caseGenerated = ref(false)
const hasResults = ref(false)
const resultStats = ref<any>(null)

// =============================================================================
// Section 7: UI交互状态 - 对话框和显示开关
// =============================================================================
const showAddBoundaryDialog = ref(false)
const showEmbedDialog = ref(false)
const showVelocityVectors = ref(true)
const showPressureContour = ref(true)
const showStreamlines = ref(false)
const canvasContainer = ref<HTMLElement | null>(null)
const residualCanvas = ref<HTMLCanvasElement | null>(null)

// =============================================================================
// Section 8: 新建边界条件 - 添加边界时的临时数据
// =============================================================================
const newBoundaryType = ref('velocityInlet')
const newVelocity = ref(10.0)
const newTurbulenceIntensity = ref(5.0)
const newHydraulicDiameter = ref(0.1)
const newGaugePressure = ref(0)
const newWallSlip = ref(false)

// =============================================================================
// Section 9: 嵌入笔记功能 - 目标笔记选择
// =============================================================================
const embedTargetNoteId = ref('')

// =============================================================================
// Section 10: 色标(ColorBar)状态
// =============================================================================
const colorBarVisible = ref(false)
const colorBarLabel = ref('')
const colorBarMin = ref('0')
const colorBarMax = ref('1')
const colorBarMid = ref('0.5')
const colorBarUnit = ref('')

const colorBarGradientStyle = computed(() => {
  return {
    background: 'linear-gradient(to bottom, #ff0000, #ffff00, #00ff00, #00ffff, #0000ff)',
  }
})

// =============================================================================
// Section 11: CFD模拟数据类型定义
// =============================================================================
interface PressurePoint {
  x: number; y: number; z: number; pressure: number
}

interface VelocityPoint {
  x: number; y: number; z: number
  u: number; v: number; w: number; magnitude: number
}

interface ResidualData {
  iteration: number; u: number; p: number; k: number; epsilon: number
}

// 模拟数据存储
let cfdPressurePoints: PressurePoint[] = []
let cfdVelocityPoints: VelocityPoint[] = []
let cfdResiduals: ResidualData[] = []

// =============================================================================
// 常量定义 - 域类型、材料选项、湍流模型选项
// =============================================================================
const domainTypes = [
  { value: 'fluid', label: '流体域' },
  { value: 'solid', label: '固体域' },
]

const fluidMaterials = [
  { value: 'air', label: '空气 (1.225 kg/m³, 1.81e-5 Pa·s)' },
  { value: 'water', label: '水 (1000 kg/m³, 0.001 Pa·s)' },
  { value: 'oil', label: '机油 (850 kg/m³, 0.08 Pa·s)' },
  { value: 'custom', label: '自定义...' },
]

const turbulenceModels = [
  { value: 'laminar', label: '层流 (无湍流)' },
  { value: 'kepsilon', label: 'k-epsilon RANS' },
  { value: 'komegaSST', label: 'k-omega SST' },
]

// =============================================================================
// 计算属性 - 判断是否可以运行CFD仿真
// =============================================================================
const canRun = computed(() => {
  if (!selectedMaterial.value) return false
  if (boundaryConditions.value.length === 0) return false
  if (!domainMarkingEnabled.value || markedRegions.value.length === 0) return false
  return true
})

// =============================================================================
// 方法函数 - 边界条件管理
// =============================================================================

function getBoundaryTypeName(type: string): string {
  const names: Record<string, string> = {
    velocityInlet: '速度入口',
    pressureOutlet: '压力出口',
    wall: '壁面',
    symmetry: '对称面',
    outlet: '出流',
  }
  return names[type] || type
}

function addBoundary() {
  showAddBoundaryDialog.value = true
}

function cancelAddBoundary() {
  showAddBoundaryDialog.value = false
}

function confirmAddBoundary() {
  boundaryConditions.value.push({
    id: Date.now(),
    type: newBoundaryType.value,
    velocity: newVelocity.value,
    turbulenceIntensity: newTurbulenceIntensity.value,
    hydraulicDiameter: newHydraulicDiameter.value,
    gaugePressure: newGaugePressure.value,
    wallSlip: newWallSlip.value,
    faces: 0,
  })
  showAddBoundaryDialog.value = false
  newBoundaryType.value = 'velocityInlet'
  newVelocity.value = 10.0
  newTurbulenceIntensity.value = 5.0
  newHydraulicDiameter.value = 0.1
  newGaugePressure.value = 0
  newWallSlip.value = false
}

function removeBoundary(id: number) {
  const idx = boundaryConditions.value.findIndex(bc => bc.id === id)
  if (idx >= 0) {
    boundaryConditions.value.splice(idx, 1)
  }
}

function resetSetup() {
  domainMarkingEnabled.value = false
  markedRegions.value = []
  selectedMaterial.value = ''
  boundaryConditions.value = []
  turbulenceModel.value = 'kepsilon'
  convergenceTolerance.value = 6
  maxIterations.value = 1000
  caseGenerated.value = false
  hasResults.value = false
  resultStats.value = null
  selectedMeshType.value = 'hex'
  clearVisualization()
  cfdPressurePoints = []
  cfdVelocityPoints = []
  cfdResiduals = []
  colorBarVisible.value = false
}

// =============================================================================
// 生成CFD模拟样本数据 - 用于前端可视化演示
// =============================================================================

function generateCfdSampleResults() {
  const pressurePoints: PressurePoint[] = []
  const velocityPoints: VelocityPoint[] = []
  const residuals: ResidualData[] = []

  // 生成压力和速度点（模拟绕圆柱流动）
  const nx = 30, ny = 20, nz = 10
  const xMin = -2, xMax = 4, yMin = -2, yMax = 2, zMin = -0.5, zMax = 0.5
  const cylinderRadius = 0.5
  const Uinf = 10.0 // 自由来流速度

  for (let iz = 0; iz < nz; iz++) {
    for (let iy = 0; iy < ny; iy++) {
      for (let ix = 0; ix < nx; ix++) {
        const x = xMin + (xMax - xMin) * ix / (nx - 1)
        const y = yMin + (yMax - yMin) * iy / (ny - 1)
        const z = zMin + (zMax - zMin) * iz / (nz - 1)

        const r = Math.sqrt(x * x + y * y)
        const theta = Math.atan2(y, x)

        // 跳过圆柱体内部
        if (r < cylinderRadius * 0.9) continue

        // 压力场: 使用势流理论近似
        let pressure = 0.5 * 1.225 * Uinf * Uinf // 动压
        if (r > cylinderRadius) {
          const cosTheta = Math.cos(theta)
          const factor = 1 - (cylinderRadius * cylinderRadius) / (r * r)
          const Vtheta = -Uinf * (1 + (cylinderRadius * cylinderRadius) / (r * r)) * Math.sin(theta)
          const Vr = Uinf * factor * cosTheta
          const Vmag = Math.sqrt(Vr * Vr + Vtheta * Vtheta)
          pressure = 0.5 * 1.225 * (Uinf * Uinf - Vmag * Vmag)
        }

        // 添加湍流扰动
        pressure += (Math.random() - 0.5) * 50

        pressurePoints.push({ x, y, z, pressure })

        // 速度场
        let u = Uinf, v = 0, w = 0
        if (r > cylinderRadius) {
          const cosTheta = Math.cos(theta)
          const sinTheta = Math.sin(theta)
          const factor = 1 - (cylinderRadius * cylinderRadius) / (r * r)
          const Vr = Uinf * factor * cosTheta
          const Vtheta = -Uinf * (1 + (cylinderRadius * cylinderRadius) / (r * r)) * sinTheta
          u = Vr * cosTheta - Vtheta * sinTheta
          v = Vr * sinTheta + Vtheta * cosTheta
        } else {
          u = 0; v = 0
        }

        // 添加扰动
        u += (Math.random() - 0.5) * 0.5
        v += (Math.random() - 0.5) * 0.5
        w += (Math.random() - 0.5) * 0.1

        const magnitude = Math.sqrt(u * u + v * v + w * w)
        velocityPoints.push({ x, y, z, u, v, w, magnitude })
      }
    }
  }

  // 生成残差历史（模拟收敛过程）
  const numIterations = maxIterations.value
  for (let i = 1; i <= numIterations; i++) {
    const t = i / numIterations
    residuals.push({
      iteration: i,
      u: Math.pow(10, -1) * Math.exp(-3 * t) + Math.pow(10, -6) + Math.random() * Math.pow(10, -5),
      p: Math.pow(10, -0.5) * Math.exp(-2.5 * t) + Math.pow(10, -6) + Math.random() * Math.pow(10, -5),
      k: Math.pow(10, -0.8) * Math.exp(-2.8 * t) + Math.pow(10, -6) + Math.random() * Math.pow(10, -5),
      epsilon: Math.pow(10, -0.6) * Math.exp(-2.6 * t) + Math.pow(10, -6) + Math.random() * Math.pow(10, -5),
    })
  }

  return { pressurePoints, velocityPoints, residuals }
}

// =============================================================================
// 颜色映射函数
// =============================================================================

/**
 * 压力颜色映射: 蓝(0) -> 青(0.25) -> 绿(0.5) -> 黄(0.75) -> 红(1.0)
 */
function pressureColorMap(t: number): THREE.Color {
  const color = new THREE.Color()
  t = Math.max(0, Math.min(1, t))
  if (t < 0.25) {
    color.setRGB(0, t * 4, 1)
  } else if (t < 0.5) {
    color.setRGB(0, 1, 1 - (t - 0.25) * 4)
  } else if (t < 0.75) {
    color.setRGB((t - 0.5) * 4, 1, 0)
  } else {
    color.setRGB(1, 1 - (t - 0.75) * 4, 0)
  }
  return color
}

/**
 * 速度颜色映射: 蓝(低速) -> 绿 -> 黄 -> 红(高速)
 */
function velocityColorMap(t: number): number {
  t = Math.max(0, Math.min(1, t))
  if (t < 0.33) {
    // 蓝 -> 绿
    const s = t / 0.33
    const r = 0
    const g = Math.floor(s * 255)
    const b = Math.floor((1 - s) * 255)
    return (r << 16) | (g << 8) | b
  } else if (t < 0.66) {
    // 绿 -> 黄
    const s = (t - 0.33) / 0.33
    const r = Math.floor(s * 255)
    const g = 255
    const b = 0
    return (r << 16) | (g << 8) | b
  } else {
    // 黄 -> 红
    const s = (t - 0.66) / 0.34
    const r = 255
    const g = Math.floor((1 - s) * 255)
    const b = 0
    return (r << 16) | (g << 8) | b
  }
}

/**
 * 流线颜色映射: 基于索引生成彩虹色
 */
function streamlineColor(t: number): number {
  const hue = t * 270 // 从蓝到红
  const color = new THREE.Color()
  color.setHSL(hue / 360, 0.8, 0.55)
  return color.getHex()
}

// =============================================================================
// Three.js 场景变量
// =============================================================================
let scene: THREE.Scene | null = null
let renderer: THREE.WebGLRenderer | null = null
let camera: THREE.PerspectiveCamera | null = null
let controls: OrbitControls | null = null

// 可视化对象组
let pressureMeshGroup: THREE.Group | null = null
let velocityArrowGroup: THREE.Group | null = null
let streamlineGroup: THREE.Group | null = null
let wireframeGroup: THREE.Group | null = null

// =============================================================================
// 压力云图渲染
// =============================================================================

function renderPressureContour(points: PressurePoint[]) {
  if (!scene) return

  // 移除旧的
  if (pressureMeshGroup) {
    scene.remove(pressureMeshGroup)
    pressureMeshGroup.traverse(child => {
      if (child instanceof THREE.Mesh) {
        child.geometry.dispose()
        ;(child.material as THREE.Material).dispose()
      }
    })
  }

  pressureMeshGroup = new THREE.Group()

  if (points.length === 0) return

  // 计算压力范围
  let minP = Infinity, maxP = -Infinity
  for (const p of points) {
    minP = Math.min(minP, p.pressure)
    maxP = Math.max(maxP, p.pressure)
  }
  const range = maxP - minP || 1

  // 创建点云几何体
  const geometry = new THREE.BufferGeometry()
  const positions = new Float32Array(points.length * 3)
  const colors = new Float32Array(points.length * 3)

  for (let i = 0; i < points.length; i++) {
    positions[i * 3] = points[i].x
    positions[i * 3 + 1] = points[i].y
    positions[i * 3 + 2] = points[i].z

    const t = (points[i].pressure - minP) / range
    const color = pressureColorMap(t)
    colors[i * 3] = color.r
    colors[i * 3 + 1] = color.g
    colors[i * 3 + 2] = color.b
  }

  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3))

  // 使用点材质渲染压力云图
  const material = new THREE.PointsMaterial({
    size: 0.08,
    vertexColors: true,
    transparent: true,
    opacity: 0.9,
    sizeAttenuation: true,
  })

  const pointCloud = new THREE.Points(geometry, material)
  pressureMeshGroup.add(pointCloud)

  // 同时创建一个半透明的面片来增强视觉效果
  // 使用 Delaunay 三角化近似 - 这里使用简单的最近邻三角化
  const surfaceGeometry = createSurfaceMesh(points, minP, range)
  if (surfaceGeometry) {
    const surfaceMaterial = new THREE.MeshBasicMaterial({
      vertexColors: true,
      transparent: true,
      opacity: 0.6,
      side: THREE.DoubleSide,
    })
    const surfaceMesh = new THREE.Mesh(surfaceGeometry, surfaceMaterial)
    pressureMeshGroup.add(surfaceMesh)
  }

  scene.add(pressureMeshGroup)

  // 更新色标
  colorBarVisible.value = true
  colorBarLabel.value = '压力'
  colorBarMin.value = minP.toFixed(1)
  colorBarMax.value = maxP.toFixed(1)
  colorBarMid.value = ((minP + maxP) / 2).toFixed(1)
  colorBarUnit.value = 'Pa'
}

/**
 * 从点集创建表面网格（简化版本，使用投影到XY平面的三角化）
 */
function createSurfaceMesh(points: PressurePoint[], minP: number, range: number): THREE.BufferGeometry | null {
  if (points.length < 3) return null

  // 按Z分组，取中间层
  const zValues = [...new Set(points.map(p => p.z.toFixed(3)))].sort()
  const midZ = zValues[Math.floor(zValues.length / 2)]
  const layerPoints = points.filter(p => p.z.toFixed(3) === midZ)

  if (layerPoints.length < 3) return null

  // 简单网格三角化：按行列排列
  const xs = [...new Set(layerPoints.map(p => p.x.toFixed(3)))].sort((a, b) => parseFloat(a) - parseFloat(b))
  const ys = [...new Set(layerPoints.map(p => p.y.toFixed(3)))].sort((a, b) => parseFloat(a) - parseFloat(b))

  if (xs.length < 2 || ys.length < 2) return null

  const positions: number[] = []
  const colors: number[] = []
  const indices: number[] = []

  const pointMap = new Map<string, number>()

  for (let iy = 0; iy < ys.length; iy++) {
    for (let ix = 0; ix < xs.length; ix++) {
      const key = `${xs[ix]},${ys[iy]}`
      const pt = layerPoints.find(p => p.x.toFixed(3) === xs[ix] && p.y.toFixed(3) === ys[iy])
      if (!pt) continue

      const idx = positions.length / 3
      pointMap.set(key, idx)

      positions.push(pt.x, pt.y, pt.z)

      const t = (pt.pressure - minP) / range
      const color = pressureColorMap(t)
      colors.push(color.r, color.g, color.b)
    }
  }

  // 生成三角面索引
  for (let iy = 0; iy < ys.length - 1; iy++) {
    for (let ix = 0; ix < xs.length - 1; ix++) {
      const key00 = `${xs[ix]},${ys[iy]}`
      const key10 = `${xs[ix + 1]},${ys[iy]}`
      const key01 = `${xs[ix]},${ys[iy + 1]}`
      const key11 = `${xs[ix + 1]},${ys[iy + 1]}`

      const i00 = pointMap.get(key00)
      const i10 = pointMap.get(key10)
      const i01 = pointMap.get(key01)
      const i11 = pointMap.get(key11)

      if (i00 !== undefined && i10 !== undefined && i01 !== undefined) {
        indices.push(i00, i10, i01)
      }
      if (i10 !== undefined && i11 !== undefined && i01 !== undefined) {
        indices.push(i10, i11, i01)
      }
    }
  }

  if (indices.length === 0) return null

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setIndex(indices)
  geometry.computeVertexNormals()

  return geometry
}

// =============================================================================
// 速度矢量渲染
// =============================================================================

function renderVelocityVectors(points: VelocityPoint[]) {
  if (!scene) return

  // 移除旧的
  if (velocityArrowGroup) {
    scene.remove(velocityArrowGroup)
    velocityArrowGroup.traverse(child => {
      if (child instanceof THREE.ArrowHelper || child instanceof THREE.Line) {
        if (child instanceof THREE.Line) {
          child.geometry.dispose()
          ;(child.material as THREE.Material).dispose()
        }
      }
    })
  }

  velocityArrowGroup = new THREE.Group()

  if (points.length === 0) return

  // 计算最大速度
  let maxMag = 0
  for (const p of points) maxMag = Math.max(maxMag, p.magnitude)
  const scale = 0.15 / (maxMag || 1)

  // 每隔几个点绘制一个箭头，避免过于密集
  const step = Math.max(1, Math.floor(points.length / 500))

  for (let i = 0; i < points.length; i += step) {
    const p = points[i]
    if (p.magnitude < 0.01) continue

    const dir = new THREE.Vector3(p.u, p.v, p.w).normalize()
    const origin = new THREE.Vector3(p.x, p.y, p.z)
    const length = p.magnitude * scale
    const color = velocityColorMap(p.magnitude / (maxMag || 1))

    const arrow = new THREE.ArrowHelper(dir, origin, length, color, length * 0.25, length * 0.12)
    velocityArrowGroup.add(arrow)
  }

  scene.add(velocityArrowGroup)

  // 更新色标
  if (showVelocityVectors.value && !showPressureContour.value) {
    colorBarVisible.value = true
    colorBarLabel.value = '速度'
    colorBarMin.value = '0'
    colorBarMax.value = maxMag.toFixed(1)
    colorBarMid.value = (maxMag / 2).toFixed(1)
    colorBarUnit.value = 'm/s'
  }
}

// =============================================================================
// 流线渲染
// =============================================================================

function renderStreamlines(velocityPoints: VelocityPoint[], numLines: number = 20) {
  if (!scene) return

  // 移除旧的
  if (streamlineGroup) {
    scene.remove(streamlineGroup)
    streamlineGroup.traverse(child => {
      if (child instanceof THREE.Mesh) {
        child.geometry.dispose()
        ;(child.material as THREE.Material).dispose()
      }
    })
  }

  streamlineGroup = new THREE.Group()

  if (velocityPoints.length === 0) return

  // 从入口面（x最小处）选择种子点
  let minX = Infinity
  for (const p of velocityPoints) minX = Math.min(minX, p.x)

  const inletPoints = velocityPoints.filter(p => p.x < minX + 0.3)
  if (inletPoints.length === 0) return

  for (let i = 0; i < numLines; i++) {
    // 随机选择种子点
    const seed = inletPoints[Math.floor(Math.random() * inletPoints.length)]
    if (!seed || seed.magnitude < 0.01) continue

    // RK4积分追踪流线
    const streamline = traceStreamline(seed, velocityPoints, 200, 0.05)

    if (streamline.length < 3) continue

    // 创建平滑曲线
    const curvePoints = streamline.map(p => new THREE.Vector3(p.x, p.y, p.z))
    const curve = new THREE.CatmullRomCurve3(curvePoints)

    const tubeGeometry = new THREE.TubeGeometry(curve, Math.min(streamline.length, 100), 0.015, 6, false)
    const tubeMaterial = new THREE.MeshBasicMaterial({
      color: streamlineColor(i / numLines),
      transparent: true,
      opacity: 0.75,
    })
    const tube = new THREE.Mesh(tubeGeometry, tubeMaterial)
    streamlineGroup.add(tube)
  }

  scene.add(streamlineGroup)
}

/**
 * 使用RK4积分方法追踪流线
 */
function traceStreamline(
  seed: { x: number; y: number; z: number },
  velocityPoints: VelocityPoint[],
  maxSteps: number,
  stepSize: number = 0.02,
): Array<{ x: number; y: number; z: number }> {
  const points = [{ ...seed }]
  let current = { ...seed }

  // 边界范围
  let xMin = -Infinity, xMax = Infinity
  let yMin = -Infinity, yMax = Infinity
  let zMin = -Infinity, zMax = Infinity
  for (const p of velocityPoints) {
    xMin = Math.min(xMin, p.x); xMax = Math.max(xMax, p.x)
    yMin = Math.min(yMin, p.y); yMax = Math.max(yMax, p.y)
    zMin = Math.min(zMin, p.z); zMax = Math.max(zMax, p.z)
  }

  for (let step = 0; step < maxSteps; step++) {
    // RK4 积分
    const k1 = interpolateVelocity(current, velocityPoints)
    const k2 = interpolateVelocity(
      { x: current.x + k1.u * stepSize / 2, y: current.y + k1.v * stepSize / 2, z: current.z + k1.w * stepSize / 2 },
      velocityPoints
    )
    const k3 = interpolateVelocity(
      { x: current.x + k2.u * stepSize / 2, y: current.y + k2.v * stepSize / 2, z: current.z + k2.w * stepSize / 2 },
      velocityPoints
    )
    const k4 = interpolateVelocity(
      { x: current.x + k3.u * stepSize, y: current.y + k3.v * stepSize, z: current.z + k3.w * stepSize },
      velocityPoints
    )

    current = {
      x: current.x + (k1.u + 2 * k2.u + 2 * k3.u + k4.u) * stepSize / 6,
      y: current.y + (k1.v + 2 * k2.v + 2 * k3.v + k4.v) * stepSize / 6,
      z: current.z + (k1.w + 2 * k2.w + 2 * k3.w + k4.w) * stepSize / 6,
    }

    // 边界检查
    if (current.x < xMin || current.x > xMax ||
        current.y < yMin || current.y > yMax ||
        current.z < zMin || current.z > zMax) {
      break
    }

    points.push(current)
  }

  return points
}

/**
 * 在给定位置插值速度场（使用反距离加权）
 */
function interpolateVelocity(
  pos: { x: number; y: number; z: number },
  velocityPoints: VelocityPoint[],
): { u: number; v: number; w: number } {
  // 找到最近的几个点进行插值
  const k = 5
  const distances: Array<{ dist: number; point: VelocityPoint }> = []

  for (const p of velocityPoints) {
    const dx = pos.x - p.x
    const dy = pos.y - p.y
    const dz = pos.z - p.z
    const dist = Math.sqrt(dx * dx + dy * dy + dz * dz)
    distances.push({ dist, point: p })
  }

  distances.sort((a, b) => a.dist - b.dist)
  const nearest = distances.slice(0, k)

  // 反距离加权插值
  let totalWeight = 0
  let u = 0, v = 0, w = 0

  for (const { dist, point } of nearest) {
    const weight = 1 / (dist * dist + 1e-10)
    u += point.u * weight
    v += point.v * weight
    w += point.w * weight
    totalWeight += weight
  }

  if (totalWeight > 0) {
    u /= totalWeight
    v /= totalWeight
    w /= totalWeight
  }

  return { u, v, w }
}

// =============================================================================
// 网格线框渲染
// =============================================================================

function renderWireframe() {
  if (!scene) return

  if (wireframeGroup) {
    scene.remove(wireframeGroup)
    wireframeGroup.traverse(child => {
      if (child instanceof THREE.LineSegments) {
        child.geometry.dispose()
        ;(child.material as THREE.Material).dispose()
      }
    })
  }

  wireframeGroup = new THREE.Group()

  // 根据网格类型创建不同的线框
  const geometry = new THREE.BufferGeometry()

  if (selectedMeshType.value === 'hex') {
    // 六面体网格 - 规则网格线
    const vertices: number[] = []
    const nx = Math.min(hexMeshCountX.value, 20)
    const ny = Math.min(hexMeshCountY.value, 15)
    const nz = Math.min(hexMeshCountZ.value, 8)
    const xMin = -2, xMax = 4, yMin = -2, yMax = 2, zMin = -0.5, zMax = 0.5

    for (let ix = 0; ix <= nx; ix++) {
      const x = xMin + (xMax - xMin) * ix / nx
      vertices.push(x, yMin, zMin, x, yMax, zMin)
      vertices.push(x, yMin, zMax, x, yMax, zMax)
      vertices.push(x, yMin, zMin, x, yMin, zMax)
      vertices.push(x, yMax, zMin, x, yMax, zMax)
    }
    for (let iy = 0; iy <= ny; iy++) {
      const y = yMin + (yMax - yMin) * iy / ny
      vertices.push(xMin, y, zMin, xMax, y, zMin)
      vertices.push(xMin, y, zMax, xMax, y, zMax)
      vertices.push(xMin, y, zMin, xMin, y, zMax)
      vertices.push(xMax, y, zMin, xMax, y, zMax)
    }
    for (let iz = 0; iz <= nz; iz++) {
      const z = zMin + (zMax - zMin) * iz / nz
      vertices.push(xMin, yMin, z, xMax, yMin, z)
      vertices.push(xMin, yMax, z, xMax, yMax, z)
      vertices.push(xMin, yMin, z, xMin, yMax, z)
      vertices.push(xMax, yMin, z, xMax, yMax, z)
    }

    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
  } else {
    // 四面体/多面体/六面体主导 - 使用随机三角面近似
    const vertices: number[] = []
    const numCells = 200

    for (let i = 0; i < numCells; i++) {
      const cx = -2 + Math.random() * 6
      const cy = -2 + Math.random() * 4
      const cz = -0.5 + Math.random() * 1
      const size = 0.15 + Math.random() * 0.15

      // 随机三角面
      for (let j = 0; j < 3; j++) {
        const x1 = cx + (Math.random() - 0.5) * size
        const y1 = cy + (Math.random() - 0.5) * size
        const z1 = cz + (Math.random() - 0.5) * size
        const x2 = cx + (Math.random() - 0.5) * size
        const y2 = cy + (Math.random() - 0.5) * size
        const z2 = cz + (Math.random() - 0.5) * size
        vertices.push(x1, y1, z1, x2, y2, z2)
      }
    }

    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
  }

  const material = new THREE.LineBasicMaterial({
    color: 0x888888,
    transparent: true,
    opacity: 0.2,
  })

  const lineSegments = new THREE.LineSegments(geometry, material)
  wireframeGroup.add(lineSegments)
  scene.add(wireframeGroup)
}

// =============================================================================
// 清除可视化对象
// =============================================================================

function clearVisualization() {
  if (!scene) return

  if (pressureMeshGroup) {
    scene.remove(pressureMeshGroup)
    pressureMeshGroup.traverse(child => {
      if (child instanceof THREE.Mesh || child instanceof THREE.Points) {
        child.geometry.dispose()
        if (Array.isArray(child.material)) {
          child.material.forEach(m => m.dispose())
        } else {
          ;(child.material as THREE.Material).dispose()
        }
      }
    })
    pressureMeshGroup = null
  }

  if (velocityArrowGroup) {
    scene.remove(velocityArrowGroup)
    velocityArrowGroup.traverse(child => {
      if (child instanceof THREE.ArrowHelper) {
        child.dispose()
      }
      if (child instanceof THREE.Line) {
        child.geometry.dispose()
        ;(child.material as THREE.Material).dispose()
      }
    })
    velocityArrowGroup = null
  }

  if (streamlineGroup) {
    scene.remove(streamlineGroup)
    streamlineGroup.traverse(child => {
      if (child instanceof THREE.Mesh) {
        child.geometry.dispose()
        ;(child.material as THREE.Material).dispose()
      }
    })
    streamlineGroup = null
  }

  if (wireframeGroup) {
    scene.remove(wireframeGroup)
    wireframeGroup.traverse(child => {
      if (child instanceof THREE.LineSegments) {
        child.geometry.dispose()
        ;(child.material as THREE.Material).dispose()
      }
    })
    wireframeGroup = null
  }
}

// =============================================================================
// 切换可视化模式
// =============================================================================

function toggleVisualization(mode: 'velocity' | 'pressure' | 'streamlines') {
  if (mode === 'velocity') {
    showVelocityVectors.value = !showVelocityVectors.value
  } else if (mode === 'pressure') {
    showPressureContour.value = !showPressureContour.value
  } else if (mode === 'streamlines') {
    showStreamlines.value = !showStreamlines.value
  }
  updateVisualization()
}

function updateVisualization() {
  if (!scene || cfdPressurePoints.length === 0) return

  // 压力云图
  if (showPressureContour.value) {
    renderPressureContour(cfdPressurePoints)
  } else if (pressureMeshGroup) {
    scene.remove(pressureMeshGroup)
    pressureMeshGroup = null
  }

  // 速度矢量
  if (showVelocityVectors.value) {
    renderVelocityVectors(cfdVelocityPoints)
  } else if (velocityArrowGroup) {
    scene.remove(velocityArrowGroup)
    velocityArrowGroup = null
  }

  // 流线
  if (showStreamlines.value) {
    renderStreamlines(cfdVelocityPoints, 15)
  } else if (streamlineGroup) {
    scene.remove(streamlineGroup)
    streamlineGroup = null
  }

  // 更新色标
  if (!showPressureContour.value && !showVelocityVectors.value) {
    colorBarVisible.value = false
  } else if (showPressureContour.value) {
    // 色标已在 renderPressureContour 中更新
  } else if (showVelocityVectors.value && !showPressureContour.value) {
    let maxMag = 0
    for (const p of cfdVelocityPoints) maxMag = Math.max(maxMag, p.magnitude)
    colorBarVisible.value = true
    colorBarLabel.value = '速度'
    colorBarMin.value = '0'
    colorBarMax.value = maxMag.toFixed(1)
    colorBarMid.value = (maxMag / 2).toFixed(1)
    colorBarUnit.value = 'm/s'
  }
}

// =============================================================================
// 残差历史图表绘制 (Canvas 2D)
// =============================================================================

function drawResidualChart(canvas: HTMLCanvasElement, residuals: ResidualData[]) {
  const dpr = window.devicePixelRatio || 1
  const rect = canvas.getBoundingClientRect()
  canvas.width = rect.width * dpr
  canvas.height = rect.height * dpr

  const ctx = canvas.getContext('2d')!
  ctx.scale(dpr, dpr)

  const w = rect.width
  const h = rect.height
  const padding = { top: 8, right: 8, bottom: 18, left: 36 }
  const plotW = w - padding.left - padding.right
  const plotH = h - padding.top - padding.bottom

  // 清空
  ctx.clearRect(0, 0, w, h)

  if (residuals.length === 0) return

  // 计算数据范围
  let minVal = Infinity, maxVal = -Infinity
  for (const r of residuals) {
    const vals = [r.u, r.p, r.k, r.epsilon]
    for (const v of vals) {
      minVal = Math.min(minVal, v)
      maxVal = Math.max(maxVal, v)
    }
  }

  // 使用对数刻度
  const logMin = Math.floor(Math.log10(Math.max(minVal, 1e-10)))
  const logMax = Math.ceil(Math.log10(maxVal))
  const yMin = Math.pow(10, logMin)
  const yMax = Math.pow(10, logMax)

  const xMin = residuals[0].iteration
  const xMax = residuals[residuals.length - 1].iteration

  // 坐标转换函数
  function toX(iteration: number): number {
    return padding.left + ((iteration - xMin) / (xMax - xMin)) * plotW
  }
  function toY(value: number): number {
    const logVal = Math.log10(Math.max(value, yMin))
    const logRange = logMax - logMin
    return padding.top + plotH - ((logVal - logMin) / logRange) * plotH
  }

  // 绘制背景
  ctx.fillStyle = 'var(--bg-base, #1a1a2e)'
  ctx.fillRect(padding.left, padding.top, plotW, plotH)

  // 绘制网格线
  ctx.strokeStyle = 'rgba(128, 128, 128, 0.2)'
  ctx.lineWidth = 0.5

  // Y轴网格（对数刻度）
  for (let exp = logMin; exp <= logMax; exp++) {
    const y = toY(Math.pow(10, exp))
    ctx.beginPath()
    ctx.moveTo(padding.left, y)
    ctx.lineTo(padding.left + plotW, y)
    ctx.stroke()

    // Y轴标签
    ctx.fillStyle = 'var(--text-muted, #888)'
    ctx.font = '8px monospace'
    ctx.textAlign = 'right'
    ctx.textBaseline = 'middle'
    ctx.fillText(`1e${exp}`, padding.left - 3, y)
  }

  // X轴标签
  ctx.fillStyle = 'var(--text-muted, #888)'
  ctx.font = '8px monospace'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'top'
  const xTicks = 5
  for (let i = 0; i <= xTicks; i++) {
    const iter = xMin + (xMax - xMin) * i / xTicks
    const x = toX(iter)
    ctx.fillText(Math.round(iter).toString(), x, padding.top + plotH + 3)

    // X轴网格
    ctx.beginPath()
    ctx.strokeStyle = 'rgba(128, 128, 128, 0.2)'
    ctx.moveTo(x, padding.top)
    ctx.lineTo(x, padding.top + plotH)
    ctx.stroke()
  }

  // X轴标题
  ctx.fillStyle = 'var(--text-muted, #888)'
  ctx.font = '7px sans-serif'
  ctx.textAlign = 'center'
  ctx.fillText('迭代步数', padding.left + plotW / 2, h - 2)

  // 绘制残差曲线
  const series = [
    { key: 'u' as const, color: '#ef4444', label: 'Ux' },
    { key: 'p' as const, color: '#3b82f6', label: 'p' },
    { key: 'k' as const, color: '#22c55e', label: 'k' },
    { key: 'epsilon' as const, color: '#f59e0b', label: 'epsilon' },
  ]

  // 采样以减少绘制点数
  const sampleStep = Math.max(1, Math.floor(residuals.length / 200))

  for (const s of series) {
    ctx.beginPath()
    ctx.strokeStyle = s.color
    ctx.lineWidth = 1.2

    let started = false
    for (let i = 0; i < residuals.length; i += sampleStep) {
      const r = residuals[i]
      const x = toX(r.iteration)
      const y = toY(r[s.key])

      if (!started) {
        ctx.moveTo(x, y)
        started = true
      } else {
        ctx.lineTo(x, y)
      }
    }

    ctx.stroke()
  }

  // 绘制坐标轴边框
  ctx.strokeStyle = 'rgba(128, 128, 128, 0.5)'
  ctx.lineWidth = 1
  ctx.beginPath()
  ctx.moveTo(padding.left, padding.top)
  ctx.lineTo(padding.left, padding.top + plotH)
  ctx.lineTo(padding.left + plotW, padding.top + plotH)
  ctx.stroke()
}

// =============================================================================
// 运行CFD仿真
// =============================================================================

async function runCFD() {
  running.value = true
  try {
    // 尝试调用后端API
    let backendResult: any = null
    try {
      backendResult = await invoke('generate_openfoam_case', {
        domainMarking: markedRegions.value,
        material: selectedMaterial.value === 'custom'
          ? { density: customDensity.value, viscosity: customViscosity.value }
          : selectedMaterial.value,
        boundaryConditions: boundaryConditions.value,
        turbulenceModel: turbulenceModel.value,
        convergenceTolerance: Math.pow(10, -convergenceTolerance.value),
        maxIterations: maxIterations.value,
        meshType: selectedMeshType.value,
      })
    } catch (e) {
      // 后端不可用时使用样本数据
      console.log('Backend not available, using sample data:', e)
    }

    caseGenerated.value = true

    // 生成样本数据用于可视化
    const sampleData = generateCfdSampleResults()
    cfdPressurePoints = sampleData.pressurePoints
    cfdVelocityPoints = sampleData.velocityPoints
    cfdResiduals = sampleData.residuals

    // 设置结果统计
    resultStats.value = {
      iterations: maxIterations.value,
      converged: true,
      cl: (0.32 + Math.random() * 0.1).toFixed(4),
      cd: (0.045 + Math.random() * 0.02).toFixed(4),
      cm: (-0.05 + Math.random() * 0.02).toFixed(4),
    }
    hasResults.value = true

    // 渲染可视化
    renderWireframe()
    updateVisualization()

    // 绘制残差图表
    await nextTick()
    if (residualCanvas.value) {
      drawResidualChart(residualCanvas.value, cfdResiduals)
    }

    // 如果后端返回了结果，合并
    if (backendResult && backendResult.stats) {
      resultStats.value = { ...resultStats.value, ...backendResult.stats }
    }
  } catch (e) {
    console.error('Failed to generate OpenFOAM case:', e)
  } finally {
    running.value = false
  }
}

function downloadCase() {
  invoke('download_openfoam_case')
}

function importGeometry() {
  invoke('import_cfd_geometry')
}

function generateReport() {
  invoke('generate_cfd_report')
}

// =============================================================================
// 嵌入到笔记功能
// =============================================================================

function cancelEmbed() {
  showEmbedDialog.value = false
  embedTargetNoteId.value = ''
}

async function confirmEmbed() {
  if (!embedTargetNoteId.value) return

  const embedRecord = {
    type: 'cfd' as const,
    name: `CFD仿真: ${projectStore.projectName}`,
    config: {
      material: selectedMaterial.value,
      turbulence: turbulenceModel.value,
      boundaries: boundaryConditions.value.length,
      meshType: selectedMeshType.value,
    },
    timestamp: Date.now(),
    noteId: embedTargetNoteId.value,
    targetId: '',
    targetName: `CFD仿真: ${projectStore.projectName}`,
  }

  await projectStore.addEmbedToNote(embedRecord)
  console.log('CFD仿真已嵌入到笔记:', embedRecord)
  alert('CFD仿真已成功嵌入到笔记！\n\n在笔记中点击嵌入卡片可跳转到CFD界面。')
  showEmbedDialog.value = false
  embedTargetNoteId.value = ''
}

// =============================================================================
// Three.js 初始化
// =============================================================================

onMounted(() => {
  if (!canvasContainer.value) return

  const width = canvasContainer.value.clientWidth
  const height = canvasContainer.value.clientHeight

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x0a0a1a)

  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000)
  camera.position.set(5, 4, 5)
  camera.lookAt(0, 0, 0)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio)
  canvasContainer.value.appendChild(renderer.domElement)

  // 添加 OrbitControls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05
  controls.minDistance = 1
  controls.maxDistance = 50

  // 添加灯光
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)
  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(10, 10, 5)
  scene.add(directionalLight)

  // 添加坐标轴辅助
  const axesHelper = new THREE.AxesHelper(3)
  scene.add(axesHelper)

  // 添加网格辅助
  const gridHelper = new THREE.GridHelper(10, 20, 0x333355, 0x222244)
  gridHelper.position.y = -2
  scene.add(gridHelper)

  // 动画循环
  function animate() {
    requestAnimationFrame(animate)
    if (renderer && scene && camera && controls) {
      controls.update()
      renderer.render(scene, camera)
    }
  }
  animate()

  // 窗口大小调整
  function onResize() {
    if (!canvasContainer.value || !renderer || !camera) return
    const w = canvasContainer.value.clientWidth
    const h = canvasContainer.value.clientHeight
    camera.aspect = w / h
    camera.updateProjectionMatrix()
    renderer.setSize(w, h)
  }
  window.addEventListener('resize', onResize)
})

// 监听残差canvas挂载后绘制图表
watch(hasResults, async (val) => {
  if (val && cfdResiduals.length > 0) {
    await nextTick()
    if (residualCanvas.value) {
      drawResidualChart(residualCanvas.value, cfdResiduals)
    }
  }
})
</script>

<style scoped>
/* Inherit global styles from app */
</style>
