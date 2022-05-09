use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn ping() -> HttpResponse {
  HttpResponse::Ok()
    .body("alive!")
}
