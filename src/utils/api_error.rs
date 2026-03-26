use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ApiError {
    Unauthorized(String),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
    Conflict(String),
    Forbidden(String),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    success: bool,
    status_code: u16,
    message: String,
    data: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };

        let body = serde_json::to_string(&ApiErrorResponse {
            success: false,
            status_code: status.as_u16(),
            message,
            data: None,
        })
        .unwrap_or_else(|_| "{}".to_string());

        (status, body).into_response()
    }
}
