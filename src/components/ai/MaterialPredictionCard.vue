<template>
  <div class="prediction-card" :class="{ low: isLowConfidence }">
    <div class="card-header">
      <span class="property-name">{{ label }}</span>
      <span class="property-unit">{{ unit }}</span>
    </div>
    <div class="card-value">{{ formattedValue }}</div>
    <div class="card-confidence">
      <div class="confidence-bar">
        <div
          class="confidence-range"
          :style="rangeStyle"
        ></div>
        <div
          class="confidence-marker"
          :style="markerStyle"
        ></div>
      </div>
      <div class="confidence-text">
        <span class="uncertainty">± {{ formattedUncertainty }}</span>
        <span v-if="isLowConfidence" class="low-badge">低置信</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { PropertyPrediction } from '../../api/mlPredict'

const props = defineProps<{
  prediction: PropertyPrediction
  label: string
}>()

const unit = computed(() => props.prediction.unit)

const formattedValue = computed(() => {
  const v = props.prediction.predictedValue
  const prop = props.prediction.propertyName
  if (prop === 'poissons_ratio') return v.toFixed(3)
  if (v >= 1e9) return (v / 1e9).toFixed(2) + ' GPa'
  if (v >= 1e6) return (v / 1e6).toFixed(1) + ' MPa'
  if (v >= 1e3) return (v / 1e3).toFixed(1) + ' kPa'
  return v.toFixed(2)
})

const formattedUncertainty = computed(() => {
  const u = props.prediction.uncertainty
  const prop = props.prediction.propertyName
  if (prop === 'poissons_ratio') return u.toFixed(4)
  if (u >= 1e9) return (u / 1e9).toFixed(2) + ' GPa'
  if (u >= 1e6) return (u / 1e6).toFixed(1) + ' MPa'
  if (u >= 1e3) return (u / 1e3).toFixed(1) + ' kPa'
  return u.toFixed(2)
})

const isLowConfidence = computed(() => {
  const v = props.prediction.predictedValue
  const u = props.prediction.uncertainty
  return v !== 0 && (u / Math.abs(v)) > 0.10
})

const rangeWidth = computed(() => {
  const range = props.prediction.confidenceUpper - props.prediction.confidenceLower
  const total = props.prediction.predictedValue * 2 || 1
  return Math.min((range / total) * 100, 100)
})

const rangeLeft = computed(() => {
  const total = props.prediction.predictedValue * 2 || 1
  const offset = props.prediction.confidenceLower - (props.prediction.predictedValue - total / 2)
  return Math.max(0, Math.min((offset / total) * 100, 100))
})

const rangeStyle = computed(() => ({
  left: rangeLeft.value + '%',
  width: rangeWidth.value + '%',
}))

const markerStyle = computed(() => ({
  left: '50%',
}))
</script>

<style scoped>
.prediction-card {
  background: var(--bg-primary, #11111b);
  border: 1px solid var(--border-color, #313244);
  border-radius: 8px;
  padding: 12px;
  transition: border-color 0.2s;
}
.prediction-card.low {
  border-color: rgba(249, 226, 175, 0.4);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}
.property-name {
  font-size: 12px;
  color: var(--text-secondary, #a6adc8);
}
.property-unit {
  font-size: 11px;
  color: var(--text-secondary, #a6adc8);
  opacity: 0.7;
}

.card-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--accent-color, #89b4fa);
  margin-bottom: 8px;
}

.card-confidence {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.confidence-bar {
  position: relative;
  height: 4px;
  background: var(--bg-secondary, #1e1e2e);
  border-radius: 2px;
  overflow: hidden;
}
.confidence-range {
  position: absolute;
  top: 0;
  height: 100%;
  background: rgba(137, 180, 250, 0.3);
  border-radius: 2px;
}
.confidence-marker {
  position: absolute;
  top: -2px;
  width: 2px;
  height: 8px;
  background: var(--accent-color, #89b4fa);
  border-radius: 1px;
  transform: translateX(-50%);
}

.confidence-text {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.uncertainty {
  font-size: 10px;
  color: var(--text-secondary, #a6adc8);
}
.low-badge {
  font-size: 10px;
  background: rgba(249, 226, 175, 0.15);
  color: #f9e2af;
  padding: 1px 6px;
  border-radius: 4px;
}
</style>
