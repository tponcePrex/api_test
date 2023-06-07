use actix_web::{App, HttpServer, web};
//use actix_web::middleware::Logger;

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{

    HttpServer::new(move || {
        //let logger = Logger::default();
        App::new()
            //.wrap(logger)
            .service(
                web::scope("/api")
                    .configure(api::config)
            )
    })
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}
