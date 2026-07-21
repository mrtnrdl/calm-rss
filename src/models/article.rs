use anyhow::Result;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Article {
    pub id: String,
    pub feed_id: String,
    pub title: Option<String>,
    pub url: String,
    pub author: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub is_read: bool,
    pub is_starred: bool,
    pub created_at: NaiveDateTime,
}

pub struct UpsertArticle<'a> {
    pub feed_id: &'a str,
    pub title: Option<&'a str>,
    pub url: &'a str,
    pub author: Option<&'a str>,
    pub content: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub published_at: Option<&'a str>,
}

impl Article {
    pub async fn upsert(pool: &SqlitePool, input: UpsertArticle<'_>) -> Result<Self> {
        let id = Uuid::new_v4().to_string();

        let article = sqlx::query_as::<_, Self>(
            r#"INSERT INTO articles (id, feed_id, title, url, author, content, summary, published_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT(feed_id, url) DO UPDATE SET
                 title = excluded.title,
                 author = excluded.author,
                 content = excluded.content,
                 summary = excluded.summary,
                 published_at = excluded.published_at
               RETURNING id, feed_id, title, url, author, content, summary,
                         published_at, is_read, is_starred, created_at"#,
        )
        .bind(&id)
        .bind(input.feed_id)
        .bind(input.title)
        .bind(input.url)
        .bind(input.author)
        .bind(input.content)
        .bind(input.summary)
        .bind(input.published_at)
        .fetch_one(pool)
        .await?;

        Ok(article)
    }

    pub async fn find_by_feed(pool: &SqlitePool, feed_id: &str) -> Result<Vec<Self>> {
        let articles = sqlx::query_as::<_, Self>(
            "SELECT id, feed_id, title, url, author, content, summary, \
             published_at, is_read, is_starred, created_at \
             FROM articles WHERE feed_id = ? \
             ORDER BY published_at DESC NULLS LAST, created_at DESC",
        )
        .bind(feed_id)
        .fetch_all(pool)
        .await?;

        Ok(articles)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>> {
        let article = sqlx::query_as::<_, Self>(
            "SELECT id, feed_id, title, url, author, content, summary, \
             published_at, is_read, is_starred, created_at \
             FROM articles WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(article)
    }

    pub async fn toggle_read(pool: &SqlitePool, id: &str) -> Result<Option<Self>> {
        sqlx::query("UPDATE articles SET is_read = NOT is_read WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Self::find_by_id(pool, id).await
    }

    pub async fn toggle_starred(pool: &SqlitePool, id: &str) -> Result<Option<Self>> {
        sqlx::query("UPDATE articles SET is_starred = NOT is_starred WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Self::find_by_id(pool, id).await
    }
}
