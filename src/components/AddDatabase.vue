<script setup lang="ts">
import { ref, reactive } from 'vue'
import type { FormInst, FormRules } from 'naive-ui'
import { DatabaseType, type DatabaseConfig, type TestConnectionResult } from '../types/database'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'

const message = useMessage()
const showModal = ref(false)
const formRef = ref<FormInst | null>(null)
const loading = ref(false)

const formModel = reactive<Omit<DatabaseConfig, 'id' | 'createTime' | 'lastConnectTime'>>({
  name: '',
  type_: DatabaseType.SQLite3,
  host: '',
  username: '',
  password: ''
})

const rules: FormRules = {
  name: [
    { required: true, message: '请输入连接名称', trigger: 'blur' }
  ],
  host: [
    { required: true, message: '请输入数据库地址', trigger: 'blur' }
  ]
}

const handleTest = async () => {
  try {
    await formRef.value?.validate()
    loading.value = true
    
    console.log('开始测试连接:', formModel)
    
    const result = await invoke<TestConnectionResult>('test_connection', {
      config: {
        ...formModel,
        username: formModel.username || null,
        password: formModel.password || null
      }
    })
    
    console.log('连接测试结果:', result)
    
    if (result.success) {
      message.success(result.message)
    } else {
      message.error(result.message)
    }
  } catch (err: any) {
    console.error('连接测试错误:', err)
    if (err.message) {
      message.error(err.message)
    } else {
      message.error('连接测试失败')
    }
  } finally {
    loading.value = false
  }
}

const emit = defineEmits<{
  (e: 'database-added'): void
}>()

const handleSubmit = async () => {
  try {
    await formRef.value?.validate()
    loading.value = true
    
    // 保存到数据库
    await invoke('add_database', {
      config: {
        ...formModel,
        username: formModel.username || null,
        password: formModel.password || null
      }
    })
    
    showModal.value = false
    message.success('添加成功')
    emit('database-added')  // 触发刷新列表事件
    resetForm()
  } catch (err: any) {
    console.error('添加数据库失败:', err)
    if (err.message) {
      message.error(err.message)
    } else {
      message.error('添加数据库失败')
    }
  } finally {
    loading.value = false
  }
}

const resetForm = () => {
  formRef.value?.restoreValidation()
  Object.assign(formModel, {
    name: '',
    type_: DatabaseType.SQLite3,
    host: '',
    username: '',
    password: ''
  })
}
</script>

<template>
  <div class="add-database">
    <n-button type="primary" size="large" @click="showModal = true">
      添加数据库
    </n-button>

    <n-modal
      v-model:show="showModal"
      preset="dialog"
      title="添加数据库连接"
      :show-icon="false"
      @after-leave="resetForm"
    >
      <n-form
        ref="formRef"
        :model="formModel"
        :rules="rules"
        label-placement="left"
        label-width="100"
        require-mark-placement="right-hanging"
      >
        <n-form-item label="连接名称" path="name">
          <n-input v-model:value="formModel.name" placeholder="请输入连接名称" />
        </n-form-item>
        <n-form-item label="数据库类型" path="type_">
          <n-select
            v-model:value="formModel.type_"
            :options="[
              { label: 'SQLite3', value: DatabaseType.SQLite3 },
              { label: 'MySQL', value: DatabaseType.MySQL },
              { label: 'Oracle', value: DatabaseType.Oracle },
              { label: 'DM', value: DatabaseType.DM }
            ]"
          />
        </n-form-item>
        <n-form-item label="数据库地址" path="host">
          <n-input v-model:value="formModel.host" placeholder="请输入数据库地址" />
        </n-form-item>
        <n-form-item label="用户名" path="username">
          <n-input 
            v-model:value="formModel.username" 
            placeholder="请输入用户名（可选）" 
          />
        </n-form-item>
        <n-form-item label="密码" path="password">
          <n-input
            v-model:value="formModel.password"
            type="password"
            show-password-on="click"
            placeholder="请输入密码（可选）"
          />
        </n-form-item>
      </n-form>
      
      <template #action>
        <n-space>
          <n-button :disabled="loading" @click="showModal = false">
            取消
          </n-button>
          <n-button
            :loading="loading"
            type="info"
            @click="handleTest"
          >
            测试连接
          </n-button>
          <n-button
            :loading="loading"
            type="primary"
            @click="handleSubmit"
          >
            确定
          </n-button>
        </n-space>
      </template>
    </n-modal>
  </div>
</template>

<style scoped>
.add-database {
  padding: 20px;
  display: flex;
  justify-content: center;
}
</style> 