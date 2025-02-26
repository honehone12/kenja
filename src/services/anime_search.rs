tonic::include_proto!("kenja_anime_search");

use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use crate::{
    database::search_engine::Engine,
    services::anime_search::anime_search_server::AnimeSearch
};

type SearchStreamItem = Result<Candidate, Status>;

pub(crate) struct AnimeSearchService {
    db: Engine
}

// #[tonic::async_trait]
// impl AnimeSearch for AnimeSearchService {
    
//     type SearchStream = Pin<Box<dyn Stream<Item = SearchStreamItem> + Send + 'static>>;

//     async fn search(&self, req: Request<Keyword>)
//     -> Result<Response<Self::SearchStream>, Status> {

//         Ok(())
//     }
// }
