mod search_engine;
mod services {
    pub(crate) mod anime_search;
}
mod documents {
    pub(crate) mod anime_search;
}

use std::env;
use search_engine::Engine;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let db_uri = env::var("DB_URI")?;
    let db = Engine::new(db_uri).await?;

    Ok(())
}
