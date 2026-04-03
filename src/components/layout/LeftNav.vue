<template>
  <nav class="desktop-only h-full bg-[var(--bg-surface)] flex flex-col border-r border-[var(--border-default)]">
    <!-- Main Navigation -->
    <div class="flex flex-col items-center py-3 gap-1 flex-1">
      <router-link 
        v-for="item in mainNavItems" 
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ 'active': isActive(item.path) }"
        :title="item.label"
      >
        <component :is="item.icon" class="nav-icon" />
        <span class="nav-label">{{ item.label }}</span>
        <span v-if="item.badge" class="nav-badge">{{ item.badge }}</span>
      </router-link>
    </div>

    <!-- Divider -->
    <div class="mx-3 h-px bg-[var(--border-subtle)]"></div>

    <!-- Bottom Section -->
    <div class="flex flex-col items-center py-3 gap-1">
      <router-link 
        to="/settings" 
        class="nav-item"
        :class="{ 'active': $route.path === '/settings' }"
        title="设置"
      >
        <SettingsIcon class="nav-icon" />
        <span class="nav-label">设置</span>
      </router-link>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { h } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

// SVG Icon Components
const HomeIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z' }),
    h('polyline', { points: '9 22 9 12 15 12 15 22' })
  ])
}

const NotesIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z' }),
    h('polyline', { points: '14 2 14 8 20 8' }),
    h('line', { x1: '16', y1: '13', x2: '8', y2: '13' }),
    h('line', { x1: '16', y1: '17', x2: '8', y2: '17' }),
    h('polyline', { points: '10 9 9 9 8 9' })
  ])
}

const ModelingIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z' }),
    h('polyline', { points: '3.27 6.96 12 12.01 20.73 6.96' }),
    h('line', { x1: '12', y1: '22.08', x2: '12', y2: '12' })
  ])
}

const CodeIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('polyline', { points: '16 18 22 12 16 6' }),
    h('polyline', { points: '8 6 2 12 8 18' })
  ])
}

const SimulationIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '3' }),
    h('path', { d: 'M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z' })
  ])
}

const FatigueIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M22 12h-4l-3 9L9 3l-3 9H2' })
  ])
}

const TransientIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('polygon', { points: '13 2 3 14 12 14 11 22 21 10 12 10 13 2' })
  ])
}

const ExplicitIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '10' }),
    h('line', { x1: '12', y1: '8', x2: '12', y2: '12' }),
    h('line', { x1: '12', y1: '16', x2: '12.01', y2: '16' })
  ])
}

const CFDIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z' })
  ])
}

const ThermalIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z' })
  ])
}

const TopologyIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '3', y: '3', width: '7', height: '7' }),
    h('rect', { x: '14', y: '3', width: '7', height: '7' }),
    h('rect', { x: '14', y: '14', width: '7', height: '7' }),
    h('rect', { x: '3', y: '14', width: '7', height: '7' })
  ])
}

const ThermalFluidIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z' }),
    h('path', { d: 'M8 14s1.5 2 4 2 4-2 4-2' })
  ])
}

const CohesiveIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M16 3h5v5' }),
    h('path', { d: 'M4 20L21 3' }),
    h('path', { d: 'M21 16v5h-5' }),
    h('path', { d: 'M15 15l6 6' }),
    h('path', { d: 'M4 4l5 5' })
  ])
}

const XfemIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2v-4M9 21H5a2 2 0 0 1-2-2v-4m0-4h18' }),
    h('path', { d: 'M14 9l-4 6' })
  ])
}

const GeometryRepairIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z' })
  ])
}

const AdvancedMeshIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('polygon', { points: '12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2' }),
    h('line', { x1: '12', y1: '22', x2: '12', y2: '15.5' }),
    h('polyline', { points: '22 8.5 12 15.5 2 8.5' }),
    h('polyline', { points: '2 15.5 12 8.5 22 15.5' }),
    h('line', { x1: '12', y1: '2', x2: '12', y2: '8.5' })
  ])
}

const MeshRefinementIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '10' }),
    h('circle', { cx: '12', cy: '12', r: '6' }),
    h('circle', { cx: '12', cy: '12', r: '2' }),
    h('line', { x1: '12', y1: '2', x2: '12', y2: '6' }),
    h('line', { x1: '12', y1: '18', x2: '12', y2: '22' }),
    h('line', { x1: '2', y1: '12', x2: '6', y2: '12' }),
    h('line', { x1: '18', y1: '12', x2: '22', y2: '12' })
  ])
}

const AcousticIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M9 18V5l12-2v13' }),
    h('circle', { cx: '6', cy: '18', r: '3' }),
    h('circle', { cx: '18', cy: '16', r: '3' })
  ])
}

const CompositeIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '3', y: '3', width: '18', height: '18', rx: '2' }),
    h('line', { x1: '3', y1: '9', x2: '21', y2: '9' }),
    h('line', { x1: '3', y1: '15', x2: '21', y2: '15' }),
    h('line', { x1: '9', y1: '3', x2: '9', y2: '21' }),
    h('line', { x1: '15', y1: '3', x2: '15', y2: '21' })
  ])
}

const CreepIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M22 12h-4l-3 9L9 3l-3 9H2' }),
    h('path', { d: 'M17 3v4' }),
    h('path', { d: 'M21 3h-4' })
  ])
}

const CloudIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z' })
  ])
}

const MultiscaleIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '10' }),
    h('circle', { cx: '12', cy: '12', r: '6' }),
    h('circle', { cx: '12', cy: '12', r: '2' })
  ])
}

const DataIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('ellipse', { cx: '12', cy: '5', rx: '9', ry: '3' }),
    h('path', { d: 'M21 12c0 1.66-4 3-9 3s-9-1.34-9-3' }),
    h('path', { d: 'M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5' })
  ])
}

const CertIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 15l-2 5l2-1l2 1l-2-5z' }),
    h('circle', { cx: '12', cy: '9', r: '6' }),
    h('path', { d: 'M9 9l2 2l4-4' })
  ])
}

const MdIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '7', cy: '7', r: '2' }),
    h('circle', { cx: '17', cy: '7', r: '2' }),
    h('circle', { cx: '12', cy: '17', r: '2' }),
    h('line', { x1: '8.5', y1: '8.5', x2: '10.5', y2: '15.5' }),
    h('line', { x1: '15.5', y1: '8.5', x2: '13.5', y2: '15.5' }),
    h('line', { x1: '9', y1: '7', x2: '15', y2: '7' })
  ])
}

const AtomIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('ellipse', { cx: '12', cy: '12', rx: '10', ry: '4' }),
    h('ellipse', { cx: '12', cy: '12', rx: '10', ry: '4', transform: 'rotate(60 12 12)' }),
    h('ellipse', { cx: '12', cy: '12', rx: '10', ry: '4', transform: 'rotate(-60 12 12)' }),
    h('circle', { cx: '12', cy: '12', r: '1.5', fill: 'currentColor' })
  ])
}

const PhaseFieldIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2z' }),
    h('path', { d: 'M12 2c3 3.5 3 8.5 0 10' }),
    h('path', { d: 'M12 2c-3 3.5-3 8.5 0 10' }),
    h('path', { d: 'M2 12c3.5-3 8.5-3 10 0' }),
    h('path', { d: 'M2 12c3.5 3 8.5 3 10 0' })
  ])
}

const DftIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '3' }),
    h('ellipse', { cx: '12', cy: '12', rx: '10', ry: '4' }),
    h('ellipse', { cx: '12', cy: '12', rx: '10', ry: '4', transform: 'rotate(60 12 12)' }),
    h('ellipse', { cx: '12', cy: '12', rx: '10', ry: '4', transform: 'rotate(-60 12 12)' })
  ])
}

const OntologyIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M4 19.5A2.5 2.5 0 0 1 6.5 17H20' }),
    h('path', { d: 'M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z' }),
    h('line', { x1: '8', y1: '7', x2: '16', y2: '7' }),
    h('line', { x1: '8', y1: '11', x2: '14', y2: '11' })
  ])
}

const CoordMapIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '10' }),
    h('line', { x1: '2', y1: '12', x2: '22', y2: '12' }),
    h('line', { x1: '12', y1: '2', x2: '12', y2: '22' }),
    h('path', { d: 'M12 8l4 4-4 4' })
  ])
}

const CoarseGrainIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '6', cy: '6', r: '1.5' }),
    h('circle', { cx: '12', cy: '4', r: '1.5' }),
    h('circle', { cx: '18', cy: '7', r: '1.5' }),
    h('circle', { cx: '8', cy: '12', r: '1.5' }),
    h('circle', { cx: '16', cy: '13', r: '1.5' }),
    h('circle', { cx: '5', cy: '18', r: '1.5' }),
    h('circle', { cx: '14', cy: '19', r: '1.5' }),
    h('rect', { x: '3', y: '3', width: '18', height: '18', rx: '2', strokeDasharray: '3 3' })
  ])
}

const ErrorTrackIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M22 12h-4l-3 9L9 3l-3 9H2' }),
    h('line', { x1: '18', y1: '2', x2: '18', y2: '6' }),
    h('line', { x1: '16', y1: '4', x2: '20', y2: '4' })
  ])
}

const BenchmarkIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 15l-2 5l2-1l2 1l-2-5z' }),
    h('circle', { cx: '12', cy: '9', r: '6' }),
    h('path', { d: 'M9 9l2 2l4-4' })
  ])
}

const AuditIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '3', y: '3', width: '18', height: '18', rx: '2' }),
    h('path', { d: 'M3 9h18' }),
    h('path', { d: 'M9 21V9' }),
    h('circle', { cx: '15', cy: '15', r: '2' })
  ])
}

const CrossScaleIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '8', cy: '8', r: '3' }),
    h('circle', { cx: '16', cy: '16', r: '3' }),
    h('line', { x1: '10.5', y1: '10.5', x2: '13.5', y2: '13.5' }),
    h('circle', { cx: '8', cy: '8', r: '6', strokeDasharray: '2 2' }),
    h('circle', { cx: '16', cy: '16', r: '6', strokeDasharray: '2 2' })
  ])
}

const IntegrationIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '1', y: '6', width: '6', height: '6', rx: '1' }),
    h('rect', { x: '9', y: '6', width: '6', height: '6', rx: '1' }),
    h('rect', { x: '17', y: '6', width: '6', height: '6', rx: '1' }),
    h('path', { d: 'M7 9h2M15 9h2' }),
    h('rect', { x: '5', y: '16', width: '14', height: '4', rx: '1' }),
    h('path', { d: 'M12 12v4' })
  ])
}

const WorkflowIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z' }),
    h('polyline', { points: '14 2 14 8 20 8' }),
    h('line', { x1: '8', y1: '13', x2: '16', y2: '13' }),
    h('line', { x1: '8', y1: '17', x2: '16', y2: '17' }),
    h('polyline', { points: '10 9 9 9 8 9' })
  ])
}

const HighThroughputIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '3', y: '3', width: '18', height: '18', rx: '2' }),
    h('line', { x1: '3', y1: '9', x2: '21', y2: '9' }),
    h('line', { x1: '3', y1: '15', x2: '21', y2: '15' }),
    h('line', { x1: '9', y1: '3', x2: '9', y2: '21' }),
    h('line', { x1: '15', y1: '3', x2: '15', y2: '21' }),
    h('circle', { cx: '6', cy: '6', r: '1', fill: 'currentColor' }),
    h('circle', { cx: '12', cy: '12', r: '1', fill: 'currentColor' }),
    h('circle', { cx: '18', cy: '18', r: '1', fill: 'currentColor' })
  ])
}

const AiRecommendIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M12 2a5 5 0 0 1 5 5v3a5 5 0 0 1-10 0V7a5 5 0 0 1 5-5z' }),
    h('path', { d: 'M8 14s1.5 2 4 2 4-2 4-2' }),
    h('path', { d: 'M12 18v4' }),
    h('path', { d: 'M8 22h8' }),
    h('circle', { cx: '9', cy: '7', r: '0.5', fill: 'currentColor' }),
    h('circle', { cx: '15', cy: '7', r: '0.5', fill: 'currentColor' })
  ])
}

const NightlyIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('path', { d: 'M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z' }),
    h('path', { d: 'M14 9l-2 2 2 2' }),
    h('path', { d: 'M18 9l-2 2 2 2' })
  ])
}

const WorkspaceIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('rect', { x: '2', y: '3', width: '20', height: '14', rx: '2' }),
    h('line', { x1: '8', y1: '21', x2: '16', y2: '21' }),
    h('line', { x1: '12', y1: '17', x2: '12', y2: '21' }),
    h('rect', { x: '5', y: '6', width: '5', height: '4', rx: '0.5' }),
    h('rect', { x: '14', y: '6', width: '5', height: '4', rx: '0.5' })
  ])
}

const SettingsIcon = {
  render: () => h('svg', { class: 'nav-icon', viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', 'stroke-width': '1.5' }, [
    h('circle', { cx: '12', cy: '12', r: '3' }),
    h('path', { d: 'M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z' })
  ])
}

