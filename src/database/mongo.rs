use mongodb::Client as MongoClient;
use crate::{
    database::search_engine::SearchEngine,
    documents::anime_search::{Candidate, Rating}
};

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

    pub(crate) async fn search(&self, keyword: String, rating: Rating) {
        let collection_name = match rating {
            Rating::AllAges => "anime_text_all_ages",
            Rating::Hentai => "anime_text_hentai"
        };
        let collection = self.mongo_client
            .database("anime")
            .collection::<Candidate>(&collection_name);
    }
}

impl SearchEngine for MongoDb {
    
}