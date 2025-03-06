use mongodb::Client as MongoClient;

#[derive(Clone)]
pub struct Atlas {
    mongo_client: MongoClient
}
