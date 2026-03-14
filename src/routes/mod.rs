use axum::{Router, routing::get};
use tower_http::services::fs::ServeDir;

pub mod auth_routes;
pub mod project_routes;
pub mod task_routes;
pub mod workspace_routes;

use crate::{
    config::AppState,
    controllers::root_controller::health_check,
    routes::{
        auth_routes::create_auth_routes, project_routes::create_project_routes,
        task_routes::create_task_routes, workspace_routes::create_workspace_routes,
    },
};

/// Creates the main API router with all route groups
/// This is where you compose all your route modules together
pub fn create_routes() -> Router<AppState> {
    // Serve static files from the "public" directory at the root path
    let static_files = ServeDir::new("public");

    Router::new()
        // Specific routes first
        .route("/health", get(health_check))
        // API v1 routes
        .nest("/api/v1", api_v1_routes())
        // API v2 routes (future expansion)
        // .nest("/api/v2", api_v2_routes(pool))
        // Serve static files as fallback (only when no other routes match)
        .fallback_service(static_files)
}

// Helper function to create API v1 routes
fn api_v1_routes() -> Router<AppState> {
    Router::new()
        .merge(create_auth_routes())
        .merge(create_workspace_routes())
        .merge(create_project_routes())
        .merge(create_task_routes())
}
