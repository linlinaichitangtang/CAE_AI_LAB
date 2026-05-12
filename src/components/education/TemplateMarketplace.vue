<script setup lang="ts">
/**
 * TemplateMarketplace.vue — V3.0-003 模板市场
 * 老师发布模板，学生/其他用户浏览、搜索、评分、克隆
 */
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import {
  type ProjectTemplate,
  getPublishedTemplates,
  getTemplates,
  cloneTemplate,
  saveProjectAsTemplate,
  publishTemplate,
  unpublishTemplate
} from '@/api/share'

// ============ 状态 ============
const router = useRouter()
const templates = ref<ProjectTemplate[]>([])
const searchQuery = ref('')
const selectedCategory = ref('all')
const sortBy = ref<'clones' | 'recent' | 'rating'>('clones')
const currentUser = ref(localStorage.getItem('caelab_username') || '匿名用户')

// 评分状态
const templateRatings = ref<Record<string, number>>({})  // templateId -> rating (1-5)
const userRatedTemplates = ref<Set<string>>(new Set())

// 模态框状态
const showDetailDialog = ref(false)
const selectedTemplate = ref<ProjectTemplate | null>(null)
const showCreateTemplateDialog = ref(false)
const isCloning = ref(false)

// 创建模板表单
const newTemplateForm = ref({
  name: '',
  description: '',
  category: 'default',
  projectId: '',
  usageInstructions: ''
})

// ============ 常量 ============
const CATEGORIES = [
  { id: 'all', name: '全部', icon: '🌐' },
  { id: 'structure', name: '结构力学', icon: '🏗️' },
  { id: 'thermal', name: '热分析', icon: '🔥' },
  { id: 'cfd', name: '流体力学', icon: '💨' },
  { id: 'modal', name: '模态分析', icon: '📳' },
  { id: 'buckling', name: '屈曲分析', icon: '📉' },
  { id: 'fatigue', name: '疲劳分析', icon: '🔄' },
  { id: 'multiscale', name: '多尺度', icon: '🔬' },
  { id: 'education', name: '教学模板', icon: '📚' },
  { id: 'other', name: '其他', icon: '📦' }
]

// ============ 计算属性 ============
const filteredTemplates = computed(() => {
  let result = [...templates.value]

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(t =>
      t.name.toLowerCase().includes(query) ||
      t.description?.toLowerCase().includes(query) ||
      t.author?.toLowerCase().includes(query)
    )
  }

  // 分类过滤
  if (selectedCategory.value !== 'all') {
    result = result.filter(t => t.category === selectedCategory.value)
  }

  // 排序
  switch (sortBy.value) {
    case 'clones':
      result.sort((a, b) => (b.cloneCount || 0) - (a.cloneCount || 0))
      break
    case 'recent':
      result.sort((a, b) => new Date(b.createdAt || 0).getTime() - new Date(a.createdAt || 0).getTime())
      break
    case 'rating':
      result.sort((a, b) => getAverageRating(b) - getAverageRating(a))
      break
  }

  return result
})

// ============ 工具函数 ============
function loadTemplates() {
  // 合并本地模板和已发布模板
  const localTemplates = getTemplates()
  const publishedTemplates = getPublishedTemplates()

  // 合并去重
  const merged = [...publishedTemplates]
  for (const local of localTemplates) {
    if (!merged.find(t => t.id === local.id)) {
      merged.push(local)
    }
  }

  templates.value = merged
}

function getAverageRating(template: ProjectTemplate): number {
  const ratings = templateRatings.value[template.id]
  if (!ratings) return 0
  return ratings
}

function getRatingStars(rating: number): string {
  const full = Math.floor(rating)
  const half = rating - full >= 0.5 ? 1 : 0
  const empty = 5 - full - half
  return '★'.repeat(full) + (half ? '½' : '') + '☆'.repeat(empty)
}

