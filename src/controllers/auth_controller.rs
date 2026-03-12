use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::config::AppState;

/**
 * Auth check endpoint
 */
pub async fn auth_check(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "auth is healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "app_name": state.app_name,
        "port": state.env_config.port,
    }))
}
