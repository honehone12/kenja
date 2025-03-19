use std::pin::Pin;
use mongodb::{
    Client as MongoClient, 
    bson::doc
};
use tokio_stream::{Stream, StreamExt};
use crate::{
    documents::anime_search::{Candidate, Rating},
    search_engines::SearchEngine
};
use super::{FORBIDDEN, SEARCH_DATABASE};

#[derive(Clone)]
pub struct Mongo {
    mongo_client: MongoClient   
}

impl Mongo {
    pub async fn new(mongo_uri: impl AsRef<str>) -> anyhow::Result<Self> {
        let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
        Ok(Self{mongo_client})
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
        keyword.retain(|c| !FORBIDDEN.contains(&c));
        let keyword = keyword
            .escape_debug()
            .to_string()
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<String>>()
            .join(" ");

        let collection = self.mongo_client
            .database(SEARCH_DATABASE)
            .collection::<Candidate>(&rating.to_string());
        
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
        search_engines::SearchEngine
    };
    use super::Mongo;
    
    #[allow(dead_code)]
    #[cfg_attr(not(feature = "atlas_test"), tokio::test)]
    async fn test_mongo_search() -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let mongo_uri = env::var("ENGINE_URI")?;
        let mongo = Mongo::new(mongo_uri).await?;
        let keyword =  String::from("school band music club");
        let mut stream = mongo.search(keyword, Rating::AllAges).await?;
        while let Some(candidate) = stream.try_next().await? {
            println!("{candidate:?}");
        }

        Ok(())
    }
}