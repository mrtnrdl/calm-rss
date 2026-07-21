use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    username: String,
    email: String,
    password_hash: String,
    created_at: NaiveDateTime,
}

impl User {
    pub async fn create(pool: &SqlitePool, input: RegisterInput) -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let password_hash = hash(&input.password, DEFAULT_COST)?;

        let user = sqlx::query_as::<_, Self>(
            r#"INSERT INTO users (id, username, email, password_hash)
               VALUES (?, ?, ?, ?)
               RETURNING id, username, email, created_at"#,
        )
        .bind(&id)
        .bind(&input.username)
        .bind(&input.email)
        .bind(&password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, Self>(
            "SELECT id, username, email, created_at FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as::<_, Self>(
            "SELECT id, username, email, created_at FROM users WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn verify_password(
        pool: &SqlitePool,
        email: &str,
        password: &str,
    ) -> Result<Option<Self>> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) if verify(password, &row.password_hash)? => Ok(Some(User {
                id: row.id,
                username: row.username,
                email: row.email,
                created_at: row.created_at,
            })),
            _ => Ok(None),
        }
    }
}
