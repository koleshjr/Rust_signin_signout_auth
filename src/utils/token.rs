use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{ErrorMessage, HttpError};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_token (
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
)-> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());

    }

    let iat = Utc::now().timestamp_nanos() as usize;
    let exp = (Utc::now() + Duration::seconds(expires_in_seconds)).timestamp_nanos() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_owned(),
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
    let decoded = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;
    match decoded {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(_) => Err(HttpError::new(
            401,
            ErrorMessage::InvalidToken.to_string(),
        )),
    }

    
}