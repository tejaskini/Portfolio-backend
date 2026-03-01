// Password utility endpoints (for development/testing only)
use actix_web::{post, web, HttpResponse};
use crate::error::MyError;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Deserialize)]
pub struct EncodePasswordRequest {
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct EncodePasswordResponse {
    pub original_password: String,
    pub encoded_password: String,
}

#[derive(Debug, Deserialize)]
pub struct DecodePasswordRequest {
    pub encoded_password: String,
}

#[derive(Debug, Serialize)]
pub struct DecodePasswordResponse {
    pub encoded_password: String,
    pub decoded_password: String,
}

/// Encode password to base64 string
/// POST /dev/encode-password
/// Body: { "password": "your_password" }
/// Returns: base64 encoded string
#[post("/encode-password")]
pub async fn encode_password(
    payload: web::Json<EncodePasswordRequest>,
) -> Result<HttpResponse, MyError> {
    // Encode the password to base64
    let encoded = general_purpose::STANDARD.encode(payload.password.as_bytes());

    Ok(HttpResponse::Ok().json(EncodePasswordResponse {
        original_password: payload.password.clone(),
        encoded_password: encoded,
    }))
}

/// Decode base64 password to plain text
/// POST /dev/decode-password
/// Body: { "encoded_password": "base64_encoded_string" }
/// Returns: plain text password
#[post("/decode-password")]
pub async fn decode_password(
    payload: web::Json<DecodePasswordRequest>,
) -> Result<HttpResponse, MyError> {
    // Decode the base64 string to bytes
    let decoded_bytes = general_purpose::STANDARD
        .decode(&payload.encoded_password)
        .map_err(|_| MyError::AuthError("Failed to decode password".to_string()))?;
    
    // Convert bytes to string
    let decoded_password = String::from_utf8(decoded_bytes)
        .map_err(|_| MyError::InternalError)?;

    Ok(HttpResponse::Ok().json(DecodePasswordResponse {
        encoded_password: payload.encoded_password.clone(),
        decoded_password,
    }))
}
