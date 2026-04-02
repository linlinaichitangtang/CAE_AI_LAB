<template>
  <div class="relative w-full h-full flex items-center justify-center overflow-hidden">
    <!-- Rotating wireframe cube -->
    <div class="wireframe-cube-container">
      <svg
        class="wireframe-cube"
        :width="size"
        :height="size"
        viewBox="0 0 200 200"
        xmlns="http://www.w3.org/2000/svg"
      >
        <defs>
          <linearGradient id="wireGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" style="stop-color: var(--primary, #3B82F6); stop-opacity: 0.8" />
            <stop offset="100%" style="stop-color: var(--primary, #3B82F6); stop-opacity: 0.2" />
          </linearGradient>
          <filter id="glow">
            <feGaussianBlur stdDeviation="2" result="coloredBlur" />
            <feMerge>
              <feMergeNode in="coloredBlur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>
        <!-- Back face -->
        <polygon
          class="cube-face cube-back"
          points="60,40 140,40 160,70 80,70"
          fill="none"
          stroke="url(#wireGrad)"
          stroke-width="1.5"
          filter="url(#glow)"
        />
        <polygon
          class="cube-face cube-back"
          points="80,70 160,70 160,130 80,130"
          fill="none"
          stroke="url(#wireGrad)"
          stroke-width="1.5"
          filter="url(#glow)"
        />
        <polygon
          class="cube-face cube-back"
          points="60,40 80,70 80,130 60,100"
          fill="none"
          stroke="url(#wireGrad)"
          stroke-width="1.5"
          filter="url(#glow)"
        />
        <!-- Front face -->
        <polygon
          class="cube-face cube-front"
          points="40,70 120,70 140,100 60,100"
          fill="none"
          stroke="url(#wireGrad)"
          stroke-width="2"
          filter="url(#glow)"
        />
        <polygon
          class="cube-face cube-front"
          points="60,100 140,100 140,160 60,160"
          fill="none"
          stroke="url(#wireGrad)"
          stroke-width="2"
          filter="url(#glow)"
        />
        <polygon
          class="cube-face cube-front"
          points="40,70 60,100 60,160 40,130"
          fill="none"
          stroke="url(#wireGrad)"
          stroke-width="2"
          filter="url(#glow)"
        />
        <!-- Connecting edges -->
        <line class="cube-edge" x1="60" y1="40" x2="40" y2="70" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="140" y1="40" x2="120" y2="70" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="160" y1="70" x2="140" y2="100" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="80" y1="70" x2="60" y2="100" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="80" y1="130" x2="60" y2="160" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="160" y1="130" x2="140" y2="160" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="60" y1="100" x2="60" y2="160" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
        <line class="cube-edge" x1="140" y1="100" x2="140" y2="160" stroke="url(#wireGrad)" stroke-width="1" opacity="0.5" />
      </svg>
    </div>
    <!-- Glow overlay -->
    <div class="absolute inset-0 pointer-events-none wireframe-glow"></div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  size?: number
}>(), {
  size: 200
})
</script>

<style scoped>
.wireframe-cube-container {
  animation: cube-rotate 8s linear infinite;
  perspective: 600px;
}

.wireframe-cube {
  filter: drop-shadow(0 0 8px var(--primary, #3B82F6));
}

.cube-face {
  transition: opacity 0.3s;
}

.cube-back {
  opacity: 0.4;
}

.cube-front {
  opacity: 0.9;
}

.cube-edge {
  animation: edge-pulse 3s ease-in-out infinite;
}

@keyframes cube-rotate {
  0% {
    transform: rotateY(0deg) rotateX(15deg);
  }
  25% {
    transform: rotateY(90deg) rotateX(5deg);
  }
  50% {
    transform: rotateY(180deg) rotateX(-5deg);
  }
  75% {
    transform: rotateY(270deg) rotateX(5deg);
  }
  100% {
    transform: rotateY(360deg) rotateX(15deg);
  }
}

@keyframes edge-pulse {
  0%, 100% {
    opacity: 0.3;
  }
  50% {
    opacity: 0.7;
  }
}

.wireframe-glow {
  background: radial-gradient(
    ellipse at center,
    color-mix(in srgb, var(--primary, #3B82F6) 10%, transparent) 0%,
    transparent 70%
  );
  animation: glow-pulse 4s ease-in-out infinite;
}

@keyframes glow-pulse {
  0%, 100% {
    opacity: 0.3;
  }
  50% {
    opacity: 0.8;
  }
}

/* Dark mode adjustments */
:deep(.dark) .wireframe-cube {
  filter: drop-shadow(0 0 12px var(--primary, #3B82F6));
}

:deep(.dark) .wireframe-glow {
  background: radial-gradient(
    ellipse at center,
    color-mix(in srgb, var(--primary, #3B82F6) 15%, transparent) 0%,
    transparent 70%
  );
}
</style>
