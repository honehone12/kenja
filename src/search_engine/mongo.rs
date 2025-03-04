use std::pin::Pin;
use mongodb::{
    Client as MongoClient, 
    bson::doc
};
use tokio_stream::{Stream, StreamExt};
use tracing::debug;
use super::SearchEngine;
use crate::documents::anime_search::{Candidate, Rating};

const FORBIDDEN: [char; 8]  = ['$', '.', '{', '}', '[', ']', ':', ';'];

#[derive(Clone)]
pub(crate) struct Mongo {
    mongo_client: MongoClient   
}

impl Mongo {
    pub(crate) async fn new(mongo_uri: impl AsRef<str>) 
    -> anyhow::Result<Mongo> {
        let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
        let mongo_db = Self{mongo_client};
        Ok(mongo_db)
    }
}

#[async_trait::async_trait]
impl SearchEngine for Mongo {
    async fn search(&self, mut keyword: String, rating: Rating)
    -> anyhow::Result<
        Pin<Box<
            dyn Stream<Item = anyhow::Result<Candidate>> + Send + 'static
        >>,
    > {
        let collection = self.mongo_client
            .database("anime")
            .collection::<Candidate>(match rating {
                Rating::AllAges => "flat_ani_chara_all_ages",
                Rating::Hentai => "flat_ani_chara_hentai"
            });

        keyword.retain(|c| !FORBIDDEN.contains(&c));
        let keyword = keyword
            .escape_debug()
            .to_string()
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<String>>()
            .join(" ");
        debug!("search keyword: {keyword:?}");
        
        let candidates = collection.find(doc! {
            "$text": {"$search": keyword}
        }).await?;

        Ok(Box::pin(candidates.map(|r| r.map_err(|e| e.into()))))
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use tokio_stream::StreamExt;
    use crate::{
        documents::anime_search::Rating, 
        search_engine::SearchEngine
    };
    use super::Mongo;
    
    #[tokio::test]
    async fn test_search_engine() -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let mongo_uri = env::var("ENGINE_URI")?;
        let mongo = Mongo::new(mongo_uri).await?;
        let keyword = "school band music club".to_string();
        let mut stream = mongo.search(keyword, Rating::AllAges).await?;
        while let Some(candidate) = stream.try_next().await? {
            println!("{candidate:?}");
        }

        Ok(())
    }
}