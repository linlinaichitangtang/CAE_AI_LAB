<template>
  <div class="validation-report">
    <!-- 报告标题 -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-semibold text-gray-800 flex items-center gap-2">
        <span class="w-5 h-5 rounded bg-blue-600 text-white text-xs flex items-center justify-center">V</span>
        验证报告
      </h3>
      <button
        v-if="report"
        @click="$emit('close')"
        class="text-xs text-gray-400 hover:text-gray-600 transition"
      >
        关闭
      </button>
    </div>

    <!-- 无报告时的提示 -->
    <div v-if="!report" class="text-center py-6">
      <div class="text-3xl mb-2 text-gray-300">&#x2714;</div>
      <p class="text-xs text-gray-400">运行标准算例求解后，验证报告将自动生成</p>
    </div>

    <!-- 验证报告内容 -->
    <div v-else class="space-y-4">
      <!-- 算例信息 -->
      <div class="bg-gray-50 rounded-lg p-3">
        <h4 class="text-xs font-semibold text-gray-700 mb-1">{{ report.caseName }}</h4>
        <p class="text-[10px] text-gray-500">
          生成时间: {{ formatTimestamp(report.timestamp) }}
        </p>
      </div>

      <!-- 验收结论 -->
      <div
        class="rounded-lg p-3 text-center"
        :class="report.result === 'PASS' ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'"
      >
        <div
          class="text-lg font-bold"
          :class="report.result === 'PASS' ? 'text-green-600' : 'text-red-600'"
        >
          {{ report.result === 'PASS' ? 'PASS' : 'FAIL' }}
        </div>
        <p class="text-xs mt-1" :class="report.result === 'PASS' ? 'text-green-700' : 'text-red-700'">
          {{ report.result === 'PASS' ? '所有指标均在验收标准范围内' : '部分指标超出验收标准' }}
        </p>
      </div>

      <!-- 对比表格 -->
      <div class="border rounded-lg overflow-hidden">
        <table class="w-full text-xs">
          <thead>
            <tr class="bg-gray-100">
              <th class="px-3 py-2 text-left font-medium text-gray-600">指标</th>
              <th class="px-3 py-2 text-right font-medium text-gray-600">理论值</th>
              <th class="px-3 py-2 text-right font-medium text-gray-600">数值解</th>
              <th class="px-3 py-2 text-right font-medium text-gray-600">误差</th>
              <th class="px-3 py-2 text-right font-medium text-gray-600">限值</th>
            </tr>
          </thead>
          <tbody>
            <!-- 最大位移 -->
            <tr class="border-t">
              <td class="px-3 py-2 text-gray-700 font-medium">最大位移</td>
              <td class="px-3 py-2 text-right text-gray-600">
                {{ formatDisplacement(report.theoretical.max_displacement) }}
              </td>
              <td class="px-3 py-2 text-right text-gray-600">
                {{ formatDisplacement(report.numerical.max_displacement) }}
              </td>
              <td class="px-3 py-2 text-right">
                <span
                  class="inline-block px-1.5 py-0.5 rounded text-[10px] font-medium"
                  :class="getErrorClass(report.errors.displacement_error)"
                >
                  {{ report.errors.displacement_error.toFixed(2) }}%
                </span>
              </td>
              <td class="px-3 py-2 text-right text-gray-500">
                {{ report.acceptance.displacement_error_limit }}%
              </td>
            </tr>
            <!-- 最大应力 -->
            <tr class="border-t">
              <td class="px-3 py-2 text-gray-700 font-medium">最大应力</td>
              <td class="px-3 py-2 text-right text-gray-600">
                {{ formatStress(report.theoretical.max_stress) }}
              </td>
              <td class="px-3 py-2 text-right text-gray-600">
                {{ formatStress(report.numerical.max_stress) }}
              </td>
              <td class="px-3 py-2 text-right">
                <span
                  class="inline-block px-1.5 py-0.5 rounded text-[10px] font-medium"
                  :class="getErrorClass(report.errors.stress_error)"
                >
                  {{ report.errors.stress_error.toFixed(2) }}%
                </span>
              </td>
              <td class="px-3 py-2 text-right text-gray-500">
                {{ report.acceptance.stress_error_limit }}%
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- 误差等级图例 -->
      <div class="flex items-center gap-3 text-[10px] text-gray-500">
        <span class="flex items-center gap-1">
          <span class="w-2.5 h-2.5 rounded-full bg-green-500"></span>
          优秀 (&lt;2%)
        </span>
        <span class="flex items-center gap-1">
          <span class="w-2.5 h-2.5 rounded-full bg-yellow-500"></span>
          合格 (2%~5%)
        </span>
        <span class="flex items-center gap-1">
          <span class="w-2.5 h-2.5 rounded-full bg-red-500"></span>
          超标 (&ge;5%)
        </span>
      </div>

      <!-- 详细说明 -->
      <div class="text-xs text-gray-600 bg-gray-50 rounded p-3">
        <p class="font-medium mb-1">说明</p>
        <p>{{ report.details }}</p>
      </div>

      <!-- 理论公式 -->
      <div v-if="caseData" class="text-xs text-gray-600 bg-blue-50 rounded p-3">
        <p class="font-medium mb-1 text-blue-700">理论公式</p>
        <p class="font-mono text-[11px]">{{ caseData.theoretical.formula }}</p>
        <p class="mt-1 text-[10px] text-gray-500">参考文献: {{ caseData.theoretical.reference }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ValidationReport } from '@/utils/standardCases'
import { getCaseById, getErrorLevelClass } from '@/utils/standardCases'

const props = defineProps<{
  report: ValidationReport | null
}>()

defineEmits<{
  close: []
}>()

const caseData = computed(() => {
  if (!props.report) return null
  return getCaseById(props.report.caseId)
})

function getErrorClass(errorPercent: number): string {
  if (errorPercent < 2) {
    return 'text-green-700 bg-green-100'
  } else if (errorPercent < 5) {
    return 'text-yellow-700 bg-yellow-100'
  } else {
    return 'text-red-700 bg-red-100'
  }
}

function formatDisplacement(value: number): string {
  if (Math.abs(value) >= 0.001) {
    return (value * 1000).toFixed(4) + ' mm'
  } else if (Math.abs(value) >= 1e-6) {
    return (value * 1e6).toFixed(2) + ' um'
  } else {
    return value.toExponential(4) + ' m'
  }
}

function formatStress(value: number): string {
  if (Math.abs(value) >= 1e6) {
    return (value / 1e6).toFixed(2) + ' MPa'
  } else if (Math.abs(value) >= 1e3) {
    return (value / 1e3).toFixed(2) + ' kPa'
  } else {
    return value.toFixed(2) + ' Pa'
  }
}

function formatTimestamp(iso: string): string {
  try {
    const d = new Date(iso)
    return d.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    })
  } catch {
    return iso
  }
}
</script>
