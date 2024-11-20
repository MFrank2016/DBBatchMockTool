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
        <el-menu-item @click="handleEdit">
          <el-icon><Edit /></el-icon>
          <span>编辑</span>
        </el-menu-item>
        <el-menu-item @click="handleDelete" class="text-red">
          <el-icon><Delete /></el-icon>
          <span>删除</span>
        </el-menu-item>
      </el-menu>
    </div>

    <div
      v-if="showTableContextMenu"
      class="context-menu"
      :style="contextMenuStyle"
    >
      <el-menu>
        <el-menu-item @click="handleViewStructure">
          <el-icon><View /></el-icon>
          <span>查看表结构</span>
        </el-menu-item>
      </el-menu>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import type Node from 'element-plus/es/components/tree/src/model/node'
import { DataBase, DatabaseConfig, Table, ColumnInfo } from '../types/database'
import { Edit, Delete, View } from '@element-plus/icons-vue'
import type { CSSProperties } from 'vue'

const emit = defineEmits<{
  'edit-database': [id: number]
  'table-select': [params: { configId: number, dbName: string, tableName: string }]
  'update:currentTable': [table: Table]
  'update:tableColumns': [columns: ColumnInfo[]]
}>()

const treeRef = ref()
const treeData = ref<NodeDataType[]>([])

// 树的配置
const defaultProps = {
  children: 'children',
  label: 'name',
  isLeaf: 'isLeaf'
}

// 获取节点图标
const getNodeIcon = (node: any) => {
  const level = node.level || node.data?.level
  switch (level) {
    case 1: return 'Connection'
    case 2: return 'DataBase' 
    case 3: return 'Grid'
    default: return ''
  }
}

// 定义树节点数据结构
interface TreeNodeData {
  id?: number;
  name: string;
  level: number;
  type_?: string;
  configId?: number;
  dbName?: string;
}

// 保留一个 currentNode 声明
const currentNode = ref<TreeNodeData | null>(null)

// 定义基础节点类型
interface TreeNodeBase {
  level: number;
  isLeaf: boolean;
}

// 定义各种节点类型
interface ConnectionNodeData extends DatabaseConfig, TreeNodeBase {}
interface DatabaseNodeData extends DataBase, TreeNodeBase {}
interface TableNodeData extends Table, TreeNodeBase {}

// 使用联合类型定义所有可能的节点数据类型
type NodeDataType = ConnectionNodeData | DatabaseNodeData | TableNodeData;

// 定义 resolve 函数的类型
type ResolveFunction = (data: NodeDataType[]) => void;

// 修改 loadNode 函数
const loadNode = async (node: Node, resolve: ResolveFunction) => {
  if (node.level === 0) {
    try {
      const configs = await invoke<DatabaseConfig[]>('list_configs')
      const connectionNodes: ConnectionNodeData[] = configs.map(config => ({
        ...config,
        level: 1,
        isLeaf: false
      }))
      resolve(connectionNodes)
    } catch (err) {
      console.error('加载数据库配置失败:', err)
      ElMessage.error('加载数据库配置失败')
      resolve([])
    }
  } else if (node.level === 1) {
    try {
      const databases = await invoke<DataBase[]>('list_database_schemas', {
        args: {
          configId: (node.data as ConnectionNodeData).id
        }
      })
      const dbNodes: DatabaseNodeData[] = databases.map(db => ({
        ...db,
        level: 2,
        isLeaf: false
      }))
      resolve(dbNodes)
    } catch (err) {
      console.error('加载数据库列表失败:', err)
      ElMessage.error('加载数据库列表失败')
      resolve([])
    }
  } else if (node.level === 2) {
    try {
      const tables = await invoke<Table[]>('list_database_tables', {
        args: {
          configId: (node.parent.data as ConnectionNodeData).id,
          dbName: node.data.name
        }
      })
      const tableNodes: TableNodeData[] = tables.map(table => ({
        ...table,
        level: 3,
        isLeaf: true,
        type_: 'TABLE'  // 确保设置 type_ 属性
      }))
      resolve(tableNodes)
    } catch (err) {
      console.error('加载数据表列表失败:', err)
      ElMessage.error('加载数据表列表失败')
      resolve([])
    }
  }
}

// 修改 loadDatabases 函数
const loadDatabases = async () => {
  if (treeRef.value) {
    treeRef.value.store.setData([])
    await loadNode({ level: 0 } as Node, (data) => {
      treeData.value = data
    })
  }
}

// 暴露方法给父组件
defineExpose({
  loadDatabases
})

// 其他变量声明
const showTableColumns = ref(false)
const currentTable = ref<Table | null>(null)
const tableColumns = ref<ColumnInfo[]>([])
const showContextMenu = ref(false)
const showTableContextMenu = ref(false)
const contextMenuStyle = ref<CSSProperties>({
  position: 'fixed' as const,
  top: '0px',
  left: '0px'
})

