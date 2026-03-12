use axum::{Router, routing::get};
// use sqlx::PgPool;

pub mod auth_routes;

use crate::{
    config::AppState,
    controllers::root_controller::{health_check, root_handler},
    routes::auth_routes::create_auth_routes,
};

/// Creates the main API router with all route groups
/// This is where you compose all your route modules together
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Root routes (no prefix)
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        // API v1 routes
        .nest("/api/v1", api_v1_routes())
    // API v2 routes (future expansion)
    // .nest("/api/v2", api_v2_routes(pool))
}

// Helper function to create API v1 routes
fn api_v1_routes() -> Router<AppState> {
    Router::new().merge(create_auth_routes())
    // other routes
    // .merge(create_user_routes())
}
