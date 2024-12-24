use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use once_cell::sync::Lazy;
use r2d2::{Pool, PooledConnection};

use crate::error::AppError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub static DB: Lazy<DbPool> = Lazy::new(|| {
    let database_url: String = crate::config::CONFIG.database_url.clone();

    println!("Connecting to database {}...", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.")
});

pub fn get_connection() -> Result<DbPooledConnection, AppError> {
    DB.get().map_err(|e| {
        AppError::new(500)
            .cause(e)
            .message("Failed to load database")
    })
}