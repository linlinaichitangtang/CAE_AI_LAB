<template>
  <div class="math-editor fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="emit('cancel')">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-[600px] max-h-[80vh] flex flex-col">
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white">LaTeX 公式编辑器</h3>
      </div>
      
      <div class="p-4 flex-1 overflow-auto">
        <!-- 输入区域 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">LaTeX 语法</label>
          <textarea
            v-model="latexInput"
            @input="updatePreview"
            class="w-full h-24 p-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 text-gray-800 dark:text-white font-mono text-sm resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="输入 LaTeX 公式，如: \int_{0}^{\infty} e^{-x^2} dx"
          ></textarea>
        </div>

        <!-- 类型选择 -->
        <div class="mb-4 flex gap-4">
          <label class="flex items-center gap-2 cursor-pointer">
            <input type="radio" v-model="display" value="inline" class="text-blue-500" />
            <span class="text-sm text-gray-700 dark:text-gray-300">行内公式</span>
          </label>
          <label class="flex items-center gap-2 cursor-pointer">
            <input type="radio" v-model="display" value="block" class="text-blue-500" />
            <span class="text-sm text-gray-700 dark:text-gray-300">独立公式块</span>
          </label>
        </div>

        <!-- 预览 -->
        <div class="border border-gray-200 dark:border-gray-600 rounded-lg p-4 bg-gray-50 dark:bg-gray-700 min-h-[80px] flex items-center justify-center">
          <div v-if="renderError" class="text-red-500 text-sm">
            {{ renderError }}
          </div>
          <div v-else v-html="renderedLatex" class="text-gray-800 dark:text-white"></div>
        </div>

        <!-- 常用符号 -->
        <div class="mt-4">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">常用符号</label>
          <div class="flex flex-wrap gap-2">
            <button v-for="sym in commonSymbols" :key="sym" @click="insertSymbol(sym)"
              class="px-3 py-1 bg-gray-100 dark:bg-gray-600 rounded hover:bg-gray-200 dark:hover:bg-gray-500 text-gray-700 dark:text-gray-300 font-mono text-sm">
              {{ sym }}
            </button>
          </div>
        </div>
      </div>

      <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
        <button @click="emit('cancel')" class="px-4 py-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded">
          取消
        </button>
        <button @click="handleSave" class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
          插入公式
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import katex from 'katex'

interface Props {
  initialLatex?: string
  initialDisplay?: 'inline' | 'block'
}

const props = withDefaults(defineProps<Props>(), {
  initialLatex: '',
  initialDisplay: 'inline'
})

const emit = defineEmits<{
  (e: 'save', latex: string, display: 'inline' | 'block'): void
  (e: 'cancel'): void
}>()

const latexInput = ref(props.initialLatex)
const display = ref<'inline' | 'block'>(props.initialDisplay)
const renderedLatex = ref('')
const renderError = ref('')

const commonSymbols = [
  '\\alpha', '\\beta', '\\gamma', '\\delta',
  '\\int', '\\sum', '\\prod', '\\sqrt',
  '\\frac', '\\partial', '\\infty', '\\neq',
  '\\leq', '\\geq', '\\cdot', '\\times'
]

const updatePreview = () => {
  renderError.value = ''
  if (!latexInput.value.trim()) {
    renderedLatex.value = ''
    return
  }
  try {
    renderedLatex.value = katex.renderToString(latexInput.value, {
      displayMode: display.value === 'block',
      throwOnError: true
    })
  } catch (e: any) {
    renderError.value = e.message
    renderedLatex.value = ''
  }
}

const insertSymbol = (symbol: string) => {
  latexInput.value += symbol
  updatePreview()
}

const handleSave = () => {
  if (!latexInput.value.trim()) {
    return
  }
  emit('save', latexInput.value, display.value)
}

onMounted(() => {
  updatePreview()
})
</script>

<style>
.math-editor katex {
  font-size: 1.2em;
}
</style>