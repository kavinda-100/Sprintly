use axum::{Router, routing::get};
// use sqlx::PgPool;

use crate::{controllers::root_controller::{health_check, root_handler}, config::AppState};

/// Creates the main API router with all route groups
/// This is where you compose all your route modules together
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Root routes (no prefix)
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        // API v1 routes
        // .nest("/api/v1", api_v1_routes(pool))
    // API v2 routes (future expansion)
    // .nest("/api/v2", api_v2_routes(pool))
}


// fn api_v1_routes() -> Router<AppState> {
//     Router::new().merge(vehicle_routes())
// }