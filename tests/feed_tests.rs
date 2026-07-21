mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_list_feeds_empty() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/feeds")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, serde_json::json!([]));
}

#[tokio::test]
async fn test_create_feed_requires_auth() {
    let (app, _pool) = helpers::setup().await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/feeds")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "url": "https://example.com/feed.xml"
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_feed_empty_url() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/feeds")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "url": ""
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_nonexistent_feed() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/feeds/nonexistent-id")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_feed() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("DELETE")
        .uri("/api/feeds/nonexistent-id")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
