use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};
use crate::search_engines::SearchEngine;
use super::display_messages::{INTERNAL_ERROR, INVALID_ARGUMENT};
use tracing::error;

const FORBIDDEN: [char; 8]  = ['$', '.', '{', '}', '[', ']', ':', ';'];
const MAX_KEYWORD: usize = 50;

tonic::include_proto!("kenja_anime_search");

pub struct AnimeSearchService<EN: SearchEngine> {
    engine: EN
}

impl<EN: SearchEngine> AnimeSearchService<EN> {
    pub fn new(engine: EN) -> Self {
        Self{engine}
    }
} 

#[tonic::async_trait]
impl<EN: SearchEngine> anime_search_server::AnimeSearch for AnimeSearchService<EN> {
    type SearchStream = Pin<Box<
        dyn Stream<
            Item = Result<Candidate, Status>
        > + Send + 'static
    >>;

    async fn search(&self, req: Request<Query>)
    -> Result<Response<Self::SearchStream>, Status> {
        let query = req.into_inner();
        if query.keyword.len() >= MAX_KEYWORD {
            return Err(Status::invalid_argument(INVALID_ARGUMENT))
        }
        let rating = query.rating().into();
        let mut keyword = query.keyword;
        keyword.retain(|c| !FORBIDDEN.contains(&c));
        let keyword = keyword
            .escape_debug()
            .to_string()
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<String>>()
            .join(" ");

        let engine = self.engine.clone();

        let stream = match engine.search(&keyword, rating).await {
            Ok(s) => {
                s.map(|r| {
                    match r {
                        Ok(c) => Ok(c.into()),
                        Err(e) => {
                            error!("{e}");
                            Err(Status::internal(INTERNAL_ERROR)) 
                        }
                    }
                })
            }
            Err(e) => {
                error!("{e}");
                return Err(Status::internal(INTERNAL_ERROR));
            }
        };

        Ok(Response::new(Box::pin(stream)))
    }
}

#[cfg(test)]
mod test {
    use std::{env, time::Duration};
    use tokio_stream::StreamExt;
    use tonic::{transport::Server, Request};
    use crate::search_engines::mongo::Mongo;
    use super::{
        anime_search_server::AnimeSearchServer, 
        AnimeSearchService, Query, Rating,
        anime_search_client::AnimeSearchClient 
    };

    #[tokio::test]
    async fn test_search_service() -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let serve_at = env::var("SERVE_AT")?.parse()?;
        let connect_to = format!("http://{serve_at}");

        let handle = tokio::spawn(async move {
            let engine_uri = env::var("ENGINE_URI").unwrap();
            let engine = Mongo::new(engine_uri).await.unwrap();
            let anime_search_service = AnimeSearchService::new(engine);
            let anime_search_server = AnimeSearchServer::new(anime_search_service);

            Server::builder()
                .add_service(anime_search_server)
                .serve(serve_at).await
                .unwrap();
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut client = AnimeSearchClient::connect(connect_to).await?;
        
        let res = client.search(Request::new(Query{ 
            keyword: "school music band club".to_string(), 
            rating: Rating::AllAges.into() 
        })).await?;

        let mut stream = res.into_inner();
        while let Some(c) = stream.try_next().await? {
            println!("{c:?}")
        }

        handle.abort();
        Ok(())
    }
}