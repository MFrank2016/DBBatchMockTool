export enum DataGenerationRule {
  Name = 'name',           // 姓名
  Age = 'age',            // 年龄
  Phone = 'phone',        // 电话
  Address = 'address',    // 地址
  BankCard = 'bankCard',  // 银行卡号
  Random = 'random',      // 随机值
  Fixed = 'fixed',        // 固定值
  None = 'none'           // 不生成
}

export interface FieldMapping {
  fieldName: string;      // 字段名
  fieldType: string;      // 字段类型
  nullable: boolean;      // 是否可空
  isPrimary: boolean;     // 是否主键
  rule: DataGenerationRule; // 生成规则
  customValue?: string;   // 自定义值（用于固定值规则）
} 