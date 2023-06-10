use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use chrono::Local;
use crate::config::data::{EnvironmentConfig};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_trivia_data)
        .service(alive);
}

#[get("/trivia_data")]
pub(crate) async fn get_trivia_data() -> impl Responder {
    //HttpResponse::Ok().body("Sup, dude").finish()

    let client = reqwest::Client::new();

    match client.get(EnvironmentConfig::instance().get_online_api_url().await)
        .header("X-Api-Key", EnvironmentConfig::instance().get_online_api_token().await)
        //rPmSGJ/l3zCkWdIskzBnmw==nxKmGehTFaPNuNfr
        .send()
        .await {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                HttpResponse::with_body(StatusCode::OK, text)
            } else {
                HttpResponse::with_body(StatusCode::BAD_GATEWAY, String::from("Bad gateway"))
            }
        },
        Err(_) => {
            HttpResponse::with_body(StatusCode::BAD_GATEWAY, String::from("Bad gateway"))
        }
    }
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