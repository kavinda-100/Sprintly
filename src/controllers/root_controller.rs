use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::config::AppState;

/**
 * Health check endpoint
 */
pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    tracing::info!("Health check endpoint called");

    Json(json!({
        "status": "API is Healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "app_name": state.app_name,
        "port": state.env_config.port,
    }))
}
