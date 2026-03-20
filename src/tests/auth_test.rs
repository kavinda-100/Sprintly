use crate::tests::{before_each_test, convert_response_to_string, send_request};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use serde_json::Value;
use uuid::Uuid;

// ===================================== helpers ==================================================

/**
 * This function is a helper to send a request to create a user.
 */
#[allow(dead_code)]
async fn create_user(app: axum::Router, email: &str) {
    let request_body = serde_json::json!({
        "name": "Test User",
        "email": email,
        "password": "password123",
        "confirm_password": "password123"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();

    let _ = send_request(app, request).await;
}

//=================================== tests for user registration ==================================================

/**
 * This test verify the user registration process.
 */
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

    // convert the bytes into a UTF-8 string
    let body_string = convert_response_to_string(response).await;

    // Assert that the response status code is 200 OK
    assert_eq!(status, StatusCode::OK);

    // Deserialize the response body into a JSON value
    let body_json: Value = serde_json::from_str(&body_string).unwrap();

    // Assert that the response contains the expected status message
    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["status_code"], 201);
    assert_eq!(body_json["message"], "User registered successfully");
}

/**
 * This test verify the user registration process with mismatched passwords.
 */
#[tokio::test]
async fn test_register_user_password_mismatch() {
    let app = before_each_test().await;

    let unique_email = format!("test+{}@example.com", Uuid::new_v4());
    let request_body = serde_json::json!({
        "name": "Test User",
        "email": unique_email,
        "password": "password123",
        "confirm_password": "password456"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();

    let response = send_request(app, request).await;
    let status = response.status();

    // convert the bytes into a UTF-8 string
    let body_string = convert_response_to_string(response).await;

    // Deserialize the response body into a JSON value
    let body_json: Value = serde_json::from_str(&body_string).unwrap();

    // Assert that the response status code is 400 Bad Request
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body_json["success"], false);
    assert_eq!(body_json["status_code"], 400);
    assert_eq!(
        body_json["message"],
        "confirm_password: Passwords do not match"
    );
}

/**
 * This test verify the user registration process with an already registered email.
 */
#[tokio::test]
async fn test_register_user_email_already_registered() {
    let app = before_each_test().await;

    let email = "test@conflict2.com";

    // Create a user with the email to set up the conflict scenario
    create_user(app.clone(), email).await;

    // let unique_email = format!("test+{}@example.com", Uuid::new_v4());
    let request_body = serde_json::json!({
        "name": "Test User",
        "email": email,
        "password": "password123",
        "confirm_password": "password123"
    });

    let request_two = Request::builder()
        .method("POST")
        .uri("/api/v1/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();

    // Send the second request to attempt registering the same email again
    let response = send_request(app, request_two).await;
    let status = response.status();

    // Convert the response body string
    let body_string = convert_response_to_string(response).await;

    // Deserialize the response body into a JSON value
    let body_json: Value = serde_json::from_str(&body_string).unwrap();

    // Assert that the response status code is 400 Bad Request
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body_json["success"], false);
    assert_eq!(body_json["status_code"], 409);
    assert_eq!(body_json["message"], "User is already registered");
}
