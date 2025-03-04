use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};
use crate::{
    documents::anime_search::Rating as RatingQuery,
    search_engine::SearchEngine
};
use super::display_messages::INTERNAL_ERROR;
use tracing::error;

tonic::include_proto!("kenja_anime_search");

pub(crate) struct AnimeSearchService<EN: SearchEngine> {
    engine: EN
}

impl<EN: SearchEngine> AnimeSearchService<EN> {
    pub(crate) fn new(engine: EN) -> Self {
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

    async fn search(&self, req: Request<Keyword>)
    -> Result<Response<Self::SearchStream>, Status> {
        let query = req.into_inner();
        let rating = RatingQuery::from(query.rating());
        let keyword = query.keyword;
        let engine = self.engine.clone();

        let stream = match engine.search(keyword, rating).await {
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
    use std::env;
    use tonic::{transport::Server, Request};
    use crate::{services::anime_search::{anime_search_client::AnimeSearchClient, Keyword, Rating}, Mongo};
    use super::{anime_search_server::AnimeSearchServer, AnimeSearchService};

    #[tokio::test]
    async fn test_search_servide() -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let serve_at = env::var("SERVE_AT")?;
        let addr = serve_at.parse()?;

        let engine_uri = env::var("ENGINE_URI")?;
        let engine = Mongo::new(engine_uri).await?;
        let anime_search_service = AnimeSearchService::new(engine);
        let anime_search_server = AnimeSearchServer::new(anime_search_service);

        let server = Server::builder()
            .add_service(anime_search_server)
            .serve(addr);

        let mut client = AnimeSearchClient::connect(serve_at).await?;
        
        client.search(Request::new(Keyword{ 
            keyword: "school music band club".to_string(), 
            rating: Rating::AllAges.into() 
        }));

        server.await?;
        Ok(())
    }
}