mod search_engine;
mod services {
    pub(crate) mod anime_search;
}
mod documents {
    pub(crate) mod anime_search;
}

use std::env;
use search_engine::Engine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    
    let engine_uri = env::var("ENGINE_URI")?;
    let engine = Engine::new(engine_uri).await?;

    Ok(())
}
