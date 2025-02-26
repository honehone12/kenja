use mongodb::Client as MongoClient;
use super::search_engine::SearchEngine;

#[derive(Clone)]
pub(crate) struct MongoDb {
    mongo_client: MongoClient    
}

impl MongoDb {
    pub(crate) async fn new(mongo_uri: impl AsRef<str>) 
    -> anyhow::Result<MongoDb> {
        let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
        let mongo_db = Self{
            mongo_client
        };
        Ok(mongo_db)
    }
}

impl SearchEngine for MongoDb {
    
}