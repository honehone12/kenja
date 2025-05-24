use std::{env, net::{IpAddr, Ipv4Addr}};
use kenja::search_engines::mongodb::atlas::Atlas;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    
    let engine_uri = env::var("ENGINE_URI")?;
    let engine = Atlas::new(engine_uri).await?;

    kenja::server_main(engine, IpAddr::V4(Ipv4Addr::UNSPECIFIED)).await
}
