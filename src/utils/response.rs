use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub status_code: u16,
    pub message: String,
    // Only include data if it's Some, otherwise skip it in the JSON response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(
        success: bool,
        status_code: StatusCode,
        message: impl Into<String>,
        data: Option<T>,
    ) -> Self {
        Self {
            success,
            status_code: status_code.as_u16(),
            message: message.into(),
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK);

        let body = serde_json::to_string(&ApiResponse {
            success: true,
            status_code: status.as_u16(),
            message: self.message,
            data: self.data,
        })
        .unwrap_or_else(|_| "{}".to_string());

        (status, body).into_response()
    }
}
