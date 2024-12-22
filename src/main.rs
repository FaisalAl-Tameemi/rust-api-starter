use actix_web::{web, App, HttpServer, get};

use rust_api_starter::config::CONFIG;

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    format!("Hello {} from {:?}!", data.app_name, CONFIG.host)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: CONFIG.app_name.clone(),
            }))
            .service(
                web::scope("/api")
                    .service(index),
            )
    })
    .bind((CONFIG.host.clone(), CONFIG.port))?
    .run()
    .await
}

