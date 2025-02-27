use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use crate::search_engine::Engine;

tonic::include_proto!("kenja_anime_search");

pub(crate) struct AnimeSearchService {
    db: Engine
}

// #[tonic::async_trait]
// impl anime_search_server::AnimeSearch for AnimeSearchService {
//     type SearchStream = Pin<Box<
//         impl Stream<
//             Item = Result<Candidate, Status>
//         > + Send + 'static
//     >>;

//     async fn search(&self, req: Request<Keyword>)
//     -> Result<Response<Self::SearchStream>, Status> {

//         Ok(())
//     }
// }
