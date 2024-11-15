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

    <el-dropdown
      v-if="showContextMenu"
      :visible="showContextMenu"
      trigger="contextmenu"
      :teleported="false"
      @visible-change="handleContextMenuVisibleChange"
      :style="contextMenuStyle"
    >
      <span></span>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item @click="handleRefresh">刷新</el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import type Node from 'element-plus/es/components/tree/src/model/node'
import { Connection, DataBase, DatabaseConfig, Table, ColumnInfo } from '../types/database'

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

// 将 loadDatabaseConfigs 方法暴露给父组件
defineExpose({
  loadDatabases: async () => {
    // 重置树并触发重新加载
    if (treeRef.value) {
      treeRef.value.store.setData([])
      await loadNode({ level: 0 } as Node, (data) => {
        treeData.value = data
      })
    }
  }
})

const showTableColumns = ref(false)
const currentTable = ref<Table | null>(null)
const tableColumns = ref<ColumnInfo[]>([])

const showContextMenu = ref(false)
const contextMenuStyle = ref({
  position: 'fixed',
  top: '0px',
  left: '0px'
})
const currentNode = ref<Node | null>(null)

// 处理右键点击
const handleContextMenu = (event: MouseEvent, node: Node) => {
  event.preventDefault()
  currentNode.value = node
  showContextMenu.value = true
  contextMenuStyle.value = {
    position: 'fixed',
    top: event.clientY + 'px',
    left: event.clientX + 'px'
  }
}

// 处理右键菜单显示状态变化
const handleContextMenuVisibleChange = (visible: boolean) => {
  if (!visible) {
    showContextMenu.value = false
  }
}

// 处理刷新操作
const handleRefresh = async () => {
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
</style> 