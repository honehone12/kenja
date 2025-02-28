use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};
use crate::{
    documents::anime_search::Rating as RatingQuery,
    search_engine::SearchEngine
};
use super::messages::INTERNAL_ERROR;
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
        let rating = RatingQuery::from_msg(query.rating());
        let keyword = query.keyword;
        let engine = self.engine.clone();

        let stream = match engine.search(keyword, rating).await {
            Ok(s) => {
                s.map(|r| {
                    match r {
                        Ok(c) => Ok(c.into_msg()),
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
    #[tokio::test]
    async fn test_search_servide() -> anyhow::Result<()> {
        Ok(())
    }
}