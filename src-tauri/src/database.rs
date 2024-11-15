use crate::models::{DataBase, Table};
use crate::error::AppError;
use crate::db::DatabaseManager;
use tauri::State;
use log::{debug, error};

#[tauri::command]
pub async fn list_database_schemas(config_id: i32, state: State<'_, AppState>) -> Result<Vec<DataBase>, String> {
    debug!("获取数据库列表, config_id: {}", config_id);
    
    match state.db_manager.list_databases(config_id as i64) {
        Ok(names) => {
            let databases = names.into_iter()
                .map(|name| DataBase {
                    name,
                    encoding: None,
                    collation: None,
                })
                .collect();
            Ok(databases)
        }
        Err(e) => {
            error!("获取数据库列表失败: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command] 
pub async fn list_database_tables(
    config_id: i32,
    db_name: String,
    state: State<'_, AppState>
) -> Result<Vec<Table>, String> {
    debug!("获取数据表列表, config_id: {}, db_name: {}", config_id, db_name);
    
    match state.db_manager.list_tables(config_id as i64, &db_name) {
        Ok(names) => {
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
        Err(e) => {
            error!("获取数据表列表失败: {}", e);
            Err(e.to_string())
        }
    }
} 