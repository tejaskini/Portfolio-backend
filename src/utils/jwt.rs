// JWT utilities
use jsonwebtoken::{encode, Header};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::error::MyError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // username
    pub exp: usize,      // expiration
    pub admin_type: String,
    pub is_admin: bool,
    pub user_id: String,
}

pub fn create_jwt(username: &str,user_id: &str, user_type: &str, is_admin: bool) -> Result<String, MyError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
        admin_type: user_type.to_string(),
        is_admin: is_admin,
        user_id: user_id.to_string(),

    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| MyError::InternalError)
}
