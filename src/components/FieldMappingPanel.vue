<template>
  <div class="field-mapping-panel">
    <div v-if="currentTable" class="panel-header">
      <h3>{{ currentTable.name }} - 字段映射</h3>
    </div>
    
    <el-table
      v-if="currentTable"
      :data="mappings"
      class="mapping-table"
    >
      <el-table-column prop="fieldName" label="字段名" width="180" />
      <el-table-column label="类型" width="120">
        <template #default="{ row }">
          {{ formatFieldType(row.fieldType, row.length) }}
        </template>
      </el-table-column>
      <el-table-column label="可空" width="80">
        <template #default="{ row }">
          <el-tag :type="row.nullable ? 'info' : 'danger'" size="small">
            {{ row.nullable ? '是' : '否' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="主键" width="80">
        <template #default="{ row }">
          <el-tag v-if="row.isPrimary" type="success" size="small">是</el-tag>
          <span v-else>-</span>
        </template>
      </el-table-column>
      <el-table-column label="生成规则" width="200">
        <template #default="{ row }">
          <el-select
            v-model="row.rule"
            placeholder="选择生成规则"
            @change="handleRuleChange(row)"
          >
            <el-option
              v-for="rule in availableRules"
              :key="rule.value"
              :label="rule.label"
              :value="rule.value"
            />
          </el-select>
        </template>
      </el-table-column>
      <el-table-column label="自定义值" min-width="200">
        <template #default="{ row }">
          <el-input
            v-if="row.rule === DataGenerationRule.Fixed"
            v-model="row.customValue"
            placeholder="请输入自定义值"
          />
          <span v-else>-</span>
        </template>
      </el-table-column>
    </el-table>

    <div v-else class="empty-state">
      <el-empty description="请选择一个数据表" />
    </div>

    <div v-if="currentTable" class="action-bar">
      <el-button type="primary" @click="handleSave">保存配置</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import type { Table } from '../types/database'
import type { FieldMapping } from '../types/rules'
import { DataGenerationRule } from '../types/rules'

const props = defineProps<{
  currentTable?: Table
  columns: any[]
}>()

const emit = defineEmits(['save-mappings'])
const mappings = ref<FieldMapping[]>([])

// 格式化字段类型
const formatFieldType = (type: string, length?: number) => {
  if (!type) return '-'
  
  const formattedType = type.toUpperCase()
  if (length) {
    return `${formattedType}(${length})`
  }
  return formattedType
}

// 可用的生成规则
const availableRules = [
  { label: '姓名', value: DataGenerationRule.Name },
  { label: '年龄', value: DataGenerationRule.Age },
  { label: '电话', value: DataGenerationRule.Phone },
  { label: '地址', value: DataGenerationRule.Address },
  { label: '银行卡号', value: DataGenerationRule.BankCard },
  { label: '随机值', value: DataGenerationRule.Random },
  { label: '固定值', value: DataGenerationRule.Fixed },
  { label: '不生成', value: DataGenerationRule.None }
]

// 根据字段类型推荐生成规则
const suggestRule = (_fieldType: string, fieldName: string): DataGenerationRule => {
  const namePattern = /name|username|nickname/i
  const agePattern = /age|year/i
  const phonePattern = /phone|mobile|tel/i
  const addressPattern = /address|location/i
  const cardPattern = /card|account/i
  
  if (namePattern.test(fieldName)) return DataGenerationRule.Name
  if (agePattern.test(fieldName)) return DataGenerationRule.Age
  if (phonePattern.test(fieldName)) return DataGenerationRule.Phone
  if (addressPattern.test(fieldName)) return DataGenerationRule.Address
  if (cardPattern.test(fieldName)) return DataGenerationRule.BankCard
  
  return DataGenerationRule.Random
}

// 监听列信息变化
watch(() => props.columns, (newColumns) => {
  if (!newColumns) return
  
  console.log('收到的列信息:', newColumns)
  
  mappings.value = newColumns.map(col => {
    console.log('处理列:', col)
    return {
      fieldName: col.name,
      fieldType: col.type,
      length: col.length,
      nullable: col.nullable,
      isPrimary: col.isPrimary,
      rule: suggestRule(col.type, col.name),
      customValue: ''
    }
  })
  
  console.log('映射结果:', mappings.value)
}, { immediate: true })

// 处理规则变化
const handleRuleChange = (row: FieldMapping) => {
  if (row.rule !== DataGenerationRule.Fixed) {
    row.customValue = ''
  }
}

// 处理保存
const handleSave = () => {
  try {
    // 验证必填项
    const hasInvalidMapping = mappings.value.some(mapping => 
      mapping.rule === DataGenerationRule.Fixed && !mapping.customValue
    )
    
    if (hasInvalidMapping) {
      ElMessage.warning('请为所有固定值规则的字段填写自定义值')
      return
    }
    
    emit('save-mappings', mappings.value)
    ElMessage.success('保存成功')
  } catch (err) {
    console.error('保存失败:', err)
    ElMessage.error('保存失败')
  }
}
</script>

<style scoped>
.field-mapping-panel {
  height: 100%;
  padding: 20px;
  background: white;
  border-radius: 4px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  flex: 1;
}

.panel-header {
  margin-bottom: 20px;
}

.mapping-table {
  flex: 1;
  overflow: auto;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-bar {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid var(--el-border-color-light);
  display: flex;
  justify-content: flex-end;
}
</style> 