use axum::{Router, routing::get};

use crate::{config::AppState, controllers::auth_controller::auth_check};

// Auth routes module
// This module defines all routes related to authentication
// Route path: base_url/api/v1/auth/*
pub fn create_auth_routes() -> Router<AppState> {
    Router::new()
    .route("/auth/check", get(auth_check))
}
