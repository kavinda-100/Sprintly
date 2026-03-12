use axum::Json;
use serde_json::{Value, json};

/**
 * Root handler for the base route "/", returns a welcome message.
 */
pub async fn root_handler() -> Json<Value> {
    Json(json!({
        "message": "Welcome to the Sprintly API!",
        "version": "1.0",
        "endpoints": {
            "vehicles": "/api/v1/*"
        }
    }))
}

/**
 * Health check endpoint
 */
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
