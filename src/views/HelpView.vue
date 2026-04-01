<template>
  <div class="h-full flex flex-col bg-[var(--bg-base)]">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 bg-[var(--bg-surface)] border-b border-[var(--border-default)]">
      <div>
        <h1 class="text-xl font-semibold text-[var(--text-primary)] flex items-center gap-2">
          <svg class="w-5 h-5 text-[var(--primary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/>
            <line x1="12" y1="17" x2="12.01" y2="17"/>
          </svg>
          帮助文档
        </h1>
        <p class="text-sm text-[var(--text-muted)] mt-0.5">CAELab 使用指南和功能文档</p>
      </div>
      <div class="flex items-center gap-2">
        <button
          @click="showOnboarding = true"
          class="px-3 py-1.5 text-sm border border-[var(--border-default)] rounded-md text-[var(--text-secondary)] hover:bg-[var(--bg-elevated)] transition flex items-center gap-1.5"
        >
          <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <polyline points="10 9 9 9 8 9"/>
          </svg>
          重新显示引导
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Sidebar: Categories & Search -->
      <div class="w-64 bg-[var(--bg-surface)] border-r border-[var(--border-default)] flex flex-col">
        <!-- Search -->
        <div class="p-3 border-b border-[var(--border-subtle)]">
          <div class="relative">
            <svg class="absolute left-2.5 top-1/2 -translate-y-1/2 w-4 h-4 text-[var(--text-muted)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8"/>
              <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            </svg>
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索帮助..."
              class="w-full pl-8 pr-3 py-2 text-sm bg-[var(--bg-elevated)] border border-[var(--border-default)] rounded-md text-[var(--text-primary)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-1 focus:ring-[var(--primary)] focus:border-[var(--primary)]"
            />
          </div>
        </div>

        <!-- Categories -->
        <div class="flex-1 overflow-y-auto py-2">
          <div
            v-for="cat in categories"
            :key="cat.id"
            class="px-3 py-1.5"
          >
            <button
              @click="activeCategory = cat.id"
              :class="[
                'w-full flex items-center gap-2 px-3 py-2 rounded-md text-sm transition',
                activeCategory === cat.id
                  ? 'bg-[var(--primary-glow)] text-[var(--primary)] font-medium'
                  : 'text-[var(--text-secondary)] hover:bg-[var(--bg-elevated)] hover:text-[var(--text-primary)]'
              ]"
            >
              <span class="text-base">{{ cat.icon }}</span>
              <span>{{ cat.label }}</span>
              <span class="ml-auto text-xs text-[var(--text-muted)]">{{ getArticlesByCategory(cat.id).length }}</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Right Content: Article List & Detail -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Article List -->
        <div class="w-72 bg-[var(--bg-base)] border-r border-[var(--border-subtle)] overflow-y-auto">
          <div class="p-3">
            <h3 class="text-xs font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2">
              {{ searchQuery ? '搜索结果' : activeCategoryLabel }}
            </h3>
            <div class="space-y-1">
              <button
                v-for="article in filteredArticles"
                :key="article.id"
                @click="selectArticle(article)"
                :class="[
                  'w-full text-left px-3 py-2.5 rounded-md transition text-sm',
                  selectedArticle?.id === article.id
                    ? 'bg-[var(--primary-glow)] border border-[var(--primary)]'
                    : 'hover:bg-[var(--bg-elevated)] border border-transparent'
                ]"
              >
                <div class="flex items-center gap-2">
                  <span class="text-base">{{ article.icon }}</span>
                  <span :class="selectedArticle?.id === article.id ? 'text-[var(--primary)] font-medium' : 'text-[var(--text-primary)]'">
                    {{ article.title }}
                  </span>
                </div>
                <p class="text-xs text-[var(--text-muted)] mt-1 line-clamp-2 pl-7">
                  {{ getArticleExcerpt(article) }}
                </p>
              </button>
              <div v-if="filteredArticles.length === 0" class="text-center py-8 text-[var(--text-muted)] text-sm">
                没有找到匹配的文章
              </div>
            </div>
          </div>
        </div>

        <!-- Article Detail -->
        <div class="flex-1 overflow-y-auto bg-[var(--bg-base)]">
          <div v-if="selectedArticle" class="max-w-3xl mx-auto px-8 py-6">
            <div class="mb-6">
              <div class="flex items-center gap-3 mb-2">
                <span class="text-2xl">{{ selectedArticle.icon }}</span>
                <h2 class="text-2xl font-bold text-[var(--text-primary)]">{{ selectedArticle.title }}</h2>
              </div>
              <div class="flex gap-2 mt-2">
                <span
                  v-for="keyword in selectedArticle.keywords.slice(0, 5)"
                  :key="keyword"
                  class="px-2 py-0.5 text-xs bg-[var(--bg-elevated)] text-[var(--text-muted)] rounded-full border border-[var(--border-subtle)]"
                >
                  {{ keyword }}
                </span>
              </div>
            </div>
            <div class="prose prose-sm max-w-none text-[var(--text-primary)]" v-html="selectedArticle.content"></div>
          </div>
          <div v-else class="flex items-center justify-center h-full text-[var(--text-muted)]">
            <div class="text-center">
              <svg class="w-16 h-16 mx-auto mb-4 opacity-30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                <circle cx="12" cy="12" r="10"/>
                <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/>
                <line x1="12" y1="17" x2="12.01" y2="17"/>
              </svg>
              <p class="text-sm">从左侧选择一篇文章开始阅读</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Onboarding Guide -->
    <OnboardingGuide
      v-if="showOnboarding"
      @close="showOnboarding = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { helpArticles, searchHelp, getArticlesByCategory as getArticlesByCategoryFn, type HelpArticle } from '@/utils/helpDocs'
