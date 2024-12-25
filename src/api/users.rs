use bcrypt::{verify, hash, DEFAULT_COST};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web_validator::{Json, Path};
use chrono::{Duration, Utc};
use diesel::{query_dsl::methods::{FilterDsl, FindDsl}, ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{error::AppError, models::user::{LoginUser, NewUser, User}, schema, util::{db, token::JwtToken}};
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
    let mut conn = db::get_connection()?;
    let mut user = user.into_inner();
    user.password = hash(&user.password, DEFAULT_COST).unwrap();
    
    let results: User = diesel::insert_into(users::table)
        .values(user)
        .returning(users::all_columns)
        .get_result(&mut conn)
        .map_err(|_| AppError::new(500).message("Failed to create user"))?;

    let token_exp = Utc::now() + Duration::weeks(4);
    let token = JwtToken::new(results.id.to_string(), token_exp.timestamp() as usize, None);

    Ok(HttpResponse::Ok().json(json!({
        "id": results.id,
        "username": results.username,
        "token": token.encode(None)?,
    })))
}

#[post("/users/login")]
pub async fn login(data: Json<LoginUser>) -> Result<impl Responder, AppError> {
    let mut conn = db::get_connection()?;
    let user = users::table
        .filter(users::username.eq(&data.username))
        .first::<User>(&mut conn)
        .map_err(|_| AppError::new(404).message("User not found"))?;

    let is_valid = verify(&data.password, &user.password)
        .map_err(|_| AppError::new(500).message("Failed to verify password"))?;
    
    if !is_valid {
        return Err(AppError::new(401).message("Invalid password")); 
    }

    let token_exp = Utc::now() + Duration::weeks(4);
    let token = JwtToken::new(user.id.to_string(), token_exp.timestamp() as usize, None);

    Ok(HttpResponse::Ok().json(json!({
        "id": user.id,
        "username": user.username,
        "token": token.encode(None)?,
    })))
}
