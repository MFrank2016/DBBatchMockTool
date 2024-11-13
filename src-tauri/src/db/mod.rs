use rusqlite::Connection;
use tauri::Manager;
use std::sync::Mutex;
use crate::error::AppError;
use crate::models::{DatabaseConfig, DatabaseType};
use log::{debug, error};
use std::str::FromStr;
use std::io::{Error as IoError, ErrorKind};
use anyhow::Result;

pub mod connection;

pub struct DatabaseManager {
    conn: Mutex<Connection>,
}

impl DatabaseManager {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, AppError> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|_| AppError::Config("无法获取应用数据目录".into()))?;

        std::fs::create_dir_all(&app_dir)
            .map_err(|e| AppError::Config(e.to_string()))?;

        let db_path = app_dir.join("database.db");
        let conn = Connection::open(db_path)
            .map_err(|e| AppError::Database(e.to_string()))?;

        // 修改表结构，允许 username 和 password 为 NULL
        conn.execute(
            "CREATE TABLE IF NOT EXISTS database_configs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                host TEXT NOT NULL,
                username TEXT,           -- 移除 NOT NULL 约束
                password TEXT,           -- 移除 NOT NULL 约束
                create_time DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_connect_time DATETIME
            )",
            [],
        )
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn add_config(&self, config: &DatabaseConfig) -> Result<i64, AppError> {
        let conn = self.conn.lock()
            .map_err(|e| AppError::Database(format!("无法获取数据库锁: {}", e)))?;
            
        conn.execute(
            "INSERT INTO database_configs (name, type, host, username, password, create_time)
             VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)",
            (
                &config.name,
                config.type_.to_string(),
                &config.host,
                &config.username,
                &config.password,
            ),
        )
        .map_err(|e| AppError::Database(e.to_string()))?;
        
        Ok(conn.last_insert_rowid())
    }

    pub fn list_configs(&self) -> Result<Vec<DatabaseConfig>, AppError> {
        let conn = self.conn.lock().map_err(|e| 
            AppError::Database(format!("无法获取数据库锁: {}", e))
        )?;

        let mut stmt = conn.prepare(
            "SELECT id, name, type, host, username, password, 
                    datetime(create_time) as create_time, 
                    datetime(last_connect_time) as last_connect_time 
             FROM database_configs 
             ORDER BY create_time DESC"
        ).map_err(|e| AppError::Database(e.to_string()))?;

        let configs = stmt.query_map([], |row| {
            let type_str: String = row.get(2)?;
            let db_type = DatabaseType::from_str(&type_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(IoError::new(ErrorKind::InvalidData, e))
                ))?;
            
            Ok(DatabaseConfig {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                type_: db_type,
                host: row.get(3)?,
                username: row.get(4)?,
                password: row.get(5)?,
                create_time: row.get(6)?,
                last_connect_time: row.get(7)?,
            })
        }).map_err(|e| AppError::Database(e.to_string()))?;

        configs.collect::<rusqlite::Result<_>>()
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub fn update_last_connect_time(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock()
            .map_err(|e| AppError::Database(format!("无法获取数据库锁: {}", e)))?;
            
        conn.execute(
            "UPDATE database_configs 
             SET last_connect_time = CURRENT_TIMESTAMP 
             WHERE id = ?1",
            [id],
        ).map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn test_connection(&self, config: &DatabaseConfig) -> Result<(), AppError> {
        debug!("Testing connection for database: {}", config.name);
        let conn = connection::create_connection(config)?;
        
        match conn.test_connection() {
            Ok(_) => {
                if let Some(id) = config.id {
                    debug!("Updating last connect time for database id: {}", id);
                    self.update_last_connect_time(id)?;
                }
                Ok(())
            }
            Err(e) => {
                error!("Connection test failed: {}", e);
                Err(e)
            }
        }
    }
}