import OnboardingGuide from '@/components/common/OnboardingGuide.vue'

const searchQuery = ref('')
const activeCategory = ref<string>('getting-started')
const selectedArticle = ref<HelpArticle | null>(null)
const showOnboarding = ref(false)

const categories = [
  { id: 'getting-started', label: '入门指南', icon: '🚀' },
  { id: 'modeling', label: '建模', icon: '📁' },
  { id: 'mesh', label: '网格', icon: '🔲' },
  { id: 'simulation', label: '仿真', icon: '⚡' },
  { id: 'postprocessing', label: '后处理', icon: '📊' },
  { id: 'advanced', label: '高级功能', icon: '🔬' },
]

const activeCategoryLabel = computed(() => {
  return categories.find(c => c.id === activeCategory.value)?.label || ''
})

const filteredArticles = computed(() => {
  if (searchQuery.value.trim()) {
    return searchHelp(searchQuery.value)
  }
  return getArticlesByCategoryFn(activeCategory.value)
})

function getArticlesByCategory(category: string): HelpArticle[] {
  return getArticlesByCategoryFn(category)
}

function selectArticle(article: HelpArticle) {
  selectedArticle.value = article
}

function getArticleExcerpt(article: HelpArticle): string {
  const text = article.content.replace(/<[^>]*>/g, '').trim()
  return text.substring(0, 80) + (text.length > 80 ? '...' : '')
}
</script>

<style scoped>
.prose :deep(h2) {
  font-size: 1.25rem;
  font-weight: 700;
  margin-top: 1.5rem;
  margin-bottom: 0.75rem;
  color: var(--text-primary);
}

.prose :deep(h3) {
  font-size: 1.1rem;
  font-weight: 600;
  margin-top: 1.25rem;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.prose :deep(p) {
  margin-bottom: 0.75rem;
  line-height: 1.7;
  color: var(--text-secondary);
}

.prose :deep(ul),
.prose :deep(ol) {
  margin-bottom: 0.75rem;
  padding-left: 1.5rem;
}

.prose :deep(li) {
  margin-bottom: 0.375rem;
  line-height: 1.6;
  color: var(--text-secondary);
}

.prose :deep(strong) {
  color: var(--text-primary);
  font-weight: 600;
}

.prose :deep(a) {
  color: var(--primary);
  text-decoration: underline;
}

.prose :deep(a:hover) {
  color: var(--primary-hover);
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
