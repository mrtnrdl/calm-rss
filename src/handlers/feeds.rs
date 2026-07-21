use axum::Json;
use axum::extract::{Path, State};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::feed::{CreateFeedInput, Feed};
use crate::parser;

pub async fn list(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<Feed>>, AppError> {
    let feeds = Feed::find_by_user(&state.db, &user_id).await?;
    Ok(Json(feeds))
}

pub async fn create(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Json(input): Json<CreateFeedInput>,
) -> Result<Json<Feed>, AppError> {
    if input.url.is_empty() {
        return Err(AppError::BadRequest("URL is required".into()));
    }

    if Feed::find_by_url_for_user(&state.db, &user_id, &input.url)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Feed already exists".into()));
    }

    let parsed = parser::fetch_feed(&input.url).await?;

    let feed = Feed::create(
        &state.db,
        &user_id,
        &parsed.title,
        &input.url,
        parsed.site_url.as_deref(),
    )
    .await?;

    parser::store_articles(&state.db, &feed.id, &parsed.entries).await?;

    Feed::update_last_fetched(&state.db, &feed.id).await?;

    Ok(Json(feed))
}

pub async fn get_one(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Path(feed_id): Path<String>,
) -> Result<Json<Feed>, AppError> {
    let feed = Feed::find_by_id(&state.db, &feed_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Feed not found".into()))?;

    if feed.user_id != user_id {
        return Err(AppError::NotFound("Feed not found".into()));
    }

    Ok(Json(feed))
}

pub async fn delete(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
    Path(feed_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let feed = Feed::find_by_id(&state.db, &feed_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Feed not found".into()))?;

    if feed.user_id != user_id {
        return Err(AppError::NotFound("Feed not found".into()));
    }

    Feed::delete(&state.db, &feed_id).await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}
