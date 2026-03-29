<template>
  <div class="embed-selector fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="emit('cancel')">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[700px] max-h-[80vh] flex flex-col">
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white">嵌入对象</h3>
      </div>
      
      <div class="p-4">
        <!-- 嵌入类型选择 -->
        <div class="grid grid-cols-3 gap-4 mb-6">
          <button
            v-for="type in embedTypes"
            :key="type.id"
            @click="selectType = type.id"
            :class="selectType === type.id ? 'ring-2 ring-blue-500 bg-blue-50 dark:bg-blue-900/30' : ''"
            class="p-4 border border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 text-center transition-all"
          >
            <div class="text-3xl mb-2">{{ type.icon }}</div>
            <div class="text-sm font-medium text-gray-800 dark:text-white">{{ type.name }}</div>
            <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ type.desc }}</div>
          </button>
        </div>

        <!-- 3D模型选择 -->
        <div v-if="selectType === 'model'" class="embed-content">
          <div v-if="availableModels.length > 0" class="grid grid-cols-4 gap-3">
            <div
              v-for="model in availableModels"
              :key="model.id"
              @click="selectItem(model)"
              :class="selectedItem?.id === model.id ? 'ring-2 ring-blue-500' : ''"
              class="p-3 border border-gray-200 dark:border-gray-600 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              <div class="text-2xl mb-2 text-center">📐</div>
              <div class="text-sm text-gray-800 dark:text-white truncate text-center">{{ model.name }}</div>
            </div>
          </div>
          <div v-else class="text-center text-gray-500 py-8">
            暂无可嵌入的3D模型
          </div>
        </div>

        <!-- 代码块选择 -->
        <div v-if="selectType === 'code'" class="embed-content">
          <div v-if="availableCodes.length > 0" class="space-y-2">
            <div
              v-for="code in availableCodes"
              :key="code.id"
              @click="selectItem(code)"
              :class="selectedItem?.id === code.id ? 'ring-2 ring-blue-500 bg-blue-50 dark:bg-blue-900/30' : ''"
              class="p-3 border border-gray-200 dark:border-gray-600 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <span class="text-2xl">📄</span>
                  <div>
                    <div class="text-sm font-medium text-gray-800 dark:text-white">{{ code.name }}</div>
                    <div class="text-xs text-gray-500 dark:text-gray-400">{{ code.language }}</div>
                  </div>
                </div>
                <span class="text-xs bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">{{ code.language }}</span>
              </div>
            </div>
          </div>
          <div v-else class="text-center text-gray-500 py-8">
            暂无可嵌入的代码文件
          </div>
        </div>

        <!-- 仿真结果选择 -->
        <div v-if="selectType === 'simulation'" class="embed-content">
          <div v-if="availableSimulations.length > 0" class="grid grid-cols-2 gap-3">
            <div
              v-for="sim in availableSimulations"
              :key="sim.id"
              @click="selectItem(sim)"
              :class="selectedItem?.id === sim.id ? 'ring-2 ring-blue-500' : ''"
              class="p-4 border border-gray-200 dark:border-gray-600 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700"
            >
              <div class="text-2xl mb-2 text-center">📊</div>
              <div class="text-sm font-medium text-gray-800 dark:text-white text-center">{{ sim.name }}</div>
              <div class="text-xs text-gray-500 dark:text-gray-400 text-center mt-1">{{ sim.type }}</div>
            </div>
          </div>
          <div v-else class="text-center text-gray-500 py-8">
            暂无可嵌入的仿真结果
          </div>
        </div>
      </div>

      <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
        <button @click="emit('cancel')" class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
          取消
        </button>
        <button @click="handleEmbed" :disabled="!selectedItem" class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed">
          嵌入
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { EmbedItem } from '@/types'

const emit = defineEmits<{
  (e: 'select', item: EmbedItem): void
  (e: 'cancel'): void
}>()

const selectType = ref<'model' | 'code' | 'simulation'>('model')
const selectedItem = ref<EmbedItem | null>(null)

const embedTypes = [
  { id: 'model' as const, icon: '📐', name: '3D模型', desc: '从项目模型库选择' },
  { id: 'code' as const, icon: '📄', name: '代码块', desc: '嵌入代码文件' },
  { id: 'simulation' as const, icon: '📊', name: '仿真结果', desc: '嵌入分析结果' }
]

// 模拟数据，实际应从store/API获取
const availableModels = [
  { id: 'model-1', name: '支架结构', type: 'model' as const },
  { id: 'model-2', name: '齿轮组件', type: 'model' as const },
  { id: 'model-3', name: '外壳设计', type: 'model' as const }
]

const availableCodes = [
  { id: 'code-1', name: 'main.py', language: 'Python', type: 'code' as const },
  { id: 'code-2', name: 'solver.py', language: 'Python', type: 'code' as const },
  { id: 'code-3', name: 'api.ts', language: 'TypeScript', type: 'code' as const }
]

const availableSimulations = [
  { id: 'sim-1', name: '静力学分析', type: 'Stress', type2: 'simulation' as const },
  { id: 'sim-2', name: '模态分析', type: 'Modal', type2: 'simulation' as const },
  { id: 'sim-3', name: '热分析', type: 'Thermal', type2: 'simulation' as const }
]

const selectItem = (item: any) => {
  selectedItem.value = item
}

const handleEmbed = () => {
  if (selectedItem.value) {
    emit('select', selectedItem.value)
  }
}
</script>

<style scoped>
.embed-content {
  min-height: 200px;
}
</style>