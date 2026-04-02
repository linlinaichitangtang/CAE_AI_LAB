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
      <div class="flex gap-2">
        <button 
          @click="triggerImport"
          class="btn btn-ghost flex items-center gap-2"
          title="导入 .caelabzip 项目文件"
        >
          <span>📦</span>
          <span class="hidden sm:inline">导入</span>
        </button>
        <button 
          @click="showTemplatesDialog = true"
          class="btn btn-ghost flex items-center gap-2"
          title="项目管理"
        >
          <span>📋</span>
          <span class="hidden sm:inline">模板</span>
        </button>
        <input
          ref="fileInput"
          type="file"
          accept=".caelabzip"
          class="hidden"
          @change="handleImport"
        />
      </div>
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
                class="absolute right-0 top-full mt-1 bg-[var(--bg-surface)] rounded-lg shadow-lg border border-[var(--border-subtle)] py-1 z-20 min-w-[160px]"
                @click.stop
              >
                <button 
                  @click.stop="doEditProject(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>✏️</span> 编辑
                </button>
                <button 
                  @click.stop="doExportProject(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>📦</span> 导出 .caelabzip
                </button>
                <button 
                  @click.stop="doSaveAsTemplate(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>📋</span> 另存为模板
                </button>
                <button 
                  @click.stop="doShareProject(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>🔗</span> 分享链接
                </button>
                <button 
                  @click.stop="doTeamShareProject(project)"
                  class="w-full px-4 py-2 text-left text-sm text-[var(--text-primary)] hover:bg-[var(--bg-hover)] flex items-center gap-2"
                >
                  <span>👥</span> 团队共享
                </button>
                <div class="border-t border-[var(--border-subtle)] my-1"></div>
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
          <!-- Shared Members Avatars -->
          <div v-if="projectShareMap[project.id] && projectShareMap[project.id].length > 0" class="flex items-center gap-1 mt-2">
            <div class="flex -space-x-2">
              <div
                v-for="(share, idx) in projectShareMap[project.id].slice(0, 4)"
                :key="share.id"
                class="w-6 h-6 rounded-full flex items-center justify-center text-[10px] font-semibold text-white border-2 border-[var(--bg-surface)]"
                :style="{ backgroundColor: getAvatarColor(share.shared_with_name), zIndex: 4 - idx }"
                :title="share.shared_with_name + ' (' + getPermissionLabel(share.permission) + ')'"
              >
                {{ share.shared_with_name.charAt(0).toUpperCase() }}
              </div>
            </div>
            <span v-if="projectShareMap[project.id].length > 4" class="text-xs text-[var(--text-muted)] ml-1">
              +{{ projectShareMap[project.id].length - 4 }}
            </span>
            <span class="text-xs text-[var(--text-muted)] ml-1">
              {{ projectShareMap[project.id].length }} 人共享
            </span>
          </div>
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

    <!-- Templates Section -->
    <div v-if="templates.length > 0" class="mb-8">
      <h3 class="text-sm font-semibold text-[var(--text-secondary)] uppercase tracking-wider mb-4">项目模板</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div 
          v-for="template in templates" 
          :key="template.id"
          class="card cursor-pointer group"
          @click="createFromTemplate(template)"
        >
          <div class="flex items-center gap-3 mb-3">
            <div class="w-10 h-10 rounded-lg bg-[var(--primary-glow)] flex items-center justify-center">
              <span class="text-[var(--primary)] text-lg">{{ template.icon }}</span>
            </div>
            <div>
              <h4 class="font-semibold text-[var(--text-primary)]">{{ template.name }}</h4>
              <p class="text-xs text-[var(--text-muted)]">{{ template.category }}</p>
            </div>
          </div>
          <p class="text-sm text-[var(--text-secondary)] line-clamp-2">{{ template.description || '暂无描述' }}</p>
          <div class="flex justify-end gap-2 mt-4 pt-4 border-t border-[var(--border-subtle)]">
            <button 
              @click.stop="deleteTemplateItem(template.id)"
              class="text-xs text-[var(--text-muted)] hover:text-[var(--accent-red)]"
            >
              删除
            </button>
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

    <!-- Template Dialog -->
    <div v-if="showTemplateDialog" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="showTemplateDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg animate-slide-in">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">另存为模板</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">模板名称</label>
            <input 
              v-model="templateForm.name"
              type="text"
              class="input w-full"
              placeholder="输入模板名称"
            />
          </div>
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">模板描述</label>
            <textarea 
              v-model="templateForm.description"
              rows="3"
              class="input w-full resize-none"
              placeholder="输入模板描述（可选）"
            ></textarea>
          </div>
          <div>
            <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">分类</label>
            <input 
              v-model="templateForm.category"
              type="text"
              class="input w-full"
              placeholder="输入分类名称"
            />
          </div>
        </div>
        <div class="flex justify-end gap-3 mt-6">
          <button 
            @click="showTemplateDialog = false"
            class="btn btn-ghost"
          >
            取消
          </button>
          <button 
            @click="handleSaveAsTemplate"
            :disabled="!templateForm.name.trim()"
            class="btn btn-primary"
          >
            保存
          </button>
        </div>
      </div>
    </div>

    <!-- Share Dialog -->
    <div v-if="showShareDialog" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="showShareDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg animate-slide-in">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">分享项目</h3>
        <p class="text-sm text-[var(--text-secondary)] mb-4">
          复制以下链接分享项目「<span class="text-[var(--text-primary)]">{{ sharingProject?.name }}</span>」
        </p>
        <div class="flex gap-2">
          <input 
            v-model="shareLink"
            type="text"
            class="input w-full bg-[var(--bg-elevated)]"
            readonly
          />
          <button 
            @click="handleCopyLink"
            class="btn btn-primary flex items-center gap-2"
          >
            <span>{{ copyButtonText }}</span>
          </button>
        </div>
        <p class="text-xs text-[var(--text-muted)] mt-4">
          提示：分享链接包含项目数据，接收者可通过链接直接导入项目
        </p>
        <div class="flex justify-end gap-3 mt-6">
          <button 
            @click="showShareDialog = false"
            class="btn btn-ghost"
          >
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- Team Share Dialog -->
    <ShareDialog
      v-if="showTeamShareDialog && teamSharingProject"
      :project-id="teamSharingProject.id"
      :project-name="teamSharingProject.name"
      @close="showTeamShareDialog = false"
      @updated="loadAllProjectShares"
    />

    <!-- Templates Management Dialog -->
    <div v-if="showTemplatesDialog" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="showTemplatesDialog = false">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-2xl shadow-lg animate-slide-in">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">项目模板管理</h3>
        
        <!-- Empty State -->
        <div v-if="templates.length === 0" class="text-center py-8">
          <div class="w-16 h-16 rounded-xl bg-[var(--bg-elevated)] flex items-center justify-center mb-4 mx-auto">
            <span class="text-3xl opacity-50">📋</span>
          </div>
          <p class="text-[var(--text-secondary)] mb-4">暂无模板</p>
          <p class="text-xs text-[var(--text-muted)]">在项目菜单中选择「另存为模板」来创建模板</p>
        </div>
        
        <!-- Templates Grid -->
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-96 overflow-auto">
          <div 
            v-for="template in templates" 
            :key="template.id"
            class="card"
          >
            <div class="flex items-start gap-3">
              <div class="w-10 h-10 rounded-lg bg-[var(--primary-glow)] flex items-center justify-center flex-shrink-0">
                <span class="text-[var(--primary)] text-lg">{{ template.icon }}</span>
              </div>
              <div class="flex-1 min-w-0">
                <h4 class="font-semibold text-[var(--text-primary)]">{{ template.name }}</h4>
                <p class="text-xs text-[var(--text-muted)] mb-1">{{ template.category }}</p>
                <p class="text-sm text-[var(--text-secondary)] line-clamp-2">{{ template.description || '暂无描述' }}</p>
              </div>
            </div>
            <div class="flex justify-end gap-2 mt-4 pt-4 border-t border-[var(--border-subtle)]">
              <button 
                @click="createFromTemplate(template)"
                class="btn btn-primary text-sm"
              >
                创建项目
              </button>
              <button 
                @click="deleteTemplateItem(template.id)"
                class="btn btn-ghost text-sm text-[var(--accent-red)]"
              >
                删除
              </button>
            </div>
          </div>
        </div>
        
        <div class="flex justify-end gap-3 mt-6">
          <button 
            @click="showTemplatesDialog = false"
            class="btn btn-ghost"
          >
            关闭
          </button>
        </div>
      </div>
    </div>

    <!-- Export Progress Dialog -->
    <div v-if="exportingProject" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-sm shadow-lg animate-slide-in text-center">
        <div class="w-12 h-12 border-4 border-[var(--primary)] border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-2">正在导出项目</h3>
        <p class="text-sm text-[var(--text-secondary)]">
          「{{ exportingProject.name }}」
        </p>
        <p class="text-xs text-[var(--text-muted)] mt-2">{{ exportProgress }}</p>
      </div>
    </div>

    <!-- Import Dialog -->
    <div v-if="showImportDialog" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50 animate-fade-in" @click.self="closeImportDialog">
      <div class="bg-[var(--bg-surface)] rounded-xl p-6 w-full max-w-md shadow-lg animate-slide-in">
        <h3 class="text-lg font-semibold text-[var(--text-primary)] mb-4">导入项目</h3>
        
        <div v-if="importingFile" class="mb-4">
          <p class="text-sm text-[var(--text-secondary)] mb-4">
            正在导入: <span class="text-[var(--text-primary)]">{{ importingFile.name }}</span>
          </p>
          <div class="space-y-3">
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">项目名称（可选）</label>
              <input 
                v-model="importForm.name"
                type="text"
                class="input w-full"
                placeholder="留空则使用原项目名称"
              />
            </div>
            <div>
              <label class="block text-xs font-medium text-[var(--text-secondary)] mb-2 uppercase tracking-wider">项目描述（可选）</label>
              <textarea 
                v-model="importForm.description"
                rows="2"
                class="input w-full resize-none"
                placeholder="留空则使用原项目描述"
              ></textarea>
            </div>
          </div>
        </div>
        
        <div v-else class="mb-4">
          <div 
            class="border-2 border-dashed border-[var(--border-subtle)] rounded-xl p-8 text-center cursor-pointer hover:border-[var(--primary)] transition-colors"
            @click="triggerImport"
          >
            <span class="text-4xl block mb-2">📥</span>
            <p class="text-[var(--text-primary)] mb-1">点击或拖拽文件到此处</p>
            <p class="text-xs text-[var(--text-muted)]">支持 .caelabzip 格式的CAELab项目文件</p>
          </div>
        </div>
        
        <div class="flex justify-end gap-3">
          <button 
            @click="closeImportDialog"
            class="btn btn-ghost"
          >
            取消
          </button>
          <button 
            v-if="importingFile"
            @click="handleConfirmImport"
            class="btn btn-primary"
          >
            导入
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
import ShareDialog from '@/components/collaboration/ShareDialog.vue'
import { 
  exportProjectAsZip, 
  importProjectFromZip, 
  saveProjectAsTemplate, 
  getTemplates, 
  createProjectFromTemplate, 
  deleteTemplate,
  generateShareLink, 
  copyToClipboard,
  type ProjectTemplate
} from '@/api/share'
import { listProjectShares, type ProjectShare } from '@/api/collaboration'

