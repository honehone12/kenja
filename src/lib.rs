pub mod search_engines;
pub mod services {
    pub mod anime_search;

    pub(crate) const INTERNAL_ERROR: &'static str = "internal server error";
    pub(crate) const INVALID_ARGUMENT: &'static str = "invalid argument";
}
pub mod documents {
    pub mod anime_search;
}

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server as GrpcServer;
use services::anime_search::{
    anime_search_server::AnimeSearchServer, 
    AnimeSearchService
};
use search_engines::SearchEngine;

pub async fn server_main<EN>(engine: EN) 
-> anyhow::Result<()>
where EN: SearchEngine {
    let anime_search_service = AnimeSearchService::new(engine);
    let anime_search_server = AnimeSearchServer::new(anime_search_service);
    let serve_at = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 50051);

    GrpcServer::builder()
        .add_service(anime_search_server)
        .serve(serve_at).await?;

    Ok(())
}