const mainNavItems: Array<{
  path: string
  icon: any
  label: string
  badge?: string
}> = [
  { path: '/', icon: HomeIcon, label: '首页' },
  { path: '/notes', icon: NotesIcon, label: '笔记' },
  { path: '/modeling', icon: ModelingIcon, label: '建模' },
  { path: '/code', icon: CodeIcon, label: '代码' },
  { path: '/simulation', icon: SimulationIcon, label: '仿真' },
  { path: '/fatigue', icon: FatigueIcon, label: '疲劳' },
  { path: '/transient', icon: TransientIcon, label: '瞬态' },
  { path: '/explicit', icon: ExplicitIcon, label: '显式' },
  { path: '/cfd', icon: CFDIcon, label: 'CFD' },
  { path: '/thermal', icon: ThermalIcon, label: '热耦合' },
  { path: '/thermal-fluid', icon: ThermalFluidIcon, label: '热流' },
  { path: '/cohesive', icon: CohesiveIcon, label: '内聚力' },
  { path: '/xfem', icon: XfemIcon, label: 'XFEM' },
  { path: '/geometry-repair', icon: GeometryRepairIcon, label: '几何修复' },
  { path: '/advanced-mesh', icon: AdvancedMeshIcon, label: '高级网格' },
  { path: '/mesh-refinement', icon: MeshRefinementIcon, label: '网格加密' },
  { path: '/acoustic', icon: AcousticIcon, label: '声学' },
  { path: '/composite', icon: CompositeIcon, label: '复合' },
  { path: '/creep', icon: CreepIcon, label: '蠕变' },
  { path: '/cloud-hpc', icon: CloudIcon, label: '云HPC' },
  { path: '/multiscale', icon: MultiscaleIcon, label: '多尺度' },
  { path: '/data-asset', icon: DataIcon, label: '数据' },
  { path: '/certification', icon: CertIcon, label: '认证' },
  { path: '/md', icon: MdIcon, label: 'MD' },
  { path: '/atom-builder', icon: AtomIcon, label: '原子建模' },
  { path: '/phase-field', icon: PhaseFieldIcon, label: '相场' },
  { path: '/dft-input', icon: DftIcon, label: 'DFT' },
  { path: '/ontology', icon: OntologyIcon, label: '本体' },
  { path: '/coordinate-mapping', icon: CoordMapIcon, label: '坐标映射' },
  { path: '/coarse-graining', icon: CoarseGrainIcon, label: '粗粒化' },
  { path: '/error-tracking', icon: ErrorTrackIcon, label: '误差追踪' },
  { path: '/benchmark', icon: BenchmarkIcon, label: '算例库' },
  { path: '/audit-log', icon: AuditIcon, label: '审计' },
  { path: '/cross-scale-viz', icon: CrossScaleIcon, label: '跨尺度可视化' },
  { path: '/integration', icon: IntegrationIcon, label: '集成测试' },
  { path: '/workflow-template', icon: WorkflowIcon, label: '模板' },
  { path: '/high-throughput', icon: HighThroughputIcon, label: '高通量' },
  { path: '/ai-recommend', icon: AiRecommendIcon, label: 'AI推荐' },
  { path: '/nightly-ci', icon: NightlyIcon, label: 'Nightly' },
  { path: '/multiscale-workspace', icon: WorkspaceIcon, label: '工作区' },
  { path: '/topology', icon: TopologyIcon, label: '拓扑' }
]

function isActive(path: string): boolean {
  if (path === '/') {
    return route.path === '/'
  }
  return route.path.startsWith(path)
}
</script>

<style scoped>
.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 52px;
  height: 52px;
  border-radius: var(--radius-md);
  color: var(--text-muted);
  text-decoration: none;
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  position: relative;
}

.nav-item:hover {
  background-color: var(--bg-elevated);
  color: var(--text-secondary);
}

.nav-item.active {
  background-color: var(--primary-glow);
  color: var(--primary);
}

.nav-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background-color: var(--primary);
  border-radius: 0 3px 3px 0;
}

.nav-icon {
  width: 20px;
  height: 20px;
  margin-bottom: 2px;
}

.nav-label {
  font-size: 9px;
  font-weight: 500;
  letter-spacing: 0.2px;
}

.nav-badge {
  position: absolute;
  top: 6px;
  right: 8px;
  min-width: 14px;
  height: 14px;
  padding: 0 4px;
  background: var(--accent-red);
  color: white;
  font-size: 8px;
  font-weight: 600;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
