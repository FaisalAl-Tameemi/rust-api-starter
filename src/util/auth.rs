
use actix::fut::{Ready, ready};
use actix_web::{
    dev::Payload,
    http::header::{self, HeaderMap},
    FromRequest, HttpRequest,
};

use crate::error::AppError;
use super::token::JwtToken;

pub struct AuthenticatedUser {
    pub decoded_token: JwtToken
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = match get_auth_header(req.headers()) {
            Ok(h) => h,
            Err(e) => return ready(Err(e)),
        };

        // Remove "Bearer " prefix
        let token = match auth_header.strip_prefix("Bearer ") {
            Some(token) => token,
            None => return ready(Err(AppError::new(401).message("Invalid token format"))),
        };

        let token_data = match JwtToken::decode(token, None) {
            Ok(data) => data,
            Err(_) => return ready(Err(AppError::new(401).message("Invalid token"))),
        };

        ready(Ok(AuthenticatedUser {
            decoded_token: token_data,
        }))
    }
}

fn get_auth_header(headers: &HeaderMap) -> Result<&str, AppError> {
    let header = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::new(401).message("Missing Authorization header"))?;

    header
        .to_str()
        .map_err(|_| AppError::new(401).message("Invalid Authorization header"))
}

