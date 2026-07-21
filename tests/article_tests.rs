mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_list_articles_nonexistent_feed() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/feeds/nonexistent-id/articles")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_article_requires_auth() {
    let (app, _pool) = helpers::setup().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/articles/some-id")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_nonexistent_article() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/articles/nonexistent-id")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_toggle_read_nonexistent_article() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("PATCH")
        .uri("/api/articles/nonexistent-id/read")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_toggle_starred_nonexistent_article() {
    let (app, _pool) = helpers::setup().await;
    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("PATCH")
        .uri("/api/articles/nonexistent-id/star")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
