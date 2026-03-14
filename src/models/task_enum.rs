use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text")]
pub enum TaskStatus {
    #[serde(rename = "todo")]
    Todo,

    #[serde(rename = "in_progress")]
    InProgress,

    #[serde(rename = "done")]
    Done,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "text")]
pub enum TaskPriority {
    #[serde(rename = "low")]
    Low,

    #[serde(rename = "medium")]
    Medium,

    #[serde(rename = "high")]
    High,
}
