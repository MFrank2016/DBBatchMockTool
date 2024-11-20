<template>
  <el-container class="layout">
    <el-header class="header">
      <MenuBar />
    </el-header>
    
    <el-container class="main-container">
      <el-aside width="300px" class="sider">
        <DatabaseTree 
          ref="databaseTreeRef"
          @edit-database="handleEditDatabase"
          v-model:current-table="currentTable"
          v-model:table-columns="tableColumns"
        />
      </el-aside>
      <el-main class="content">
        <AddDatabase 
          ref="addDatabaseRef" 
          v-model:edit-id="editId"
          @database-added="handleDatabaseAdded"
          @database-updated="handleDatabaseUpdated"
        />
        <FieldMappingPanel
          :current-table="currentTable"
          :columns="tableColumns"
          @save-mappings="handleSaveMappings"
        />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MenuBar from './components/MenuBar.vue'
import DatabaseTree from './components/DatabaseTree.vue'
import AddDatabase from './components/AddDatabase.vue'
import FieldMappingPanel from './components/FieldMappingPanel.vue'
import type { Table } from './types/database'
import { useMenuStore } from './stores/menu'
import { MenuCommand, AddDatabaseCommand } from './types/menu'
import { FieldMapping } from './types/rules'

const menuStore = useMenuStore()
const addDatabaseRef = ref()
const databaseTreeRef = ref()
const editId = ref<number | undefined>(undefined)
const currentTable = ref<Table | undefined>(undefined)
const tableColumns = ref<any[]>([])

const emit = defineEmits([
  'update:currentTable',
  'update:tableColumns'
])

// 注册菜单命令处理函数
onMounted(() => {
  console.log('注册菜单命令')
  menuStore.registerCommand(
    MenuCommand.ADD_DATABASE, 
    new AddDatabaseCommand(() => {
      console.log('执行添加数据库命令')
      addDatabaseRef.value?.showDialog()
    })
  )
})

const handleEditDatabase = (id: number) => {
  console.log('编辑数据库:', id)
  editId.value = id
}

// 添加处理函数
const handleDatabaseAdded = async () => {
  console.log('数据库添加成功，刷新列表')
  await databaseTreeRef.value?.loadDatabases()
}

// 处理数据库更新
const handleDatabaseUpdated = async () => {
  console.log('数据库更新成功，刷新列表')
  await databaseTreeRef.value?.loadDatabases()
}

// // 在 DatabaseTree 组件中触发表选择事件时更新当前表和列信息
// const handleTableSelect = (table: Table, columns: any[]) => {
//   currentTable.value = table
//   tableColumns.value = columns
// }

const handleSaveMappings = (mappings: FieldMapping[]) => {
  console.log('保存字段映射:', mappings)
  // TODO: 调用后端 API 保存映射配置
}
</script>

<style scoped>
.layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 0;
}

.main-container {
  flex: 1;
  overflow: hidden;
}

.sider {
  background: var(--el-color-primary-light-9);
  height: 100%;
  overflow: auto;
}

.content {
  padding: 24px;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

/* 确保 AddDatabase 组件不占用额外空间 */
:deep(.add-database) {
  flex: 0 0 auto;
}

/* 让 FieldMappingPanel 组件占据剩余空间 */
:deep(.field-mapping-panel) {
  flex: 1;
  overflow: auto;
}
</style>
