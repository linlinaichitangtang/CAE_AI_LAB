<script setup lang="ts">
/**
 * SimulationAI.vue — AI辅助功能模块
 * 从 SimulationView.vue 提取 (~300 行)
 */
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '@/stores/project'
import { useAiStore } from '@/stores/ai'
import * as caeApi from '@/api/cae'

const projectStore = useProjectStore()
const aiStore = useAiStore()

// ========== AI设置参数相关状态 ==========
const showAISetupDialogFlag = ref(false)
const aiSetupPrompt = ref('')
const aiSetupThinking = ref(false)
const aiSetupError = ref<string | null>(null)
const aiParsedSetup = ref<{
  fixedPosition?: string
  loadPosition?: string
  loadMagnitude?: string
  loadDirection?: string
} | null>(null)

// ========== AI解释结果相关状态 ==========
const showAIResultDialogFlag = ref(false)
const aiResultThinking = ref(false)
const aiResultError = ref<string | null>(null)
const aiResultExplanation = ref<{
  maxStressPosition?: string
  maxStressValue?: string
  isSafe?: boolean
  safetyCheck?: string
  safetyFactor?: string
  improvementSuggestions?: string
} | null>(null)

const emit = defineEmits<{
  (e: 'parsed-setup-applied', setup: typeof aiParsedSetup.value): void
}>()

// 打开AI设置参数对话框
function showAISetupDialog() {
  aiSetupPrompt.value = ''
  aiSetupError.value = null
  aiParsedSetup.value = null
  showAISetupDialogFlag.value = true
}

// 打开AI解释结果对话框
function showAIResultDialog() {
  aiResultError.value = null
  aiResultExplanation.value = null
  showAIResultDialogFlag.value = true
  analyzeSimulationResult()
}

// 获取当前材料
function getCurrentMaterial() {
  if (projectStore.currentMaterial) {
    return projectStore.currentMaterial
  }
  return { elastic_modulus: 210000, poisson_ratio: 0.3 }
}

// 解析AI设置参数并应用
async function applyAISetup() {
  if (!aiSetupPrompt.value.trim()) return

  aiSetupThinking.value = true
  aiSetupError.value = null
  aiParsedSetup.value = null

  try {
    const systemPrompt = `你是CAE仿真参数解析助手。用户会描述一个力学问题，你需要解析出边界条件参数。
请严格按照以下JSON格式输出（不要有其他内容）：
{
  "fixedPosition": "固定位置描述，如：左端固定、右端固定、一端固定等",
  "loadPosition": "荷载位置描述，如：右端顶部、右端底部、顶部等",
  "loadMagnitude": "荷载大小及单位，如：1000N、500N/mm等",
  "loadDirection": "荷载方向描述，如：向下、向上、水平向右等"
}

如果无法解析，请返回：
{
  "error": "无法解析该描述，请使用更明确的语言描述"
}`

    const response = await invoke<string>('ai_chat_completion', {
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: aiSetupPrompt.value }
      ],
      config: {
        aiSource: aiStore.config.aiSource,
        apiUrl: aiStore.config.apiUrl,
        apiKey: aiStore.config.apiKey,
        modelName: aiStore.config.modelName,
        temperature: 0.3,
        maxTokens: 500
      }
    })

    try {
      const parsed = JSON.parse(response)
      if (parsed.error) {
        aiSetupError.value = parsed.error
      } else {
        aiParsedSetup.value = parsed
        await applyParsedSetupToBC(parsed)
        emit('parsed-setup-applied', parsed)
      }
    } catch (parseErr) {
      const match = response.match(/\{[\s\S]*\}/)
      if (match) {
        const parsed = JSON.parse(match[0])
        if (parsed.error) {
          aiSetupError.value = parsed.error
        } else {
          aiParsedSetup.value = parsed
          await applyParsedSetupToBC(parsed)
          emit('parsed-setup-applied', parsed)
        }
      } else {
        aiSetupError.value = '无法解析AI响应，请重试'
      }
    }
  } catch (e: any) {
    aiSetupError.value = String(e)
  } finally {
    aiSetupThinking.value = false
  }
}