const router = useRouter()

// State
const projects = ref<Project[]>([])
const templates = ref<ProjectTemplate[]>([])
const showCreateDialog = ref(false)
const showDeleteDialog = ref(false)
const showTemplateDialog = ref(false)
const showShareDialog = ref(false)
const showTemplatesDialog = ref(false)
const showImportDialog = ref(false)
const editingProject = ref<Project | null>(null)
const deletingProject = ref<Project | null>(null)
const savingAsTemplate = ref<Project | null>(null)
const sharingProject = ref<Project | null>(null)
const activeMenu = ref<string | null>(null)

// Team share state
const showTeamShareDialog = ref(false)
const teamSharingProject = ref<Project | null>(null)
const projectShareMap = ref<Record<string, ProjectShare[]>>({})

// Import state
const fileInput = ref<HTMLInputElement | null>(null)
const importingFile = ref<File | null>(null)
const importForm = ref({
  name: '',
  description: ''
})

// Template form
const templateForm = ref({
  name: '',
  description: '',
  category: 'default'
})

// Share state
const shareLink = ref('')
const copyButtonText = ref('复制')

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

// Load templates
function loadTemplates() {
  templates.value = getTemplates()
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
    await loadAllProjectShares()
  } catch (error) {
    console.error('Failed to save project:', error)
  }
}

