//use actix_web::middleware::Logger;

mod api;
mod config;
mod looped;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{

    api::api_start().await?;

    Ok(())
}
