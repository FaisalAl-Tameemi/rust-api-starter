use actix_web::{middleware, App, HttpServer};

use rust_api_starter::config::CONFIG;
use rust_api_starter::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(server::initialize)
            .wrap(middleware::Logger::default())
    })
    .bind((CONFIG.host.clone(), CONFIG.port))?
    .run()
    .await
}

