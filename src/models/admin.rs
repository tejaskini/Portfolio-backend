// Admin model
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password_hash: String,
     pub created_at: i64,
     pub admin_type: Option<ObjectId>
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
   
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    pub email_or_phone: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetVerifyRequest {
    pub email_or_phone: String,
}
