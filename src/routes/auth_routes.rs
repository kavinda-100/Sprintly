use axum::{
    Router,
    routing::{get, post},
};

use crate::{config::AppState, controllers::auth_controller::register_user};

// Auth routes module
// This module defines all routes related to authentication
// Route path: base_url/api/v1/auth/*
pub fn create_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/me", get(|| async { "Get current user info" }))
        .route("/auth/register", post(register_user))
}
