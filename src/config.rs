use config::Config;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub host: String,
    pub app_name: String,
    pub jwt_secret: String,
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    dotenv().ok();

    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    config.try_deserialize().unwrap()
});