use actix_web::{get, HttpResponse, web};
use nanoid::nanoid;
use chrono::offset::Utc;
use serde::Serialize;
use wither::prelude::*;

use crate::db::models::{
  file::File
};
use crate::AppData;

#[derive(Serialize)]
struct Response {
  url: String
}

#[get("/files/upload_url")]
pub async fn req(data: web::Data<AppData>) -> HttpResponse {
  let generated_id: String = nanoid!(10);

  let mut empty_file: File = File{
    id: None,
    file_id: generated_id.clone(),
    name: String::from(""),
    author: String::from(""),
    upload_date: Utc::now().timestamp(),
    ready: false
  };
  empty_file.save(&data.db, None).await.expect("Failed to save a File object");

  HttpResponse::Ok()
    .json(Response {
      url: format!(
        "//{}:{}/files/{}/upload",
        data.config.bind_ip,
        data.config.bind_port,
        generated_id
      )
    })
}
