use clap::{Parser, Subcommand};
use kenja::services::anime_search::{
    anime_search_client::AnimeSearchClient, 
    Query, Rating
};
use tokio_stream::StreamExt;
use tonic::Request;
use tracing::info;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Search {
        keyword: String,
        #[arg(long, default_value_t = false)]
        hentai: bool 
    }
}

async fn search(
    connect_to: String, 
    keyword: String,
    rating: Rating
) -> anyhow::Result<()> {
    let mut client = AnimeSearchClient::connect(connect_to).await?;
    let query = Query{
        keyword,
        rating: rating.into()
    };
    let res = client.search(Request::new(query)).await?;
    let mut stream = res.into_inner();
    
    let mut count = 0u64;
    while let Some(c) = stream.try_next().await? {
        info!("{c:?}");
        count += 1;
    }

    info!("received {count}items");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let cli = Cli::parse();
    const ADDR: &str = "http://localhost:50051";

    match cli.command {
        Command::Search { keyword, hentai } => {
            search(
                ADDR.to_string(), 
                keyword, 
                match hentai {
                    false => Rating::AllAges,
                    true => Rating::Hentai
                }
            ).await?;   
        }
    }

    Ok(())
}
