use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodePermission {
    LOCKED,
    OPEN,
}

impl AsRef<str> for NodePermission {
    fn as_ref(&self) -> &str {
        match self {
            NodePermission::LOCKED => "LOCKED",
            NodePermission::OPEN => "OPEN",
        }
    }
}

impl From<String> for NodePermission {
    fn from(s: String) -> Self {
        match s.as_str() {
            "LOCKED" => NodePermission::LOCKED,
            _ => NodePermission::OPEN,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Node {
    pub id: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub node_type: String,
    pub payload: Option<Vec<u8>>,
    pub wasm_hash: Option<String>,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Edge {
    pub from_id: String,
    pub to_id: String,
    pub contract: Option<String>,
}

pub async fn connect(url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(url).await
}

pub async fn migrate(_pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // This is a placeholder for migration logic
    Ok(())
}
