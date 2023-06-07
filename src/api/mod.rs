use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use chrono::Utc;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_service)
        .service(alive);
}

#[get("/hello_service")]
pub(crate) async fn hello_service() -> impl Responder {
    //HttpResponse::Ok().body("Sup, dude").finish()
    let hello = String::from("Hello");
    hello.customize().with_status(StatusCode::OK)
}

#[get("/alive")]
pub(crate) async fn alive() -> HttpResponse {
    let curr_time = Utc::now().date_naive();
    HttpResponse::Ok().body(
        format!("Alive here man\nCurrent time: {curr_time}\nVersion: {VERSION}\nAuthor: {AUTHOR}")
    )
}