/**
 * useDockerDeployment.ts — V3.4-003 本地私有化部署
 * Docker 一键部署，数据不出本地
 */

import { ref, computed } from 'vue'

// ============ 类型定义 ============

export type DeploymentStatus = 'idle' | 'preparing' | 'pulling' | 'building' | 'starting' | 'running' | 'stopped' | 'error' | 'removing'
export type ContainerStatus = 'created' | 'running' | 'paused' | 'restarting' | 'removing' | 'exited' | 'dead'
export type VolumeType = 'data' | 'config' | 'logs' | 'cache'

export interface DockerConfig {
  enabled: boolean
  registry?: string
  imagePrefix?: string
  proxy?: {
    enabled: boolean
    http?: string
    https?: string
    noProxy?: string
  }
}

export interface DockerImage {
  id: string
  name: string
  tag: string
  size: number
  created: string
  digest?: string
}

export interface DockerContainer {
  id: string
  name: string
  image: string
  status: ContainerStatus
  ports: Array<{ host: number; container: number; protocol: string }>
  volumes: Array<{ host: string; container: string; mode: 'rw' | 'ro' }>
  created: string
  started: string
  env: Record<string, string>
}

export interface DeploymentEnvironment {
  id: string
  name: string
  description: string
  type: 'development' | 'staging' | 'production'
  config: ServiceConfig
  containers: ContainerConfig[]
  networks: NetworkConfig[]
  volumes: VolumeConfig[]
}

export interface ServiceConfig {
  host: string
  port: number
  ssl: boolean
  sslCert?: string
  sslKey?: string
  storagePath: string
  dataPath: string
  logPath: string
}

export interface ContainerConfig {
  id: string
  name: string
  image: string
  tag: string
  ports: Array<{ host: number; container: number; protocol: string }>
  volumes: Array<{ name: string; hostPath?: string; containerPath: string; mode: 'rw' | 'ro' }>
  environment: Record<string, string>
  dependencies: string[]  // 其他容器名称
  healthCheck: {
    enabled: boolean
    command: string
    interval: number
    timeout: number
    retries: number
  }
  resources: {
    memory?: string
    cpu?: string
    gpu?: boolean
    gpuCount?: number
  }
  autoRestart: boolean
}

export interface NetworkConfig {
  id: string
  name: string
  driver: 'bridge' | 'host' | 'overlay' | 'none'
  subnet?: string
  gateway?: string
}

export interface VolumeConfig {
  id: string
  name: string
  type: VolumeType
  hostPath?: string
  size?: string  // 用于 tmpfs 或配额
  driver?: string
}

export interface DeploymentLog {
  id: string
  timestamp: string
  level: 'info' | 'warn' | 'error' | 'debug'
  container?: string
  message: string
}

export interface DeploymentProgress {
  stage: DeploymentStatus
  progress: number
  currentAction?: string
  errorMessage?: string
}

export interface SystemResources {
  memory: {
    total: number
    used: number
    available: number
  }
  cpu: {
    cores: number
    usage: number
  }
  disk: {
    total: number
    used: number
    available: number
  }
  docker: {
    containers: number
    images: number
    volumes: number
  }
}

// ============ 部署模板 ============

