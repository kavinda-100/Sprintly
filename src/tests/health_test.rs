use crate::tests::before_each_test;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};

use serde_json::Value;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_health_check() {
    let app = before_each_test().await;

    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body_json["status"], "API is Healthy");
    assert_eq!(body_json["app_name"], "Sprintly API");
    assert!(body_json.get("timestamp").is_some());
}
