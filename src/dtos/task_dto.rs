use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskPayload {
    pub project_id: Uuid,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Title must be between 2 and 100 characters"
    ))]
    pub title: String,

    #[validate(length(max = 1000, message = "Description too long (1000 characters max)"))]
    pub description: Option<String>,

    pub task_status: i64,
    pub task_priority: i64,

    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTaskPayload {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Title must be between 2 and 100 characters"
    ))]
    pub title: Option<String>,

    #[validate(length(max = 1000, message = "Description too long (1000 characters max)"))]
    pub description: Option<String>,

    pub task_status: Option<i64>,
    pub task_priority: Option<i64>,

    pub due_date: Option<String>,

    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
// GET /projects/{project_id}/tasks?status=todo&priority=high&page=1&page_size=20
pub struct TaskQuery {
    pub status: Option<i64>,
    pub priority: Option<i64>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub task_status: i64,
    pub task_priority: i64,
    pub owner_id: Uuid,
    pub due_date: Option<String>,
    pub position: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
