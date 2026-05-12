/**
 * useProjectSharing.ts — V3.2-004 项目分享+评论
 * 科研团队共享项目，评论/标注特定结果
 */

import { ref } from 'vue'

// ============ 类型定义 ============

export interface ShareProject {
  id: string
  projectId: string
  projectName: string
  ownerId: string
  ownerName: string
  createdAt: string
  updatedAt: string

  // 分享设置
  shareType: 'link' | 'team' | 'public'
  permissions: SharePermission[]
  accessCode?: string  // 访问码（可选）

  // 协作者
  collaborators: ShareCollaborator[]

  // 活动日志
  activityLog: ActivityEntry[]
}

export interface SharePermission {
  role: 'viewer' | 'editor' | 'admin'
  canViewResults: boolean
  canEditMesh: boolean
  canEditBoundaryConditions: boolean
  canExport: boolean
  canShare: boolean
}

export interface ShareCollaborator {
  id: string
  userId: string
  userName: string
  avatarColor: string
  permission: SharePermission
  addedAt: string
  lastAccess?: string
}

export interface ActivityEntry {
  id: string
  userId: string
  userName: string
  action: 'view' | 'edit' | 'comment' | 'download' | 'share' | 'permission_change'
  target?: string  // 被操作的目标（如注释 ID）
  details?: string
  timestamp: string
}

export interface ProjectComment {
  id: string
  projectId: string
  versionId?: string
  resultId?: string

  authorId: string
  authorName: string
  authorAvatarColor: string

  content: string
  createdAt: string
  updatedAt: string

  // 位置信息（可选，用于标注特定区域）
  position?: {
    type: 'mesh_node' | 'mesh_element' | 'result_point' | 'result_region'
    ids?: number[]
    coordinates?: { x: number; y: number; z: number }
  }

  // 状态
  isResolved: boolean
  resolvedBy?: string
  resolvedAt?: string

  // 回复
  replies: CommentReply[]
}

export interface CommentReply {
  id: string
  authorId: string
  authorName: string
  content: string
  createdAt: string
}

export interface ShareLink {
  id: string
  projectId: string
  code: string
  createdAt: string
  expiresAt?: string
  permission: SharePermission
  usageCount: number
  lastUsed?: string
}

// ============ 存储键 ============

const SHARES_KEY = 'caelab_shares'
const COMMENTS_KEY = 'caelab_comments'
const LINKS_KEY = 'caelab_share_links'

// ============ 工具函数 ============

