use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use calm_rss::config::Config;
use calm_rss::db;
use calm_rss::{AppState, router};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    let pool = db::create_pool(&config.database_url).await?;

    let state = AppState {
        db: pool,
        config: config.clone(),
    };

    let app = router(state);
    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;

    tracing::info!("listening on {}", config.listen_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
