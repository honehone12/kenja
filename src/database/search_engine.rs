use std::pin::Pin;
use mongodb::error::Error;
use tokio_stream::Stream;
use crate::documents::anime_search::{Candidate, Rating};

pub(crate) type Engine = super::mongo::MongoDb;

pub(crate) trait SearchEngine {
    async fn search(&self, keyword: String, rating: Rating)
    -> anyhow::Result<Pin<Box<impl Stream<Item = Result<Candidate, Error>>>>>;
}
