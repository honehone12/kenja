pub mod mongo;

use std::pin::Pin;
use tokio_stream::Stream;
use crate::documents::anime_search::{Candidate, Rating};

#[async_trait::async_trait]
pub trait SearchEngine: Clone + Send + Sync + 'static {
    async fn search(&self, keyword: &str, rating: Rating)
    -> anyhow::Result<
        Pin<Box<
            dyn Stream<Item = anyhow::Result<Candidate>> + Send + 'static
        >>,
    >;
}
