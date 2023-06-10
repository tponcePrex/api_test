//use actix_web::middleware::Logger;

mod api;
mod config;
mod user_data;
mod datatypes;
mod traits;
pub mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{

    api::api_start().await?;

    Ok(())
}
