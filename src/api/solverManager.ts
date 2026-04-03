/**
 * 求解器管理 API
 * 通过 Tauri invoke 调用后端求解器管理命令
 */

import { invoke } from '@tauri-apps/api/core'

// ============================================
// 类型定义
// ============================================

export interface SolverStatus {
  name: string
  display_name: string
  scale: string
  installed: boolean
  version: string | null
  path: string | null
  install_method: string | null
}

export interface SolverVerifyResult {
  works: boolean
  version: string | null
  error_message: string | null
}

export interface InstallResult {
  success: boolean
  message: string
  duration_s: number
}

export interface UninstallResult {
  success: boolean
  message: string
}

export interface InstallMethod {
  name: string
  display_name: string
  available: boolean
}

export interface SolverInfo {
  name: string
  display_name: string
  scale: string
  description: string
  version: string | null
  path: string | null
  installed: boolean
  install_methods: string[]
  estimated_size_mb: number
  website: string
}

// ============================================
// API 函数
// ============================================

/** 检测系统中已安装的求解器 */
export async function detectSolvers(): Promise<SolverStatus[]> {
  return invoke<SolverStatus[]>('detect_solvers')
}

/** 验证指定求解器是否可用 */
export async function checkSolverWorks(solverName: string): Promise<SolverVerifyResult> {
  return invoke<SolverVerifyResult>('check_solver_works', { solverName })
}

/** 安装指定求解器 */
export async function installSolver(solverName: string, method: string): Promise<InstallResult> {
  return invoke<InstallResult>('install_solver', { solverName, method })
}

/** 卸载指定求解器 */
export async function uninstallSolver(solverName: string): Promise<UninstallResult> {
  return invoke<UninstallResult>('uninstall_solver', { solverName })
}

/** 获取可用的安装方法列表 */
export async function getInstallMethods(): Promise<InstallMethod[]> {
  return invoke<InstallMethod[]>('get_install_methods')
}

/** 获取指定求解器的详细信息 */
export async function getSolverInfo(solverName: string): Promise<SolverInfo> {
  return invoke<SolverInfo>('get_solver_info', { solverName })
}
