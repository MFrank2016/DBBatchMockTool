import type Node from 'element-plus/es/components/tree/src/model/node'

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

// 重命名为 Connection 以避免混淆
export type Connection = DatabaseConfig

export interface DataBase {
  name: string
  encoding?: string
  collation?: string
}

export interface Table {
  name: string
  type_: string
  engine?: string
  comment?: string
}

// 定义我们的树节点数据接口
export interface TreeNode extends Node {
  key: string
  label: string
  type: TreeNodeType
  configId?: number
  children?: TreeNode[]
  isLoading?: boolean
  connected?: boolean
}

export interface TestConnectionResult {
    success: boolean;
    message: string;
} 

export enum TreeNodeType {
  Connection = 'connection',
  Database = 'database',
  Table = 'table'
} 

export interface ColumnInfo {
  name: string;
  type: string;
  length?: number;
  nullable: boolean;
  isPrimary: boolean;
  comment?: string;
} 