mod services {
    pub(crate) mod anime_search;
}
mod database {
    pub(crate) mod search_engine;
    pub(crate) mod mongo;
}
mod documents {
    pub(crate) mod anime_search;
}

use std::env;
use database::search_engine::Engine;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let db_uri = env::var("DB_URI")?;
    let db = Engine::new(db_uri).await?;

    Ok(())
}
