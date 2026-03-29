<template>
  <div class="p-6">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">欢迎使用 CAELab</h2>
    <p class="text-gray-600 mb-6">科研与工程创作一体化工作台</p>
    
    <div class="grid grid-cols-3 gap-4">
      <div v-for="module in modules" :key="module.path" 
           class="p-4 bg-white rounded-lg shadow hover:shadow-md transition-shadow cursor-pointer"
           @click="$router.push(module.path)">
        <div class="text-3xl mb-2">{{ module.icon }}</div>
        <h3 class="font-semibold text-gray-800">{{ module.name }}</h3>
        <p class="text-sm text-gray-500 mt-1">{{ module.desc }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const modules = [
  { name: '笔记创作', icon: '📝', path: '/notes', desc: '富文本编辑、LaTeX公式、手写输入' },
  { name: '3D建模', icon: '🎨', path: '/modeling', desc: '几何建模、外部模型导入' },
  { name: '代码编辑', icon: '💻', path: '/code', desc: '语法高亮、代码运行' },
  { name: '仿真分析', icon: '🔬', path: '/simulation', desc: '结构力学仿真、结果后处理' }
]
</script>