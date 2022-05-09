use std::fs;
use toml;
use serde_derive::Deserialize;
use actix_web::{App, HttpServer, web::Data};
use wither::mongodb::Database;

mod routes;
mod utils;
mod db;

use utils::logger;

#[derive(Deserialize, Clone)]
pub struct Config {
  mongo_url: String,
  mongo_database: String,
  bind_ip: String,
  bind_port: u16
}

pub struct AppData {
  db: Database,
  config: Config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config: Config = toml::from_str(
    fs::read_to_string("./config.toml").expect("No config file found").as_str()
  ).expect("Failed to parse the config file");

  logger::info("Connecting to the MongoDB database");

  let database: Database = db::init(config.clone()).await;

  logger::info("Starting the web server");
  logger::info(format!("Binded to {}:{}", config.bind_ip, config.bind_port.to_string()).as_str());

  let data_config: Config = config.clone();

  HttpServer::new(move || {
    App::new()
      .app_data(
        Data::new(
          AppData {
            db: database.clone(),
            config: data_config.clone()
          }
        )
      )
      .service(routes::root::ping)
      .service(routes::files::upload_url::req)
  })
    .bind((config.bind_ip, config.bind_port))?
    .run()
    .await
}
