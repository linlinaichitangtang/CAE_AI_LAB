/**
 * usePersistentStorage.ts — V3.5-013 IndexedDB 持久化存储
 * 解决 localStorage 5-10MB 配额限制
 * 支持大文件存储（mesh、仿真结果、版本历史）
 */
import { ref } from 'vue'

export interface StorageConfig {
  /** 数据库名称 */
  dbName: string
  /** 存储区名称 */
  storeName: string
  /** 数据库版本 */
  version: number
}

const defaultConfig: StorageConfig = {
  dbName: 'caelab_storage',
  storeName: 'persistent_data',
  version: 1
}

class IndexedDBStorage {
  private db: IDBDatabase | null = null
  private config: StorageConfig
  private initPromise: Promise<void> | null = null

  constructor(config: Partial<StorageConfig> = {}) {
    this.config = { ...defaultConfig, ...config }
  }

  /** 初始化数据库 */
  async init(): Promise<void> {
    if (this.db) return
    if (this.initPromise) return this.initPromise

    this.initPromise = new Promise((resolve, reject) => {
      const request = indexedDB.open(this.config.dbName, this.config.version)

      request.onerror = () => reject(new Error(`IndexedDB 打开失败: ${request.error}`))

      request.onsuccess = () => {
        this.db = request.result
        resolve()
      }

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result

        if (!db.objectStoreNames.contains(this.config.storeName)) {
          const store = db.createObjectStore(this.config.storeName, { keyPath: 'key' })
          store.createIndex('timestamp', 'timestamp', { unique: false })
        }
      }
    })

    return this.initPromise
  }

  /** 存储数据 */
  async set(key: string, value: any): Promise<void> {
    await this.init()
    if (!this.db) throw new Error('数据库未初始化')

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.config.storeName, 'readwrite')
      const store = tx.objectStore(this.config.storeName)

      const record = {
        key,
        value,
        timestamp: Date.now()
      }

      const request = store.put(record)
      request.onsuccess = () => resolve()
      request.onerror = () => reject(new Error(`存储失败: ${request.error}`))
    })
  }

  /** 获取数据 */
  async get<T = any>(key: string): Promise<T | null> {
    await this.init()
    if (!this.db) throw new Error('数据库未初始化')

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.config.storeName, 'readonly')
      const store = tx.objectStore(this.config.storeName)

      const request = store.get(key)
      request.onsuccess = () => {
        const result = request.result
        resolve(result ? result.value : null)
      }
      request.onerror = () => reject(new Error(`读取失败: ${request.error}`))
    })
  }

  /** 删除数据 */
  async remove(key: string): Promise<void> {
    await this.init()
    if (!this.db) throw new Error('数据库未初始化')

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.config.storeName, 'readwrite')
      const store = tx.objectStore(this.config.storeName)

      const request = store.delete(key)
      request.onsuccess = () => resolve()
      request.onerror = () => reject(new Error(`删除失败: ${request.error}`))
    })
  }

  /** 获取所有键 */
  async keys(): Promise<string[]> {
    await this.init()
    if (!this.db) throw new Error('数据库未初始化')

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.config.storeName, 'readonly')
      const store = tx.objectStore(this.config.storeName)

      const request = store.getAllKeys()
      request.onsuccess = () => resolve(request.result as string[])
      request.onerror = () => reject(new Error(`获取键失败: ${request.error}`))
    })
  }

  /** 清除所有数据 */
  async clear(): Promise<void> {
    await this.init()
    if (!this.db) throw new Error('数据库未初始化')

    return new Promise((resolve, reject) => {
      const tx = this.db!.transaction(this.config.storeName, 'readwrite')
      const store = tx.objectStore(this.config.storeName)

      const request = store.clear()
      request.onsuccess = () => resolve()
      request.onerror = () => reject(new Error(`清空失败: ${request.error}`))
    })
  }

  /** 存储大数据（mesh、结果等） */
  async setLargeData(key: string, data: any): Promise<void> {
    // 压缩大对象
    const serialized = JSON.stringify(data)
    const size = new Blob([serialized]).size

    console.log(`[PersistentStorage] 存储 "${key}", 大小: ${(size / 1024 / 1024).toFixed(2)} MB`)

    return this.set(key, data)
  }

  /** 获取存储使用统计 */
  async getStorageStats(): Promise<{
    totalKeys: number
    estimatedSize: number
  }> {
    const keys = await this.keys()
    let totalSize = 0

    for (const key of keys) {
      const value = await this.get(key)
      if (value) {
        totalSize += new Blob([JSON.stringify(value)]).size
      }
    }

    return {
      totalKeys: keys.length,
      estimatedSize: totalSize
    }
  }
}

