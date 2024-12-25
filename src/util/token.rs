use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub sub: String,
    pub exp: usize,
    pub meta: Option<JwtTokenMeta>,
    pub iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtTokenMeta {
    pub company: String,
}

impl JwtToken {
    pub fn new(sub: String, exp: usize, meta: Option<JwtTokenMeta>) -> Self {
        Self { sub, exp, meta, iat: 0 }
    }

    pub fn encode(&self, secret: Option<&str>) -> Result<String, JwtError> {
        let secret = secret.unwrap_or(CONFIG.jwt_secret.as_str());
        let key = EncodingKey::from_secret(secret.as_bytes());
        encode(&Header::default(), &self, &key)
    }

    pub fn decode(token: &str, secret: Option<&str>) -> Result<Self, JwtError> {
        let secret = secret.unwrap_or(CONFIG.jwt_secret.as_str());
        let key = DecodingKey::from_secret(secret.as_bytes());
        decode::<Self>(token, &key, &Validation::default())
            .map(|token| token.claims)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, Duration};

    use super::*;

    #[test]
    fn test_encode_decode() {
        let expiry = Utc::now() + Duration::weeks(4);
        let token = JwtToken::new("123".to_string(), expiry.timestamp() as usize, None);
        let encoded = token.encode(None).unwrap();
        let decoded = JwtToken::decode(&encoded, None).unwrap();
        assert_eq!(decoded.sub, "123");
    }

    #[test]
    fn test_encode_decode_with_meta() {
        let expiry = Utc::now() + Duration::weeks(4);
        let meta = JwtTokenMeta { company: "company".to_string() };
        let token = JwtToken::new("123".to_string(), expiry.timestamp() as usize, Some(meta));
        let encoded = token.encode(None).unwrap();
        let decoded = JwtToken::decode(&encoded, None).unwrap();
        assert_eq!(decoded.sub, "123");
        assert_eq!(decoded.meta.unwrap().company, "company");
    }
}