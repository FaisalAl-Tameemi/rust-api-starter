use actix_web::{get, web};
use actix_web_validator::{JsonConfig, PathConfig, QueryConfig};
use crate::{api::users, config::CONFIG, util::error::{handle_error, json_handle_error}};

pub struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    format!("Hello {} from {:?}!", data.app_name, CONFIG.host)
}

pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.app_data(PathConfig::default().error_handler(handle_error));
    cfg.app_data(QueryConfig::default().error_handler(handle_error));
    cfg.app_data(JsonConfig::default().error_handler(json_handle_error));
    cfg.app_data(web::Data::new(AppState {
        app_name: CONFIG.app_name.clone()
    }));
    cfg.service(index);
    cfg.service((users::list, users::get, users::create, users::login));
}