// 将解析的设置应用到边界条件
async function applyParsedSetupToBC(parsed: any) {
  if (!projectStore.currentMesh) return

  const nodes = projectStore.currentMesh.nodes

  let fixedBcName = 'AI设置-固定'
  if (parsed.fixedPosition) {
    if (parsed.fixedPosition.includes('左')) {
      const fixedBc = await caeApi.createCantileverFixedBc(nodes)
      projectStore.addFixedBc(fixedBc)
    } else if (parsed.fixedPosition.includes('右')) {
      const maxX = Math.max(...nodes.map((n: any) => n.x))
      const rightNodes = nodes.filter((n: any) => Math.abs(n.x - maxX) < 0.01)
      const fixedBc = {
        name: fixedBcName,
        nodes: rightNodes.map((n: any) => n.id),
        bc_type: 'DISPLACEMENT'
      }
      projectStore.addFixedBc(fixedBc)
    }
  }

  let loadMagnitude = 1000
  if (parsed.loadMagnitude) {
    const match = parsed.loadMagnitude.match(/(\d+(?:\.\d+)?)\s*(N|kN)?/i)
    if (match) {
      loadMagnitude = parseFloat(match[1])
      if (match[2]?.toLowerCase() === 'kn') {
        loadMagnitude *= 1000
      }
    }
  }

  if (parsed.loadPosition) {
    let loadNodeId = 0
    if (parsed.loadPosition.includes('右') || parsed.loadPosition.includes('端')) {
      const maxX = Math.max(...nodes.map((n: any) => n.x))
      const rightNodes = nodes.filter((n: any) => Math.abs(n.x - maxX) < 0.01)
      const maxY = Math.max(...rightNodes.map((n: any) => n.y))
      const topRightNode = rightNodes.find((n: any) => Math.abs(n.y - maxY) < 0.01)
      if (topRightNode) {
        loadNodeId = topRightNode.id
      }
    }

    if (loadNodeId > 0) {
      let direction = 'Y'
      if (parsed.loadDirection) {
        if (parsed.loadDirection.includes('下')) direction = 'Y'
        else if (parsed.loadDirection.includes('上')) direction = 'Y'
        else if (parsed.loadDirection.includes('左')) direction = 'X'
        else if (parsed.loadDirection.includes('右')) direction = 'X'
      }
      const load = {
        name: 'AI设置-荷载',
        node: loadNodeId,
        magnitude: loadMagnitude,
        direction: direction
      }
      projectStore.addPointLoad(load)
    }
  }
}

