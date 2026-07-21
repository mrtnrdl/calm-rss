use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn new(user_id: &str) -> Self {
        let exp = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        Self {
            sub: user_id.to_string(),
            exp,
        }
    }

    pub fn encode(secret: &str, user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Self::new(user_id);
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn decode(secret: &str, token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_required_spec_claims(&["exp"]);

        decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Config: axum::extract::FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config: Config = axum::extract::FromRef::from_ref(state);

        let header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims =
            Claims::decode(&config.jwt_secret, token).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