// 处理右键点击
const handleContextMenu = (event: MouseEvent, node: any) => {
  console.log('右键点击事件触发:', event)
  console.log('节点数据:', node)
  console.log('节点级别:', node.level)
  
  event.preventDefault()
  
  try {
    if (node.level === 1) {
      // 处理连接节点
      currentNode.value = {
        name: node.name,
        level: node.level,
        type_: node.type_,
        configId: node.id
      }
      
      console.log('连接节点信息:', currentNode.value)
      
      showContextMenu.value = true
      showTableContextMenu.value = false
    } else if (node.level === 3) {
      // 处理表节点
      const nodeData = {
        tableName: node.name,
        configId: undefined as number | undefined,
        dbName: undefined as string | undefined
      }
      
      // 安全地获取父节点
      if (treeRef.value) {
        const currentNode = treeRef.value.getNode(node)
        console.log('当前节点:', currentNode)
        
        if (currentNode?.parent) {
          const dbNode = currentNode.parent
          console.log('数据库节点:', dbNode)
          nodeData.dbName = dbNode.data?.name
          
          if (dbNode.parent) {
            const connNode = dbNode.parent
            console.log('连接节点:', connNode)
            nodeData.configId = connNode.data?.id
          }
        }
      }
      
      console.log('表节点信息:', nodeData)
      
      if (!nodeData.tableName) {
        throw new Error('无法获取表名')
      }
      
      // 构建节点信息
      currentNode.value = {
        name: nodeData.tableName,
        level: node.level,
        type_: node.type_,
        configId: nodeData.configId,
        dbName: nodeData.dbName
      }
      
      showTableContextMenu.value = true
      showContextMenu.value = false
    }
    
    contextMenuStyle.value = {
      position: 'fixed' as const,
      top: `${event.clientY}px`,
      left: `${event.clientX}px`
    }
    
    console.log('当前节点信息:', currentNode.value)
  } catch (err) {
    console.error('处理右键菜单时出错:', err)
    ElMessage.error('无法显示菜单')
    // 重置菜单状态
    showContextMenu.value = false
    showTableContextMenu.value = false
  }
}

// 修改 handleViewStructure 函数
const handleViewStructure = async () => {
  if (!currentNode.value) {
    console.error('无法获取节点信息')
    ElMessage.error('无法获取表结构信息')
    return
  }

  try {
    if (!currentNode.value.configId || !currentNode.value.dbName || !currentNode.value.name) {
      console.error('缺少必要的节点信息:', currentNode.value)
      ElMessage.error('无法获取表结构信息')
      return
    }

    const params = {
      configId: currentNode.value.configId,
      dbName: currentNode.value.dbName,
      tableName: currentNode.value.name
    }

    console.log('获取表结构参数:', params)

    const columns = await invoke<ColumnInfo[]>('get_table_columns', {
      args: params
    })

    console.log('表结构信息:', columns)
    currentTable.value = {
      name: currentNode.value.name,
      type_: currentNode.value.type_ || 'TABLE',
      engine: undefined,
      comment: undefined
    }
    tableColumns.value = columns
    showTableColumns.value = true
  } catch (err) {
    console.error('获取表结构失败:', err)
    ElMessage.error('获取表结构失败')
  } finally {
    showTableContextMenu.value = false
  }
}

// 添加点击外部关闭菜单的处理
const handleClickOutside = () => {
  if (showContextMenu.value) {
    showContextMenu.value = false
  }
  if (showTableContextMenu.value) {
    showTableContextMenu.value = false
  }
}

// 添加 node-expand 事件处理，阻止表节点展开
const handleNodeExpand = (_data: any, node: Node) => {
  // 阻止数据表节点（level 3）展开
  if (node.level === 3) {
    node.expanded = false
  }
}

// 修改 handleNodeClick 函数
const handleNodeClick = async (data: any, node: Node) => {
  console.log('节点点击:', data, node)
  
  if (node.level === 3) { // 表节点
    try {
      // 获取必要的信息
      let configId: number | undefined
      let dbName: string | undefined
      
      if (node.parent) {
        const dbNode = node.parent
        dbName = dbNode.data?.name
        
        if (dbNode.parent) {
          const connNode = dbNode.parent
          configId = connNode.data?.id
        }
      }
      
      if (!configId || !dbName) {
        console.error('无法获取必要的节点信息')
        return
      }
      
      // 获取表结构
      const columns = await invoke<ColumnInfo[]>('get_table_columns', {
        args: {
          configId,
          dbName,
          tableName: data.name
        }
      })
      
      console.log('获取到的表结构:', columns)
      
      // 确保列信息包含所有必要的字段
      const processedColumns = columns.map(col => ({
        name: col.name,
        type: col.type,
        length: col.length,
        nullable: col.nullable,
        isPrimary: col.isPrimary,
        comment: col.comment
      }))
      
      // 更新当前表信息
      const tableInfo: Table = {
        name: data.name,
        type_: data.type_ || 'TABLE',
        engine: '',
        comment: ''
      }
      
      // 触发事件通知父组件更新状态
      emit('update:currentTable', tableInfo)
      emit('update:tableColumns', processedColumns)
      
    } catch (err) {
      console.error('获取表结构失败:', err)
      ElMessage.error('获取表结构失败')
    }
  }
}

// 添加事件监听
onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})

// 添加删除处理函数
const handleDelete = async () => {
  if (!currentNode.value || !isConnectionNode(currentNode.value)) {
    return
  }

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
    
    const configId = (currentNode.value as ConnectionNodeData).id
    await invoke('delete_database', { 
      args: {
        configId
      }
    })
    
    ElMessage.success('删除成功')
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

// 修改 handleEdit 函数
const handleEdit = () => {
  if (!currentNode.value?.configId) {
    console.error('无法获取配置ID:', currentNode.value)
    ElMessage.error('无法编辑数据库配置')
    return
  }
  
  emit('edit-database', currentNode.value.configId)
  showContextMenu.value = false
}

// 添加类型守卫函数
function isConnectionNode(node: TreeNodeData): boolean {
  return node.level === 1
}

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