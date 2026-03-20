use crate::tests::{before_each_test, convert_response_to_string, create_test_user, send_request};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use serde_json::Value;

// ====================================== tests for workspace management ==================================================

/**
 * This test verify the workspace creation process with valid input.
 */
#[tokio::test]
async fn test_create_workspace() {
    let app = before_each_test().await;

    // test user
    let user = create_test_user();
    let workspace_name = "Test Workspace";

    // prepare the request payload
    let payload = serde_json::json!({
        "name": workspace_name
    });

    // request to create a workspace
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/workspaces")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    // send the request to create a workspace
    let response = send_request(app, request).await;
    let status = response.status();

    // convert the bytes into a UTF-8 string
    let body_string = convert_response_to_string(response).await;

    // Deserialize the response body into a JSON value
    let body_json: Value = serde_json::from_str(&body_string).unwrap();

    // Assert that the response status code is 200 OK
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["status_code"], 201);
    assert_eq!(body_json["message"], "Workspace created successfully");
    // Assert that the response contains the expected user information
    assert!(body_json["data"]["id"].as_str().is_some());
    assert_eq!(body_json["data"]["name"], workspace_name);
    assert_eq!(body_json["data"]["owner_id"], user.id.to_string());
    assert!(body_json["data"]["created_at"].as_str().is_some());
    assert!(body_json["data"]["updated_at"].as_str().is_some());
}

// =================================== tests for get workspaces ==================================================

/**
 * This test verify the get workspaces created by a logged in user process.
 */
#[tokio::test]
async fn test_get_workspaces_for_logged_in_user() {
    let app = before_each_test().await;

    // test user (logged in user)
    let _ = create_test_user();

    // request to get workspaces
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/workspaces")
        .body(Body::empty())
        .unwrap();

    // send the request to get workspaces
    let response = send_request(app, request).await;
    let status = response.status();

    // convert the bytes into a UTF-8 string
    let body_string = convert_response_to_string(response).await;

    // Deserialize the response body into a JSON value
    let body_json: Value = serde_json::from_str(&body_string).unwrap();

    // Assert that the response status code is 200 OK
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["status_code"], 200);
    assert_eq!(body_json["message"], "Workspaces retrieved successfully");
}
