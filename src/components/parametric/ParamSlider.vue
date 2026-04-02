<template>
  <div class="param-slider" :class="{ 'param-slider--dark': isDark }">
    <div class="param-slider__header">
      <label class="param-slider__label">{{ param.name }}</label>
      <span class="param-slider__unit" v-if="param.unit">{{ param.unit }}</span>
    </div>

    <div class="param-slider__body">
      <!-- Slider -->
      <div class="param-slider__track-wrapper">
        <input
          ref="sliderRef"
          type="range"
          class="param-slider__input"
          :min="param.min"
          :max="param.max"
          :step="param.step"
          :value="displayValue"
          @input="onSliderInput"
          @change="onSliderChange"
          :disabled="disabled"
        />
        <div
          class="param-slider__fill"
          :style="fillStyle"
        />
      </div>

      <!-- Numeric input -->
      <div class="param-slider__input-wrapper">
        <input
          type="number"
          class="param-slider__number"
          :value="displayValue"
          @input="onNumberInput"
          @change="onNumberChange"
          @blur="onNumberBlur"
          :min="param.min"
          :max="param.max"
          :step="param.step"
          :disabled="disabled"
        />
      </div>
    </div>

    <!-- Range labels -->
    <div class="param-slider__range-labels">
      <span>{{ formatValue(param.min) }}</span>
      <span>{{ formatValue(param.max) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import type { ParamDef } from '@/stores/parametric'

const props = defineProps<{
  param: ParamDef
  modelValue: number
  disabled?: boolean
  debounceMs?: number
}>()

const emit = defineEmits<{
  'update:modelValue': [value: number]
  'change': [value: number]
}>()

const sliderRef = ref<HTMLInputElement | null>(null)
const isDark = ref(false)
const localValue = ref(props.modelValue)
let debounceTimer: ReturnType<typeof setTimeout> | null = null

const displayValue = computed(() => {
  return Number(localValue.value.toFixed(getDecimalPlaces(props.param.step)))
})

const fillStyle = computed(() => {
  const range = props.param.max - props.param.min
  if (range === 0) return { width: '0%' }
  const pct = ((localValue.value - props.param.min) / range) * 100
  return { width: `${Math.min(Math.max(pct, 0), 100)}%` }
})

function getDecimalPlaces(step: number): number {
  if (step >= 1) return 0
  const str = step.toString()
  const dotIdx = str.indexOf('.')
  return dotIdx === -1 ? 0 : str.length - dotIdx - 1
}

function formatValue(val: number): string {
  const places = getDecimalPlaces(props.param.step)
  return val.toFixed(places)
}

function clampValue(val: number): number {
  const clamped = Math.min(Math.max(val, props.param.min), props.param.max)
  return Math.round(clamped / props.param.step) * props.param.step
}

function onSliderInput(e: Event) {
  const target = e.target as HTMLInputElement
  const raw = parseFloat(target.value)
  if (isNaN(raw)) return
  const val = clampValue(raw)
  localValue.value = val

  const ms = props.debounceMs ?? 50
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    emit('update:modelValue', val)
  }, ms)
}

function onSliderChange(e: Event) {
  const target = e.target as HTMLInputElement
  const raw = parseFloat(target.value)
  if (isNaN(raw)) return
  const val = clampValue(raw)
  localValue.value = val
  emit('update:modelValue', val)
  emit('change', val)
}

function onNumberInput(e: Event) {
  const target = e.target as HTMLInputElement
  const raw = parseFloat(target.value)
  if (isNaN(raw)) return
  const val = clampValue(raw)
  localValue.value = val

  const ms = props.debounceMs ?? 50
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    emit('update:modelValue', val)
  }, ms)
}

function onNumberChange(e: Event) {
  const target = e.target as HTMLInputElement
  const raw = parseFloat(target.value)
  if (isNaN(raw)) return
  const val = clampValue(raw)
  localValue.value = val
  emit('update:modelValue', val)
  emit('change', val)
}

function onNumberBlur() {
  localValue.value = clampValue(localValue.value)
}

// Sync with external modelValue changes
watch(() => props.modelValue, (newVal) => {
  if (newVal !== localValue.value) {
    localValue.value = newVal
  }
})

// Detect dark mode
function checkDarkMode() {
  isDark.value = document.documentElement.classList.contains('dark')
}

onMounted(() => {
  checkDarkMode()
  window.addEventListener('classchange', checkDarkMode)
  // Observe attribute changes on html for dark mode
  const observer = new MutationObserver(checkDarkMode)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
  onUnmounted(() => {
    observer.disconnect()
    window.removeEventListener('classchange', checkDarkMode)
    if (debounceTimer) clearTimeout(debounceTimer)
  })
})
</script>

<style scoped>
.param-slider {
  padding: 8px 0;
}

.param-slider__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 6px;
}

.param-slider__label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary, #374151);
}

.param-slider--dark .param-slider__label {
  color: var(--text-primary, #e5e7eb);
}

.param-slider__unit {
  font-size: 11px;
  color: var(--text-secondary, #9ca3af);
  margin-left: 4px;
}

.param-slider__body {
  display: flex;
  align-items: center;
  gap: 10px;
}

.param-slider__track-wrapper {
  flex: 1;
  position: relative;
  height: 28px;
  display: flex;
  align-items: center;
}

.param-slider__input {
  -webkit-appearance: none;
  appearance: none;
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-tertiary, #e5e7eb);
  outline: none;
  position: relative;
  z-index: 2;
  cursor: pointer;
}

.param-slider--dark .param-slider__input {
  background: #374151;
}

.param-slider__input::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--primary, #4f46e5);
  cursor: pointer;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.25);
  transition: box-shadow 0.15s ease;
}

.param-slider__input::-webkit-slider-thumb:hover {
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.4);
}

.param-slider__input::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--primary, #4f46e5);
  cursor: pointer;
  border: none;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.25);
}

.param-slider__input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.param-slider__input:disabled::-webkit-slider-thumb {
  cursor: not-allowed;
}

.param-slider__fill {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  height: 6px;
  border-radius: 3px;
  background: var(--primary, #4f46e5);
  opacity: 0.3;
  z-index: 1;
  pointer-events: none;
}

.param-slider__input-wrapper {
  width: 80px;
  flex-shrink: 0;
}

.param-slider__number {
  width: 100%;
  padding: 4px 6px;
  font-size: 12px;
  border: 1px solid var(--border-color, #d1d5db);
  border-radius: 6px;
  background: var(--bg-primary, #ffffff);
  color: var(--text-primary, #374151);
  text-align: center;
  outline: none;
  transition: border-color 0.15s ease;
  -moz-appearance: textfield;
}

.param-slider__number::-webkit-inner-spin-button,
.param-slider__number::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.param-slider--dark .param-slider__number {
  background: #1f2937;
  border-color: #4b5563;
  color: #e5e7eb;
}

.param-slider__number:focus {
  border-color: var(--primary, #4f46e5);
  box-shadow: 0 0 0 2px rgba(79, 70, 229, 0.15);
}

.param-slider__number:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.param-slider__range-labels {
  display: flex;
  justify-content: space-between;
  margin-top: 2px;
  font-size: 10px;
  color: var(--text-secondary, #9ca3af);
}
</style>
