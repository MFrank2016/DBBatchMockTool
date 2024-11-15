<template>
  <div class="add-database">    
    <el-dialog
      v-model="showModal"
      :title="isEdit ? '编辑数据库' : '添加数据库'"
      width="600px"
    >
      <el-form
        ref="formRef"
        :model="formModel"
        :rules="rules"
        label-position="left"
        label-width="100px"
      >
        <el-form-item label="数据库名称" prop="name">
          <el-input v-model="formModel.name" placeholder="请输入数据库名称" />
        </el-form-item>

        <el-form-item label="数据库类型" prop="type_">
          <el-select
            v-model="formModel.type_"
            placeholder="请选择数据库类型"
          >
            <el-option
              v-for="item in databaseTypes"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>

        <el-form-item label="主机地址" prop="host">
          <el-input v-model="formModel.host" placeholder="请输入主机地址" />
        </el-form-item>

        <el-form-item label="用户名" prop="username">
          <el-input v-model="formModel.username" placeholder="请输入用户名" />
        </el-form-item>

        <el-form-item label="密码" prop="password">
          <el-input
            v-model="formModel.password"
            type="password"
            show-password
            placeholder="请输入密码"
          />
        </el-form-item>
      </el-form>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="showModal = false">取消</el-button>
          <el-button
            type="primary"
            :loading="loading"
            @click="handleSubmit"
          >
            确定
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import { ElMessage } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { DatabaseType, type DatabaseConfig, type TestConnectionResult } from '../types/database'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  editId?: number
}>()

const emit = defineEmits(['database-added', 'database-updated', 'update:editId'])
const message = ElMessage
const showModal = ref(false)
const formRef = ref<FormInstance | null>(null)
const loading = ref(false)
const isEdit = ref(false)

const formModel = reactive<Omit<DatabaseConfig, 'createTime' | 'lastConnectTime'>>({
  id: undefined,
  name: '',
  type_: DatabaseType.SQLite3,
  host: '',
  username: '',
  password: ''
})

// 测试连接
const testConnection = async () => {
  try {
    loading.value = true
    
    if (isEdit.value) {
      // 编辑模式：使用已有配置ID测试
      if (!formModel.id) {
        message.error('数据库配置 ID 不存在')
        return false
      }
      
      const result = await invoke<TestConnectionResult>('test_connection', { 
        args: {
          configId: formModel.id
        }
      })
      
      if (result.success) {
        message.success('连接测试成功')
        return true
      } else {
        message.error(`连接测试失败: ${result.message}`)
        return false
      }
    } else {
      // 新增模式：直接使用当前配置测试
      const result = await invoke<TestConnectionResult>('test_connection_with_config', {
        config: {
          name: formModel.name,
          type_: formModel.type_,
          host: formModel.host,
          username: formModel.username,
          password: formModel.password
        }
      })
      
      if (result.success) {
        message.success('连接测试成功')
        return true
      } else {
        message.error(`连接测试失败: ${result.message}`)
        return false
      }
    }
  } catch (err) {
    console.error('连接测试失败:', err)
    message.error(`连接测试失败: ${err}`)
    return false
  } finally {
    loading.value = false
  }
}

// 监听编辑ID变化
watch(() => props.editId, async (newId) => {
  if (newId) {
    isEdit.value = true
    await loadConfig(newId)
    showModal.value = true
  }
})

// 加载配置
const loadConfig = async (id: number) => {
  try {
    loading.value = true
    console.log('开始加载数据库配置:', id)
    
    const config = await invoke<DatabaseConfig>('get_database', { 
      args: {
        configId: id
      }
    })
    
    console.log('加载到的配置:', config)
    Object.assign(formModel, config)
  } catch (err) {
    console.error('加载配置失败:', err)
    message.error('加载配置失败')
  } finally {
    loading.value = false
  }
}

// 处理提交
const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    // 1. 表单验证
    await formRef.value.validate()
    loading.value = true
    
    if (isEdit.value) {
      // 编模式：先测试连接，成功后更新配置
      const connectionSuccess = await testConnection()
      if (!connectionSuccess) {
        return
      }
      
      await invoke('update_database', { config: formModel })
      message.success('更新成功')
      emit('database-updated')
    } else {
      // 新增模式：先测试连接，成功后保存配置
      const result = await invoke<TestConnectionResult>('test_connection_with_config', {
        config: {
          name: formModel.name,
          type_: formModel.type_,
          host: formModel.host,
          username: formModel.username,
          password: formModel.password
        }
      })
      
      if (!result.success) {
        message.error(`连接测试失败: ${result.message}`)
        return
      }
      
      // 连接测试成功，保存配置
      const id = await invoke<number>('add_database', { config: formModel })
      formModel.id = id
      message.success('添加成功')
      emit('database-added')
    }
    
    showModal.value = false
    resetForm()
  } catch (err) {
    console.error('操作失败:', err)
    message.error('操作失败')
  } finally {
    loading.value = false
  }
}

// 重置表单
const resetForm = () => {
  formModel.id = undefined
  formModel.name = ''
  formModel.type_ = DatabaseType.SQLite3
  formModel.host = ''
  formModel.username = ''
  formModel.password = ''
  isEdit.value = false
  // 重置 editId
  emit('update:editId', undefined)
}

// 添加表单验证规则
const rules: FormRules = {
  name: [
    { required: true, message: '请输入数据库名称', trigger: 'blur' }
  ],
  type_: [
    { required: true, message: '请选择数据库类型', trigger: 'change' }
  ],
  host: [
    { required: true, message: '请输入主机地址', trigger: 'blur' }
  ]
}

// 数据类型选项
const databaseTypes = [
  { label: 'SQLite3', value: DatabaseType.SQLite3 },
  { label: 'MySQL', value: DatabaseType.MySQL },
  { label: 'Oracle', value: DatabaseType.Oracle },
  { label: 'DM', value: DatabaseType.DM }
]


// 添加显示对话框的方法
const showDialog = () => {
  console.log('显示添加数据库对话框')
  showModal.value = true
}

// 暴露方法给父组件
defineExpose({
  showDialog
})
</script>

<style scoped>
.add-database {
  padding: 20px;
  display: flex;
  justify-content: center;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style> 