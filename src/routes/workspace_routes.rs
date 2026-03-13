use axum::{
    Router,
    routing::{post, put},
};

use crate::{
    config::AppState,
    controllers::workspace_controller::{
        create_workspace, delete_workspace, get_workspaces, update_workspace,
    },
};

// Workspace routes module
// This module defines all routes related to workspaces
// Route path: base_url/api/v1/workspaces/*
pub fn create_workspace_routes() -> Router<AppState> {
    Router::new()
        .route("/workspaces", post(create_workspace).get(get_workspaces))
        .route(
            "/workspaces/{id}",
            put(update_workspace).delete(delete_workspace),
        )
}
