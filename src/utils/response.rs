use axum::http::StatusCode;
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

impl<T> ApiResponse<T> {
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
