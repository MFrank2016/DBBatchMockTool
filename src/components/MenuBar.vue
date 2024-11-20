<template>
  <el-menu mode="horizontal" :ellipsis="false" class="menu-bar">
    <el-menu-item index="1">数据库管理工具</el-menu-item>
    
    <el-sub-menu index="2">
      <template #title>数据库管理</template>
      <el-menu-item 
        v-for="item in databaseMenuItems" 
        :key="item.id"
        :index="item.id"
        @click="item.handler"
      >
        {{ item.label }}
      </el-menu-item>
    </el-sub-menu>
  </el-menu>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { MenuCommand } from '../types/menu'
import { useMenuStore } from '../stores/menu'
import type { MenuItem } from '../types/menu'

const menuStore = useMenuStore()

// 使用 computed 获取菜单项
const databaseMenuItems = computed<MenuItem[]>(() => [
  {
    id: '2-1',
    label: '添加数据库',
    handler: () => menuStore.executeCommand(MenuCommand.ADD_DATABASE)
  }
])
</script>

<style scoped>
.menu-bar {
  border-bottom: solid 1px var(--el-border-color-light);
}
</style> 