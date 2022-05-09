use serde::{Serialize, Deserialize};
use wither::{Model, bson::{doc, oid::ObjectId}};

#[derive(Debug, Model, Serialize, Deserialize)]
#[model(
  index(
    keys=r#"doc!{
      "id": 1,
      "file_id": 2,
      "name": 3,
      "author": 4,
      "upload_date": 5,
      "ready": 6
    }"#,
    options=r#"doc!{"unique": true}"#
  )
)]
pub struct File {
  #[serde(rename="_id", skip_serializing_if="Option::is_none")]
  pub id: Option<ObjectId>,

  pub file_id: String,
  pub name: String,
  pub author: String,
  pub upload_date: i64,
  pub ready: bool
}
