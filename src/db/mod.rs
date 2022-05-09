use wither::mongodb::{Client, Database};

use crate::Config;

pub mod models;

pub async fn init(config: Config) -> Database {
  let db: Database = Client::with_uri_str(config.mongo_url.as_str())
    .await
    .expect("Failed to connect to the MongoDB database")
    .database(config.mongo_database.as_str());
  
  models::init(db.clone()).await;

  db
}
