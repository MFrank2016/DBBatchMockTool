use crate::error::AppError;
use crate::models::{DatabaseConfig, DatabaseType, ColumnInfo};
use rusqlite::Connection as SQLiteConnection;
use std::path::Path;
use log::{debug, error};

pub trait DatabaseConnection {
    fn test_connection(&self) -> Result<(), AppError>;
    fn list_databases(&self) -> Result<Vec<String>, AppError>;
    fn list_tables(&self, database: &str) -> Result<Vec<String>, AppError>;
    fn get_table_columns(&self, table_name: &str) -> Result<Vec<ColumnInfo>, AppError>;
}

pub struct SQLite3Connection {
    path: String,
}

impl SQLite3Connection {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl DatabaseConnection for SQLite3Connection {
    fn test_connection(&self) -> Result<(), AppError> {
        debug!("测试 SQLite3 连接: {}", self.path);
        
        if !Path::new(&self.path).exists() {
            let err_msg = format!("找不到数据库文件：{}", self.path);
            error!("错误: {}", err_msg);
            return Err(AppError::Connection(err_msg));
        }

        match SQLiteConnection::open(&self.path) {
            Ok(conn) => {
                match conn.execute_batch("SELECT 1") {
                    Ok(_) => {
                        debug!("SQLite3 连接测试成功");
                        Ok(())
                    }
                    Err(e) => {
                        let err_msg = format!("数据库查询测试失败：{}", e);
                        error!("错误: {}", err_msg);
                        Err(AppError::Connection(err_msg))
                    }
                }
            }
            Err(e) => {
                let err_msg = format!("无法打开数据库文件：{}", e);
                error!("错误: {}", err_msg);
                Err(AppError::Connection(err_msg))
            }
        }
    }

    fn list_databases(&self) -> Result<Vec<String>, AppError> {
        // SQLite 是单文件数据库，返回文件名作为数据库名
        let db_name = Path::new(&self.path)
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| AppError::Connection("无效的数据库路径".into()))?;
        
        Ok(vec![db_name.to_string()])
    }

    fn list_tables(&self, _database: &str) -> Result<Vec<String>, AppError> {
        let conn = SQLiteConnection::open(&self.path)
            .map_err(|e| AppError::Connection(format!("无法打开数据库：{}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master 
             WHERE type='table' AND name NOT LIKE 'sqlite_%'
             ORDER BY name"
        ).map_err(|e| AppError::Connection(format!("查询失败：{}", e)))?;

        let tables = stmt.query_map([], |row| row.get(0))
            .map_err(|e| AppError::Connection(format!("获取表名失败：{}", e)))?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| AppError::Connection(format!("处理表名失败：{}", e)))?;

        Ok(tables)
    }

    fn get_table_columns(&self, table_name: &str) -> Result<Vec<ColumnInfo>, AppError> {
        let conn = SQLiteConnection::open(&self.path)
            .map_err(|e| AppError::Connection(format!("无法打开数据库：{}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT 
                name,           -- 列名
                type,           -- 数据类型
                [notnull],      -- 是否可空
                pk,            -- 是否主键
                dflt_value     -- 默认值
             FROM pragma_table_info(?)"
        ).map_err(|e| AppError::Connection(format!("查询失败：{}", e)))?;

        let columns = stmt.query_map([table_name], |row| {
            let type_str: String = row.get(1)?;
            // 解析类型字符串，提取类型和长度信息
            let (type_, length) = parse_sqlite_type(&type_str);
            
            Ok(ColumnInfo {
                name: row.get(0)?,
                type_: type_.to_string(),
                length,
                nullable: !row.get::<_, i32>(2)?.eq(&1),
                is_primary: row.get::<_, i32>(3)?.eq(&1),
                comment: None,
            })
        }).map_err(|e| AppError::Connection(format!("查询失败：{}", e)))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Connection(format!("处理列信息失败：{}", e)))?;

        Ok(columns)
    }
}

// 添加一个辅助函数来解析 SQLite 类型字符串
fn parse_sqlite_type(type_str: &str) -> (&str, Option<i32>) {
    // 处理带有长度的类型，如 VARCHAR(255)
    if let Some(pos) = type_str.find('(') {
        if let Some(end_pos) = type_str.find(')') {
            let base_type = &type_str[..pos];
            let length_str = &type_str[pos + 1..end_pos];
            if let Ok(length) = length_str.parse::<i32>() {
                return (base_type, Some(length));
            }
        }
    }
    (type_str, None)
}

pub fn create_connection(config: &DatabaseConfig) -> Result<Box<dyn DatabaseConnection>, AppError> {
    debug!("创建数据库连接，类型: {:?}", config.type_);
    
    match config.type_ {
        DatabaseType::SQLite3 => {
            debug!("创建 SQLite3 连接: {}", config.host);
            Ok(Box::new(SQLite3Connection::new(config.host.clone())))
        },
        DatabaseType::MySQL => {
            let err_msg = "MySQL 数据库连接暂未实现，请等待后续版本";
            error!("{}", err_msg);
            Err(AppError::Connection(err_msg.into()))
        },
        DatabaseType::Oracle => {
            let err_msg = "Oracle 数据库连接暂未实现，请等待后续版本";
            error!("{}", err_msg);
            Err(AppError::Connection(err_msg.into()))
        },
        DatabaseType::DM => {
            let err_msg = "达梦数据库连接暂未实现，请等待后续版本";
            error!("{}", err_msg);
            Err(AppError::Connection(err_msg.into()))
        },
    }
} 