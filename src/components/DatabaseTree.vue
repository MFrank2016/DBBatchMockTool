<template>
  <div class="database-tree">
    <el-tree
      ref="treeRef"
      :data="treeData"
      :props="defaultProps"
      @node-click="handleNodeClick"
      @node-expand="handleNodeExpand"
      :load="loadNode"
      lazy
      @node-contextmenu="handleContextMenu"
    >
      <template #default="{ node }">
        <span class="custom-tree-node">
          <span>
            <el-icon class="mr-1">
              <component :is="getNodeIcon(node)" />
            </el-icon>
            {{ node.label }}
          </span>
        </span>
      </template>
    </el-tree>

    <el-dialog
      v-model="showTableColumns"
      :title="`表结构：${currentTable?.name || ''}`"
      width="800px"
    >
      <el-table :data="tableColumns" style="width: 100%">
        <el-table-column prop="name" label="字段名" width="180" />
        <el-table-column prop="type" label="类型" width="120" />
        <el-table-column prop="length" label="长度" width="80">
          <template #default="{ row }">
            {{ row.length || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="nullable" label="可空" width="80">
          <template #default="{ row }">
            <el-tag :type="row.nullable ? 'info' : 'danger'" size="small">
              {{ row.nullable ? '是' : '否' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="isPrimary" label="主键" width="80">
          <template #default="{ row }">
            <el-tag v-if="row.isPrimary" type="success" size="small">是</el-tag>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="comment" label="备注">
          <template #default="{ row }">
            {{ row.comment || '-' }}
          </template>
        </el-table-column>
      </el-table>
    </el-dialog>

    <div
      v-if="showContextMenu"
      class="context-menu"
      :style="contextMenuStyle"
    >
      <el-menu>
        <template v-if="shouldShowEditDelete">
          <el-menu-item @click="handleEdit">
            <el-icon><Edit /></el-icon>
            <span>编辑</span>
          </el-menu-item>
          <el-menu-item @click="handleDelete" class="text-red">
            <el-icon><Delete /></el-icon>
            <span>删除</span>
          </el-menu-item>
          <el-divider />
        </template>
        <el-menu-item @click="handleRefresh">
          <el-icon><Refresh /></el-icon>
          <span>刷新</span>
        </el-menu-item>
      </el-menu>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import type Node from 'element-plus/es/components/tree/src/model/node'
import { Connection, DataBase, DatabaseConfig, Table, ColumnInfo } from '../types/database'
import { Edit, Delete, Refresh } from '@element-plus/icons-vue'
import type { CSSProperties } from 'vue'

const emit = defineEmits(['edit-database'])

const treeRef = ref()
const treeData = ref<Connection[]>([])

// 树的配置
const defaultProps = {
  children: 'children',
  label: 'name',
  isLeaf: 'isLeaf'
}

// 获取节点图标
const getNodeIcon = (node: Node) => {
  switch (node.level) {
    case 1: return 'Connection'
    case 2: return 'DataBase' 
    case 3: return 'Grid'
    default: return ''
  }
}

// 异步加载子节点
const loadNode = async (node: Node, resolve: (data: any[]) => void) => {
  if (node.level === 0) {
    try {
      const configs = await invoke<DatabaseConfig[]>('list_configs')
      resolve(configs.map(config => ({
        ...config,
        isLeaf: false
      })))
    } catch (err) {
      console.error('加载数据库配置失败:', err)
      ElMessage.error('加载数据库配置失败')
      resolve([])
    }
  } else if (node.level === 1) {
    try {
      const databases = await invoke<DataBase[]>('list_database_schemas', {
        args: {
          configId: node.data.id
        }
      })
      resolve(databases.map(db => ({
        ...db,
        isLeaf: false
      })))
    } catch (err) {
      console.error('加载数据库列表失败:', err)
      ElMessage.error('加载数据库列表失败')
      resolve([])
    }
  } else if (node.level === 2) {
    try {
      const tables = await invoke<Table[]>('list_database_tables', {
        args: {
          configId: node.parent.data.id,
          dbName: node.data.name
        }
      })
      resolve(tables.map(table => ({
        ...table,
        isLeaf: true
      })))
    } catch (err) {
      console.error('加载数据表列表失败:', err)
      ElMessage.error('加载数据表列表失败')
      resolve([])
    }
  }
}

// 节点点击事件
const handleNodeClick = async (data: any, node: Node) => {
  if (node.level === 3) { // 表级别
    try {
      currentTable.value = data
      const columns = await invoke<ColumnInfo[]>('get_table_columns', {
        args: {
          configId: node.parent.parent.data.id,
          dbName: node.parent.data.name,
          tableName: data.name
        }
      })
      console.log('表结构信息:', columns)
      tableColumns.value = columns
      showTableColumns.value = true
    } catch (err) {
      console.error('获取表结构失败:', err)
      ElMessage.error('获取表结构失败')
    }
  }
}

// 编辑数据库配置

// 删除数据库配置

// 将 loadDatabases 方法定义为常量
const loadDatabases = async () => {
  // 重置树并触发重新加载
  if (treeRef.value) {
    treeRef.value.store.setData([])
    await loadNode({ level: 0 } as Node, (data) => {
      treeData.value = data
    })
  }
}

// 处理删除操作
const handleDelete = async () => {
  console.log('点击删除按钮')
  if (!currentNode.value?.id) return
  
  try {
    await ElMessageBox.confirm(
      '确定要删除这个数据库连接吗？',
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const configId = currentNode.value.id
    console.log('删除数据库配置:', configId)
    await invoke('delete_database', { 
      args: {
        configId
      }
    })
    
    ElMessage.success('删除成功')
    // 刷新列表
    await loadDatabases()
  } catch (err) {
    if (err !== 'cancel') {
      console.error('删除失败:', err)
      ElMessage.error('删除失败')
    }
  } finally {
    showContextMenu.value = false
  }
}

// 暴露方法给父组件
defineExpose({
  loadDatabases
})

const showTableColumns = ref(false)
const currentTable = ref<Table | null>(null)
const tableColumns = ref<ColumnInfo[]>([])

const showContextMenu = ref(false)
const contextMenuStyle = ref<CSSProperties>({
  position: 'fixed' as const,
  top: '0px',
  left: '0px'
})
const currentNode = ref<Node | null>(null)

// 添加计算属性来判断是否显示编辑和删除选项
const shouldShowEditDelete = computed(() => {
  console.log('当前节点数据:', currentNode.value)
  const hasId = Boolean(currentNode.value?.id)
  console.log('节点是否有 ID:', hasId)
  return hasId
})

// 处理右键点击
const handleContextMenu = (event: MouseEvent, node: any) => {
  console.log('右键点击事件触发:', event)
  console.log('节点数据:', node)
  event.preventDefault()
  
  // 直接存储节点数据
  currentNode.value = node  // 直接存储整个节点
  console.log('存储后的节点数据:', currentNode.value)
  
  showContextMenu.value = true
  contextMenuStyle.value = {
    position: 'fixed' as const,
    top: `${event.clientY}px`,
    left: `${event.clientX}px`
  }
}

// 处理刷新操作
const handleRefresh = async () => {
  console.log('点击刷新按钮')
  if (!currentNode.value) return
  
  try {
    // 清空子节点
    currentNode.value.childNodes = []
    // 重新加载
    await loadNode(currentNode.value, (data) => {
      currentNode.value?.setData(data)
    })
    ElMessage.success('刷新成功')
  } catch (err) {
    console.error('刷新失败:', err)
    ElMessage.error('刷新失败')
  } finally {
    showContextMenu.value = false
  }
}

// 添加 node-expand 事件处理，阻止表节点展开
const handleNodeExpand = (_data: any, node: Node) => {
  if (node.level === 3) { // 表级别
    node.expanded = false
  }
}

// 处理编辑操作
const handleEdit = () => {
  console.log('点击编辑按钮')
  if (!currentNode.value?.id) return
  const configId = currentNode.value.id
  console.log('编辑数据库配置:', configId)
  emit('edit-database', configId)
  showContextMenu.value = false
}

// 添加全局点击事件处理
const handleClickOutside = () => {
  if (showContextMenu.value) {
    showContextMenu.value = false
  }
}

// 添加事件监听
onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.database-tree {
  padding: 20px;
  position: relative;
}

.custom-tree-node {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 14px;
  padding-right: 8px;
}

.mr-1 {
  margin-right: 4px;
}

.text-red {
  color: var(--el-color-danger);
}

:deep(.el-dropdown-menu__item.text-red:hover) {
  color: var(--el-color-danger);
  background-color: var(--el-color-danger-light-9);
}

.context-menu {
  position: fixed;
  z-index: 9999;
  background: white;
  border: 1px solid var(--el-border-color-light);
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  padding: 4px 0;  /* 添加内边距 */
}

.context-menu :deep(.el-menu) {
  border: none;
  min-width: 150px;  /* 增加最小宽度 */
  background: transparent;  /* 设置背景透明 */
}

.context-menu :deep(.el-menu-item) {
  height: 36px;
  line-height: 36px;
  padding: 0 16px;
}

.context-menu :deep(.el-divider) {
  margin: 4px 0;  /* 调整分割线间距 */
}

.text-red {
  color: var(--el-color-danger);
}

.text-red:hover {
  color: var(--el-color-danger) !important;
  background-color: var(--el-color-danger-light-9) !important;
}
</style> 