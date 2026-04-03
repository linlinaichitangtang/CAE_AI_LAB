<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)]">
      <div>
        <h2 class="text-lg font-semibold text-[var(--text-primary)]">AI 参数推荐</h2>
        <p class="text-sm text-[var(--text-muted)]">V1.9-007 基于机器学习的仿真参数智能推荐</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="resetAll" class="btn btn-ghost text-xs">重置</button>
        <button v-if="recommendations.length > 0" @click="exportRecommendations" class="btn btn-ghost text-xs" style="color: var(--accent-green); border-color: var(--accent-green);">
          导出推荐
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel: Configuration (w-80) -->
      <div class="w-80 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] overflow-y-auto p-4 space-y-4">

        <!-- Step 1: 材料体系与目标属性 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">1</span>
            材料体系与目标
          </h4>
          <div class="space-y-2">
            <div>
              <label class="label">材料体系</label>
              <select v-model="context.material_system" class="input w-full text-xs">
                <option value="Mg-Al alloy">Mg-Al 合金</option>
                <option value="Ti-6Al-4V">Ti-6Al-4V</option>
                <option value="Inconel 718">Inconel 718</option>
                <option value="SiC/SiC CMC">SiC/SiC 陶瓷基复合材料</option>
                <option value="CFRP">碳纤维增强复合材料</option>
                <option value="custom">自定义</option>
              </select>
            </div>
            <div>
              <label class="label">目标属性</label>
              <select v-model="context.target_property" class="input w-full text-xs">
                <option value="creep_rate">蠕变速率</option>
                <option value="yield_strength">屈服强度</option>
                <option value="fatigue_life">疲劳寿命</option>
                <option value="thermal_conductivity">热导率</option>
                <option value="fracture_toughness">断裂韧性</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Step 2: 尺度选择 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">2</span>
            仿真尺度
          </h4>
          <div class="flex flex-col gap-1.5">
            <button
              v-for="scale in scaleOptions"
              :key="scale.value"
              @click="context.scale = scale.value"
              :class="['px-3 py-2 rounded text-xs text-left transition border',
                context.scale === scale.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)] hover:bg-[var(--bg-hover)]']"
            >
              <div class="font-medium">{{ scale.label }}</div>
              <div class="text-[10px] mt-0.5 opacity-80">{{ scale.desc }}</div>
            </button>
          </div>
        </div>

        <!-- Step 3: 约束条件 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">3</span>
            约束条件
          </h4>
          <div class="space-y-2">
            <div v-for="(constraint, idx) in constraints" :key="idx" class="flex items-center gap-1.5">
              <input v-model="constraint.name" type="text" class="input w-full text-xs" placeholder="参数名" />
              <span class="text-[var(--text-muted)] text-xs whitespace-nowrap">&le;</span>
              <input v-model.number="constraint.max_value" type="number" class="input text-xs" style="width: 80px;" />
              <button @click="removeConstraint(idx)" class="text-[var(--accent-red)] text-xs px-1">x</button>
            </div>
            <button @click="addConstraint" class="btn btn-ghost text-xs w-full" style="border-style: dashed;">
              + 添加约束
            </button>
          </div>
        </div>

        <!-- Step 4: 获取推荐 -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">4</span>
            获取推荐
          </h4>
          <button @click="getRecommendations" :disabled="loading" class="btn btn-primary text-xs w-full">
            {{ loading ? '推理中...' : '获取推荐' }}
          </button>
          <div v-if="loading" class="mt-2 flex items-center gap-2 text-xs text-[var(--text-muted)]">
            <div class="w-3 h-3 border-2 border-[var(--primary)] border-t-transparent rounded-full animate-spin"></div>
            AI 模型推理中，请稍候...
          </div>
        </div>

        <!-- Step 5: 推荐结果卡片 -->
        <div v-if="recommendations.length > 0" class="panel-section space-y-2">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">5</span>
            推荐参数
          </h4>
          <div v-for="(rec, ri) in recommendations" :key="ri" class="p-2.5 rounded border border-[var(--border-subtle)] bg-[var(--bg-elevated)]">
            <div class="flex items-center justify-between mb-1.5">
              <span class="text-xs font-medium text-[var(--text-primary)]">{{ rec.parameter_name }}</span>
              <span class="font-mono text-xs text-[var(--primary)]">{{ rec.recommended_value }}</span>
            </div>
            <!-- 置信度条 -->
            <div class="mb-1.5">
              <div class="flex items-center justify-between text-[10px] text-[var(--text-muted)] mb-0.5">
                <span>置信度</span>
                <span>{{ (rec.confidence * 100).toFixed(1) }}%</span>
              </div>
              <div class="w-full h-1.5 rounded-full bg-[var(--bg-base)] overflow-hidden">
                <div
                  class="h-full rounded-full transition-all duration-500"
                  :style="{
                    width: (rec.confidence * 100) + '%',
                    background: rec.confidence > 0.8 ? 'var(--accent-green)' : rec.confidence > 0.6 ? 'var(--accent-yellow)' : 'var(--accent-red)'
                  }"
                ></div>
              </div>
            </div>
            <!-- 推荐理由 -->
            <p class="text-[10px] text-[var(--text-muted)] mb-1.5">{{ rec.rationale }}</p>
            <!-- 备选值 -->
            <div v-if="rec.alternatives.length > 0" class="text-[10px]">
              <span class="text-[var(--text-muted)]">备选: </span>
              <span v-for="(alt, ai) in rec.alternatives" :key="ai" class="text-[var(--text-secondary)]">
                {{ alt.value }}<span v-if="ai < rec.alternatives.length - 1">, </span>
              </span>
            </div>
          </div>
        </div>

        <!-- Step 6: 反馈 -->
        <div v-if="recommendations.length > 0" class="panel-section">
          <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">
            <span class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-blue-600 text-white text-xs mr-1">6</span>
            推荐反馈
          </h4>
          <div class="space-y-2">
            <div class="flex gap-1.5">
              <button
                v-for="fb in feedbackOptions"
                :key="fb.value"
                @click="feedback = fb.value"
                :class="['flex-1 px-2 py-1.5 rounded text-xs transition border',
                  feedback === fb.value
                    ? 'bg-blue-600 text-white border-blue-600'
                    : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] border-[var(--border-default)]']"
              >
                {{ fb.label }}
              </button>
            </div>
            <div>
              <label class="label">实际值 (可选)</label>
              <input v-model="actualValue" type="text" class="input w-full text-xs" placeholder="填写实际仿真结果" />
            </div>
            <button @click="submitFeedback" :disabled="!feedback" class="btn btn-ghost text-xs w-full">提交反馈</button>
          </div>
        </div>
      </div>

      <!-- Right Panel: Visualization / Results -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto p-4 space-y-4">

          <!-- 模型性能卡片 -->
          <div class="grid grid-cols-3 gap-3">
            <div v-for="(perf, pi) in modelPerformances" :key="pi" class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-3">
              <div class="flex items-center justify-between mb-2">
                <h4 class="text-xs font-medium text-[var(--text-primary)]">{{ perf.model_name }}</h4>
                <span class="text-[10px] px-1.5 py-0.5 rounded-full"
                  :style="{
                    background: perf.accuracy > 90 ? 'rgba(34,197,94,0.15)' : perf.accuracy > 80 ? 'rgba(234,179,8,0.15)' : 'rgba(239,68,68,0.15)',
                    color: perf.accuracy > 90 ? 'var(--accent-green)' : perf.accuracy > 80 ? 'var(--accent-yellow)' : 'var(--accent-red)'
                  }">
                  {{ perf.accuracy.toFixed(1) }}%
                </span>
              </div>
              <div class="space-y-1 text-xs">
                <div class="flex justify-between text-[var(--text-secondary)]">
                  <span>训练管线数</span>
                  <span class="font-mono text-[var(--text-primary)]">{{ perf.training_pipelines }}</span>
                </div>
                <div class="flex justify-between text-[var(--text-secondary)]">
                  <span>最近训练</span>
                  <span class="font-mono text-[var(--text-primary)]">{{ perf.last_trained }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 推荐置信度可视化 -->
          <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">推荐置信度与置信区间</h4>
            <svg viewBox="0 0 600 220" class="w-full" style="max-height: 220px;">
              <!-- 背景网格 -->
              <line v-for="i in 5" :key="'bg'+i" :x1="140" :y1="20 + i * 38" :x2="580" :y2="20 + i * 38" stroke="var(--border-subtle)" stroke-width="0.5" />
              <!-- X 轴 -->
              <line x1="140" y1="210" x2="580" y2="210" stroke="var(--text-muted)" stroke-width="1" />
              <text x="360" y="218" text-anchor="middle" fill="var(--text-muted)" font-size="10">置信度 (%)</text>
              <!-- X 轴刻度 -->
              <text v-for="tick in [0, 25, 50, 75, 100]" :key="'xt'+tick" :x="140 + tick * 4.4" :y="205" text-anchor="middle" fill="var(--text-muted)" font-size="9">{{ tick }}</text>
              <!-- 置信区间条 -->
              <g v-for="(rec, ri) in confidenceBars" :key="'cb'+ri">
                <!-- 置信区间背景 -->
                <rect
                  :x="140 + rec.ci_low * 4.4"
                  :y="20 + ri * 38 + 8"
                  :width="(rec.ci_high - rec.ci_low) * 4.4"
                  height="22"
                  rx="4"
                  fill="var(--bg-elevated)"
                  stroke="var(--border-subtle)"
                  stroke-width="0.5"
                />
                <!-- 置信度主条 -->
                <rect
                  :x="140"
                  :y="20 + ri * 38 + 12"
                  :width="rec.confidence * 4.4"
                  height="14"
                  rx="3"
                  :fill="rec.confidence > 0.8 ? 'var(--accent-green)' : rec.confidence > 0.6 ? 'var(--accent-yellow)' : 'var(--accent-red)'"
                  opacity="0.8"
                />
                <!-- 推荐值标记 -->
                <circle
                  :cx="140 + rec.confidence * 4.4"
                  :cy="20 + ri * 38 + 19"
                  r="4"
                  fill="var(--primary)"
                  stroke="white"
                  stroke-width="1.5"
                />
                <!-- 参数名 -->
                <text x="135" :y="20 + ri * 38 + 23" text-anchor="end" fill="var(--text-secondary)" font-size="11">{{ rec.parameter_name }}</text>
                <!-- 置信度数值 -->
                <text :x="140 + rec.confidence * 4.4 + 8" :y="20 + ri * 38 + 23" fill="var(--text-primary)" font-size="10" font-weight="500">
                  {{ (rec.confidence * 100).toFixed(1) }}%
                </text>
              </g>
            </svg>
          </div>

          <!-- 推荐历史表格 -->
          <div class="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg p-4">
            <h4 class="text-sm font-medium mb-3 text-[var(--text-primary)]">推荐历史记录</h4>
            <div class="overflow-x-auto">
              <table class="w-full text-xs">
                <thead>
                  <tr class="border-b border-[var(--border-subtle)]">
                    <th class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">时间</th>
                    <th class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">材料体系</th>
                    <th class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">尺度</th>
                    <th class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">置信度</th>
                    <th class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">状态</th>
                    <th class="text-left py-1.5 px-2 text-[var(--text-muted)] font-medium">实际改善</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(hist, hi) in recommendationHistory" :key="hi" class="border-b border-[var(--border-subtle)]">
                    <td class="py-1.5 px-2 text-[var(--text-secondary)] font-mono">{{ hist.timestamp }}</td>
                    <td class="py-1.5 px-2 text-[var(--text-secondary)]">{{ hist.material }}</td>
                    <td class="py-1.5 px-2 text-[var(--text-secondary)]">{{ hist.scale }}</td>
                    <td class="py-1.5 px-2">
                      <span class="font-mono" :style="{ color: hist.confidence > 0.8 ? 'var(--accent-green)' : hist.confidence > 0.6 ? 'var(--accent-yellow)' : 'var(--accent-red)' }">
                        {{ (hist.confidence * 100).toFixed(1) }}%
                      </span>
                    </td>
                    <td class="py-1.5 px-2">
                      <span class="px-1.5 py-0.5 rounded text-[10px]"
                        :style="{
                          background: hist.applied ? 'rgba(34,197,94,0.15)' : 'rgba(156,163,175,0.15)',
                          color: hist.applied ? 'var(--accent-green)' : 'var(--text-muted)'
                        }">
                        {{ hist.applied ? '已应用' : '未应用' }}
                      </span>
                    </td>
                    <td class="py-1.5 px-2 text-[var(--text-secondary)] font-mono">{{ hist.improvement }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import type { RecommendationScope, RecommendationContext, ParameterRecommendation, RecommendationResult, ModelPerformance } from '../api/aiRecommend'

// ============ 尺度选项 ============
const scaleOptions = ref([
  { value: 'DFT' as RecommendationScope, label: 'DFT', desc: '密度泛函理论，电子尺度' },
  { value: 'MD' as RecommendationScope, label: 'MD', desc: '分子动力学，原子尺度' },
  { value: 'Phase Field' as RecommendationScope, label: 'Phase Field', desc: '相场法，介观尺度' },
  { value: 'FE' as RecommendationScope, label: 'FE', desc: '有限元，宏观尺度' },
  { value: 'Multiscale' as RecommendationScope, label: 'Multiscale', desc: '多尺度耦合' },
])

// ============ 推荐上下文 ============
const context = reactive<RecommendationContext>({
  material_system: 'Mg-Al alloy',
  target_property: 'creep_rate',
  scale: 'fe',
  constraints: [],
  historical_data_range: { pipeline_count: 0, date_range: { start: '', end: '' } },
})

// ============ 约束条件 ============
const constraints = ref<Array<{ name: string; max_value: number }>>([
  { name: 'max_temperature', max_value: 573 },
  { name: 'max_stress', max_value: 150 },
])

function addConstraint() {
  constraints.value.push({ name: '', max_value: 0 })
}

function removeConstraint(idx: number) {
  constraints.value.splice(idx, 1)
}

// ============ 加载状态 ============
const loading = ref(false)

// ============ 推荐结果 ============
const recommendations = ref<ParameterRecommendation[]>([])

// ============ 反馈 ============
const feedbackOptions = [
  { value: 'helpful', label: '有帮助' },
  { value: 'partially', label: '部分有用' },
  { value: 'not_helpful', label: '无帮助' },
]
const feedback = ref('')
const actualValue = ref('')

// ============ 模型性能 ============
const modelPerformances = ref<ModelPerformance[]>([
  {
    model_name: 'CreepNet-v3',
    accuracy: 94.2,
    training_pipelines: 128,
    last_trained: '2026-03-28',
    scales: ['fe', 'md'],
  },
  {
    model_name: 'ParamOpt-XGB',
    accuracy: 91.7,
    training_pipelines: 64,
    last_trained: '2026-03-25',
    scales: ['dft', 'md'],
  },
  {
    model_name: 'MatGPT-Finetune',
    accuracy: 87.3,
    training_pipelines: 32,
    last_trained: '2026-03-20',
    scales: ['phase_field', 'multiscale'],
  },
])

// ============ 推荐历史 ============
const recommendationHistory = ref([
  { timestamp: '2026-04-02 14:30', material: 'Mg-Al alloy', scale: 'FE', confidence: 0.92, applied: true, improvement: '+23.5%' },
  { timestamp: '2026-04-01 09:15', material: 'Ti-6Al-4V', scale: 'MD', confidence: 0.88, applied: true, improvement: '+18.2%' },
  { timestamp: '2026-03-31 16:45', material: 'Inconel 718', scale: 'FE', confidence: 0.95, applied: true, improvement: '+31.7%' },
  { timestamp: '2026-03-30 11:20', material: 'Mg-Al alloy', scale: 'Phase Field', confidence: 0.76, applied: false, improvement: '-' },
  { timestamp: '2026-03-29 08:50', material: 'SiC/SiC CMC', scale: 'Multiscale', confidence: 0.83, applied: true, improvement: '+12.4%' },
  { timestamp: '2026-03-28 15:10', material: 'CFRP', scale: 'FE', confidence: 0.71, applied: false, improvement: '-' },
  { timestamp: '2026-03-27 10:30', material: 'Ti-6Al-4V', scale: 'DFT', confidence: 0.89, applied: true, improvement: '+15.8%' },
  { timestamp: '2026-03-26 13:00', material: 'Mg-Al alloy', scale: 'FE', confidence: 0.91, applied: true, improvement: '+20.1%' },
])

// ============ 置信度可视化数据 ============
const confidenceBars = computed(() => {
  if (recommendations.value.length === 0) return []
  return recommendations.value.map(rec => ({
    parameter_name: rec.parameter_name,
    confidence: rec.confidence,
    ci_low: Math.max(0, rec.confidence - 0.12 - Math.random() * 0.08),
    ci_high: Math.min(1, rec.confidence + 0.08 + Math.random() * 0.06),
  }))
})

// ============ Mock 推荐数据 ============
function generateMockRecommendations(): ParameterRecommendation[] {
  return [
    {
      parameter_name: 'Norton 指数 n',
      recommended_value: 4.82,
      confidence: 0.94,
      rationale: '基于 Mg-Al 合金在 473-573K 范围内的 1,240 条蠕变实验数据，幂律指数 n=4.82 可最佳拟合稳态蠕变行为，R²=0.973。',
      alternatives: [{ value: 4.65, expected_performance: 0.89 }, { value: 5.01, expected_performance: 0.85 }, { value: 4.50, expected_performance: 0.82 }],
      scale: 'fe',
    },
    {
      parameter_name: '激活能 Q (kJ/mol)',
      recommended_value: 135.6,
      confidence: 0.91,
      rationale: 'Arrhenius 分析表明位错攀移控制蠕变的激活能为 135.6 kJ/mol，与 Al 自扩散激活能 (142 kJ/mol) 接近。',
      alternatives: [{ value: 128.3, expected_performance: 0.86 }, { value: 141.2, expected_performance: 0.84 }, { value: 130.0, expected_performance: 0.87 }],
      scale: 'fe',
    },
    {
      parameter_name: 'Norton 系数 A',
      recommended_value: 2.15e-14,
      confidence: 0.87,
      rationale: '结合 n 和 Q 的最优值，通过最小二乘回归得到 A=2.15e-14 MPa^-n s^-1，预测误差在 15% 以内。',
      alternatives: [{ value: 1.89e-14, expected_performance: 0.80 }, { value: 2.45e-14, expected_performance: 0.78 }, { value: 2.00e-14, expected_performance: 0.83 }],
      scale: 'fe',
    },
    {
      parameter_name: '晶粒尺寸 exponent p',
      recommended_value: 0.52,
      confidence: 0.78,
      rationale: 'Hall-Petch 关系的蠕变修正指数 p=0.52，表明晶界滑移对蠕变有中等贡献，与 TEM 观察一致。',
      alternatives: [{ value: 0.45, expected_performance: 0.72 }, { value: 0.60, expected_performance: 0.68 }, { value: 0.50, expected_performance: 0.75 }],
      scale: 'fe',
    },
    {
      parameter_name: '初始应变 threshold',
      recommended_value: 0.002,
      confidence: 0.72,
      rationale: '基于 85% 置信区间推荐的初始蠕变应变阈值，超过此值后进入稳态蠕变阶段，用于加速稳态判定。',
      alternatives: [{ value: 0.0015, expected_performance: 0.65 }, { value: 0.0025, expected_performance: 0.63 }, { value: 0.0018, expected_performance: 0.67 }],
      scale: 'fe',
    },
  ]
}

// ============ 操作方法 ============
function getRecommendations() {
  loading.value = true
  recommendations.value = []

  // 模拟 AI 推理延迟
  setTimeout(() => {
    recommendations.value = generateMockRecommendations()
    loading.value = false
  }, 1800)
}

function submitFeedback() {
  if (!feedback.value) return
  // 提交反馈逻辑
  feedback.value = ''
  actualValue.value = ''
}

function exportRecommendations() {
  // 导出逻辑
}

function resetAll() {
  context.material_system = 'Mg-Al alloy'
  context.target_property = 'creep_rate'
  context.scale = 'fe'
  constraints.value = [
    { name: 'max_temperature', max_value: 573 },
    { name: 'max_stress', max_value: 150 },
  ]
  recommendations.value = []
  feedback.value = ''
  actualValue.value = ''
}

onMounted(() => {
  // 初始化
})
</script>
