use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::task_enum::{TaskPriority, TaskStatus};

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

    pub task_status: TaskStatus,
    pub task_priority: TaskPriority,

    pub due_date: Option<NaiveDateTime>,
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

    pub task_status: Option<TaskStatus>,
    pub task_priority: Option<TaskPriority>,

    pub due_date: Option<chrono::NaiveDateTime>,

    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
// GET /projects/{project_id}/tasks?status=todo&priority=high&page=1&page_size=20
pub struct TaskQuery {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub task_status: TaskStatus,
    pub task_priority: TaskPriority,
    pub owner_id: Uuid,
    pub due_date: Option<NaiveDateTime>,
    pub position: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
