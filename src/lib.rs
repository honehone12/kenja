pub mod search_engines;
pub mod services {
    pub mod anime_search;

    pub(crate) const INTERNAL_ERROR: &'static str = "internal server error";
    pub(crate) const INVALID_ARGUMENT: &'static str = "invalid argument";
}
pub mod documents {
    pub mod anime_search;
}

use std::{env, net::{IpAddr, Ipv4Addr, SocketAddr}};
use tonic::transport::Server as GrpcServer;
use services::anime_search::{
    anime_search_server::AnimeSearchServer, 
    AnimeSearchService
};
use search_engines::mongodb::atlas::Atlas;

pub async fn process_main() -> anyhow::Result<()> {
    let engine_uri = env::var("ENGINE_URI")?;
    let engine = Atlas::new(engine_uri).await?;
    let anime_search_service = AnimeSearchService::new(engine);
    let anime_search_server = AnimeSearchServer::new(anime_search_service);
    let serve_at = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);

    GrpcServer::builder()
        .add_service(anime_search_server)
        .serve(serve_at).await?;

    Ok(())
}
