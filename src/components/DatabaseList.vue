<template>
  <div class="database-list" @click="showContextMenu = false">
    <el-header class="header">
      <h3>数据库列表</h3>
    </el-header>
    
    <el-main class="content">
      <el-empty v-if="!loading && treeData.length === 0" description="暂无数据库" class="empty-list" />
      <el-tree
        v-else
        :data="treeData"
        node-key="key"
        :props="defaultProps"
        highlight-current
        @node-expand="handleExpand"
        @node-click="handleNodeClick"
        @node-contextmenu="handleContextMenu"
        v-loading="loading"
      >
        <template #default="{ data }">
          <div class="tree-node">
            <span class="icon">
              <i :class="getNodeIcon(data)" />
            </span>
            <span class="label">{{ data.label }}</span>
            <span v-if="data.type === TreeNodeType.Connection" class="connection-status">
              <i :class="data.connected ? 'i-mdi-check-circle' : 'i-mdi-alert-circle'" />
            </span>
          </div>
        </template>
      </el-tree>
    </el-main>

    <!-- 右键菜单 -->
    <div v-show="showContextMenu" 
         class="context-menu"
         :style="contextMenuStyle">
      <div class="menu-item" @click="handleEdit">
        <i class="i-mdi-pencil" />
        <span>编辑</span>
      </div>
      <div class="menu-item delete" @click="handleDelete">
        <i class="i-mdi-delete" />
        <span>删除</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ElTree, ElEmpty, ElHeader, ElMain, ElMessage, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { 
  type DatabaseConfig, 
  TreeNodeType, 
  type TreeNode
} from '../types/database'
import type { CSSProperties } from 'vue'

const message = ElMessage
const emit = defineEmits(['edit-database'])
const loading = ref(false)
const treeData = ref<TreeNode[]>([])

const defaultProps = {
  children: 'children',
  label: 'label'
}

// 右键菜单相关
const showContextMenu = ref(false)
const contextMenuPosition = ref({ x: 0, y: 0 })
const currentNode = ref<TreeNode | null>(null)

const contextMenuStyle = computed<CSSProperties>(() => ({
  position: 'fixed' as const,
  left: `${contextMenuPosition.value.x}px`,
  top: `${contextMenuPosition.value.y}px`
}))

// 定 handleExpand 方法
const handleExpand = (data: any) => {
  console.log('节点展开:', data)
}

// 获取节点图标
const getNodeIcon = (node: TreeNode) => {
  switch (node.type) {
    case TreeNodeType.Connection:
      return 'i-mdi-database'
    case TreeNodeType.Database:
      return 'i-mdi-database-outline'
    case TreeNodeType.Table:
      return 'i-mdi-table'
    default:
      return 'i-mdi-help-circle-outline'
  }
}

// 处理右键点击
const handleContextMenu = (event: MouseEvent, node: TreeNode) => {
  console.log('右键点击节点:', node)
  
  if (node.type !== TreeNodeType.Connection) {
    console.log('非连接节点，忽略右键菜单')
    return
  }
  
  event.preventDefault()
  event.stopPropagation()
  
  // 保存当前节点信息
  currentNode.value = node
  console.log('设置当前节点:', currentNode.value)
  
  // 设置菜单位置
  contextMenuPosition.value = {
    x: event.clientX,
    y: event.clientY
  }
  
  showContextMenu.value = true
}

// 处理节点点击
const handleNodeClick = () => {
  showContextMenu.value = false
}

// 添加全局点击事件监听
onMounted(() => {
  document.addEventListener('click', (event) => {
    const contextMenu = document.querySelector('.context-menu')
    if (contextMenu && !contextMenu.contains(event.target as Node)) {
      showContextMenu.value = false
    }
  })
})

// 处理编辑
const handleEdit = () => {
  if (!currentNode.value?.configId) return
  
  // 从当前节点获取config_id
  const config_id = currentNode.value.configId
  emit('edit-database', config_id)
  showContextMenu.value = false
}

// 处理删除
const handleDelete = async () => {
  console.log('开始删除操作, 当前节点:', currentNode.value)
  
  if (!currentNode.value?.configId) {
    console.error('当前节点无效或缺少 configId:', currentNode.value)
    return
  }
  
  try {
    await ElMessageBox.confirm(
      '确定要删除该数据库配置吗？',
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    loading.value = true
    const configId = currentNode.value.configId
    
    console.log('准备删除数据库配置:', {
      configId,
      nodeInfo: currentNode.value
    })
    
    await invoke('delete_database', { 
      args: { 
        configId: currentNode.value.configId
      }
    })
    
    console.log('删除成功')
    message.success('删除成功')
    await loadDatabases()
  } catch (err) {
    if (err !== 'cancel') {
      console.error('删除失败:', err)
      message.error('删除失败')
    }
  } finally {
    loading.value = false
    showContextMenu.value = false
  }
}

// 加载数据库列表
const loadDatabases = async () => {
  try {
    loading.value = true
    console.log('开始加载数据库配置...')
    const configs = await invoke<DatabaseConfig[]>('list_configs')
    console.log('后端返回的数据库配置:', configs)
    
    const mappedData = configs.map(config => ({
      key: `connection-${config.id}`,
      label: config.name,
      type: TreeNodeType.Connection,
      configId: config.id,
      children: [],
      isLoading: false,
      connected: false
    }))
    
    console.log('转换后的树形数据:', mappedData)
    treeData.value = mappedData
  } catch (err) {
    console.error('加载数据库列表失败:', err)
    message.error('加载数据库列表失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  console.log('组件已挂载，开始加载数据库列表')
  loadDatabases()
})

defineExpose({
  loadDatabases
})
</script>

<style scoped>
.database-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 16px;
  border-bottom: 1px solid var(--el-border-color);
}

.content {
  flex: 1;
  padding: 16px;
  overflow: auto;
}

.empty-list {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.tree-node {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 2px 4px;
  border-radius: 4px;
  cursor: pointer;
  height: 24px;
  margin: 2px 0;
}

.tree-node:hover {
  background-color: var(--el-color-primary-light-9);
}

.icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  line-height: 1;
  text-align: left;
}

.connection-status {
  margin-left: auto;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.connected {
  color: var(--el-color-success);
}

.disconnected {
  color: var(--el-color-warning);
}

.context-menu {
  position: fixed;
  background: white;
  border: 1px solid var(--el-border-color-light);
  border-radius: 4px;
  padding: 4px 0;
  min-width: 120px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  z-index: 3000;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.menu-item:hover {
  background-color: var(--el-color-primary-light-9);
}

.menu-item.delete {
  color: var(--el-color-danger);
  border-top: 1px solid var(--el-border-color-lighter);
  margin-top: 4px;
  padding-top: 8px;
}

.menu-item i {
  font-size: 16px;
}
</style> 