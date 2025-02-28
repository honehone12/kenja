pub(crate) mod mongo;

use std::pin::Pin;
use tokio_stream::Stream;
use crate::documents::anime_search::{Candidate, Rating};

pub(crate) type Engine = mongo::Mongo;

#[async_trait::async_trait]
pub(crate) trait SearchEngine: Clone + Send + Sync + 'static {
    async fn search(self, keyword: String, rating: Rating)
    -> anyhow::Result<
        Pin<Box<
            dyn Stream<Item = anyhow::Result<Candidate>> + Send + 'static
        >>,
    >;
}
