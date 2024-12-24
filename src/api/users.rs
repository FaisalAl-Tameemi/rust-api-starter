use actix_web::{get, HttpResponse, Responder, post};
use actix_web_validator::{Json, Path};
use diesel::{query_dsl::methods::FindDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::AppError, models::user::{NewUser, User}, schema, util::db};
use schema::*;

#[get("/users")]
pub async fn list() -> Result<impl Responder, AppError> {
    let mut conn = db::get_connection()?;
    let users = users::table.load::<User>(&mut conn)
        .map_err(|_| AppError::new(500).message("Failed to fetch users"))?;

    Ok(HttpResponse::Ok().json(users))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetUserParams {
    #[validate(range(min = 1, max = 100))]
    id: i32,
}

#[get("/users/{id}")]
pub async fn get(params: Path<GetUserParams>) -> Result<impl Responder, AppError> {
    let mut conn = db::get_connection()?;
    let user = users::table.find(params.id).first::<User>(&mut conn)
        .map_err(|_| AppError::new(404).message("User not found"))?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
pub async fn create(user: Json<NewUser>) -> Result<impl Responder, AppError> {
    println!("{:?}", user);
    let mut conn = db::get_connection()?;
    
    let results = diesel::insert_into(users::table).values(user.into_inner()).execute(&mut conn)
        .map_err(|_| AppError::new(500).message("Failed to create user"))?;

    Ok(HttpResponse::Ok().json(results))
}