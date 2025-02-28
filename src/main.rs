mod search_engine;
mod services {
    pub(crate) mod anime_search;
    pub(crate) mod messages;
}
mod documents {
    pub(crate) mod anime_search;
}

use std::env;
use services::anime_search::{anime_search_server::AnimeSearchServer, AnimeSearchService};
use search_engine::mongo::Mongo;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    
    let serve_at = env::var("SERVE_AT")?.parse()?;

    let engine_uri = env::var("ENGINE_URI")?;
    let engine = Mongo::new(engine_uri).await?;
    let anime_search_service = AnimeSearchService::new(engine);
    let anime_search_server = AnimeSearchServer::new(anime_search_service);

    Server::builder()
        .add_service(anime_search_server)
        .serve(serve_at).await?;

    Ok(())
}