// 分析仿真结果
async function analyzeSimulationResult() {
  if (!projectStore.lastResult || !projectStore.currentMesh) return

  aiResultThinking.value = true
  aiResultError.value = null

  try {
    const nodes = projectStore.currentMesh.nodes
    const elements = projectStore.currentMesh.elements

    const resultSet = projectStore.lastResult
    let maxStress = 0
    let maxStressNode = 0

    if (resultSet.node_values) {
      for (const stepResults of resultSet.node_values) {
        for (const nodeResult of stepResults) {
          const val = Math.abs(nodeResult.value)
          if (val > maxStress) {
            maxStress = val
            maxStressNode = nodeResult.node_id
          }
        }
      }
    }

    const maxNode = nodes.find((n: any) => n.id === maxStressNode)
    const positionStr = maxNode
      ? `X=${maxNode.x.toFixed(2)}, Y=${maxNode.y.toFixed(2)}, Z=${maxNode.z.toFixed(2)}`
      : '待计算'

    const material = getCurrentMaterial()
    const systemPrompt = `你是CAE仿真结果分析专家。请根据以下仿真参数和分析结果，给出专业的强度校核和改进建议。

仿真参数：
- 弹性模量: ${material.elastic_modulus} MPa
- 泊松比: ${material.poisson_ratio}
- 网格: ${nodes.length} 节点, ${elements.length} 单元

分析结果：
- 最大应力位置: ${positionStr}
- 最大应力值: ${maxStress.toFixed(2)} MPa

请按以下JSON格式输出分析结果：
{
  "maxStressPosition": "最大应力位置描述",
  "maxStressValue": "最大应力数值",
  "isSafe": true/false,
  "safetyCheck": "强度校核说明",
  "safetyFactor": "安全系数（如适用）",
  "improvementSuggestions": "改进建议"
}

假设材料为普通钢材，屈服强度约为250 MPa。`

    const response = await invoke<string>('ai_chat_completion', {
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: '请分析这个仿真结果，给出强度校核和改进建议。' }
      ],
      config: {
        aiSource: aiStore.config.aiSource,
        apiUrl: aiStore.config.apiUrl,
        apiKey: aiStore.config.apiKey,
        modelName: aiStore.config.modelName,
        temperature: 0.5,
        maxTokens: 1000
      }
    })

    try {
      const parsed = JSON.parse(response)
      aiResultExplanation.value = {
        maxStressPosition: positionStr,
        maxStressValue: maxStress.toFixed(2),
        isSafe: parsed.isSafe ?? (maxStress < 250),
        safetyCheck: parsed.safetyCheck ?? (maxStress < 250 ? '最大应力小于屈服强度，安全' : '最大应力超过屈服强度，不安全'),
        safetyFactor: parsed.safetyFactor ?? (maxStress > 0 ? (250 / maxStress).toFixed(2) : 'N/A'),
        improvementSuggestions: parsed.improvementSuggestions
      }
    } catch (parseErr) {
      const match = response.match(/\{[\s\S]*\}/)
      if (match) {
        const parsed = JSON.parse(match[0])
        aiResultExplanation.value = {
          maxStressPosition: positionStr,
          maxStressValue: maxStress.toFixed(2),
          isSafe: parsed.isSafe ?? (maxStress < 250),
          safetyCheck: parsed.safetyCheck ?? (maxStress < 250 ? '最大应力小于屈服强度，安全' : '最大应力超过屈服强度，不安全'),
          safetyFactor: parsed.safetyFactor ?? (maxStress > 0 ? (250 / maxStress).toFixed(2) : 'N/A'),
          improvementSuggestions: parsed.improvementSuggestions
        }
      } else {
        aiResultError.value = '无法解析AI响应'
      }
    }
  } catch (e: any) {
    aiResultError.value = String(e)
  } finally {
    aiResultThinking.value = false
  }
}

// 暴露给父组件使用
defineExpose({
  showAISetupDialog,
  showAIResultDialog,
  showAISetupDialogFlag,
  showAIResultDialogFlag,
  aiParsedSetup
})
</script>

