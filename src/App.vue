<script setup lang="ts">
import DatabaseList from './components/DatabaseList.vue'
import AddDatabase from './components/AddDatabase.vue'
import { ref } from 'vue'

const databaseListRef = ref()

const handleDatabaseAdded = () => {
  // 刷新数据库列表
  databaseListRef.value?.loadDatabases()
}
</script>

<template>
  <n-config-provider>
    <n-message-provider>
      <n-layout class="layout">
        <n-layout has-sider>
          <n-layout-sider
            bordered
            collapse-mode="width"
            :collapsed-width="0"
            :width="300"
            show-trigger
            class="sider"
          >
            <DatabaseList ref="databaseListRef" />
          </n-layout-sider>
          <n-layout-content class="content">
            <AddDatabase @database-added="handleDatabaseAdded" />
          </n-layout-content>
        </n-layout>
      </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>
.layout {
  height: 100vh;
}

.sider {
  background: var(--n-color);
}

.content {
  padding: 24px;
}
</style>
