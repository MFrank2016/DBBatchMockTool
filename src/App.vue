<template>
  <el-container class="layout">
    <el-header class="header">
      <MenuBar />
    </el-header>
    
    <el-container>
      <el-aside width="300px" class="sider">
        <DatabaseTree 
          ref="databaseTreeRef"
          @edit-database="handleEditDatabase"
        />
      </el-aside>
      <el-main class="content">
        <AddDatabase 
          ref="addDatabaseRef" 
          v-model:edit-id="editId"
          @database-added="handleDatabaseAdded"
          @database-updated="handleDatabaseUpdated"
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
import { useMenuStore } from './stores/menu'
import { MenuCommand, AddDatabaseCommand } from './types/menu'

const menuStore = useMenuStore()
const addDatabaseRef = ref()
const databaseTreeRef = ref()
const editId = ref<number | undefined>(undefined)

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

.sider {
  background: var(--el-color-primary-light-9);
}

.content {
  padding: 24px;
}
</style>
