use crate::tests::{before_each_test, send_request};

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};

use serde_json::Value;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_register_user() {
    let app = before_each_test().await;

    let request_body = serde_json::json!({
        "name": "Test User",
        "email": "test@example.com",
        "password": "password123",
        "confirm_password": "password123"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();

    let response = send_request(app, request).await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

    // print the response body for debugging
    println!("Response body: {:?}", body_json);

    // Assert that the response contains the expected status message
    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["status_code"], 201);
    assert_eq!(body_json["message"], "User registered successfully");
}
