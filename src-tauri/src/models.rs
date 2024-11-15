use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    SQLite3,
    MySQL,
    Oracle,
    DM,
}

impl ToString for DatabaseType {
    fn to_string(&self) -> String {
        match self {
            DatabaseType::SQLite3 => "SQLite3".to_string(),
            DatabaseType::MySQL => "MySQL".to_string(),
            DatabaseType::Oracle => "Oracle".to_string(),
            DatabaseType::DM => "DM".to_string(),
        }
    }
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SQLite3" => Ok(DatabaseType::SQLite3),
            "MySQL" => Ok(DatabaseType::MySQL),
            "Oracle" => Ok(DatabaseType::Oracle),
            "DM" => Ok(DatabaseType::DM),
            _ => Err(format!("未知的数据库类型: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    pub id: Option<i64>,
    pub name: String,
    #[serde(rename = "type_")]
    pub type_: DatabaseType,
    pub host: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub create_time: Option<String>,
    pub last_connect_time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TestConnectionResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct DataBase {
    pub name: String,
    pub encoding: Option<String>,
    pub collation: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub name: String,
    pub type_: String,
    pub engine: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDatabaseSchemasArgs {
    pub config_id: i32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDatabaseTablesArgs {
    pub config_id: i32,
    pub db_name: String
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub length: Option<i32>,
    pub nullable: bool,
    pub is_primary: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTableColumnsArgs {
    pub config_id: i32,
    pub db_name: String,
    pub table_name: String,
} 