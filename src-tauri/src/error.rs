use serde::Serialize;
use thiserror::Error;
use rusqlite;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(String),
    
    #[error("连接错误: {0}")]
    Connection(String),
    
    #[error("配置错误: {0}")]
    Config(String),
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
} 