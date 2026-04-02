<template>
  <div class="h-full flex flex-col" style="background: var(--bg-base)">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b" style="background: var(--bg-surface); border-color: var(--border-subtle)">
      <div>
        <h2 class="text-lg font-semibold" style="color: var(--text-primary)">仿真数据资产管理</h2>
        <p class="text-sm" style="color: var(--text-muted)">版本化存储 / 标签 / 检索 / 权限，数据血缘追踪，导出溯源报告</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="showCreateModal = true" class="btn btn-primary text-xs">
          新建资产
        </button>
        <button
          @click="exportLineage"
          :disabled="!selectedAsset"
          class="btn btn-ghost text-xs"
        >
          导出溯源报告
        </button>
        <button
          @click="showPermissionModal = true"
          :disabled="!selectedAsset"
          class="btn btn-ghost text-xs"
        >
          设置权限
        </button>
        <button
          @click="archiveAsset"
          :disabled="!selectedAsset || selectedAsset.status === 'archived'"
          class="btn btn-ghost text-xs"
        >
          归档
        </button>
        <button
          @click="toggleLock"
          :disabled="!selectedAsset"
          class="btn btn-ghost text-xs"
        >
          {{ selectedAsset?.status === 'locked' ? '解锁' : '锁定' }}
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Panel (w-80) -->
      <div class="w-80 overflow-y-auto p-4 space-y-4 border-r" style="background: var(--bg-surface); border-color: var(--border-subtle)">

        <!-- Search & Filter -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">搜索过滤</h4>
          <div class="space-y-2">
            <div>
              <label class="label">关键词</label>
              <input v-model="searchFilter.keyword" type="text" class="input w-full text-xs" placeholder="搜索资产名称..." @input="applyFilter" />
            </div>
            <div>
              <label class="label">标签</label>
              <input v-model="tagInput" type="text" class="input w-full text-xs" placeholder="输入标签后回车" @keydown.enter="addFilterTag" />
            </div>
            <div v-if="(searchFilter.tags ?? []).length > 0" class="flex flex-wrap gap-1">
              <span
                v-for="(tag, idx) in (searchFilter.tags ?? [])"
                :key="idx"
                class="inline-flex items-center gap-1 px-2 py-0.5 text-[10px] rounded-full cursor-pointer"
                style="background: var(--primary); color: white"
                @click="removeFilterTag(idx)"
              >
                {{ tag }} &times;
              </span>
            </div>
            <div>
              <label class="label">作者</label>
              <input v-model="searchFilter.author" type="text" class="input w-full text-xs" placeholder="作者名" @input="applyFilter" />
            </div>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="label">开始日期</label>
                <input v-model="searchFilter.date_range!.from" type="date" class="input w-full text-xs" @change="applyFilter" />
              </div>
              <div>
                <label class="label">结束日期</label>
                <input v-model="searchFilter.date_range!.to" type="date" class="input w-full text-xs" @change="applyFilter" />
              </div>
            </div>
            <div>
              <label class="label">文件类型</label>
              <select v-model="searchFilter.file_type" class="input w-full text-xs" @change="applyFilter">
                <option value="">全部类型</option>
                <option value="inp">INP (Abaqus)</option>
                <option value="odb">ODB (结果)</option>
                <option value="csv">CSV (数据)</option>
                <option value="vtk">VTK (可视化)</option>
                <option value="stl">STL (网格)</option>
                <option value="json">JSON (配置)</option>
              </select>
            </div>
            <div>
              <label class="label">项目</label>
              <select v-model="searchFilter.project_id" class="input w-full text-xs" @change="applyFilter">
                <option value="">全部项目</option>
                <option value="proj-001">航空发动机叶片</option>
                <option value="proj-002">汽车碰撞仿真</option>
                <option value="proj-003">复合材料层合板</option>
                <option value="proj-004">热交换器优化</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Tag Management -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">标签管理</h4>
          <div class="flex flex-wrap gap-1">
            <span
              v-for="tag in allTags"
              :key="tag"
              @click="toggleTagFilter(tag)"
              :class="['inline-flex items-center px-2 py-0.5 text-[10px] rounded-full cursor-pointer transition',
                activeTags.has(tag) ? 'text-white' : '']"
              :style="activeTags.has(tag)
                ? 'background: var(--primary)'
                : 'background: var(--bg-elevated); color: var(--text-secondary); border: 1px solid var(--border-default)'"
            >
              {{ tag }}
            </span>
          </div>
        </div>

        <!-- Quick Stats -->
        <div class="panel-section">
          <h4 class="text-sm font-medium mb-3" style="color: var(--text-primary)">资产统计</h4>
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <span class="text-xs" style="color: var(--text-secondary)">总资产数</span>
              <span class="text-xs font-mono font-semibold" style="color: var(--text-primary)">{{ assets.length }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-xs" style="color: var(--text-secondary)">活跃</span>
              <span class="text-xs font-mono" style="color: var(--accent-green)">{{ assets.filter(a => a.status === 'active').length }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-xs" style="color: var(--text-secondary)">已归档</span>
              <span class="text-xs font-mono" style="color: var(--text-muted)">{{ assets.filter(a => a.status === 'archived').length }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-xs" style="color: var(--text-secondary)">已锁定</span>
              <span class="text-xs font-mono" style="color: var(--accent-red)">{{ assets.filter(a => a.status === 'locked').length }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-xs" style="color: var(--text-secondary)">总大小</span>
              <span class="text-xs font-mono" style="color: var(--text-primary)">{{ totalSizeMB.toFixed(1) }} MB</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Main Content -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Asset List Table -->
        <div class="flex-1 overflow-auto" style="background: var(--bg-base)">
          <table class="w-full text-xs">
            <thead>
              <tr style="background: var(--bg-surface); border-bottom: 1px solid var(--border-subtle)">
                <th class="text-left px-3 py-2 font-medium cursor-pointer select-none" style="color: var(--text-muted)" @click="sortBy('name')">
                  名称 {{ sortKey === 'name' ? (sortDir === 'asc' ? '^' : 'v') : '' }}
                </th>
                <th class="text-left px-3 py-2 font-medium cursor-pointer select-none" style="color: var(--text-muted)" @click="sortBy('version')">
                  版本 {{ sortKey === 'version' ? (sortDir === 'asc' ? '^' : 'v') : '' }}
                </th>
                <th class="text-left px-3 py-2 font-medium" style="color: var(--text-muted)">标签</th>
                <th class="text-left px-3 py-2 font-medium cursor-pointer select-none" style="color: var(--text-muted)" @click="sortBy('file_size_mb')">
                  大小 {{ sortKey === 'file_size_mb' ? (sortDir === 'asc' ? '^' : 'v') : '' }}
                </th>
                <th class="text-left px-3 py-2 font-medium cursor-pointer select-none" style="color: var(--text-muted)" @click="sortBy('author')">
                  作者 {{ sortKey === 'author' ? (sortDir === 'asc' ? '^' : 'v') : '' }}
                </th>
                <th class="text-left px-3 py-2 font-medium cursor-pointer select-none" style="color: var(--text-muted)" @click="sortBy('modified_at')">
                  修改时间 {{ sortKey === 'modified_at' ? (sortDir === 'asc' ? '^' : 'v') : '' }}
                </th>
                <th class="text-left px-3 py-2 font-medium" style="color: var(--text-muted)">状态</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="asset in filteredAssets"
                :key="asset.id"
                @click="selectAsset(asset)"
                :class="['cursor-pointer transition', selectedAsset?.id === asset.id ? '' : '']"
                :style="{
                  background: selectedAsset?.id === asset.id ? 'var(--bg-elevated)' : 'transparent',
                  borderBottom: '1px solid var(--border-subtle)',
                }"
              >
                <td class="px-3 py-2" style="color: var(--text-primary)">
                  <div class="font-medium">{{ asset.name }}</div>
                  <div class="text-[10px] mt-0.5" style="color: var(--text-muted)">{{ asset.description.substring(0, 40) }}{{ asset.description.length > 40 ? '...' : '' }}</div>
                </td>
                <td class="px-3 py-2 font-mono" style="color: var(--text-secondary)">v{{ asset.version }}</td>
                <td class="px-3 py-2">
                  <div class="flex flex-wrap gap-1">
                    <span
                      v-for="tag in asset.tags.slice(0, 3)"
                      :key="tag"
                      class="px-1.5 py-0.5 text-[9px] rounded"
                      style="background: var(--bg-elevated); color: var(--text-secondary); border: 1px solid var(--border-default)"
                    >
                      {{ tag }}
                    </span>
                    <span v-if="asset.tags.length > 3" class="text-[9px]" style="color: var(--text-muted)">+{{ asset.tags.length - 3 }}</span>
                  </div>
                </td>
                <td class="px-3 py-2 font-mono" style="color: var(--text-secondary)">{{ asset.file_size_mb.toFixed(1) }} MB</td>
                <td class="px-3 py-2" style="color: var(--text-secondary)">{{ asset.author }}</td>
                <td class="px-3 py-2 font-mono" style="color: var(--text-muted)">{{ formatDate(asset.modified_at) }}</td>
                <td class="px-3 py-2">
                  <span
                    :class="['px-2 py-0.5 text-[10px] rounded-full', statusClass(asset.status)]"
                  >
                    {{ statusLabel(asset.status) }}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
          <div v-if="filteredAssets.length === 0" class="flex items-center justify-center py-12" style="color: var(--text-muted)">
            <div class="text-center">
              <div class="text-3xl mb-2">&#128194;</div>
              <div class="text-sm">暂无匹配的仿真数据资产</div>
            </div>
          </div>
        </div>

        <!-- Bottom Panel: Detail + Lineage -->
        <div class="h-72 flex border-t" style="border-color: var(--border-subtle); background: var(--bg-surface)">
          <!-- Asset Detail Panel -->
          <div class="w-80 overflow-y-auto p-4 border-r" style="border-color: var(--border-subtle)">
            <template v-if="selectedAsset">
              <h4 class="text-xs font-medium mb-3" style="color: var(--text-secondary)">资产详情</h4>
              <div class="space-y-2">
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">ID</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-secondary)">{{ selectedAsset.id }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">名称</span>
                  <span class="text-[10px] font-medium" style="color: var(--text-primary)">{{ selectedAsset.name }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">版本</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-secondary)">v{{ selectedAsset.version }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">文件类型</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-secondary)">{{ selectedAsset.file_type.toUpperCase() }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">大小</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-secondary)">{{ selectedAsset.file_size_mb.toFixed(2) }} MB</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">作者</span>
                  <span class="text-[10px]" style="color: var(--text-secondary)">{{ selectedAsset.author }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">创建时间</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-muted)">{{ formatDate(selectedAsset.created_at) }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">修改时间</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-muted)">{{ formatDate(selectedAsset.modified_at) }}</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">MD5</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-muted)">{{ selectedAsset.checksum_md5.substring(0, 12) }}...</span>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-[10px]" style="color: var(--text-muted)">父级资产</span>
                  <span class="text-[10px] font-mono" style="color: var(--text-secondary)">{{ selectedAsset.parent_id || '无' }}</span>
                </div>
                <div>
                  <span class="text-[10px] block mb-1" style="color: var(--text-muted)">描述</span>
                  <span class="text-[10px] block" style="color: var(--text-secondary)">{{ selectedAsset.description }}</span>
                </div>
              </div>

              <!-- Version History -->
              <div class="mt-4">
                <h4 class="text-xs font-medium mb-2" style="color: var(--text-secondary)">版本历史</h4>
                <div class="space-y-1">
                  <div
                    v-for="ver in versionHistory"
                    :key="ver.id"
                    class="flex items-center justify-between px-2 py-1 rounded text-[10px]"
                    style="background: var(--bg-elevated)"
                  >
                    <span class="font-mono" style="color: var(--text-primary)">v{{ ver.version }}</span>
                    <span style="color: var(--text-muted)">{{ formatDate(ver.modified_at) }}</span>
                  </div>
                </div>
              </div>

              <!-- Permissions -->
              <div class="mt-4">
                <h4 class="text-xs font-medium mb-2" style="color: var(--text-secondary)">权限列表</h4>
                <div class="space-y-1">
                  <div
                    v-for="perm in assetPermissions"
                    :key="perm.user_id + perm.permission"
                    class="flex items-center justify-between px-2 py-1 rounded text-[10px]"
                    style="background: var(--bg-elevated)"
                  >
                    <span style="color: var(--text-secondary)">{{ perm.user_id }}</span>
                    <span
                      :class="['px-1.5 py-0.5 rounded text-[9px]', perm.permission === 'admin' ? 'text-red-400' : perm.permission === 'write' ? 'text-yellow-400' : 'text-green-400']"
                    >
                      {{ perm.permission }}
                    </span>
                  </div>
                </div>
              </div>
            </template>
            <div v-else class="flex items-center justify-center h-full" style="color: var(--text-muted)">
              <div class="text-center">
                <div class="text-2xl mb-1">&#128196;</div>
                <div class="text-xs">选择资产查看详情</div>
              </div>
            </div>
          </div>

          <!-- Data Lineage Graph -->
          <div class="flex-1 overflow-hidden p-4">
            <h4 class="text-xs font-medium mb-2" style="color: var(--text-secondary)">数据血缘图</h4>
            <div class="relative w-full h-full">
              <svg ref="lineageSvg" class="w-full h-full" style="background: var(--bg-base); border-radius: var(--radius-md)">
                <!-- Lineage graph drawn here -->
                <defs>
                  <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                    <polygon points="0 0, 8 3, 0 6" fill="rgba(255,255,255,0.3)" />
                  </marker>
                  <marker id="arrowhead-active" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
                    <polygon points="0 0, 8 3, 0 6" fill="#4F8CFF" />
                  </marker>
                </defs>

                <!-- Upstream connections -->
                <line
                  v-for="conn in lineageConnections"
                  :key="'conn-' + conn.from + '-' + conn.to"
                  :x1="conn.x1" :y1="conn.y1" :x2="conn.x2" :y2="conn.y2"
                  :stroke="conn.active ? '#4F8CFF' : 'rgba(255,255,255,0.12)'"
                  :stroke-width="conn.active ? 2 : 1"
                  :stroke-dasharray="conn.type === 'export' ? '6,3' : 'none'"
                  :marker-end="conn.active ? 'url(#arrowhead-active)' : 'url(#arrowhead)'"
                />
                <text
                  v-for="conn in lineageConnections"
                  :key="'label-' + conn.from + '-' + conn.to"
                  :x="(conn.x1 + conn.x2) / 2"
                  :y="(conn.y1 + conn.y2) / 2 - 6"
                  fill="rgba(255,255,255,0.4)"
                  font-size="9"
                  text-anchor="middle"
                >
                  {{ conn.type }}
                </text>

                <!-- Asset nodes -->
                <g v-for="node in lineageNodes" :key="node.id">
                  <rect
                    :x="node.x" :y="node.y" :width="node.w" :height="node.h"
                    :rx="6" :ry="6"
                    :fill="node.active ? 'rgba(79,140,255,0.15)' : 'rgba(20,21,26,0.9)'"
                    :stroke="node.active ? '#4F8CFF' : 'rgba(255,255,255,0.15)'"
                    :stroke-width="node.active ? 2 : 1"
                    class="cursor-pointer"
                    @click="selectAssetById(node.id)"
                  />
                  <text
                    :x="node.x + node.w / 2"
                    :y="node.y + 18"
                    fill="white"
                    font-size="10"
                    text-anchor="middle"
                    font-weight="500"
                  >
                    {{ node.name }}
                  </text>
                  <text
                    :x="node.x + node.w / 2"
                    :y="node.y + 32"
                    fill="rgba(255,255,255,0.4)"
                    font-size="8"
                    text-anchor="middle"
                  >
                    {{ node.type }}
                  </text>
                </g>

                <!-- No lineage placeholder -->
                <text
                  v-if="lineageNodes.length === 0"
                  :x="lineageSvgWidth / 2"
                  :y="lineageSvgHeight / 2"
                  fill="rgba(255,255,255,0.3)"
                  font-size="12"
                  text-anchor="middle"
                >
                  选择资产以查看数据血缘关系
                </text>
              </svg>
            </div>
          </div>

          <!-- Lineage Report Preview -->
          <div class="w-72 overflow-y-auto p-4 border-l" style="border-color: var(--border-subtle)">
            <h4 class="text-xs font-medium mb-2" style="color: var(--text-secondary)">溯源报告预览</h4>
            <div v-if="selectedAsset" class="space-y-2 text-[10px]" style="color: var(--text-secondary)">
              <div class="p-2 rounded" style="background: var(--bg-elevated)">
                <div class="font-medium mb-1" style="color: var(--text-primary)">数据溯源报告</div>
                <div>资产: {{ selectedAsset.name }}</div>
                <div>版本: v{{ selectedAsset.version }}</div>
                <div>生成时间: {{ new Date().toLocaleString('zh-CN') }}</div>
              </div>
              <div class="p-2 rounded" style="background: var(--bg-elevated)">
                <div class="font-medium mb-1" style="color: var(--text-primary)">血缘链路</div>
                <div v-if="lineageNodes.length > 0">
                  <div v-for="node in lineageNodes" :key="'report-' + node.id" class="flex items-center gap-1 py-0.5">
                    <span :style="{ color: node.active ? '#4F8CFF' : 'var(--text-muted)' }">&#9679;</span>
                    <span>{{ node.name }} ({{ node.type }})</span>
                  </div>
                </div>
                <div v-else style="color: var(--text-muted)">暂无血缘数据</div>
              </div>
              <div class="p-2 rounded" style="background: var(--bg-elevated)">
                <div class="font-medium mb-1" style="color: var(--text-primary)">完整性校验</div>
                <div class="flex items-center gap-1">
                  <span style="color: var(--accent-green)">&#10003;</span>
                  <span>MD5: {{ selectedAsset.checksum_md5.substring(0, 16) }}...</span>
                </div>
                <div class="flex items-center gap-1 mt-0.5">
                  <span style="color: var(--accent-green)">&#10003;</span>
                  <span>版本链完整</span>
                </div>
                <div class="flex items-center gap-1 mt-0.5">
                  <span style="color: var(--accent-green)">&#10003;</span>
                  <span>权限验证通过</span>
                </div>
              </div>
            </div>
            <div v-else class="flex items-center justify-center h-full" style="color: var(--text-muted)">
              <div class="text-center text-xs">选择资产预览溯源报告</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Asset Modal -->
    <div v-if="showCreateModal" class="fixed inset-0 flex items-center justify-center z-50" style="background: rgba(0,0,0,0.6)">
      <div class="w-96 p-6 rounded-lg shadow-xl" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
        <h3 class="text-sm font-semibold mb-4" style="color: var(--text-primary)">新建仿真数据资产</h3>
        <div class="space-y-3">
          <div>
            <label class="label">资产名称</label>
            <input v-model="newAsset.name" type="text" class="input w-full text-xs" placeholder="输入资产名称" />
          </div>
          <div>
            <label class="label">描述</label>
            <textarea v-model="newAsset.description" class="input w-full text-xs" rows="2" placeholder="输入描述"></textarea>
          </div>
          <div>
            <label class="label">标签 (逗号分隔)</label>
            <input v-model="newAsset.tagsStr" type="text" class="input w-full text-xs" placeholder="mesh, thermal, v1" />
          </div>
          <div>
            <label class="label">文件类型</label>
            <select v-model="newAsset.file_type" class="input w-full text-xs">
              <option value="inp">INP (Abaqus)</option>
              <option value="odb">ODB (结果)</option>
              <option value="csv">CSV (数据)</option>
              <option value="vtk">VTK (可视化)</option>
              <option value="stl">STL (网格)</option>
              <option value="json">JSON (配置)</option>
            </select>
          </div>
          <div>
            <label class="label">父级资产 ID</label>
            <input v-model="newAsset.parent_id" type="text" class="input w-full text-xs" placeholder="可选" />
          </div>
          <div class="flex items-center justify-between">
            <label class="label mb-0">自动版本管理</label>
            <button
              @click="newAsset.auto_version = !newAsset.auto_version"
              :class="['relative w-10 h-5 rounded-full transition-colors',
                newAsset.auto_version ? 'bg-blue-600' : 'bg-gray-600']"
            >
              <span
                :class="['absolute top-0.5 w-4 h-4 rounded-full bg-white transition-transform',
                  newAsset.auto_version ? 'left-5' : 'left-0.5']"
              ></span>
            </button>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-5">
          <button @click="showCreateModal = false" class="btn btn-ghost text-xs">取消</button>
          <button @click="createNewAsset" class="btn btn-primary text-xs">创建</button>
        </div>
      </div>
    </div>

    <!-- Permission Modal -->
    <div v-if="showPermissionModal" class="fixed inset-0 flex items-center justify-center z-50" style="background: rgba(0,0,0,0.6)">
      <div class="w-80 p-6 rounded-lg shadow-xl" style="background: var(--bg-surface); border: 1px solid var(--border-subtle)">
        <h3 class="text-sm font-semibold mb-4" style="color: var(--text-primary)">设置权限 - {{ selectedAsset?.name }}</h3>
        <div class="space-y-3">
          <div>
            <label class="label">用户 ID</label>
            <input v-model="permForm.userId" type="text" class="input w-full text-xs" placeholder="输入用户 ID" />
          </div>
          <div>
            <label class="label">权限级别</label>
            <select v-model="permForm.permission" class="input w-full text-xs">
              <option value="read">读取</option>
              <option value="write">读写</option>
              <option value="admin">管理</option>
            </select>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-5">
          <button @click="showPermissionModal = false" class="btn btn-ghost text-xs">取消</button>
          <button @click="applyPermission" class="btn btn-primary text-xs">确认</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import type {
  SimulationAsset,
  DataLineage,
  AssetPermission,
  SearchFilter,
  AssetStatus,
} from '../api/dataAsset'

// ============ State ============

const assets = ref<SimulationAsset[]>([])
const selectedAsset = ref<SimulationAsset | null>(null)
const versionHistory = ref<SimulationAsset[]>([])
const assetPermissions = ref<AssetPermission[]>([])
const lineageData = ref<DataLineage | null>(null)
const fullLineage = ref<DataLineage[]>([])

const showCreateModal = ref(false)
const showPermissionModal = ref(false)
const tagInput = ref('')
const sortKey = ref('modified_at')
const sortDir = ref<'asc' | 'desc'>('desc')

const searchFilter = reactive<SearchFilter>({
  keyword: '',
  tags: [],
  author: '',
  date_range: { from: '', to: '' },
  file_type: '',
  project_id: '',
})

const newAsset = reactive({
  name: '',
  description: '',
  tagsStr: '',
  file_type: 'inp',
  parent_id: '',
  auto_version: true,
})

const permForm = reactive({
  userId: '',
  permission: 'read' as 'read' | 'write' | 'admin',
})

const lineageSvgWidth = 600
const lineageSvgHeight = 220

// ============ Mock Data ============

const mockAssets: SimulationAsset[] = [
  {
    id: 'asset-001', name: '叶片几何模型', project_id: 'proj-001', description: '航空发动机涡轮叶片 CAD 几何模型，包含冷却孔结构',
    version: '3.2', tags: ['几何', '叶片', 'CAD'], created_at: '2025-11-15T08:30:00Z', modified_at: '2026-03-20T14:22:00Z',
    file_size_mb: 45.3, file_type: 'stp', author: '张工', status: 'active', checksum_md5: 'a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6', parent_id: null,
  },
  {
    id: 'asset-002', name: '叶片结构化网格', project_id: 'proj-001', description: '六面体主导结构化网格，含边界层，节点数 2.1M',
    version: '2.1', tags: ['网格', '叶片', '六面体'], created_at: '2025-12-01T10:00:00Z', modified_at: '2026-03-18T09:15:00Z',
    file_size_mb: 128.7, file_type: 'inp', author: '李工', status: 'active', checksum_md5: 'b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7', parent_id: 'asset-001',
  },
  {
    id: 'asset-003', name: '叶片稳态热分析', project_id: 'proj-001', description: '稳态热传导分析结果，最高温度 950K',
    version: '1.5', tags: ['热分析', '叶片', '稳态'], created_at: '2026-01-10T11:30:00Z', modified_at: '2026-03-15T16:45:00Z',
    file_size_mb: 256.4, file_type: 'odb', author: '王工', status: 'active', checksum_md5: 'c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8', parent_id: 'asset-002',
  },
  {
    id: 'asset-004', name: '叶片瞬态热分析', project_id: 'proj-001', description: '瞬态热分析结果，模拟 300 个循环',
    version: '1.0', tags: ['热分析', '瞬态', '叶片'], created_at: '2026-02-20T09:00:00Z', modified_at: '2026-03-22T11:30:00Z',
    file_size_mb: 512.8, file_type: 'odb', author: '王工', status: 'active', checksum_md5: 'd4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9', parent_id: 'asset-003',
  },
  {
    id: 'asset-005', name: '碰撞模型网格', project_id: 'proj-002', description: '汽车正面碰撞有限元网格，含假人模型',
    version: '4.0', tags: ['碰撞', '网格', '汽车'], created_at: '2025-10-05T14:00:00Z', modified_at: '2026-03-10T08:00:00Z',
    file_size_mb: 89.2, file_type: 'inp', author: '赵工', status: 'locked', checksum_md5: 'e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0', parent_id: null,
  },
  {
    id: 'asset-006', name: '碰撞仿真结果', project_id: 'proj-002', description: '正面碰撞仿真结果，120ms 仿真时长',
    version: '2.3', tags: ['碰撞', '结果', '动力学'], created_at: '2025-12-15T16:30:00Z', modified_at: '2026-03-12T10:20:00Z',
    file_size_mb: 1024.5, file_type: 'odb', author: '赵工', status: 'active', checksum_md5: 'f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1', parent_id: 'asset-005',
  },
  {
    id: 'asset-007', name: '复合材料铺层设计', project_id: 'proj-003', description: '碳纤维/环氧树脂层合板铺层方案 [45/0/-45/90]2s',
    version: '1.2', tags: ['复合材料', '铺层', '设计'], created_at: '2026-01-25T09:00:00Z', modified_at: '2026-03-05T13:40:00Z',
    file_size_mb: 2.1, file_type: 'json', author: '陈工', status: 'active', checksum_md5: 'a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2', parent_id: null,
  },
  {
    id: 'asset-008', name: '层合板渐进损伤分析', project_id: 'proj-003', description: '渐进损伤分析结果，含分层扩展',
    version: '1.1', tags: ['复合材料', '损伤', '分层'], created_at: '2026-02-10T10:30:00Z', modified_at: '2026-03-08T15:10:00Z',
    file_size_mb: 384.6, file_type: 'odb', author: '陈工', status: 'archived', checksum_md5: 'b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3', parent_id: 'asset-007',
  },
  {
    id: 'asset-009', name: '热交换器参数化模型', project_id: 'proj-004', description: '板翅式热交换器参数化几何模型',
    version: '2.0', tags: ['热交换器', '参数化', '几何'], created_at: '2026-01-05T08:00:00Z', modified_at: '2026-03-01T12:00:00Z',
    file_size_mb: 15.8, file_type: 'stp', author: '刘工', status: 'active', checksum_md5: 'c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4', parent_id: null,
  },
  {
    id: 'asset-010', name: 'CFD 流场分析结果', project_id: 'proj-004', description: '热交换器内部流场 CFD 分析，雷诺数 15000',
    version: '1.3', tags: ['CFD', '流场', '热交换器'], created_at: '2026-02-01T14:00:00Z', modified_at: '2026-03-19T17:30:00Z',
    file_size_mb: 678.3, file_type: 'vtk', author: '刘工', status: 'active', checksum_md5: 'd0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5', parent_id: 'asset-009',
  },
]

const mockPermissions: AssetPermission[] = [
  { asset_id: 'asset-001', user_id: '张工', permission: 'admin', granted_at: '2025-11-15T08:30:00Z' },
  { asset_id: 'asset-001', user_id: '李工', permission: 'write', granted_at: '2025-12-01T10:00:00Z' },
  { asset_id: 'asset-001', user_id: '王工', permission: 'read', granted_at: '2026-01-10T11:30:00Z' },
  { asset_id: 'asset-005', user_id: '赵工', permission: 'admin', granted_at: '2025-10-05T14:00:00Z' },
  { asset_id: 'asset-005', user_id: '钱工', permission: 'read', granted_at: '2025-12-15T16:30:00Z' },
]

const mockLineage: Record<string, DataLineage> = {
  'asset-001': { asset_id: 'asset-001', upstream_ids: [], downstream_ids: ['asset-002'], transformation_type: 'mesh', created_at: '2025-12-01T10:00:00Z' },
  'asset-002': { asset_id: 'asset-002', upstream_ids: ['asset-001'], downstream_ids: ['asset-003'], transformation_type: 'mesh', created_at: '2025-12-01T10:00:00Z' },
  'asset-003': { asset_id: 'asset-003', upstream_ids: ['asset-002'], downstream_ids: ['asset-004'], transformation_type: 'solve', created_at: '2026-01-10T11:30:00Z' },
  'asset-004': { asset_id: 'asset-004', upstream_ids: ['asset-003'], downstream_ids: [], transformation_type: 'solve', created_at: '2026-02-20T09:00:00Z' },
  'asset-005': { asset_id: 'asset-005', upstream_ids: [], downstream_ids: ['asset-006'], transformation_type: 'mesh', created_at: '2025-12-15T16:30:00Z' },
  'asset-006': { asset_id: 'asset-006', upstream_ids: ['asset-005'], downstream_ids: [], transformation_type: 'solve', created_at: '2025-12-15T16:30:00Z' },
  'asset-007': { asset_id: 'asset-007', upstream_ids: [], downstream_ids: ['asset-008'], transformation_type: 'mesh', created_at: '2026-02-10T10:30:00Z' },
  'asset-008': { asset_id: 'asset-008', upstream_ids: ['asset-007'], downstream_ids: [], transformation_type: 'solve', created_at: '2026-02-10T10:30:00Z' },
  'asset-009': { asset_id: 'asset-009', upstream_ids: [], downstream_ids: ['asset-010'], transformation_type: 'mesh', created_at: '2026-02-01T14:00:00Z' },
  'asset-010': { asset_id: 'asset-010', upstream_ids: ['asset-009'], downstream_ids: [], transformation_type: 'solve', created_at: '2026-02-01T14:00:00Z' },
}

// ============ Computed ============

const allTags = computed(() => {
  const tagSet = new Set<string>()
  for (const asset of assets.value) {
    for (const tag of asset.tags) {
      tagSet.add(tag)
    }
  }
  return Array.from(tagSet).sort()
})

const activeTags = computed(() => {
  return new Set(searchFilter.tags ?? [])
})

const filteredAssets = computed(() => {
  let result = [...assets.value]

  if (searchFilter.keyword) {
    const kw = searchFilter.keyword.toLowerCase()
    result = result.filter(a =>
      a.name.toLowerCase().includes(kw) ||
      a.description.toLowerCase().includes(kw) ||
      a.author.toLowerCase().includes(kw)
    )
  }

  if ((searchFilter.tags ?? []).length > 0) {
    result = result.filter(a =>
      (searchFilter.tags ?? []).some(tag => a.tags.includes(tag))
    )
  }

  if (searchFilter.author) {
    const author = searchFilter.author.toLowerCase()
    result = result.filter(a => a.author.toLowerCase().includes(author))
  }

  if (searchFilter.file_type) {
    result = result.filter(a => a.file_type === searchFilter.file_type)
  }

  if (searchFilter.project_id) {
    result = result.filter(a => a.project_id === searchFilter.project_id)
  }

  if (searchFilter.date_range?.from) {
    const from = searchFilter.date_range.from
    result = result.filter(a => a.modified_at >= from)
  }

  if (searchFilter.date_range?.to) {
    const to = searchFilter.date_range.to
    result = result.filter(a => a.modified_at <= to + 'T23:59:59Z')
  }

  // Sort
  result.sort((a, b) => {
    const key = sortKey.value
    const dir = sortDir.value === 'asc' ? 1 : -1
    const va = (a as any)[key]
    const vb = (b as any)[key]
    if (typeof va === 'string' && typeof vb === 'string') {
      return va.localeCompare(vb) * dir
    }
    return ((va as number) - (vb as number)) * dir
  })

  return result
})

const totalSizeMB = computed(() => {
  return assets.value.reduce((sum, a) => sum + a.file_size_mb, 0)
})

const lineageNodes = computed(() => {
  if (!selectedAsset.value) return []

  const nodes: Array<{ id: string; name: string; type: string; x: number; y: number; w: number; h: number; active: boolean }> = []
  const nodeW = 120
  const nodeH = 42
  const gapX = 30
  const gapY = 16

  // Collect all related nodes
  const nodeIds = new Set<string>()
  nodeIds.add(selectedAsset.value.id)

  if (lineageData.value) {
    for (const uid of lineageData.value.upstream_ids) nodeIds.add(uid)
    for (const did of lineageData.value.downstream_ids) nodeIds.add(did)
  }

  // Recursively add upstream/downstream
  const queue = [...nodeIds]
  while (queue.length > 0) {
    const id = queue.pop()!
    const lineage = mockLineage[id]
    if (lineage) {
      for (const uid of lineage.upstream_ids) {
        if (!nodeIds.has(uid)) { nodeIds.add(uid); queue.push(uid) }
      }
      for (const did of lineage.downstream_ids) {
        if (!nodeIds.has(did)) { nodeIds.add(did); queue.push(did) }
      }
    }
  }

  // Build tree layout
  const idArr = Array.from(nodeIds)
  const assetMap = new Map(assets.value.map(a => [a.id, a]))

  // Calculate depth for each node
  const depths = new Map<string, number>()
  function getDepth(id: string): number {
    if (depths.has(id)) return depths.get(id)!
    const lineage = mockLineage[id]
    if (!lineage || lineage.upstream_ids.length === 0) {
      depths.set(id, 0)
      return 0
    }
    const maxUpstream = Math.max(...lineage.upstream_ids.map(getDepth))
    depths.set(id, maxUpstream + 1)
    return maxUpstream + 1
  }

  for (const id of idArr) {
    getDepth(id)
  }

  // Group by depth
  const depthGroups = new Map<number, string[]>()
  for (const id of idArr) {
    const d = depths.get(id) ?? 0
    if (!depthGroups.has(d)) depthGroups.set(d, [])
    depthGroups.get(d)!.push(id)
  }

  const maxDepth = Math.max(...Array.from(depthGroups.keys()), 0)
  const totalLevels = maxDepth + 1

  const totalWidth = totalLevels * nodeW + (totalLevels - 1) * gapX
  const startX = Math.max(10, (lineageSvgWidth - totalWidth) / 2)

  for (let d = 0; d <= maxDepth; d++) {
    const group = depthGroups.get(d) ?? []
    const totalHeight = group.length * nodeH + (group.length - 1) * gapY
    const startY = Math.max(10, (lineageSvgHeight - totalHeight) / 2)

    for (let i = 0; i < group.length; i++) {
      const id = group[i]
      const asset = assetMap.get(id)
      nodes.push({
        id,
        name: asset?.name ?? id,
        type: asset?.file_type.toUpperCase() ?? 'N/A',
        x: startX + d * (nodeW + gapX),
        y: startY + i * (nodeH + gapY),
        w: nodeW,
        h: nodeH,
        active: id === selectedAsset.value.id,
      })
    }
  }

  return nodes
})

const lineageConnections = computed(() => {
  if (!selectedAsset.value || lineageNodes.value.length === 0) return []

  const connections: Array<{
    from: string; to: string; type: string
    x1: number; y1: number; x2: number; y2: number
    active: boolean
  }> = []

  const nodeMap = new Map(lineageNodes.value.map(n => [n.id, n]))

  for (const id of nodeMap.keys()) {
    const lineage = mockLineage[id]
    if (lineage) {
      for (const upstreamId of lineage.upstream_ids) {
        const fromNode = nodeMap.get(upstreamId)
        const toNode = nodeMap.get(id)
        if (fromNode && toNode) {
          connections.push({
            from: upstreamId,
            to: id,
            type: lineage.transformation_type,
            x1: fromNode.x + fromNode.w,
            y1: fromNode.y + fromNode.h / 2,
            x2: toNode.x,
            y2: toNode.y + toNode.h / 2,
            active: upstreamId === selectedAsset.value.id || id === selectedAsset.value.id,
          })
        }
      }
    }
  }

  return connections
})

// ============ Methods ============

function formatDate(dateStr: string): string {
  const d = new Date(dateStr)
  return d.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

function statusLabel(status: AssetStatus): string {
  switch (status) {
    case 'active': return '活跃'
    case 'archived': return '已归档'
    case 'locked': return '已锁定'
  }
}

function statusClass(status: AssetStatus): string {
  switch (status) {
    case 'active': return 'text-green-400 bg-green-400/10'
    case 'archived': return 'text-gray-400 bg-gray-400/10'
    case 'locked': return 'text-red-400 bg-red-400/10'
  }
}

function sortBy(key: string) {
  if (sortKey.value === key) {
    sortDir.value = sortDir.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortKey.value = key
    sortDir.value = 'asc'
  }
}

function applyFilter() {
  // Computed property handles filtering reactively
}

function addFilterTag() {
  const tag = tagInput.value.trim()
  if (tag && !(searchFilter.tags ?? []).includes(tag)) {
    if (!searchFilter.tags) searchFilter.tags = []
    searchFilter.tags.push(tag)
    tagInput.value = ''
  }
}

function removeFilterTag(idx: number) {
  searchFilter.tags?.splice(idx, 1)
}

function toggleTagFilter(tag: string) {
  if (!searchFilter.tags) searchFilter.tags = []
  const idx = searchFilter.tags.indexOf(tag)
  if (idx >= 0) {
    searchFilter.tags.splice(idx, 1)
  } else {
    searchFilter.tags.push(tag)
  }
}

function selectAsset(asset: SimulationAsset) {
  selectedAsset.value = asset
  lineageData.value = mockLineage[asset.id] ?? null

  // Generate version history
  const baseVersion = parseFloat(asset.version)
  versionHistory.value = []
  for (let v = Math.max(1, Math.floor(baseVersion)); v <= Math.ceil(baseVersion); v += 0.1) {
    versionHistory.value.push({
      ...asset,
      version: v.toFixed(1),
      id: `${asset.id}-v${v.toFixed(1)}`,
      modified_at: new Date(
        new Date(asset.created_at).getTime() +
        (v - 1) * 30 * 24 * 3600 * 1000
      ).toISOString(),
    })
  }
  versionHistory.value.reverse()

  // Get permissions
  assetPermissions.value = mockPermissions.filter(p => p.asset_id === asset.id)
}

function selectAssetById(id: string) {
  const asset = assets.value.find(a => a.id === id)
  if (asset) selectAsset(asset)
}

function createNewAsset() {
  if (!newAsset.name.trim()) return

  const tags = newAsset.tagsStr.split(',').map(t => t.trim()).filter(Boolean)
  const now = new Date().toISOString()

  const asset: SimulationAsset = {
    id: `asset-${String(assets.value.length + 1).padStart(3, '0')}`,
    name: newAsset.name,
    project_id: searchFilter.project_id || 'proj-001',
    description: newAsset.description || '新建仿真数据资产',
    version: '1.0',
    tags,
    created_at: now,
    modified_at: now,
    file_size_mb: Math.random() * 50 + 1,
    file_type: newAsset.file_type,
    author: '当前用户',
    status: 'active',
    checksum_md5: Array.from({ length: 32 }, () => '0123456789abcdef'[Math.floor(Math.random() * 16)]).join(''),
    parent_id: newAsset.parent_id || null,
  }

  assets.value.unshift(asset)
  mockLineage[asset.id] = {
    asset_id: asset.id,
    upstream_ids: asset.parent_id ? [asset.parent_id] : [],
    downstream_ids: [],
    transformation_type: 'mesh',
    created_at: now,
  }

  showCreateModal.value = false
  newAsset.name = ''
  newAsset.description = ''
  newAsset.tagsStr = ''
  newAsset.parent_id = ''
  newAsset.auto_version = true

  selectAsset(asset)
}

function archiveAsset() {
  if (!selectedAsset.value || selectedAsset.value.status === 'archived') return
  selectedAsset.value.status = 'archived'
  const idx = assets.value.findIndex(a => a.id === selectedAsset.value!.id)
  if (idx >= 0) assets.value[idx].status = 'archived'
}

function toggleLock() {
  if (!selectedAsset.value) return
  if (selectedAsset.value.status === 'locked') {
    selectedAsset.value.status = 'active'
    const idx = assets.value.findIndex(a => a.id === selectedAsset.value!.id)
    if (idx >= 0) assets.value[idx].status = 'active'
  } else if (selectedAsset.value.status === 'active') {
    selectedAsset.value.status = 'locked'
    const idx = assets.value.findIndex(a => a.id === selectedAsset.value!.id)
    if (idx >= 0) assets.value[idx].status = 'locked'
  }
}

function applyPermission() {
  if (!selectedAsset.value || !permForm.userId.trim()) return

  const perm: AssetPermission = {
    asset_id: selectedAsset.value.id,
    user_id: permForm.userId,
    permission: permForm.permission,
    granted_at: new Date().toISOString(),
  }

  // Remove existing permission for same user
  const idx = assetPermissions.value.findIndex(p => p.asset_id === perm.asset_id && p.user_id === perm.user_id)
  if (idx >= 0) assetPermissions.value.splice(idx, 1)

  assetPermissions.value.push(perm)
  mockPermissions.push(perm)

  showPermissionModal.value = false
  permForm.userId = ''
  permForm.permission = 'read'
}

function exportLineage() {
  if (!selectedAsset.value) return

  const report = generateLineageReport(selectedAsset.value)
  const blob = new Blob([report], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `lineage_report_${selectedAsset.value.name}.txt`
  a.click()
  URL.revokeObjectURL(url)
}

function generateLineageReport(asset: SimulationAsset): string {
  const lines: string[] = []
  lines.push('=' .repeat(60))
  lines.push('仿真数据溯源报告')
  lines.push('=' .repeat(60))
  lines.push('')
  lines.push(`资产名称: ${asset.name}`)
  lines.push(`资产 ID: ${asset.id}`)
  lines.push(`版本: v${asset.version}`)
  lines.push(`状态: ${statusLabel(asset.status)}`)
  lines.push(`作者: ${asset.author}`)
  lines.push(`创建时间: ${asset.created_at}`)
  lines.push(`修改时间: ${asset.modified_at}`)
  lines.push(`文件大小: ${asset.file_size_mb.toFixed(2)} MB`)
  lines.push(`文件类型: ${asset.file_type}`)
  lines.push(`MD5: ${asset.checksum_md5}`)
  lines.push(`标签: ${asset.tags.join(', ')}`)
  lines.push(`父级资产: ${asset.parent_id || '无'}`)
  lines.push('')
  lines.push('-'.repeat(60))
  lines.push('数据血缘链路')
  lines.push('-'.repeat(60))

  const lineage = mockLineage[asset.id]
  if (lineage) {
    lines.push(`上游资产: ${lineage.upstream_ids.length > 0 ? lineage.upstream_ids.join(', ') : '无'}`)
    lines.push(`下游资产: ${lineage.downstream_ids.length > 0 ? lineage.downstream_ids.join(', ') : '无'}`)
    lines.push(`转换类型: ${lineage.transformation_type}`)
  } else {
    lines.push('暂无血缘数据')
  }

  lines.push('')
  lines.push('-'.repeat(60))
  lines.push('版本历史')
  lines.push('-'.repeat(60))
  for (const ver of versionHistory.value) {
    lines.push(`v${ver.version} - ${formatDate(ver.modified_at)}`)
  }

  lines.push('')
  lines.push('-'.repeat(60))
  lines.push('权限列表')
  lines.push('-'.repeat(60))
  for (const perm of assetPermissions.value) {
    lines.push(`用户: ${perm.user_id}, 权限: ${perm.permission}, 授权时间: ${formatDate(perm.granted_at)}`)
  }

  lines.push('')
  lines.push(`报告生成时间: ${new Date().toLocaleString('zh-CN')}`)
  lines.push('=' .repeat(60))

  return lines.join('\n')
}

// ============ Lifecycle ============

onMounted(() => {
  assets.value = mockAssets
})
</script>

<style scoped>
.panel-section {
  margin-bottom: 4px;
  padding: 12px;
  background: var(--bg-elevated);
  border-radius: var(--radius-md);
}

.label {
  display: block;
  font-size: 10px;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
}

.input {
  width: 100%;
  padding: 8px;
  background: var(--bg-base);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 12px;
  transition: border-color var(--duration-fast) var(--ease-out);
}

.input:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px var(--primary-glow);
}

textarea.input {
  resize: vertical;
  min-height: 40px;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-weight: 500;
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--primary);
  color: #fff;
}

.btn-primary:hover {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
}

.btn-ghost:hover {
  background: var(--bg-elevated);
}

.btn-ghost:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
