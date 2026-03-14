use axum::{
    Router,
    routing::{post, put},
};

use crate::{
    config::AppState,
    controllers::project_controller::{create_project, delete_project, update_project},
};

// Project routes module
// This module defines all routes related to projects
// Route path: base_url/api/v1/projects/*
pub fn create_project_routes() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project))
        .route("/projects/{id}", put(update_project).delete(delete_project))
}
