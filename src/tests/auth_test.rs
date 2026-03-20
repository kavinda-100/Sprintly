use crate::tests::{before_each_test, send_request};

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};

use serde_json::Value;
use uuid::Uuid;

#[tokio::test]
async fn test_register_user() {
    let app = before_each_test().await;

    let unique_email = format!("test+{}@example.com", Uuid::new_v4());
    let request_body = serde_json::json!({
        "name": "Test User",
        "email": unique_email,
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
    let status = response.status();

    // Convert the response body into bytes
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    // Print status and body before assertions so failures still show debug info.
    let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();
    println!("Register response status: {}", status);
    println!("Register response body: {}", body_string);

    assert_eq!(
        status,
        StatusCode::CREATED,
        "Unexpected register response body: {}",
        body_string
    );

    // Deserialize the response body into a JSON value
    let body_json: Value = serde_json::from_str(&body_string).unwrap();

    // Assert that the response contains the expected status message
    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["status_code"], 201);
    assert_eq!(body_json["message"], "User registered successfully");
}
