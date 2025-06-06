use std::pin::Pin;
use mongodb::{
    bson::doc,
    Client as MongoClient
};
use tokio_stream::{Stream, StreamExt};
use crate::{
    documents::anime_search::{Candidate, Rating},
    search_engines::SearchEngine
};
use super::FORBIDDEN;

#[derive(Clone)]
pub struct Atlas {
    mongo_client: MongoClient,
    db_name: String
}

impl Atlas {
    pub async fn new(mongo_uri: impl AsRef<str>, db_name: String) -> anyhow::Result<Self> {
        let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
        Ok(Self{mongo_client, db_name})
    }
}

#[async_trait::async_trait]
impl SearchEngine for Atlas {
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
            .collect::<Vec<&str>>()
            .join(" ");

        let collection = self.mongo_client
            .database(&self.db_name)
            .collection::<Candidate>(&rating.to_string());

        let candidates = collection.aggregate(vec![
            doc! {
                "$search": doc! {
                    "index": "default",
                    "text": doc! {
                        "query": keyword,
                        "path": vec![
                            "name", 
                            "name_english", 
                            "aliases",
                            "description",
                            "parent.name"
                        ],
                        "matchCriteria": "all"
                    }
                }
            }
        ])
            .with_type::<Candidate>().await?;
        
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
    use super::Atlas;

    #[allow(dead_code)]
    #[cfg_attr(feature = "atlas_test", tokio::test)]
    async fn test_atlas_search() -> anyhow::Result<()> {
        dotenvy::dotenv()?;

        let mongo_uri = env::var("ENGINE_URI")?;
        let db_name = env::var("SEARCH_DATABASE")?;
        let mongo = Atlas::new(mongo_uri, db_name).await?;
        let keyword =  String::from("school band music club");
        let mut stream = mongo.search(keyword, Rating::AllAges).await?;
        while let Some(candidate) = stream.try_next().await? {
            println!("{candidate:?}");
        }

        Ok(())
    }
}