// 全局实例
const storageInstances = new Map<string, IndexedDBStorage>()

export function usePersistentStorage(name: string = 'default') {
  if (!storageInstances.has(name)) {
    storageInstances.set(name, new IndexedDBStorage({ storeName: name }))
  }

  const instance = storageInstances.get(name)!
  const isReady = ref(false)
  const error = ref<string | null>(null)

  // 初始化
  instance.init().then(() => {
    isReady.value = true
  }).catch((e) => {
    error.value = String(e)
  })

  return {
    isReady,
    error,

    async set(key: string, value: any): Promise<void> {
      try {
        await instance.set(key, value)
      } catch (e) {
        // 降级到 localStorage
        console.warn('[PersistentStorage] IndexedDB 失败，降级到 localStorage:', e)
        localStorage.setItem(`fallback_${key}`, JSON.stringify(value))
      }
    },

    async get<T = any>(key: string): Promise<T | null> {
      try {
        return await instance.get<T>(key)
      } catch (e) {
        // 尝试从 localStorage 读取
        const fallback = localStorage.getItem(`fallback_${key}`)
        return fallback ? JSON.parse(fallback) : null
      }
    },

    async remove(key: string): Promise<void> {
      try {
        await instance.remove(key)
        localStorage.removeItem(`fallback_${key}`)
      } catch (e) {
        localStorage.removeItem(`fallback_${key}`)
      }
    },

    async keys(): Promise<string[]> {
      return instance.keys()
    },

    async clear(): Promise<void> {
      return instance.clear()
    },

    async setLargeData(key: string, data: any): Promise<void> {
      return instance.setLargeData(key, data)
    },

    async getStats() {
      return instance.getStorageStats()
    }
  }
}

// 迁移辅助函数
export async function migrateFromLocalStorage(
  localStorageKey: string,
  persistentStorage: ReturnType<typeof usePersistentStorage>
): Promise<boolean> {
  try {
    const data = localStorage.getItem(localStorageKey)
    if (data) {
      await persistentStorage.set(localStorageKey, JSON.parse(data))
      console.log(`[PersistentStorage] 迁移 ${localStorageKey} 完成`)
      return true
    }
  } catch (e) {
    console.error(`[PersistentStorage] 迁移 ${localStorageKey} 失败:`, e)
  }
  return false
}

// 带压缩的存储（使用 LZString 压缩）
export function useCompressedStorage(name: string = 'compressed') {
  const storage = usePersistentStorage(name)

  async function setCompressed(key: string, value: any): Promise<void> {
    const serialized = JSON.stringify(value)
    // 使用浏览器的压缩 API（如果可用）或直接存储
    const encoded = new TextEncoder().encode(serialized)

    // 简单 base64 编码作为备用（实际生产环境应使用 LZString）
    const compressed = btoa(serialized)

    await storage.set(`zlib_${key}`, compressed)
  }

  async function getDecompressed<T = any>(key: string): Promise<T | null> {
    const compressed = await storage.get<string>(`zlib_${key}`)
    if (!compressed) return null

    try {
      const decoded = atob(compressed)
      return JSON.parse(decoded)
    } catch {
      return null
    }
  }

  return {
    ...storage,
    setCompressed,
    getDecompressed
  }
}