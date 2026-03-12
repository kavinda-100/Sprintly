use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::config::AppState;

/**
 * Root handler for the base route "/", returns a welcome message.
 */
pub async fn root_handler() -> Json<Value> {
    Json(json!({
        "message": "Welcome to the Sprintly API!",
        "version": "1.0",
        "endpoints": {
            "auth": "/api/v1/auth/*"
        }
    }))
}

/**
 * Health check endpoint
 */
pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "app_name": state.app_name,
        "port": state.env_config.port,
    }))
}
