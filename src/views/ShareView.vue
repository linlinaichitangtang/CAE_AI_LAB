<template>
  <div class="h-full flex flex-col p-6 overflow-auto">
    <div v-if="loading" class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <div class="w-12 h-12 border-4 border-[var(--primary)] border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
        <p class="text-[var(--text-secondary)]">正在导入项目...</p>
      </div>
    </div>

    <div v-else-if="error" class="flex-1 flex items-center justify-center">
      <div class="text-center max-w-md">
        <div class="w-16 h-16 rounded-xl bg-[var(--accent-red)]/20 flex items-center justify-center mx-auto mb-4">
          <span class="text-3xl">❌</span>
        </div>
        <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">导入失败</h2>
        <p class="text-[var(--text-secondary)] mb-6">{{ error }}</p>
        <button @click="goHome" class="btn btn-primary">
          返回首页
        </button>
      </div>
    </div>

    <div v-else-if="success" class="flex-1 flex items-center justify-center">
      <div class="text-center max-w-md">
        <div class="w-16 h-16 rounded-xl bg-[var(--primary)]/20 flex items-center justify-center mx-auto mb-4">
          <span class="text-3xl">✅</span>
        </div>
        <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">项目导入成功</h2>
        <p class="text-[var(--text-secondary)] mb-6">「{{ projectName }}」已成功导入，现在开始你的创作之旅吧！</p>
        <div class="flex gap-3 justify-center">
          <button @click="goToNotes" class="btn btn-primary">
            前往笔记
          </button>
          <button @click="goHome" class="btn btn-ghost">
            返回首页
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { parseShareLink } from '@/api/share'

const router = useRouter()
const route = useRoute()

const loading = ref(true)
const error = ref('')
const success = ref(false)
const projectName = ref('')

onMounted(async () => {
  const data = route.query.data as string
  
  if (!data) {
    error.value = '无效的分享链接'
    loading.value = false
    return
  }
  
  const projectId = parseShareLink(`${window.location.origin}/share?data=${data}`)
  
  if (!projectId) {
    error.value = '无效的分享链接格式'
    loading.value = false
    return
  }
  
  // For now, we need a different approach since share links encode project ID
  // but we need actual project data. In a real app, this would fetch from server.
  // For this implementation, we'll show a message
  error.value = '分享链接功能需要后端服务器支持存储项目数据。当前仅支持本地导出/导入。'
  loading.value = false
})

function goHome() {
  router.push('/')
}

function goToNotes() {
  router.push('/notes')
}
</script>
