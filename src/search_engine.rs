pub(crate) mod mongo;

use std::{pin::Pin, error::Error as StdError};
use tokio_stream::Stream;
use crate::documents::anime_search::{Candidate, Rating};

pub(crate) type Engine = mongo::Mongo;

pub(crate) trait SearchEngine {
    async fn search(&self, keyword: &str, rating: Rating)
    -> anyhow::Result<
        Pin<Box<
            impl Stream<
                Item = Result<Candidate, Box<dyn StdError>>
            >
        >>
    >;
}
