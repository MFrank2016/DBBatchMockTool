// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod error;

use db::DatabaseManager;
use models::{DatabaseConfig, TestConnectionResult};
use error::AppError;
use tauri::{State, Manager};
use log::{error, info, debug, LevelFilter};
use std::io::Write;

#[tauri::command]
async fn add_database(
    config: DatabaseConfig,
    db: State<'_, DatabaseManager>,
) -> Result<i64, AppError> {
    db.add_config(&config)
}

#[tauri::command]
async fn list_databases(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<DatabaseConfig>, AppError> {
    db.list_configs()
}

#[tauri::command]
async fn test_connection(
    config: DatabaseConfig,
    db: State<'_, DatabaseManager>,
) -> Result<TestConnectionResult, String> {
    debug!("收到连接测试请求");
    println!("收到连接测试请求");

    match db.test_connection(&config) {
        Ok(_) => {
            let msg = format!("数据库 {} 连接成功", config.name);
            info!("{}", msg);
            println!("{}", msg);
            Ok(TestConnectionResult {
                success: true,
                message: msg,
            })
        }
        Err(e) => {
            let msg = format!("数据库 {} 连接失败: {}", config.name, e);
            error!("{}", msg);
            println!("{}", msg);
            Ok(TestConnectionResult {
                success: false,
                message: msg,
            })
        }
    }
}

fn main() {
    // 配置日志
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    info!("应用程序启动");
    debug!("调试日志已启用");

    tauri::Builder::default()
        .setup(|app| {
            debug!("初始化数据库管理器...");
            let db = DatabaseManager::new(app.handle())
                .map_err(|e| {
                    error!("数据库管理器初始化失败: {}", e);
                    e
                })?;
            app.manage(db);
            debug!("数据库管理器初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_database,
            list_databases,
            test_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
