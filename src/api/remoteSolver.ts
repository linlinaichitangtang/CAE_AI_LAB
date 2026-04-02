/**
 * 远程仿真提交服务
 * 移动端将仿真任务提交到桌面端或云端求解
 */

export interface RemoteSolverConfig {
  /** 求解服务器地址 */
  serverUrl: string
  /** 认证 token */
  authToken?: string
  /** 超时时间（毫秒） */
  timeout?: number
}

export interface RemoteJob {
  id: string
  status: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled'
  progress: number
  submittedAt: string
  completedAt?: string
  result?: {
    maxDisplacement: number
    maxStress: number
    resultFileUrl: string
  }
  error?: string
}

export interface SubmitJobRequest {
  projectName: string
  analysisType: string
  meshData: {
    nodes: number[][]
    elements: number[][]
    element_type: string
  }
  material: {
    name: string
    elastic_modulus: number
    poisson_ratio: number
    density: number
  }
  boundaryConditions: any[]
  solverConfig?: {
    numThreads?: number
    maxIterations?: number
    convergenceTolerance?: number
  }
}

/**
 * 提交仿真任务到远程服务器
 */
export async function submitRemoteJob(
  config: RemoteSolverConfig,
  request: SubmitJobRequest
): Promise<RemoteJob> {
  const response = await fetch(`${config.serverUrl}/api/solver/submit`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(config.authToken ? { 'Authorization': `Bearer ${config.authToken}` } : {}),
    },
    body: JSON.stringify(request),
    signal: AbortSignal.timeout(config.timeout ?? 30000),
  })

  if (!response.ok) {
    throw new Error(`提交失败: ${response.status} ${response.statusText}`)
  }

  return response.json()
}

/**
 * 查询远程任务状态
 */
export async function getRemoteJobStatus(
  config: RemoteSolverConfig,
  jobId: string
): Promise<RemoteJob> {
  const response = await fetch(`${config.serverUrl}/api/solver/jobs/${jobId}`, {
    headers: config.authToken ? { 'Authorization': `Bearer ${config.authToken}` } : {},
    signal: AbortSignal.timeout(config.timeout ?? 10000),
  })

  if (!response.ok) {
    throw new Error(`查询失败: ${response.status}`)
  }

  return response.json()
}

/**
 * 取消远程任务
 */
export async function cancelRemoteJob(
  config: RemoteSolverConfig,
  jobId: string
): Promise<void> {
  const response = await fetch(`${config.serverUrl}/api/solver/jobs/${jobId}/cancel`, {
    method: 'POST',
    headers: config.authToken ? { 'Authorization': `Bearer ${config.authToken}` } : {},
    signal: AbortSignal.timeout(10000),
  })

  if (!response.ok) {
    throw new Error(`取消失败: ${response.status}`)
  }
}

/**
 * 下载远程任务结果
 */
export async function downloadRemoteResult(
  config: RemoteSolverConfig,
  resultFileUrl: string
): Promise<Blob> {
  const response = await fetch(`${config.serverUrl}${resultFileUrl}`, {
    headers: config.authToken ? { 'Authorization': `Bearer ${config.authToken}` } : {},
    signal: AbortSignal.timeout(60000),
  })

  if (!response.ok) {
    throw new Error(`下载失败: ${response.status}`)
  }

  return response.blob()
}

/**
 * 轮询任务状态直到完成
 */
export async function pollJobUntilComplete(
  config: RemoteSolverConfig,
  jobId: string,
  onProgress?: (job: RemoteJob) => void,
  pollInterval: number = 2000,
  maxPolls: number = 1800 // 1 小时
): Promise<RemoteJob> {
  for (let i = 0; i < maxPolls; i++) {
    const job = await getRemoteJobStatus(config, jobId)
    onProgress?.(job)

    if (job.status === 'completed' || job.status === 'failed' || job.status === 'cancelled') {
      return job
    }

    await new Promise(resolve => setTimeout(resolve, pollInterval))
  }

  throw new Error('轮询超时')
}