// Export state
const exportingProject = ref<Project | null>(null)
const exportProgress = ref('')

// Export project
async function doExportProject(project: Project) {
  try {
    exportingProject.value = project
    exportProgress.value = '正在准备导出...'
    await exportProjectAsZip(project.id, (progress) => {
      exportProgress.value = progress.message
    })
    activeMenu.value = null
    exportingProject.value = null
    exportProgress.value = ''
  } catch (error) {
    console.error('Failed to export project:', error)
    alert('导出失败: ' + (error as Error).message)
    exportingProject.value = null
    exportProgress.value = ''
  }
}

// Save as template
function doSaveAsTemplate(project: Project) {
  savingAsTemplate.value = project
  templateForm.value = {
    name: project.name,
    description: project.description,
    category: 'default'
  }
  showTemplateDialog.value = true
  activeMenu.value = null
}

async function handleSaveAsTemplate() {
  if (!savingAsTemplate.value || !templateForm.value.name.trim()) return
  
  try {
    await saveProjectAsTemplate(
      savingAsTemplate.value.id,
      templateForm.value.name,
      templateForm.value.description,
      templateForm.value.category
    )
    showTemplateDialog.value = false
    loadTemplates()
  } catch (error) {
    console.error('Failed to save template:', error)
    alert('保存模板失败: ' + (error as Error).message)
  }
}

