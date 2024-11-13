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
pub struct DatabaseConfig {
    pub id: Option<i64>,
    pub name: String,
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