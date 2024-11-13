<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { DatabaseConfig } from '../types/database'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'

const message = useMessage()
const databases = ref<DatabaseConfig[]>([])
const selectedId = ref<number>()
const loading = ref(false)

const loadDatabases = async () => {
  try {
    loading.value = true
    const data = await invoke<DatabaseConfig[]>('list_databases')
    console.log('加载的数据库列表:', data)
    databases.value = data
  } catch (err) {
    console.error('加载数据库列表失败:', err)
    message.error('加载数据库列表失败')
  } finally {
    loading.value = false
  }
}

const handleSelect = (db: DatabaseConfig) => {
  selectedId.value = db.id
}

// 暴露方法供父组件调用
defineExpose({
  loadDatabases
})

onMounted(() => {
  loadDatabases()
})
</script>

<template>
  <div class="database-list">
    <n-layout-header class="header">
      <n-h3>数据库连接</n-h3>
      <n-badge :value="databases.length" />
    </n-layout-header>
    
    <n-layout-content class="content">
      <div v-if="loading" class="empty-list">
        <n-spin size="large" />
      </div>
      <div v-else-if="databases.length === 0" class="empty-list">
        <n-empty description="暂无数据库连接" />
      </div>
      <n-list v-else>
        <n-list-item
          v-for="db in databases"
          :key="db.id"
          class="list-item"
          :class="{ active: selectedId === db.id }"
          @click="handleSelect(db)"
        >
          <n-thing :title="db.name">
            <template #description>
              <n-space vertical size="small">
                <span>{{ db.host }}</span>
                <n-space>
                  <n-tag :type="db.type_ === 'SQLite3' ? 'success' : 'info'">
                    {{ db.type_ }}
                  </n-tag>
                  <n-tag type="warning" v-if="db.lastConnectTime">
                    最后连接: {{ db.lastConnectTime }}
                  </n-tag>
                </n-space>
              </n-space>
            </template>
          </n-thing>
        </n-list-item>
      </n-list>
    </n-layout-content>
  </div>
</template>

<style scoped>
.database-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 16px;
  border-bottom: 1px solid var(--n-border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.empty-list {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.list-item {
  cursor: pointer;
  padding: 12px;
  border-radius: 4px;
  transition: background-color 0.3s;
}

.list-item:hover {
  background-color: var(--n-hover-color);
}

.list-item.active {
  background-color: var(--n-hover-color);
}
</style> 