function formatDate(dateStr?: string): string {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

function formatNumber(n?: number): string {
  if (!n) return '0'
  if (n >= 1000) return (n / 1000).toFixed(1) + 'K'
  return n.toString()
}

// ============ 操作函数 ============
function openTemplateDetail(template: ProjectTemplate) {
  selectedTemplate.value = template
  showDetailDialog.value = true
}

function closeDetailDialog() {
  showDetailDialog.value = false
  selectedTemplate.value = null
}

async function handleClone(template: ProjectTemplate) {
  isCloning.value = true
  try {
    const newProject = await cloneTemplate(template.id, `${template.name} (克隆)`)
    alert(`模板「${template.name}」克隆成功！`)
    // 增加克隆计数（如果是在市场克隆）
    template.cloneCount = (template.cloneCount || 0) + 1
    closeDetailDialog()
    // 跳转到新项目
    router.push({ path: '/notes', query: { projectId: newProject.id } })
  } catch (e: any) {
    alert(`克隆失败: ${e.message || e}`)
  } finally {
    isCloning.value = false
  }
}

function handleRate(template: ProjectTemplate, rating: number) {
  if (userRatedTemplates.value.has(template.id)) {
    // 已经评过，更新
    templateRatings.value[template.id] = rating
  } else {
    // 新评分
    templateRatings.value[template.id] = rating
    userRatedTemplates.value.add(template.id)
  }
}

function handleTogglePublish(template: ProjectTemplate) {
  if (template.published) {
    unpublishTemplate(template.id)
    template.published = false
  } else {
    publishTemplate(template.id)
    template.published = true
  }
  loadTemplates()
}

// ============ 生命周期 ============
onMounted(() => {
  loadTemplates()
  // 加载本地存储的评分
  try {
    const stored = localStorage.getItem('caelab_template_ratings')
    if (stored) {
      templateRatings.value = JSON.parse(stored)
    }
    const rated = localStorage.getItem('caelab_user_rated_templates')
    if (rated) {
      userRatedTemplates.value = new Set(JSON.parse(rated))
    }
  } catch {}
})

// 保存评分到本地
watch(templateRatings, (val) => {
  localStorage.setItem('caelab_template_ratings', JSON.stringify(val))
  localStorage.setItem('caelab_user_rated_templates', JSON.stringify([...userRatedTemplates.value]))
}, { deep: true })
</script>

<template>
  <div class="template-marketplace h-full flex flex-col bg-[var(--bg-ground)]">
    <!-- 顶部栏 -->
    <div class="bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] px-6 py-4">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h1 class="text-xl font-bold text-[var(--text-primary)] flex items-center gap-2">
            <span>🛒</span>
            <span>模板市场</span>
          </h1>
          <p class="text-sm text-[var(--text-muted)] mt-1">
            发现和分享高质量仿真模板
          </p>
        </div>
        <div class="flex gap-2">
          <button
            @click="loadTemplates"
            class="px-3 py-1.5 border border-[var(--border-subtle)] rounded text-sm hover:bg-[var(--bg-hover)]"
          >
            🔄 刷新
          </button>
        </div>
      </div>

      <!-- 搜索和过滤 -->
      <div class="flex flex-wrap gap-3">
        <!-- 搜索框 -->
        <div class="flex-1 min-w-[200px]">
          <input
            v-model="searchQuery"
            type="text"
            class="input w-full"
            placeholder="搜索模板名称、描述或作者..."
          />
        </div>

        <!-- 排序 -->
        <select v-model="sortBy" class="input w-auto">
          <option value="clones">🔥 最受欢迎</option>
          <option value="recent">🕐 最新发布</option>
          <option value="rating">⭐ 评分最高</option>
        </select>
      </div>

      <!-- 分类标签 -->
      <div class="flex flex-wrap gap-2 mt-3">
        <button
          v-for="cat in CATEGORIES"
          :key="cat.id"
          @click="selectedCategory = cat.id"
          class="px-3 py-1.5 rounded-full text-xs font-medium transition-colors"
          :class="selectedCategory === cat.id
            ? 'bg-[var(--primary)] text-white'
            : 'bg-[var(--bg-elevated)] text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'"
        >
          {{ cat.icon }} {{ cat.name }}
        </button>
      </div>
    </div>

    <!-- 模板列表 -->
    <div class="flex-1 overflow-y-auto p-6">
      <div v-if="filteredTemplates.length === 0" class="text-center py-16">
        <div class="text-6xl mb-4 opacity-30">🔍</div>
        <p class="text-lg text-[var(--text-muted)]">暂无模板</p>
        <p class="text-sm mt-1">
          {{ searchQuery || selectedCategory !== 'all' ? '尝试调整搜索条件' : '成为第一个发布模板的人！' }}
        </p>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        <div
          v-for="template in filteredTemplates"
          :key="template.id"
          class="bg-[var(--bg-surface)] rounded-xl border border-[var(--border-subtle)] overflow-hidden hover:shadow-lg transition-shadow cursor-pointer group"
          @click="openTemplateDetail(template)"
        >
          <!-- 卡片头部 -->
          <div class="p-4 bg-gradient-to-br from-[var(--bg-elevated)] to-[var(--bg-surface)]">
            <div class="flex items-start justify-between">
              <div class="w-12 h-12 rounded-xl bg-[var(--primary-glow)] flex items-center justify-center text-2xl">
                {{ template.icon || '📋' }}
              </div>
              <div v-if="template.published" class="px-2 py-0.5 bg-green-100 text-green-700 text-xs rounded-full">
                🌍 已发布
              </div>
            </div>
            <h3 class="font-semibold text-[var(--text-primary)] mt-3 line-clamp-1">{{ template.name }}</h3>
            <p class="text-xs text-[var(--text-muted)] mt-1 line-clamp-2">{{ template.description || '暂无描述' }}</p>
          </div>

          <!-- 卡片信息 -->
          <div class="px-4 py-3 border-t border-[var(--border-subtle)]">
            <div class="flex items-center justify-between text-xs text-[var(--text-muted)]">
              <div class="flex items-center gap-3">
                <span>👤 {{ template.author || '匿名' }}</span>
                <span>📅 {{ formatDate(template.createdAt) }}</span>
              </div>
            </div>
            <div class="flex items-center justify-between mt-2">
              <div class="flex items-center gap-2">
                <span class="text-xs text-[var(--text-muted)]">
                  ⬇️ {{ formatNumber(template.cloneCount) }} 次克隆
                </span>
                <span v-if="template.usageInstructions" class="text-xs text-[var(--primary)]">
                  📖 有说明
                </span>
              </div>
              <div class="flex items-center gap-1">
                <span class="text-xs" :class="getAverageRating(template) > 0 ? 'text-yellow-500' : 'text-gray-300'">
                  {{ getRatingStars(getAverageRating(template)) }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- ========== 模板详情弹窗 ========== -->
    <div v-if="showDetailDialog && selectedTemplate" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="closeDetailDialog">
      <div class="bg-[var(--bg-surface)] rounded-xl w-full max-w-2xl max-h-[85vh] overflow-hidden shadow-2xl">
        <!-- 弹窗头部 -->
        <div class="px-6 py-4 border-b border-[var(--border-subtle)] flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-xl bg-[var(--primary-glow)] flex items-center justify-center text-2xl">
              {{ selectedTemplate.icon || '📋' }}
            </div>
            <div>
              <h2 class="text-lg font-bold text-[var(--text-primary)]">{{ selectedTemplate.name }}</h2>
              <p class="text-sm text-[var(--text-muted)]">by {{ selectedTemplate.author || '匿名' }}</p>
            </div>
          </div>
          <button @click="closeDetailDialog" class="text-2xl text-[var(--text-muted)] hover:text-[var(--text-primary)]">&times;</button>
        </div>

        <!-- 弹窗内容 -->
        <div class="flex-1 overflow-y-auto p-6">
          <!-- 描述 -->
          <div class="mb-6">
            <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-2">描述</h3>
            <p class="text-sm text-[var(--text-primary)]">{{ selectedTemplate.description || '暂无描述' }}</p>
          </div>

          <!-- 统计 -->
          <div class="grid grid-cols-3 gap-4 mb-6">
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold text-[var(--primary)]">{{ formatNumber(selectedTemplate.cloneCount) }}</p>
              <p class="text-xs text-[var(--text-muted)]">克隆次数</p>
            </div>
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold text-yellow-500">{{ getAverageRating(selectedTemplate).toFixed(1) }}</p>
              <p class="text-xs text-[var(--text-muted)]">平均评分</p>
            </div>
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-center">
              <p class="text-2xl font-bold text-blue-500">{{ CATEGORIES.find(c => c.id === selectedTemplate.category)?.icon || '📦' }}</p>
              <p class="text-xs text-[var(--text-muted)]">{{ CATEGORIES.find(c => c.id === selectedTemplate.category)?.name || '其他' }}</p>
            </div>
          </div>

          <!-- 使用说明 -->
          <div v-if="selectedTemplate.usageInstructions" class="mb-6">
            <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-2">使用说明</h3>
            <div class="bg-[var(--bg-elevated)] rounded-lg p-3 text-sm text-[var(--text-secondary)]">
              {{ selectedTemplate.usageInstructions }}
            </div>
          </div>

          <!-- 评分 -->
          <div class="mb-6">
            <h3 class="text-sm font-medium text-[var(--text-secondary)] mb-2">评分</h3>
            <div class="flex items-center gap-2">
              <button
                v-for="star in [1, 2, 3, 4, 5]"
                :key="star"
                @click="handleRate(selectedTemplate, star)"
                class="text-2xl transition-transform hover:scale-110"
                :class="star <= getAverageRating(selectedTemplate) ? 'text-yellow-400' : 'text-gray-300'"
              >
                ★
              </button>
              <span class="text-sm text-[var(--text-muted)] ml-2">
                {{ userRatedTemplates.has(selectedTemplate.id) ? '已评分' : '点击评分' }}
              </span>
            </div>
          </div>

          <!-- 发布时间 -->
          <div class="text-xs text-[var(--text-muted)]">
            发布于 {{ formatDate(selectedTemplate.createdAt) }}
          </div>
        </div>

        <!-- 弹窗底部 -->
        <div class="px-6 py-4 border-t border-[var(--border-subtle)] flex justify-between gap-3">
          <button
            v-if="selectedTemplate.author === currentUser"
            @click="handleTogglePublish(selectedTemplate)"
            class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)]"
          >
            {{ selectedTemplate.published ? '📛 取消发布' : '🌍 发布模板' }}
          </button>
          <div class="flex-1"></div>
          <button @click="closeDetailDialog" class="px-4 py-2 border border-[var(--border-subtle)] rounded-lg text-sm hover:bg-[var(--bg-hover)]">
            关闭
          </button>
          <button
            @click="handleClone(selectedTemplate)"
            :disabled="isCloning"
            class="px-4 py-2 bg-[var(--primary)] text-white rounded-lg text-sm hover:opacity-90 disabled:opacity-50"
          >
            {{ isCloning ? '克隆中...' : '⬇️ 克隆模板' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>