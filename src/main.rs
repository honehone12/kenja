use std::env;
use kenja::{
    services::anime_search::{
        anime_search_server::AnimeSearchServer, 
        AnimeSearchService
    },
    search_engines::mongo::Mongo
};
use tonic::transport::Server as GrpcServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    
    let serve_at = env::var("SERVE_AT")?.parse()?;

    let engine_uri = env::var("ENGINE_URI")?;
    let engine = Mongo::new(engine_uri).await?;
    let anime_search_service = AnimeSearchService::new(engine);
    let anime_search_server = AnimeSearchServer::new(anime_search_service);

    GrpcServer::builder()
        .add_service(anime_search_server)
        .serve(serve_at).await?;

    Ok(())
}
