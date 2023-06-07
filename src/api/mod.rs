use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use chrono::Local;

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
    let curr_datetime = Local::now();
    HttpResponse::Ok().body(
        format!(
            "Alive here man\n\
            Current date: {}\n\
            Current time: {}\n\
            Version: {VERSION}\n\
            Author: {AUTHOR}",
            curr_datetime.date_naive(),
            curr_datetime.time().format("%H:%M:%S"),
        )
    )
}