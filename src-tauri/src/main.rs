// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod error;

use db::DatabaseManager;
use models::{DatabaseConfig, TestConnectionResult, ListDatabaseSchemasArgs, ListDatabaseTablesArgs, DataBase, Table};
use error::AppError;
use tauri::{State, Manager};
use log::{info, debug, error};
use std::io::Write;
use chrono::Local;
use std::fs;
use env_logger::{Builder, Target};
use serde::Deserialize;

// 定义应用状态
pub struct AppState {
    pub db_manager: DatabaseManager
}

// 定义所有命令的参数结构体
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteDatabaseArgs {
    config_id: i64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDatabaseArgs {
    config_id: i64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListDatabasesArgs {
    config_id: i64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListTablesArgs {
    config_id: i64,
    database: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestConnectionArgs {
    config_id: i64
}

#[tauri::command]
async fn add_database(
    config: DatabaseConfig,
    state: State<'_, AppState>,
) -> Result<i64, AppError> {
    state.db_manager.add_config(&config)
}

#[tauri::command]
async fn list_databases(
    args: ListDatabasesArgs,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    debug!("获取数据库列表, config_id: {}", args.config_id);
    state.db_manager.list_databases(args.config_id)
}

#[tauri::command]
async fn list_tables(
    args: ListTablesArgs,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    debug!("获取表列表, config_id: {}, database: {}", args.config_id, args.database);
    state.db_manager.list_tables(args.config_id, &args.database)
}

#[tauri::command]
async fn test_connection(
    args: TestConnectionArgs, 
    state: State<'_, AppState>
) -> Result<TestConnectionResult, String> {
    debug!("测试数据库连接, config_id: {}", args.config_id);
    
    let config = state.db_manager.get_config(args.config_id)
        .map_err(|e| e.to_string())?;
        
    match state.db_manager.test_connection(&config) {
        Ok(_) => Ok(TestConnectionResult {
            success: true,
            message: "连接成功".to_string()
        }),
        Err(e) => Ok(TestConnectionResult {
            success: false,
            message: e.to_string()
        })
    }
}

#[tauri::command]
async fn test_connection_with_config(
    config: DatabaseConfig,
    state: State<'_, AppState>
) -> Result<TestConnectionResult, String> {
    debug!("测试数据库连接配置: {:?}", config);
    
    // 直接测试连接
    match state.db_manager.test_connection(&config) {
        Ok(_) => Ok(TestConnectionResult {
            success: true,
            message: "连接成功".to_string()
        }),
        Err(e) => Ok(TestConnectionResult {
            success: false,
            message: e.to_string()
        })
    }
}

#[tauri::command]
async fn delete_database(
    args: DeleteDatabaseArgs,
    state: State<'_, AppState>
) -> Result<(), AppError> {
    debug!("接收到删除数据库配置请求: config_id = {}", args.config_id);
    
    match state.db_manager.delete_config(args.config_id) {
        Ok(_) => {
            info!("成功删除数据库配置: config_id = {}", args.config_id);
            Ok(())
        }
        Err(e) => {
            error!("删除数据库配置失败: config_id = {}, error = {:?}", args.config_id, e);
            Err(e)
        }
    }
}

#[tauri::command]
async fn update_database(
    config: DatabaseConfig,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    debug!("更新数据库配置: {:?}", config);
    state.db_manager.update_config(&config)
}

#[tauri::command]
async fn list_configs(
    state: State<'_, AppState>,
) -> Result<Vec<DatabaseConfig>, AppError> {
    debug!("获取数据库配置列表");
    state.db_manager.list_configs()
}

#[tauri::command]
async fn get_database(
    args: GetDatabaseArgs,
    state: State<'_, AppState>
) -> Result<DatabaseConfig, AppError> {
    debug!("获取数据库配置, config_id: {}", args.config_id);
    state.db_manager.get_config(args.config_id)
}

#[tauri::command]
async fn list_database_schemas(
    args: ListDatabaseSchemasArgs,
    state: State<'_, AppState>
) -> Result<Vec<DataBase>, AppError> {
    debug!("获取数据库列表, config_id: {}", args.config_id);
    
    let names = state.db_manager.list_databases(args.config_id as i64)?;
    let databases = names.into_iter()
        .map(|name| DataBase {
            name,
            encoding: None,
            collation: None,
        })
        .collect();
    Ok(databases)
}

#[tauri::command] 
async fn list_database_tables(
    args: ListDatabaseTablesArgs,
    state: State<'_, AppState>
) -> Result<Vec<Table>, AppError> {
    debug!("获取数据表列表, config_id: {}, db_name: {}", args.config_id, args.db_name);
    
    let names = state.db_manager.list_tables(args.config_id as i64, &args.db_name)?;
    let tables = names.into_iter()
        .map(|name| Table {
            name,
            type_: "TABLE".to_string(),
            engine: None,
            comment: None,
        })
        .collect();
    Ok(tables)
}

fn main() {
    setup_logging();
    println!("日志系统已初始化完成");
    
    tauri::Builder::default()
        .setup(|app| {
            debug!("Tauri 应用程序开始初始化");
            // 获取 app_handle
            let app_handle = app.handle();
            
            // 初始化数据库管理器
            debug!("开始初始化数据库管理器");
            let db_manager = DatabaseManager::new(&app_handle)?;
            debug!("数据库管理器初始化完成");
            
            // 创建应用状态
            let state = AppState { db_manager };
            // 管理应用状态
            app.manage(state);
            debug!("应用状态初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_database,
            list_configs,
            list_databases,
            list_tables,
            test_connection,
            test_connection_with_config,
            delete_database,
            update_database,
            get_database,
            list_database_schemas,
            list_database_tables,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 设置日志
fn setup_logging() {
    let log_dir = "logs";
    fs::create_dir_all(log_dir).expect("Failed to create log directory");
    
    let log_file = format!("{}/app_{}.log", log_dir, Local::now().format("%Y-%m-%d"));
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .expect("Failed to open log file");

    // 创建一个多写入器，同时写入文件和控制台
    let mut builder = Builder::new();
    
    // 设置日志格式
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(LOG_LEVEL)
        // 添加文件输出
        .target(Target::Pipe(Box::new(file)))
        // 添加控制台输出
        .target(Target::Stdout);

    // 初始化日志系统
    builder.init();

    // 输出初始化日志
    info!("应用程序启动");
    debug!("调试日志已启用");
    debug!("日志系统初始化完成");
}

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;
