use wither::mongodb::Database;
use wither::prelude::*;

pub mod file;

use file::File;

pub async fn init(db: Database) {
  File::sync(&db).await.expect("Failed to sync model File");
}
