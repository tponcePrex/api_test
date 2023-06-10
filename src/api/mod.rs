use actix_web::{App, HttpServer, web};
use crate::config::environment::{EnvironmentConfig};

pub(crate) mod services;

pub(crate) async fn api_start() -> std::io::Result<()> {

    HttpServer::new(move || {
        //let logger = Logger::default();
        App::new()
            //.wrap(logger)
            .service(
                web::scope("/api")
                    .configure(services::config)
            )
    })
        .bind(EnvironmentConfig::instance().get_api_bind().await)?
        .run()
        .await?;

    Ok(())
}
