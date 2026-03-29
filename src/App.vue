<template>
  <div id="app" class="h-screen flex flex-col bg-gray-50">
    <TopBar />
    <div class="flex-1 flex overflow-hidden">
      <LeftNav />
      <main class="flex-1 overflow-auto">
        <router-view />
      </main>
    </div>
    <StatusBar />
  </div>
</template>

<script setup lang="ts">
import TopBar from './components/layout/TopBar.vue'
import LeftNav from './components/layout/LeftNav.vue'
import StatusBar from './components/layout/StatusBar.vue'
</script>