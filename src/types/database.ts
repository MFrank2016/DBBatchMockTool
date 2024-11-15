export enum DatabaseType {
  SQLite3 = 'SQLite3',
  MySQL = 'MySQL',
  Oracle = 'Oracle',
  DM = 'DM'  // 达梦数据库
}

export interface DatabaseConfig {
  id?: number
  name: string
  type_: DatabaseType
  host: string
  username?: string
  password?: string
  createTime?: string
  lastConnectTime?: string
}

export interface TestConnectionResult {
  success: boolean
  message: string
}

// 新增树节点类型定义
export enum TreeNodeType {
  Connection = 'connection',
  Database = 'database',
  Table = 'table'
}

export interface TreeNode {
  key: string           // 唯一标识
  label: string        // 显示名称
  type: TreeNodeType   // 节点类型
  configId?: number    // 数据库配置ID
  database?: string    // 数据库名称
  children?: TreeNode[] // 子节点
  isLoading?: boolean  // 是否正在加载
  connected?: boolean  // 新增：连接状态
}

// 新增：右键菜单项
export interface ContextMenuItem {
  label: string
  key: string
  icon?: string
  disabled?: boolean
  children?: ContextMenuItem[]
}

// 新增：右键菜单类型
export enum ContextMenuType {
  Connection = 'connection',
  Database = 'database',
  Table = 'table'
} 