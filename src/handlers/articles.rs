use axum::Json;
use axum::extract::{Path, State};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::article::Article;
use crate::models::feed::Feed;

pub async fn list_by_feed(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Path(feed_id): Path<String>,
) -> Result<Json<Vec<Article>>, AppError> {
    let feed = Feed::find_by_id(&state.db, &feed_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Feed not found".into()))?;

    if feed.user_id != user_id {
        return Err(AppError::NotFound("Feed not found".into()));
    }

    let articles = Article::find_by_feed(&state.db, &feed_id).await?;
    Ok(Json(articles))
}

pub async fn get_one(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Path(article_id): Path<String>,
) -> Result<Json<Article>, AppError> {
    let article = Article::find_by_id(&state.db, &article_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".into()))?;

    let feed = Feed::find_by_id(&state.db, &article.feed_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Feed not found".into()))?;

    if feed.user_id != user_id {
        return Err(AppError::NotFound("Article not found".into()));
    }

    Ok(Json(article))
}

pub async fn toggle_read(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Path(article_id): Path<String>,
) -> Result<Json<Article>, AppError> {
    let article = Article::find_by_id(&state.db, &article_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".into()))?;

    let feed = Feed::find_by_id(&state.db, &article.feed_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Feed not found".into()))?;

    if feed.user_id != user_id {
        return Err(AppError::NotFound("Article not found".into()));
    }

    let article = Article::toggle_read(&state.db, &article_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".into()))?;

    Ok(Json(article))
}

pub async fn toggle_starred(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Path(article_id): Path<String>,
) -> Result<Json<Article>, AppError> {
    let article = Article::find_by_id(&state.db, &article_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".into()))?;

    let feed = Feed::find_by_id(&state.db, &article.feed_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Feed not found".into()))?;

    if feed.user_id != user_id {
        return Err(AppError::NotFound("Article not found".into()));
    }

    let article = Article::toggle_starred(&state.db, &article_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Article not found".into()))?;

    Ok(Json(article))
}
