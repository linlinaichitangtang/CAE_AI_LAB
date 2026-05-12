/**
 * usePluginMarket.ts — V3.4-006 插件市场
 * 第三方扩展（优化器、可视化工具、专用求解器）
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type PluginCategory =
  | 'solver'
  | 'visualization'
  | 'optimizer'
  | 'importer'
  | 'exporter'
  | 'preprocessor'
  | 'postprocessor'
  | 'ai'
  | 'utility'
  | 'custom'

export type PluginStatus = 'installed' | 'not_installed' | 'update_available' | 'disabled'
export type PluginSource = 'official' | 'community' | 'verified' | 'local'
export type LicenseType = 'free' | 'commercial' | 'open_source' | 'trial'
export type CompatibilityStatus = 'compatible' | 'incompatible' | 'unknown'

export interface PluginVersion {
  version: string
  releaseDate: string
  changelog: string
  minAppVersion: string
  maxAppVersion?: string
  downloadSize: number
  downloadUrl: string
  checksum: string
}

export interface PluginRequirements {
  os?: string[]
  memory?: string
  gpu?: boolean
  dependencies?: Array<{ name: string; version: string }>
}

export interface PluginPermissions {
  network?: boolean
  filesystem?: boolean
  compute?: boolean
  dataAccess?: boolean
}

export interface Plugin {
  id: string
  name: string
  displayName: string
  description: string
  longDescription?: string
  author: {
    name: string
    email?: string
    website?: string
    organization?: string
  }

  // 分类与标签
  category: PluginCategory
  tags: string[]
  icon?: string

  // 版本信息
  currentVersion: string
  versions: PluginVersion[]

  // 统计
  downloads: number
  rating: number
  reviewCount: number

  // 元数据
  license: LicenseType
  price?: number
  source: PluginSource
  repositoryUrl?: string
  homepageUrl?: string

  // 兼容性
  compatibility: CompatibilityStatus
  requirements: PluginRequirements
  permissions: PluginPermissions

  // 安装信息
  installedSize: number
  lastUpdated: string
  verified: boolean
  featured: boolean

  // 截图/预览
  screenshots?: string[]
}

export interface PluginReview {
  id: string
  pluginId: string
  author: string
  rating: number
  title: string
  content: string
  createdAt: string
  updatedAt?: string
  helpful: number
  version?: string
}

export interface PluginInstallation {
  id: string
  pluginId: string
  version: string
  installedAt: string
  updatedAt?: string
  enabled: boolean
  config: Record<string, any>
}

export interface PluginUpdate {
  pluginId: string
  currentVersion: string
  newVersion: string
  changelog: string
  critical: boolean
}

export interface PluginCategoryInfo {
  id: PluginCategory
  name: string
  description: string
  icon: string
  pluginCount: number
}

// ============ 插件市场数据 ============

export const PLUGIN_CATEGORIES: PluginCategoryInfo[] = [
  { id: 'solver', name: '求解器', description: '专用仿真求解器', icon: '🔧', pluginCount: 12 },
  { id: 'visualization', name: '可视化', description: '高级可视化工具', icon: '📊', pluginCount: 8 },
  { id: 'optimizer', name: '优化器', description: '设计优化算法', icon: '🎯', pluginCount: 6 },
  { id: 'importer', name: '导入工具', description: '文件格式导入', icon: '📥', pluginCount: 15 },
  { id: 'exporter', name: '导出工具', description: '文件格式导出', icon: '📤', pluginCount: 10 },
  { id: 'preprocessor', name: '前处理', description: '几何清理与修复', icon: '⚙️', pluginCount: 7 },
  { id: 'postprocessor', name: '后处理', description: '结果分析与报告', icon: '📈', pluginCount: 9 },
  { id: 'ai', name: 'AI 增强', description: '人工智能辅助', icon: '🤖', pluginCount: 5 },
  { id: 'utility', name: '工具', description: '实用辅助工具', icon: '🛠️', pluginCount: 11 },
  { id: 'custom', name: '自定义', description: '用户开发插件', icon: '✨', pluginCount: 3 }
]

// 示例插件数据
const SAMPLE_PLUGINS: Plugin[] = [
  {
    id: 'topology-optimizer',
    name: 'topology-optimizer',
    displayName: '拓扑优化器',
    description: '基于 SIMP 方法的结构拓扑优化，支持最小柔顺性、应力和位移约束',
    longDescription: '这是一个专业的拓扑优化插件，采用 SIMP（固体各向同性材料惩罚）方法。可以快速生成满足指定约束的优化结构设计。支持多种优化目标，包括最小柔顺性、最大刚度和应力约束。',
    author: {
      name: 'CAELab Team',
      organization: 'CAELab'
    },
    category: 'optimizer',
    tags: ['topology', 'optimization', 'SIMP', 'structural'],
    currentVersion: '2.1.0',
    versions: [
      {
        version: '2.1.0',
        releaseDate: '2026-04-15',
        changelog: '新增多目标优化支持\n性能优化',
        minAppVersion: '3.0.0',
        downloadSize: 2457600,
        downloadUrl: 'https://plugins.caelab.com/topology-optimizer/v2.1.0',
        checksum: 'abc123'
      }
    ],
    downloads: 15420,
    rating: 4.8,
    reviewCount: 89,
    license: 'open_source',
    source: 'official',
    repositoryUrl: 'https://github.com/caelab/topology-optimizer',
    homepageUrl: 'https://plugins.caelab.com/topology-optimizer',
    compatibility: 'compatible',
    requirements: {
      memory: '4GB',
      gpu: true
    },
    permissions: {
      compute: true
    },
    installedSize: 5242880,
    lastUpdated: '2026-04-15',
    verified: true,
    featured: true
  },
  {
    id: 'abaqus-importer',
    name: 'abaqus-importer',
    displayName: 'ABAQUS 导入工具',
    description: '支持导入 .inp, .cae, .odb 文件，保持几何、网格和结果数据',
    author: {
      name: 'Engineering Tools Inc.',
      organization: 'Engineering Tools'
    },
    category: 'importer',
    tags: ['abaqus', 'import', 'inp', 'cae', 'odb'],
    currentVersion: '1.5.2',
    versions: [
      {
        version: '1.5.2',
        releaseDate: '2026-03-20',
        changelog: '修复 ODB 结果读取问题\n支持 ABAQUS 2024',
        minAppVersion: '2.8.0',
        downloadSize: 15728640,
        downloadUrl: 'https://plugins.caelab.com/abaqus-importer/v1.5.2',
        checksum: 'def456'
      }
    ],
    downloads: 28750,
    rating: 4.6,
    reviewCount: 156,
    license: 'commercial',
    price: 299,
    source: 'verified',
    compatibility: 'compatible',
    requirements: {},
    permissions: {
      filesystem: true
    },
    installedSize: 31457280,
    lastUpdated: '2026-03-20',
    verified: true,
    featured: true
  },
  {
    id: 'advanced-visualizer',
    name: 'advanced-visualizer',
    displayName: '高级可视化工具',
    description: '支持等值面、体渲染、粒子追踪等多种高级可视化效果',
    author: {
      name: 'VizTools',
      organization: 'VizTools Labs'
    },
    category: 'visualization',
    tags: ['visualization', 'rendering', 'isosurface', 'volume'],
    currentVersion: '3.0.1',
    versions: [
      {
        version: '3.0.1',
        releaseDate: '2026-04-28',
        changelog: '新增体渲染功能\nGPU 加速优化',
        minAppVersion: '3.1.0',
        downloadSize: 83886080,
        downloadUrl: 'https://plugins.caelab.com/advanced-visualizer/v3.0.1',
        checksum: 'ghi789'
      }
    ],
    downloads: 12380,
    rating: 4.9,
    reviewCount: 72,
    license: 'commercial',
    price: 199,
    source: 'verified',
    compatibility: 'compatible',
    requirements: {
      memory: '8GB',
      gpu: true
    },
    permissions: {
      compute: true
    },
    installedSize: 167772160,
    lastUpdated: '2026-04-28',
    verified: true,
    featured: false
  },
  {
    id: 'lattice-generator',
    name: 'lattice-generator',
    displayName: '晶格结构生成器',
    description: '快速生成点阵/晶格结构，支持多种晶格类型和参数化设计',
    author: {
      name: 'DesignLab',
      organization: 'DesignLab Studio'
    },
    category: 'preprocessor',
    tags: ['lattice', 'tpms', 'bcc', 'gyroid', 'structure'],
    currentVersion: '1.2.0',
    versions: [
      {
        version: '1.2.0',
        releaseDate: '2026-02-10',
        changelog: '支持 TPMS 类型\n参数化界面优化',
        minAppVersion: '3.0.0',
        downloadSize: 3145728,
        downloadUrl: 'https://plugins.caelab.com/lattice-generator/v1.2.0',
        checksum: 'jkl012'
      }
    ],
    downloads: 8920,
    rating: 4.7,
    reviewCount: 45,
    license: 'free',
    source: 'community',
    compatibility: 'compatible',
    requirements: {},
    permissions: {
      compute: true
    },
    installedSize: 6291456,
    lastUpdated: '2026-02-10',
    verified: false,
    featured: false
  },
  {
    id: 'ansa-exporter',
    name: 'ansa-exporter',
    displayName: 'ANSA 导出工具',
    description: '导出兼容 ANSA 格式的网格和模型数据',
    author: {
      name: 'CAE Solutions',
      organization: 'CAE Solutions Ltd.'
    },
    category: 'exporter',
    tags: ['ansa', 'export', 'mesh', 'bdf'],
    currentVersion: '2.0.3',
    versions: [
      {
        version: '2.0.3',
        releaseDate: '2026-01-15',
        changelog: '支持 ANSA 2024\n多语言界面',
        minAppVersion: '2.5.0',
        downloadSize: 5242880,
        downloadUrl: 'https://plugins.caelab.com/ansa-exporter/v2.0.3',
        checksum: 'mno345'
      }
    ],
    downloads: 5640,
    rating: 4.4,
    reviewCount: 28,
    license: 'commercial',
    price: 149,
    source: 'verified',
    compatibility: 'compatible',
    requirements: {},
    permissions: {
      filesystem: true
    },
    installedSize: 10485760,
    lastUpdated: '2026-01-15',
    verified: true,
    featured: false
  },
  {
    id: 'pdf-report-generator',
    name: 'pdf-report-generator',
    displayName: 'PDF 报告生成器',
    description: '生成专业级 PDF 仿真报告，支持自定义模板',
    author: {
      name: 'CAELab Team',
      organization: 'CAELab'
    },
    category: 'postprocessor',
    tags: ['report', 'pdf', 'export', 'template'],
    currentVersion: '1.8.0',
    versions: [
      {
        version: '1.8.0',
        releaseDate: '2026-03-05',
        changelog: '新增 5 种报告模板\n支持中文',
        minAppVersion: '3.0.0',
        downloadSize: 10485760,
        downloadUrl: 'https://plugins.caelab.com/pdf-report-generator/v1.8.0',
        checksum: 'pqr678'
      }
    ],
    downloads: 34560,
    rating: 4.5,
    reviewCount: 203,
    license: 'free',
    source: 'official',
    compatibility: 'compatible',
    requirements: {},
    permissions: {
      filesystem: true
    },
    installedSize: 20971520,
    lastUpdated: '2026-03-05',
    verified: true,
    featured: true
  },
  {
    id: 'surrogate-model',
    name: 'surrogate-model',
    displayName: '代理模型工具箱',
    description: '基于机器学习的快速预测模型，用于加速仿真和优化',
    author: {
      name: 'AI Engineering',
      organization: 'AI Engineering Corp.'
    },
    category: 'ai',
    tags: ['ai', 'ml', 'surrogate', 'prediction', 'neural network'],
    currentVersion: '2.3.0',
    versions: [
      {
        version: '2.3.0',
        releaseDate: '2026-04-20',
        changelog: '支持 Transformer 模型\n新增不确定性量化',
        minAppVersion: '3.1.0',
        downloadSize: 52428800,
        downloadUrl: 'https://plugins.caelab.com/surrogate-model/v2.3.0',
        checksum: 'stu901'
      }
    ],
    downloads: 7890,
    rating: 4.6,
    reviewCount: 41,
    license: 'trial',
    price: 499,
    source: 'verified',
    compatibility: 'compatible',
    requirements: {
      memory: '8GB',
      gpu: true
    },
    permissions: {
      compute: true,
      dataAccess: true
    },
    installedSize: 104857600,
    lastUpdated: '2026-04-20',
    verified: true,
    featured: true
  },
  {
    id: 'fatigue-analysis',
    name: 'fatigue-analysis',
    displayName: '疲劳分析工具',
    description: '支持高周疲劳、低周疲劳和热疲劳分析',
    author: {
      name: 'Endurance CAE',
      organization: 'Endurance CAE Inc.'
    },
    category: 'solver',
    tags: ['fatigue', 'stress', 'cyclic', 'lifecycle'],
    currentVersion: '3.1.2',
    versions: [
      {
        version: '3.1.2',
        releaseDate: '2026-02-28',
        changelog: '支持热-机械耦合疲劳\n新增 S-N 曲线库',
        minAppVersion: '3.0.0',
        downloadSize: 18874368,
        downloadUrl: 'https://plugins.caelab.com/fatigue-analysis/v3.1.2',
        checksum: 'vwx234'
      }
    ],
    downloads: 11230,
    rating: 4.7,
    reviewCount: 67,
    license: 'commercial',
    price: 399,
    source: 'verified',
    compatibility: 'compatible',
    requirements: {},
    permissions: {
      compute: true
    },
    installedSize: 37748736,
    lastUpdated: '2026-02-28',
    verified: true,
    featured: false
  }
]

// ============ 存储键 ============

const INSTALLED_KEY = 'caelab_installed_plugins'
const UPDATES_KEY = 'caelab_plugin_updates'
const FAVORITES_KEY = 'caelab_plugin_favorites'

// ============ 工具函数 ============

function generateId(): string {
  return `plugin_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`
}

// ============ 主 Composable ============

export function usePluginMarket() {
  const availablePlugins = ref<Plugin[]>(SAMPLE_PLUGINS)
  const installedPlugins = ref<PluginInstallation[]>([])
  const favoriteIds = ref<string[]>([])
  const updateNotifications = ref<PluginUpdate[]>([])

  const searchQuery = ref('')
  const selectedCategory = ref<PluginCategory | 'all'>('all')
  const selectedSource = ref<PluginSource | 'all'>('all')
  const sortBy = ref<'downloads' | 'rating' | 'date' | 'name'>('downloads')
  const sortOrder = ref<'asc' | 'desc'>('desc')

  // ============ 初始化 ============

  function loadData(): void {
    const storedInstalled = getStorage<PluginInstallation[]>(INSTALLED_KEY)
    if (storedInstalled) installedPlugins.value = storedInstalled

    const storedFavorites = getStorage<string[]>(FAVORITES_KEY)
    if (storedFavorites) favoriteIds.value = storedFavorites

    const storedUpdates = getStorage<PluginUpdate[]>(UPDATES_KEY)
    if (storedUpdates) updateNotifications.value = storedUpdates
  }

  // ============ 过滤和搜索 ============

  const filteredPlugins = computed(() => {
    let result = [...availablePlugins.value]

    // 搜索过滤
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      result = result.filter(p =>
        p.name.toLowerCase().includes(query) ||
        p.displayName.toLowerCase().includes(query) ||
        p.description.toLowerCase().includes(query) ||
        p.tags.some(t => t.toLowerCase().includes(query))
      )
    }

    // 分类过滤
    if (selectedCategory.value !== 'all') {
      result = result.filter(p => p.category === selectedCategory.value)
    }

    // 来源过滤
    if (selectedSource.value !== 'all') {
      result = result.filter(p => p.source === selectedSource.value)
    }

    // 排序
    result.sort((a, b) => {
      let comparison = 0

      switch (sortBy.value) {
        case 'downloads':
          comparison = a.downloads - b.downloads
          break
        case 'rating':
          comparison = a.rating - b.rating
          break
        case 'date':
          comparison = new Date(a.lastUpdated).getTime() - new Date(b.lastUpdated).getTime()
          break
        case 'name':
          comparison = a.displayName.localeCompare(b.displayName)
          break
      }

      return sortOrder.value === 'asc' ? comparison : -comparison
    })

    return result
  })

  const featuredPlugins = computed(() =>
    availablePlugins.value.filter(p => p.featured)
  )

  // ============ 插件操作 ============

  function getPlugin(pluginId: string): Plugin | undefined {
    return availablePlugins.value.find(p => p.id === pluginId)
  }

  function getInstallation(pluginId: string): PluginInstallation | undefined {
    return installedPlugins.value.find(i => i.pluginId === pluginId)
  }

  function getPluginStatus(pluginId: string): PluginStatus {
    const installation = installedPlugins.value.find(i => i.pluginId === pluginId)
    if (!installation) return 'not_installed'
    return installation.enabled ? 'installed' : 'disabled'
  }

  /**
   * 安装插件
   */
  async function installPlugin(pluginId: string, version?: string): Promise<boolean> {
    const plugin = getPlugin(pluginId)
    if (!plugin) return false

    const targetVersion = version || plugin.currentVersion

    // 模拟下载和安装
    await simulateDelay(1000)

    const installation: PluginInstallation = {
      id: generateId(),
      pluginId,
      version: targetVersion,
      installedAt: new Date().toISOString(),
      enabled: true,
      config: {}
    }

    installedPlugins.value.push(installation)
    saveInstalled()

    return true
  }

  /**
   * 卸载插件
   */
  async function uninstallPlugin(pluginId: string): Promise<boolean> {
    const index = installedPlugins.value.findIndex(i => i.pluginId === pluginId)
    if (index === -1) return false

    installedPlugins.value.splice(index, 1)
    saveInstalled()

    return true
  }

  /**
   * 启用/禁用插件
   */
  function togglePlugin(pluginId: string): boolean {
    const installation = installedPlugins.value.find(i => i.pluginId === pluginId)
    if (!installation) return false

    installation.enabled = !installation.enabled
    saveInstalled()

    return installation.enabled
  }

  /**
   * 更新插件
   */
  async function updatePlugin(pluginId: string): Promise<boolean> {
    const plugin = getPlugin(pluginId)
    if (!plugin) return false

    // 模拟更新过程
    await simulateDelay(1500)

    const installation = installedPlugins.value.find(i => i.pluginId === pluginId)
    if (installation) {
      installation.version = plugin.currentVersion
      installation.updatedAt = new Date().toISOString()
      saveInstalled()
    }

    // 移除更新通知
    updateNotifications.value = updateNotifications.value.filter(u => u.pluginId !== pluginId)
    saveUpdates()

    return true
  }

  /**
   * 检查更新
   */
  async function checkForUpdates(): Promise<PluginUpdate[]> {
    // 模拟检查更新
    await simulateDelay(500)

    const updates: PluginUpdate[] = []

    for (const installation of installedPlugins.value) {
      const plugin = getPlugin(installation.pluginId)
      if (plugin && installation.version !== plugin.currentVersion) {
        updates.push({
          pluginId: plugin.id,
          currentVersion: installation.version,
          newVersion: plugin.currentVersion,
          changelog: plugin.versions[0]?.changelog || '',
          critical: false
        })
      }
    }

    updateNotifications.value = updates
    saveUpdates()

    return updates
  }

  // ============ 收藏 ============

  function toggleFavorite(pluginId: string): boolean {
    const index = favoriteIds.value.indexOf(pluginId)
    if (index === -1) {
      favoriteIds.value.push(pluginId)
      saveFavorites()
      return true
    } else {
      favoriteIds.value.splice(index, 1)
      saveFavorites()
      return false
    }
  }

  function isFavorite(pluginId: string): boolean {
    return favoriteIds.value.includes(pluginId)
  }

  function getFavoritePlugins(): Plugin[] {
    return availablePlugins.value.filter(p => favoriteIds.value.includes(p.id))
  }

  // ============ 插件配置 ============

  function getPluginConfig(pluginId: string): Record<string, any> {
    const installation = installedPlugins.value.find(i => i.pluginId === pluginId)
    return installation?.config || {}
  }

  function updatePluginConfig(pluginId: string, updates: Record<string, any>): boolean {
    const installation = installedPlugins.value.find(i => i.pluginId === pluginId)
    if (!installation) return false

    installation.config = { ...installation.config, ...updates }
    saveInstalled()

    return true
  }

  // ============ 分类统计 ============

  const categoryStats = computed(() => {
    const stats: Record<PluginCategory, { total: number; installed: number }> = {} as any

    for (const category of PLUGIN_CATEGORIES) {
      const plugins = availablePlugins.value.filter(p => p.category === category.id)
      const installed = installedPlugins.value.filter(i =>
        plugins.some(p => p.id === i.pluginId)
      )

      stats[category.id] = {
        total: plugins.length,
        installed: installed.length
      }
    }

    return stats
  })

  // ============ 统计 ============

  const stats = computed(() => ({
    totalPlugins: availablePlugins.value.length,
    installedCount: installedPlugins.value.length,
    updateAvailable: updateNotifications.value.length,
    favoriteCount: favoriteIds.value.length,
    totalDownloads: availablePlugins.value.reduce((sum, p) => sum + p.downloads, 0)
  }))

  // ============ 持久化 ============

  function saveInstalled(): void {
    setStorage(INSTALLED_KEY, installedPlugins.value)
  }

  function saveFavorites(): void {
    setStorage(FAVORITES_KEY, favoriteIds.value)
  }

  function saveUpdates(): void {
    setStorage(UPDATES_KEY, updateNotifications.value)
  }

  // ============ 辅助函数 ============

  function simulateDelay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  function setSearchQuery(query: string): void {
    searchQuery.value = query
  }

  function setCategory(category: PluginCategory | 'all'): void {
    selectedCategory.value = category
  }

  function setSource(source: PluginSource | 'all'): void {
    selectedSource.value = source
  }

  function setSort(by: typeof sortBy.value, order?: typeof sortOrder.value): void {
    sortBy.value = by
    if (order) sortOrder.value = order
  }

  // 初始化
  loadData()

  return {
    // 状态
    availablePlugins,
    installedPlugins,
    favoriteIds,
    updateNotifications,

    // 过滤状态
    searchQuery,
    selectedCategory,
    selectedSource,
    sortBy,
    sortOrder,

    // 计算属性
    filteredPlugins,
    featuredPlugins,
    categoryStats,
    stats,

    // 插件操作
    getPlugin,
    getInstallation,
    getPluginStatus,
    installPlugin,
    uninstallPlugin,
    togglePlugin,
    updatePlugin,
    checkForUpdates,

    // 收藏
    toggleFavorite,
    isFavorite,
    getFavoritePlugins,

    // 配置
    getPluginConfig,
    updatePluginConfig,

    // 过滤
    setSearchQuery,
    setCategory,
    setSource,
    setSort,

    // 工具
    formatBytes,

    // 数据
    PLUGIN_CATEGORIES
  }
}
