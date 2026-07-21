pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod parser;

use axum::extract::FromRef;
use axum::routing::{Router, get, patch, post};
use sqlx::SqlitePool;

use crate::config::Config;
use crate::handlers::{articles, auth, feeds};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

pub fn router(state: AppState) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/me", get(auth::me));

    let feed_routes = Router::new()
        .route("/", get(feeds::list).post(feeds::create))
        .route("/{feed_id}", get(feeds::get_one).delete(feeds::delete))
        .route("/{feed_id}/articles", get(articles::list_by_feed));

    let article_routes = Router::new()
        .route("/{article_id}", get(articles::get_one))
        .route("/{article_id}/read", patch(articles::toggle_read))
        .route("/{article_id}/star", patch(articles::toggle_starred));

    Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/api/auth", auth_routes)
        .nest("/api/feeds", feed_routes)
        .nest("/api/articles", article_routes)
        .with_state(state)
}
