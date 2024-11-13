use crate::error::AppError;
use crate::models::{DatabaseConfig, DatabaseType};
use rusqlite::Connection as SQLiteConnection;
use std::path::Path;
use log::{debug, error};

pub trait DatabaseConnection {
    fn test_connection(&self) -> Result<(), AppError>;
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
        println!("测试 SQLite3 连接: {}", self.path);
        
        if !Path::new(&self.path).exists() {
            let err_msg = format!("找不到数据库文件：{}", self.path);
            println!("错误: {}", err_msg);
            return Err(AppError::Connection(err_msg));
        }

        match SQLiteConnection::open(&self.path) {
            Ok(conn) => {
                match conn.execute_batch("SELECT 1") {
                    Ok(_) => {
                        println!("SQLite3 连接测试成功");
                        Ok(())
                    }
                    Err(e) => {
                        let err_msg = format!("数据库查询测试失败：{}", e);
                        println!("错误: {}", err_msg);
                        Err(AppError::Connection(err_msg))
                    }
                }
            }
            Err(e) => {
                let err_msg = format!("无法打开数据库文件：{}", e);
                println!("错误: {}", err_msg);
                Err(AppError::Connection(err_msg))
            }
        }
    }
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