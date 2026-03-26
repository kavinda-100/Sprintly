use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskStatusPayload {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Name must be between 2 and 50 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
#[allow(dead_code)]
pub struct UpdateTaskStatusPayload {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Name must be between 2 and 50 characters"
    ))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskStatusResponse {
    pub id: i64,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
