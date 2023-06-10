use std::collections::HashMap;
use std::ops::Deref;
use actix_web::{get, HttpResponse, web};
use actix_web::http::{StatusCode};
use chrono::Local;
use crate::config::data::{EnvironmentConfig};
use crate::user_data::{QuestionsStatus, UserData, UserDataPatch};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_trivia_data)
        .service(alive)
        .service(get_user_data);
}

#[get("/trivia_data")]
pub(crate) async fn get_trivia_data() -> HttpResponse {

    let client = reqwest::Client::new();

    match client.get(EnvironmentConfig::instance().get_online_api_url().await)
        .header("X-Api-Key", EnvironmentConfig::instance().get_online_api_token().await)
        //rPmSGJ/l3zCkWdIskzBnmw==nxKmGehTFaPNuNfr
        .send()
        .await {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {},
                StatusCode::BAD_REQUEST => return HttpResponse::BadRequest().body("Bad request"),
                _ => return HttpResponse::BadGateway().body("Bad gateway")
            }
            match response.text().await {
                Ok(text) => HttpResponse::Ok().content_type("application/json").body(text),
                Err(e) => HttpResponse::BadGateway().body(format!("Bad gateway: {e}"))
            }
        },
        Err(e) => {
            HttpResponse::BadGateway().body(format!("Bad gateway: {e}"))
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

#[get("/user_data")]
async fn get_user_data() -> HttpResponse {

    //  TODO: remove this and implement the edit user endpoint
    let questions_status = QuestionsStatus::new(
        Some(String::from("How big is a tiger's butt?")),
        Some(false),
        Some(String::from("Very")),
        Some(String::from("Not so much"))
    );
    let mut questions_hash = HashMap::new();
    questions_hash.insert(1, questions_status);

    let new_user_data = UserDataPatch::new(
        69,
        String::from("johnnyboy_69"),
        questions_hash,
        420
    );

    if let Err(e) = UserData::update_user(new_user_data).await {
        return HttpResponse::BadGateway().body(format!("Bad gateway: {e}"))
        //return HttpResponse::with_body(StatusCode::BAD_GATEWAY, format!("Bad gateway: {e}"))
    }

    let user_data = UserData::get_inner().write().await;

    match serde_json::to_string(user_data.deref()) {
        Ok(user_data) => HttpResponse::Ok().content_type("application/json").body(user_data),
        Err(e) => HttpResponse::BadGateway().body(format!("Bad gateway: {e}"))
    }
}
