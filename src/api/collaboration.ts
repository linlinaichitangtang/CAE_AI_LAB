/**
 * CAELab Collaboration API
 * Team project sharing and comment/annotation functionality
 */

import { invoke } from '@tauri-apps/api/core'

// ============ Types ============

export interface ProjectShare {
  id: string
  project_id: string
  shared_with_name: string
  permission: 'read' | 'write' | 'admin'
  created_at: string
}

export interface Comment {
  id: string
  project_id: string
  note_id: string | null
  author_name: string
  content: string
  mention_ids: string | null
  created_at: string
  updated_at: string
  resolved: boolean
}

// ============ Project Share API ============

export async function shareProject(
  projectId: string,
  sharedWithName: string,
  permission: 'read' | 'write' | 'admin'
): Promise<ProjectShare> {
  return await invoke<ProjectShare>('share_project', {
    projectId,
    sharedWithName,
    permission,
  })
}

export async function listProjectShares(projectId: string): Promise<ProjectShare[]> {
  return await invoke<ProjectShare[]>('list_project_shares', { projectId })
}

export async function removeProjectShare(shareId: string): Promise<void> {
  await invoke('remove_project_share', { shareId })
}

export async function updateSharePermission(
  shareId: string,
  permission: 'read' | 'write' | 'admin'
): Promise<void> {
  await invoke('update_share_permission', { shareId, permission })
}

// ============ Comment API ============

export async function addComment(
  projectId: string,
  authorName: string,
  content: string,
  noteId?: string | null,
  mentionIds?: string | null
): Promise<Comment> {
  return await invoke<Comment>('add_comment', {
    projectId,
    noteId: noteId || null,
    authorName,
    content,
    mentionIds: mentionIds || null,
  })
}

export async function listComments(
  projectId: string,
  noteId?: string | null
): Promise<Comment[]> {
  return await invoke<Comment[]>('list_comments', {
    projectId,
    noteId: noteId || null,
  })
}

export async function updateComment(
  commentId: string,
  content: string
): Promise<Comment> {
  return await invoke<Comment>('update_comment', { commentId, content })
}

export async function deleteComment(commentId: string): Promise<void> {
  await invoke('delete_comment', { commentId })
}

export async function resolveComment(
  commentId: string,
  resolved: boolean
): Promise<void> {
  await invoke('resolve_comment', { commentId, resolved })
}
