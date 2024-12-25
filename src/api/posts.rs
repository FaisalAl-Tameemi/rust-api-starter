use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use crate::error::AppError;
use crate::util::auth::AuthenticatedUser;

#[get("/posts")]
pub async fn list(requestor: AuthenticatedUser) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().json(json!({
        "message": "Hello, world!",
        "requestor": requestor.decoded_token,
    })))
}