export const DEPLOYMENT_TEMPLATES: Omit<DeploymentEnvironment, 'id'>[] = [
  {
    name: '标准部署',
    description: '单节点完整部署，包含所有服务',
    type: 'production',
    config: {
      host: 'localhost',
      port: 3000,
      ssl: false,
      storagePath: './data/storage',
      dataPath: './data/projects',
      logPath: './data/logs'
    },
    containers: [
      {
        id: 'caelab-app',
        name: 'caelab-app',
        image: 'caelab/caelab-app',
        tag: 'latest',
        ports: [{ host: 3000, container: 3000, protocol: 'tcp' }],
        volumes: [
          { name: 'caelab-data', containerPath: '/app/data', mode: 'rw' },
          { name: 'caelab-logs', containerPath: '/app/logs', mode: 'rw' }
        ],
        environment: {
          NODE_ENV: 'production',
          PORT: '3000'
        },
        dependencies: [],
        healthCheck: {
          enabled: true,
          command: 'curl -f http://localhost:3000/health || exit 1',
          interval: 30,
          timeout: 10,
          retries: 3
        },
        resources: { memory: '4g', cpu: '2' },
        autoRestart: true
      },
      {
        id: 'caelab-db',
        name: 'caelab-postgres',
        image: 'postgres',
        tag: '15',
        ports: [{ host: 5432, container: 5432, protocol: 'tcp' }],
        volumes: [
          { name: 'caelab-db-data', containerPath: '/var/lib/postgresql/data', mode: 'rw' }
        ],
        environment: {
          POSTGRES_DB: 'caelab',
          POSTGRES_USER: 'caelab',
          POSTGRES_PASSWORD: 'changeme'
        },
        dependencies: [],
        healthCheck: {
          enabled: true,
          command: 'pg_isready -U caelab',
          interval: 30,
          timeout: 10,
          retries: 3
        },
        resources: { memory: '2g', cpu: '1' },
        autoRestart: true
      }
    ],
    networks: [
      { id: 'caelab-net', name: 'caelab-network', driver: 'bridge', subnet: '172.20.0.0/16' }
    ],
    volumes: [
      { id: 'caelab-data', name: 'caelab-data', type: 'data' },
      { id: 'caelab-logs', name: 'caelab-logs', type: 'logs' },
      { id: 'caelab-db-data', name: 'caelab-db-data', type: 'data' }
    ]
  },
  {
    name: '最小部署',
    description: '仅核心服务，最小资源占用',
    type: 'development',
    config: {
      host: 'localhost',
      port: 3000,
      ssl: false,
      storagePath: './data/storage',
      dataPath: './data/projects',
      logPath: './data/logs'
    },
    containers: [
      {
        id: 'caelab-app',
        name: 'caelab-app',
        image: 'caelab/caelab-app',
        tag: 'latest',
        ports: [{ host: 3000, container: 3000, protocol: 'tcp' }],
        volumes: [
          { name: 'caelab-data', containerPath: '/app/data', mode: 'rw' }
        ],
        environment: {
          NODE_ENV: 'development',
          PORT: '3000',
          USE_SQLITE: 'true'
        },
        dependencies: [],
        healthCheck: {
          enabled: true,
          command: 'curl -f http://localhost:3000/health || exit 1',
          interval: 60,
          timeout: 30,
          retries: 3
        },
        resources: { memory: '1g', cpu: '0.5' },
        autoRestart: true
      }
    ],
    networks: [
      { id: 'caelab-net', name: 'caelab-network', driver: 'bridge' }
    ],
    volumes: [
      { id: 'caelab-data', name: 'caelab-data', type: 'data' }
    ]
  }
]

// ============ Docker Compose 模板 ============

const DOCKER_COMPOSE_TEMPLATE = `version: '3.8'

services:
{{containers}}

networks:
{{networks}}

volumes:
{{volumes}}
`

const CONTAINER_TEMPLATE = `  {{name}}:
    image: {{image}}:{{tag}}
    container_name: {{name}}
    ports:
{{ports}}    environment:
{{env}}    volumes:
{{volumes}}    restart: {{restart}}
    healthcheck:
      test: "{{healthcheck}}"
      interval: {{interval}}s
      timeout: {{timeout}}s
      retries: {{retries}}
    networks:
      - {{network}}
{{resources}}`

// ============ 存储键 ============

const CONFIG_KEY = 'caelab_docker_config'
const ENVIRONMENTS_KEY = 'caelab_deployment_envs'
const LOGS_KEY = 'caelab_deployment_logs'

// ============ 工具函数 ============

