mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_register_success() {
    let (app, _db) = helpers::setup().await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "username": "testuser",
                "email": "test@example.com",
                "password": "password123"
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["token"].is_string());
    assert_eq!(json["user"]["username"], "testuser");
    assert_eq!(json["user"]["email"], "test@example.com");
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let (app, _db) = helpers::setup().await;

    let body = serde_json::to_string(&serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password123"
    }))
    .unwrap();

    let req1 = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(body.clone()))
        .unwrap();

    let resp1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);

    let req2 = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_register_short_password() {
    let (app, _db) = helpers::setup().await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "username": "testuser",
                "email": "test@example.com",
                "password": "short"
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_login_success() {
    let (app, _db) = helpers::setup().await;

    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;
    assert!(!token.is_empty());

    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "email": "test@example.com",
                "password": "password123"
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["token"].is_string());
    assert_eq!(json["user"]["email"], "test@example.com");
}

#[tokio::test]
async fn test_login_wrong_password() {
    let (app, _db) = helpers::setup().await;

    let _token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "email": "test@example.com",
                "password": "wrongpassword"
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_me_requires_auth() {
    let (app, _db) = helpers::setup().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/auth/me")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_me_returns_user() {
    let (app, _db) = helpers::setup().await;

    let token = helpers::register_user(&app, "testuser", "test@example.com", "password123").await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/auth/me")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["username"], "testuser");
    assert_eq!(json["email"], "test@example.com");
}