function generateId(): string {
  return `share_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
}

function generateAccessCode(): string {
  return Math.random().toString(36).substring(2, 8).toUpperCase()
}

function getStorage<T>(key: string): T | null {
  const raw = localStorage.getItem(key)
  return raw ? JSON.parse(raw) : null
}

function setStorage<T>(key: string, data: T): void {
  localStorage.setItem(key, JSON.stringify(data))
}

function getAvatarColor(name: string): string {
  const colors = ['#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#F97316']
  let hash = 0
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash)
  }
  return colors[Math.abs(hash) % colors.length]
}

// ============ 主 Composable ============

export function useProjectSharing() {
  const shares = ref<ShareProject[]>([])
  const comments = ref<ProjectComment[]>([])
  const shareLinks = ref<ShareLink[]>([])

  // 当前用户
  const currentUserId = ref(localStorage.getItem('caelab_user_id') || generateId())
  const currentUserName = ref(localStorage.getItem('caelab_username') || 'Anonymous')

  // ============ 初始化 ============

  function loadData(): void {
    const storedShares = getStorage<ShareProject[]>(SHARES_KEY)
    if (storedShares) shares.value = storedShares

    const storedComments = getStorage<ProjectComment[]>(COMMENTS_KEY)
    if (storedComments) comments.value = storedComments

    const storedLinks = getStorage<ShareLink[]>(LINKS_KEY)
    if (storedLinks) shareLinks.value = storedLinks
  }

  // ============ 分享管理 ============

  /**
   * 创建分享
   */
  function createShare(
    projectId: string,
    projectName: string,
    options?: {
      shareType?: 'link' | 'team' | 'public'
      permissions?: SharePermission[]
    }
  ): ShareProject {
    const share: ShareProject = {
      id: generateId(),
      projectId,
      projectName,
      ownerId: currentUserId.value,
      ownerName: currentUserName.value,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      shareType: options?.shareType || 'team',
      permissions: options?.permissions || [getDefaultPermission('viewer')],
      collaborators: [],
      activityLog: []
    }

    shares.value.unshift(share)
    saveData()

    return share
  }

  /**
   * 获取项目的分享
   */
  function getProjectShare(projectId: string): ShareProject | undefined {
    return shares.value.find(s => s.projectId === projectId)
  }

  /**
   * 更新分享设置
   */
  function updateShare(shareId: string, updates: Partial<ShareProject>): ShareProject | null {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return null

    Object.assign(share, updates, { updatedAt: new Date().toISOString() })
    saveData()
    return share
  }

  /**
   * 删除分享
   */
  function deleteShare(shareId: string): boolean {
    const index = shares.value.findIndex(s => s.id === shareId)
    if (index === -1) return false

    shares.value.splice(index, 1)

    // 同时删除相关的评论和链接
    const share = shares.value[index]
    if (share) {
      comments.value = comments.value.filter(c => c.projectId !== share.projectId)
      shareLinks.value = shareLinks.value.filter(l => l.projectId !== share.projectId)
    }

    saveData()
    return true
  }

  // ============ 协作者管理 ============

  /**
   * 添加协作者
   */
  function addCollaborator(
    shareId: string,
    userName: string,
    permission: SharePermission
  ): ShareCollaborator | null {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return null

    const collaborator: ShareCollaborator = {
      id: generateId(),
      userId: generateId(),
      userName,
      avatarColor: getAvatarColor(userName),
      permission,
      addedAt: new Date().toISOString()
    }

    share.collaborators.push(collaborator)
    share.updatedAt = new Date().toISOString()

    logActivity(shareId, 'permission_change', `${userName} added as ${permission.role}`)

    saveData()
    return collaborator
  }

  /**
   * 移除协作者
   */
  function removeCollaborator(shareId: string, collaboratorId: string): boolean {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return false

    const collaborator = share.collaborators.find(c => c.id === collaboratorId)
    if (!collaborator) return false

    share.collaborators = share.collaborators.filter(c => c.id !== collaboratorId)
    share.updatedAt = new Date().toISOString()

    logActivity(shareId, 'permission_change', `${collaborator.userName} removed`)

    saveData()
    return true
  }

  /**
   * 更新协作者权限
   */
  function updateCollaboratorPermission(shareId: string, collaboratorId: string, permission: SharePermission): boolean {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return false

    const collaborator = share.collaborators.find(c => c.id === collaboratorId)
    if (!collaborator) return false

    collaborator.permission = permission
    share.updatedAt = new Date().toISOString()

    logActivity(shareId, 'permission_change', `${collaborator.userName} permission updated`)

    saveData()
    return true
  }

  // ============ 活动日志 ============

  function logActivity(shareId: string, action: ActivityEntry['action'], details?: string, target?: string): void {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return

    const entry: ActivityEntry = {
      id: generateId(),
      userId: currentUserId.value,
      userName: currentUserName.value,
      action,
      target,
      details,
      timestamp: new Date().toISOString()
    }

    share.activityLog.unshift(entry)

    // 限制活动日志长度
    if (share.activityLog.length > 100) {
      share.activityLog.splice(100)
    }
  }

  /**
   * 获取分享活动日志
   */
  function getActivityLog(shareId: string, limit?: number): ActivityEntry[] {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return []

    const log = share.activityLog
    return limit ? log.slice(0, limit) : log
  }

  // ============ 分享链接 ============

  /**
   * 创建分享链接
   */
  function createShareLink(shareId: string, options?: { expiresAt?: string; permission?: SharePermission }): ShareLink | null {
    const share = shares.value.find(s => s.id === shareId)
    if (!share) return null

    const link: ShareLink = {
      id: generateId(),
      projectId: share.projectId,
      code: generateAccessCode(),
      createdAt: new Date().toISOString(),
      expiresAt: options?.expiresAt,
      permission: options?.permission || getDefaultPermission('viewer'),
      usageCount: 0
    }

    shareLinks.value.push(link)

    logActivity(shareId, 'share', `Share link created: ${link.code}`)

    saveData()
    return link
  }

  /**
   * 通过链接访问分享
   */
  function accessViaLink(code: string): { share: ShareProject; permission: SharePermission } | null {
    const link = shareLinks.value.find(l => l.code === code)
    if (!link) return null

    // 检查是否过期
    if (link.expiresAt && new Date(link.expiresAt) < new Date()) {
      return null
    }

    const share = shares.value.find(s => s.id === link.id || s.projectId === link.projectId)
    if (!share) return null

    // 更新使用统计
    link.usageCount++
    link.lastUsed = new Date().toISOString()

    logActivity(share.id, 'view', `Accessed via link`)

    saveData()
    return { share, permission: link.permission }
  }

  /**
   * 删除分享链接
   */
  function deleteShareLink(shareId: string, linkId: string): boolean {
    const link = shareLinks.value.find(l => l.id === linkId && l.projectId === shareId)
    if (!link) return false

    shareLinks.value = shareLinks.value.filter(l => l.id !== linkId)
    saveData()
    return true
  }

  // ============ 评论功能 ============

  /**
   * 添加评论
   */
  function addComment(
    projectId: string,
    content: string,
    options?: {
      versionId?: string
      resultId?: string
      position?: ProjectComment['position']
    }
  ): ProjectComment {
    const comment: ProjectComment = {
      id: generateId(),
      projectId,
      versionId: options?.versionId,
      resultId: options?.resultId,
      authorId: currentUserId.value,
      authorName: currentUserName.value,
      authorAvatarColor: getAvatarColor(currentUserName.value),
      content,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      position: options?.position,
      isResolved: false,
      replies: []
    }

    comments.value.unshift(comment)

    // 记录到分享活动
    const share = getProjectShare(projectId)
    if (share) {
      logActivity(share.id, 'comment', content.slice(0, 50))
    }

    saveData()
    return comment
  }

  /**
   * 回复评论
   */
  function replyToComment(commentId: string, content: string): CommentReply | null {
    const comment = comments.value.find(c => c.id === commentId)
    if (!comment) return null

    const reply: CommentReply = {
      id: generateId(),
      authorId: currentUserId.value,
      authorName: currentUserName.value,
      content,
      createdAt: new Date().toISOString()
    }

    comment.replies.push(reply)
    comment.updatedAt = new Date().toISOString()

    saveData()
    return reply
  }

  /**
   * 更新评论
   */
  function updateComment(commentId: string, content: string): boolean {
    const comment = comments.value.find(c => c.id === commentId)
    if (!comment) return false

    // 只允许作者更新
    if (comment.authorId !== currentUserId.value) return false

    comment.content = content
    comment.updatedAt = new Date().toISOString()

    saveData()
    return true
  }

  /**
   * 删除评论
   */
  function deleteComment(commentId: string): boolean {
    const index = comments.value.findIndex(c => c.id === commentId)
    if (index === -1) return false

    // 只允许作者删除
    const comment = comments.value[index]
    if (comment.authorId !== currentUserId.value) return false

    comments.value.splice(index, 1)
    saveData()
    return true
  }

  /**
   * 标记评论已解决
   */
  function resolveComment(commentId: string): boolean {
    const comment = comments.value.find(c => c.id === commentId)
    if (!comment) return false

    comment.isResolved = true
    comment.resolvedBy = currentUserName.value
    comment.resolvedAt = new Date().toISOString()

    saveData()
    return true
  }

  /**
   * 获取项目的所有评论
   */
  function getProjectComments(projectId: string, includeResolved?: boolean): ProjectComment[] {
    return comments.value.filter(c =>
      c.projectId === projectId &&
      (includeResolved || !c.isResolved)
    )
  }

  /**
   * 获取特定版本的评论
   */
  function getVersionComments(versionId: string): ProjectComment[] {
    return comments.value.filter(c => c.versionId === versionId)
  }

  // ============ 权限工具 ============

  function getDefaultPermission(role: 'viewer' | 'editor' | 'admin'): SharePermission {
    switch (role) {
      case 'viewer':
        return { role: 'viewer', canViewResults: true, canEditMesh: false, canEditBoundaryConditions: false, canExport: false, canShare: false }
      case 'editor':
        return { role: 'editor', canViewResults: true, canEditMesh: true, canEditBoundaryConditions: true, canExport: true, canShare: true }
      case 'admin':
        return { role: 'admin', canViewResults: true, canEditMesh: true, canEditBoundaryConditions: true, canExport: true, canShare: true }
    }
  }

  function hasPermission(share: ShareProject, action: keyof SharePermission): boolean {
    // 检查当前用户权限
    const collaborator = share.collaborators.find(c => c.userId === currentUserId.value)
    if (!collaborator) {
      // 如果是所有者，有所有权限
      if (share.ownerId === currentUserId.value) {
        return true
      }
      return false
    }

    return collaborator.permission[action] as boolean
  }

  // ============ 统计 ============

  function getStats(): {
    totalShares: number
    totalCollaborators: number
    totalComments: number
    totalLinks: number
  } {
    let totalCollaborators = 0
    for (const share of shares.value) {
      totalCollaborators += share.collaborators.length
    }

    return {
      totalShares: shares.value.length,
      totalCollaborators,
      totalComments: comments.value.length,
      totalLinks: shareLinks.value.length
    }
  }

  // ============ 持久化 ============

  function saveData(): void {
    setStorage(SHARES_KEY, shares.value)
    setStorage(COMMENTS_KEY, comments.value)
    setStorage(LINKS_KEY, shareLinks.value)
  }

  // 初始化
  loadData()

  return {
    // 状态
    shares,
    comments,
    shareLinks,
    currentUserId,
    currentUserName,

    // 分享管理
    createShare,
    getProjectShare,
    updateShare,
    deleteShare,

    // 协作者管理
    addCollaborator,
    removeCollaborator,
    updateCollaboratorPermission,

    // 活动日志
    logActivity,
    getActivityLog,

    // 分享链接
    createShareLink,
    accessViaLink,
    deleteShareLink,

    // 评论功能
    addComment,
    replyToComment,
    updateComment,
    deleteComment,
    resolveComment,
    getProjectComments,
    getVersionComments,

    // 权限工具
    hasPermission,
    getDefaultPermission,

    // 统计
    getStats
  }
}