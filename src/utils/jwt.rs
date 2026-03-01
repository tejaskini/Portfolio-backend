// JWT utilities
use jsonwebtoken::{encode, Header};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::error::MyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID or Username
    pub exp: usize,
}

pub fn create_jwt(username: &str) -> Result<String, MyError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| MyError::InternalError)
}