// Share project
function doShareProject(project: Project) {
  sharingProject.value = project
  shareLink.value = generateShareLink(project.id)
  copyButtonText.value = '复制'
  showShareDialog.value = true
  activeMenu.value = null
}

async function handleCopyLink() {
  const success = await copyToClipboard(shareLink.value)
  if (success) {
    copyButtonText.value = '已复制!'
    setTimeout(() => {
      copyButtonText.value = '复制'
    }, 2000)
  }
}

// Team share functions
function doTeamShareProject(project: Project) {
  teamSharingProject.value = project
  showTeamShareDialog.value = true
  activeMenu.value = null
}

async function loadAllProjectShares() {
  const map: Record<string, ProjectShare[]> = {}
  for (const project of projects.value) {
    try {
      map[project.id] = await listProjectShares(project.id)
    } catch (e) {
      console.error(`Failed to load shares for project ${project.id}:`, e)
      map[project.id] = []
    }
  }
  projectShareMap.value = map
}

const avatarColors = [
  '#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6',
  '#EC4899', '#06B6D4', '#F97316', '#14B8A6', '#6366F1',
]

function getAvatarColor(name: string): string {
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return avatarColors[Math.abs(hash) % avatarColors.length]
}

function getPermissionLabel(permission: string): string {
  const labels: Record<string, string> = {
    read: '只读',
    write: '可编辑',
    admin: '管理员',
  }
  return labels[permission] || permission
}

// Template management
async function createFromTemplate(template: ProjectTemplate) {
  try {
    const newProject = await createProjectFromTemplate(template.id)
    showTemplatesDialog.value = false
    await loadProjects()
    router.push({ path: '/notes', query: { projectId: newProject.id } })
  } catch (error) {
    console.error('Failed to create project from template:', error)
    alert('从模板创建项目失败: ' + (error as Error).message)
  }
}

function deleteTemplateItem(templateId: string) {
  deleteTemplate(templateId)
  loadTemplates()
}

// Import functions
function triggerImport() {
  fileInput.value?.click()
}

function handleImport(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    importingFile.value = file
    importForm.value = { name: '', description: '' }
    showImportDialog.value = true
  }
  // Reset input
  target.value = ''
}

function closeImportDialog() {
  showImportDialog.value = false
  importingFile.value = null
  importForm.value = { name: '', description: '' }
}

async function handleConfirmImport() {
  if (!importingFile.value) return

  try {
    const result = await importProjectFromZip(importingFile.value, {
      name: importForm.value.name || undefined,
      description: importForm.value.description || undefined
    })
    closeImportDialog()
    await loadProjects()
    router.push({ path: '/notes', query: { projectId: result.project.id } })
  } catch (error) {
    console.error('Failed to import project:', error)
    alert('导入失败: ' + (error as Error).message)
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
    await loadAllProjectShares()
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

onMounted(async () => {
  await loadProjects()
  loadTemplates()
  document.addEventListener('click', handleClickOutside)
  // Load project shares
  loadAllProjectShares()
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