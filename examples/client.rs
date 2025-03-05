use std::env;
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
        #[arg(long, default_value_t = true)]
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
    
    while let Some(c) = stream.try_next().await? {
        info!("{c:?}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let addr = env::var("SERVER_AT")?;
    let connect_to = format!("http://{addr}");

    let cli = Cli::parse();

    match cli.command {
        Command::Search { keyword, hentai } => {
            search(
                connect_to.to_string(), 
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
