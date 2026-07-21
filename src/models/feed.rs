use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Feed {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub url: String,
    pub site_url: Option<String>,
    pub last_fetched_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateFeedInput {
    pub url: String,
}

impl Feed {
    pub async fn create(
        pool: &SqlitePool,
        user_id: &str,
        title: &str,
        url: &str,
        site_url: Option<&str>,
    ) -> Result<Self> {
        let id = Uuid::new_v4().to_string();

        let feed = sqlx::query_as::<_, Self>(
            r#"INSERT INTO feeds (id, user_id, title, url, site_url)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id, user_id, title, url, site_url, last_fetched_at, created_at"#,
        )
        .bind(&id)
        .bind(user_id)
        .bind(title)
        .bind(url)
        .bind(site_url)
        .fetch_one(pool)
        .await?;

        Ok(feed)
    }

    pub async fn find_by_user(pool: &SqlitePool, user_id: &str) -> Result<Vec<Self>> {
        let feeds = sqlx::query_as::<_, Self>(
            "SELECT id, user_id, title, url, site_url, last_fetched_at, created_at \
             FROM feeds WHERE user_id = ? ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(feeds)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>> {
        let feed = sqlx::query_as::<_, Self>(
            "SELECT id, user_id, title, url, site_url, last_fetched_at, created_at \
             FROM feeds WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(feed)
    }

    pub async fn find_by_url_for_user(
        pool: &SqlitePool,
        user_id: &str,
        url: &str,
    ) -> Result<Option<Self>> {
        let feed = sqlx::query_as::<_, Self>(
            "SELECT id, user_id, title, url, site_url, last_fetched_at, created_at \
             FROM feeds WHERE user_id = ? AND url = ?",
        )
        .bind(user_id)
        .bind(url)
        .fetch_optional(pool)
        .await?;

        Ok(feed)
    }

    pub async fn update_last_fetched(pool: &SqlitePool, id: &str) -> Result<()> {
        sqlx::query("UPDATE feeds SET last_fetched_at = datetime('now') WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM feeds WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
