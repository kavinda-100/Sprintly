use axum::{
    Router,
    routing::{post, put},
};

use crate::{
    config::AppState,
    controllers::task_controller::{create_task, delete_task, get_task_by_id, update_task},
};

// Task routes module
// This module defines all routes related to tasks
// Route path: base_url/api/v1/tasks/*
pub fn create_task_routes() -> Router<AppState> {
    Router::new().route("/tasks", post(create_task)).route(
        "/tasks/{task_id}",
        put(update_task).delete(delete_task).get(get_task_by_id),
    )
}
