use mongodb::Cursor;
use tokio_stream::Stream;

pub(crate) type Engine = super::mongo::MongoDb;

pub(crate) trait SearchEngine {
}
