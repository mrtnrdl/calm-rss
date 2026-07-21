use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use feed_rs::parser;
use sqlx::SqlitePool;

pub struct ParsedFeed {
    pub title: String,
    pub site_url: Option<String>,
    pub entries: Vec<ParsedEntry>,
}

pub struct ParsedEntry {
    pub title: Option<String>,
    pub url: String,
    pub author: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub published_at: Option<NaiveDateTime>,
}

pub async fn fetch_feed(url: &str) -> Result<ParsedFeed> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("calm-rss/0.1")
        .build()?;

    let response = client
        .get(url)
        .send()
        .await
        .context("failed to fetch feed")?
        .error_for_status()
        .context("feed server returned error")?;

    let bytes = response.bytes().await.context("failed to read feed body")?;

    let feed = parser::parse(&bytes[..]).context("failed to parse feed")?;

    let title = feed
        .title
        .map(|t| t.content)
        .unwrap_or_else(|| url.to_string());

    let site_url = feed.links.first().map(|l| l.href.clone());

    let entries = feed
        .entries
        .into_iter()
        .filter_map(|entry| {
            let url = entry.links.first()?.href.clone();
            let title = entry.title.map(|t| t.content);
            let author = entry.authors.first().map(|a| a.name.clone());
            let published_at = entry.published.or(entry.updated).map(|dt| dt.naive_utc());
            let summary = entry.summary.map(|s| s.content);
            let content = entry.content.and_then(|c| c.body);

            Some(ParsedEntry {
                title,
                url,
                author,
                content,
                summary,
                published_at,
            })
        })
        .collect();

    Ok(ParsedFeed {
        title,
        site_url,
        entries,
    })
}

pub async fn store_articles(
    pool: &SqlitePool,
    feed_id: &str,
    entries: &[ParsedEntry],
) -> Result<usize> {
    let mut count = 0;

    for entry in entries {
        crate::models::article::Article::upsert(
            pool,
            crate::models::article::UpsertArticle {
                feed_id,
                title: entry.title.as_deref(),
                url: &entry.url,
                author: entry.author.as_deref(),
                content: entry.content.as_deref(),
                summary: entry.summary.as_deref(),
                published_at: entry.published_at.map(|dt| dt.to_string()).as_deref(),
            },
        )
        .await?;
        count += 1;
    }

    Ok(count)
}
