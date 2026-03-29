<template>
  <div class="h-full flex flex-col p-6 overflow-auto">
    <!-- Welcome Header -->
    <div class="flex justify-between items-center mb-8">
      <div>
        <h1 class="text-2xl font-bold text-[var(--text-primary)]">欢迎使用 CAELab</h1>
        <p class="text-[var(--text-secondary)] mt-1">科研与工程创作一体化工作台</p>
      </div>
      <button 
        @click="showCreateDialog = true"
        class="btn btn-primary flex items-center gap-2"
      >
        <span class="text-lg">+</span>
        <span>新建项目</span>
      </button>
    </div>

    <!-- Recent Projects -->
    <div v-if="recentProjects.length > 0" class="mb-8">
      <h3 class="text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider mb-4">最近项目</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div 
          v-for="project in recentProjects" 
          :key="project.id"
          class="card cursor-pointer group"
          @click="openProject(project)"
        >
          <div class="flex justify-between items-start mb-3">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-lg bg-[var(--primary-glow)] flex items-center justify-center">
                <span class="text-[var(--primary)] text-lg">📁</span>
              </div>
              <div>
                <h4 class="font-semibold text-[var(--text-primary)]">{{ project.name }}</h4>
                <p class="text-xs text-[var(--text-muted)] mt-0.5">{{ formatDate(project.created_at) }}</p>
              </div>
            </div>
            <div class="relative">
              <button 
                @click.stop="toggleMenu(project.id)"
                class="icon-btn opacity-0 group-hover:opacity-100"
              >
                <span class="text-lg">⋮</span>
              </button>
              <!-- Dropdown Menu -->
              <div 
                v-if="activeMenu === project.id"
                class="absolute right-0 top-full mt-1 bg-[var(--bg-surface)] rounded-lg shadow-lg border border-[var(--border-subtle)] py-1 z-20 min-w-[120px]"
                @click.stop
              >
                <button 
                  @click.stop="doEditProject(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>✏️</span> 编辑
                </button>
                <button 
                  @click.stop="doConfirmDelete(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--accent-red)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>🗑️</span> 删除
                </button>
              </div>
            </div>
          </div>
          <p class="text-sm text-[var(--text-secondary)] line-clamp-2">{{ project.description || '暂无描述' }}</p>
          <!-- Module Quick Access -->
          <div class="flex gap-2 mt-4 pt-4 border-t border-[var(--border-subtle)]">
            <span 
              v-for="mod in moduleIcons" 
              :key="mod.path"
              @click.stop="openModule(project, mod.path)"
              class="w-8 h-8 flex items-center justify-center rounded bg-[var(--bg-elevated)] hover:bg-[var(--bg-hover)] cursor-pointer transition-colors"
              :title="mod.name"
            >
              {{ mod.icon }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-16 flex-1 flex flex-col items-center justify-center">
      <div class="w-20 h-20 rounded-2xl bg-[var(--bg-surface)] flex items-center justify-center mb-6">
        <span class="text-5xl opacity-50">🚀</span>
      </div>
      <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-2">开始你的创作之旅</h3>
      <p class="text-[var(--text-secondary)] mb-6 max-w-md">创建你的第一个CAELab项目，体验笔记、建模、代码与仿真的一体化工作流</p>
      <button 
        @click="showCreateDialog = true"
        class="btn btn-primary"
      >
        创建第一个项目
      </button>
    </div>

    <!-- Module Cards -->
    <div class="mt-auto pt-8 border-t border-[var(--border-subtle)]">
      <h3 class="text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider mb-4">功能模块</h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div 
          v-for="module in modules" 
          :key="module.path"
          class="card cursor-pointer group"
          @click="handleModuleClick(module.path)"
        >
          <div class="w-12 h-12 rounded-xl bg-[var(--bg-elevated)] flex items-center justify-center mb-3 group-hover:bg-[var(--primary-glow)] transition-colors">
            <span class="text-2xl">{{ module.icon }}</span>
          </div>
          <h4 class="font-semibold text-[var(--text-primary)] mb-1">{{ module.name }}</h4>
          <p class="text-xs text-[var(--text-muted)]">{{ module.desc }}</p>
        </div>
      </div>
    </div>

    <!-- Create/Edit Dialog -->
    <div v-if="showCreateDialog" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="closeDialog">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg animate-slide-in">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">
          {{ editingProject ? '编辑项目' : '创建新项目' }}
        </h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">项目名称</label>
            <input 
              v-model="projectForm.name"
              type="text"
              class="input w-full"
              placeholder="输入项目名称"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">项目描述</label>
            <textarea 
              v-model="projectForm.description"
              rows="3"
              class="input w-full resize-none"
              placeholder="输入项目描述（可选）"
            ></textarea>
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button 
            @click="closeDialog"
            class="btn btn-ghost"
          >
            取消
          </button>
          <button 
            @click="handleCreateOrUpdate"
            :disabled="!projectForm.name.trim()"
            class="btn btn-primary"
          >
            {{ editingProject ? '保存' : '创建' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Dialog -->
    <div v-if="showDeleteDialog" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="showDeleteDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-sm shadow-lg animate-slide-in">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-10 h-10 rounded-full bg-[var(--accent-red)]/20 flex items-center justify-center">
            <span class="text-[var(--accent-red)] text-xl">⚠️</span>
          </div>
          <h3 class="text-lg font-semibold text-[var(--text-primary)]">确认删除</h3>
        </div>
        <p class="text-[var(--text-secondary)] mb-6">
          确定要删除项目「<span class="text-[var(--text-primary)]">{{ deletingProject?.name }}</span>」吗？此操作不可恢复。
        </p>
        <div class="flex justify-end gap-3">
          <button 
            @click="showDeleteDialog = false"
            class="btn btn-ghost"
          >
            取消
          </button>
          <button 
            @click="handleDelete"
            class="btn bg-[var(--accent-red)] text-white hover:bg-[var(--accent-red)]/90"
          >
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { listProjects, createProject, updateProject, deleteProject, type Project } from '@/api'
import { formatProjectDate } from '@/api'

const router = useRouter()

// State
const projects = ref<Project[]>([])
const showCreateDialog = ref(false)
const showDeleteDialog = ref(false)
const editingProject = ref<Project | null>(null)
const deletingProject = ref<Project | null>(null)
const activeMenu = ref<string | null>(null)

// Form
const projectForm = ref({
  name: '',
  description: ''
})

// Module icons for quick access
const moduleIcons = [
  { name: '笔记', icon: '📝', path: '/notes' },
  { name: '建模', icon: '🎨', path: '/modeling' },
  { name: '代码', icon: '💻', path: '/code' },
  { name: '仿真', icon: '🔬', path: '/simulation' }
]

// Modules list
const modules = [
  { name: '笔记创作', icon: '📝', path: '/notes', desc: '富文本编辑、LaTeX公式、手写输入' },
  { name: '3D建模', icon: '🎨', path: '/modeling', desc: '几何建模、外部模型导入' },
  { name: '代码编辑', icon: '💻', path: '/code', desc: '语法高亮、代码运行' },
  { name: '仿真分析', icon: '🔬', path: '/simulation', desc: '结构力学仿真、结果后处理' }
]

// Computed: recent projects (sorted by creation date, limit 6)
const recentProjects = computed(() => {
  return [...projects.value]
    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    .slice(0, 6)
})

// Format date
function formatDate(dateStr: string): string {
  return formatProjectDate(dateStr)
}

// Load projects
async function loadProjects() {
  try {
    projects.value = await listProjects()
  } catch (error) {
    console.error('Failed to load projects:', error)
  }
}

// Create or update project
async function handleCreateOrUpdate() {
  if (!projectForm.value.name.trim()) return
  
  try {
    if (editingProject.value) {
      await updateProject({
        id: editingProject.value.id,
        name: projectForm.value.name,
        description: projectForm.value.description
      })
    } else {
      await createProject({
        name: projectForm.value.name,
        description: projectForm.value.description
      })
    }
    closeDialog()
    await loadProjects()
  } catch (error) {
    console.error('Failed to save project:', error)
  }
}

// Edit project action
function doEditProject(project: Project) {
  editingProject.value = project
  projectForm.value.name = project.name
  projectForm.value.description = project.description
  showCreateDialog.value = true
  activeMenu.value = null
}

// Confirm delete action
function doConfirmDelete(project: Project) {
  deletingProject.value = project
  showDeleteDialog.value = true
  activeMenu.value = null
}

// Execute delete
async function handleDelete() {
  if (!deletingProject.value) return
  
  try {
    await deleteProject(deletingProject.value.id)
    showDeleteDialog.value = false
    deletingProject.value = null
    await loadProjects()
  } catch (error) {
    console.error('Failed to delete project:', error)
  }
}

// Close dialog
function closeDialog() {
  showCreateDialog.value = false
  editingProject.value = null
  projectForm.value.name = ''
  projectForm.value.description = ''
}

// Toggle menu
function toggleMenu(projectId: string) {
  activeMenu.value = activeMenu.value === projectId ? null : projectId
}

// Open project
function openProject(project: Project) {
  router.push({ path: '/notes', query: { projectId: project.id } })
}

// Open project module
function openModule(project: Project, path: string) {
  router.push({ path, query: { projectId: project.id } })
}

// Handle module click
function handleModuleClick(path: string) {
  if (recentProjects.value.length > 0) {
    const currentProject = recentProjects.value[0]
    router.push({ path, query: { projectId: currentProject.id } })
  } else {
    router.push(path)
  }
}

// Click outside to close menu
function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (!target.closest('.relative')) {
    activeMenu.value = null
  }
}

onMounted(() => {
  loadProjects()
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>