function generateId(): string {
  return `deploy_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`
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
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`
}

// ============ 主 Composable ============

export function useDockerDeployment() {
  const config = ref<DockerConfig>({
    enabled: false,
    registry: 'docker.io',
    imagePrefix: 'caelab'
  })

  const environments = ref<DeploymentEnvironment[]>([])
  const currentEnvironment = ref<DeploymentEnvironment | null>(null)
  const containers = ref<DockerContainer[]>([])
  const images = ref<DockerImage[]>([])
  const logs = ref<DeploymentLog[]>([])

  const deploymentProgress = ref<DeploymentProgress>({
    stage: 'idle',
    progress: 0
  })

  const systemResources = ref<SystemResources>({
    memory: { total: 0, used: 0, available: 0 },
    cpu: { cores: 0, usage: 0 },
    disk: { total: 0, used: 0, available: 0 },
    docker: { containers: 0, images: 0, volumes: 0 }
  })

  // ============ 初始化 ============

  function loadData(): void {
    const storedConfig = getStorage<DockerConfig>(CONFIG_KEY)
    if (storedConfig) config.value = storedConfig

    const storedEnvs = getStorage<DeploymentEnvironment[]>(ENVIRONMENTS_KEY)
    if (storedEnvs) environments.value = storedEnvs

    const storedLogs = getStorage<DeploymentLog[]>(LOGS_KEY)
    if (storedLogs) logs.value = storedLogs
  }

  // ============ 配置管理 ============

  function updateConfig(updates: Partial<DockerConfig>): void {
    Object.assign(config.value, updates)
    setStorage(CONFIG_KEY, config.value)
  }

  // ============ 环境管理 ============

  function createEnvironment(
    name: string,
    description: string,
    type: DeploymentEnvironment['type'],
    templateIndex: number = 0
  ): DeploymentEnvironment {
    const template = DEPLOYMENT_TEMPLATES[templateIndex]

    const env: DeploymentEnvironment = {
      id: generateId(),
      name,
      description,
      type,
      config: { ...template.config },
      containers: template.containers.map(c => ({ ...c, id: generateId() })),
      networks: [...template.networks],
      volumes: [...template.volumes]
    }

    environments.value.push(env)
    saveEnvironments()
    return env
  }

  function updateEnvironment(envId: string, updates: Partial<DeploymentEnvironment>): boolean {
    const env = environments.value.find(e => e.id === envId)
    if (!env) return false
    Object.assign(env, updates)
    saveEnvironments()
    return true
  }

  function deleteEnvironment(envId: string): boolean {
    const index = environments.value.findIndex(e => e.id === envId)
    if (index === -1) return false
    environments.value.splice(index, 1)
    saveEnvironments()
    return true
  }

  function cloneEnvironment(envId: string, newName: string): DeploymentEnvironment | null {
    const env = environments.value.find(e => e.id === envId)
    if (!env) return null

    const cloned: DeploymentEnvironment = {
      ...env,
      id: generateId(),
      name: newName,
      containers: env.containers.map(c => ({ ...c, id: generateId() })),
      networks: env.networks.map(n => ({ ...n, id: generateId() })),
      volumes: env.volumes.map(v => ({ ...v, id: generateId() }))
    }

    environments.value.push(cloned)
    saveEnvironments()
    return cloned
  }

  // ============ Docker Compose 生成 ============

  function generateDockerCompose(env: DeploymentEnvironment): string {
    const networkName = env.networks[0]?.name || 'caelab-network'

    // 生成 services
    const servicesStr = env.containers.map(container => {
      const ports = container.ports
        .map(p => `      - "${p.host}:${p.container}/${p.protocol}"`)
        .join('\n')

      const envStr = Object.entries(container.environment)
        .map(([k, v]) => `      ${k}: "${v}"`)
        .join('\n')

      const volumesStr = container.volumes
        .map(v => `      - ${v.name}:${v.containerPath}:${v.mode}`)
        .join('\n')

      const restart = container.autoRestart ? 'unless-stopped' : 'no'

      let resourcesStr = ''
      if (container.resources.memory || container.resources.cpu) {
        resourcesStr = `    deploy:\n      resources:\n`
        if (container.resources.memory) {
          resourcesStr += `        limits:\n          memory: ${container.resources.memory}\n`
        }
        if (container.resources.cpu) {
          resourcesStr += `        limits:\n          cpus: "${container.resources.cpu}"\n`
        }
      }

      const healthcheck = container.healthCheck.enabled
        ? container.healthCheck.command.replace(/"/g, '\'')
        : 'true'

      return CONTAINER_TEMPLATE
        .replace('{{name}}', container.name)
        .replace('{{image}}', container.image)
        .replace('{{tag}}', container.tag)
        .replace('{{ports}}', ports)
        .replace('{{env}}', envStr)
        .replace('{{volumes}}', volumesStr)
        .replace('{{restart}}', restart)
        .replace('{{healthcheck}}', healthcheck)
        .replace('{{interval}}', String(container.healthCheck.interval))
        .replace('{{timeout}}', String(container.healthCheck.timeout))
        .replace('{{retries}}', String(container.healthCheck.retries))
        .replace('{{network}}', networkName)
        .replace('{{resources}}', resourcesStr)
    }).join('\n\n')

    // 生成 networks
    const networksStr = env.networks.map(n => `  ${n.name}:\n    driver: ${n.driver}${n.subnet ? `\n    ipam:\n      config:\n        - subnet: ${n.subnet}` : ''}`).join('\n')

    // 生成 volumes
    const volumesStr = env.volumes.map(v => `  ${v.name}:`).join('\n')

    return DOCKER_COMPOSE_TEMPLATE
      .replace('{{containers}}', servicesStr)
      .replace('{{networks}}', networksStr)
      .replace('{{volumes}}', volumesStr)
  }

  /**
   * 下载 docker-compose.yml
   */
  function downloadDockerCompose(envId: string): void {
    const env = environments.value.find(e => e.id === envId)
    if (!env) return

    const content = generateDockerCompose(env)
    const blob = new Blob([content], { type: 'text/yaml' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = 'docker-compose.yml'
    a.click()

    URL.revokeObjectURL(url)
  }

  /**
   * 导出完整部署包
   */
  function exportDeploymentPackage(envId: string): void {
    const env = environments.value.find(e => e.id === envId)
    if (!env) return

    const dockerCompose = generateDockerCompose(env)
    const envContent = generateEnvFile(env)
    const readme = generateReadme(env)

    const content = `
# CAELab 部署包

## 文件说明

- docker-compose.yml: Docker Compose 配置
- .env: 环境变量配置（请根据实际情况修改）
- README.md: 部署说明

${readme}

${'```bash'}\n# 启动部署\ndocker-compose up -d\n\n# 查看状态\ndocker-compose ps\n\n# 查看日志\ndocker-compose logs -f\n\n# 停止部署\ndocker-compose down\n${'```'}
`

    const blob = new Blob([content], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = 'caelab-deployment.tar.gz'
    a.click()

    URL.revokeObjectURL(url)
  }

  function generateEnvFile(env: DeploymentEnvironment): string {
    const envVars: Record<string, string> = {
      CAELAB_HOST: env.config.host,
      CAELAB_PORT: String(env.config.port),
      CAELAB_SSL: env.config.ssl ? 'true' : 'false',
      CAELAB_DATA_PATH: env.config.dataPath,
      CAELAB_STORAGE_PATH: env.config.storagePath,
      CAELAB_LOG_PATH: env.config.logPath
    }

    // 添加数据库配置
    const dbContainer = env.containers.find(c => c.name.includes('postgres') || c.name.includes('db'))
    if (dbContainer) {
      envVars.POSTGRES_DB = 'caelab'
      envVars.POSTGRES_USER = 'caelab'
      envVars.POSTGRES_PASSWORD = 'changeme_this_in_production'
    }

    return Object.entries(envVars)
      .map(([k, v]) => `${k}=${v}`)
      .join('\n')
  }

  function generateReadme(env: DeploymentEnvironment): string {
    return `# ${env.name}

${env.description}

## 系统要求

- Docker 20.10+
- Docker Compose 2.0+
- ${env.type === 'production' ? '4GB+ RAM, 2+ CPU cores' : '2GB+ RAM, 1+ CPU core'}

## 快速开始

1. 编辑 .env 文件，设置必要的环境变量
2. 运行部署脚本:

\`\`\`bash
docker-compose up -d
\`\`\`

3. 访问 http://${env.config.host}:${env.config.port}

## 服务端口

${env.containers.map(c => `- ${c.name}: ${c.ports.map(p => `${p.host}`).join(', ')}`).join('\n')}

## 数据持久化

数据存储在以下 Docker volumes 中:
${env.volumes.map(v => `- ${v.name}`).join('\n')}

## 备份与恢复

\`\`\`bash
# 备份
docker-compose down
tar -czf backup.tar.gz ./data

# 恢复
tar -xzf backup.tar.gz
docker-compose up -d
\`\`\`
`
  }

  // ============ 部署操作（模拟） ============

  async function deployEnvironment(envId: string): Promise<boolean> {
    const env = environments.value.find(e => e.id === envId)
    if (!env) return false

    deploymentProgress.value = { stage: 'preparing', progress: 0 }
    currentEnvironment.value = env

    addLog('info', 'System', `开始部署环境: ${env.name}`)

    try {
      // 阶段1: 拉取镜像
      deploymentProgress.value = { stage: 'pulling', progress: 20, currentAction: '拉取镜像' }
      for (const container of env.containers) {
        addLog('info', container.image, `拉取镜像 ${container.image}:${container.tag}`)
        await simulateDelay(500)
      }

      // 阶段2: 创建网络
      deploymentProgress.value = { stage: 'building', progress: 40, currentAction: '创建网络' }
      for (const network of env.networks) {
        addLog('info', network.name, `创建网络 ${network.name}`)
        await simulateDelay(300)
      }

      // 阶段3: 创建卷
      deploymentProgress.value = { stage: 'building', progress: 50, currentAction: '创建存储卷' }
      for (const volume of env.volumes) {
        addLog('info', volume.name, `创建卷 ${volume.name}`)
        await simulateDelay(200)
      }

      // 阶段4: 启动容器
      deploymentProgress.value = { stage: 'starting', progress: 70, currentAction: '启动容器' }
      for (const container of env.containers) {
        addLog('info', container.name, `启动容器 ${container.name}`)
        await simulateDelay(400)

        // 模拟健康检查
        if (container.healthCheck.enabled) {
          addLog('info', container.name, `健康检查通过`)
          await simulateDelay(200)
        }
      }

      // 完成
      deploymentProgress.value = { stage: 'running', progress: 100 }
      addLog('info', '部署完成', `${env.name} 已成功启动`)

      // 更新容器状态
      updateContainerStatus(env)

      return true
    } catch (e: any) {
      deploymentProgress.value = { stage: 'error', progress: 0, errorMessage: e.message }
      addLog('error', '部署失败', e.message)
      return false
    }
  }

  async function stopEnvironment(envId: string): Promise<boolean> {
    const env = environments.value.find(e => e.id === envId)
    if (!env) return false

    deploymentProgress.value = { stage: 'stopped', progress: 0 }
    addLog('info', 'System', `停止环境: ${env.name}`)

    try {
      for (const container of env.containers) {
        addLog('info', container.name, `停止容器 ${container.name}`)
        await simulateDelay(300)
      }

      deploymentProgress.value = { stage: 'stopped', progress: 100 }
      addLog('info', '环境已停止', env.name)

      return true
    } catch (e: any) {
      deploymentProgress.value = { stage: 'error', progress: 0, errorMessage: e.message }
      return false
    }
  }

  async function removeEnvironment(envId: string): Promise<boolean> {
    deploymentProgress.value = { stage: 'removing', progress: 0 }
    addLog('info', 'System', `移除环境`)

    try {
      await simulateDelay(1000)
      deploymentProgress.value = { stage: 'idle', progress: 100 }
      addLog('info', 'System', '环境已移除')
      return true
    } catch (e: any) {
      deploymentProgress.value = { stage: 'error', progress: 0, errorMessage: e.message }
      return false
    }
  }

  function updateContainerStatus(env: DeploymentEnvironment): void {
    containers.value = env.containers.map(c => ({
      id: c.id,
      name: c.name,
      image: `${c.image}:${c.tag}`,
      status: 'running' as ContainerStatus,
      ports: c.ports,
      volumes: c.volumes.map(v => ({
        host: v.hostPath || v.name,
        container: v.containerPath,
        mode: v.mode
      })),
      created: new Date().toISOString(),
      started: new Date().toISOString(),
      env: c.environment
    }))
  }

  // ============ 容器操作 ============

  function getContainer(name: string): DockerContainer | undefined {
    return containers.value.find(c => c.name === name)
  }

  async function startContainer(name: string): Promise<boolean> {
    addLog('info', name, `启动容器`)
    await simulateDelay(500)
    const container = containers.value.find(c => c.name === name)
    if (container) container.status = 'running'
    return true
  }

  async function stopContainer(name: string): Promise<boolean> {
    addLog('info', name, `停止容器`)
    await simulateDelay(500)
    const container = containers.value.find(c => c.name === name)
    if (container) container.status = 'exited'
    return true
  }

  async function restartContainer(name: string): Promise<boolean> {
    addLog('info', name, `重启容器`)
    await simulateDelay(800)
    return true
  }

  // ============ 日志 ============

  function addLog(level: DeploymentLog['level'], container: string, message: string): void {
    const entry: DeploymentLog = {
      id: generateId(),
      timestamp: new Date().toISOString(),
      level,
      container,
      message
    }
    logs.value.unshift(entry)

    // 限制日志数量
    if (logs.value.length > 1000) {
      logs.value = logs.value.slice(0, 1000)
    }

    setStorage(LOGS_KEY, logs.value)
  }

  function getLogs(options?: {
    container?: string
    level?: DeploymentLog['level']
    since?: string
    limit?: number
  }): DeploymentLog[] {
    let filtered = logs.value

    if (options?.container) {
      filtered = filtered.filter(l => l.container === options.container)
    }
    if (options?.level) {
      filtered = filtered.filter(l => l.level === options.level)
    }
    if (options?.since) {
      const sinceTime = new Date(options.since).getTime()
      filtered = filtered.filter(l => new Date(l.timestamp).getTime() > sinceTime)
    }

    return filtered.slice(0, options?.limit || 100)
  }

  function clearLogs(): void {
    logs.value = []
    setStorage(LOGS_KEY, logs.value)
  }

  // ============ 系统资源 ============

  async function refreshSystemResources(): Promise<void> {
    // 模拟获取系统资源
    systemResources.value = {
      memory: {
        total: 16 * 1024 * 1024 * 1024,
        used: 8 * 1024 * 1024 * 1024,
        available: 8 * 1024 * 1024 * 1024
      },
      cpu: {
        cores: 8,
        usage: 25 + Math.random() * 30
      },
      disk: {
        total: 500 * 1024 * 1024 * 1024,
        used: 200 * 1024 * 1024 * 1024,
        available: 300 * 1024 * 1024 * 1024
      },
      docker: {
        containers: containers.value.length,
        images: images.value.length,
        volumes: currentEnvironment.value?.volumes.length || 0
      }
    }
  }

  // ============ 健康检查 ============

  async function checkDeploymentHealth(): Promise<{
    healthy: boolean
    services: Array<{ name: string; status: 'up' | 'down' | 'degraded'; message?: string }>
  }> {
    const services: Array<{ name: string; status: 'up' | 'down' | 'degraded'; message?: string }> = []

    for (const container of containers.value) {
      if (container.status === 'running') {
        services.push({ name: container.name, status: 'up' })
      } else if (container.status === 'paused') {
        services.push({ name: container.name, status: 'degraded', message: 'Paused' })
      } else {
        services.push({ name: container.name, status: 'down', message: 'Not running' })
      }
    }

    return {
      healthy: services.every(s => s.status === 'up'),
      services
    }
  }

  // ============ 辅助函数 ============

  function simulateDelay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  function saveEnvironments(): void {
    setStorage(ENVIRONMENTS_KEY, environments.value)
  }

  // ============ 计算属性 ============

  const isDeployed = computed(() =>
    deploymentProgress.value.stage === 'running'
  )

  const deploymentStatus = computed(() => deploymentProgress.value.stage)

  const resourceUsage = computed(() => ({
    memoryPercent: Math.round((systemResources.value.memory.used / systemResources.value.memory.total) * 100),
    cpuPercent: Math.round(systemResources.value.cpu.usage),
    diskPercent: Math.round((systemResources.value.disk.used / systemResources.value.disk.total) * 100)
  }))

  // 初始化
  loadData()

  return {
    // 状态
    config,
    environments,
    currentEnvironment,
    containers,
    images,
    logs,
    deploymentProgress,
    systemResources,

    // 计算属性
    isDeployed,
    deploymentStatus,
    resourceUsage,

    // 配置
    updateConfig,

    // 环境管理
    createEnvironment,
    updateEnvironment,
    deleteEnvironment,
    cloneEnvironment,

    // Docker Compose
    generateDockerCompose,
    downloadDockerCompose,
    exportDeploymentPackage,

    // 部署操作
    deployEnvironment,
    stopEnvironment,
    removeEnvironment,

    // 容器操作
    getContainer,
    startContainer,
    stopContainer,
    restartContainer,

    // 日志
    getLogs,
    clearLogs,

    // 系统资源
    refreshSystemResources,

    // 健康检查
    checkDeploymentHealth,

    // 模板
    DEPLOYMENT_TEMPLATES
  }
}
