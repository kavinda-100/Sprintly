use axum::{Router, routing::post};

use crate::{config::AppState, controllers::workspace_controller::create_workspace};

// Workspace routes module
// This module defines all routes related to workspaces
// Route path: base_url/api/v1/workspaces/*
pub fn create_workspace_routes() -> Router<AppState> {
    Router::new().route("/workspaces", post(create_workspace))
}