<template>
  <!-- AI设置参数对话框 -->
  <div v-if="showAISetupDialogFlag" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl w-[600px] max-h-[80vh] flex flex-col">
      <div class="p-4 border-b flex justify-between items-center">
        <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
          <span>🤖</span>
          <span>AI辅助边界条件设置</span>
        </h3>
        <button @click="showAISetupDialogFlag = false" class="text-gray-500 hover:text-gray-700">✕</button>
      </div>

      <div class="p-4 flex-1 overflow-y-auto">
        <div class="bg-blue-50 rounded-lg p-3 mb-4">
          <p class="text-sm text-blue-700">
            描述你的力学问题（如："悬臂梁左端固定，右侧施加1000N向下的力"），AI将自动解析并应用边界条件。
          </p>
        </div>

        <textarea
          v-model="aiSetupPrompt"
          class="w-full px-3 py-2 border rounded-lg text-sm"
          rows="4"
          placeholder="请描述边界条件，例如：悬臂梁左端固定，右侧顶部施加1000N向下的力"
        />

        <div v-if="aiSetupError" class="mt-2 text-sm text-red-600 bg-red-50 rounded px-3 py-2">
          {{ aiSetupError }}
        </div>

        <div v-if="aiSetupThinking" class="mt-3 text-sm text-gray-500 flex items-center gap-2">
          <span class="animate-spin">⏳</span>
          <span>AI正在解析...</span>
        </div>

        <div v-if="aiParsedSetup" class="mt-4 bg-green-50 rounded-lg p-3">
          <h4 class="text-sm font-medium text-green-700 mb-2">✅ 解析结果</h4>
          <div class="text-sm text-green-600 space-y-1">
            <div v-if="aiParsedSetup.fixedPosition">固定位置: {{ aiParsedSetup.fixedPosition }}</div>
            <div v-if="aiParsedSetup.loadPosition">荷载位置: {{ aiParsedSetup.loadPosition }}</div>
            <div v-if="aiParsedSetup.loadMagnitude">荷载大小: {{ aiParsedSetup.loadMagnitude }}</div>
            <div v-if="aiParsedSetup.loadDirection">荷载方向: {{ aiParsedSetup.loadDirection }}</div>
          </div>
        </div>
      </div>

      <div class="p-4 border-t flex justify-end gap-3">
        <button
          @click="showAISetupDialogFlag = false"
          class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 text-sm"
        >
          取消
        </button>
        <button
          @click="applyAISetup"
          :disabled="!aiSetupPrompt.trim() || aiSetupThinking"
          class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm disabled:opacity-50"
        >
          {{ aiSetupThinking ? '解析中...' : '解析并应用' }}
        </button>
      </div>
    </div>
  </div>

  <!-- AI解释结果对话框 -->
  <div v-if="showAIResultDialogFlag" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl w-[600px] max-h-[80vh] flex flex-col">
      <div class="p-4 border-b flex justify-between items-center">
        <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2">
          <span>📊</span>
          <span>AI仿真结果分析</span>
        </h3>
        <button @click="showAIResultDialogFlag = false" class="text-gray-500 hover:text-gray-700">✕</button>
      </div>

      <div class="p-4 flex-1 overflow-y-auto">
        <div v-if="aiResultThinking" class="text-center py-8">
          <span class="animate-spin text-3xl">⏳</span>
          <p class="text-gray-500 mt-2">AI正在分析仿真结果...</p>
        </div>

        <div v-else-if="aiResultError" class="text-center py-8">
          <p class="text-red-500">{{ aiResultError }}</p>
          <button @click="analyzeSimulationResult" class="mt-2 text-blue-600 underline">重试</button>
        </div>

        <div v-else-if="aiResultExplanation" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div class="bg-gray-50 rounded-lg p-4">
              <div class="text-xs text-gray-500 mb-1">最大应力位置</div>
              <div class="text-sm font-medium">{{ aiResultExplanation.maxStressPosition }}</div>
            </div>
            <div class="bg-gray-50 rounded-lg p-4">
              <div class="text-xs text-gray-500 mb-1">最大应力值</div>
              <div class="text-sm font-medium">{{ aiResultExplanation.maxStressValue }} MPa</div>
            </div>
          </div>

          <div class="bg-green-50 rounded-lg p-4" v-if="aiResultExplanation.isSafe">
            <div class="flex items-center gap-2 mb-1">
              <span class="text-green-600">✅</span>
              <span class="font-medium text-green-700">安全</span>
            </div>
            <p class="text-sm text-green-700">{{ aiResultExplanation.safetyCheck }}</p>
            <div v-if="aiResultExplanation.safetyFactor" class="mt-2 text-sm">
              安全系数: <span class="font-bold">{{ aiResultExplanation.safetyFactor }}</span>
            </div>
          </div>

          <div class="bg-red-50 rounded-lg p-4" v-else>
            <div class="flex items-center gap-2 mb-1">
              <span class="text-red-600">⚠️</span>
              <span class="font-medium text-red-700">危险</span>
            </div>
            <p class="text-sm text-red-700">{{ aiResultExplanation.safetyCheck }}</p>
          </div>

          <div v-if="aiResultExplanation.improvementSuggestions" class="bg-blue-50 rounded-lg p-4">
            <h4 class="text-sm font-medium text-blue-700 mb-2">💡 改进建议</h4>
            <p class="text-sm text-blue-700">{{ aiResultExplanation.improvementSuggestions }}</p>
          </div>
        </div>
      </div>

      <div class="p-4 border-t flex justify-end">
        <button
          @click="showAIResultDialogFlag = false"
          class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 text-sm"
        >
          关闭
        </button>
      </div>
    </div>
  </div>
</template>