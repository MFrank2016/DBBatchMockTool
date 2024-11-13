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