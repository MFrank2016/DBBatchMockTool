<script setup lang="ts">
import DatabaseList from './components/DatabaseList.vue'
import AddDatabase from './components/AddDatabase.vue'
import { ref } from 'vue'

const databaseListRef = ref()
const editId = ref<number>()

const handleDatabaseAdded = () => {
  databaseListRef.value?.loadDatabases()
}

const handleDatabaseUpdated = () => {
  databaseListRef.value?.loadDatabases()
}

const handleEditDatabase = (id: number) => {
  editId.value = id
}
</script>

<template>
  <el-container class="layout">
    <el-aside width="300px" class="sider">
      <DatabaseList 
        ref="databaseListRef"
        @edit-database="handleEditDatabase"
      />
    </el-aside>
    <el-main class="content">
      <AddDatabase 
        v-model:edit-id="editId"
        @database-added="handleDatabaseAdded"
        @database-updated="handleDatabaseUpdated"
      />
    </el-main>
  </el-container>
</template>

<style scoped>
.layout {
  height: 100vh;
}

.sider {
  background: var(--el-color-primary-light-9);
}

.content {
  padding: 24px;
}
</style>
