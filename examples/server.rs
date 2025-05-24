use std::{env, net::{IpAddr, Ipv4Addr}};
use kenja::search_engines::mongodb::{atlas::Atlas, mongo::Mongo};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    if cfg!(feature = "atlas_test") {
        let engine_uri = env::var("ENGINE_URI")?;
        let engine = Atlas::new(engine_uri).await?;
        kenja::server_main(engine, IpAddr::V4(Ipv4Addr::LOCALHOST)).await
    } else {
        let engine_uri = env::var("DEV_ENGINE_URI")?;
        let engine = Mongo::new(engine_uri).await?;
        kenja::server_main(engine, IpAddr::V4(Ipv4Addr::LOCALHOST)).await
    }
}
