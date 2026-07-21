use axum::Json;
use axum::extract::State;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::{AuthUser, Claims};
use crate::models::user::{AuthResponse, LoginInput, RegisterInput, User};

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterInput>,
) -> Result<Json<AuthResponse>, AppError> {
    if input.username.is_empty() || input.email.is_empty() || input.password.is_empty() {
        return Err(AppError::BadRequest("All fields are required".into()));
    }

    if input.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".into(),
        ));
    }

    if User::find_by_email(&state.db, &input.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Email already registered".into()));
    }

    let user = User::create(&state.db, input).await?;
    let token = Claims::encode(&state.config.jwt_secret, &user.id)
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginInput>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = User::verify_password(&state.db, &input.email, &input.password)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    let token = Claims::encode(&state.config.jwt_secret, &user.id)
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn me(
    AuthUser { user_id }: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = User::find_by_id(&state.db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(serde_json::json!(user)))
}
