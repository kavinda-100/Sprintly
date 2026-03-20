use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    config::AppState,
    controllers::project_controller::{
        create_project, delete_project, get_all_tasks_for_project, get_project_by_id,
        update_project,
    },
};

// Project routes module
// This module defines all routes related to projects
// Route path: base_url/api/v1/projects/*
pub fn create_project_routes() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route(
            "/projects/{project_id}",
            get(get_project_by_id)
                .put(update_project)
                .delete(delete_project),
        )
        .route(
            "/projects/{project_id}/tasks",
            get(get_all_tasks_for_project),
        )
}
