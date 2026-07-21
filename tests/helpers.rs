use axum::Router;
use axum::body::Body;
use axum::http::Request;
use calm_rss::config::Config;
use calm_rss::db;
use calm_rss::{AppState, router};
use sqlx::SqlitePool;
use tower::ServiceExt;

pub async fn setup() -> (Router, SqlitePool) {
    let config = Config::for_test();
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("failed to create test pool");

    let state = AppState {
        db: pool.clone(),
        config,
    };

    (router(state), pool)
}

pub async fn register_user(app: &Router, username: &str, email: &str, password: &str) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&serde_json::json!({
                "username": username,
                "email": email,
                "password": password,
            }))
            .unwrap(),
        ))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    json["token"].as_str().unwrap().to_string()
}
